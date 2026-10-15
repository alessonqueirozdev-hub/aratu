//! Checking an exercise: does what the student wrote sound like the answer?
//!
//! The two pieces are compared by what they play, not by how they are
//! written, so `c.c` and `c__c`, or `dó4` and `c4`, are the same answer. The
//! differences are told in a musician's words: which bar, which beat, which
//! note is missing, extra, too long or too soft.

use crate::diag::Lang;
use crate::model::Piece;
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
}

impl Default for CheckOptions {
    fn default() -> CheckOptions {
        CheckOptions {
            bars: None,
            lengths: true,
            dynamics: false,
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
}

fn drum_index(d: Drum) -> u8 {
    match d {
        Drum::Kick => 0,
        Drum::Snare => 1,
        Drum::ClosedHat => 2,
        Drum::OpenHat => 3,
        Drum::Ride => 4,
        Drum::Crash => 5,
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

fn sound_name(s: Heard, lang: Lang) -> String {
    match s {
        Heard::Note(n) => note_name(n, lang),
        Heard::Drum(d) => {
            let names: [[&str; 3]; 6] = [
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
            ];
            let col = match lang {
                Lang::Pt => 0,
                Lang::Es => 1,
                Lang::En => 2,
            };
            names[d as usize][col].to_string()
        }
    }
}

/// "tempo 3", "a metade do tempo 3", "tempo 3 + 1/4".
fn where_(bar: i64, beat: Ratio, lang: Lang) -> String {
    let n = beat.floor() + 1;
    let frac = beat - Ratio::int(beat.floor());
    let b = match lang {
        Lang::Pt if frac.is_zero() => format!("tempo {n}"),
        Lang::Pt if frac == Ratio::new(1, 2) => format!("metade do tempo {n}"),
        Lang::Pt => format!("tempo {n} + {frac}"),
        Lang::Es if frac.is_zero() => format!("tiempo {n}"),
        Lang::Es if frac == Ratio::new(1, 2) => format!("mitad del tiempo {n}"),
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
            "No compasso 1, metade do tempo 1: faltou o chimbal fechado."
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
