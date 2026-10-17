//! Unrolling the piece into timed events.
//!
//! The piece is played bar by bar from its start. In each bar every line
//! that sounds there (a part of the piece, the pattern being played, the
//! section of the form) is laid out in beats; chance is drawn; the tempo,
//! the fermatas and the feel give each beat its time; and notes tied over
//! bar lines are joined. Rendering is deterministic: the same piece, seed
//! and live anchors always give the same events, so any range of bars can
//! be rendered on its own.

use crate::ast::{
    ArpStep, ArpWay, Feel, Hit, Length, LevelTarget, LineKind, Ornament, Slot, Sound, Stroke,
    TempoChange, TimeUnit, Transform,
};
use crate::dynamics::PICKUP;
use crate::model::{Action, At, Command, DuckSource, Line, Motion, Part, Piece};
use crate::motifs::by_degrees;
use crate::ratio::Ratio;
use crate::span::Span;
use crate::theory::{ACCENT, Chord, Key, Meter};
use crate::time::{BarClock, Tempo};
use std::collections::HashMap;

/// How long notes tied over bar lines are followed past the last bar asked for.
const TIE_LOOKAHEAD: i64 = 8;

/// How loud each echo is against the sound before it.
const ECHO: f64 = 0.6;

#[derive(Clone, Debug)]
pub struct Options {
    /// For each line written without a time (keyed by [`Command::key`]), the
    /// bar from which it has been sounding. Kept by the editor while playing.
    pub anchors: HashMap<String, i64>,
    /// The bar from which a line not in `anchors` sounds.
    pub default_anchor: i64,
    /// The seed used when the piece says `seed random`.
    pub random_seed: u64,
    /// Where in the first bar asked for rendering begins, in beats. Sounds
    /// that start before it are left out (the editor keeps those of the
    /// version of the piece it replaces mid-bar); parameters set at that
    /// bar line are set at this beat instead.
    pub from_beat: Ratio,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            anchors: HashMap::new(),
            default_anchor: 1,
            random_seed: 0,
            from_beat: Ratio::ZERO,
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
    HighTom,
    MidTom,
    FloorTom,
    SideStick,
    /// Hands clapping: palmas.
    Clap,
    PandeiroThumb,
    PandeiroFingers,
    PandeiroSlap,
    PandeiroMuted,
    ZabumbaOpen,
    ZabumbaMuted,
    /// The *bacalhau*: a thin stick on the zabumba's other skin.
    ZabumbaStick,
    TriangleOpen,
    TriangleClosed,
    AgogoHigh,
    AgogoLow,
    SurdoOpen,
    SurdoMuted,
    TamborimOpen,
    TamborimMuted,
    AlfaiaOpen,
    AlfaiaMuted,
    GongueOpen,
    GongueMuted,
    Shaker,
    CuicaHigh,
    CuicaLow,
    CajonBass,
    CajonSlap,
    /// The flamenco *golpe* on a guitar's top, from a strum line.
    GuitarTap,
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
            't' => Drum::HighTom,
            'm' => Drum::MidTom,
            'f' => Drum::FloorTom,
            'k' => Drum::SideStick,
            'p' => Drum::Clap,
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
            Drum::HighTom => "high-tom",
            Drum::MidTom => "mid-tom",
            Drum::FloorTom => "floor-tom",
            Drum::SideStick => "side-stick",
            Drum::Clap => "clap",
            Drum::PandeiroThumb => "pandeiro-thumb",
            Drum::PandeiroFingers => "pandeiro-fingers",
            Drum::PandeiroSlap => "pandeiro-slap",
            Drum::PandeiroMuted => "pandeiro-muted",
            Drum::ZabumbaOpen => "zabumba-open",
            Drum::ZabumbaMuted => "zabumba-muted",
            Drum::ZabumbaStick => "zabumba-stick",
            Drum::TriangleOpen => "triangle-open",
            Drum::TriangleClosed => "triangle-closed",
            Drum::AgogoHigh => "agogo-high",
            Drum::AgogoLow => "agogo-low",
            Drum::SurdoOpen => "surdo-open",
            Drum::SurdoMuted => "surdo-muted",
            Drum::TamborimOpen => "tamborim-open",
            Drum::TamborimMuted => "tamborim-muted",
            Drum::AlfaiaOpen => "alfaia-open",
            Drum::AlfaiaMuted => "alfaia-muted",
            Drum::GongueOpen => "gongue-open",
            Drum::GongueMuted => "gongue-muted",
            Drum::Shaker => "shaker",
            Drum::CuicaHigh => "cuica-high",
            Drum::CuicaLow => "cuica-low",
            Drum::CajonBass => "cajon-bass",
            Drum::CajonSlap => "cajon-slap",
            Drum::GuitarTap => "guitar-tap",
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
        /// For a parameter that moves: the seconds since its last point,
        /// over which the sound goes smoothly to this value. 0 sets it.
        glide: f64,
        /// `from bar 9 to 16` is over and nothing written comes back: the
        /// parameter returns to the value its part is set up with.
        reset: bool,
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
    /// Written in upper case: the engine may play it another way (a slap
    /// bass pops it).
    pub accent: bool,
    /// The written symbol that made it, for the editor to light up.
    pub span: Span,
    /// For a strum, the chord symbol it plays, lit up with it.
    pub also: Option<Span>,
    /// A grace note or stroke: how many come before its note from it on (1
    /// right before); 0 for anything else.
    pub grace: u8,
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
    /// When each whole beat of the bar starts, in seconds from the piece's
    /// start: where a change with `now:` can come in.
    pub beat_times: Vec<f64>,
}

#[derive(Clone, Debug, Default)]
pub struct Rendered {
    pub events: Vec<Event>,
    pub bars: Vec<BarInfo>,
}

/// Renders bars `from..to` of the piece.
pub fn render(piece: &Piece, opts: &Options, from: i64, to: i64) -> Rendered {
    let mut s = Scheduler::new(piece, opts);
    s.run(from.max(0), to.max(from), opts.from_beat)
}

fn mix(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9e37_79b9_7f4a_7c15);
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

/// For `temas.random`: the place in `order` of a line's bar `n`, one
/// melody chosen at random (from the seed) each time round.
fn random_place(line: &Line, seed: u64, n: i64) -> usize {
    let count = line.rounds.len();
    let mut at = 0i64;
    let mut round = 0i64;
    loop {
        let pick =
            ((chance(seed, line.seed_key, round, 0x11575) * count as f64) as usize).min(count - 1);
        let (start, len) = line.rounds[pick];
        let len = (len as i64).max(1);
        if n.max(0) < at + len {
            return start + (n.max(0) - at) as usize;
        }
        at += len;
        round += 1;
    }
}

/// A length in beats of the meter where it starts.
fn beats_of(l: Length, meter: Meter) -> f64 {
    let unit = meter.beat_unit().to_f64();
    l.count
        * match l.unit {
            TimeUnit::Bars => meter.beats().to_f64(),
            TimeUnit::Beats => 1.0,
            TimeUnit::Eighths => 0.125 / unit,
            TimeUnit::Sixteenths => 0.0625 / unit,
        }
}

/// The value of a moving parameter `p` beats after it starts from `from`.
fn motion_value(from: f64, m: &Motion, rise: f64, fall: f64, p: f64) -> f64 {
    let cycle = rise + fall;
    let q = if m.repeat {
        p.rem_euclid(cycle)
    } else {
        p.min(cycle)
    };
    if q <= rise {
        between(from, m.to, q / rise, m.log)
    } else {
        between(m.to, from, (q - rise) / fall, m.log)
    }
}

/// The value a fraction `t` of the way from `a` to `b`, by equal ratios
/// for a parameter whose knob works that way.
fn between(a: f64, b: f64, t: f64, log: bool) -> f64 {
    if log && a > 0.0 && b > 0.0 {
        a * (b / a).powf(t)
    } else {
        a + (b - a) * t
    }
}

/// A number in `[0, 1)` fixed by its inputs.
fn chance(seed: u64, line: u64, bar: i64, what: u64) -> f64 {
    let h = mix(seed ^ mix(line ^ mix((bar as u64) ^ mix(what))));
    (h >> 11) as f64 / (1u64 << 53) as f64
}

/// A stretch of a bar and the chord sounding in it, with where that chord
/// is written, for strums to play it and the editor to light it.
type Region<'p> = (Ratio, Ratio, Option<&'p Chord>, Option<Span>);

/// A moving parameter's command with the bar it starts from and, for
/// `every`, the bar after the stretch it belongs to and whether it starts
/// there.
type Moving<'p> = (i64, Option<(i64, bool)>, &'p Command);

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
    accent: bool,
    staccato: bool,
    /// A fixed length in seconds (a muted strum), instead of the written one.
    fixed: Option<f64>,
    part: &'p str,
    line: &'p str,
    span: Span,
    also: Option<Span>,
    humanize: f64,
    jitter: f64,
    /// See [`Event::grace`].
    grace: u8,
    /// `echo 1 eighth x3`: played again that much later, softer, so many
    /// times.
    echo: Option<(Length, u32)>,
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
    last_chord: HashMap<u64, Option<(&'p Chord, Span)>>,
    done: Vec<Sounding<'p>>,
    params: Vec<(i64, Event)>,
    /// When each moving parameter had its last point, by its command.
    last_point: HashMap<usize, f64>,
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
            last_point: HashMap::new(),
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

    /// The key at a bar, for ornaments: C major when none is written.
    fn key_at(&self, bar: i64) -> Key {
        let mut k = self.piece.key.unwrap_or(Key {
            tonic: 0,
            minor: false,
        });
        for (b, c) in &self.timed {
            if *b > bar {
                break;
            }
            if let Action::Key(x) = c.action {
                k = x;
            }
        }
        k
    }

    fn bar_beats(&self, bar: i64) -> Ratio {
        if bar == 0 {
            self.piece.pickup
        } else {
            self.meter_at(bar).beats()
        }
    }

    fn run(&mut self, from: i64, to: i64, from_beat: Ratio) -> Rendered {
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
                        // Whole bars are counted bar by bar, so a meter
                        // change inside the ramp is followed exactly.
                        TempoChange::Ramp { length, .. }
                            if length.unit == TimeUnit::Bars && length.count.fract() == 0.0 =>
                        {
                            (g..g + length.count as i64)
                                .map(|x| self.bar_beats(x))
                                .fold(Ratio::ZERO, |a, b| a + b)
                        }
                        TempoChange::Ramp { length, .. } => {
                            let beats = beats_of(length, self.meter_at(g.max(1)));
                            Ratio::new((beats * 1920.0).round() as i64, 1920)
                        }
                        _ => Ratio::ZERO,
                    };
                    self.tempo.change(t, start_beat + c.beat, len);
                }
            }
            // A change at a beat (`at bar 9 beat 3: play fill`) cuts the
            // bar: each stretch is laid out as things stand where it starts.
            let mut cuts: Vec<Ratio> = self
                .timed
                .iter()
                .filter(|(b, c)| {
                    *b == g
                        && c.beat > Ratio::ZERO
                        && matches!(
                            c.action,
                            Action::Play { .. }
                                | Action::Add { .. }
                                | Action::Mute { .. }
                                | Action::Transform { .. }
                        )
                })
                .map(|(_, c)| c.beat)
                .collect();
            cuts.sort();
            cuts.dedup();
            let starts: Vec<Ratio> = std::iter::once(Ratio::ZERO)
                .chain(cuts.iter().copied())
                .collect();
            let mut laid: Vec<(Instance<'p>, Vec<Proto<'p>>)> = Vec::new();
            for (k, &x) in starts.iter().enumerate() {
                let end = starts.get(k + 1).copied();
                let mut instances = self.instances(g, x);
                instances.retain(|i| !self.muted(&i.part.name, g, x));
                for inst in instances {
                    let mut protos = self.lay_out(inst, g, beats, x);
                    if !cuts.is_empty() {
                        protos.retain(|p| p.start >= x && end.is_none_or(|e| p.start < e));
                    }
                    match laid.iter_mut().find(|(i, _)| same_instance(i, &inst)) {
                        Some((_, ps)) => ps.extend(protos),
                        None => laid.push((inst, protos)),
                    }
                }
            }
            let mut fermatas = Vec::new();
            for (_, protos) in &mut laid {
                if !cuts.is_empty() {
                    protos.sort_by_key(|p| p.start);
                }
                for p in protos.iter() {
                    if let Some(h) = p.hit {
                        if h.fermata {
                            fermatas.push((p.start, p.start + p.len, f64::from(h.hold)));
                        }
                    }
                }
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
                        let chord = match p.hit.map(|h| (&h.sound, h.span)) {
                            Some((Sound::Chord(c), s)) => Some((c, s)),
                            Some(_) => None,
                            None => last,
                        };
                        list.push((
                            p.start,
                            p.start + p.len,
                            chord.map(|c| c.0),
                            chord.map(|c| c.1),
                        ));
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
            let mut sets: Vec<(Ratio, &'p Command, f64, bool)> = Vec::new();
            for (b, c) in &self.timed {
                let Action::Param { value, motion, .. } = &c.action else {
                    continue;
                };
                if *b == g && motion.is_none() {
                    let beat = if g == from {
                        from_beat.max(c.beat)
                    } else {
                        c.beat
                    };
                    sets.push((beat, c, *value, false));
                }
                // `from bar 9 to 16` is over: what was in force comes back.
                if c.until == Some(g) {
                    if let Some((v, reset)) = self.restored(c, g) {
                        let beat = if g == from { from_beat } else { Ratio::ZERO };
                        sets.push((beat, c, v, reset));
                    }
                }
            }
            for (beat, c, value, reset) in sets {
                let Action::Param { part, id, .. } = &c.action else {
                    continue;
                };
                let clock = self.clocks.last().unwrap();
                self.params.push((
                    g,
                    Event {
                        bar: g,
                        beat,
                        time: clock.time(beat),
                        duration: 0.0,
                        part: part.clone().unwrap_or_else(|| "all".into()),
                        line: String::new(),
                        kind: EventKind::Param {
                            id: id.clone(),
                            value,
                            glide: 0.0,
                            reset,
                        },
                        velocity: 0,
                        accent: false,
                        also: None,
                        span: c.span,
                        grace: 0,
                    },
                ));
            }
            self.motions(g, from, from_beat);
            g += 1;
        }
        // Whatever is still held ends where rendering stopped.
        let open: Vec<Sounding<'p>> = self.open.drain().map(|(_, s)| s).collect();
        self.done.extend(open);
        self.echoes();
        self.ducks(from, to, from_beat);

        let mut events = Vec::new();
        for s in std::mem::take(&mut self.done) {
            if s.start.0 < from || s.start.0 >= to {
                continue;
            }
            if s.start.0 == from && s.written < from_beat {
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
                    accent: s.accent,
                    span: s.span,
                    also: s.also,
                    grace: s.grace,
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
                beat_times: (0..)
                    .map(Ratio::int)
                    .take_while(|b| *b < c.beats)
                    .map(|b| c.time(b))
                    .collect(),
            })
            .collect();
        Rendered { events, bars }
    }

    /// A place in the piece moved `by` beats later, over bar lines.
    fn later(&self, (bar, offset): (i64, Ratio), by: Ratio) -> (i64, Ratio) {
        let (mut b, mut x) = (bar, offset + by);
        while let Some(c) = self.clocks.get(b.max(0) as usize) {
            if x < c.beats {
                break;
            }
            x -= c.beats;
            b += 1;
        }
        (b, x)
    }

    /// `lead.echo 1 eighth x3`: every sound played again, each echo that
    /// much after the one before and softer.
    fn echoes(&mut self) {
        let mut more = Vec::new();
        for s in &self.done {
            let Some((length, times)) = s.echo else {
                continue;
            };
            let gap = beats_of(length, self.meter_at(s.start.0.max(1)));
            for k in 1..=times {
                let by = Ratio::new((gap * f64::from(k) * 1920.0).round() as i64, 1920);
                let start = self.later(s.start, by);
                let written = self.later((s.start.0, s.written), by).1;
                more.push(Sounding {
                    start,
                    end: self.later(s.end, by),
                    written,
                    velocity: (f64::from(s.velocity) * ECHO.powi(k as i32))
                        .round()
                        .clamp(1.0, 127.0) as u8,
                    accent: false,
                    echo: None,
                    ..s.clone()
                });
            }
        }
        self.done.extend(more);
    }

    /// The volume of a part that ducks, at its points: down at each hit of
    /// what it ducks under, then back over its length, a point every
    /// sixteenth of a beat for the sound to glide between, and the written
    /// volume again at the end.
    fn ducks(&mut self, from: i64, to: i64, from_beat: Ratio) {
        let piece = self.piece;
        let commands: Vec<(Option<i64>, &'p Command)> = self
            .timed
            .iter()
            .map(|(b, c)| (Some(*b), *c))
            .chain(self.every.iter().map(|c| (None, *c)))
            .filter(|(_, c)| matches!(c.action, Action::Duck { .. }))
            .collect();
        if commands.is_empty() {
            return;
        }
        // The duck in force for `target` at a place, the last one written.
        let in_force = |target: &str, g: i64, x: Ratio| -> Option<&'p Command> {
            let mut found = None;
            for (b, c) in &commands {
                let Action::Duck { target: t, .. } = &c.action else {
                    continue;
                };
                if t != target && t != "all" {
                    continue;
                }
                let on = match (b, c.at) {
                    (Some(b), _) => c.in_force(*b, g, x),
                    (None, At::Every { bars, first }) => {
                        let pos = (g - 1).rem_euclid(bars.max(1));
                        if first { pos == 0 } else { pos == bars - 1 }
                    }
                    _ => false,
                };
                if on {
                    found = Some(*c);
                }
            }
            found
        };
        let mut names: Vec<&str> = Vec::new();
        for p in piece.all_parts() {
            if !names.contains(&p.name.as_str()) {
                names.push(&p.name);
            }
        }
        let step = Ratio::new(1, 16);
        for target in names {
            // Its hits: where, how deep, how long back, from what volume.
            let mut hits: Vec<(Ratio, f64, Ratio, f64, &'p Command)> = Vec::new();
            for s in &self.done {
                // A part never ducks under itself; hits long before what is
                // asked for are back up already.
                if s.grace > 0 || s.part == target || s.start.0 < from - 8 || s.start.0 >= to {
                    continue;
                }
                let Some(c) = in_force(target, s.start.0, s.written) else {
                    continue;
                };
                let Action::Duck {
                    source,
                    amount,
                    length,
                    ..
                } = &c.action
                else {
                    continue;
                };
                let from_it = match source {
                    DuckSource::Named(n) => {
                        s.part == n || s.line == n || s.line.ends_with(&format!(".{n}"))
                    }
                    DuckSource::Drum(ch) => s.kinds.iter().any(
                        |(k, _)| matches!(k, EventKind::Drum(d) if Some(*d) == Drum::from_symbol(*ch)),
                    ),
                };
                let Some(clock) = self.clocks.get(s.start.0.max(0) as usize) else {
                    continue;
                };
                if !from_it {
                    continue;
                }
                let back = beats_of(*length, self.meter_at(s.start.0.max(1)));
                let back = Ratio::new((back * 1920.0).round() as i64, 1920).max(step);
                let at = clock.start_beat + s.start.1;
                let volume = self.volume(target, s.start.0, s.written);
                hits.push((at, *amount, back, volume, c));
            }
            hits.sort_by_key(|h| h.0);
            hits.dedup_by_key(|h| h.0);
            let mut last_time: Option<f64> = None;
            for (i, &(at, amount, back, volume, c)) in hits.iter().enumerate() {
                let end = at + back;
                let until = hits.get(i + 1).map_or(end, |h| h.0.min(end));
                let mut points = Vec::new();
                let mut x = at;
                while x < until {
                    let t = ((x - at) / back).to_f64();
                    points.push((x, volume * (1.0 - amount * (1.0 - t) * (1.0 - t))));
                    x += step;
                }
                if until == end {
                    points.push((end, volume));
                }
                for (k, (x, value)) in points.into_iter().enumerate() {
                    let Some((bar, beat)) = self.place(x) else {
                        continue;
                    };
                    let time = self.clocks[bar as usize].time(beat);
                    // Down at once at the hit; along the curve after it.
                    let glide = match last_time {
                        Some(t) if k > 0 => (time - t).max(0.0),
                        _ => 0.0,
                    };
                    last_time = Some(time);
                    if bar < from || bar >= to || (bar == from && beat < from_beat) {
                        continue;
                    }
                    self.params.push((
                        bar,
                        Event {
                            bar,
                            beat,
                            time,
                            duration: 0.0,
                            part: target.to_string(),
                            line: String::new(),
                            kind: EventKind::Param {
                                id: "volume".into(),
                                value,
                                glide,
                                reset: false,
                            },
                            velocity: 0,
                            accent: false,
                            also: None,
                            span: c.span,
                            grace: 0,
                        },
                    ));
                }
            }
        }
    }

    /// The bar and offset of a place counted in beats from the piece's
    /// start.
    fn place(&self, at: Ratio) -> Option<(i64, Ratio)> {
        self.clocks
            .iter()
            .find(|c| c.start_beat <= at && at < c.start_beat + c.beats)
            .map(|c| (c.bar, at - c.start_beat))
    }

    /// A part's volume as written at a place: the last set at a bar, else
    /// its own, else the one for every part.
    fn volume(&self, part: &str, g: i64, x: Ratio) -> f64 {
        let mut set = None;
        for (b, c) in &self.timed {
            if let Action::Param {
                part: Some(p),
                id,
                value,
                motion: None,
                ..
            } = &c.action
            {
                if p == part && id == "volume" && c.in_force(*b, g, x) {
                    set = Some(*value);
                }
            }
        }
        let last = |list: &[crate::model::ParamSet]| {
            list.iter()
                .rev()
                .find(|p| p.id == "volume")
                .map(|p| p.value)
        };
        set.or_else(|| {
            self.piece
                .all_parts()
                .filter(|p| p.name == part)
                .find_map(|p| last(&p.params))
        })
        .or_else(|| last(&self.piece.params))
        .unwrap_or(0.8)
    }

    fn time_of(&self, (bar, offset): (i64, Ratio)) -> f64 {
        match self.clocks.get(bar as usize) {
            Some(c) => c.time(offset),
            None => self.clocks.last().map_or(0.0, |c| c.end_time()),
        }
    }

    /// Whether a strum or an arpeggiator plays the part's chords, which then
    /// sound only through it.
    fn has_strum(&self, part: &Part) -> bool {
        part.lines
            .iter()
            .any(|l| matches!(l.kind, LineKind::Strum | LineKind::Arp))
    }

    /// How long an arpeggiator's step is in bar `g`, in its beats.
    fn arp_step(&self, h: &Hit, g: i64) -> Ratio {
        let Sound::Arp(style) = &h.sound else {
            return Ratio::ONE;
        };
        match style.step {
            ArpStep::PerBeat(n) => Ratio::new(1, i64::from(n.max(1))),
            // A note value, in beats of the meter: in 6/8 an eighth is a
            // third of a beat.
            ArpStep::Note(d) => {
                Ratio::new(1, i64::from(d.max(1))) / self.meter_at(g.max(1)).beat_unit()
            }
        }
    }

    /// The points of the parameters moving in bar `g`, one every sixteenth
    /// of a beat and one where each turns, for the sound to glide between.
    fn motions(&mut self, g: i64, from: i64, from_beat: Ratio) {
        // Only the bar before the first one asked for matters: it tells
        // when the last point was.
        if g + 1 < from {
            return;
        }
        let mut moving: Vec<Moving<'p>> = Vec::new();
        for (b, c) in &self.timed {
            if c.in_bar(*b, g) {
                moving.push((*b, None, *c));
            }
        }
        // `every 8 bars, first:` starts it at the first bar of each eight;
        // `every 8 bars:` has it end where the eight end, like a fill. The
        // eight before ends on this bar's first beat.
        for c in &self.every {
            let At::Every { bars, first } = c.at else {
                continue;
            };
            let bars = bars.max(1);
            let a = g - (g - 1).rem_euclid(bars);
            for a in [a - bars, a] {
                if a >= 1 {
                    moving.push((a, Some((a + bars, first)), *c));
                }
            }
        }
        let clock = &self.clocks[g as usize];
        let beats = clock.beats;
        for (a, stretch, c) in moving {
            let Action::Param {
                part,
                id,
                value,
                motion: Some(m),
                ..
            } = &c.action
            else {
                continue;
            };
            // A later change of the same parameter takes over while it is in
            // force; this one goes on after it, from where it would be.
            let rivals: Vec<(i64, &'p Command)> = self
                .timed
                .iter()
                .filter(|(b, other)| {
                    (*b, other.beat) > (a, c.beat)
                        && !std::ptr::eq(*other, c)
                        && matches!(&other.action, Action::Param { part: p2, id: i2, .. } if p2 == part && i2 == id)
                })
                .map(|(b, other)| (*b, *other))
                .collect();
            let Some(first_beat) = self.clocks.get(a.max(0) as usize).map(|k| k.start_beat) else {
                continue;
            };
            let meter = self.meter_at(a.max(1));
            let rise = beats_of(m.rise, meter);
            let fall = m.fall.map_or(0.0, |l| beats_of(l, meter));
            let cycle = rise + fall;
            if cycle <= 0.0 {
                continue;
            }
            // Where it starts, in beats from the start of the piece.
            let start = match stretch {
                None => first_beat.to_f64() + c.beat.to_f64(),
                Some((e, false)) => {
                    let len = (a..e)
                        .map(|x| self.bar_beats(x))
                        .fold(Ratio::ZERO, |s, b| s + b);
                    ((first_beat + len).to_f64() - cycle).max(first_beat.to_f64())
                }
                _ => first_beat.to_f64(),
            };
            let p0 = clock.start_beat.to_f64() - start;
            let p1 = p0 + beats.to_f64();
            // The grid, and where it turns.
            let mut at: Vec<Ratio> = Vec::new();
            let step = Ratio::new(1, 16);
            let mut x = Ratio::ZERO;
            while x < beats {
                at.push(x);
                x += step;
            }
            let rounds = if m.repeat {
                (p1 / cycle).ceil() as i64
            } else {
                0
            };
            let mut turns = vec![0.0];
            for k in (p0 / cycle).floor().max(0.0) as i64..=rounds {
                turns.push(k as f64 * cycle + rise);
                turns.push((k + 1) as f64 * cycle);
            }
            for t in turns {
                if t >= p0 && t < p1 {
                    at.push(Ratio::new(((t - p0) * 1920.0).round() as i64, 1920));
                }
            }
            at.sort();
            at.dedup();
            let key = std::ptr::from_ref(c) as usize;
            let target = part.clone().unwrap_or_else(|| "all".into());
            for x in at {
                let p = p0 + x.to_f64();
                if p < -1e-9 {
                    continue;
                }
                if rivals.iter().any(|(b, o)| o.in_force(*b, g, x)) {
                    self.last_point.remove(&key);
                    continue;
                }
                // Its stretch is over: only the point on its last bar line.
                if stretch.is_some_and(|(e, _)| g > e || (g == e && x > Ratio::ZERO)) {
                    break;
                }
                if !m.repeat && p > cycle + 1e-9 {
                    break;
                }
                let time = clock.time(x);
                let glide = match self.last_point.get(&key) {
                    Some(t) if p > 1e-9 => (time - t).max(0.0),
                    _ => 0.0,
                };
                self.last_point.insert(key, time);
                if g < from || (g == from && x < from_beat) {
                    continue;
                }
                // Going up again and again without coming down: up to the
                // top, then straight back to the start.
                let rounds_done = p / cycle;
                let saw = m.repeat
                    && fall == 0.0
                    && p > 1e-9
                    && (rounds_done - rounds_done.round()).abs() < 1e-9;
                let mut points = Vec::new();
                if saw {
                    points.push((m.to, glide));
                    points.push((*value, 0.0));
                } else {
                    points.push((motion_value(*value, m, rise, fall, p), glide));
                }
                for (v, glide) in points {
                    self.params.push((
                        g,
                        Event {
                            bar: g,
                            beat: x,
                            time,
                            duration: 0.0,
                            part: target.clone(),
                            line: String::new(),
                            kind: EventKind::Param {
                                id: id.clone(),
                                value: v,
                                glide,
                                reset: false,
                            },
                            velocity: 0,
                            accent: false,
                            also: None,
                            span: c.span,
                            grace: 0,
                        },
                    ));
                }
            }
        }
    }

    /// What a parameter set `from bar 9 to 16` goes back to at its end: an
    /// earlier value written at a bar, or, with `true`, the value its part
    /// is set up with. Nothing when an earlier motion takes it on.
    fn restored(&self, c: &Command, g: i64) -> Option<(f64, bool)> {
        let Action::Param { part, id, .. } = &c.action else {
            return None;
        };
        // The latest earlier change of the same parameter still in force.
        let mut last = None;
        for (b, o) in &self.timed {
            let Action::Param {
                part: p2, id: i2, ..
            } = &o.action
            else {
                continue;
            };
            if !std::ptr::eq(*o, c) && p2 == part && i2 == id && o.in_force(*b, g, Ratio::ZERO) {
                last = Some(&o.action);
            }
        }
        match last {
            None => Some((0.0, true)),
            Some(Action::Param {
                value,
                motion: None,
                ..
            }) => Some((*value, false)),
            Some(_) => None,
        }
    }

    /// Every line sounding at bar `g`.
    fn instances(&self, g: i64, x: Ratio) -> Vec<Instance<'p>> {
        let piece = self.piece;
        let mut out = Vec::new();
        for part in &piece.parts {
            for line in &part.lines {
                // Of a line written `at bar n:` too, the version that has
                // begun last sounds, counting its bars from where it began.
                let sounding = |o: &Line| o.from <= g.max(1) && o.until.is_none_or(|u| g < u);
                let later = part
                    .lines
                    .iter()
                    .any(|o| o.name == line.name && o.from > line.from && sounding(o));
                if later || !sounding(line) {
                    continue;
                }
                if g == 0 {
                    if line.pickup.is_some() && line.from == 1 {
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
                        local: Some(g - line.from),
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

        // The pattern being played, unless an `every` takes its place. A
        // `play ... once` takes the place of the one before only while it
        // lasts; that one keeps counting its bars meanwhile.
        let mut current: Option<(usize, i64)> = None;
        for (b, c) in &self.timed {
            if *b > g {
                break;
            }
            if !c.in_force(*b, g, x) {
                continue;
            }
            if let Action::Play { list, once } = &c.action {
                let lens: Vec<i64> = list
                    .iter()
                    .map(|(i, t)| group_len(&piece.patterns[*i].parts) * *t as i64)
                    .collect();
                let total: i64 = lens.iter().sum();
                if *once && g - b >= total {
                    continue;
                }
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
                Action::Play { ref list, .. } => (list[0].0, false),
                Action::Add { pattern, .. } => (pattern, true),
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
            if !c.in_force(*b, g, x) {
                continue;
            }
            if let Action::Add { pattern: i, once } = c.action {
                let len = group_len(&piece.patterns[i].parts);
                if once && g - b >= len {
                    continue;
                }
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

    /// The transformations in force for an instance at beat `x` of bar `g`.
    fn transforms(&self, inst: &Instance, g: i64, x: Ratio) -> Vec<Transform> {
        self.timed
            .iter()
            .filter(|(b, c)| c.in_force(*b, g, x))
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
    fn lay_out(&self, inst: Instance<'p>, g: i64, beats: Ratio, x: Ratio) -> Vec<Proto<'p>> {
        let line = inst.line;
        let transforms = self.transforms(&inst, g, x);
        let len = line.order.len() as i64;
        if len == 0 && inst.local.is_some() {
            return Vec::new();
        }
        let has = |f: &dyn Fn(&Transform) -> bool| transforms.iter().any(f);
        let retro = has(&|t| matches!(t, Transform::Retrograde | Transform::RetroInvert));
        // How many bars of the line go by in one bar: `fast 2` two, `slow 2`
        // half of one; `diminish` and `augment` are `fast 2` and `slow 2`,
        // and every one in force counts, so `fast 2` and `slow 2` cancel.
        let (mut quick, mut long) = (1i64, 1i64);
        for t in &transforms {
            match t {
                Transform::Fast(n) => quick *= i64::from(*n),
                Transform::Diminish => quick *= 2,
                Transform::Slow(n) => long *= i64::from(*n),
                Transform::Augment => long *= 2,
                _ => {}
            }
        }
        let speed = Ratio::new(quick, long);

        // Which bars of the line, and how each is fitted into this bar:
        // (place in order, written bar, offset, scale, window), the window
        // being the stretch of the scaled bar that sounds here.
        type Piece = (usize, usize, Ratio, Ratio, Option<(Ratio, Ratio)>);
        let mut pieces: Vec<Piece> = Vec::new();
        match inst.local {
            None => pieces.push((PICKUP, line.pickup.unwrap(), Ratio::ZERO, Ratio::ONE, None)),
            Some(j) => {
                let pick = |n: i64| -> usize {
                    if !line.rounds.is_empty() {
                        return random_place(line, self.seed, n);
                    }
                    let k = n.rem_euclid(len);
                    (if retro { len - 1 - k } else { k }) as usize
                };
                if speed == Ratio::ONE {
                    let k = pick(j);
                    pieces.push((k, line.order[k], Ratio::ZERO, Ratio::ONE, None));
                } else {
                    // This bar plays the line from bar `a` to bar `b` of
                    // its own, counted in bars.
                    let scale = Ratio::ONE / speed;
                    let a = Ratio::int(j) * speed;
                    let b = Ratio::int(j + 1) * speed;
                    let first = a.floor();
                    let last = -((-b).floor());
                    for m in first..last {
                        let lo = a.max(Ratio::int(m));
                        let hi = b.min(Ratio::int(m + 1));
                        if lo >= hi {
                            continue;
                        }
                        let k = pick(m);
                        let from = (lo - Ratio::int(m)) * beats * scale;
                        let to = (hi - Ratio::int(m)) * beats * scale;
                        let offset = (lo - a) * beats * scale;
                        pieces.push((k, line.order[k], offset, scale, Some((from, to))));
                    }
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
            let mut at = Ratio::ZERO;
            if line.kind == LineKind::Arp {
                // An arpeggiator's steps, as many as fit in this bar's meter.
                if let Some(Slot::Hit(h)) = written.first().and_then(|b| b.slots.first()) {
                    let step = self.arp_step(h, g);
                    while at < beats {
                        let len = step.min(beats - at);
                        flats.push((at, len, Flat::Hit(h)));
                        at += len;
                    }
                }
            }
            for beat in written.iter().filter(|_| line.kind != LineKind::Arp) {
                let len = share * Ratio::int(beat.len as i64);
                self.flatten(
                    &beat.slots,
                    at,
                    len,
                    line.seed_key,
                    g,
                    &mut counter,
                    &mut flats,
                );
                at += len;
            }
            let bar_len = at;

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
            // A hit written once and played several times in the bar (a
            // Euclidean rhythm, `[c?]x4`) draws its chance anew each time.
            let mut again: HashMap<usize, u64> = HashMap::new();
            for (start, plen, f) in flats {
                match f {
                    Flat::Hit(h) => {
                        protos.extend(cur.take());
                        started = true;
                        let times = again.entry(h.span.start).or_insert(0);
                        let what = 0x5eed ^ h.span.start as u64 ^ (*times << 40);
                        *times += 1;
                        let mut keep = h
                            .chance
                            .is_none_or(|p| chance(self.seed, line.seed_key, g, what) < p);
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
                if let Some((w0, w1)) = window {
                    let (s, e) = (p.start.max(w0), (p.start + p.len).min(w1));
                    if s >= e {
                        continue;
                    }
                    if s > p.start {
                        p.hit = None;
                    }
                    p.start = s - w0;
                    p.len = e - s;
                }
                p.start += offset;
                if window.is_some() {
                    p.open = p.start + p.len >= beats;
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
    fn muted(&self, part: &str, g: i64, x: Ratio) -> bool {
        let applies = |t: &str| t == part || t == "all";
        let mut muted = false;
        for (b, c) in &self.timed {
            if *b > g {
                break;
            }
            if !c.in_force(*b, g, x) {
                continue;
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

    /// How much a live level change (`melodia: cresc to ff over 4 bars`)
    /// scales a note whose written dynamic gives it `written`. The level
    /// moves from what is written to the dynamic asked for, so a part
    /// written in p does reach ff; accents keep their place above it.
    fn level(&self, part: &str, g: i64, frac: f64, written: f64) -> f64 {
        let written = written.max(1.0);
        let mut total = 1.0;
        for target in ["all", part] {
            let mut level = written;
            for (b, c) in &self.timed {
                if *b > g {
                    break;
                }
                let Action::Level {
                    target: t,
                    to,
                    length,
                } = &c.action
                else {
                    continue;
                };
                if t != target || c.until.is_some_and(|u| g >= u) {
                    continue;
                }
                // At a beat: from there on, counting from there.
                let off = c.beat.to_f64() / self.bar_beats(*b).to_f64().max(1e-9);
                if *b == g && frac < off {
                    continue;
                }
                let goal = match to {
                    LevelTarget::Dynamic(d) => d.velocity() as f64,
                    LevelTarget::Silence => 0.0,
                };
                // The length in bars of the meter where it starts.
                let meter = self.meter_at((*b).max(1));
                let bars = beats_of(*length, meter) / meter.beats().to_f64();
                if bars <= 0.0 {
                    level = goal;
                } else {
                    let progress = ((g - b) as f64 + frac - off) / bars;
                    level = if progress >= 1.0 {
                        goal
                    } else {
                        level + (goal - level) * progress
                    };
                }
            }
            total *= level / written;
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
                if !c.in_bar(*b, g) {
                    continue;
                }
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
                // The same way for every part in a bar, whatever it is called:
                // `all: rubato` bends the piece together, as a conductor does.
                let sign = if chance(self.seed, 0, g, 0x4ba7) < 0.5 {
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
        let part = inst.part.name.as_str();
        let (_, _, human) = self.feel(part, g);
        let first_pitch = first_pitch(inst.line);

        // The next hit's level, for a roll that crescendos into it.
        let next_level: Vec<Option<f64>> = (0..protos.len())
            .map(|i| {
                protos[i + 1..]
                    .iter()
                    .find(|q| q.hit.is_some())
                    .map(|q| q.velocity)
            })
            .collect();
        for (index, p) in protos.into_iter().enumerate() {
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
            if h.accent || h.marcato {
                velocity += ACCENT as f64;
            }
            // Marcato is more than an accent; a ghost note barely sounds.
            if h.marcato {
                velocity += ACCENT as f64 * 0.6;
            }
            if h.ghost {
                velocity *= 0.35;
            }
            let mut transpose = 0;
            let mut invert = false;
            let mut echo = None;
            for t in &self.transforms(&inst, g, p.start) {
                match t {
                    Transform::Velocity(x) => velocity *= x,
                    Transform::Transpose(n) => transpose += *n as i32,
                    Transform::Invert | Transform::RetroInvert => invert = true,
                    Transform::Echo { length, times } => echo = Some((*length, *times)),
                    _ => {}
                }
            }
            let frac = (p.start / beats.max(Ratio::new(1, 1000))).to_f64();
            velocity *= self.level(part, g, frac, p.velocity);
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
            let mut also = None;
            match &h.sound {
                Sound::Drum(c) => {
                    let drum = match inst.line.kind {
                        LineKind::Percussion(p) => p.stroke(*c),
                        _ => Drum::from_symbol(*c),
                    };
                    if let Some(d) = drum {
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
                // A step of an arpeggiator: the next note of the chord
                // sounding there, counted from where the chord starts.
                Sound::Arp(style) => {
                    let region = chords
                        .iter()
                        .find(|(s, e, _, _)| p.start >= *s && p.start < *e);
                    let Some((from, _, Some(chord), at)) = region else {
                        continue;
                    };
                    also = *at;
                    let tones = arp_tones(chord, style.octaves);
                    let n = tones.len();
                    let step = p.len.max(Ratio::new(1, 1920));
                    let i = ((p.start - *from) / step).floor().max(0) as usize;
                    let pick = match style.way {
                        ArpWay::Up => i % n,
                        ArpWay::Down => n - 1 - i % n,
                        ArpWay::UpDown if n < 2 => 0,
                        ArpWay::UpDown => {
                            let k = i % (2 * n - 2);
                            if k < n { k } else { 2 * n - 2 - k }
                        }
                        ArpWay::Random => {
                            let r = chance(
                                self.seed,
                                inst.line.seed_key,
                                g,
                                0xa4b ^ (p.start.num() as u64) << 16 ^ p.start.den() as u64,
                            );
                            ((r * n as f64) as usize).min(n - 1)
                        }
                    };
                    kinds.extend(pitch(tones[pick]).map(|k| (k, 0.0)));
                }
                // A tap sounds the top, not the strings, so it needs no chord.
                Sound::Strum(Stroke::Tap) => kinds.push((EventKind::Drum(Drum::GuitarTap), 0.0)),
                Sound::Strum(stroke) => {
                    let region = chords
                        .iter()
                        .find(|(s, e, _, _)| p.start >= *s && p.start < *e);
                    let Some((_, _, Some(chord), at)) = region else {
                        continue;
                    };
                    also = *at;
                    let mut strings = chord.guitar_voicing();
                    let spread = if h.accent { 0.006 } else { 0.012 };
                    match stroke {
                        Stroke::Down | Stroke::DownTap => {}
                        Stroke::Up => {
                            strings.reverse();
                            strings.truncate(4);
                        }
                        Stroke::Muted => fixed = Some(0.04),
                        Stroke::Tap => {}
                    }
                    // The golpe lands with the first string.
                    if *stroke == Stroke::DownTap {
                        kinds.push((EventKind::Drum(Drum::GuitarTap), 0.0));
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
            // A rolled chord: from the lowest note up, a little apart.
            if h.arp {
                kinds.sort_by_key(|(k, _)| match k {
                    EventKind::Note(n) => *n,
                    _ => 0,
                });
                for (i, k) in kinds.iter_mut().enumerate() {
                    k.1 = i as f64 * 0.045;
                }
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
                accent: h.accent || h.marcato,
                staccato: h.staccato,
                fixed,
                part: inst.part.name.as_str(),
                line: inst.line.label.as_str(),
                span: h.span,
                also,
                humanize: human.unwrap_or(0.0),
                jitter,
                grace: 0,
                echo,
            };
            // Grace notes come just before it, in no time of the bar's own:
            // a drum's grace strokes (a flam, a drag) closer and softer.
            let count = h.grace.len();
            for (i, g) in h.grace.iter().enumerate() {
                let (gap, level, kinds) = match g {
                    Sound::Drum(_) => (0.03, 0.45, s.kinds.clone()),
                    Sound::Notes(ns) => (
                        0.06,
                        0.8,
                        ns.iter()
                            .filter_map(|n| pitch(*n))
                            .map(|k| (k, 0.0))
                            .collect(),
                    ),
                    _ => continue,
                };
                let ahead = -gap * (count - i) as f64;
                self.done.push(Sounding {
                    kinds: kinds.into_iter().map(|(k, _)| (k, ahead)).collect(),
                    velocity: (s.velocity as f64 * level).round().clamp(1.0, 127.0) as u8,
                    accent: false,
                    staccato: false,
                    fixed: Some(gap),
                    grace: (count - i) as u8,
                    ..s.clone()
                });
            }
            if let (Some(o), Sound::Notes(ns)) = (h.ornament, &h.sound) {
                let tonality = self.key_at(g);
                let neighbour: Vec<(EventKind, f64)> = ns
                    .iter()
                    .map(|n| match o {
                        Ornament::Trill { alter } => by_degrees(*n, tonality, 1) + alter,
                        Ornament::Mordent { lower } => {
                            by_degrees(*n, tonality, if lower { -1 } else { 1 })
                        }
                    })
                    .filter_map(pitch)
                    .map(|k| (k, 0.0))
                    .collect();
                let until = end_off.min(beats);
                let len = until - p.start;
                // A stroke of an eighth of a beat, as a roll's, or less on a
                // short note.
                let eighth = Ratio::new(1, 8);
                let strokes: Vec<(Ratio, Ratio, bool)> = match o {
                    // The note and its neighbour in turn, starting and
                    // ending on the note.
                    Ornament::Trill { .. } => {
                        let mut n = (len / eighth).floor().max(1);
                        if n % 2 == 0 {
                            n -= 1;
                        }
                        let step = len / Ratio::int(n);
                        (0..n)
                            .map(|k| {
                                let a = p.start + step * Ratio::int(k);
                                (a, a + step, k % 2 == 0)
                            })
                            .collect()
                    }
                    // The note, its neighbour, then the note held.
                    Ornament::Mordent { .. } => {
                        let step = eighth.min(len / Ratio::int(3));
                        vec![
                            (p.start, p.start + step, true),
                            (p.start + step, p.start + step * Ratio::int(2), false),
                        ]
                    }
                };
                let last = strokes.last().map_or(p.start, |x| x.1);
                for (k, (a, b, own)) in strokes.into_iter().enumerate() {
                    self.done.push(Sounding {
                        start: (g, self.warp(part, g, beats, a)),
                        end: (g, self.warp(part, g, beats, b)),
                        written: a,
                        kinds: if own {
                            s.kinds.clone()
                        } else {
                            neighbour.clone()
                        },
                        accent: s.accent && k == 0,
                        ..s.clone()
                    });
                }
                if matches!(o, Ornament::Trill { .. }) {
                    continue;
                }
                // The mordent's last note is the note itself, held as written.
                let s = Sounding {
                    start: (g, self.warp(part, g, beats, last)),
                    written: last,
                    accent: false,
                    ..s
                };
                if p.open {
                    self.open.insert(key, s);
                } else {
                    self.done.push(s);
                }
                continue;
            }
            if h.roll {
                // A roll: strokes of an eighth of a beat (thirty-seconds in
                // 4/4) over the hit's length, the hands alternating, rising
                // or falling to the next hit under a hairpin.
                let step = Ratio::new(1, 8);
                let until = end_off.min(beats);
                let goal = match (h.hairpin, next_level[index]) {
                    (Some(_), Some(next)) if p.velocity > 0.0 => next / p.velocity,
                    _ => 1.0,
                };
                let span_beats = (until - p.start).to_f64().max(1e-9);
                let mut at = p.start;
                let mut stroke = 0;
                while at < until {
                    let t = (at - p.start).to_f64() / span_beats;
                    let hand = if stroke % 2 == 0 { 1.0 } else { 0.88 };
                    let level = s.velocity as f64 * (1.0 + (goal - 1.0) * t) * hand;
                    let next = (at + step).min(until);
                    self.done.push(Sounding {
                        start: (g, self.warp(part, g, beats, at)),
                        end: (g, self.warp(part, g, beats, next)),
                        written: at,
                        velocity: level.round().clamp(1.0, 127.0) as u8,
                        accent: s.accent && stroke == 0,
                        ..s.clone()
                    });
                    at = next;
                    stroke += 1;
                }
                continue;
            }
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

/// The same line from the same place, laid out in two stretches of a bar.
fn same_instance(a: &Instance, b: &Instance) -> bool {
    std::ptr::eq(a.line, b.line)
        && std::ptr::eq(a.part, b.part)
        && a.origin == b.origin
        && a.local == b.local
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

/// The notes an arpeggiator goes through, low to high: the chord's notes
/// from the third octave, as a keyboard's chord sits, over `octaves`.
fn arp_tones(chord: &Chord, octaves: u8) -> Vec<i32> {
    let mut tones: Vec<i32> = (0..i32::from(octaves.max(1)))
        .flat_map(|o| {
            chord
                .intervals
                .iter()
                .map(move |i| 48 + chord.root + i + 12 * o)
        })
        .collect();
    tones.sort_unstable();
    tones.dedup();
    tones
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
