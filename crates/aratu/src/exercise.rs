//! Checking an exercise: does what the student wrote sound like the answer?
//!
//! The two pieces are compared by what they play, not by how they are
//! written, so `c.c` and `c__c`, or `dó4` and `c4`, are the same answer. The
//! differences are told in a musician's words: which bar, which beat, which
//! note is missing, extra, too long or too soft. When asked, also when the
//! notes come (the tempo, a swing, a fermata) and how they sound (each
//! part's voice, its effects and its numbers).

use crate::diag::Lang;
use crate::model::{ParamSet, Part, Piece};
use crate::ratio::Ratio;
use crate::schedule::{Drum, EventKind, Options, render};
use crate::theory::Dyn;

/// What to compare besides which sounds start where.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckOptions {
    /// How many bars to compare, from bar 1 (and the pickup, if any); by
    /// default one time round of the answer's longest line.
    pub bars: Option<i64>,
    /// Compare how long each note lasts.
    pub lengths: bool,
    /// Compare dynamics.
    pub dynamics: bool,
    /// Compare when each sound comes, in seconds: the tempo and its
    /// changes, swing, fermatas.
    pub timing: bool,
    /// Compare each part's voice, effects and numbers, and the master's.
    pub sound: bool,
}

impl Default for CheckOptions {
    fn default() -> CheckOptions {
        CheckOptions {
            bars: None,
            lengths: true,
            dynamics: false,
            timing: false,
            sound: false,
        }
    }
}

/// A sound, named for the student.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Heard {
    Note(u8),
    Drum(u8),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Problem {
    Missing {
        bar: i64,
        beat: Ratio,
        sound: Heard,
    },
    Extra {
        bar: i64,
        beat: Ratio,
        sound: Heard,
    },
    Length {
        bar: i64,
        beat: Ratio,
        sound: Heard,
        expected: f64,
        found: f64,
    },
    Dynamic {
        bar: i64,
        beat: Ratio,
        sound: Heard,
        expected: u8,
        found: u8,
    },
    /// The piece's tempo, at its start.
    Tempo {
        expected: f64,
        found: f64,
    },
    /// A sound in its place, but early (negative) or late, in seconds.
    Time {
        bar: i64,
        beat: Ratio,
        sound: Heard,
        off: f64,
    },
    /// A part plays another voice; "" is the part's default voice.
    Voice {
        part: String,
        expected: String,
        found: String,
    },
    /// A part should go through an effect (`missing`), or should not.
    Effect {
        part: String,
        effect: String,
        missing: bool,
    },
    /// A number of a part, of every part, or of the master.
    Number {
        part: Option<String>,
        id: String,
        expected: f64,
        found: f64,
    },
    /// A parameter set while playing, at a bar and beat: where it should be
    /// (`None`: it should not change there) and where it is (`None`: it
    /// does not change there).
    Change {
        bar: i64,
        beat: Ratio,
        /// `part.id`, or the id alone.
        name: String,
        id: String,
        expected: Option<f64>,
        found: Option<f64>,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Key {
    bar: i64,
    beat: Ratio,
    sound: Heard,
}

struct Played {
    key: Key,
    /// Length in beats at the bar's tempo.
    beats: f64,
    velocity: u8,
    /// When it starts, in seconds from the first bar compared.
    time: f64,
}

fn drum_index(d: Drum) -> u8 {
    match d {
        Drum::Kick => 0,
        Drum::Snare => 1,
        Drum::ClosedHat => 2,
        Drum::OpenHat => 3,
        Drum::Ride => 4,
        Drum::Crash => 5,
        Drum::HighTom => 6,
        Drum::MidTom => 7,
        Drum::FloorTom => 8,
        Drum::SideStick => 9,
        Drum::GuitarTap => 10,
        Drum::Clap => 11,
        // Percussion strokes count by their place after the kit.
        other => 12 + other as u8,
    }
}

fn played(piece: &Piece, bars: i64) -> Vec<Played> {
    let from = if piece.pickup.is_zero() { 1 } else { 0 };
    let r = render(piece, &Options::default(), from, bars + 1);
    let mut out: Vec<Played> = r
        .events
        .iter()
        .filter_map(|e| {
            let sound = match e.kind {
                EventKind::Note(n) => Heard::Note(n),
                EventKind::Drum(d) => Heard::Drum(drum_index(d)),
                EventKind::Param { .. } => return None,
            };
            let bpm = r
                .bars
                .iter()
                .find(|b| b.bar == e.bar)
                .map_or(120.0, |b| b.bpm);
            Some(Played {
                key: Key {
                    bar: e.bar,
                    beat: e.beat,
                    sound,
                },
                beats: e.duration * bpm / 60.0,
                velocity: e.velocity,
                time: e.time,
            })
        })
        .collect();
    out.sort_by_key(|p| p.key);
    // A chord strummed plays the same note once per string at most; keep one.
    out.dedup_by_key(|p| p.key);
    out
}

/// Compares the student's piece with the answer.
pub fn check(student: &Piece, answer: &Piece, opts: CheckOptions) -> Vec<Problem> {
    let bars = opts.bars.unwrap_or_else(|| one_round(answer));
    let a = played(answer, bars);
    let s = played(student, bars);
    let mut problems = Vec::new();
    // Another tempo puts every note out of place: said once, as a tempo.
    let mut timing = opts.timing;
    if timing && (answer.tempo - student.tempo).abs() > 0.5 {
        problems.push(Problem::Tempo {
            expected: answer.tempo,
            found: student.tempo,
        });
        timing = false;
    }
    let (mut i, mut j) = (0, 0);
    while i < a.len() || j < s.len() {
        match (a.get(i), s.get(j)) {
            (Some(x), Some(y)) if x.key == y.key => {
                let k = x.key;
                if opts.lengths
                    && matches!(k.sound, Heard::Note(_))
                    && (x.beats - y.beats).abs() > 0.02
                {
                    problems.push(Problem::Length {
                        bar: k.bar,
                        beat: k.beat,
                        sound: k.sound,
                        expected: x.beats,
                        found: y.beats,
                    });
                }
                if timing && (x.time - y.time).abs() > 0.012 {
                    problems.push(Problem::Time {
                        bar: k.bar,
                        beat: k.beat,
                        sound: k.sound,
                        off: y.time - x.time,
                    });
                }
                if opts.dynamics && x.velocity.abs_diff(y.velocity) > 6 {
                    problems.push(Problem::Dynamic {
                        bar: k.bar,
                        beat: k.beat,
                        sound: k.sound,
                        expected: x.velocity,
                        found: y.velocity,
                    });
                }
                i += 1;
                j += 1;
            }
            (Some(x), Some(y)) if x.key < y.key => {
                problems.push(missing(x));
                i += 1;
            }
            (Some(_), Some(y)) => {
                problems.push(extra(y));
                j += 1;
            }
            (Some(x), None) => {
                problems.push(missing(x));
                i += 1;
            }
            (None, Some(y)) => {
                problems.push(extra(y));
                j += 1;
            }
            (None, None) => break,
        }
    }
    if opts.sound {
        problems.extend(sound(student, answer));
        problems.extend(changes(student, answer, bars));
    }
    problems
}

/// The names of a piece's parts, each once, in the order they first appear.
fn part_names(piece: &Piece) -> Vec<&str> {
    let mut names: Vec<&str> = Vec::new();
    for p in piece.all_parts() {
        if !names.contains(&p.name.as_str()) {
            names.push(&p.name);
        }
    }
    names
}

/// The student's part for one of the answer's: the part of the same name,
/// else the part in the same place, so that the same exercise written in
/// another language, with its parts named in it, is the same answer.
fn counterpart<'a>(student: &'a Piece, answer: &Piece, name: &str) -> Option<&'a Part> {
    if let Some(p) = student.all_parts().find(|p| p.name == name) {
        return Some(p);
    }
    let place = part_names(answer).iter().position(|n| *n == name)?;
    let other = *part_names(student).get(place)?;
    student.all_parts().find(|p| p.name == other)
}

/// A parameter set while playing.
struct Set {
    /// Bar, beat, and the parameter with its part by place (`#0.reverb.mix`).
    key: (i64, Ratio, String),
    value: f64,
    /// `part.id` as written, or the id alone.
    name: String,
}

/// Where a parameter is set while playing: `at bar 9:`, or along the way of
/// one that moves.
fn set_while_playing(piece: &Piece, bars: i64) -> Vec<Set> {
    let from = if piece.pickup.is_zero() { 1 } else { 0 };
    let r = render(piece, &Options::default(), from, bars + 1);
    let names = part_names(piece);
    let mut out: Vec<Set> = r
        .events
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::Param { id, value, .. } => {
                let (key, name) = match names.iter().position(|n| *n == e.part) {
                    Some(i) => (format!("#{i}.{id}"), format!("{}.{id}", e.part)),
                    None => (id.clone(), id.clone()),
                };
                Some(Set {
                    key: (e.bar, e.beat, key),
                    value: *value,
                    name,
                })
            }
            _ => None,
        })
        .collect();
    out.sort_by(|a, b| a.key.cmp(&b.key));
    out
}

/// The parameters set while playing, told once per parameter: the first
/// place where the student's differs.
fn changes(student: &Piece, answer: &Piece, bars: i64) -> Vec<Problem> {
    let a = set_while_playing(answer, bars);
    let s = set_while_playing(student, bars);
    let mut told: Vec<String> = Vec::new();
    let mut problems = Vec::new();
    let mut keys: Vec<&(i64, Ratio, String)> = a.iter().chain(s.iter()).map(|x| &x.key).collect();
    keys.sort();
    keys.dedup();
    for k in keys {
        if told.contains(&k.2) {
            continue;
        }
        let find = |list: &[Set]| list.iter().find(|x| &x.key == k).map(|x| x.value);
        let (x, y) = (find(&a), find(&s));
        // Named as the answer names it, else as the student does.
        let name = a
            .iter()
            .chain(s.iter())
            .find(|x| &x.key == k)
            .map_or(String::new(), |x| x.name.clone());
        // The id: the key without its part, when it has one.
        let id = match k.2.strip_prefix('#') {
            Some(rest) => rest.split_once('.').map_or(rest, |(_, id)| id).to_string(),
            None => k.2.clone(),
        };
        let differs = match (x, y) {
            (Some(x), Some(y)) => !same_number(&id, x, y),
            _ => true,
        };
        if differs {
            told.push(k.2.clone());
            problems.push(Problem::Change {
                bar: k.0,
                beat: k.1,
                name,
                id,
                expected: x,
                found: y,
            });
        }
    }
    problems
}

/// A part's voice, canonical; "" when the chain names none.
fn voice_of(part: &Part) -> String {
    part.chain
        .first()
        .and_then(|c| crate::voices::find(c))
        .map_or(String::new(), |v| v.code.to_string())
}

fn effects_of(part: &Part) -> Vec<&str> {
    part.chain
        .iter()
        .map(String::as_str)
        .filter(|c| crate::voices::EFFECTS.contains(c))
        .collect()
}

/// The number a part ends up with: its own, else the one written for every
/// part, else the default.
fn number(part: Option<&Part>, piece: &Piece, id: &str) -> Option<f64> {
    let last = |list: &[ParamSet]| list.iter().rev().find(|p| p.id == id).map(|p| p.value);
    part.and_then(|p| last(&p.params))
        .or_else(|| last(&piece.params))
        .or_else(|| crate::params::lookup(id).map(|i| i.default))
}

/// Two numbers are the same when their knobs are in the same place.
fn same_number(id: &str, a: f64, b: f64) -> bool {
    match crate::params::lookup(id) {
        Some(info) => {
            (crate::params::to_knob(a, info) - crate::params::to_knob(b, info)).abs() < 0.01
        }
        None => (a - b).abs() < 1e-6,
    }
}

/// How the two pieces sound: each part's voice, effects and numbers, and
/// the master's. Parts are matched by name, or by place (see
/// [`counterpart`]), in sections and patterns too, once each, as the sound
/// has one of each; a part the student has not written is already told by
/// its notes.
fn sound(student: &Piece, answer: &Piece) -> Vec<Problem> {
    let mut problems = Vec::new();
    let mut seen: Vec<&str> = Vec::new();
    for a in answer.all_parts() {
        if seen.contains(&a.name.as_str()) {
            continue;
        }
        seen.push(&a.name);
        let Some(s) = counterpart(student, answer, &a.name) else {
            continue;
        };
        let (va, vs) = (voice_of(a), voice_of(s));
        if va != vs {
            problems.push(Problem::Voice {
                part: a.name.clone(),
                expected: va,
                found: vs,
            });
        }
        let (ea, es) = (effects_of(a), effects_of(s));
        for e in ea.iter().filter(|e| !es.contains(e)) {
            problems.push(Problem::Effect {
                part: a.name.clone(),
                effect: e.to_string(),
                missing: true,
            });
        }
        for e in es.iter().filter(|e| !ea.contains(e)) {
            problems.push(Problem::Effect {
                part: a.name.clone(),
                effect: e.to_string(),
                missing: false,
            });
        }
        let mut ids: Vec<&str> = [a, s]
            .iter()
            .flat_map(|p| p.params.iter())
            .chain(answer.params.iter())
            .chain(student.params.iter())
            .map(|p| p.id.as_str())
            .filter(|id| !id.starts_with("master."))
            .collect();
        ids.sort_unstable();
        ids.dedup();
        for id in ids {
            let (Some(x), Some(y)) = (number(Some(a), answer, id), number(Some(s), student, id))
            else {
                continue;
            };
            if !same_number(id, x, y) {
                problems.push(Problem::Number {
                    part: Some(a.name.clone()),
                    id: id.to_string(),
                    expected: x,
                    found: y,
                });
            }
        }
    }
    let mut ids: Vec<&str> = answer
        .params
        .iter()
        .chain(student.params.iter())
        .map(|p| p.id.as_str())
        .filter(|id| id.starts_with("master."))
        .collect();
    ids.sort_unstable();
    ids.dedup();
    for id in ids {
        let (Some(x), Some(y)) = (number(None, answer, id), number(None, student, id)) else {
            continue;
        };
        if !same_number(id, x, y) {
            problems.push(Problem::Number {
                part: None,
                id: id.to_string(),
                expected: x,
                found: y,
            });
        }
    }
    problems
}

/// The bars of one time round: the longest line of the piece.
fn one_round(piece: &Piece) -> i64 {
    piece
        .all_parts()
        .flat_map(|p| p.lines.iter())
        .map(|l| l.order.len() as i64)
        .max()
        .unwrap_or(1)
        .max(1)
}

fn missing(p: &Played) -> Problem {
    Problem::Missing {
        bar: p.key.bar,
        beat: p.key.beat,
        sound: p.key.sound,
    }
}

fn extra(p: &Played) -> Problem {
    Problem::Extra {
        bar: p.key.bar,
        beat: p.key.beat,
        sound: p.key.sound,
    }
}

/// A note's name: solfège in Portuguese and Spanish, letters in English.
pub fn note_name(n: u8, lang: Lang) -> String {
    let octave = n as i32 / 12 - 1;
    let pc = n as usize % 12;
    let name = match lang {
        Lang::En => [
            "C", "C#", "D", "Eb", "E", "F", "F#", "G", "Ab", "A", "Bb", "B",
        ][pc],
        Lang::Pt => [
            "dó", "dó#", "ré", "mib", "mi", "fá", "fá#", "sol", "láb", "lá", "sib", "si",
        ][pc],
        Lang::Es => [
            "do", "do#", "re", "mib", "mi", "fa", "fa#", "sol", "lab", "la", "sib", "si",
        ][pc],
    };
    format!("{name}{octave}")
}

/// The kit's sounds by their index, then the guitar's golpe and the clap.
const KIT_NAMES: [[&str; 3]; 12] = [
    ["o bumbo", "el bombo", "the bass drum"],
    ["a caixa", "la caja", "the snare"],
    [
        "o chimbal fechado",
        "el hi-hat cerrado",
        "the closed hi-hat",
    ],
    ["o chimbal aberto", "el hi-hat abierto", "the open hi-hat"],
    ["a condução", "el ride", "the ride"],
    ["o ataque", "el crash", "the crash"],
    ["o tom agudo", "el tom agudo", "the high tom"],
    ["o tom médio", "el tom medio", "the mid tom"],
    ["o surdo da bateria", "el tom de piso", "the floor tom"],
    ["o aro", "el aro", "the side stick"],
    [
        "o golpe no tampo",
        "el golpe en la tapa",
        "the golpe on the top",
    ],
    ["a palma", "la palmada", "the clap"],
];

/// The percussion strokes, in the order of `Drum` from the pandeiro's thumb.
const PERC_NAMES: [[&str; 3]; 24] = [
    [
        "o pandeiro (polegar)",
        "el pandeiro (pulgar)",
        "the pandeiro (thumb)",
    ],
    [
        "o pandeiro (dedos)",
        "el pandeiro (dedos)",
        "the pandeiro (fingers)",
    ],
    [
        "o pandeiro (tapa)",
        "el pandeiro (palmada)",
        "the pandeiro (slap)",
    ],
    [
        "o pandeiro (abafado)",
        "el pandeiro (apagado)",
        "the pandeiro (muted)",
    ],
    [
        "a zabumba (aberta)",
        "la zabumba (abierta)",
        "the zabumba (open)",
    ],
    [
        "a zabumba (abafada)",
        "la zabumba (apagada)",
        "the zabumba (muted)",
    ],
    [
        "o bacalhau da zabumba",
        "el bacalhau de la zabumba",
        "the zabumba's bacalhau",
    ],
    [
        "o triângulo (aberto)",
        "el triángulo (abierto)",
        "the triangle (open)",
    ],
    [
        "o triângulo (fechado)",
        "el triángulo (cerrado)",
        "the triangle (closed)",
    ],
    ["o agogô (agudo)", "el agogó (agudo)", "the agogô (high)"],
    ["o agogô (grave)", "el agogó (grave)", "the agogô (low)"],
    ["o surdo (aberto)", "el surdo (abierto)", "the surdo (open)"],
    [
        "o surdo (abafado)",
        "el surdo (apagado)",
        "the surdo (muted)",
    ],
    [
        "o tamborim (aberto)",
        "el tamborim (abierto)",
        "the tamborim (open)",
    ],
    [
        "o tamborim (abafado)",
        "el tamborim (apagado)",
        "the tamborim (muted)",
    ],
    [
        "a alfaia (aberta)",
        "la alfaia (abierta)",
        "the alfaia (open)",
    ],
    [
        "a alfaia (abafada)",
        "la alfaia (apagada)",
        "the alfaia (muted)",
    ],
    [
        "o gonguê (aberto)",
        "el gonguê (abierto)",
        "the gonguê (open)",
    ],
    [
        "o gonguê (abafado)",
        "el gonguê (apagado)",
        "the gonguê (muted)",
    ],
    ["o ganzá", "el ganzá", "the shaker"],
    ["a cuíca (aguda)", "la cuíca (aguda)", "the cuíca (high)"],
    ["a cuíca (grave)", "la cuíca (grave)", "the cuíca (low)"],
    ["o cajón (grave)", "el cajón (grave)", "the cajón (bass)"],
    [
        "o cajón (tapa)",
        "el cajón (golpe agudo)",
        "the cajón (slap)",
    ],
];

fn sound_name(s: Heard, lang: Lang) -> String {
    let col = match lang {
        Lang::Pt => 0,
        Lang::Es => 1,
        Lang::En => 2,
    };
    match s {
        Heard::Note(n) => note_name(n, lang),
        Heard::Drum(d) => {
            let first = 12 + Drum::PandeiroThumb as u8;
            let names = match d {
                0..12 => KIT_NAMES.get(d as usize),
                _ => d
                    .checked_sub(first)
                    .and_then(|i| PERC_NAMES.get(i as usize)),
            };
            names.map_or("?", |n| n[col]).to_string()
        }
    }
}

/// A voice by its code and name; "" is the part's default voice.
fn voice_name(code: &str, lang: Lang) -> String {
    match crate::voices::find(code) {
        Some(v) => format!("{} ({})", v.code, v.name(lang)),
        None => match lang {
            Lang::Pt => "o som padrão",
            Lang::Es => "el sonido por defecto",
            Lang::En => "the default sound",
        }
        .to_string(),
    }
}

/// A number as it is written, with its unit: `800 Hz`, `0.5`, `-6 dB`.
fn number_text(id: &str, x: f64) -> String {
    let unit = crate::params::lookup(id).map_or("", |i| i.unit);
    let n = format!("{:.2}", x);
    let n = n.trim_end_matches('0').trim_end_matches('.');
    if unit.is_empty() {
        n.to_string()
    } else {
        format!("{n} {unit}")
    }
}

/// "tempo 3", "a metade do tempo 3", "tempo 3 + 1/4".
fn where_(bar: i64, beat: Ratio, lang: Lang) -> String {
    let n = beat.floor() + 1;
    let frac = beat - Ratio::int(beat.floor());
    let b = match lang {
        Lang::Pt if frac.is_zero() => format!("tempo {n}"),
        Lang::Pt if frac == Ratio::new(1, 2) => format!("segunda metade do tempo {n}"),
        Lang::Pt => format!("tempo {n} + {frac}"),
        Lang::Es if frac.is_zero() => format!("tiempo {n}"),
        Lang::Es if frac == Ratio::new(1, 2) => format!("segunda mitad del tiempo {n}"),
        Lang::Es => format!("tiempo {n} + {frac}"),
        Lang::En if frac.is_zero() => format!("beat {n}"),
        Lang::En if frac == Ratio::new(1, 2) => format!("the \"and\" of beat {n}"),
        Lang::En => format!("beat {n} + {frac}"),
    };
    match lang {
        Lang::Pt => format!("No compasso {bar}, {b}"),
        Lang::Es => format!("En el compás {bar}, {b}"),
        Lang::En => format!("In bar {bar}, {b}"),
    }
}

/// A length in beats, in words: "1 tempo", "1/2 tempo", "1 1/2 tempos".
fn beats(x: f64, lang: Lang) -> String {
    let twelfths = (x * 12.0).round() as i64;
    let r = Ratio::new(twelfths, 12);
    let whole = r.floor();
    let frac = r - Ratio::int(whole);
    let number = match (whole, frac.is_zero()) {
        (w, true) => format!("{w}"),
        (0, false) => format!("{frac}"),
        (w, false) => format!("{w} {frac}"),
    };
    let one = r <= Ratio::ONE;
    let unit = match (lang, one) {
        (Lang::Pt, true) => "tempo",
        (Lang::Pt, false) => "tempos",
        (Lang::Es, true) => "tiempo",
        (Lang::Es, false) => "tiempos",
        (Lang::En, true) => "beat",
        (Lang::En, false) => "beats",
    };
    format!("{number} {unit}")
}

/// The nearest dynamic mark to a velocity.
fn mark(v: u8) -> &'static str {
    const MARKS: [(u8, &str); 8] = [
        (16, "ppp"),
        (33, "pp"),
        (49, "p"),
        (64, "mp"),
        (80, "mf"),
        (96, "f"),
        (112, "ff"),
        (127, "fff"),
    ];
    let _ = Dyn::Mf;
    MARKS
        .iter()
        .min_by_key(|(x, _)| x.abs_diff(v))
        .map_or("mf", |(_, m)| m)
}

impl Problem {
    /// The problem in words.
    pub fn text(&self, lang: Lang) -> String {
        match *self {
            Problem::Missing { bar, beat, sound } => {
                let w = where_(bar, beat, lang);
                let s = sound_name(sound, lang);
                match lang {
                    Lang::Pt => format!("{w}: faltou {s}."),
                    Lang::Es => format!("{w}: falta {s}."),
                    Lang::En => format!("{w}: {s} is missing."),
                }
            }
            Problem::Extra { bar, beat, sound } => {
                let w = where_(bar, beat, lang);
                let s = sound_name(sound, lang);
                match lang {
                    Lang::Pt => format!("{w}: {s} não estava pedido aqui."),
                    Lang::Es => format!("{w}: {s} no se pedía aquí."),
                    Lang::En => format!("{w}: {s} was not asked for here."),
                }
            }
            Problem::Length {
                bar,
                beat,
                sound,
                expected,
                found,
            } => {
                let w = where_(bar, beat, lang);
                let s = sound_name(sound, lang);
                let (e, f) = (beats(expected, lang), beats(found, lang));
                match lang {
                    Lang::Pt => format!("{w}: {s} devia durar {e}, mas dura {f}."),
                    Lang::Es => format!("{w}: {s} debería durar {e}, pero dura {f}."),
                    Lang::En => format!("{w}: {s} should last {e}, but lasts {f}."),
                }
            }
            Problem::Dynamic {
                bar,
                beat,
                sound,
                expected,
                found,
            } => {
                let w = where_(bar, beat, lang);
                let s = sound_name(sound, lang);
                let (e, f) = (mark(expected), mark(found));
                match lang {
                    Lang::Pt => format!("{w}: {s} devia soar {e}, mas soa {f}."),
                    Lang::Es => format!("{w}: {s} debería sonar {e}, pero suena {f}."),
                    Lang::En => format!("{w}: {s} should sound {e}, but sounds {f}."),
                }
            }
            Problem::Tempo { expected, found } => {
                let (e, f) = (expected.round(), found.round());
                match lang {
                    Lang::Pt => format!("O andamento devia ser {e}, mas é {f}."),
                    Lang::Es => format!("El tempo debería ser {e}, pero es {f}."),
                    Lang::En => format!("The tempo should be {e}, but is {f}."),
                }
            }
            Problem::Time {
                bar,
                beat,
                sound,
                off,
            } => {
                let w = where_(bar, beat, lang);
                let s = sound_name(sound, lang);
                let ms = (off.abs() * 1000.0).round();
                match (lang, off < 0.0) {
                    (Lang::Pt, true) => format!("{w}: {s} chega {ms} ms antes do esperado."),
                    (Lang::Pt, false) => format!("{w}: {s} chega {ms} ms depois do esperado."),
                    (Lang::Es, true) => format!("{w}: {s} llega {ms} ms antes de lo esperado."),
                    (Lang::Es, false) => format!("{w}: {s} llega {ms} ms después de lo esperado."),
                    (Lang::En, true) => format!("{w}: {s} comes {ms} ms early."),
                    (Lang::En, false) => format!("{w}: {s} comes {ms} ms late."),
                }
            }
            Problem::Voice {
                ref part,
                ref expected,
                ref found,
            } => {
                let (e, f) = (voice_name(expected, lang), voice_name(found, lang));
                match lang {
                    Lang::Pt => format!("A parte {part} devia tocar com {e}, mas toca com {f}."),
                    Lang::Es => {
                        format!("La parte {part} debería sonar con {e}, pero suena con {f}.")
                    }
                    Lang::En => format!("The part {part} should play {e}, but plays {f}."),
                }
            }
            Problem::Effect {
                ref part,
                ref effect,
                missing,
            } => match (lang, missing) {
                (Lang::Pt, true) => format!("A parte {part} devia passar pelo {effect}."),
                (Lang::Pt, false) => format!("A parte {part} não precisa do {effect}."),
                (Lang::Es, true) => format!("La parte {part} debería pasar por el {effect}."),
                (Lang::Es, false) => format!("La parte {part} no necesita el {effect}."),
                (Lang::En, true) => format!("The part {part} should go through the {effect}."),
                (Lang::En, false) => format!("The part {part} does not need the {effect}."),
            },
            Problem::Number {
                ref part,
                ref id,
                expected,
                found,
            } => {
                let name = match part {
                    Some(p) => format!("{p}.{id}"),
                    None => id.clone(),
                };
                let (e, f) = (number_text(id, expected), number_text(id, found));
                match lang {
                    Lang::Pt => format!("{name} devia ser {e}, mas é {f}."),
                    Lang::Es => format!("{name} debería ser {e}, pero es {f}."),
                    Lang::En => format!("{name} should be {e}, but is {f}."),
                }
            }
            Problem::Change {
                bar,
                beat,
                ref name,
                ref id,
                expected,
                found,
            } => {
                let w = where_(bar, beat, lang);
                let n = |x: f64| number_text(id, x);
                match (lang, expected, found) {
                    (Lang::Pt, Some(e), Some(f)) => {
                        format!("{w}: {name} devia estar em {}, mas está em {}.", n(e), n(f))
                    }
                    (Lang::Pt, Some(e), None) => format!("{w}: {name} devia mudar para {}.", n(e)),
                    (Lang::Pt, None, _) => format!("{w}: {name} não devia mudar aqui."),
                    (Lang::Es, Some(e), Some(f)) => {
                        format!(
                            "{w}: {name} debería estar en {}, pero está en {}.",
                            n(e),
                            n(f)
                        )
                    }
                    (Lang::Es, Some(e), None) => format!("{w}: {name} debería cambiar a {}.", n(e)),
                    (Lang::Es, None, _) => format!("{w}: {name} no debería cambiar aquí."),
                    (Lang::En, Some(e), Some(f)) => {
                        format!("{w}: {name} should be at {}, but is at {}.", n(e), n(f))
                    }
                    (Lang::En, Some(e), None) => format!("{w}: {name} should change to {}.", n(e)),
                    (Lang::En, None, _) => format!("{w}: {name} should not change here."),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compile;

    fn check_src(student: &str, answer: &str, opts: CheckOptions) -> Vec<Problem> {
        let s = compile(student);
        assert!(!s.has_errors(), "{student}");
        let a = compile(answer);
        assert!(!a.has_errors(), "{answer}");
        check(&s.piece, &a.piece, opts)
    }

    fn ok(student: &str, answer: &str) {
        let p = check_src(student, answer, CheckOptions::default());
        assert!(p.is_empty(), "{student} vs {answer}: {p:?}");
    }

    #[test]
    fn different_spellings_of_the_same_music_pass() {
        ok("hh = c.c c__c cc_c -c", "hh = c__c c.c cc_c -c");
        ok("m = | dó4 ré4 mi4 fá4 |", "m = | c4 d e f |");
        ok("m = | c4 _ _ _ |", "m = c4 _ _ _");
        ok("drums { hh = cc cc cc cc }", "hh = cc cc cc cc");
        ok("m = | [c4 d4] e4 _ - |", "m = | [c4 d] e _ - |");
    }

    #[test]
    fn a_wrong_note_is_named_where_it_is() {
        let p = check_src(
            "m = | c4 d4 e4 f4 |",
            "m = | c4 d4 g4 f4 |",
            CheckOptions::default(),
        );
        assert_eq!(p.len(), 2);
        assert_eq!(
            p[0].text(Lang::Pt),
            "No compasso 1, tempo 3: mi4 não estava pedido aqui."
        );
        assert_eq!(p[1].text(Lang::Pt), "No compasso 1, tempo 3: faltou sol4.");
        assert_eq!(p[1].text(Lang::Es), "En el compás 1, tiempo 3: falta sol4.");
        assert_eq!(p[1].text(Lang::En), "In bar 1, beat 3: G4 is missing.");
    }

    #[test]
    fn rhythm_and_length_problems() {
        let p = check_src("hh = c c c c", "hh = cc c c c", CheckOptions::default());
        assert_eq!(
            p[0].text(Lang::Pt),
            "No compasso 1, segunda metade do tempo 1: faltou o chimbal fechado."
        );
        let p = check_src(
            "m = | c4 _ d4 e4 |",
            "m = | c4 d4 _ e4 |",
            CheckOptions::default(),
        );
        let texts: Vec<String> = p.iter().map(|x| x.text(Lang::Pt)).collect();
        assert!(
            texts.contains(
                &"No compasso 1, tempo 1: dó4 devia durar 1 tempo, mas dura 2 tempos.".to_string()
            ),
            "{texts:?}"
        );
    }

    #[test]
    fn dynamics_are_compared_when_asked() {
        let opts = CheckOptions {
            dynamics: true,
            ..CheckOptions::default()
        };
        let p = check_src("m = | p:c4 d4 e4 f4 |", "m = | f:c4 d4 e4 f4 |", opts);
        assert_eq!(
            p[0].text(Lang::Pt),
            "No compasso 1, tempo 1: dó4 devia soar f, mas soa p."
        );
        assert!(
            check_src(
                "m = | p:c4 d4 e4 f4 |",
                "m = | f:c4 d4 e4 f4 |",
                CheckOptions::default()
            )
            .is_empty()
        );
    }

    #[test]
    fn every_drum_and_stroke_has_a_name() {
        let p = check_src("tom = - - - -", "tom = - - - tf", CheckOptions::default());
        assert_eq!(
            p[0].text(Lang::Pt),
            "No compasso 1, tempo 4: faltou o tom agudo."
        );
        let p = check_src(
            "surdo = o - - -",
            "surdo = o - m -",
            CheckOptions::default(),
        );
        assert_eq!(
            p[0].text(Lang::Es),
            "En el compás 1, tiempo 3: falta el surdo (apagado)."
        );
        let p = check_src(
            "cajón = b - - -",
            "cajón = b - - t",
            CheckOptions::default(),
        );
        assert_eq!(
            p[0].text(Lang::En),
            "In bar 1, beat 4: the cajón (slap) is missing."
        );
    }

    #[test]
    fn when_the_notes_come_is_compared_when_asked() {
        let opts = CheckOptions {
            timing: true,
            ..CheckOptions::default()
        };
        let p = check_src("hh = cc cc cc cc", "swing 66%\nhh = cc cc cc cc", opts);
        assert!(
            matches!(p[0], Problem::Time { off, .. } if off < 0.0),
            "{p:?}"
        );
        assert!(
            check_src(
                "hh = cc cc cc cc",
                "swing 66%\nhh = cc cc cc cc",
                CheckOptions::default()
            )
            .is_empty()
        );
        let p = check_src("tempo: 100\nhh = c c c c", "tempo: 120\nhh = c c c c", opts);
        assert_eq!(p.len(), 1);
        assert_eq!(p[0].text(Lang::Pt), "O andamento devia ser 120, mas é 100.");
    }

    #[test]
    fn the_sound_is_compared_when_asked() {
        let opts = CheckOptions {
            sound: true,
            ..CheckOptions::default()
        };
        let p = check_src(
            "baixo: synth.saw -> chorus\nbaixo = | c2 _ _ _ |",
            "baixo: bass.slap -> delay\nbaixo = | c2 _ _ _ |\nbaixo.delay.mix = 0.5",
            opts,
        );
        let texts: Vec<String> = p.iter().map(|p| p.text(Lang::Pt)).collect();
        assert!(
            texts
                .iter()
                .any(|t| t.starts_with("A parte baixo devia tocar com bass.slap")),
            "{texts:?}"
        );
        assert!(
            texts.contains(&"A parte baixo devia passar pelo delay.".to_string()),
            "{texts:?}"
        );
        assert!(
            texts.contains(&"A parte baixo não precisa do chorus.".to_string()),
            "{texts:?}"
        );
        assert!(
            texts
                .iter()
                .any(|t| t.starts_with("baixo.delay.mix devia ser 0.5")),
            "{texts:?}"
        );
        // A number for every part is the same as the number on each.
        let p = check_src(
            "baixo = | c2 _ _ _ |\nreverb.mix = 0.5",
            "baixo = | c2 _ _ _ |\nbaixo.reverb.mix = 0.5",
            opts,
        );
        assert!(p.is_empty(), "{p:?}");
        let p = check_src(
            "baixo = | c2 _ _ _ |",
            "baixo = | c2 _ _ _ |\nmaster.reverb.decay = 3 s",
            opts,
        );
        assert_eq!(
            p[0].text(Lang::En),
            "master.reverb.decay should be 3 s, but is 2.2 s."
        );
        // Numbers set while playing, and numbers that move.
        let p = check_src(
            "baixo = | c2 _ _ _ |",
            "baixo = | c2 _ _ _ |\nat bar 2: baixo.reverb.mix = 0.6",
            CheckOptions {
                bars: Some(2),
                ..opts
            },
        );
        assert_eq!(
            p[0].text(Lang::Pt),
            "No compasso 2, tempo 1: baixo.reverb.mix devia mudar para 0.6."
        );
        let moving = "baixo: synth.saw -> lowpass\nbaixo = | c2 _ _ _ |\nbaixo.lowpass.cutoff = 300 Hz -> 4000 Hz over 2 bars";
        let p = check_src(moving, moving, opts);
        assert!(p.is_empty(), "{p:?}");
        let p = check_src(
            &moving.replace("4000", "2000"),
            moving,
            CheckOptions {
                bars: Some(2),
                ..opts
            },
        );
        assert_eq!(p.len(), 1, "{p:?}");
    }

    #[test]
    fn only_the_bars_asked_for_count() {
        let opts = CheckOptions {
            bars: Some(1),
            ..CheckOptions::default()
        };
        assert!(
            check_src(
                "m = | c4 _ _ _ | d4 _ _ _ |",
                "m = | c4 _ _ _ | e4 _ _ _ |",
                opts
            )
            .is_empty()
        );
    }
}

#[test]
fn a_part_going_out_is_heard_as_silence() {
    use crate::compile;
    let s = compile("drums { hh = c c c c }\nbaixo = | dó2 _ _ _ |");
    let a = compile(
        "drums { hh = c c c c }\nbaixo = | dó2 _ _ _ |\nat bar 2: drums: out\nat bar 3: drums: in",
    );
    assert!(!a.has_errors(), "{:?}", a.diagnostics);
    let opts = CheckOptions {
        bars: Some(4),
        ..CheckOptions::default()
    };
    let p = check(&s.piece, &a.piece, opts);
    assert!(!p.is_empty(), "{:?}", a.piece.commands);
}
