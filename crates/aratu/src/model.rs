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
    pub value: f64,
    pub number_span: Span,
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
}

impl Line {
    /// The number of beats written in bar `i` (for a chord line, the beats
    /// of the meter, since chords share the bar).
    pub fn beats(&self, i: usize, meter: Meter) -> Ratio {
        match (&self.bars[i].content, self.kind) {
            (_, LineKind::Chords) => meter.beats(),
            (BarContent::Beats(b), _) => Ratio::int(b.len() as i64),
            (BarContent::Simile(_) | BarContent::Rest(_), _) => meter.beats(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Command {
    pub at: At,
    pub action: Action,
    pub span: Span,
    /// The statement's text with spaces collapsed: while playing live, the
    /// editor remembers from which bar each such line has been sounding.
    pub key: String,
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
    /// Patterns by index into `patterns`, with how many times each.
    Play(Vec<(usize, u32)>),
    Add(usize),
    Level {
        target: String,
        to: LevelTarget,
        bars: i64,
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
    },
    Transform {
        target: String,
        transform: Transform,
    },
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
        LineKind::Chords | LineKind::Strum => "chords".into(),
        LineKind::Pitched => line.to_string(),
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
            (StmtKind::Line(l), _) => {
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
                };
                if let Some(i) = part.lines.iter().position(|x| x.name == line.name) {
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
            (StmtKind::Param { path, value }, When::Bare) => match &s.part {
                Some(p) => {
                    let id = path
                        .iter()
                        .map(|n| n.text.as_str())
                        .collect::<Vec<_>>()
                        .join(".");
                    if let Some(v) = param_value(&id, value, diags) {
                        list.get(&p.text, p.span).params.push(ParamSet {
                            id,
                            value: v,
                            number_span: value.number_span,
                        });
                    }
                }
                None => global_params.push((path.clone(), value.clone(), s.span)),
            },
            (StmtKind::Chain { target, units }, _) => {
                let target = target.as_ref().or(s.part.as_ref());
                if let Some(t) = target {
                    list.get(&t.text, t.span).chain =
                        units.iter().map(|u| u.text.clone()).collect();
                }
            }
            _ => {}
        }
    }

    // Parameters written as `bass.reverb.mix = 0.42` outside the block.
    for (path, value, _) in global_params {
        let first = &path[0].text;
        let is_part = path.len() > 1 && top.parts.iter().any(|p| &p.name == first);
        let (part, id_path) = if is_part {
            (Some(&path[0]), &path[1..])
        } else {
            (None, &path[..])
        };
        let id = id_path
            .iter()
            .map(|n| n.text.as_str())
            .collect::<Vec<_>>()
            .join(".");
        if let Some(v) = param_value(&id, &value, diags) {
            let set = ParamSet {
                id,
                value: v,
                number_span: value.number_span,
            };
            match part {
                Some(p) => top.get(&p.text, p.span).params.push(set),
                None => piece.params.push(set),
            }
        }
    }

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

    // Timed and live commands.
    for s in &ast.stmts {
        let at = match s.when {
            When::Bare => At::Live,
            When::AtBar(b, _) => At::Bar(b),
            When::Every { bars, first, .. } => At::Every { bars, first },
            When::Now => At::Now,
        };
        let action = match &s.kind {
            StmtKind::TempoChange(t) => Action::Tempo(*t),
            StmtKind::MeterChange(m) => Action::Meter(*m),
            StmtKind::KeyChange(k) => Action::Key(*k),
            StmtKind::Play(entries) => {
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
                Action::Play(list)
            }
            StmtKind::Add(n) => match piece.patterns.iter().position(|g| g.name == n.text) {
                Some(i) => Action::Add(i),
                None => {
                    diags.push(Diagnostic::error(
                        n.span,
                        Message::UnknownPattern {
                            name: n.text.clone(),
                        },
                    ));
                    continue;
                }
            },
            StmtKind::Level { target, dynamic } => {
                check_target(&piece, target, diags);
                Action::Level {
                    target: target.text.clone(),
                    to: LevelTarget::Dynamic(*dynamic),
                    bars: 0,
                }
            }
            StmtKind::Mute { target, muted } => {
                check_target(&piece, target, diags);
                Action::Mute {
                    target: target.text.clone(),
                    muted: *muted,
                }
            }
            StmtKind::LevelRamp { target, to, bars } => {
                check_target(&piece, target, diags);
                Action::Level {
                    target: target.text.clone(),
                    to: *to,
                    bars: *bars,
                }
            }
            StmtKind::Feel { target, feel } => {
                if let Some(t) = target {
                    check_target(&piece, t, diags);
                }
                Action::Feel {
                    target: target.as_ref().map(|t| t.text.clone()),
                    feel: *feel,
                }
            }
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
            StmtKind::Param { path, value } if s.when != When::Bare => {
                let (part, id_path) = match &s.part {
                    Some(p) => (Some(p.text.clone()), &path[..]),
                    None if path.len() > 1
                        && piece.parts.iter().any(|p| p.name == path[0].text) =>
                    {
                        (Some(path[0].text.clone()), &path[1..])
                    }
                    None => (None, &path[..]),
                };
                let id = id_path
                    .iter()
                    .map(|n| n.text.as_str())
                    .collect::<Vec<_>>()
                    .join(".");
                let Some(v) = param_value(&id, value, diags) else {
                    continue;
                };
                Action::Param { part, id, value: v }
            }
            _ => continue,
        };
        piece.commands.push(Command {
            at,
            action,
            span: s.span,
            key: collapse(&src[s.span.start..s.span.end]),
        });
    }

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
            }
        }
    };
    check(None, &mut piece.parts, diags);
    for g in piece.patterns.iter_mut().chain(piece.sections.iter_mut()) {
        check(Some(&g.name.clone()), &mut g.parts, diags);
    }
    piece
}

fn check_target(piece: &Piece, target: &Name, diags: &mut Vec<Diagnostic>) {
    if target.text == "all" || piece.all_parts().any(|p| p.name == target.text) {
        return;
    }
    diags.push(Diagnostic::warning(
        target.span,
        Message::UnknownTarget {
            name: target.text.clone(),
        },
    ));
}

/// The value of a parameter in its own unit, with warnings for an unknown
/// parameter, a unit that does not fit, or a value out of range.
fn param_value(id: &str, value: &Value, diags: &mut Vec<Diagnostic>) -> Option<f64> {
    let Some(info) = params::lookup(id) else {
        diags.push(Diagnostic::warning(
            value.span,
            Message::UnknownParam {
                name: id.to_string(),
            },
        ));
        return Some(value.number);
    };
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
    Some(v.clamp(info.min, info.max))
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
    let first_beats = line.beats(0, meters.first);
    let has_pickup = !pickup.is_zero()
        && line.kind != LineKind::Chords
        && first_beats == pickup
        && line.bars.len() > 1;
    let start = usize::from(has_pickup);
    if has_pickup {
        line.pickup = Some(0);
    }
    line.order = unroll(&line.bars[start..], &line.label, 1, diags)
        .into_iter()
        .map(|i| i + start)
        .collect();
    if line.order.is_empty() {
        return;
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

/// Why bar `i` does not fit `meter`, if it does not.
fn bar_fits(line: &Line, i: usize, meter: Meter, number: usize) -> Option<Message> {
    let BarContent::Beats(beats) = &line.bars[i].content else {
        return None;
    };
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
    let found = Ratio::int(beats.len() as i64);
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
        assert!(matches!(p.commands[0].action, Action::Play(ref v) if v == &[(0, 1)]));
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
            Action::Param { part: Some(b), id, value } if b == "bass" && id == "reverb.mix" && *value == 0.65
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
