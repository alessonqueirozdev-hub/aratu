//! Unrolling the piece into timed events.
//!
//! The piece is played bar by bar from its start. In each bar every line
//! that sounds there (a part of the piece, the pattern being played, the
//! section of the form) is laid out in beats; chance is drawn; the tempo,
//! the fermatas and the feel give each beat its time; and notes tied over
//! bar lines are joined. Rendering is deterministic: the same piece, seed
//! and live anchors always give the same events, so any range of bars can
//! be rendered on its own.

use crate::ast::{Feel, Hit, LevelTarget, LineKind, Slot, Sound, Stroke, TempoChange, Transform};
use crate::dynamics::PICKUP;
use crate::model::{Action, At, Command, Line, Part, Piece};
use crate::ratio::Ratio;
use crate::span::Span;
use crate::theory::{ACCENT, Chord, Dyn, Meter};
use crate::time::{BarClock, Tempo};
use std::collections::HashMap;

/// How long notes tied over bar lines are followed past the last bar asked for.
const TIE_LOOKAHEAD: i64 = 8;

#[derive(Clone, Debug)]
pub struct Options {
    /// For each line written without a time (keyed by [`Command::key`]), the
    /// bar from which it has been sounding. Kept by the editor while playing.
    pub anchors: HashMap<String, i64>,
    /// The bar from which a line not in `anchors` sounds.
    pub default_anchor: i64,
    /// The seed used when the piece says `seed random`.
    pub random_seed: u64,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            anchors: HashMap::new(),
            default_anchor: 1,
            random_seed: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Drum {
    Kick,
    Snare,
    ClosedHat,
    OpenHat,
    Ride,
    Crash,
}

impl Drum {
    pub fn from_symbol(c: char) -> Option<Drum> {
        Some(match c {
            'b' => Drum::Kick,
            's' => Drum::Snare,
            'c' => Drum::ClosedHat,
            'o' => Drum::OpenHat,
            'r' => Drum::Ride,
            'x' => Drum::Crash,
            _ => return None,
        })
    }

    pub fn name(self) -> &'static str {
        match self {
            Drum::Kick => "kick",
            Drum::Snare => "snare",
            Drum::ClosedHat => "closed-hat",
            Drum::OpenHat => "open-hat",
            Drum::Ride => "ride",
            Drum::Crash => "crash",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum EventKind {
    /// A pitched note, as a MIDI note number.
    Note(u8),
    Drum(Drum),
    /// A parameter set at this time.
    Param {
        id: String,
        value: f64,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Event {
    /// The global bar the event starts in (0 is the pickup).
    pub bar: i64,
    /// Where in the bar it starts, in beats, as written.
    pub beat: Ratio,
    /// Seconds from the start of the piece.
    pub time: f64,
    pub duration: f64,
    pub part: String,
    pub line: String,
    pub kind: EventKind,
    pub velocity: u8,
    /// The written symbol that made it, for the editor to light up.
    pub span: Span,
}

/// The time and frame of one bar.
#[derive(Clone, Debug, PartialEq)]
pub struct BarInfo {
    pub bar: i64,
    pub start_time: f64,
    pub end_time: f64,
    pub beats: Ratio,
    pub meter: Meter,
    /// The tempo at the bar's first beat.
    pub bpm: f64,
}

#[derive(Clone, Debug, Default)]
pub struct Rendered {
    pub events: Vec<Event>,
    pub bars: Vec<BarInfo>,
}

/// Renders bars `from..to` of the piece.
pub fn render(piece: &Piece, opts: &Options, from: i64, to: i64) -> Rendered {
    let mut s = Scheduler::new(piece, opts);
    s.run(from.max(0), to.max(from))
}

fn mix(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9e37_79b9_7f4a_7c15);
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

/// A number in `[0, 1)` fixed by its inputs.
fn chance(seed: u64, line: u64, bar: i64, what: u64) -> f64 {
    let h = mix(seed ^ mix(line ^ mix((bar as u64) ^ mix(what))));
    (h >> 11) as f64 / (1u64 << 53) as f64
}

/// Where a chord sounds in a bar: start, end, and the chord (none for N.C.).
type Region<'p> = (Ratio, Ratio, Option<&'p Chord>);

/// One part of a bar after choices are made.
#[derive(Clone, Copy, Debug)]
enum Flat<'p> {
    Hit(&'p Hit),
    Hold,
    Rest,
}

/// A sound laid out in a bar, before ties and time.
#[derive(Clone, Debug)]
struct Proto<'p> {
    start: Ratio,
    len: Ratio,
    /// `None`: the bar begins by holding the sound of the bar before.
    hit: Option<&'p Hit>,
    velocity: f64,
    /// Reaches the end of the bar, so the next bar may hold it on.
    open: bool,
}

/// A sound being held, possibly over bar lines.
#[derive(Clone, Debug)]
struct Sounding<'p> {
    start: (i64, Ratio),
    end: (i64, Ratio),
    written: Ratio,
    kinds: Vec<(EventKind, f64)>,
    velocity: u8,
    staccato: bool,
    /// A fixed length in seconds (a muted strum), instead of the written one.
    fixed: Option<f64>,
    part: &'p str,
    line: &'p str,
    span: Span,
    humanize: f64,
    jitter: f64,
}

/// A line sounding at a bar, and where in its own loop it is.
#[derive(Clone, Copy)]
struct Instance<'p> {
    part: &'p Part,
    line: &'p Line,
    /// Bars since the instance began (0 for its first bar); `None` for the
    /// pickup bar.
    local: Option<i64>,
    /// Distinguishes the same line played from different places (the
    /// piece, a pattern, a section) so ties never jump between them.
    origin: u64,
    /// The pattern or section it comes from.
    group: Option<&'p str>,
}

struct Scheduler<'p> {
    piece: &'p Piece,
    seed: u64,
    /// Commands with the bar they take effect at.
    timed: Vec<(i64, &'p Command)>,
    every: Vec<&'p Command>,
    tempo: Tempo,
    clocks: Vec<BarClock>,
    /// Open sounds by instance.
    open: HashMap<u64, Sounding<'p>>,
    /// The chord sounding in each part instance, for holds over bar lines.
    last_chord: HashMap<u64, Option<&'p Chord>>,
    done: Vec<Sounding<'p>>,
    params: Vec<(i64, Event)>,
}

impl<'p> Scheduler<'p> {
    fn new(piece: &'p Piece, opts: &Options) -> Scheduler<'p> {
        let seed = match piece.seed {
            crate::ast::Seed::Fixed(n) => n,
            crate::ast::Seed::Random => opts.random_seed,
        };
        let mut timed = Vec::new();
        let mut every = Vec::new();
        for c in &piece.commands {
            match c.at {
                At::Bar(b) => timed.push((b, c)),
                At::Live | At::Now => {
                    let b = *opts.anchors.get(&c.key).unwrap_or(&opts.default_anchor);
                    timed.push((b, c));
                }
                At::Every { .. } => every.push(c),
            }
        }
        timed.sort_by_key(|(b, _)| *b);
        Scheduler {
            piece,
            seed,
            timed,
            every,
            tempo: Tempo::new(piece.tempo),
            clocks: Vec::new(),
            open: HashMap::new(),
            last_chord: HashMap::new(),
            done: Vec::new(),
            params: Vec::new(),
        }
    }

    fn meter_at(&self, bar: i64) -> Meter {
        let mut m = self.piece.meter;
        for (b, c) in &self.timed {
            if *b > bar {
                break;
            }
            if let Action::Meter(x) = c.action {
                m = x;
            }
        }
        m
    }

    fn bar_beats(&self, bar: i64) -> Ratio {
        if bar == 0 {
            self.piece.pickup
        } else {
            self.meter_at(bar).beats()
        }
    }

    fn run(&mut self, from: i64, to: i64) -> Rendered {
        let mut g = 0;
        let mut start_beat = Ratio::ZERO;
        let mut start_time = 0.0;
        loop {
            let busy = self.open.values().any(|s| s.start.0 < to);
            if g >= to && (!busy || g >= to + TIE_LOOKAHEAD) {
                break;
            }
            let beats = self.bar_beats(g);
            // Tempo changes at the bar line.
            for (b, c) in self.timed.clone() {
                if b != g {
                    continue;
                }
                if let Action::Tempo(t) = c.action {
                    let len = match t {
                        TempoChange::Ramp { bars, .. } => (g..g + bars)
                            .map(|x| self.bar_beats(x))
                            .fold(Ratio::ZERO, |a, b| a + b),
                        _ => Ratio::ZERO,
                    };
                    self.tempo.change(t, start_beat, len);
                }
            }
            let mut instances = self.instances(g);
            instances.retain(|i| !self.muted(&i.part.name, g));
            let mut fermatas = Vec::new();
            let mut laid: Vec<(Instance<'p>, Vec<Proto<'p>>)> = Vec::new();
            for inst in instances {
                let protos = self.lay_out(inst, g, beats);
                for p in &protos {
                    if let Some(h) = p.hit {
                        if h.fermata {
                            fermatas.push((p.start, p.start + p.len));
                        }
                    }
                }
                laid.push((inst, protos));
            }
            let clock = BarClock::new(
                g,
                start_beat,
                beats,
                start_time,
                self.tempo.clone(),
                fermatas,
            );
            start_time = clock.end_time();
            start_beat += beats;
            self.clocks.push(clock);

            // Chords first, so strums in the same part can find them.
            let mut regions: HashMap<u64, Vec<Region<'p>>> = HashMap::new();
            for (inst, protos) in &laid {
                if inst.line.kind == LineKind::Chords && self.has_strum(inst.part) {
                    let key = part_key(inst);
                    let mut list = Vec::new();
                    let mut last = self.last_chord.get(&key).copied().flatten();
                    for p in protos {
                        let chord = match p.hit.map(|h| &h.sound) {
                            Some(Sound::Chord(c)) => Some(c),
                            Some(_) => None,
                            None => last,
                        };
                        list.push((p.start, p.start + p.len, chord));
                        last = chord;
                    }
                    // A chord held to the end of the bar stays for the next.
                    let ends_open = protos.last().is_some_and(|p| p.open);
                    self.last_chord
                        .insert(key, if ends_open { last } else { None });
                    regions.insert(key, list);
                }
            }
            for (inst, protos) in laid {
                if inst.line.kind == LineKind::Chords && self.has_strum(inst.part) {
                    continue;
                }
                let chords = regions.get(&part_key(&inst)).cloned().unwrap_or_default();
                self.sound(inst, g, beats, protos, &chords);
            }
            // Sounds the next bar did not hold on are finished.
            let ended: Vec<u64> = self
                .open
                .iter()
                .filter(|(_, s)| s.end.0 < g)
                .map(|(k, _)| *k)
                .collect();
            for k in ended {
                let s = self.open.remove(&k).unwrap();
                self.done.push(s);
            }
            for (b, c) in &self.timed {
                if *b != g {
                    continue;
                }
                if let Action::Param {
                    part, id, value, ..
                } = &c.action
                {
                    let clock = self.clocks.last().unwrap();
                    self.params.push((
                        g,
                        Event {
                            bar: g,
                            beat: Ratio::ZERO,
                            time: clock.start_time,
                            duration: 0.0,
                            part: part.clone().unwrap_or_else(|| "all".into()),
                            line: String::new(),
                            kind: EventKind::Param {
                                id: id.clone(),
                                value: *value,
                            },
                            velocity: 0,
                            span: c.span,
                        },
                    ));
                }
            }
            g += 1;
        }
        // Whatever is still held ends where rendering stopped.
        let open: Vec<Sounding<'p>> = self.open.drain().map(|(_, s)| s).collect();
        self.done.extend(open);

        let mut events = Vec::new();
        for s in std::mem::take(&mut self.done) {
            if s.start.0 < from || s.start.0 >= to {
                continue;
            }
            let t0 = self.time_of(s.start) + s.jitter;
            let t1 = self.time_of(s.end) + s.jitter;
            let mut dur = match s.fixed {
                Some(d) => d,
                None => (t1 - t0).max(0.0),
            };
            if s.staccato {
                dur *= 0.5;
            }
            for (kind, delay) in &s.kinds {
                events.push(Event {
                    bar: s.start.0,
                    beat: s.written,
                    time: t0 + delay,
                    duration: dur,
                    part: s.part.to_string(),
                    line: s.line.to_string(),
                    kind: kind.clone(),
                    velocity: s.velocity,
                    span: s.span,
                });
            }
            let _ = s.humanize;
        }
        events.extend(
            self.params
                .drain(..)
                .filter(|(g, _)| *g >= from && *g < to)
                .map(|(_, e)| e),
        );
        events.sort_by(|a, b| {
            a.time
                .total_cmp(&b.time)
                .then(a.span.start.cmp(&b.span.start))
        });
        let bars = self
            .clocks
            .iter()
            .filter(|c| c.bar >= from && c.bar < to)
            .map(|c| BarInfo {
                bar: c.bar,
                start_time: c.start_time,
                end_time: c.end_time(),
                beats: c.beats,
                meter: self.meter_at(c.bar.max(1)),
                bpm: c.bpm_at(Ratio::ZERO),
            })
            .collect();
        Rendered { events, bars }
    }

    fn time_of(&self, (bar, offset): (i64, Ratio)) -> f64 {
        match self.clocks.get(bar as usize) {
            Some(c) => c.time(offset),
            None => self.clocks.last().map_or(0.0, |c| c.end_time()),
        }
    }

    fn has_strum(&self, part: &Part) -> bool {
        part.lines.iter().any(|l| l.kind == LineKind::Strum)
    }

    /// Every line sounding at bar `g`.
    fn instances(&self, g: i64) -> Vec<Instance<'p>> {
        let piece = self.piece;
        let mut out = Vec::new();
        for part in &piece.parts {
            for line in &part.lines {
                if g == 0 {
                    if line.pickup.is_some() {
                        out.push(Instance {
                            part,
                            line,
                            local: None,
                            origin: 1,
                            group: None,
                        });
                    }
                } else {
                    out.push(Instance {
                        part,
                        line,
                        local: Some(g - 1),
                        origin: 1,
                        group: None,
                    });
                }
            }
        }
        if g == 0 {
            return out;
        }

        // The section of the form.
        let sec_len = |i: usize| group_len(&piece.sections[i].parts);
        let total: i64 = piece
            .form
            .iter()
            .map(|(i, t)| sec_len(*i) * *t as i64)
            .sum();
        if total > 0 {
            let mut pos = (g - 1).rem_euclid(total);
            for (i, times) in &piece.form {
                let len = sec_len(*i) * *times as i64;
                if pos < len {
                    let sec = &piece.sections[*i];
                    push_group(
                        &mut out,
                        &sec.parts,
                        pos % sec_len(*i),
                        2 + *i as u64,
                        &sec.name,
                    );
                    break;
                }
                pos -= len;
            }
        }

        // The pattern being played, unless an `every` takes its place.
        let mut current: Option<(usize, i64)> = None;
        for (b, c) in &self.timed {
            if *b > g {
                break;
            }
            if let Action::Play(list) = &c.action {
                let lens: Vec<i64> = list
                    .iter()
                    .map(|(i, t)| group_len(&piece.patterns[*i].parts) * *t as i64)
                    .collect();
                let total: i64 = lens.iter().sum();
                let mut pos = (g - b).rem_euclid(total.max(1));
                for ((i, _), len) in list.iter().zip(&lens) {
                    if pos < *len {
                        current = Some((*i, pos % group_len(&piece.patterns[*i].parts)));
                        break;
                    }
                    pos -= len;
                }
            }
        }
        let mut layers: Vec<(usize, i64)> = Vec::new();
        for c in &self.every {
            let At::Every { bars, first } = c.at else {
                continue;
            };
            let (pattern, add) = match c.action {
                Action::Play(ref list) => (list[0].0, false),
                Action::Add(i) => (i, true),
                _ => continue,
            };
            let len = group_len(&piece.patterns[pattern].parts).min(bars);
            let pos = (g - 1).rem_euclid(bars);
            let local = if first {
                (pos < len).then_some(pos)
            } else {
                (pos >= bars - len).then(|| pos - (bars - len))
            };
            if let Some(local) = local {
                if add {
                    layers.push((pattern, local));
                } else {
                    current = Some((pattern, local));
                }
            }
        }
        for (b, c) in &self.timed {
            if *b > g {
                break;
            }
            if let Action::Add(i) = c.action {
                let len = group_len(&piece.patterns[i].parts);
                layers.push((i, (g - b).rem_euclid(len)));
            }
        }
        for (k, (i, local)) in current.into_iter().chain(layers).enumerate() {
            let pat = &piece.patterns[i];
            push_group(
                &mut out,
                &pat.parts,
                local,
                1000 + (k as u64) * 100 + i as u64,
                &pat.name,
            );
        }
        out
    }

    /// The transformations in force for an instance at bar `g`.
    fn transforms(&self, inst: &Instance, g: i64) -> Vec<Transform> {
        self.timed
            .iter()
            .filter(|(b, _)| *b <= g)
            .filter_map(|(_, c)| match &c.action {
                Action::Transform { target, transform } => {
                    let t = target.as_str();
                    let hit = t == inst.line.name
                        || t == inst.part.name
                        || t == inst.line.label
                        || t == format!("{}.{}", inst.part.name, inst.line.name)
                        || Some(t) == inst.group;
                    hit.then_some(*transform)
                }
                _ => None,
            })
            .collect()
    }

    /// Lays out the bar of a line that sounds at `g`: choices made, chance
    /// drawn, holds joined, transformations applied.
    fn lay_out(&self, inst: Instance<'p>, g: i64, beats: Ratio) -> Vec<Proto<'p>> {
        let line = inst.line;
        let transforms = self.transforms(&inst, g);
        let len = line.order.len() as i64;
        if len == 0 && inst.local.is_some() {
            return Vec::new();
        }
        let has = |f: &dyn Fn(&Transform) -> bool| transforms.iter().any(f);
        let retro = has(&|t| matches!(t, Transform::Retrograde));
        let augment = has(&|t| matches!(t, Transform::Augment));
        let diminish = has(&|t| matches!(t, Transform::Diminish));

        // Which bars of the line, and how each is fitted into this bar:
        // (place in order, written bar, offset, scale, window start).
        let mut pieces: Vec<(usize, usize, Ratio, Ratio, Option<Ratio>)> = Vec::new();
        match inst.local {
            None => pieces.push((PICKUP, line.pickup.unwrap(), Ratio::ZERO, Ratio::ONE, None)),
            Some(j) => {
                let pick = |n: i64| -> usize {
                    let k = n.rem_euclid(len);
                    (if retro { len - 1 - k } else { k }) as usize
                };
                if augment && !diminish {
                    let k = pick(j / 2);
                    let half = Ratio::new(j.rem_euclid(2), 1) * beats;
                    pieces.push((k, line.order[k], Ratio::ZERO, Ratio::int(2), Some(half)));
                } else if diminish && !augment {
                    let half = beats / Ratio::int(2);
                    for (n, off) in [(2 * j, Ratio::ZERO), (2 * j + 1, half)] {
                        let k = pick(n);
                        pieces.push((k, line.order[k], off, Ratio::new(1, 2), None));
                    }
                } else {
                    let k = pick(j);
                    pieces.push((k, line.order[k], Ratio::ZERO, Ratio::ONE, None));
                }
            }
        }

        let mut out = Vec::new();
        for (k, bar_index, offset, scale, window) in pieces {
            let mut counter = 0u64;
            let mut flats: Vec<(Ratio, Ratio, Flat<'p>)> = Vec::new();
            let crate::ast::BarContent::Beats(written) = &line.bars[bar_index].content else {
                continue;
            };
            let share = if line.kind == LineKind::Chords {
                beats / Ratio::int(written.len().max(1) as i64)
            } else {
                Ratio::ONE
            };
            for (b, beat) in written.iter().enumerate() {
                self.flatten(
                    &beat.slots,
                    share * Ratio::int(b as i64),
                    share,
                    line.seed_key,
                    g,
                    &mut counter,
                    &mut flats,
                );
            }
            let bar_len = share * Ratio::int(written.len() as i64);

            // Rotation by the bar's finest part.
            for t in &transforms {
                if let Transform::Rotate(n) = t {
                    if let Some(grid) = flats.iter().map(|f| f.1).reduce(ratio_gcd) {
                        let shift = grid * Ratio::int(*n);
                        for f in flats.iter_mut() {
                            let s = f.0 + shift;
                            f.0 = s - bar_len * Ratio::int((s / bar_len).floor());
                        }
                        flats.sort_by_key(|f| f.0);
                    }
                }
            }

            let strum = line.kind == LineKind::Strum;
            let mut protos: Vec<Proto<'p>> = Vec::new();
            let mut cur: Option<Proto<'p>> = None;
            let mut started = false;
            for (start, plen, f) in flats {
                match f {
                    Flat::Hit(h) => {
                        protos.extend(cur.take());
                        started = true;
                        let mut keep = h.chance.is_none_or(|p| {
                            chance(self.seed, line.seed_key, g, 0x5eed ^ h.span.start as u64) < p
                        });
                        for t in &transforms {
                            if let Transform::Density(p) = t {
                                let r = chance(
                                    self.seed,
                                    line.seed_key,
                                    g,
                                    0xde75 ^ h.span.start as u64 ^ (start.num() as u64) << 20,
                                );
                                keep &= r < p / 100.0;
                            }
                        }
                        if keep {
                            let base = line.levels.get(&(k, h.span.start)).copied().unwrap_or(80);
                            cur = Some(Proto {
                                start,
                                len: plen,
                                hit: Some(h),
                                velocity: base as f64,
                                open: false,
                            });
                        }
                    }
                    Flat::Hold => match cur.as_mut() {
                        Some(p) => p.len += plen,
                        None if !started && start.is_zero() => {
                            cur = Some(Proto {
                                start,
                                len: plen,
                                hit: None,
                                velocity: 0.0,
                                open: false,
                            });
                        }
                        None if !started => {
                            if let Some(p) = protos.last_mut().filter(|p| p.hit.is_none()) {
                                p.len += plen;
                            }
                        }
                        None => {}
                    },
                    Flat::Rest => {
                        if strum {
                            if let Some(p) = cur.as_mut() {
                                p.len += plen;
                                continue;
                            }
                        }
                        protos.extend(cur.take());
                        started = true;
                    }
                }
            }
            protos.extend(cur.take());
            if let Some(last) = protos.last_mut() {
                last.open = last.start + last.len >= bar_len;
            }

            // Fit into this bar.
            for mut p in protos {
                p.start = p.start * scale;
                p.len = p.len * scale;
                if let Some(w) = window {
                    let (s, e) = (p.start.max(w), (p.start + p.len).min(w + beats));
                    if s >= e {
                        continue;
                    }
                    if s > p.start {
                        p.hit = None;
                    }
                    p.start = s - w;
                    p.len = e - s;
                    p.open = p.start + p.len >= beats;
                }
                p.start += offset;
                if scale != Ratio::ONE && window.is_none() {
                    p.open = p.open && offset + bar_len * scale >= beats;
                }
                if retro {
                    p.start = beats - p.start - p.len;
                    if p.hit.is_none() {
                        continue;
                    }
                    p.open = false;
                }
                if p.start >= beats {
                    continue;
                }
                out.push(p);
            }
        }
        if retro {
            out.sort_by_key(|p| p.start);
        }
        out
    }

    /// Lays `slots` over `[start, start + len)`, choosing among options.
    #[allow(clippy::too_many_arguments)]
    fn flatten(
        &self,
        slots: &'p [Slot],
        start: Ratio,
        len: Ratio,
        line_key: u64,
        g: i64,
        counter: &mut u64,
        out: &mut Vec<(Ratio, Ratio, Flat<'p>)>,
    ) {
        if slots.is_empty() {
            return;
        }
        let part = len / Ratio::int(slots.len() as i64);
        for (i, s) in slots.iter().enumerate() {
            let at = start + part * Ratio::int(i as i64);
            match s {
                Slot::Hit(h) => out.push((at, part, Flat::Hit(h))),
                Slot::Hold(_) => out.push((at, part, Flat::Hold)),
                Slot::Rest(_) => out.push((at, part, Flat::Rest)),
                Slot::Group(inner, _) => self.flatten(inner, at, part, line_key, g, counter, out),
                Slot::Choice(options, span) => {
                    *counter += 1;
                    let r = chance(
                        self.seed,
                        line_key,
                        g,
                        0xc401 ^ (span.start as u64) << 8 ^ *counter,
                    );
                    let pick = ((r * options.len() as f64) as usize).min(options.len() - 1);
                    self.flatten(&options[pick], at, part, line_key, g, counter, out);
                }
            }
        }
    }

    /// The live level of a part at an offset of bar `g`, as a factor of the
    /// written dynamics (1 at mf).
    fn muted(&self, part: &str, g: i64) -> bool {
        let applies = |t: &str| t == part || t == "all";
        let mut muted = false;
        for (b, c) in &self.timed {
            if *b > g {
                break;
            }
            if let Action::Mute { target, muted: m } = &c.action {
                if applies(target) {
                    muted = *m;
                }
            }
        }
        for c in &self.every {
            let (At::Every { bars, first }, Action::Mute { target, muted: m }) = (c.at, &c.action)
            else {
                continue;
            };
            let pos = (g - 1).rem_euclid(bars);
            let hit = if first { pos == 0 } else { pos == bars - 1 };
            if hit && applies(target) {
                muted = *m;
            }
        }
        muted
    }

    fn level(&self, part: &str, g: i64, frac: f64) -> f64 {
        let mut total = 1.0;
        for target in ["all", part] {
            let mut factor = 1.0;
            for (b, c) in &self.timed {
                if *b > g {
                    break;
                }
                let Action::Level {
                    target: t,
                    to,
                    bars,
                } = &c.action
                else {
                    continue;
                };
                if t != target {
                    continue;
                }
                let goal = match to {
                    LevelTarget::Dynamic(d) => d.velocity() as f64 / Dyn::Mf.velocity() as f64,
                    LevelTarget::Silence => 0.0,
                };
                if *bars == 0 {
                    factor = goal;
                } else {
                    let progress = ((g - b) as f64 + frac) / *bars as f64;
                    factor = if progress >= 1.0 {
                        goal
                    } else {
                        factor + (goal - factor) * progress
                    };
                }
            }
            total *= factor;
        }
        total
    }

    /// The feel in force for a part at bar `g`.
    fn feel(&self, part: &str, g: i64) -> (Option<f64>, Option<f64>, Option<f64>) {
        let (mut swing, mut rubato, mut human) = (None, None, None);
        for pass in [None, Some(part)] {
            for (b, c) in &self.timed {
                if *b > g {
                    break;
                }
                let Action::Feel { target, feel } = &c.action else {
                    continue;
                };
                let applies = match (pass, target.as_deref()) {
                    (None, None) | (None, Some("all")) => true,
                    (Some(p), Some(t)) => p == t,
                    _ => false,
                };
                if !applies {
                    continue;
                }
                match feel {
                    Feel::Swing(s) => swing = Some(*s),
                    Feel::Rubato(r) => rubato = Some(*r),
                    Feel::Humanize(h) => human = Some(*h),
                }
            }
        }
        (swing, rubato, human)
    }

    /// Moves an offset by swing and rubato.
    fn warp(&self, part: &str, g: i64, beats: Ratio, x: Ratio) -> Ratio {
        if g == 0 {
            return x;
        }
        let (swing, rubato, _) = self.feel(part, g);
        let mut y = x.to_f64();
        if let Some(s) = swing {
            if !self.meter_at(g).is_compound() {
                let s = (s / 100.0).clamp(0.01, 0.99);
                let b = y.floor();
                let u = y - b;
                let u = if u <= 0.5 {
                    u * 2.0 * s
                } else {
                    s + (u - 0.5) * 2.0 * (1.0 - s)
                };
                y = b + u;
            }
        }
        if let Some(r) = rubato {
            let len = beats.to_f64();
            if len > 0.0 {
                let sign = if chance(self.seed, mix(part.len() as u64), g, 0x4ba7) < 0.5 {
                    -1.0
                } else {
                    1.0
                };
                let a = (r / 100.0).clamp(0.0, 0.9) * sign;
                let tau = std::f64::consts::TAU;
                y += a * len / tau * (tau * y / len).sin();
            }
        }
        if (y - x.to_f64()).abs() < 1e-12 {
            return x;
        }
        // Back to an exact fraction with a fine grain (1/7680 of a beat).
        Ratio::new((y * 7680.0).round() as i64, 7680)
    }

    /// Turns laid-out sounds into notes, joining ties with the bar before.
    fn sound(
        &mut self,
        inst: Instance<'p>,
        g: i64,
        beats: Ratio,
        protos: Vec<Proto<'p>>,
        chords: &[Region<'p>],
    ) {
        let key = mix(inst.line.seed_key ^ mix(inst.origin));
        let transforms = self.transforms(&inst, g);
        let part = inst.part.name.as_str();
        let (_, _, human) = self.feel(part, g);
        let first_pitch = first_pitch(inst.line);

        for p in protos {
            let end_off = p.start + p.len;
            let Some(h) = p.hit else {
                // The bar holds on the sound of the bar before.
                let end = self.warp(part, g, beats, end_off);
                if let Some(s) = self.open.get_mut(&key) {
                    if s.end.0 == g - 1 && p.start.is_zero() {
                        s.end = (g, end);
                        if !p.open {
                            let s = self.open.remove(&key).unwrap();
                            self.done.push(s);
                        }
                        continue;
                    }
                }
                continue;
            };
            if let Some(s) = self.open.remove(&key) {
                self.done.push(s);
            }

            let mut velocity = p.velocity;
            if h.accent {
                velocity += ACCENT as f64;
            }
            let mut transpose = 0;
            let mut invert = false;
            for t in &transforms {
                match t {
                    Transform::Velocity(x) => velocity *= x,
                    Transform::Transpose(n) => transpose += *n as i32,
                    Transform::Invert => invert = true,
                    _ => {}
                }
            }
            let frac = (p.start / beats.max(Ratio::new(1, 1000))).to_f64();
            velocity *= self.level(part, g, frac);
            let jitter = match human {
                Some(ms) => {
                    let r = chance(
                        self.seed,
                        key,
                        g,
                        0x7177 ^ p.start.num() as u64 ^ (p.start.den() as u64) << 32,
                    );
                    velocity += (r - 0.5) * 16.0;
                    (r * 2.0 - 1.0) * ms / 1000.0
                }
                None => 0.0,
            };
            let velocity = velocity.round().clamp(0.0, 127.0) as u8;
            if velocity == 0 {
                continue;
            }
            let pitch = |n: i32| -> Option<EventKind> {
                let n = if invert { 2 * first_pitch - n } else { n } + transpose;
                u8::try_from(n)
                    .ok()
                    .filter(|n| *n < 128)
                    .map(EventKind::Note)
            };

            let mut kinds: Vec<(EventKind, f64)> = Vec::new();
            let mut fixed = None;
            match &h.sound {
                Sound::Drum(c) => {
                    if let Some(d) = Drum::from_symbol(*c) {
                        kinds.push((EventKind::Drum(d), 0.0));
                    }
                }
                Sound::Notes(ns) => {
                    kinds.extend(ns.iter().filter_map(|n| pitch(*n)).map(|k| (k, 0.0)))
                }
                Sound::Chord(c) => kinds.extend(
                    c.block_voicing()
                        .into_iter()
                        .filter_map(pitch)
                        .map(|k| (k, 0.0)),
                ),
                Sound::Strum(stroke) => {
                    let chord = chords
                        .iter()
                        .find(|(s, e, _)| p.start >= *s && p.start < *e)
                        .and_then(|(_, _, c)| *c);
                    let Some(chord) = chord else { continue };
                    let mut strings = chord.guitar_voicing();
                    let spread = if h.accent { 0.006 } else { 0.012 };
                    match stroke {
                        Stroke::Down => {}
                        Stroke::Up => {
                            strings.reverse();
                            strings.truncate(4);
                        }
                        Stroke::Muted => fixed = Some(0.04),
                    }
                    for (i, n) in strings.into_iter().enumerate() {
                        if let Some(k) = pitch(n) {
                            kinds.push((k, i as f64 * spread));
                        }
                    }
                }
            }
            if kinds.is_empty() {
                continue;
            }
            let velocity = if matches!(h.sound, Sound::Strum(Stroke::Muted)) {
                (velocity as f64 * 0.6) as u8
            } else {
                velocity
            };
            let s = Sounding {
                start: (g, self.warp(part, g, beats, p.start)),
                end: (g, self.warp(part, g, beats, end_off.min(beats))),
                written: p.start,
                kinds,
                velocity,
                staccato: h.staccato,
                fixed,
                part: inst.part.name.as_str(),
                line: inst.line.label.as_str(),
                span: h.span,
                humanize: human.unwrap_or(0.0),
                jitter,
            };
            if p.open && !matches!(h.sound, Sound::Drum(_)) {
                self.open.insert(key, s);
            } else {
                self.done.push(s);
            }
        }
    }
}

fn part_key(inst: &Instance) -> u64 {
    mix(inst.origin ^ mix(inst.part.name.len() as u64 ^ inst.part.span.start as u64))
}

fn ratio_gcd(a: Ratio, b: Ratio) -> Ratio {
    fn g(a: i64, b: i64) -> i64 {
        if b == 0 { a.abs() } else { g(b, a % b) }
    }
    // gcd(a/b, c/d) = gcd(a·d, c·b) / (b·d)
    let n = g(a.num() * b.den(), b.num() * a.den());
    Ratio::new(n, a.den() * b.den())
}

/// The bars a group lasts: its longest line.
fn group_len(parts: &[Part]) -> i64 {
    parts
        .iter()
        .flat_map(|p| p.lines.iter())
        .map(|l| l.order.len() as i64)
        .max()
        .unwrap_or(1)
        .max(1)
}

fn push_group<'p>(
    out: &mut Vec<Instance<'p>>,
    parts: &'p [Part],
    local: i64,
    origin: u64,
    name: &'p str,
) {
    for part in parts {
        for line in &part.lines {
            out.push(Instance {
                part,
                line,
                local: Some(local),
                origin,
                group: Some(name),
            });
        }
    }
}

/// The first written pitch of a line, the axis of `invert`.
fn first_pitch(line: &Line) -> i32 {
    let mut first = None;
    for &i in &line.order {
        if let crate::ast::BarContent::Beats(beats) = &line.bars[i].content {
            for b in beats {
                crate::dynamics::visit(&b.slots, Ratio::ZERO, Ratio::ONE, &mut |h, _, _| {
                    if first.is_none() {
                        if let Sound::Notes(n) = &h.sound {
                            first = n.first().copied();
                        }
                    }
                });
            }
        }
        if first.is_some() {
            break;
        }
    }
    first.unwrap_or(60)
}
