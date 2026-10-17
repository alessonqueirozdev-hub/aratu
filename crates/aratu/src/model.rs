//! The piece as it will be played: parts and their lines gathered from the
//! statements, repeats unrolled, names resolved, and every bar checked.

use crate::ast::*;
use crate::diag::{Change, Diagnostic, Message};
use crate::params;
use crate::ratio::Ratio;
use crate::span::Span;
use crate::theory::{Key, Meter};
use crate::unroll::{expand_similes, unroll};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Piece {
    pub meter: Meter,
    pub tempo: f64,
    pub key: Option<Key>,
    /// The length of the pickup bar in beats; zero when there is none.
    pub pickup: Ratio,
    pub seed: Seed,
    pub parts: Vec<Part>,
    pub patterns: Vec<Group>,
    pub sections: Vec<Group>,
    /// Sections in order, by index into `sections`, with how many times.
    pub form: Vec<(usize, u32)>,
    pub commands: Vec<Command>,
    /// Parameters set outside any part, for the whole mix.
    pub params: Vec<ParamSet>,
    /// Where the number of `tempo:` is written, for its knob.
    pub tempo_span: Option<Span>,
    /// Melody lists by name, with how many melodies each has, for the
    /// editor's stage buttons.
    pub lists: Vec<(String, usize)>,
    /// Twelve-tone rows by name, for the editor's matrix.
    pub rows: Vec<(String, Vec<i32>)>,
}

/// A pattern or a section: parts prepared under a name.
#[derive(Clone, Debug)]
pub struct Group {
    pub name: String,
    pub span: Span,
    pub parts: Vec<Part>,
}

#[derive(Clone, Debug)]
pub struct Part {
    pub name: String,
    pub span: Span,
    pub lines: Vec<Line>,
    /// `synth.saw -> lowpass -> reverb`
    pub chain: Vec<String>,
    pub params: Vec<ParamSet>,
}

#[derive(Clone, Debug)]
pub struct ParamSet {
    pub id: String,
    /// The value in the parameter's own unit (Hz, ms, 0 to 1).
    pub value: f64,
    /// Where the number is written, for the knob that rewrites it.
    pub number_span: Span,
    /// The written number per unit of `value`: 0.001 for `1.2 kHz`, 100
    /// for `40%`, 1 when written in the parameter's own unit.
    pub factor: f64,
}

/// A parameter moving from its value to another over a length of time,
/// maybe back again, maybe again and again.
#[derive(Clone, Debug)]
pub struct Motion {
    /// The value it moves to, in the parameter's own unit.
    pub to: f64,
    /// Where that number is written, and the written number per unit, for
    /// its knob.
    pub to_span: Span,
    pub to_factor: f64,
    pub rise: Length,
    pub fall: Option<Length>,
    pub repeat: bool,
    /// Moves by equal ratios (frequencies, times), as the knob turns.
    pub log: bool,
}

#[derive(Clone, Debug)]
pub struct Line {
    pub name: String,
    /// How messages name the line: `bass`, `drums.hh`, `groove.hh`.
    pub label: String,
    pub kind: LineKind,
    /// The bars as written, with `%` and `%%` written out.
    pub bars: Vec<Bar>,
    /// The pickup bar, played once before bar 1.
    pub pickup: Option<usize>,
    /// The bars of one time round, in playing order, as indices into `bars`.
    pub order: Vec<usize>,
    pub span: Span,
    /// Identifies the line for chance, so that editing another line does not
    /// change how this one improvises.
    pub seed_key: u64,
    /// The velocity of every hit, by the bar's place in order (or
    /// [crate::dynamics::PICKUP]) and the hit's source offset.
    pub levels: HashMap<(usize, usize), u8>,
    /// `temas.random`: how many written bars each melody has.
    pub variants: Vec<usize>,
    /// `temas.random`: where each melody starts in `order` and how many
    /// bars it plays; one is chosen each time round.
    pub rounds: Vec<(usize, usize)>,
    /// The bar this version of the line sounds from: 1, or `n` for one
    /// written `at bar n: name = …`, which starts there from its own first
    /// bar and takes the place of the one before.
    pub from: i64,
    /// `from bar 9 to 16: name = …`: the bar it stops at, 17, where the
    /// version before it comes back.
    pub until: Option<i64>,
}

impl Line {
    /// The number of beats written in bar `i` (for a chord line, the beats
    /// of the meter, since chords share the bar).
    pub fn beats(&self, i: usize, meter: Meter) -> Ratio {
        match (&self.bars[i].content, self.kind) {
            (_, LineKind::Chords | LineKind::Arp) => meter.beats(),
            (BarContent::Beats(b), _) => Ratio::int(b.iter().map(|x| x.len as i64).sum()),
            (BarContent::Simile(_) | BarContent::Rest(_), _) => meter.beats(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Command {
    pub at: At,
    /// Where in its bar it takes effect, in beats from the bar line:
    /// `at bar 9 beat 3` is 2.
    pub beat: Ratio,
    /// `from bar 9 to 16`: the bar it stops at, 17, where what was in force
    /// before comes back.
    pub until: Option<i64>,
    pub action: Action,
    pub span: Span,
    /// The statement's text with spaces collapsed: while playing live, the
    /// editor remembers from which bar each such line has been sounding.
    pub key: String,
}

impl Command {
    /// Whether it is in force at beat `x` of bar `g`, taking effect at
    /// bar `b` (its own, or where it came in while playing live).
    pub fn in_force(&self, b: i64, g: i64, x: Ratio) -> bool {
        (b < g || (b == g && self.beat <= x)) && self.until.is_none_or(|u| g < u)
    }

    /// Whether it is in force at some time in bar `g`.
    pub fn in_bar(&self, b: i64, g: i64) -> bool {
        b <= g && self.until.is_none_or(|u| g < u)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum At {
    Bar(i64),
    Every {
        bars: i64,
        first: bool,
    },
    /// Written without a time.
    Live,
    /// `now:`
    Now,
}

#[derive(Clone, Debug)]
pub enum Action {
    Tempo(TempoChange),
    Meter(Meter),
    Key(Key),
    /// Patterns by index into `patterns`, with how many times each; with
    /// `once`, the list is played through once instead of looped.
    Play {
        list: Vec<(usize, u32)>,
        once: bool,
    },
    /// A pattern by index, on top; with `once`, for its own length only.
    Add {
        pattern: usize,
        once: bool,
    },
    Level {
        target: String,
        to: LevelTarget,
        /// How long it takes to get there; nothing for at once.
        length: Length,
    },
    /// `guitar: out` or `guitar: in`.
    Mute {
        target: String,
        muted: bool,
    },
    Feel {
        target: Option<String>,
        feel: Feel,
    },
    Param {
        part: Option<String>,
        id: String,
        value: f64,
        number_span: Span,
        factor: f64,
        /// `-> 4000 Hz over 4 bars`: from `value` to another.
        motion: Option<Motion>,
    },
    Transform {
        target: String,
        transform: Transform,
    },
    /// `pad: duck by kick`: the part's volume dips at the source's hits.
    Duck {
        target: String,
        source: DuckSource,
        amount: f64,
        length: Length,
    },
}

/// What a part ducks under.
#[derive(Clone, Debug, PartialEq)]
pub enum DuckSource {
    /// A part or a line, by its name: every sound it plays.
    Named(String),
    /// A drum by its name, from any line that plays it: `duck by kick`
    /// when the kick's line is called `bd`.
    Drum(char),
}

/// The drum a word names, for `duck by kick`, in the three languages.
pub fn drum_named(word: &str) -> Option<char> {
    Some(match crate::words::fold(word).as_str() {
        "kick" | "bd" | "bumbo" | "bombo" => 'b',
        "snare" | "sd" | "caixa" | "caja" => 's',
        "clap" | "cp" | "palma" | "palmas" => 'p',
        "hat" | "hihat" | "hh" | "chimbal" | "charles" => 'c',
        _ => return None,
    })
}

/// Parts by name in the order they first appear.
#[derive(Default)]
struct PartList {
    parts: Vec<Part>,
}

impl PartList {
    fn get(&mut self, name: &str, span: Span) -> &mut Part {
        if let Some(i) = self.parts.iter().position(|p| p.name == name) {
            return &mut self.parts[i];
        }
        self.parts.push(Part {
            name: name.to_string(),
            span,
            lines: Vec::new(),
            chain: Vec::new(),
            params: Vec::new(),
        });
        self.parts.last_mut().unwrap()
    }
}

/// The part a line written outside any part block belongs to.
fn loose_part(line: &str, kind: LineKind) -> String {
    match kind {
        LineKind::Drums => "drums".into(),
        LineKind::Chords | LineKind::Strum | LineKind::Arp => "chords".into(),
        LineKind::Pitched | LineKind::Percussion(_) => line.to_string(),
    }
}

fn fnv(text: &str) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in text.bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(0x0100_0000_01b3);
    }
    h
}

fn collapse(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The meter at each bar.
pub struct MeterMap {
    pub first: Meter,
    /// Changes by bar, in order.
    pub changes: Vec<(i64, Meter)>,
}

impl MeterMap {
    pub fn at(&self, bar: i64) -> Meter {
        self.changes
            .iter()
            .rev()
            .find(|(b, _)| *b <= bar)
            .map_or(self.first, |(_, m)| *m)
    }

    pub fn last_change(&self) -> i64 {
        self.changes.last().map_or(1, |(b, _)| *b)
    }
}

impl Piece {
    pub fn meter_map(&self) -> MeterMap {
        let mut changes: Vec<(i64, Meter)> = self
            .commands
            .iter()
            .filter_map(|c| match (c.at, &c.action) {
                (At::Bar(b), Action::Meter(m)) => Some((b, *m)),
                _ => None,
            })
            .collect();
        changes.sort_by_key(|(b, _)| *b);
        MeterMap {
            first: self.meter,
            changes,
        }
    }

    /// All lines of the piece with the part they belong to.
    pub fn all_parts(&self) -> impl Iterator<Item = &Part> {
        self.parts
            .iter()
            .chain(self.patterns.iter().flat_map(|g| g.parts.iter()))
            .chain(self.sections.iter().flat_map(|g| g.parts.iter()))
    }
}

/// Gathers the statements into a piece and checks it.
pub fn build(ast: &Ast, src: &str, diags: &mut Vec<Diagnostic>) -> Piece {
    let mut piece = Piece {
        meter: Meter::COMMON,
        tempo: 120.0,
        key: None,
        pickup: Ratio::ZERO,
        seed: Seed::Fixed(1),
        parts: Vec::new(),
        patterns: Vec::new(),
        sections: Vec::new(),
        form: Vec::new(),
        commands: Vec::new(),
        params: Vec::new(),
        tempo_span: None,
        lists: ast.lists.clone(),
        rows: ast.rows.clone(),
    };

    // The frame.
    let mut seen: HashMap<&'static str, Span> = HashMap::new();
    let mut pickup_beat: Option<(i64, Span)> = None;
    let mut form: Option<&Vec<FormEntry>> = None;
    for s in &ast.stmts {
        let word = match &s.kind {
            StmtKind::Meter(m) => {
                piece.meter = *m;
                "meter"
            }
            StmtKind::Tempo(t) => {
                piece.tempo = *t;
                // The number is the first run of digits after the colon.
                let text = &src[s.span.start..s.span.end];
                if let Some(colon) = text.find(':') {
                    let after = &text[colon..];
                    if let Some(d) = after.find(|c: char| c.is_ascii_digit()) {
                        let len = after[d..]
                            .find(|c: char| !(c.is_ascii_digit() || c == '.'))
                            .unwrap_or(after.len() - d);
                        let start = s.span.start + colon + d;
                        piece.tempo_span = Some(Span::new(start, start + len));
                    }
                }
                "tempo"
            }
            StmtKind::Key(k) => {
                piece.key = Some(*k);
                "key"
            }
            StmtKind::Pickup(b) => {
                pickup_beat = Some((*b, s.span));
                "pickup"
            }
            StmtKind::Seed(seed) => {
                piece.seed = *seed;
                "seed"
            }
            StmtKind::Form(f) => {
                form = Some(f);
                "form"
            }
            _ => continue,
        };
        if seen.insert(word, s.span).is_some() {
            diags.push(Diagnostic::warning(
                s.span,
                Message::DuplicateHeader { name: word.into() },
            ));
        }
    }
    if let Some((beat, span)) = pickup_beat {
        let beats = piece.meter.beats();
        if beat < 1 || Ratio::int(beat) > beats {
            diags.push(Diagnostic::error(
                span,
                Message::BadPickup {
                    beat,
                    beats: beats.floor(),
                },
            ));
        } else if beat > 1 {
            piece.pickup = beats - Ratio::int(beat - 1);
        }
    }

    // Parts, patterns and sections.
    let mut top = PartList::default();
    let mut groups: Vec<(Scope, Span, PartList)> = Vec::new();
    let mut global_params: Vec<(Vec<Name>, Value, Span)> = Vec::new();
    // `all: chorus -> delay`: effects every part has.
    let mut global_effects: Vec<String> = Vec::new();
    for s in &ast.stmts {
        let list = match &s.scope {
            Scope::Top => &mut top,
            scope => {
                let i = match groups.iter().position(|(g, _, _)| g == scope) {
                    Some(i) => i,
                    None => {
                        groups.push((scope.clone(), s.span, PartList::default()));
                        groups.len() - 1
                    }
                };
                &mut groups[i].2
            }
        };
        match (&s.kind, &s.when) {
            (StmtKind::Line(l), when) => {
                let (from, until) = match (when, &s.scope) {
                    (When::AtBar(n, _), Scope::Top) => (*n, None),
                    (When::Range { from, to, .. }, Scope::Top) => (*from, Some(to + 1)),
                    (When::Bare | When::Now, _) => (1, None),
                    (_, _) => {
                        diags.push(Diagnostic::error(s.span, Message::LineChangeOnlyAtBar));
                        continue;
                    }
                };
                let part_name = s
                    .part
                    .as_ref()
                    .map_or_else(|| loose_part(&l.name.text, l.kind), |p| p.text.clone());
                let part_span = s.part.as_ref().map_or(l.name.span, |p| p.span);
                let part = list.get(&part_name, part_span);
                let line = Line {
                    name: l.name.text.clone(),
                    label: String::new(),
                    kind: l.kind,
                    bars: l.bars.clone(),
                    pickup: None,
                    order: Vec::new(),
                    span: s.span,
                    seed_key: 0,
                    levels: HashMap::new(),
                    variants: l.variants.clone(),
                    rounds: Vec::new(),
                    from,
                    until,
                };
                if let Some(i) = part
                    .lines
                    .iter()
                    .position(|x| x.name == line.name && x.from == from)
                {
                    diags.push(Diagnostic::warning(
                        l.name.span,
                        Message::DuplicateLine {
                            name: line.name.clone(),
                        },
                    ));
                    part.lines[i] = line;
                } else {
                    part.lines.push(line);
                }
            }
            (
                StmtKind::Param {
                    path,
                    value,
                    sweep: None,
                },
                When::Bare,
            ) => match &s.part {
                Some(p) => {
                    let id = path
                        .iter()
                        .map(|n| n.text.as_str())
                        .collect::<Vec<_>>()
                        .join(".");
                    if let Some((v, factor)) = param_value(&id, value, diags) {
                        list.get(&p.text, p.span).params.push(ParamSet {
                            id,
                            value: v,
                            number_span: value.number_span,
                            factor,
                        });
                    }
                }
                None => global_params.push((path.clone(), value.clone(), s.span)),
            },
            (StmtKind::Chain { target, units }, _) => {
                // The first unit may be a voice; every unit must be one or
                // an effect.
                for (i, u) in units.iter().enumerate() {
                    let known = crate::voices::EFFECTS.contains(&u.text.as_str())
                        || (i == 0 && crate::voices::find(&u.text).is_some());
                    if !known {
                        diags.push(Diagnostic::warning(
                            u.span,
                            Message::UnknownSound {
                                name: u.text.clone(),
                                closest: crate::voices::closest(&u.text).map(String::from),
                            },
                        ));
                    }
                }
                let target = target.as_ref().or(s.part.as_ref());
                match target {
                    Some(t) if t.text == "all" => {
                        global_effects = units
                            .iter()
                            .map(|u| u.text.clone())
                            .filter(|u| crate::voices::EFFECTS.contains(&u.as_str()))
                            .collect();
                    }
                    Some(t) => {
                        list.get(&t.text, t.span).chain =
                            units.iter().map(|u| u.text.clone()).collect();
                    }
                    None => {}
                }
            }
            _ => {}
        }
    }

    // Parameters written as `bass.reverb.mix = 0.42` outside the block, for
    // a part of the piece or of its patterns and sections.
    for (path, value, _) in global_params {
        let names: Vec<&str> = top
            .parts
            .iter()
            .chain(groups.iter().flat_map(|(_, _, l)| l.parts.iter()))
            .map(|p| p.name.as_str())
            .collect();
        let part = (path.len() > 1)
            .then(|| part_named(&names, &path[0].text))
            .flatten();
        let id_path = if part.is_some() {
            &path[1..]
        } else {
            &path[..]
        };
        let id = id_path
            .iter()
            .map(|n| n.text.as_str())
            .collect::<Vec<_>>()
            .join(".");
        if let Some((v, factor)) = param_value(&id, &value, diags) {
            let set = ParamSet {
                id,
                value: v,
                number_span: value.number_span,
                factor,
            };
            match part {
                Some(p) => top.get(&p, path[0].span).params.push(set),
                None => piece.params.push(set),
            }
        }
    }

    // A part set up outside the patterns and sections it plays in (its
    // numbers, its chain) is set up so wherever it plays; a chain written
    // for it inside stays.
    for (_, _, list) in &mut groups {
        for part in &mut list.parts {
            let Some(outside) = top.parts.iter().find(|p| p.name == part.name) else {
                continue;
            };
            part.params.extend(outside.params.iter().cloned());
            if part.chain.is_empty() {
                part.chain = outside.chain.clone();
            }
        }
    }
    // Set up outside with no lines there, it plays only inside them.
    top.parts.retain(|p| {
        !p.lines.is_empty()
            || !groups
                .iter()
                .any(|(_, _, l)| l.parts.iter().any(|q| q.name == p.name))
    });

    piece.parts = top.parts;
    for (scope, span, list) in groups {
        let (name, is_pattern) = match scope {
            Scope::Pattern(n) => (n, true),
            Scope::Section(n) => (n, false),
            Scope::Top => unreachable!(),
        };
        let g = Group {
            name,
            span,
            parts: list.parts,
        };
        if is_pattern {
            piece.patterns.push(g);
        } else {
            piece.sections.push(g);
        }
    }
    // The effects written for all go on every part's chain, after its own.
    if !global_effects.is_empty() {
        let groups = piece.patterns.iter_mut().chain(piece.sections.iter_mut());
        let parts = piece
            .parts
            .iter_mut()
            .chain(groups.flat_map(|g| g.parts.iter_mut()));
        for part in parts {
            for e in &global_effects {
                if !part.chain.contains(e) {
                    part.chain.push(e.clone());
                }
            }
        }
    }

    // Timed and live commands.
    for s in &ast.stmts {
        let (at, beat, until) = match s.when {
            When::Bare => (At::Live, Ratio::ZERO, None),
            When::AtBar(b, _) => (At::Bar(b), Ratio::ZERO, None),
            When::AtBeat { bar, beat, .. } => (
                At::Bar(bar),
                Ratio::new(((beat - 1.0) * 1920.0).round() as i64, 1920),
                None,
            ),
            When::Range { from, to, .. } => (At::Bar(from), Ratio::ZERO, Some(to + 1)),
            When::Every { bars, first, .. } => (At::Every { bars, first }, Ratio::ZERO, None),
            When::Now => (At::Now, Ratio::ZERO, None),
        };
        // Tempo, meter and key hold until the next change; meter and key
        // change at a bar line.
        let frame = matches!(
            s.kind,
            StmtKind::TempoChange(_) | StmtKind::MeterChange(_) | StmtKind::KeyChange(_)
        );
        if frame && until.is_some() {
            diags.push(Diagnostic::error(s.span, Message::FrameInRange));
            continue;
        }
        if matches!(s.kind, StmtKind::MeterChange(_) | StmtKind::KeyChange(_)) && beat > Ratio::ZERO
        {
            diags.push(Diagnostic::error(s.span, Message::FrameAtBeat));
            continue;
        }
        let action = match &s.kind {
            StmtKind::TempoChange(t) => Action::Tempo(*t),
            StmtKind::MeterChange(m) => Action::Meter(*m),
            StmtKind::KeyChange(k) => Action::Key(*k),
            StmtKind::Play { entries, once } => {
                let mut list = Vec::new();
                for e in entries {
                    match piece.patterns.iter().position(|g| g.name == e.pattern.text) {
                        Some(i) => list.push((i, e.times)),
                        None => diags.push(Diagnostic::error(
                            e.pattern.span,
                            Message::UnknownPattern {
                                name: e.pattern.text.clone(),
                            },
                        )),
                    }
                }
                if list.is_empty() {
                    continue;
                }
                Action::Play { list, once: *once }
            }
            StmtKind::Add { pattern: n, once } => {
                match piece.patterns.iter().position(|g| g.name == n.text) {
                    Some(i) => Action::Add {
                        pattern: i,
                        once: *once,
                    },
                    None => {
                        diags.push(Diagnostic::error(
                            n.span,
                            Message::UnknownPattern {
                                name: n.text.clone(),
                            },
                        ));
                        continue;
                    }
                }
            }
            StmtKind::Level { target, dynamic } => Action::Level {
                target: target_of(&piece, target, diags),
                to: LevelTarget::Dynamic(*dynamic),
                length: Length {
                    count: 0.0,
                    unit: TimeUnit::Bars,
                },
            },
            StmtKind::Mute { target, muted } => Action::Mute {
                target: target_of(&piece, target, diags),
                muted: *muted,
            },
            StmtKind::LevelRamp { target, to, length } => Action::Level {
                target: target_of(&piece, target, diags),
                to: *to,
                length: *length,
            },
            StmtKind::Feel { target, feel } => Action::Feel {
                target: target.as_ref().map(|t| target_of(&piece, t, diags)),
                feel: *feel,
            },
            StmtKind::Transform { target, transform } => {
                let known = piece.all_parts().any(|p| {
                    p.name == target.text
                        || p.lines.iter().any(|l| {
                            l.name == target.text || format!("{}.{}", p.name, l.name) == target.text
                        })
                }) || piece.patterns.iter().any(|g| g.name == target.text);
                if !known {
                    diags.push(Diagnostic::warning(
                        target.span,
                        Message::UnknownTarget {
                            name: target.text.clone(),
                        },
                    ));
                }
                Action::Transform {
                    target: target.text.clone(),
                    transform: *transform,
                }
            }
            StmtKind::Duck {
                target,
                source,
                amount,
                length,
            } => Action::Duck {
                target: target_of(&piece, target, diags),
                source: duck_source(&piece, source, diags),
                amount: *amount,
                length: *length,
            },
            // A parameter that moves is a command even without a time: it
            // starts moving where it is written while playing live.
            StmtKind::Param { path, value, sweep } if s.when != When::Bare || sweep.is_some() => {
                let names: Vec<&str> = piece.all_parts().map(|p| p.name.as_str()).collect();
                let (part, id_path) = match &s.part {
                    Some(p) => (Some(p.text.clone()), &path[..]),
                    None if path.len() > 1 && part_named(&names, &path[0].text).is_some() => {
                        (part_named(&names, &path[0].text), &path[1..])
                    }
                    None => (None, &path[..]),
                };
                let id = id_path
                    .iter()
                    .map(|n| n.text.as_str())
                    .collect::<Vec<_>>()
                    .join(".");
                let Some((v, factor)) = param_value(&id, value, diags) else {
                    continue;
                };
                let motion = match sweep {
                    None => None,
                    Some(sw) => {
                        let Some((to, to_factor)) = param_value(&id, &sw.to, diags) else {
                            continue;
                        };
                        Some(Motion {
                            to,
                            to_span: sw.to.number_span,
                            to_factor,
                            rise: sw.rise,
                            fall: sw.fall,
                            repeat: sw.repeat,
                            log: params::lookup(&id).is_some_and(|i| i.scale == params::Scale::Log),
                        })
                    }
                };
                Action::Param {
                    part,
                    id,
                    value: v,
                    number_span: value.number_span,
                    factor,
                    motion,
                }
            }
            _ => continue,
        };
        let text = &src[s.span.start..s.span.end];
        // A parameter that moves keeps where it is in its motion while its
        // numbers change, as when a knob turns: only what it moves counts.
        let key = match &action {
            Action::Param {
                motion: Some(_), ..
            } => format!("{} = ->", collapse(text.split('=').next().unwrap_or(text))),
            _ => collapse(text),
        };
        piece.commands.push(Command {
            at,
            beat,
            until,
            action,
            span: s.span,
            key,
        });
    }

    // A beat the bar does not have.
    let meters = piece.meter_map();
    for c in &piece.commands {
        let At::Bar(bar) = c.at else {
            continue;
        };
        let beats = meters.at(bar).beats();
        if c.beat >= beats {
            diags.push(Diagnostic::error(
                c.span,
                Message::BeatOutOfBar {
                    bar,
                    beat: c.beat.to_f64() + 1.0,
                    beats,
                },
            ));
        }
    }
    piece.commands.retain(|c| match c.at {
        At::Bar(bar) => c.beat < meters.at(bar).beats(),
        _ => true,
    });

    // Two changes of the same kind at one bar.
    let mut at_bar: HashMap<(i64, u8), Span> = HashMap::new();
    for c in &piece.commands {
        let (At::Bar(b), kind) = (c.at, &c.action) else {
            continue;
        };
        let (k, what) = match kind {
            Action::Tempo(_) => (0, Change::Tempo),
            Action::Meter(_) => (1, Change::Meter),
            Action::Key(_) => (2, Change::Key),
            _ => continue,
        };
        if at_bar.insert((b, k), c.span).is_some() {
            diags.push(Diagnostic::warning(
                c.span,
                Message::SameBarChange { bar: b, what },
            ));
        }
    }

    if let Some(entries) = form {
        for e in entries {
            match piece.sections.iter().position(|g| g.name == e.section.text) {
                Some(i) => piece.form.push((i, e.times)),
                None => diags.push(Diagnostic::error(
                    e.section.span,
                    Message::UnknownSection {
                        name: e.section.text.clone(),
                    },
                )),
            }
        }
    }

    // Unroll and check every line.
    let meters = piece.meter_map();
    let pickup = piece.pickup;
    let check = |group: Option<&str>, parts: &mut Vec<Part>, diags: &mut Vec<Diagnostic>| {
        for part in parts.iter_mut() {
            let has_chords = part.lines.iter().any(|l| l.kind == LineKind::Chords);
            for line in part.lines.iter_mut() {
                let mut label = Vec::new();
                label.extend(group);
                if part.name != line.name {
                    label.push(part.name.as_str());
                }
                label.push(line.name.as_str());
                line.label = label.join(".");
                line.seed_key = fnv(&line.label);
                prepare_line(line, &meters, pickup, group.is_none(), diags);
                if line.kind == LineKind::Strum && !has_chords {
                    diags.push(Diagnostic::error(
                        line.span,
                        Message::StrumWithoutChords {
                            part: part.name.clone(),
                        },
                    ));
                }
                if line.kind == LineKind::Arp && !has_chords {
                    diags.push(Diagnostic::error(
                        line.span,
                        Message::ArpWithoutChords {
                            part: part.name.clone(),
                        },
                    ));
                }
            }
        }
    };
    check(None, &mut piece.parts, diags);
    for g in piece.patterns.iter_mut().chain(piece.sections.iter_mut()) {
        check(Some(&g.name.clone()), &mut g.parts, diags);
    }
    piece
}

/// The part a command is for, with a warning when there is none of that
/// name; a drum part may be called by its name in any language.
fn target_of(piece: &Piece, target: &Name, diags: &mut Vec<Diagnostic>) -> String {
    if target.text == "all" {
        return target.text.clone();
    }
    let names: Vec<&str> = piece.all_parts().map(|p| p.name.as_str()).collect();
    if let Some(n) = part_named(&names, &target.text) {
        return n;
    }
    // `hh: out` names a line; out, in and levels are for its part.
    let owner = piece
        .all_parts()
        .find(|p| p.lines.iter().any(|l| l.name == target.text));
    let message = match owner {
        Some(p) => Message::LineNotPart {
            line: target.text.clone(),
            part: p.name.clone(),
        },
        None => Message::UnknownTarget {
            name: target.text.clone(),
        },
    };
    diags.push(Diagnostic::warning(target.span, message));
    target.text.clone()
}

/// What `duck by …` names: a part or a line of the piece, else a drum by
/// its name, with a warning when it is neither.
fn duck_source(piece: &Piece, source: &Name, diags: &mut Vec<Diagnostic>) -> DuckSource {
    let names: Vec<&str> = piece.all_parts().map(|p| p.name.as_str()).collect();
    if let Some(n) = part_named(&names, &source.text) {
        return DuckSource::Named(n);
    }
    let line = piece.all_parts().any(|p| {
        p.lines
            .iter()
            .any(|l| l.name == source.text || format!("{}.{}", p.name, l.name) == source.text)
    });
    if line {
        return DuckSource::Named(source.text.clone());
    }
    if let Some(d) = drum_named(&source.text) {
        return DuckSource::Drum(d);
    }
    diags.push(Diagnostic::warning(
        source.span,
        Message::UnknownTarget {
            name: source.text.clone(),
        },
    ));
    DuckSource::Named(source.text.clone())
}

/// The part a name stands for among `names`: the part of that name, or,
/// for a drum part's name in any language (`drums`, `bateria`, `kit`),
/// the drum part there is.
fn part_named(names: &[&str], name: &str) -> Option<String> {
    if names.contains(&name) {
        return Some(name.to_string());
    }
    let drums = |n: &str| crate::parse::DRUM_PARTS.contains(&n.to_lowercase().as_str());
    if drums(name) {
        return names.iter().find(|n| drums(n)).map(|n| n.to_string());
    }
    None
}

/// The value of a parameter in its own unit and the factor back to the
/// written number, with warnings for an unknown parameter, a unit that does
/// not fit, or a value out of range.
fn param_value(id: &str, value: &Value, diags: &mut Vec<Diagnostic>) -> Option<(f64, f64)> {
    let Some(info) = params::lookup(id) else {
        diags.push(Diagnostic::warning(
            value.span,
            Message::UnknownParam {
                name: id.to_string(),
                closest: params::closest(id).map(String::from),
            },
        ));
        return Some((value.number, 1.0));
    };
    let factor = params::normalize(1.0, value.unit.as_deref(), info).map_or(1.0, |m| 1.0 / m);
    let Some(v) = params::normalize(value.number, value.unit.as_deref(), info) else {
        diags.push(Diagnostic::warning(
            value.span,
            Message::BadUnit {
                unit: value.unit.clone().unwrap_or_default(),
                name: id.to_string(),
            },
        ));
        return None;
    };
    if v < info.min || v > info.max {
        diags.push(Diagnostic::warning(
            value.span,
            Message::ParamOutOfRange {
                name: id.to_string(),
                value: value.number,
                min: info.min,
                max: info.max,
            },
        ));
    }
    Some((v.clamp(info.min, info.max), factor))
}

/// Writes out similes, finds the pickup, unrolls the line and checks that
/// each bar fits its meter.
fn prepare_line(
    line: &mut Line,
    meters: &MeterMap,
    pickup: Ratio,
    from_bar_one: bool,
    diags: &mut Vec<Diagnostic>,
) {
    line.bars = expand_similes(&line.bars, diags);
    if line.bars.is_empty() {
        return;
    }
    // A Euclidean rhythm alone in its bar spreads over the whole bar.
    for bar in &mut line.bars {
        fill_bar(bar, meters.first);
    }
    let first_beats = line.beats(0, meters.first);
    let has_pickup = !pickup.is_zero()
        && !matches!(line.kind, LineKind::Chords | LineKind::Arp)
        && first_beats == pickup
        && line.bars.len() > 1;
    let has_pickup = has_pickup && line.variants.is_empty();
    let start = usize::from(has_pickup);
    if has_pickup {
        line.pickup = Some(0);
    }
    if line.variants.is_empty() {
        line.order = unroll(&line.bars[start..], &line.label, 1, diags)
            .into_iter()
            .map(|i| i + start)
            .collect();
    } else {
        // Each melody of the list unrolled on its own, so one can be chosen
        // each time round.
        let mut first = 0;
        line.order.clear();
        line.rounds.clear();
        for &n in &line.variants.clone() {
            let end = (first + n).min(line.bars.len());
            let order: Vec<usize> = unroll(&line.bars[first..end], &line.label, 1, diags)
                .into_iter()
                .map(|i| i + first)
                .collect();
            line.rounds.push((line.order.len(), order.len()));
            line.order.extend(order);
            first = end;
        }
    }
    if line.order.is_empty() {
        return;
    }

    // `7/8 = 2+2+3`: a bar written one word per group (`cc cc ccc`)
    // gives each word its group's length.
    let order = line.order.clone();
    for (k, &i) in order.iter().enumerate() {
        let meter = if from_bar_one {
            meters.at(1 + k as i64)
        } else {
            meters.first
        };
        let groups = meter.groups();
        fill_bar(&mut line.bars[i], meter);
        if let BarContent::Beats(beats) = &mut line.bars[i].content {
            if !groups.is_empty()
                && line.kind != LineKind::Chords
                && beats.len() == groups.len()
                && beats.iter().all(|b| b.len == 1)
            {
                for (b, g) in beats.iter_mut().zip(groups) {
                    b.len = u32::from(*g);
                }
            }
        }
    }

    // Each bar against the meter of the bar where it sounds. Lines of
    // patterns and sections are checked against the meter at their start.
    let len = line.order.len() as i64;
    let passes = if from_bar_one {
        ((meters.last_change() - 1) / len + 1).max(1)
    } else {
        1
    };
    let mut reported = vec![false; line.bars.len()];
    for pass in 0..passes {
        for (k, &i) in line.order.iter().enumerate() {
            let bar = 1 + pass * len + k as i64;
            let meter = if from_bar_one {
                meters.at(bar)
            } else {
                meters.first
            };
            if reported[i] {
                continue;
            }
            if let Some(msg) = bar_fits(line, i, meter, i - start + 1) {
                diags.push(Diagnostic::error(line.bars[i].span, msg));
                reported[i] = true;
            }
        }
    }
    line.levels = crate::dynamics::levels(line, diags);
    if let Some(p) = line.pickup {
        if line.beats(p, meters.first) != pickup {
            diags.push(Diagnostic::error(
                line.bars[p].span,
                Message::PickupBeats {
                    line: line.label.clone(),
                    found: line.beats(p, meters.first),
                    expected: pickup,
                },
            ));
        }
    }
}

/// Gives a bar that is a Euclidean rhythm alone the length of the meter.
fn fill_bar(bar: &mut Bar, meter: Meter) {
    if let BarContent::Beats(beats) = &mut bar.content {
        if let [only] = beats.as_mut_slice() {
            if only.fill {
                only.len = meter.beats().floor().max(1) as u32;
            }
        }
    }
}

/// Why bar `i` does not fit `meter`, if it does not.
fn bar_fits(line: &Line, i: usize, meter: Meter, number: usize) -> Option<Message> {
    let BarContent::Beats(beats) = &line.bars[i].content else {
        return None;
    };
    // An arpeggiator's steps fill whatever bar it plays in.
    if line.kind == LineKind::Arp {
        return None;
    }
    if line.kind == LineKind::Chords {
        // Each chord must start on a beat.
        let n = beats.len() as i64;
        let total = meter.beats();
        let share = total / Ratio::int(n);
        for (k, b) in beats.iter().enumerate() {
            let starts_chord = !matches!(b.slots.as_slice(), [Slot::Hold(_)]);
            if starts_chord && !(share * Ratio::int(k as i64)).is_integer() {
                return Some(Message::ChordsOffBeat {
                    line: line.label.clone(),
                    bar: number,
                    chords: beats
                        .iter()
                        .filter(|b| !matches!(b.slots.as_slice(), [Slot::Hold(_)]))
                        .count(),
                    beats: total,
                });
            }
        }
        return None;
    }
    let found = Ratio::int(beats.iter().map(|b| b.len as i64).sum());
    (found != meter.beats()).then(|| Message::BeatCount {
        line: line.label.clone(),
        bar: number,
        found,
        expected: meter.beats(),
        meter: meter.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diag::Lang;
    use crate::parse::parse;

    fn build_src(src: &str) -> (Piece, Vec<Diagnostic>) {
        let (ast, mut d) = parse(src);
        let p = build(&ast, src, &mut d);
        (p, d)
    }

    fn errors(src: &str) -> Vec<Message> {
        let (_, d) = build_src(src);
        d.into_iter().map(|d| d.message).collect()
    }

    #[test]
    fn the_first_piece_of_the_manual() {
        let src = "meter: 4/4\ntempo: 96\nkey: A minor\n\ndrums {\n  hh = cc cc co cc\n  bd = b - b -\n  sd = - s - s\n}\n\nbass = | a2 _ e2 _ | f2 _ c3 _ |\n\nguitar {\n  chords = | Am | F | C G | G |\n  strum  = D du -u du\n}\n";
        let (p, d) = build_src(src);
        assert!(d.is_empty(), "{d:?}");
        assert_eq!(p.tempo, 96.0);
        let names: Vec<&str> = p.parts.iter().map(|p| p.name.as_str()).collect();
        assert_eq!(names, ["drums", "bass", "guitar"]);
        assert_eq!(p.parts[0].lines[0].label, "drums.hh");
        assert_eq!(p.parts[1].lines[0].label, "bass");
        assert_eq!(p.parts[2].lines[0].order, [0, 1, 2, 3]);
    }

    #[test]
    fn a_bar_with_the_wrong_number_of_beats() {
        let src = "meter: 4/4\nbass = | a2 _ e2 _ | f2 _ c3 _ g2 |";
        let (_, d) = build_src(src);
        assert_eq!(
            d[0].message,
            Message::BeatCount {
                line: "bass".into(),
                bar: 2,
                found: Ratio::int(5),
                expected: Ratio::int(4),
                meter: "4/4".into()
            }
        );
        assert_eq!(
            d[0].text(Lang::Pt),
            "o compasso 2 de `bass` tem 5 tempos, mas a fórmula de compasso é 4/4"
        );
        assert_eq!(
            d[0].text(Lang::En),
            "bar 2 of `bass` has 5 beats, but the meter is 4/4"
        );
        assert_eq!(
            d[0].text(Lang::Es),
            "el compás 2 de `bass` tiene 5 tiempos, pero la indicación de compás es 4/4"
        );
    }

    #[test]
    fn compound_and_changing_meters() {
        assert!(errors("meter: 6/8\nhh = ccc ccc").is_empty());
        assert!(matches!(
            errors("meter: 6/8\nhh = c c c c c c")[0],
            Message::BeatCount { .. }
        ));
        // Bar 3 of the line sounds at bar 3, where the meter is 3/4.
        let e = errors("at bar 3: meter 3/4\nbass = | c4 _ _ _ | c4 _ _ _ | c4 _ _ _ |");
        assert!(matches!(&e[0], Message::BeatCount { bar: 3, .. }));
        assert!(errors("at bar 3: meter 3/4\nbass = | c4 _ _ _ | c4 _ _ _ | c4 _ _ |").is_empty());
    }

    #[test]
    fn pickups() {
        let src = "meter: 4/4\npickup: beat 4\nmelody = | g4 | c5 _ _ _ | g4 _ _ _ |\nbass = | c3 _ _ _ |";
        let (p, d) = build_src(src);
        assert!(d.is_empty(), "{d:?}");
        assert_eq!(p.pickup, Ratio::ONE);
        assert_eq!(p.parts[0].lines[0].pickup, Some(0));
        assert_eq!(p.parts[0].lines[0].order, [1, 2]);
        assert_eq!(p.parts[1].lines[0].pickup, None);
        assert!(matches!(
            errors("pickup: beat 5")[0],
            Message::BadPickup { beat: 5, beats: 4 }
        ));
    }

    #[test]
    fn chords_must_fall_on_beats() {
        assert!(errors("chords = | Am _ F G |").is_empty());
        assert!(errors("chords = | C G |").is_empty());
        assert!(matches!(
            errors("meter: 3/4\nchords = | C G |")[0],
            Message::ChordsOffBeat { chords: 2, .. }
        ));
        assert!(matches!(
            errors("chords = | Am F G |")[0],
            Message::ChordsOffBeat { .. }
        ));
    }

    #[test]
    fn names_that_point_nowhere() {
        assert!(
            matches!(&errors("play groove")[0], Message::UnknownPattern { name } if name == "groove")
        );
        assert!(matches!(
            &errors("form: intro verse")[0],
            Message::UnknownSection { .. }
        ));
        assert!(matches!(
            &errors("guitar { strum = D du -u du }")[0],
            Message::StrumWithoutChords { .. }
        ));
        let (_, d) = build_src("piano: mf");
        assert_eq!(d[0].severity, crate::diag::Severity::Warning);
    }

    #[test]
    fn patterns_sections_and_loose_lines() {
        let src = "pattern groove { hh = cc cc co cc; bd = b - b - }\nchords = | Am | F |\nstrum = D du -u du\nplay groove";
        let (p, d) = build_src(src);
        assert!(d.is_empty(), "{d:?}");
        assert_eq!(p.patterns[0].parts[0].name, "drums");
        assert_eq!(p.patterns[0].parts[0].lines[0].label, "groove.drums.hh");
        assert_eq!(p.parts[0].name, "chords");
        assert_eq!(p.parts[0].lines.len(), 2);
        assert!(
            matches!(p.commands[0].action, Action::Play { ref list, once: false } if list == &[(0, 1)])
        );
    }

    #[test]
    fn parameters() {
        let src = "bass = | c2 _ _ _ |\nbass { lowpass.cutoff = 1.2 kHz }\nbass.reverb.mix = 40%\nat bar 16: bass.reverb.mix = 0.65";
        let (p, d) = build_src(src);
        assert!(d.is_empty(), "{d:?}");
        let params = &p.parts[0].params;
        assert_eq!(
            (params[0].id.as_str(), params[0].value),
            ("lowpass.cutoff", 1200.0)
        );
        assert_eq!(
            (params[1].id.as_str(), params[1].value),
            ("reverb.mix", 0.4)
        );
        assert!(matches!(
            &p.commands[0].action,
            Action::Param { part: Some(b), id, value, .. } if b == "bass" && id == "reverb.mix" && *value == 0.65
        ));
        let (_, d) = build_src("bass { wobble = 3 }");
        assert!(matches!(d[0].message, Message::UnknownParam { .. }));
        let (_, d) = build_src("bass { reverb.mix = 3 }");
        assert!(matches!(d[0].message, Message::ParamOutOfRange { .. }));
    }

    #[test]
    fn same_bar_changes_warn() {
        let (_, d) = build_src("at bar 5: tempo 100\nat bar 5: tempo 110");
        assert!(matches!(
            d[0].message,
            Message::SameBarChange {
                bar: 5,
                what: Change::Tempo
            }
        ));
        assert_eq!(
            d[0].text(Lang::Pt),
            "duas mudanças de andamento no compasso 5; vale a última"
        );
    }
}
