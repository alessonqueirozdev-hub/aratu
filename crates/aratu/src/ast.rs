//! The piece as written: statements in order, and music lines as bars,
//! beats and the parts that share each beat.

use crate::span::Span;
use crate::theory::{Chord, Dyn, Key, Meter};

/// A whole piece: every statement in the order written.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Ast {
    pub stmts: Vec<Stmt>,
}

/// Where a statement was written.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Scope {
    Top,
    Pattern(String),
    Section(String),
}

/// When a statement takes effect.
#[derive(Clone, Debug, PartialEq)]
pub enum When {
    /// Written without a time: from the start, or, while playing live, from
    /// the bar line after it was written.
    Bare,
    /// `at bar 30:`
    AtBar(i64, Span),
    /// `every 4 bars:` (the last bar of each group) or `every 8 bars, first:`.
    Every { bars: i64, first: bool, span: Span },
    /// `now:` at the next beat when playing live.
    Now,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Stmt {
    pub scope: Scope,
    /// The part block the statement is in, such as `drums` in `drums { ... }`.
    pub part: Option<Name>,
    pub when: When,
    pub kind: StmtKind,
    pub span: Span,
}

/// A name with where it is written.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Name {
    pub text: String,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StmtKind {
    // The frame of the piece.
    Meter(Meter),
    Tempo(f64),
    Key(Key),
    /// `pickup: beat 4`: the piece starts on that beat of an incomplete bar.
    Pickup(i64),
    Seed(Seed),
    Form(Vec<FormEntry>),

    /// `hh = cc cc co cc`
    Line(LineDef),
    /// `bass.reverb.mix = 0.42`, or `reverb.mix = 0.42` inside `bass { }`.
    Param {
        path: Vec<Name>,
        value: Value,
    },
    /// `bass: synth.saw -> lowpass -> reverb`
    Chain {
        target: Option<Name>,
        units: Vec<Name>,
    },
    /// `piano: mf`
    Level {
        target: Name,
        dynamic: Dyn,
    },
    /// `guitar: out` (silent from then on) or `guitar: in` (back).
    Mute {
        target: Name,
        muted: bool,
    },
    /// `piano: cresc to ff over 4 bars`, `all: fade out over 8 bars`
    LevelRamp {
        target: Name,
        to: LevelTarget,
        bars: i64,
    },
    /// `swing 60%`, `piano: rubato 10%`, `humanize 5 ms`
    Feel {
        target: Option<Name>,
        feel: Feel,
    },

    /// `play groove x3, fill`
    Play(Vec<PlayEntry>),
    /// `add crash`: a pattern played on top of the current one.
    Add(Name),

    /// `tempo 130`, `rit to 80 over 2 bars`, `a tempo`, `tempo primo`
    TempoChange(TempoChange),
    /// `meter 7/8` after `at bar 17:`
    MeterChange(Meter),
    /// `key D major` after `at bar 17:`
    KeyChange(Key),

    /// `hh.rotate 1`, `bass.transpose +12`
    Transform {
        target: Name,
        transform: Transform,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seed {
    Fixed(u64),
    Random,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FormEntry {
    pub section: Name,
    pub times: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlayEntry {
    pub pattern: Name,
    pub times: u32,
}

/// A number with an optional unit, `800 Hz`, `0.42`, `10%`.
#[derive(Clone, Debug, PartialEq)]
pub struct Value {
    pub number: f64,
    pub unit: Option<String>,
    /// Where the number itself is written, for the knob that rewrites it.
    pub number_span: Span,
    pub span: Span,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LevelTarget {
    Dynamic(Dyn),
    /// `fade out`
    Silence,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Feel {
    /// Delay of the second eighth, 50% straight, 66% triplet feel.
    Swing(f64),
    /// How much time is robbed and given back, in percent of a beat.
    Rubato(f64),
    /// Random deviation in milliseconds.
    Humanize(f64),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TempoChange {
    Set(f64),
    /// `rit`/`rall`/`accel to N over B bars`, or `tempo A -> N over B bars`.
    Ramp {
        from: Option<f64>,
        to: f64,
        bars: i64,
    },
    ATempo,
    Primo,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Transform {
    Rotate(i64),
    Density(f64),
    Velocity(f64),
    Transpose(i64),
    Invert,
    Retrograde,
    Augment,
    Diminish,
}

/// How the symbols of a line are read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LineKind {
    /// Notes with octaves: `c4 e g`.
    Pitched,
    /// Drum symbols: `cc cc co cc`.
    Drums,
    /// A lead sheet: `| Am | F | C G |`.
    Chords,
    /// A strum pattern: `D du -u du`.
    Strum,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LineDef {
    pub name: Name,
    pub kind: LineKind,
    pub bars: Vec<Bar>,
    /// Where the music after `=` is written.
    pub music_span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Bar {
    pub span: Span,
    pub content: BarContent,
    /// `|:` before the bar.
    pub repeat_start: bool,
    /// `|1`, `|2` before the bar: it starts that ending.
    pub ending: Option<u32>,
    /// `:|` after the bar, with how many times the passage is played.
    pub repeat_end: Option<u32>,
    pub markers: Vec<(Marker, Span)>,
    pub jump: Option<(Jump, Span)>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BarContent {
    Beats(Vec<Beat>),
    /// `%` (1) or `%%` (2): repeat the previous bar or two.
    Simile(u8),
    /// `| - |`: a whole bar of rest in any meter; `| - x8 |`: eight bars.
    Rest(u32),
}

/// What sounds in one beat (or, in a chord line, one equal share of the bar).
#[derive(Clone, Debug, PartialEq)]
pub struct Beat {
    pub slots: Vec<Slot>,
    pub span: Span,
}

/// One equal part of a beat or of a group.
#[derive(Clone, Debug, PartialEq)]
pub enum Slot {
    Hit(Hit),
    /// `_`: the sound before lasts one more part.
    Hold(Span),
    /// `-`
    Rest(Span),
    /// `[d e]`: the parts inside share this one.
    Group(Vec<Slot>, Span),
    /// `{a|b}`: one of these, chosen each time round.
    Choice(Vec<Vec<Slot>>, Span),
}

impl Slot {
    pub fn span(&self) -> Span {
        match self {
            Slot::Hit(h) => h.span,
            Slot::Hold(s) | Slot::Rest(s) | Slot::Group(_, s) | Slot::Choice(_, s) => *s,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Hit {
    pub span: Span,
    pub sound: Sound,
    pub accent: bool,
    pub staccato: bool,
    pub fermata: bool,
    /// `?` (0.5) or `?30%` (0.3): the chance the hit is played.
    pub chance: Option<f64>,
    pub hairpin: Option<Hairpin>,
    /// `p:` written before it.
    pub dynamic: Option<Dyn>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Sound {
    /// One or more notes sounding together, as MIDI note numbers.
    Notes(Vec<i32>),
    Drum(char),
    Strum(Stroke),
    Chord(Chord),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stroke {
    Down,
    Up,
    Muted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hairpin {
    Crescendo,
    Diminuendo,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Marker {
    Segno,
    Fine,
    Coda,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Jump {
    /// `D.C.` (to the start) or `D.S.` (to the segno), with where to stop.
    Back { to_segno: bool, al: Al },
    /// `to coda`
    ToCoda,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Al {
    /// Play on to the end.
    End,
    Fine,
    Coda,
}

impl Jump {
    pub fn words(self) -> &'static str {
        match self {
            Jump::Back {
                to_segno: false,
                al: Al::End,
            } => "D.C.",
            Jump::Back {
                to_segno: false,
                al: Al::Fine,
            } => "D.C. al fine",
            Jump::Back {
                to_segno: false,
                al: Al::Coda,
            } => "D.C. al coda",
            Jump::Back {
                to_segno: true,
                al: Al::End,
            } => "D.S.",
            Jump::Back {
                to_segno: true,
                al: Al::Fine,
            } => "D.S. al fine",
            Jump::Back {
                to_segno: true,
                al: Al::Coda,
            } => "D.S. al coda",
            Jump::ToCoda => "to coda",
        }
    }
}
