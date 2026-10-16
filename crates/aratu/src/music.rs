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
pub const DRUM_SYMBOLS: &[char] = &['c', 'o', 'b', 's', 'r', 'x', 't', 'm', 'f', 'k'];

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
    if let Some(&(open, at)) = opens.first() {
        diags.push(Diagnostic::error(
            Span::new(base + at, base + at + 1),
            Message::UnclosedBracket { open },
        ));
    }
    flush(&mut toks, &mut word_start, text.len());
    toks
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
    seg.bars
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
        if let Some(at) = text.find(" from ") {
            return rhythm_from_row(
                cx,
                diags,
                &text[..at],
                base,
                &text[at + 6..],
                base + at + 6,
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
        if w.eq_ignore_ascii_case("octave") {
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

/// What `temas.2` or `temas.all` stands for when it fills a bar: the list's
/// melodies, or, out of range, the number asked for and how many there are.
type ListFound<'l> = Result<Vec<&'l motifs::Source>, (String, usize, usize)>;

fn list_ref<'l>(word: &str, lib: &'l Library) -> Option<ListFound<'l>> {
    let (name, which) = word.split_once('.')?;
    let items = lib.lists.get(name)?;
    if which == "all" {
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
        let mut bar = read_bar(words, span, self.kind, &mut self.octave, diags, cx);
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
        beats.extend(word_beats(w, s, kind, octave, diags, cx));
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
    diags: &mut Vec<Diagnostic>,
    cx: Cx,
) -> Vec<Beat> {
    if kind == LineKind::Chords {
        return vec![Beat {
            slots: chord_beat(w, s, diags),
            span: s,
        }];
    }
    let (core, times) = motifs::split_times(w, cx.lib);
    let cs = Span::new(s.start, s.start + core.len());
    let once = match motifs::motif_ref(core, cx.lib) {
        Some((name, ops)) => motif_beats(name, &ops, cs, kind, diags, cx),
        None => {
            let mut p = SlotParser {
                s: core,
                base: cs.start,
                pos: 0,
                kind,
                octave,
                diags,
                pending: None,
                placeholders: cx.placeholders,
            };
            vec![Beat {
                slots: p.seq(None, false),
                span: cs,
            }]
        }
    };
    let mut beats = Vec::with_capacity(once.len() * times as usize);
    for _ in 0..times {
        beats.extend(once.iter().cloned());
    }
    beats
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

fn chord_beat(w: &str, s: Span, diags: &mut Vec<Diagnostic>) -> Vec<Slot> {
    match w {
        "_" => return vec![Slot::Hold(s)],
        "-" => return vec![Slot::Rest(s)],
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
            options.push(chord_beat(opt.trim(), os, diags));
        }
        return vec![Slot::Choice(options, s)];
    }
    let (mut core, staccato, fermata, chance, bad) = split_mods(w);
    if let Some(n) = bad {
        diags.push(Diagnostic::error(s, Message::ChanceRange { percent: n }));
    }
    let mut dynamic = None;
    if let Some((d, rest)) = core.split_once(':') {
        if let Some(dy) = Dyn::parse(d) {
            dynamic = Some(dy);
            core = rest.trim_start();
        }
    }
    match Chord::parse(core) {
        Some(chord) => vec![Slot::Hit(Hit {
            span: s,
            sound: Sound::Chord(chord),
            accent: false,
            staccato,
            fermata,
            roll: false,
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
                    slots.push(Slot::Hold(self.at(start)));
                }
                '-' => {
                    self.bump();
                    slots.push(Slot::Rest(self.at(start)));
                }
                '.' => {
                    self.bump();
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
                ']' | '}' | '|' => {
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

    /// A hit with its dynamic before it and its marks after it. Returns a
    /// rest in place of an unknown symbol, so the beat still counts right.
    fn hit(&mut self) -> Option<Slot> {
        let start = self.pos;
        let mut dynamic = None;
        // A word followed by a colon: a dynamic, or a misplaced marker.
        let rest = &self.s[self.pos..];
        let wlen = rest
            .char_indices()
            .find(|(_, c)| !c.is_alphabetic())
            .map_or(rest.len(), |(i, _)| i);
        if wlen > 0 && rest[wlen..].starts_with(':') {
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
            LineKind::Drums | LineKind::Strum => {
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
                if self.kind == LineKind::Drums {
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
        };

        let mut hit = Hit {
            span: Span::default(),
            sound,
            accent,
            staccato: false,
            fermata: false,
            roll: false,
            chance: None,
            hairpin: None,
            dynamic: dynamic.or(self.pending.take()),
        };
        self.mods(&mut hit);
        hit.span = self.at(start);
        Some(Slot::Hit(hit))
    }

    fn skip_mods(&mut self) {
        let mut dummy = Hit {
            span: Span::default(),
            sound: Sound::Drum('c'),
            accent: false,
            staccato: false,
            fermata: false,
            roll: false,
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
                '^' => hit.fermata = true,
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

    /// A note name with accidentals and octave: `c4`, `f#`, `eb4`, `dó4`.
    fn note(&mut self) -> Option<(i32, bool)> {
        let rest = &self.s[self.pos..];
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
