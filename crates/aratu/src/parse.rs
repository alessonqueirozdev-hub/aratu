//! Reading statements: the frame of the piece, lines, parameters, scheduled
//! changes and live commands.

use crate::ast::*;
use crate::diag::{Diagnostic, Message, Thing};
use crate::lex::{Block, Cursor, Line, Node, layout};
use crate::motifs::{self, Library, Source};
use crate::music::parse_music_with;
use crate::span::Span;
use crate::theory::{Dyn, Key, Meter};

/// Line names read as drum lines wherever they are written.
pub const DRUM_LINES: &[&str] = &[
    "hh",
    "oh",
    "bd",
    "sd",
    "rd",
    "cr",
    "cy",
    "rim",
    "cp",
    "tom",
    "perc",
    "kick",
    "snare",
    "hat",
    "hihat",
    "ride",
    "crash",
    "clap",
    "ht",
    "mt",
    "lt",
    "ft",
    "toms",
    "floor",
    "stick",
    // Portuguese
    "chimbal",
    "bumbo",
    "caixa",
    "prato",
    "pratos",
    "condução",
    "conducao",
    "ataque",
    "aro",
    "surdo",
    "tons",
    "palma",
    "palmas",
    // Spanish
    "charles",
    "bombo",
    "caja",
    "platillo",
    "platillos",
];

/// Parts whose lines are all drum lines.
pub const DRUM_PARTS: &[&str] = &["drums", "drum", "kit", "bateria", "batería"];

/// Reads a whole piece.
pub fn parse(src: &str) -> (Ast, Vec<Diagnostic>) {
    let mut diags = Vec::new();
    let nodes = layout(src, &mut diags);
    // Motifs, rows, lists and the key first, so a line may use a motif
    // defined below it.
    let mut lib = Library::default();
    collect(&nodes, &mut lib);
    let mut p = Parser {
        stmts: Vec::new(),
        diags,
        lib,
    };
    p.nodes(&nodes, &Scope::Top, None);
    let mut lists: Vec<(String, usize)> = p
        .lib
        .lists
        .iter()
        .map(|(n, items)| (n.clone(), items.len()))
        .collect();
    lists.sort();
    let mut rows: Vec<(String, Vec<i32>)> = p
        .lib
        .rows
        .iter()
        .map(|(n, r)| (n.clone(), r.clone()))
        .collect();
    rows.sort();
    (
        Ast {
            stmts: p.stmts,
            lists,
            rows,
        },
        p.diags,
    )
}

/// `motif` and `row` lines, `list` blocks and the first `key:`, wherever
/// they are written.
fn collect(nodes: &[Node], lib: &mut Library) {
    for node in nodes {
        match node {
            Node::Line(l) => {
                let mut c = Cursor::new(*l);
                if let Some((kw, name, music)) = definition(&mut c) {
                    let source = Source {
                        text: music.text.to_string(),
                        base: music.span.start,
                    };
                    if kw == "motif" {
                        lib.motifs.entry(name.to_string()).or_insert(source);
                    } else {
                        let (pitches, _, _) = motifs::read_row(music.text);
                        lib.rows.entry(name.to_string()).or_insert(pitches);
                    }
                } else if lib.key.is_none() {
                    let mut c = Cursor::new(*l);
                    if c.keyword("key") && c.eat(":") {
                        lib.key = Key::parse(c.rest().text);
                    }
                }
            }
            Node::Block(b) => {
                let mut c = Cursor::new(b.header);
                if c.keyword("list") {
                    if let Some((n, _)) = c.name() {
                        let items = b
                            .body
                            .iter()
                            .filter_map(|n| match n {
                                Node::Line(l) => Some(Source {
                                    text: l.text.to_string(),
                                    base: l.span.start,
                                }),
                                Node::Block(_) => None,
                            })
                            .collect();
                        lib.lists.entry(n.to_string()).or_insert(items);
                    }
                } else {
                    collect(&b.body, lib);
                }
            }
        }
    }
}

/// `motif riff = …` or `row serie = …`: the keyword, the name and what
/// follows `=`. A line merely called `row` is not one.
fn definition<'a>(c: &mut Cursor<'a>) -> Option<(&'static str, &'a str, Line<'a>)> {
    let save = c.clone();
    let kw = if c.keyword("motif") {
        "motif"
    } else if c.keyword("row") {
        "row"
    } else {
        return None;
    };
    match c.name() {
        Some((n, _)) if c.eat("=") => Some((kw, n, c.rest())),
        _ => {
            *c = save;
            None
        }
    }
}

struct Parser {
    stmts: Vec<Stmt>,
    diags: Vec<Diagnostic>,
    lib: Library,
}

/// What a parse step can fail with: a message at a place.
type Fail = (Span, Message);

fn expected(c: &mut Cursor, what: Thing, after: &str) -> Fail {
    let at = c.offset();
    let rest = c.rest();
    let span = if rest.text.is_empty() {
        Span::new(at.saturating_sub(1), at)
    } else {
        rest.span
    };
    (
        span,
        Message::Expected {
            what,
            after: after.to_string(),
        },
    )
}

fn name(text: &str, span: Span) -> Name {
    Name {
        text: text.to_string(),
        span,
    }
}

impl Parser {
    fn nodes(&mut self, nodes: &[Node], scope: &Scope, part: Option<&Name>) {
        for node in nodes {
            match node {
                Node::Line(l) => self.line(*l, scope, part),
                Node::Block(b) => self.block(b, scope, part),
            }
        }
    }

    fn block(&mut self, b: &Block, scope: &Scope, part: Option<&Name>) {
        let mut c = Cursor::new(b.header);
        let (first, fspan) = c.name().expect("block headers are names");
        let second = c.name();
        match (first.to_lowercase().as_str(), second) {
            ("pattern", Some((n, _))) => self.nodes(&b.body, &Scope::Pattern(n.to_string()), None),
            ("section", Some((n, _))) => self.nodes(&b.body, &Scope::Section(n.to_string()), None),
            ("list", Some(_)) => {
                // One melody per line; its mistakes are told here.
                for node in &b.body {
                    match node {
                        Node::Line(l) => {
                            crate::music::music_of(
                                &self.lib,
                                &mut self.diags,
                                l.text,
                                l.span.start,
                                LineKind::Pitched,
                            );
                        }
                        Node::Block(inner) => self
                            .diags
                            .push(Diagnostic::error(inner.header.span, Message::UnknownLine)),
                    }
                }
            }
            (_, None) => {
                if part.is_some() {
                    // A block inside a part block: read its lines as part of it.
                    self.nodes(&b.body, scope, part);
                } else {
                    self.nodes(&b.body, scope, Some(&name(first, fspan)));
                }
            }
            (_, Some(_)) => {
                let message = match crate::suggest::closest(first, ["pattern", "section", "list"]) {
                    Some(s) => Message::UnknownWord {
                        word: first.to_string(),
                        suggestion: s.to_string(),
                    },
                    None => Message::UnknownLine,
                };
                self.diags.push(Diagnostic::error(b.header.span, message));
            }
        }
    }

    fn line(&mut self, line: Line, scope: &Scope, part: Option<&Name>) {
        let mut c = Cursor::new(line);
        if let Some((kw, _, music)) = definition(&mut c) {
            if kw == "motif" {
                self.motif(music);
            } else {
                self.row(music);
            }
            return;
        }
        match self.statement(&mut c, scope, part) {
            Ok((when, kind)) => self.stmts.push(Stmt {
                scope: scope.clone(),
                part: part.cloned(),
                when,
                kind,
                span: line.span,
            }),
            Err((span, msg)) => self.diags.push(Diagnostic::error(span, msg)),
        }
    }

    fn statement(
        &mut self,
        c: &mut Cursor,
        scope: &Scope,
        part: Option<&Name>,
    ) -> Result<(When, StmtKind), Fail> {
        // When.
        let start = c.offset();
        if c.keyword("at") {
            if !c.keyword("bar") {
                return Err(expected(c, Thing::BarNumber, "at"));
            }
            let (n, ns) = c
                .integer()
                .ok_or_else(|| expected(c, Thing::BarNumber, "at bar"))?;
            if n == 0 {
                return Err((ns, Message::BarZero));
            }
            let mut said = format!("at bar {n}");
            let mut beat = None;
            if c.keyword("beat") {
                match c.number() {
                    Some((b, _)) if b >= 1.0 => {
                        said = format!("{said} beat {b}");
                        beat = Some(b);
                    }
                    _ => return Err(expected(c, Thing::Beat, &said)),
                }
            }
            if !c.eat(":") {
                return Err(expected(c, Thing::Colon, &said));
            }
            let kind = self.action(c, scope, part)?;
            let span = c.span_from(start);
            return Ok((
                match beat {
                    Some(beat) => When::AtBeat { bar: n, beat, span },
                    None => When::AtBar(n, span),
                },
                kind,
            ));
        }
        // `from bar 9 to 16:`
        {
            let save = c.clone();
            if c.keyword("from") && c.keyword("bar") {
                let (a, as_) = c
                    .integer()
                    .ok_or_else(|| expected(c, Thing::BarNumber, "from bar"))?;
                if a == 0 {
                    return Err((as_, Message::BarZero));
                }
                if !c.keyword("to") {
                    return Err(expected(c, Thing::RangeEnd, &format!("from bar {a}")));
                }
                c.keyword("bar");
                let (z, zs) = c
                    .integer()
                    .ok_or_else(|| expected(c, Thing::RangeEnd, &format!("from bar {a}")))?;
                if z < a {
                    return Err((zs, Message::BadRange { from: a, to: z }));
                }
                if !c.eat(":") {
                    return Err(expected(c, Thing::Colon, &format!("from bar {a} to {z}")));
                }
                let kind = self.action(c, scope, part)?;
                return Ok((
                    When::Range {
                        from: a,
                        to: z,
                        span: c.span_from(start),
                    },
                    kind,
                ));
            }
            *c = save;
        }
        if c.keyword("every") {
            let (n, ns) = c
                .integer()
                .ok_or_else(|| expected(c, Thing::Number, "every"))?;
            if n == 0 {
                return Err((ns, Message::BarZero));
            }
            if !(c.keyword("bars") || c.keyword("bar")) {
                return Err(expected(c, Thing::BarsWord, &format!("every {n}")));
            }
            let mut first = false;
            if c.eat(",") {
                if c.keyword("first") {
                    first = true;
                } else if !c.keyword("last") {
                    let r = c.rest();
                    return Err((
                        r.span,
                        Message::Unexpected {
                            text: r.text.to_string(),
                        },
                    ));
                }
            }
            if !c.eat(":") {
                return Err(expected(c, Thing::Colon, &format!("every {n} bars")));
            }
            let span = c.span_from(start);
            let kind = self.action(c, scope, part)?;
            return Ok((
                When::Every {
                    bars: n,
                    first,
                    span,
                },
                kind,
            ));
        }
        {
            let save = c.clone();
            if c.keyword("now") && c.eat(":") {
                let kind = self.action(c, scope, part)?;
                return Ok((When::Now, kind));
            }
            *c = save;
        }

        // The frame of the piece.
        if let Some(kind) = self.header(c)? {
            return Ok((When::Bare, kind));
        }
        Ok((When::Bare, self.action(c, scope, part)?))
    }

    /// `meter: 4/4`, `tempo: 96`, `key: A minor`, `pickup: beat 4`,
    /// `seed 7`, `form: intro verse x2`.
    fn header(&mut self, c: &mut Cursor) -> Result<Option<StmtKind>, Fail> {
        let save = c.clone();
        let Some((word, _)) = c.name() else {
            return Ok(None);
        };
        let word = word.to_lowercase();
        if word == "seed" {
            c.eat(":");
            if c.keyword("random") {
                return Ok(Some(StmtKind::Seed(Seed::Random)));
            }
            let (n, _) = c
                .integer()
                .ok_or_else(|| expected(c, Thing::Number, "seed"))?;
            self.end(c)?;
            return Ok(Some(StmtKind::Seed(Seed::Fixed(n as u64))));
        }
        if !c.eat(":") {
            *c = save;
            return Ok(None);
        }
        let kind = match word.as_str() {
            "meter" => StmtKind::Meter(meter(c)?),
            "tempo" => StmtKind::Tempo(tempo(c, "tempo:")?),
            "key" => StmtKind::Key(key(c)?),
            "pickup" => {
                if !c.keyword("beat") {
                    return Err(expected(c, Thing::Beat, "pickup:"));
                }
                let (n, ns) = c
                    .integer()
                    .ok_or_else(|| expected(c, Thing::Beat, "pickup:"))?;
                if n == 0 {
                    return Err((ns, Message::BadPickup { beat: 0, beats: 4 }));
                }
                StmtKind::Pickup(n)
            }
            "form" => {
                let mut entries = Vec::new();
                while let Some((n, s)) = c.name() {
                    let mut times = 1;
                    if let Some(t) = times_suffix(c) {
                        times = t;
                    }
                    entries.push(FormEntry {
                        section: name(n, s),
                        times,
                    });
                }
                if entries.is_empty() {
                    return Err(expected(c, Thing::Name, "form:"));
                }
                StmtKind::Form(entries)
            }
            _ => {
                *c = save;
                return Ok(None);
            }
        };
        self.end(c)?;
        Ok(Some(kind))
    }

    fn end(&mut self, c: &mut Cursor) -> Result<(), Fail> {
        if c.at_end() {
            Ok(())
        } else {
            let r = c.rest();
            Err((
                r.span,
                Message::Unexpected {
                    text: r.text.to_string(),
                },
            ))
        }
    }

    /// Everything that can also follow `at bar N:`.
    fn action(
        &mut self,
        c: &mut Cursor,
        scope: &Scope,
        part: Option<&Name>,
    ) -> Result<StmtKind, Fail> {
        let save = c.clone();
        let Some((words, pspan)) = c.path() else {
            let r = c.rest();
            return Err((r.span, Message::UnknownLine));
        };
        let first = words[0].to_lowercase();

        // `name = music` or `path = value`.
        if c.eat("=") {
            return self.assignment(c, &words, pspan, scope, part);
        }

        if words.len() == 1 {
            match first.as_str() {
                "play" => {
                    let mut entries = Vec::new();
                    loop {
                        let (n, s) = c.name().ok_or_else(|| expected(c, Thing::Name, "play"))?;
                        let times = times_suffix(c).unwrap_or(1);
                        entries.push(PlayEntry {
                            pattern: name(n, s),
                            times,
                        });
                        if !c.eat(",") {
                            break;
                        }
                    }
                    let once = c.keyword("once");
                    self.end(c)?;
                    return Ok(StmtKind::Play { entries, once });
                }
                "add" => {
                    let (n, s) = c.name().ok_or_else(|| expected(c, Thing::Name, "add"))?;
                    let once = c.keyword("once");
                    self.end(c)?;
                    return Ok(StmtKind::Add {
                        pattern: name(n, s),
                        once,
                    });
                }
                "tempo" => {
                    if c.keyword("primo") {
                        self.end(c)?;
                        return Ok(StmtKind::TempoChange(TempoChange::Primo));
                    }
                    let from = tempo(c, "tempo")?;
                    if c.eat("->") {
                        let to = tempo(c, "->")?;
                        let length = over_length(c)?;
                        self.end(c)?;
                        return Ok(StmtKind::TempoChange(TempoChange::Ramp {
                            from: Some(from),
                            to,
                            length,
                        }));
                    }
                    self.end(c)?;
                    return Ok(StmtKind::TempoChange(TempoChange::Set(from)));
                }
                "rit" | "rall" | "accel" | "ritardando" | "rallentando" | "accelerando" => {
                    if !c.keyword("to") {
                        return Err(expected(c, Thing::Number, &first));
                    }
                    let to = tempo(c, "to")?;
                    let length = over_length(c)?;
                    self.end(c)?;
                    return Ok(StmtKind::TempoChange(TempoChange::Ramp {
                        from: None,
                        to,
                        length,
                    }));
                }
                "a" if c.keyword("tempo") => {
                    self.end(c)?;
                    return Ok(StmtKind::TempoChange(TempoChange::ATempo));
                }
                "meter" => {
                    let m = meter(c)?;
                    self.end(c)?;
                    return Ok(StmtKind::MeterChange(m));
                }
                "key" => {
                    let k = key(c)?;
                    self.end(c)?;
                    return Ok(StmtKind::KeyChange(k));
                }
                "swing" | "rubato" | "humanize" => {
                    *c = save;
                    let feel = feel(c)?;
                    self.end(c)?;
                    return Ok(StmtKind::Feel { target: None, feel });
                }
                _ => {}
            }
        }

        // `bass: ...` commands addressed to a part.
        if words.len() == 1 && c.eat(":") {
            let target = name(words[0], pspan);
            return self.part_command(c, target);
        }

        // `hh.rotate 1`
        if words.len() >= 2 {
            let verb = words.last().unwrap().to_lowercase();
            if let Some(t) = self.transform(c, &verb)? {
                let tspan = Span::new(
                    pspan.start,
                    pspan.start + words[..words.len() - 1].join(".").len(),
                );
                self.end(c)?;
                return Ok(StmtKind::Transform {
                    target: name(&words[..words.len() - 1].join("."), tspan),
                    transform: t,
                });
            }
        }

        // A signal chain inside a part block: `synth.saw -> lowpass`.
        *c = save;
        if c.clone().rest().text.contains("->") {
            let units = chain(c)?;
            return Ok(StmtKind::Chain {
                target: part.cloned(),
                units,
            });
        }

        let r = c.rest();
        if let Some(s) = crate::suggest::closest(words[0], crate::suggest::KEYWORDS.iter().copied())
        {
            return Err((
                pspan,
                Message::UnknownWord {
                    word: words[0].to_string(),
                    suggestion: s.to_string(),
                },
            ));
        }
        Err((r.span, Message::UnknownLine))
    }

    fn assignment(
        &mut self,
        c: &mut Cursor,
        words: &[&str],
        pspan: Span,
        scope: &Scope,
        part: Option<&Name>,
    ) -> Result<StmtKind, Fail> {
        let is_number = matches!(c.peek(), Some(ch) if ch.is_ascii_digit() || ch == '+' || ch == '-' && c.clone().number().is_some());
        if words.len() > 1 || is_number {
            let value = value(c).ok_or_else(|| expected(c, Thing::Value, &words.join(".")))?;
            let sweep = sweep(c)?;
            self.end(c)?;
            let mut off = pspan.start;
            let path = words
                .iter()
                .map(|w| {
                    let n = name(w, Span::new(off, off + w.len()));
                    off += w.len() + 1;
                    n
                })
                .collect();
            return Ok(StmtKind::Param { path, value, sweep });
        }
        let line_name = words[0];
        let kind = line_kind(line_name, part.map(|p| p.text.as_str()));
        let music = c.rest();
        let mut variants = Vec::new();
        let bars = if let Some((bars, lengths)) = self.list_melody(music.text, music.span, kind) {
            variants = lengths;
            bars
        } else {
            crate::music::music_of(
                &self.lib,
                &mut self.diags,
                music.text,
                music.span.start,
                kind,
            )
        };
        let _ = scope;
        Ok(StmtKind::Line(LineDef {
            name: name(line_name, pspan),
            kind,
            bars,
            music_span: music.span,
            variants,
        }))
    }

    /// Checks a motif where it is written: beats with no bar lines, read as
    /// notes or, failing that, as drums.
    fn motif(&mut self, music: Line) {
        if let Some(i) = music.text.find('|') {
            let at = music.span.start + i;
            self.diags.push(Diagnostic::error(
                Span::new(at, at + 1),
                Message::MotifWithBars,
            ));
            return;
        }
        let mut notes = Vec::new();
        parse_music_with(
            music.text,
            music.span.start,
            LineKind::Pitched,
            &mut notes,
            &self.lib,
            false,
        );
        if notes
            .iter()
            .any(|d| d.severity == crate::diag::Severity::Error)
        {
            let mut drums = Vec::new();
            parse_music_with(
                music.text,
                music.span.start,
                LineKind::Drums,
                &mut drums,
                &self.lib,
                false,
            );
            if !drums
                .iter()
                .any(|d| d.severity == crate::diag::Severity::Error)
            {
                self.diags.extend(drums);
                return;
            }
        }
        self.diags.extend(notes);
    }

    /// Checks a row: every token a note, and a twelve-note row with each
    /// pitch class once.
    fn row(&mut self, music: Line) {
        let (_, bad, repeated) = motifs::read_row(music.text);
        let base = music.span.start;
        for (at, len) in bad {
            self.diags.push(Diagnostic::error(
                Span::new(base + at, base + at + len),
                Message::BadRowNote {
                    text: music.text[at..at + len].to_string(),
                },
            ));
        }
        if let Some((at, len)) = repeated {
            self.diags.push(Diagnostic::warning(
                Span::new(base + at, base + at + len),
                Message::RowRepeats {
                    note: music.text[at..at + len].to_string(),
                },
            ));
        }
    }

    /// `temas.2` or `temas.all` from a list: the melody's bars.
    fn list_melody(
        &mut self,
        text: &str,
        span: Span,
        kind: LineKind,
    ) -> Option<(Vec<Bar>, Vec<usize>)> {
        let (name, which) = text.trim().split_once('.')?;
        let items = self.lib.lists.get(name)?.clone();
        let random = which == "random";
        let chosen: Vec<&Source> = if which == "all" || random {
            items.iter().collect()
        } else {
            let index: usize = which.parse().ok()?;
            match index.checked_sub(1).and_then(|i| items.get(i)) {
                Some(item) => vec![item],
                None => {
                    self.diags.push(Diagnostic::error(
                        span,
                        Message::ListIndex {
                            name: name.to_string(),
                            index,
                            count: items.len(),
                        },
                    ));
                    return Some((Vec::new(), Vec::new()));
                }
            }
        };
        // The melodies' own mistakes are told in the list.
        let mut quiet = Vec::new();
        let mut bars = Vec::new();
        let mut lengths = Vec::new();
        for item in chosen {
            let these = crate::music::music_of(&self.lib, &mut quiet, &item.text, item.base, kind);
            lengths.push(these.len());
            bars.extend(these);
        }
        Some((bars, if random { lengths } else { Vec::new() }))
    }

    fn part_command(&mut self, c: &mut Cursor, target: Name) -> Result<StmtKind, Fail> {
        let save = c.clone();
        let arrow = c.clone().rest().text.contains("->");
        // `andamento: 120`: a header said in Portuguese or Spanish.
        if let Some(h) = crate::suggest::header_for(&target.text).filter(|_| !arrow) {
            return Err((
                target.span,
                Message::UnknownHeader {
                    word: target.text.clone(),
                    suggestion: h.to_string(),
                },
            ));
        }
        if let Some((w, _)) = c.name() {
            let w = w.to_lowercase();
            if let Some(d) = Dyn::parse(&w) {
                if c.at_end() {
                    return Ok(StmtKind::Level { target, dynamic: d });
                }
            }
            if (w == "out" || w == "in") && c.at_end() {
                return Ok(StmtKind::Mute {
                    target,
                    muted: w == "out",
                });
            }
            match w.as_str() {
                "cresc" | "crescendo" | "dim" | "decresc" | "diminuendo" => {
                    if !c.keyword("to") {
                        return Err(expected(c, Thing::Dynamic, &w));
                    }
                    let (dw, ds) = c.name().ok_or_else(|| expected(c, Thing::Dynamic, "to"))?;
                    let d = Dyn::parse(dw).ok_or((
                        ds,
                        Message::Expected {
                            what: Thing::Dynamic,
                            after: "to".into(),
                        },
                    ))?;
                    let length = over_length(c)?;
                    self.end(c)?;
                    return Ok(StmtKind::LevelRamp {
                        target,
                        to: LevelTarget::Dynamic(d),
                        length,
                    });
                }
                "fade" => {
                    let to = if c.keyword("out") {
                        LevelTarget::Silence
                    } else if c.keyword("in") {
                        LevelTarget::Dynamic(Dyn::Mf)
                    } else {
                        return Err(expected(c, Thing::Bars, "fade"));
                    };
                    let length = over_length(c)?;
                    self.end(c)?;
                    return Ok(StmtKind::LevelRamp { target, to, length });
                }
                "swing" | "rubato" | "humanize" => {
                    *c = save;
                    let feel = feel(c)?;
                    self.end(c)?;
                    return Ok(StmtKind::Feel {
                        target: Some(target),
                        feel,
                    });
                }
                _ => {}
            }
        }
        *c = save.clone();
        match chain(c) {
            Ok(units) => Ok(StmtKind::Chain {
                target: Some(target),
                units,
            }),
            // A chain gone wrong is told as it is; anything else is not a
            // command for a part, or a misspelt header.
            Err(e) if arrow => Err(e),
            Err(_) => Err(
                match crate::suggest::closest(&target.text, crate::suggest::HEADERS.iter().copied())
                {
                    Some(h) => (
                        target.span,
                        Message::UnknownHeader {
                            word: target.text.clone(),
                            suggestion: h.to_string(),
                        },
                    ),
                    None => (
                        save.clone().rest().span,
                        Message::BadPartCommand {
                            part: target.text.clone(),
                        },
                    ),
                },
            ),
        }
    }

    fn transform(&mut self, c: &mut Cursor, verb: &str) -> Result<Option<Transform>, Fail> {
        let num = |c: &mut Cursor, what: &str| -> Result<f64, Fail> {
            c.number()
                .map(|(v, _)| v)
                .ok_or_else(|| expected(c, Thing::Number, what))
        };
        Ok(Some(match verb {
            "rotate" => Transform::Rotate(num(c, verb)? as i64),
            "density" => {
                let v = num(c, verb)?;
                c.eat("%");
                Transform::Density(v)
            }
            "velocity" => Transform::Velocity(num(c, verb)?),
            "transpose" => {
                let n = num(c, verb)? as i64;
                // In semitones, as `riff+3st` says it.
                c.keyword("st");
                Transform::Transpose(n)
            }
            // The same names as motifs and rows take.
            v if crate::motifs::mirror(v).is_some() => match crate::motifs::mirror(v) {
                Some((true, true)) => Transform::RetroInvert,
                Some((true, false)) => Transform::Invert,
                _ => Transform::Retrograde,
            },
            "augment" => Transform::Augment,
            "diminish" => Transform::Diminish,
            _ => return Ok(None),
        }))
    }
}

/// How the line's symbols are read, from its name and the part it is in.
pub fn line_kind(line: &str, part: Option<&str>) -> LineKind {
    let l = line.to_lowercase();
    if l == "chords" {
        return LineKind::Chords;
    }
    if l == "strum" {
        return LineKind::Strum;
    }
    let in_drums = part.is_some_and(|p| DRUM_PARTS.contains(&p.to_lowercase().as_str()));
    if in_drums || DRUM_LINES.contains(&l.as_str()) {
        return LineKind::Drums;
    }
    LineKind::Pitched
}

fn meter(c: &mut Cursor) -> Result<Meter, Fail> {
    let r = c.clone().rest();
    let text: String = r.text.split_whitespace().next().unwrap_or("").to_string();
    if text.is_empty() {
        return Err(expected(c, Thing::Meter, "meter"));
    }
    let m = Meter::parse(&text).ok_or((
        Span::new(r.span.start, r.span.start + text.len()),
        Message::BadMeter { text: text.clone() },
    ))?;
    // Consume the meter's characters.
    let skip = text.len();
    let rest = c.rest();
    *c = Cursor::from_str(&rest.text[skip..], rest.span.start + skip);
    Ok(m)
}

fn tempo(c: &mut Cursor, after: &str) -> Result<f64, Fail> {
    let (v, s) = c
        .number()
        .ok_or_else(|| expected(c, Thing::Number, after))?;
    c.keyword("bpm");
    if !(10.0..=400.0).contains(&v) {
        return Err((s, Message::TempoOutOfRange { bpm: v }));
    }
    Ok(v)
}

fn key(c: &mut Cursor) -> Result<Key, Fail> {
    let r = c.rest();
    Key::parse(r.text).ok_or((
        r.span,
        Message::UnknownKey {
            text: r.text.to_string(),
        },
    ))
}

/// `over 4 bars`, `over 6 beats`, `over 8 eighths`: the same lengths a
/// moving parameter takes.
fn over_length(c: &mut Cursor) -> Result<Length, Fail> {
    if !c.keyword("over") {
        return Err(expected(c, Thing::Length, "to"));
    }
    length(c, "over")
}

/// What may follow a parameter's value: `-> 4000 Hz over 4 bars`; `<->`
/// in place of the arrow goes back over the same length and starts again;
/// `back over 2 beats` goes back over another length, `loop` starts again.
fn sweep(c: &mut Cursor) -> Result<Option<Sweep>, Fail> {
    let both = c.eat("<->");
    if !both && !c.eat("->") {
        return Ok(None);
    }
    let arrow = if both { "<->" } else { "->" };
    let to = value(c).ok_or_else(|| expected(c, Thing::Value, arrow))?;
    if !c.keyword("over") {
        return Err(expected(c, Thing::Length, arrow));
    }
    let rise = length(c, "over")?;
    let mut fall = both.then_some(rise);
    let mut repeat = both;
    loop {
        let save = c.clone();
        c.eat(",");
        if c.keyword("back") {
            if !c.keyword("over") {
                return Err(expected(c, Thing::Length, "back"));
            }
            fall = Some(length(c, "back over")?);
        } else if c.keyword("loop") {
            repeat = true;
        } else {
            *c = save;
            break;
        }
    }
    Ok(Some(Sweep {
        to,
        rise,
        fall,
        repeat,
    }))
}

/// `4 bars`, `2 beats`, `8 eighths`, `16 sixteenths`, after `over`.
fn length(c: &mut Cursor, after: &str) -> Result<Length, Fail> {
    let save = c.clone();
    let count = match c.number() {
        Some((n, _)) if n > 0.0 => n,
        _ => {
            *c = save;
            return Err(expected(c, Thing::Length, after));
        }
    };
    let unit = match c.name().map(|(w, _)| w.to_lowercase()).as_deref() {
        Some("bar" | "bars") => TimeUnit::Bars,
        Some("beat" | "beats") => TimeUnit::Beats,
        Some("eighth" | "eighths") => TimeUnit::Eighths,
        Some("sixteenth" | "sixteenths") => TimeUnit::Sixteenths,
        _ => {
            *c = save;
            return Err(expected(c, Thing::Length, after));
        }
    };
    Ok(Length { count, unit })
}

/// `x3` after a name, or `3x` as people also say it.
fn times_suffix(c: &mut Cursor) -> Option<u32> {
    let save = c.clone();
    if let Some((w, _)) = c.name() {
        if let Some(n) = w.strip_prefix('x').and_then(|n| n.parse::<u32>().ok()) {
            return Some(n.max(1));
        }
    }
    *c = save.clone();
    if let Some((n, _)) = c.integer() {
        if matches!(c.name(), Some((x, _)) if x.eq_ignore_ascii_case("x")) {
            return Some((n as u32).max(1));
        }
    }
    *c = save;
    None
}

fn feel(c: &mut Cursor) -> Result<Feel, Fail> {
    let (w, _) = c.name().expect("called on a feel word");
    let w = w.to_lowercase();
    match w.as_str() {
        "swing" | "rubato" => {
            let (v, _) = c.number().ok_or_else(|| expected(c, Thing::Number, &w))?;
            c.eat("%");
            Ok(if w == "swing" {
                Feel::Swing(v)
            } else {
                Feel::Rubato(v)
            })
        }
        _ => {
            let ms = match c.number() {
                Some((v, _)) => {
                    c.keyword("ms");
                    v
                }
                None => 10.0,
            };
            Ok(Feel::Humanize(ms))
        }
    }
}

/// `synth.saw -> lowpass -> reverb`
fn chain(c: &mut Cursor) -> Result<Vec<Name>, Fail> {
    let mut units = Vec::new();
    loop {
        let (p, s) = c.path().ok_or_else(|| expected(c, Thing::Name, "->"))?;
        units.push(name(&p.join("."), s));
        if !c.eat("->") {
            break;
        }
    }
    if !c.at_end() {
        let r = c.rest();
        return Err((
            r.span,
            Message::Unexpected {
                text: r.text.to_string(),
            },
        ));
    }
    Ok(units)
}

/// A number with an optional unit.
fn value(c: &mut Cursor) -> Option<Value> {
    let (number, number_span) = c.number()?;
    let mut span = number_span;
    let mut unit = None;
    if c.eat("%") {
        unit = Some("%".to_string());
        span = c.span_from(number_span.start);
    } else {
        let save = c.clone();
        match c.name() {
            // `0.2 -> 0.8 over 4 bars`: `over` is no unit.
            Some((u, us)) if !u.eq_ignore_ascii_case("over") => {
                unit = Some(u.to_string());
                span = span.join(us);
            }
            _ => *c = save,
        }
    }
    Some(Value {
        number,
        unit,
        number_span,
        span,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn one(src: &str) -> Stmt {
        let (ast, d) = parse(src);
        assert!(d.is_empty(), "{src}: {d:?}");
        assert_eq!(ast.stmts.len(), 1, "{src}");
        ast.stmts.into_iter().next().unwrap()
    }

    fn fails(src: &str) -> Message {
        let (_, d) = parse(src);
        assert!(!d.is_empty(), "{src} should fail");
        d[0].message.clone()
    }

    #[test]
    fn the_frame_of_the_piece() {
        assert_eq!(one("meter: 4/4").kind, StmtKind::Meter(Meter::COMMON));
        assert_eq!(one("tempo: 96").kind, StmtKind::Tempo(96.0));
        assert_eq!(
            one("key: A minor").kind,
            StmtKind::Key(Key {
                tonic: 9,
                minor: true
            })
        );
        assert_eq!(one("pickup: beat 4").kind, StmtKind::Pickup(4));
        assert_eq!(one("seed 7").kind, StmtKind::Seed(Seed::Fixed(7)));
        assert_eq!(one("seed: 7").kind, StmtKind::Seed(Seed::Fixed(7)));
        assert_eq!(one("seed random").kind, StmtKind::Seed(Seed::Random));
        let StmtKind::Form(f) = one("form: intro verse x2 chorus").kind else {
            panic!()
        };
        assert_eq!(f.len(), 3);
        assert_eq!(f[1].times, 2);
        assert_eq!(
            fails("meter: 4/5"),
            Message::BadMeter { text: "4/5".into() }
        );
        assert_eq!(fails("tempo: 900"), Message::TempoOutOfRange { bpm: 900.0 });
    }

    #[test]
    fn lines_by_name_and_part() {
        let s = one("bass = | a2 _ e2 _ | f2 _ c3 _ |");
        let StmtKind::Line(l) = s.kind else { panic!() };
        assert_eq!(l.kind, LineKind::Pitched);
        assert_eq!(l.bars.len(), 2);

        let (ast, d) = parse("drums {\n  hh = cc cc co cc\n  kick = b - b -\n}");
        assert!(d.is_empty());
        assert_eq!(ast.stmts[1].part.as_ref().unwrap().text, "drums");
        let StmtKind::Line(l) = &ast.stmts[1].kind else {
            panic!()
        };
        assert_eq!(l.kind, LineKind::Drums);

        let (ast, d) = parse("guitar {\n chords = | Am | F |\n strum = D du -u du\n}");
        assert!(d.is_empty());
        let StmtKind::Line(l) = &ast.stmts[0].kind else {
            panic!()
        };
        assert_eq!(l.kind, LineKind::Chords);
        let StmtKind::Line(l) = &ast.stmts[1].kind else {
            panic!()
        };
        assert_eq!(l.kind, LineKind::Strum);
    }

    #[test]
    fn scheduled_changes() {
        let s = one("at bar 30: tempo 130");
        assert!(matches!(s.when, When::AtBar(30, _)));
        assert_eq!(s.kind, StmtKind::TempoChange(TempoChange::Set(130.0)));
        assert_eq!(
            one("rit to 80 over 2 bars").kind,
            StmtKind::TempoChange(TempoChange::Ramp {
                from: None,
                to: 80.0,
                length: Length {
                    count: 2.0,
                    unit: TimeUnit::Bars
                }
            })
        );
        assert_eq!(
            one("tempo 120 -> 140 over 4 bars").kind,
            StmtKind::TempoChange(TempoChange::Ramp {
                from: Some(120.0),
                to: 140.0,
                length: Length {
                    count: 4.0,
                    unit: TimeUnit::Bars
                }
            })
        );
        assert_eq!(
            one("a tempo").kind,
            StmtKind::TempoChange(TempoChange::ATempo)
        );
        assert_eq!(
            one("tempo primo").kind,
            StmtKind::TempoChange(TempoChange::Primo)
        );
        assert_eq!(
            one("at bar 17: meter 7/8").kind,
            StmtKind::MeterChange(Meter { num: 7, den: 8 })
        );
        let s = one("every 8 bars, first: add crash");
        assert!(matches!(
            s.when,
            When::Every {
                bars: 8,
                first: true,
                ..
            }
        ));
        let s = one("every 4 bars: play fill");
        assert!(matches!(
            s.when,
            When::Every {
                bars: 4,
                first: false,
                ..
            }
        ));
        let StmtKind::Play { entries: p, once } = one("play groove x3, fill").kind else {
            panic!()
        };
        assert!(!once);
        assert!(matches!(
            one("now: play fill once").kind,
            StmtKind::Play { once: true, .. }
        ));
        assert!(matches!(
            one("add crash once").kind,
            StmtKind::Add { once: true, .. }
        ));
        assert_eq!((p[0].pattern.text.as_str(), p[0].times), ("groove", 3));
        assert_eq!((p[1].pattern.text.as_str(), p[1].times), ("fill", 1));
        assert!(matches!(one("now: play fill").when, When::Now));
        assert_eq!(fails("at bar 0: play fill"), Message::BarZero);
    }

    #[test]
    fn parameters_chains_and_part_commands() {
        let StmtKind::Param { path, value, .. } = one("bass.reverb.mix = 0.42").kind else {
            panic!()
        };
        assert_eq!(path.len(), 3);
        assert_eq!(value.number, 0.42);
        let src = "lowpass.cutoff = 800 Hz";
        let StmtKind::Param { value, .. } = one(src).kind else {
            panic!()
        };
        assert_eq!(value.unit.as_deref(), Some("Hz"));
        assert_eq!(&src[value.number_span.start..value.number_span.end], "800");

        let StmtKind::Chain { target, units } = one("bass: synth.saw -> lowpass -> reverb").kind
        else {
            panic!()
        };
        assert_eq!(target.unwrap().text, "bass");
        assert_eq!(units.len(), 3);
        assert_eq!(units[0].text, "synth.saw");

        assert!(matches!(
            one("piano: mf").kind,
            StmtKind::Level {
                dynamic: Dyn::Mf,
                ..
            }
        ));
        assert!(matches!(
            one("piano: cresc to ff over 4 bars").kind,
            StmtKind::LevelRamp {
                to: LevelTarget::Dynamic(Dyn::Ff),
                length: Length { count: 4.0, .. },
                ..
            }
        ));
        assert!(matches!(
            one("all: fade out over 8 bars").kind,
            StmtKind::LevelRamp {
                to: LevelTarget::Silence,
                length: Length { count: 8.0, .. },
                ..
            }
        ));
        assert!(matches!(
            one("piano: rubato 10%").kind,
            StmtKind::Feel {
                feel: Feel::Rubato(10.0),
                ..
            }
        ));
        assert!(matches!(
            one("swing 60%").kind,
            StmtKind::Feel {
                feel: Feel::Swing(60.0),
                ..
            }
        ));
        assert!(matches!(
            one("humanize").kind,
            StmtKind::Feel {
                feel: Feel::Humanize(_),
                ..
            }
        ));
        assert!(matches!(
            one("humanize 5 ms").kind,
            StmtKind::Feel {
                feel: Feel::Humanize(5.0),
                ..
            }
        ));
    }

    #[test]
    fn transformations() {
        let StmtKind::Transform { target, transform } = one("hh.rotate 1").kind else {
            panic!()
        };
        assert_eq!(target.text, "hh");
        assert_eq!(transform, Transform::Rotate(1));
        assert!(matches!(
            one("bass.transpose +12").kind,
            StmtKind::Transform {
                transform: Transform::Transpose(12),
                ..
            }
        ));
        assert!(matches!(
            one("hh.density 50%").kind,
            StmtKind::Transform {
                transform: Transform::Density(50.0),
                ..
            }
        ));
        assert!(matches!(
            one("melody.retrograde").kind,
            StmtKind::Transform {
                transform: Transform::Retrograde,
                ..
            }
        ));
    }

    #[test]
    fn patterns_and_sections() {
        let (ast, d) = parse("pattern groove {\n  hh = cc cc co cc\n}\nplay groove");
        assert!(d.is_empty());
        assert_eq!(ast.stmts[0].scope, Scope::Pattern("groove".into()));
        let (ast, d) = parse("section verse { bass = | a2 _ e2 _ | }");
        assert!(d.is_empty());
        assert_eq!(ast.stmts[0].scope, Scope::Section("verse".into()));
    }

    #[test]
    fn unknown_lines() {
        assert_eq!(fails("hello world"), Message::UnknownLine);
        assert_eq!(fails("123"), Message::UnknownLine);
    }
}
