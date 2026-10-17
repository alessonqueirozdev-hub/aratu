//! Reading a music line: bar lines and repeat signs, markers and jumps, and
//! the beats with the parts that share them.
//!
//! The rules, from the manual:
//! 1. A space separates beats.
//! 2. Symbols written together share their beat equally (`cc` two eighths).
//! 3. `_` holds the sound before for one more part, `-` is a rest, and `.`
//!    is a shorthand for `__` (`c.c` is a dotted eighth and a sixteenth).
//!
//! Brackets group parts explicitly (`[d e]`), with spaces allowed inside, and
//! nest. Braces choose one of several each time round (`{c2|g1}`).

use crate::ast::*;
use crate::diag::{Diagnostic, Message};
use crate::motifs::{self, Library, MAX_DEPTH, PLACEHOLDER};
use crate::span::Span;
use crate::theory::{Chord, Dyn, accidentals, note_name};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BarLine {
    Plain,
    /// `|:`
    Start,
    /// `:|`
    End,
    /// `:|:`
    EndStart,
    /// `|1`
    Ending(u32),
    /// `:|2`
    EndEnding(u32),
}

#[derive(Clone, Debug)]
enum Tok<'a> {
    Word(&'a str, Span),
    Line(BarLine, Span),
}

/// The drum symbols and what they play.
pub const DRUM_SYMBOLS: &[char] = &['c', 'o', 'b', 's', 'r', 'x', 't', 'm', 'f', 'k', 'p'];

fn tokenize<'a>(text: &'a str, base: usize, diags: &mut Vec<Diagnostic>) -> Vec<Tok<'a>> {
    let mut toks = Vec::new();
    let mut word_start: Option<usize> = None;
    let mut opens: Vec<(char, usize)> = Vec::new();
    let bytes = text.as_bytes();
    let mut i = 0;
    let flush = |toks: &mut Vec<Tok<'a>>, ws: &mut Option<usize>, end: usize| {
        if let Some(s) = ws.take() {
            toks.push(Tok::Word(&text[s..end], Span::new(base + s, base + end)));
        }
    };
    let digits = |from: usize| -> (Option<u32>, usize) {
        let mut j = from;
        while j < bytes.len() && bytes[j].is_ascii_digit() {
            j += 1;
        }
        (text[from..j].parse().ok(), j)
    };
    while i < text.len() {
        // A line continued on the next keeps the comment of the first.
        if text[i..].starts_with("//") {
            flush(&mut toks, &mut word_start, i);
            i = text[i..].find('\n').map_or(text.len(), |n| i + n);
            continue;
        }
        let c = text[i..].chars().next().unwrap();
        let clen = c.len_utf8();
        if opens.is_empty() {
            if c.is_whitespace() {
                // A dynamic belongs to the beat after it, with or without a
                // space: `p: c4` reads as `p:c4`.
                let rest = text[i..].trim_start();
                let dynamic = word_start.is_some_and(|s| {
                    text[s..i]
                        .strip_suffix(':')
                        .is_some_and(|w| Dyn::parse(w).is_some())
                });
                if dynamic && !rest.is_empty() && !rest.starts_with('|') {
                    i = text.len() - rest.len();
                    continue;
                }
                flush(&mut toks, &mut word_start, i);
                i += clen;
                continue;
            }
            if c == '|' {
                flush(&mut toks, &mut word_start, i);
                let start = i;
                i += 1;
                let line = if text[i..].starts_with(':') {
                    i += 1;
                    BarLine::Start
                } else if text[i..].starts_with(['|', ']']) {
                    i += 1;
                    BarLine::Plain
                } else {
                    match digits(i) {
                        (Some(n), j) => {
                            i = j;
                            BarLine::Ending(n)
                        }
                        _ => BarLine::Plain,
                    }
                };
                toks.push(Tok::Line(line, Span::new(base + start, base + i)));
                continue;
            }
            if c == ':' && text[i + 1..].starts_with('|') {
                flush(&mut toks, &mut word_start, i);
                let start = i;
                i += 2;
                let line = if text[i..].starts_with(':') {
                    i += 1;
                    BarLine::EndStart
                } else {
                    match digits(i) {
                        (Some(n), j) => {
                            i = j;
                            BarLine::EndEnding(n)
                        }
                        _ => BarLine::End,
                    }
                };
                toks.push(Tok::Line(line, Span::new(base + start, base + i)));
                continue;
            }
        }
        match c {
            '[' | '{' => opens.push((c, i)),
            // `b(3, 8)`: a Euclidean rhythm keeps its spaces; a ghost
            // note's `(s)` has none.
            '(' if text[i + 1..].starts_with(|d: char| d.is_ascii_digit()) => opens.push((c, i)),
            ')' if opens.last().is_some_and(|&(o, _)| o == '(') => {
                opens.pop();
            }
            ']' | '}' => {
                let want = if c == ']' { '[' } else { '{' };
                if opens.last().is_some_and(|&(o, _)| o == want) {
                    opens.pop();
                }
                // A mismatched closer is reported when the word is read.
            }
            _ => {}
        }
        if word_start.is_none() {
            word_start = Some(i);
        }
        i += clen;
    }
    // An open parenthesis is told when its rhythm is read.
    if let Some(&(open, at)) = opens.iter().find(|(o, _)| *o != '(') {
        diags.push(Diagnostic::error(
            Span::new(base + at, base + at + 1),
            Message::UnclosedBracket { open },
        ));
    }
    flush(&mut toks, &mut word_start, text.len());
    // The bar line that ends a line and the one that starts the next, in
    // music written on several lines, are one bar line.
    let mut merged: Vec<Tok<'a>> = Vec::with_capacity(toks.len());
    for t in toks {
        if let (Some(Tok::Line(a, sa)), Tok::Line(b, sb)) = (merged.last(), &t) {
            let between = &text[sa.end - base..sb.start - base];
            let blank = between
                .lines()
                .all(|l| l.split("//").next().unwrap_or("").trim().is_empty());
            if between.contains('\n') && blank {
                let kind = match (*a, *b) {
                    (BarLine::Plain, x) | (x, BarLine::Plain) => x,
                    (BarLine::End, BarLine::Start) => BarLine::EndStart,
                    (x, _) => x,
                };
                let span = sa.join(*sb);
                *merged.last_mut().unwrap() = Tok::Line(kind, span);
                continue;
            }
        }
        merged.push(t);
    }
    merged
}

/// Reads the music after `=` in a line of the given kind.
pub fn parse_music(
    text: &str,
    base: usize,
    kind: LineKind,
    diags: &mut Vec<Diagnostic>,
) -> Vec<Bar> {
    parse_music_with(text, base, kind, diags, &Library::default(), false)
}

/// What reading music may draw on: the piece's motifs, and whether an `o`
/// stands for a note still to come from a row.
#[derive(Clone, Copy)]
struct Cx<'l> {
    lib: &'l Library,
    placeholders: bool,
    depth: u8,
}

/// Reads music that may use the piece's motifs; with `placeholders`, an
/// `o` is a note whose pitch a row will give (see [`motifs::fill`]).
pub fn parse_music_with(
    text: &str,
    base: usize,
    kind: LineKind,
    diags: &mut Vec<Diagnostic>,
    lib: &Library,
    placeholders: bool,
) -> Vec<Bar> {
    let cx = Cx {
        lib,
        placeholders,
        depth: 0,
    };
    parse_in(text, base, kind, diags, cx)
}

fn parse_in(
    text: &str,
    base: usize,
    kind: LineKind,
    diags: &mut Vec<Diagnostic>,
    cx: Cx,
) -> Vec<Bar> {
    let toks = tokenize(text, base, diags);
    let mut seg = Segments {
        bars: Vec::new(),
        kind,
        octave: None,
        graces: Vec::new(),
    };
    let mut words: Vec<(&str, Span)> = Vec::new();
    let mut before: Option<(BarLine, Span)> = None;
    for tok in toks {
        match tok {
            Tok::Word(w, s) => words.push((w, s)),
            Tok::Line(bl, s) => {
                seg.close(&mut words, before, Some((bl, s)), diags, cx);
                before = Some((bl, s));
            }
        }
    }
    seg.close(&mut words, before, None, diags, cx);
    if let Some((_, span)) = seg.graces.first() {
        diags.push(Diagnostic::error(*span, Message::GraceBeforeNothing));
    }
    seg.bars
}

/// Gives the grace notes waiting from the words before to the first note
/// of `beats`.
fn lead_into(beats: &mut [Beat], graces: &mut Vec<(Sound, Span)>) {
    fn first(slots: &mut [Slot]) -> Option<&mut Hit> {
        for slot in slots {
            let found = match slot {
                Slot::Hit(h) => Some(h),
                Slot::Group(inner, _) => first(inner),
                Slot::Choice(options, _) => options.first_mut().and_then(|o| first(o)),
                Slot::Hold(_) | Slot::Rest(_) => None,
            };
            if found.is_some() {
                return found;
            }
        }
        None
    }
    if graces.is_empty() {
        return;
    }
    if let Some(h) = beats.iter_mut().find_map(|b| first(&mut b.slots)) {
        h.grace.extend(graces.drain(..).map(|(g, _)| g));
    }
}

/// The music of a line or of a list's melody: a plain rhythm, or one whose
/// `o`s take their notes from a row (`… from serie.P0`).
pub fn music_of(
    lib: &Library,
    diags: &mut Vec<Diagnostic>,
    text: &str,
    base: usize,
    kind: LineKind,
) -> Vec<Bar> {
    let cx = Cx {
        lib,
        placeholders: false,
        depth: 0,
    };
    music_in(text, base, kind, diags, cx)
}

fn music_in(
    text: &str,
    base: usize,
    kind: LineKind,
    diags: &mut Vec<Diagnostic>,
    cx: Cx,
) -> Vec<Bar> {
    if kind == LineKind::Pitched {
        if let Some((at, len)) = row_from(text) {
            return rhythm_from_row(
                cx,
                diags,
                &text[..at],
                base,
                &text[at + len..],
                base + at + len,
                kind,
            );
        }
    }
    parse_in(text, base, kind, diags, cx)
}

/// `| o o [o o] o | from serie.P0 octave 4`: a rhythm whose `o`s take their
/// pitches from a row, a motif or a vector `[0 11 3 …]`.
fn rhythm_from_row(
    cx: Cx,
    diags: &mut Vec<Diagnostic>,
    rhythm: &str,
    base: usize,
    source: &str,
    source_base: usize,
    kind: LineKind,
) -> Vec<Bar> {
    let lib = cx.lib;
    let trimmed = source.trim_start();
    let lead = source.len() - trimmed.len();
    let src_span = |from: usize, len: usize| {
        Span::new(source_base + lead + from, source_base + lead + from + len)
    };
    let (mut pitches, rest) = if let Some(inner) = trimmed.strip_prefix('[') {
        let close = inner.find(']').unwrap_or(inner.len());
        let (pitches, bad, _) = motifs::read_row(&inner[..close]);
        for (at, len) in bad {
            diags.push(Diagnostic::error(
                src_span(1 + at, len),
                Message::BadRowNote {
                    text: inner[at..at + len].to_string(),
                },
            ));
        }
        (pitches, inner.get(close + 1..).unwrap_or(""))
    } else {
        let end = trimmed.find(char::is_whitespace).unwrap_or(trimmed.len());
        let word = &trimmed[..end];
        let pitches = match motifs::form_ref(word) {
            Err(form) => {
                diags.push(Diagnostic::error(
                    src_span(0, end),
                    Message::BadRowForm { form },
                ));
                Vec::new()
            }
            Ok((name, form, n)) => {
                let row = if let Some(r) = lib.rows.get(name) {
                    Some(r.clone())
                } else if let Some(src) = lib.motifs.get(name) {
                    // A motif's notes, in order, as a row.
                    let mut quiet = Vec::new();
                    let inner = Cx {
                        depth: cx.depth + 1,
                        placeholders: false,
                        ..cx
                    };
                    let mut bars =
                        parse_in(&src.text, src.base, LineKind::Pitched, &mut quiet, inner);
                    let mut v = Vec::new();
                    motifs::each_note_in_bars(&mut bars, &mut |n| v.extend(n.iter().copied()));
                    Some(v)
                } else {
                    None
                };
                match row {
                    Some(r) => motifs::row_form(&r, form, n),
                    None => {
                        diags.push(Diagnostic::error(
                            src_span(0, name.len()),
                            Message::UnknownRow {
                                name: name.to_string(),
                            },
                        ));
                        Vec::new()
                    }
                }
            }
        };
        (pitches, &trimmed[end..])
    };
    // `octave 3`: every pitch class in that octave.
    let mut words = rest.split_whitespace();
    if let (Some(w), Some(n)) = (words.next(), words.next()) {
        if crate::words::is(w, "octave") {
            if let Ok(o) = n.parse::<i32>() {
                for p in &mut pitches {
                    *p = p.rem_euclid(12) + 12 * (o + 1);
                }
            }
        }
    }
    let mut bars = parse_in(
        rhythm,
        base,
        kind,
        diags,
        Cx {
            placeholders: true,
            ..cx
        },
    );
    if motifs::fill(&mut bars, &pitches) == 0 {
        diags.push(Diagnostic::warning(
            Span::new(base, base + rhythm.trim_end().len()),
            Message::NoPlaceholders,
        ));
    }
    bars
}

/// Where `from` sits in a rhythm that takes its notes from a row, and how
/// long the word is with its spaces: `from`, `desde`, or `com` and
/// `con` (the short Portuguese and Spanish words for "from" are notes:
/// `do`, `de`).
fn row_from(text: &str) -> Option<(usize, usize)> {
    [" from ", " desde ", " com ", " con "]
        .iter()
        .filter_map(|w| text.find(w).map(|at| (at, w.len())))
        .min()
}

/// What `temas.2` or `temas.all` stands for when it fills a bar: the list's
/// melodies, or, out of range, the number asked for and how many there are.
type ListFound<'l> = Result<Vec<&'l motifs::Source>, (String, usize, usize)>;

fn list_ref<'l>(word: &str, lib: &'l Library) -> Option<ListFound<'l>> {
    let (name, which) = word.split_once('.')?;
    let items = lib.lists.get(name)?;
    if crate::words::is(which, "all") {
        return Some(Ok(items.iter().collect()));
    }
    let index: usize = which.parse().ok()?;
    Some(match index.checked_sub(1).and_then(|i| items.get(i)) {
        Some(item) => Ok(vec![item]),
        None => Err((name.to_string(), index, items.len())),
    })
}

/// Bars being assembled from the words between bar lines.
struct Segments {
    bars: Vec<Bar>,
    kind: LineKind,
    /// The octave carried from note to note, in written order.
    octave: Option<i32>,
    /// Grace notes at the end of a bar, leading into the next one.
    graces: Vec<(Sound, Span)>,
}

impl Segments {
    fn close(
        &mut self,
        words: &mut Vec<(&str, Span)>,
        before: Option<(BarLine, Span)>,
        after: Option<(BarLine, Span)>,
        diags: &mut Vec<Diagnostic>,
        cx: Cx,
    ) {
        // `x4` right after `:|` belongs to the bar before.
        if let Some((BarLine::End | BarLine::EndStart | BarLine::EndEnding(_), _)) = before {
            if let Some(&(w, s)) = words.first() {
                if let Some(n) = repeat_count(w) {
                    if let Some(last) = self.bars.last_mut() {
                        last.repeat_end = Some(n);
                        last.span = last.span.join(s);
                    }
                    words.remove(0);
                }
            }
        }
        if words.is_empty() {
            let at_an_edge = (before.is_none() && self.bars.is_empty()) || after.is_none();
            // `:| |:`, `:| x4 |` and `:||:` are two signs meeting at one bar
            // line, as a score draws them, not a bar left empty.
            let signs_meet = matches!(
                before,
                Some((BarLine::End | BarLine::EndStart | BarLine::EndEnding(_), _))
            ) || matches!(after, Some((BarLine::Start, _)));
            if !at_an_edge && !signs_meet {
                if let (Some((_, b)), Some((_, a))) = (before, after) {
                    diags.push(Diagnostic::error(b.join(a), Message::EmptyBar));
                }
            }
            if let (Some((bl, _)), Some(last)) = (after, self.bars.last_mut()) {
                apply_after(last, bl);
            }
            return;
        }
        // A list's melody standing for a bar: `| temas.1 | temas.3 |`. Its
        // bars come in its place; its own mistakes are told in the list.
        if words.len() == 1 {
            if let Some(found) = list_ref(words[0].0, cx.lib) {
                let (word, at) = words[0];
                words.clear();
                match found {
                    Err((name, index, count)) => {
                        diags.push(Diagnostic::error(
                            at,
                            Message::ListIndex { name, index, count },
                        ));
                    }
                    Ok(_) if cx.depth >= MAX_DEPTH => {
                        let name = word.split('.').next().unwrap_or(word).to_string();
                        diags.push(Diagnostic::error(at, Message::MotifLoop { name }));
                    }
                    Ok(items) => {
                        let inner = Cx {
                            depth: cx.depth + 1,
                            placeholders: false,
                            ..cx
                        };
                        let mut quiet = Vec::new();
                        let first = self.bars.len();
                        for item in items {
                            self.bars.extend(music_in(
                                &item.text, item.base, self.kind, &mut quiet, inner,
                            ));
                        }
                        // A list that uses itself is told where it is used.
                        if let Some(d) = quiet
                            .into_iter()
                            .find(|d| matches!(d.message, Message::MotifLoop { .. }))
                        {
                            diags.push(Diagnostic::error(at, d.message));
                        }
                        if let (Some((bl, _)), Some(bar)) = (before, self.bars.get_mut(first)) {
                            match bl {
                                BarLine::Start | BarLine::EndStart => bar.repeat_start = true,
                                BarLine::Ending(n) | BarLine::EndEnding(n) => bar.ending = Some(n),
                                _ => {}
                            }
                        }
                        if let (Some((bl, _)), Some(last)) = (after, self.bars.last_mut()) {
                            apply_after(last, bl);
                        }
                    }
                }
                return;
            }
        }
        let span = words.iter().fold(words[0].1, |s, (_, ws)| s.join(*ws));
        let mut bar = read_bar(
            words,
            span,
            self.kind,
            &mut self.octave,
            &mut self.graces,
            diags,
            cx,
        );
        if let Some((bl, _)) = before {
            match bl {
                BarLine::Start | BarLine::EndStart => bar.repeat_start = true,
                BarLine::Ending(n) | BarLine::EndEnding(n) => bar.ending = Some(n),
                _ => {}
            }
        }
        if let Some((bl, _)) = after {
            apply_after(&mut bar, bl);
        }
        self.bars.push(bar);
        words.clear();
    }
}

fn apply_after(bar: &mut Bar, bl: BarLine) {
    if let BarLine::End | BarLine::EndStart | BarLine::EndEnding(_) = bl {
        if bar.repeat_end.is_none() {
            bar.repeat_end = Some(2);
        }
    }
}

fn repeat_count(w: &str) -> Option<u32> {
    let n = w.strip_prefix(['x', '×'])?;
    n.parse().ok().filter(|&n| n >= 1)
}

const MARKERS: &[(&str, Marker)] = &[
    ("segno:", Marker::Segno),
    ("fine:", Marker::Fine),
    ("coda:", Marker::Coda),
];

fn is_jump_word(w: &str) -> Option<bool> {
    match w {
        "D.S." | "D.S" | "DS" => Some(true),
        "D.C." | "D.C" | "DC" => Some(false),
        _ => None,
    }
}

fn read_bar(
    words: &mut Vec<(&str, Span)>,
    span: Span,
    kind: LineKind,
    octave: &mut Option<i32>,
    graces: &mut Vec<(Sound, Span)>,
    diags: &mut Vec<Diagnostic>,
    cx: Cx,
) -> Bar {
    let mut bar = Bar {
        span,
        content: BarContent::Beats(Vec::new()),
        repeat_start: false,
        ending: None,
        repeat_end: None,
        markers: Vec::new(),
        jump: None,
    };

    // Markers at the start.
    let mut i = 0;
    while i < words.len() {
        let (w, s) = words[i];
        let lower = w.to_lowercase();
        let Some(&(m, marker)) = MARKERS.iter().find(|(m, _)| lower.starts_with(m)) else {
            break;
        };
        bar.markers
            .push((marker, Span::new(s.start, s.start + m.len())));
        if w.len() > m.len() {
            words[i] = (&w[m.len()..], Span::new(s.start + m.len(), s.end));
            break;
        }
        i += 1;
    }
    words.drain(..i);

    // A jump at the end.
    let n = words.len();
    let lower: Vec<String> = words.iter().map(|(w, _)| w.to_lowercase()).collect();
    if n >= 2 && lower[n - 2] == "to" && lower[n - 1] == "coda" {
        bar.jump = Some((Jump::ToCoda, words[n - 2].1.join(words[n - 1].1)));
        words.truncate(n - 2);
    } else if let Some(j) = words.iter().rposition(|(w, _)| is_jump_word(w).is_some()) {
        let to_segno = is_jump_word(words[j].0).unwrap();
        let tail: Vec<&str> = lower[j + 1..].iter().map(String::as_str).collect();
        let al = match tail.as_slice() {
            [] => Some(Al::End),
            ["al", "fine"] => Some(Al::Fine),
            ["al", "coda"] => Some(Al::Coda),
            _ => None,
        };
        let jspan = words[j..].iter().fold(words[j].1, |s, (_, ws)| s.join(*ws));
        match al {
            Some(al) => {
                bar.jump = Some((Jump::Back { to_segno, al }, jspan));
                words.truncate(j);
            }
            None => {
                diags.push(Diagnostic::error(
                    jspan,
                    Message::MisplacedJump {
                        jump: words[j].0.to_string(),
                    },
                ));
                words.truncate(j);
            }
        }
    }
    // Jump words anywhere else are misplaced.
    let mut k = 0;
    while k < words.len() {
        if is_jump_word(words[k].0).is_some() {
            diags.push(Diagnostic::error(
                words[k].1,
                Message::MisplacedJump {
                    jump: words[k].0.to_string(),
                },
            ));
            words.remove(k);
        } else {
            k += 1;
        }
    }

    // `%` and `%%` stand alone.
    if let Some(p) = words.iter().position(|(w, _)| *w == "%" || *w == "%%") {
        if words.len() == 1 {
            bar.content = BarContent::Simile(words[p].0.len() as u8);
            return bar;
        }
        diags.push(Diagnostic::error(words[p].1, Message::BarRepeatNotAlone));
        words.remove(p);
    }

    // A rest alone is a whole bar of rest; `- x8` is eight bars.
    match words.as_slice() {
        [("-", _)] => {
            bar.content = BarContent::Rest(1);
            return bar;
        }
        [("-", _), (w, _)] if repeat_count(w).is_some() => {
            bar.content = BarContent::Rest(repeat_count(w).unwrap());
            return bar;
        }
        _ => {}
    }

    let mut beats = Vec::new();
    for &(w, s) in words.iter() {
        beats.extend(word_beats(w, s, kind, octave, graces, diags, cx));
    }
    // A Euclidean rhythm fills the bar only alone in it, as a rest does.
    let alone = beats.len() == 1;
    for b in &mut beats {
        b.fill &= alone;
    }
    bar.content = BarContent::Beats(beats);
    bar
}

/// The beats one word makes: usually one, more for a motif or a repeat
/// (`[e3 d3 c3 d3]x4`, `riff+2`, `riffx2`).
fn word_beats(
    w: &str,
    s: Span,
    kind: LineKind,
    octave: &mut Option<i32>,
    graces: &mut Vec<(Sound, Span)>,
    diags: &mut Vec<Diagnostic>,
    cx: Cx,
) -> Vec<Beat> {
    if kind == LineKind::Chords {
        return vec![Beat {
            slots: chord_beat(
                w,
                s,
                diags,
                cx.lib.key.unwrap_or(crate::theory::Key {
                    tonic: 0,
                    minor: false,
                }),
            ),
            span: s,
            len: 1,
            fill: false,
        }];
    }
    let (core, times) = motifs::split_times(w, cx.lib);
    let cs = Span::new(s.start, s.start + core.len());
    let once = match motifs::motif_ref(core, cx.lib) {
        Some((name, ops)) => {
            let mut beats = motif_beats(name, &ops, cs, kind, diags, cx);
            lead_into(&mut beats, graces);
            beats
        }
        // `riff.foo`, `temas.foo`: a motif or a list with a word it lacks.
        None if unknown_pick(core, cs, cx, diags) => vec![Beat {
            slots: vec![Slot::Rest(cs)],
            span: cs,
            len: 1,
            fill: false,
        }],
        None => {
            // `2[c d e]`: a group across that many beats.
            let digits = core.bytes().take_while(u8::is_ascii_digit).count();
            let (len, body) = match core[digits..].strip_prefix('[') {
                Some(_) if digits > 0 => {
                    (core[..digits].parse::<u32>().unwrap_or(1).max(1), digits)
                }
                _ => (1, 0),
            };
            let mut p = SlotParser {
                s: &core[body..],
                base: cs.start + body,
                pos: 0,
                kind,
                octave,
                diags,
                pending: None,
                placeholders: cx.placeholders,
                graces: std::mem::take(graces),
                in_grace: false,
                euclid: None,
                key: cx.lib.key,
            };
            let slots = p.seq(None, false);
            *graces = p.graces;
            // A word of grace notes only, `` `d5 ``, is no beat: they lead
            // into the next word's first note.
            if slots.is_empty() && core.starts_with('`') {
                return Vec::new();
            }
            // A Euclidean rhythm and nothing else may fill its bar.
            let fill = body == 0
                && matches!(slots.as_slice(), [Slot::Group(_, sp)] if Some(*sp) == p.euclid);
            vec![Beat {
                slots,
                span: cs,
                len,
                fill,
            }]
        }
    };
    let mut beats = Vec::with_capacity(once.len() * times as usize);
    for _ in 0..times {
        beats.extend(once.iter().cloned());
    }
    beats
}

/// Tells `riff.foo` and `temas.foo` as a motif's or a list's mistake
/// rather than as notes; true when it did.
fn unknown_pick(word: &str, s: Span, cx: Cx, diags: &mut Vec<Diagnostic>) -> bool {
    let Some((name, rest)) = word.split_once('.') else {
        return false;
    };
    let pick = rest.split(['+', '-']).next().unwrap_or(rest).to_string();
    let message = if cx.lib.motifs.contains_key(name) {
        Message::UnknownMotifForm {
            name: name.to_string(),
            form: pick,
        }
    } else if cx.lib.lists.contains_key(name) {
        Message::UnknownListPick {
            name: name.to_string(),
            pick,
        }
    } else {
        return false;
    };
    diags.push(Diagnostic::error(s, message));
    true
}

/// A motif's beats, moved as asked. Its notes keep the spans of its
/// definition, so the editor lights the motif where it is written; its
/// own mistakes are told once, where it is defined.
fn motif_beats(
    name: &str,
    ops: &[motifs::Op],
    s: Span,
    kind: LineKind,
    diags: &mut Vec<Diagnostic>,
    cx: Cx,
) -> Vec<Beat> {
    if cx.depth >= MAX_DEPTH {
        diags.push(Diagnostic::error(
            s,
            Message::MotifLoop {
                name: name.to_string(),
            },
        ));
        return vec![Beat {
            slots: vec![Slot::Rest(s)],
            span: s,
            len: 1,
            fill: false,
        }];
    }
    let src = &cx.lib.motifs[name];
    let mut quiet = Vec::new();
    let inner = Cx {
        depth: cx.depth + 1,
        placeholders: false,
        ..cx
    };
    let bars = parse_in(&src.text, src.base, kind, &mut quiet, inner);
    // A loop found deeper is reported here, where the loop is used.
    if let Some(d) = quiet
        .into_iter()
        .find(|d| matches!(d.message, Message::MotifLoop { .. }))
    {
        diags.push(Diagnostic::error(s, d.message));
    }
    let mut beats: Vec<Beat> = bars
        .into_iter()
        .flat_map(|b| match b.content {
            BarContent::Beats(beats) => beats,
            _ => Vec::new(),
        })
        .collect();
    let key = cx.lib.key.unwrap_or(crate::theory::Key {
        tonic: 0,
        minor: false,
    });
    motifs::apply(&mut beats, ops, key);
    beats
}

/// Splits trailing `'`, `^`, `?` and `?30%` off a chord or note word.
fn split_mods(w: &str) -> (&str, bool, bool, Option<f64>, Option<f64>) {
    let mut stacc = false;
    let mut ferm = false;
    let mut chance = None;
    let mut bad_chance = None;
    let mut core = w;
    loop {
        if let Some(r) = core.strip_suffix('\'') {
            stacc = true;
            core = r;
        } else if let Some(r) = core.strip_suffix('^') {
            ferm = true;
            core = r;
        } else if let Some(q) = core.rfind('?') {
            let after = &core[q + 1..];
            if after.is_empty() {
                chance = Some(0.5);
            } else if let Some(n) = after.strip_suffix('%').and_then(|n| n.parse::<f64>().ok()) {
                if (0.0..=100.0).contains(&n) {
                    chance = Some(n / 100.0);
                } else {
                    bad_chance = Some(n);
                }
            } else {
                break;
            }
            core = &core[..q];
        } else {
            break;
        }
    }
    (core, stacc, ferm, chance, bad_chance)
}

/// The words for a rolled chord, in the three languages.
const ARP_WORDS: &[&str] = &["arp", "arpejo", "arpegio", "arpeggio"];

/// The words for a trill, in the three languages.
const TRILL_WORDS: &[&str] = &["tr", "trill", "trinado", "trino"];

/// Every word a note may take after a colon, for "did you mean".
const TECHNIQUES: &[&str] = &[
    "arp", "arpejo", "arpegio", "arpeggio", "tr", "trill", "trinado", "trino", "mord", "mordi",
    "mordent", "mordente",
];

/// The ornament a word after a colon names, with how much of `after` it
/// takes: a trill's `#` (`:tr#`); `:trb` lowers the note above.
fn ornament(word: &str, after: &str) -> Option<(Ornament, usize)> {
    if TRILL_WORDS.contains(&word) {
        return Some(if after.starts_with('#') {
            (Ornament::Trill { alter: 1 }, 1)
        } else {
            (Ornament::Trill { alter: 0 }, 0)
        });
    }
    if word
        .strip_suffix('b')
        .is_some_and(|w| TRILL_WORDS.contains(&w))
    {
        return Some((Ornament::Trill { alter: -1 }, 0));
    }
    match word {
        "mord" | "mordent" | "mordente" => Some((Ornament::Mordent { lower: false }, 0)),
        "mordi" => Some((Ornament::Mordent { lower: true }, 0)),
        _ => None,
    }
}

/// Whether `text` starts with the word of a technique.
fn names_technique(text: &str) -> bool {
    let len = text
        .char_indices()
        .find(|(_, c)| !c.is_alphabetic())
        .map_or(text.len(), |(i, _)| i);
    let word = text[..len].to_lowercase();
    ARP_WORDS.contains(&word.as_str()) || ornament(&word, "").is_some()
}

/// `:arp` after a chord or notes, in any of the three languages.
fn strip_arp(core: &str) -> (&str, bool) {
    for mark in [":arp", ":arpejo", ":arpegio", ":arpeggio"] {
        if let Some(c) = core.strip_suffix(mark) {
            return (c, true);
        }
    }
    (core, false)
}

fn chord_beat(w: &str, s: Span, diags: &mut Vec<Diagnostic>, key: crate::theory::Key) -> Vec<Slot> {
    match w {
        "_" => return vec![Slot::Hold(s)],
        // No chord: `N.C.` as a lead sheet writes it, or a rest.
        "-" | "N.C." | "N.C" | "NC" | "n.c." => return vec![Slot::Rest(s)],
        _ => {}
    }
    if let Some(inner) = w.strip_prefix('{').and_then(|r| r.strip_suffix('}')) {
        let mut options = Vec::new();
        let mut off = s.start + 1;
        for opt in inner.split('|') {
            let os = Span::new(off, off + opt.len());
            off += opt.len() + 1;
            if opt.trim().is_empty() {
                diags.push(Diagnostic::error(os, Message::EmptyChoice));
                continue;
            }
            options.push(chord_beat(opt.trim(), os, diags, key));
        }
        return vec![Slot::Choice(options, s)];
    }
    let (mut core, staccato, fermata, chance, bad) = split_mods(w);
    if let Some(n) = bad {
        diags.push(Diagnostic::error(s, Message::ChanceRange { percent: n }));
    }
    let (stripped, arp) = strip_arp(core);
    core = stripped;
    let mut dynamic = None;
    if let Some((d, rest)) = core.split_once(':') {
        if let Some(dy) = Dyn::parse(d) {
            dynamic = Some(dy);
            core = rest.trim_start();
        }
    }
    // A chord symbol, or a degree of the key: `| I | vi | IV | V7 |`.
    match Chord::parse(core).or_else(|| Chord::roman(core, key)) {
        Some(chord) => vec![Slot::Hit(Hit {
            span: s,
            sound: Sound::Chord(chord),
            accent: false,
            marcato: false,
            ghost: false,
            staccato,
            fermata,
            hold: 2,
            roll: false,
            arp,
            ornament: None,
            grace: Vec::new(),
            chance,
            hairpin: None,
            dynamic,
        })],
        None => {
            diags.push(Diagnostic::error(
                s,
                Message::UnknownChord {
                    text: core.to_string(),
                },
            ));
            vec![Slot::Rest(s)]
        }
    }
}

struct SlotParser<'p, 'd> {
    s: &'p str,
    base: usize,
    pos: usize,
    kind: LineKind,
    octave: &'p mut Option<i32>,
    diags: &'d mut Vec<Diagnostic>,
    /// `o` is a note whose pitch a row will give.
    placeholders: bool,
    /// A dynamic written before a group, `p:[c4 d4]`, waiting for the
    /// group's first hit.
    pending: Option<Dyn>,
    /// Grace notes waiting for the note they lead into, maybe from the
    /// words before: `` `d5 c5 ``.
    graces: Vec<(Sound, Span)>,
    /// Reading a grace note, which takes none of the waiting ones.
    in_grace: bool,
    /// Where the last Euclidean rhythm read is written: one alone in its
    /// bar fills the bar.
    euclid: Option<Span>,
    /// The piece's first key, for notes written as degrees (`1 3 5`).
    key: Option<crate::theory::Key>,
}

/// `hits` spread as evenly as can be over `steps` (Bjorklund's way, as
/// Toussaint describes the Euclidean rhythms): (3, 8) is `x..x..x.`, the
/// tresillo.
pub fn euclid(hits: usize, steps: usize) -> Vec<bool> {
    if hits >= steps {
        return vec![true; steps];
    }
    if hits == 0 {
        return vec![false; steps];
    }
    let mut a: Vec<Vec<bool>> = vec![vec![true]; hits];
    let mut b: Vec<Vec<bool>> = vec![vec![false]; steps - hits];
    while b.len() > 1 {
        let m = a.len().min(b.len());
        let rest_a = a.split_off(m);
        let rest_b = b.split_off(m);
        for (x, y) in a.iter_mut().zip(b) {
            x.extend(y);
        }
        b = if rest_a.is_empty() { rest_b } else { rest_a };
    }
    a.into_iter().chain(b).flatten().collect()
}

impl SlotParser<'_, '_> {
    fn peek(&self) -> Option<char> {
        self.s[self.pos..].chars().next()
    }

    fn at(&self, from: usize) -> Span {
        Span::new(self.base + from, self.base + self.pos)
    }

    fn bump(&mut self) -> char {
        let c = self.peek().unwrap();
        self.pos += c.len_utf8();
        c
    }

    /// Parts up to `close` (or `|` between choices), or to the end of the word.
    fn seq(&mut self, close: Option<char>, in_choice: bool) -> Vec<Slot> {
        let mut slots = Vec::new();
        let mut last_was_note = false;
        while let Some(c) = self.peek() {
            if Some(c) == close || (in_choice && c == '|') {
                break;
            }
            let start = self.pos;
            match c {
                c if c.is_whitespace() => {
                    self.bump();
                    last_was_note = false;
                    continue;
                }
                '_' => {
                    self.bump();
                    self.grace_before_nothing();
                    slots.push(Slot::Hold(self.at(start)));
                }
                '-' => {
                    self.bump();
                    self.grace_before_nothing();
                    slots.push(Slot::Rest(self.at(start)));
                }
                '.' => {
                    self.bump();
                    self.grace_before_nothing();
                    let sp = self.at(start);
                    slots.push(Slot::Hold(sp));
                    slots.push(Slot::Hold(sp));
                }
                '[' => {
                    self.bump();
                    let inner = self.seq(Some(']'), false);
                    // A mark before a group of rests reaches no further.
                    self.pending = None;
                    if self.peek() == Some(']') {
                        self.bump();
                    }
                    let sp = self.at(start);
                    if inner.is_empty() {
                        self.diags
                            .push(Diagnostic::error(sp, Message::EmptyBrackets));
                        slots.push(Slot::Rest(sp));
                    } else {
                        slots.push(Slot::Group(inner, sp));
                    }
                }
                '{' => {
                    self.bump();
                    let mut options = Vec::new();
                    // `p:{c4|e4}`: whichever is chosen starts piano.
                    let mark = self.pending;
                    loop {
                        let ostart = self.pos;
                        self.pending = mark;
                        let opt = self.seq(Some('}'), true);
                        self.pending = None;
                        if opt.is_empty() {
                            self.diags.push(Diagnostic::error(
                                self.at(ostart.max(start)),
                                Message::EmptyChoice,
                            ));
                        } else {
                            options.push(opt);
                        }
                        match self.peek() {
                            Some('|') => {
                                self.bump();
                            }
                            Some('}') => {
                                self.bump();
                                break;
                            }
                            _ => break,
                        }
                    }
                    let sp = self.at(start);
                    // `{a2 e3 a e}`: braces with nothing to choose from. It
                    // plays as a group; brackets are the way to say so.
                    if options.len() == 1 {
                        let end = if self.s[..self.pos].ends_with('}') {
                            self.pos - 1
                        } else {
                            self.pos
                        };
                        let inner = self.s[start + 1..end].trim().to_string();
                        self.diags
                            .push(Diagnostic::warning(sp, Message::BracesOneOption { inner }));
                    }
                    if options.is_empty() {
                        slots.push(Slot::Rest(sp));
                    } else {
                        slots.push(Slot::Choice(options, sp));
                    }
                }
                // `` `d5 c5 ``: a grace note before a note; `` `s `` a flam
                // and ``` ``s ``` a drag, strokes of the drum it comes before.
                '`' => {
                    let mut strokes = 0;
                    while self.peek() == Some('`') {
                        self.bump();
                        strokes += 1;
                    }
                    match self.kind {
                        LineKind::Drums | LineKind::Percussion(_) => match self.hit() {
                            Some(Slot::Hit(mut h)) => {
                                h.grace = vec![h.sound.clone(); strokes];
                                h.span = self.at(start);
                                slots.push(Slot::Hit(h));
                            }
                            Some(other) => slots.push(other),
                            None => self.diags.push(Diagnostic::error(
                                self.at(start),
                                Message::GraceBeforeNothing,
                            )),
                        },
                        LineKind::Pitched => {
                            let placeholders = self.placeholders;
                            self.placeholders = false;
                            self.in_grace = true;
                            let grace = self.hit();
                            self.in_grace = false;
                            self.placeholders = placeholders;
                            match grace {
                                Some(Slot::Hit(h)) => self.graces.push((h.sound, self.at(start))),
                                // An unknown note is told already.
                                Some(_) => {}
                                None => self.diags.push(Diagnostic::error(
                                    self.at(start),
                                    Message::GraceBeforeNothing,
                                )),
                            }
                        }
                        _ => self.diags.push(Diagnostic::error(
                            self.at(start),
                            Message::UnexpectedChar { ch: '`' },
                        )),
                    }
                }
                // `(s)`: a ghost note, as drum notation writes it.
                '(' => {
                    self.bump();
                    match self.hit() {
                        Some(Slot::Hit(mut h)) => {
                            if self.peek() == Some(')') {
                                self.bump();
                            } else {
                                self.diags.push(Diagnostic::error(
                                    self.at(start),
                                    Message::UnclosedBracket { open: '(' },
                                ));
                            }
                            h.ghost = true;
                            self.mods(&mut h);
                            h.span = self.at(start);
                            slots.push(Slot::Hit(h));
                        }
                        other => {
                            if self.peek() == Some(')') {
                                self.bump();
                            }
                            slots.push(other.unwrap_or(Slot::Rest(self.at(start))));
                        }
                    }
                }
                ']' | '}' | '|' | ')' => {
                    self.bump();
                    self.diags.push(Diagnostic::error(
                        self.at(start),
                        Message::UnexpectedChar { ch: c },
                    ));
                }
                _ => {
                    let was_note = last_was_note;
                    match self.hit() {
                        Some(slot) => {
                            let is_note =
                                self.kind == LineKind::Pitched && matches!(slot, Slot::Hit(_));
                            if is_note && was_note {
                                if let (Some(Slot::Hit(prev)), Slot::Hit(h)) = (slots.last(), &slot)
                                {
                                    let first = self.text(prev.span);
                                    let second = self.text(h.span);
                                    // Inside brackets already, a space is
                                    // all that is missing.
                                    let message = if close == Some(']') {
                                        Message::GluedNotesInGroup { first, second }
                                    } else {
                                        Message::GluedNotes { first, second }
                                    };
                                    self.diags
                                        .push(Diagnostic::error(prev.span.join(h.span), message));
                                }
                            }
                            slots.push(slot);
                            last_was_note = is_note;
                            continue;
                        }
                        None => {
                            if self.pos == start {
                                // Nothing was read: skip the character.
                                self.bump();
                            }
                        }
                    }
                }
            }
            last_was_note = false;
        }
        slots
    }

    fn text(&self, span: Span) -> String {
        self.s[span.start - self.base..span.end - self.base].to_string()
    }

    /// Grace notes followed by a hold or a rest lead into nothing.
    fn grace_before_nothing(&mut self) {
        if let Some((_, span)) = self.graces.first() {
            self.diags
                .push(Diagnostic::error(*span, Message::GraceBeforeNothing));
            self.graces.clear();
        }
    }

    /// A hit with its dynamic before it and its marks after it. Returns a
    /// rest in place of an unknown symbol, so the beat still counts right.
    fn hit(&mut self) -> Option<Slot> {
        let start = self.pos;
        let mut dynamic = None;
        // A word followed by a colon: a dynamic, or a misplaced marker;
        // unless it is a note or a drum with its technique (`d:tr`, `s:tr`).
        let rest = &self.s[self.pos..];
        let wlen = rest
            .char_indices()
            .find(|(_, c)| !c.is_alphabetic())
            .map_or(rest.len(), |(i, _)| i);
        let technique = wlen > 0
            && Dyn::parse(&rest[..wlen]).is_none()
            && rest[wlen..].strip_prefix(':').is_some_and(names_technique);
        if wlen > 0 && rest[wlen..].starts_with(':') && !technique {
            let word = &rest[..wlen];
            self.pos += wlen + 1;
            let sp = self.at(start);
            if let Some(d) = Dyn::parse(word) {
                dynamic = Some(d);
            } else if MARKERS
                .iter()
                .any(|(m, _)| m[..m.len() - 1].eq_ignore_ascii_case(word))
            {
                self.diags.push(Diagnostic::error(
                    sp,
                    Message::MisplacedMarker {
                        marker: word.to_lowercase(),
                    },
                ));
                return None;
            } else {
                self.diags.push(Diagnostic::error(
                    sp,
                    Message::UnknownMark {
                        word: word.to_string(),
                    },
                ));
                return None;
            }
            // `p: c4` as well as `p:c4`.
            while self.peek().is_some_and(char::is_whitespace) {
                self.bump();
            }
            // Before a group or a choice the mark waits for its first hit.
            if matches!(self.peek(), Some('[' | '{')) {
                self.pending = dynamic;
                return None;
            }
            match self.peek() {
                None | Some(']' | '}' | '|' | '_' | '-') => {
                    self.diags
                        .push(Diagnostic::error(sp, Message::DynamicBeforeNothing));
                    return None;
                }
                _ => {}
            }
        }

        let sound_start = self.pos;
        let c = self.peek()?;
        let (sound, accent) = match self.kind {
            LineKind::Drums | LineKind::Strum | LineKind::Percussion(_) => {
                if !c.is_ascii_alphabetic() {
                    self.bump();
                    self.diags.push(Diagnostic::error(
                        self.at(sound_start),
                        Message::UnexpectedChar { ch: c },
                    ));
                    return None;
                }
                self.bump();
                let lower = c.to_ascii_lowercase();
                let accent = c.is_ascii_uppercase();
                if let LineKind::Percussion(p) = self.kind {
                    if p.stroke(lower).is_none() {
                        self.skip_mods();
                        self.diags.push(Diagnostic::error(
                            self.at(sound_start),
                            Message::UnknownStroke {
                                ch: c,
                                instrument: p,
                            },
                        ));
                        return Some(Slot::Rest(self.at(start)));
                    }
                    (Sound::Drum(lower), accent)
                } else if self.kind == LineKind::Drums {
                    if !DRUM_SYMBOLS.contains(&lower) {
                        self.skip_mods();
                        self.diags.push(Diagnostic::error(
                            self.at(sound_start),
                            Message::UnknownDrum { ch: c },
                        ));
                        return Some(Slot::Rest(self.at(start)));
                    }
                    (Sound::Drum(lower), accent)
                } else {
                    let stroke = match lower {
                        'd' => Stroke::Down,
                        'u' => Stroke::Up,
                        'm' => Stroke::Muted,
                        'g' => Stroke::Tap,
                        't' => Stroke::DownTap,
                        _ => {
                            self.skip_mods();
                            self.diags.push(Diagnostic::error(
                                self.at(sound_start),
                                Message::UnknownStrum { ch: c },
                            ));
                            return Some(Slot::Rest(self.at(start)));
                        }
                    };
                    (Sound::Strum(stroke), accent)
                }
            }
            LineKind::Pitched if self.placeholders && matches!(c, 'o' | 'O') => {
                // A note waiting for its pitch: `o`, or `O` accented.
                self.bump();
                (Sound::Notes(vec![PLACEHOLDER]), c == 'O')
            }
            LineKind::Pitched => {
                let mut notes = Vec::new();
                let mut accent = false;
                loop {
                    let nstart = self.pos;
                    match self.note() {
                        Some((midi, upper)) => {
                            if notes.is_empty() {
                                accent = upper;
                            }
                            notes.push(midi);
                        }
                        None => {
                            let bad: String = self.s[nstart..]
                                .chars()
                                .take_while(|c| c.is_alphanumeric() || *c == '#')
                                .collect();
                            if bad.is_empty() {
                                let ch = self.bump();
                                self.diags.push(Diagnostic::error(
                                    self.at(nstart),
                                    Message::UnexpectedChar { ch },
                                ));
                            } else {
                                self.pos += bad.len();
                                self.diags.push(Diagnostic::error(
                                    self.at(nstart),
                                    Message::UnknownNote { text: bad },
                                ));
                            }
                            if notes.is_empty() {
                                self.skip_mods();
                                return Some(Slot::Rest(self.at(start)));
                            }
                        }
                    }
                    if self.peek() == Some('+') {
                        self.bump();
                        continue;
                    }
                    break;
                }
                (Sound::Notes(notes), accent)
            }
            LineKind::Chords => unreachable!("chord lines are read word by word"),
            LineKind::Arp => unreachable!("an arpeggiator is read by its words"),
        };

        let mut hit = Hit {
            span: Span::default(),
            sound,
            accent,
            marcato: false,
            ghost: false,
            staccato: false,
            fermata: false,
            hold: 2,
            roll: false,
            arp: false,
            ornament: None,
            // Grace notes written before it lead into it.
            grace: if self.in_grace {
                Vec::new()
            } else {
                self.graces.drain(..).map(|(g, _)| g).collect()
            },
            chance: None,
            hairpin: None,
            dynamic: dynamic.or(self.pending.take()),
        };
        self.mods(&mut hit);
        hit.span = self.at(start);
        if self.s[self.pos..].starts_with('(') {
            if let Some(group) = self.euclid(hit.clone(), start) {
                return Some(group);
            }
        }
        Some(Slot::Hit(hit))
    }

    /// `b(3,8)` after a hit: three of it spread over eight steps, `b(3,8,2)`
    /// turned two steps later, as `rotate` turns a line. Nothing when the
    /// parenthesis holds no number (a ghost note follows).
    fn euclid(&mut self, mut hit: Hit, start: usize) -> Option<Slot> {
        let inner = &self.s[self.pos + 1..];
        if !inner.trim_start().starts_with(|c: char| c.is_ascii_digit()) {
            return None;
        }
        let close = inner.find(')');
        let body = &inner[..close.unwrap_or(inner.len())];
        self.pos += 1 + body.len() + usize::from(close.is_some());
        let numbers: Vec<Option<i64>> = body.split(',').map(|n| n.trim().parse().ok()).collect();
        let sp = self.at(start);
        let (hits, steps, turn) = match numbers.as_slice() {
            [Some(h), Some(s)] if close.is_some() => (*h, *s, 0),
            [Some(h), Some(s), Some(t)] if close.is_some() => (*h, *s, *t),
            _ => {
                self.diags.push(Diagnostic::error(
                    sp,
                    Message::BadEuclidForm {
                        text: self.text(sp),
                    },
                ));
                return Some(Slot::Rest(sp));
            }
        };
        if hits < 0 || !(1..=64).contains(&steps) || hits > steps {
            self.diags
                .push(Diagnostic::error(sp, Message::BadEuclid { hits, steps }));
            return Some(Slot::Rest(sp));
        }
        // Marks after the parenthesis count for every hit too: `c(5,8)'`.
        self.mods(&mut hit);
        let sp = self.at(start);
        hit.span = sp;
        let pattern = euclid(hits as usize, steps as usize);
        let n = steps;
        let mut first = true;
        let slots = (0..n)
            .map(|i| {
                if pattern[(i - turn).rem_euclid(n) as usize] {
                    let mut h = hit.clone();
                    // Grace notes lead into the first hit only.
                    if !std::mem::take(&mut first) {
                        h.grace.clear();
                    }
                    Slot::Hit(h)
                } else {
                    Slot::Rest(sp)
                }
            })
            .collect();
        self.euclid = Some(sp);
        Some(Slot::Group(slots, sp))
    }

    fn skip_mods(&mut self) {
        let mut dummy = Hit {
            span: Span::default(),
            sound: Sound::Drum('c'),
            accent: false,
            marcato: false,
            ghost: false,
            staccato: false,
            fermata: false,
            hold: 2,
            roll: false,
            arp: false,
            ornament: None,
            grace: Vec::new(),
            chance: None,
            hairpin: None,
            dynamic: None,
        };
        self.mods(&mut dummy);
    }

    fn mods(&mut self, hit: &mut Hit) {
        while let Some(c) = self.peek() {
            let start = self.pos;
            match c {
                '\'' => hit.staccato = true,
                '!' => hit.marcato = true,
                // `:arp`, `:tr`, `:mord`: a technique, by its word.
                ':' => {
                    let rest = &self.s[self.pos + 1..];
                    let wlen = rest
                        .char_indices()
                        .find(|(_, c)| !c.is_alphabetic())
                        .map_or(rest.len(), |(i, _)| i);
                    if wlen == 0 {
                        break;
                    }
                    let word = rest[..wlen].to_lowercase();
                    self.pos += 1 + wlen;
                    if ARP_WORDS.contains(&word.as_str()) {
                        hit.arp = true;
                        continue;
                    }
                    match ornament(&word, &self.s[self.pos..]) {
                        Some((o, len)) => {
                            self.pos += len;
                            if matches!(hit.sound, Sound::Notes(_)) {
                                hit.ornament = Some(o);
                            } else {
                                self.diags.push(Diagnostic::error(
                                    self.at(start),
                                    Message::OrnamentNeedsNote { word },
                                ));
                            }
                        }
                        None => {
                            let closest =
                                crate::suggest::closest(&word, TECHNIQUES.iter().copied())
                                    .map(String::from);
                            self.diags.push(Diagnostic::error(
                                self.at(start),
                                Message::UnknownTechnique { word, closest },
                            ));
                        }
                    }
                    continue;
                }
                '^' => {
                    hit.fermata = true;
                    // `^3`: held three times as long.
                    self.bump();
                    let rest = &self.s[self.pos..];
                    let n = rest.bytes().take_while(u8::is_ascii_digit).count();
                    if let Ok(times) = rest[..n].parse::<u8>() {
                        hit.hold = times.max(1);
                    }
                    self.pos += n;
                    continue;
                }
                '~' => hit.roll = true,
                '<' => hit.hairpin = Some(Hairpin::Crescendo),
                '>' => hit.hairpin = Some(Hairpin::Diminuendo),
                '?' => {
                    self.bump();
                    let rest = &self.s[self.pos..];
                    let n = rest
                        .char_indices()
                        .find(|(_, c)| !(c.is_ascii_digit() || *c == '.'))
                        .map_or(rest.len(), |(i, _)| i);
                    if n > 0 && rest[n..].starts_with('%') {
                        let v: f64 = rest[..n].parse().unwrap_or(-1.0);
                        self.pos += n + 1;
                        if (0.0..=100.0).contains(&v) {
                            hit.chance = Some(v / 100.0);
                        } else {
                            self.diags.push(Diagnostic::error(
                                self.at(start),
                                Message::ChanceRange { percent: v },
                            ));
                        }
                    } else {
                        hit.chance = Some(0.5);
                    }
                    continue;
                }
                _ => break,
            }
            self.bump();
        }
    }

    /// A note name with accidentals and octave: `c4`, `f#`, `eb4`, `dó4`;
    /// or a degree of the key, `1` to `7` and on up (`8` is `1` an octave
    /// higher), with accidentals after it (`3b`, `4#`) and a comma for
    /// each octave down (`5,`). The tonic of the first key, in octave 4, is
    /// `1`; the scale is major or natural minor.
    fn note(&mut self) -> Option<(i32, bool)> {
        let rest = &self.s[self.pos..];
        if rest.starts_with(|c: char| c.is_ascii_digit()) {
            return self.degree();
        }
        let (pc, upper, len) = note_name(rest)?;
        let start = self.pos;
        self.pos += len;
        let (acc, alen) = accidentals(&self.s[self.pos..]);
        self.pos += alen;
        let rest = &self.s[self.pos..];
        let dlen = rest.bytes().take_while(u8::is_ascii_digit).count();
        let octave = if dlen > 0 && dlen <= 2 {
            let o: i32 = rest[..dlen].parse().ok()?;
            self.pos += dlen;
            o
        } else {
            match *self.octave {
                Some(o) => o,
                None => {
                    let name = self.s[start..self.pos].to_string();
                    self.diags.push(Diagnostic::warning(
                        self.at(start),
                        Message::NoOctave { note: name },
                    ));
                    4
                }
            }
        };
        *self.octave = Some(octave);
        Some((12 * (octave + 1) + pc + acc, upper))
    }

    /// A degree of the key (see [`Self::note`]).
    fn degree(&mut self) -> Option<(i32, bool)> {
        let start = self.pos;
        let rest = &self.s[self.pos..];
        let n = rest.bytes().take_while(u8::is_ascii_digit).count();
        let d: i32 = rest[..n].parse().ok()?;
        if !(1..=21).contains(&d) {
            self.pos += n;
            self.diags.push(Diagnostic::error(
                self.at(start),
                Message::DegreeRange { degree: d },
            ));
            return Some((60, false));
        }
        self.pos += n;
        let (acc, alen) = accidentals(&self.s[self.pos..]);
        self.pos += alen;
        let down = self.s[self.pos..].chars().take_while(|c| *c == ',').count() as i32;
        self.pos += down as usize;
        let key = self.key.unwrap_or(crate::theory::Key {
            tonic: 0,
            minor: false,
        });
        let scale: [i32; 7] = if key.minor {
            [0, 2, 3, 5, 7, 8, 10]
        } else {
            [0, 2, 4, 5, 7, 9, 11]
        };
        let step = (d - 1) as usize;
        let midi = 60 + key.tonic + scale[step % 7] + 12 * (step / 7) as i32 + acc - 12 * down;
        Some((midi, false))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ratio::Ratio;

    fn parse(text: &str, kind: LineKind) -> (Vec<Bar>, Vec<Diagnostic>) {
        let mut d = Vec::new();
        let bars = parse_music(text, 0, kind, &mut d);
        (bars, d)
    }

    fn ok(text: &str, kind: LineKind) -> Vec<Bar> {
        let (bars, d) = parse(text, kind);
        assert!(d.is_empty(), "{text}: {d:?}");
        bars
    }

    fn beats(bar: &Bar) -> &[Beat] {
        match &bar.content {
            BarContent::Beats(b) => b,
            c => panic!("not beats: {c:?}"),
        }
    }

    /// A compact picture of a beat's parts: x hit, _ hold, - rest, (..) group.
    fn shape(slots: &[Slot]) -> String {
        slots
            .iter()
            .map(|s| match s {
                Slot::Hit(_) => "x".to_string(),
                Slot::Hold(_) => "_".to_string(),
                Slot::Rest(_) => "-".to_string(),
                Slot::Group(g, _) => format!("({})", shape(g)),
                Slot::Choice(o, _) => format!(
                    "{{{}}}",
                    o.iter().map(|x| shape(x)).collect::<Vec<_>>().join("|")
                ),
            })
            .collect()
    }

    fn shapes(bar: &Bar) -> Vec<String> {
        beats(bar).iter().map(|b| shape(&b.slots)).collect()
    }

    #[test]
    fn a_line_without_bar_lines_is_one_bar() {
        let bars = ok("cc cc co cc", LineKind::Drums);
        assert_eq!(bars.len(), 1);
        assert_eq!(shapes(&bars[0]), ["xx", "xx", "xx", "xx"]);
    }

    #[test]
    fn the_rhythm_table_of_the_manual() {
        let bars = ok("ccc_ cc_c c_cc c__c c.c -c", LineKind::Drums);
        assert_eq!(
            shapes(&bars[0]),
            ["xxx_", "xx_x", "x_xx", "x__x", "x__x", "-x"]
        );
    }

    #[test]
    fn accents_staccato_and_chance() {
        let bars = ok("cC c' c?30% S^", LineKind::Drums);
        let b = beats(&bars[0]);
        let Slot::Hit(h) = &b[0].slots[1] else {
            panic!()
        };
        assert!(h.accent);
        let Slot::Hit(h) = &b[1].slots[0] else {
            panic!()
        };
        assert!(h.staccato);
        let Slot::Hit(h) = &b[2].slots[0] else {
            panic!()
        };
        assert_eq!(h.chance, Some(0.3));
        let Slot::Hit(h) = &b[3].slots[0] else {
            panic!()
        };
        assert!(h.accent && h.fermata);
        assert_eq!(h.sound, Sound::Drum('s'));
    }

    #[test]
    fn notes_with_octave_carried_and_solfege() {
        let bars = ok("| c4 e g c5 | dó4 ré4 mi4 fá#3 |", LineKind::Pitched);
        let notes: Vec<Vec<i32>> = bars
            .iter()
            .flat_map(|b| beats(b).iter())
            .map(|beat| match &beat.slots[0] {
                Slot::Hit(Hit {
                    sound: Sound::Notes(n),
                    ..
                }) => n.clone(),
                _ => panic!(),
            })
            .collect();
        assert_eq!(
            notes,
            [[60], [64], [67], [72], [60], [62], [64], [54]].map(|a| a.to_vec())
        );
    }

    #[test]
    fn brackets_chords_and_flats() {
        let bars = ok("[d4 e] [b b] [eb g] c4+e4+g4", LineKind::Pitched);
        assert_eq!(shapes(&bars[0]), ["(xx)", "(xx)", "(xx)", "x"]);
        let b = beats(&bars[0]);
        let Slot::Group(g, _) = &b[2].slots[0] else {
            panic!()
        };
        let Slot::Hit(h) = &g[0] else { panic!() };
        assert_eq!(h.sound, Sound::Notes(vec![63]));
        let Slot::Hit(h) = &b[3].slots[0] else {
            panic!()
        };
        assert_eq!(h.sound, Sound::Notes(vec![60, 64, 67]));
    }

    #[test]
    fn glued_notes_ask_for_brackets() {
        let (_, d) = parse("c4e4 _ _ _", LineKind::Pitched);
        assert_eq!(
            d[0].message,
            Message::GluedNotes {
                first: "c4".into(),
                second: "e4".into()
            }
        );
    }

    #[test]
    fn dynamics_and_hairpins() {
        let bars = ok("| p:c4< d4 e4 f4 | f:g4> f4 e4 d4 |", LineKind::Pitched);
        let b = beats(&bars[0]);
        let Slot::Hit(h) = &b[0].slots[0] else {
            panic!()
        };
        assert_eq!(h.dynamic, Some(Dyn::P));
        assert_eq!(h.hairpin, Some(Hairpin::Crescendo));
        let b = beats(&bars[1]);
        let Slot::Hit(h) = &b[0].slots[0] else {
            panic!()
        };
        assert_eq!(h.dynamic, Some(Dyn::F));
        assert_eq!(h.hairpin, Some(Hairpin::Diminuendo));
    }

    #[test]
    fn a_dynamic_before_brackets_marks_their_first_note() {
        for src in [
            "| p:[dó4< sol4] ré4 mi4 fá4 |",
            "| p: [dó4< sol4] ré4 mi4 fá4 |",
        ] {
            let bars = ok(src, LineKind::Pitched);
            let b = beats(&bars[0]);
            assert_eq!(b.len(), 4, "{src}");
            let Slot::Group(inner, _) = &b[0].slots[0] else {
                panic!("{src}: {:?}", b[0].slots)
            };
            let Slot::Hit(first) = &inner[0] else {
                panic!()
            };
            assert_eq!(first.dynamic, Some(Dyn::P), "{src}");
            assert_eq!(first.hairpin, Some(Hairpin::Crescendo), "{src}");
            let Slot::Hit(second) = &inner[1] else {
                panic!()
            };
            assert_eq!(second.dynamic, None);
        }
        // Each option of a choice starts with the mark.
        let bars = ok("| p:{c4|e4} d4 e4 f4 |", LineKind::Pitched);
        let Slot::Choice(options, _) = &beats(&bars[0])[0].slots[0] else {
            panic!()
        };
        for o in options {
            let Slot::Hit(h) = &o[0] else { panic!() };
            assert_eq!(h.dynamic, Some(Dyn::P));
        }
        // A mark before rests reaches no further than the brackets.
        let bars = ok("| p:[- -] d4 e4 f4 |", LineKind::Pitched);
        let Slot::Hit(h) = &beats(&bars[0])[1].slots[0] else {
            panic!()
        };
        assert_eq!(h.dynamic, None);
        // Notes glued inside brackets only miss a space.
        let (_, d) = parse("| p:[dó4<sol4] ré4 mi4 fá4 |", LineKind::Pitched);
        assert_eq!(d.len(), 1, "{d:?}");
        assert_eq!(
            d[0].message,
            Message::GluedNotesInGroup {
                first: "dó4<".into(),
                second: "sol4".into()
            }
        );
    }

    #[test]
    fn a_dynamic_may_have_a_space_after_it() {
        // `f:` is forte and the `f` after it is the note F, with or without
        // a space between them.
        for src in ["| p: c4 c c d | f: f f e d |", "| p:c4 c c d | f:f f e d |"] {
            let bars = ok(src, LineKind::Pitched);
            for (bar, dynamic, note) in [(0, Dyn::P, 60), (1, Dyn::F, 65)] {
                let b = beats(&bars[bar]);
                assert_eq!(b.len(), 4, "{src}");
                let Slot::Hit(h) = &b[0].slots[0] else {
                    panic!()
                };
                assert_eq!(h.dynamic, Some(dynamic), "{src}");
                assert_eq!(h.sound, Sound::Notes(vec![note]), "{src}");
            }
        }
        let bars = ok("p: s s f: S s", LineKind::Drums);
        assert_eq!(beats(&bars[0]).len(), 4);
        // A dynamic with nothing after it is still an error.
        let (_, d) = parse("| c4 d4 e4 p: |", LineKind::Pitched);
        assert!(d.iter().any(|d| d.message == Message::DynamicBeforeNothing));
    }

    #[test]
    fn choices() {
        let bars = ok("{cc|co} cc cc cc", LineKind::Drums);
        assert_eq!(shapes(&bars[0])[0], "{xx|xx}");
        let bars = ok("| {c2|g1} _ c2 _ |", LineKind::Pitched);
        assert_eq!(shapes(&bars[0]), ["{x|x}", "_", "x", "_"]);
    }

    #[test]
    fn strums() {
        let bars = ok("D du -u du", LineKind::Strum);
        assert_eq!(shapes(&bars[0]), ["x", "xx", "-x", "xx"]);
        let Slot::Hit(h) = &beats(&bars[0])[0].slots[0] else {
            panic!()
        };
        assert_eq!(h.sound, Sound::Strum(Stroke::Down));
        assert!(h.accent);
        let (_, d) = parse("D dx", LineKind::Strum);
        assert_eq!(d[0].message, Message::UnknownStrum { ch: 'x' });
    }

    #[test]
    fn chord_lines_share_the_bar() {
        let bars = ok("| Am | F | C G | G |", LineKind::Chords);
        assert_eq!(bars.len(), 4);
        assert_eq!(beats(&bars[2]).len(), 2);
        let bars = ok("| Am _ _ F |", LineKind::Chords);
        assert_eq!(shapes(&bars[0]), ["x", "_", "_", "x"]);
        let (_, d) = parse("| Am | Hm |", LineKind::Chords);
        assert_eq!(d[0].message, Message::UnknownChord { text: "Hm".into() });
    }

    #[test]
    fn repeats_endings_and_similes() {
        let bars = ok("|: Am | F |1 C | G :|2 C | C |", LineKind::Chords);
        assert_eq!(bars.len(), 6);
        assert!(bars[0].repeat_start);
        assert_eq!(bars[2].ending, Some(1));
        assert_eq!(bars[3].repeat_end, Some(2));
        assert_eq!(bars[4].ending, Some(2));
        let bars = ok("|: Am | F :| x4", LineKind::Chords);
        assert_eq!(bars.len(), 2);
        assert_eq!(bars[1].repeat_end, Some(4));
        let bars = ok("| Am | % | % | F |", LineKind::Chords);
        assert_eq!(bars[1].content, BarContent::Simile(1));
        let bars = ok("| Am | F | %% |", LineKind::Chords);
        assert_eq!(bars[2].content, BarContent::Simile(2));
    }

    #[test]
    fn markers_and_jumps() {
        let bars = ok(
            "| c4 e g c5 | segno: d4 f a _ | e4 g b _ | fine: c4 _ _ _ | g4 _ _ _ D.S. al fine |",
            LineKind::Pitched,
        );
        assert_eq!(bars[1].markers[0].0, Marker::Segno);
        assert_eq!(bars[3].markers[0].0, Marker::Fine);
        assert_eq!(
            bars[4].jump.unwrap().0,
            Jump::Back {
                to_segno: true,
                al: Al::Fine
            }
        );
        assert_eq!(beats(&bars[4]).len(), 4);
        let bars = ok(
            "| c4 _ _ _ to coda | coda: g4 _ _ _ D.C. al coda |",
            LineKind::Pitched,
        );
        assert_eq!(bars[0].jump.unwrap().0, Jump::ToCoda);
        assert_eq!(bars[1].markers[0].0, Marker::Coda);
        let (_, d) = parse("| c4 segno: e g c5 |", LineKind::Pitched);
        assert_eq!(
            d[0].message,
            Message::MisplacedMarker {
                marker: "segno".into()
            }
        );
    }

    #[test]
    fn euclidean_rhythms_as_toussaint_lists_them() {
        let show = |k, n| -> String {
            euclid(k, n)
                .into_iter()
                .map(|h| if h { 'x' } else { '.' })
                .collect()
        };
        assert_eq!(show(3, 8), "x..x..x.");
        assert_eq!(show(5, 8), "x.xx.xx.");
        assert_eq!(show(2, 5), "x.x..");
        assert_eq!(show(3, 4), "xxx.");
        assert_eq!(show(7, 12), "x.xx.x.xx.x.");
        assert_eq!(show(5, 16), "x..x..x..x..x...");
        assert_eq!(show(0, 4), "....");
        assert_eq!(show(4, 4), "xxxx");
    }

    #[test]
    fn problems_are_reported_in_place() {
        let (_, d) = parse("cc [cc cc co", LineKind::Drums);
        assert_eq!(d[0].message, Message::UnclosedBracket { open: '[' });
        assert_eq!(d[0].span, Span::new(3, 4));
        let (_, d) = parse("cc qq", LineKind::Drums);
        assert_eq!(d[0].message, Message::UnknownDrum { ch: 'q' });
        let (_, d) = parse("c4 h4", LineKind::Pitched);
        assert_eq!(d[0].message, Message::UnknownNote { text: "h4".into() });
        let (_, d) = parse("c c?150% c c", LineKind::Drums);
        assert_eq!(d[0].message, Message::ChanceRange { percent: 150.0 });
        let (_, d) = parse("| c | | c |", LineKind::Drums);
        assert_eq!(d[0].message, Message::EmptyBar);
        // Repeat signs meeting at one bar line leave no bar empty.
        for src in [
            "|: c4 e4 g4 e4 :| x10 |: g4 b4 d5 b4 :| x10 |",
            "|: c4 e4 g4 e4 :| |: g4 b4 d5 b4 :|",
            "|: c4 e4 g4 e4 :||: g4 b4 d5 b4 :|",
            "|: c4 e4 g4 e4 :| x4 | g4 b4 d5 b4 |",
        ] {
            let (bars, d) = parse(src, LineKind::Pitched);
            assert!(d.is_empty(), "{src}: {d:?}");
            assert_eq!(bars.len(), 2, "{src}");
            assert!(bars[0].repeat_end.is_some(), "{src}");
        }
        let (_, d) = parse("e g", LineKind::Pitched);
        assert_eq!(d[0].message, Message::NoOctave { note: "e".into() });
        let _ = Ratio::ONE;
    }
}
