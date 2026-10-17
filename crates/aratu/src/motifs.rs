//! Music named once and used many times: motifs (`motif riff = [e3 d3 c3
//! d3]`), twelve-tone rows (`row serie = 0 11 3 4 …`) and melody lists
//! (`list temas { … }`), and what can be done to them: transposition by
//! degrees of the key or by semitones, inversion, retrograde and retrograde
//! inversion.
//!
//! The parser collects every definition before it reads the lines, so a
//! motif may be used above the line that defines it.

use std::collections::HashMap;

use crate::ast::{Bar, BarContent, Beat, Hit, Slot, Sound};
use crate::theory::{Key, accidentals, note_name};

/// Written music kept by name: its text, and where that text starts in the
/// source, so the editor lights the definition while it plays.
#[derive(Clone, Debug, PartialEq)]
pub struct Source {
    pub text: String,
    pub base: usize,
}

/// Everything a piece defines by name.
#[derive(Clone, Debug, Default)]
pub struct Library {
    pub motifs: HashMap<String, Source>,
    pub rows: HashMap<String, Vec<i32>>,
    pub lists: HashMap<String, Vec<Source>>,
    /// The piece's first key, for moving motifs by degrees (C major when
    /// none is written).
    pub key: Option<Key>,
}

/// A note written `o` in a rhythm, waiting for a pitch from a row.
pub const PLACEHOLDER: i32 = i32::MIN;

/// How deep motifs may use motifs before it counts as a loop.
pub const MAX_DEPTH: u8 = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Op {
    /// Up (or down) this many degrees of the key.
    Degrees(i32),
    Semitones(i32),
    /// Every interval turned upside down around the first note.
    Invert,
    /// The pitches in reverse order, in the same rhythm.
    Retrograde,
}

/// A use of a motif: `riff`, `riff+2`, `riff-1`, `riff+3st`, `riff.inv`,
/// `riff.retro`, `riff.ri+2`. `None` when `word` is not one.
pub fn motif_ref<'a>(word: &'a str, lib: &Library) -> Option<(&'a str, Vec<Op>)> {
    let end = word.find(['.', '+', '-']).unwrap_or(word.len());
    let name = &word[..end];
    if !lib.motifs.contains_key(name) {
        return None;
    }
    let mut ops = Vec::new();
    let mut rest = &word[end..];
    while let Some(r) = rest.strip_prefix('.') {
        let len = r.find(['.', '+', '-']).unwrap_or(r.len());
        let (moves, flips) = mirror(&r[..len])?;
        if moves {
            ops.push(Op::Invert);
        }
        if flips {
            ops.push(Op::Retrograde);
        }
        rest = &r[len..];
    }
    if let Some(sign) = rest.chars().next().filter(|c| *c == '+' || *c == '-') {
        let digits = rest[1..].chars().take_while(char::is_ascii_digit).count();
        if digits == 0 {
            return None;
        }
        let n: i32 = rest[1..1 + digits].parse().ok()?;
        let n = if sign == '-' { -n } else { n };
        rest = &rest[1 + digits..];
        if let Some(r) = rest.strip_prefix("st") {
            ops.push(Op::Semitones(n));
            rest = r;
        } else {
            ops.push(Op::Degrees(n));
        }
    }
    rest.is_empty().then_some((name, ops))
}

/// The names of inversion, retrograde and their mix, the same for motifs,
/// rows and lines: `inv`, `invert`, `inversion` or `I`; `retro`,
/// `retrograde` or `R`; `ri` or `RI`. Whether it turns the intervals
/// upside down, and whether it plays backwards.
pub fn mirror(word: &str) -> Option<(bool, bool)> {
    Some(match crate::words::fold(word).as_str() {
        "inv" | "invert" | "inversion" | "i" | "inversao" | "inverter" | "inverte" | "invertir" => {
            (true, false)
        }
        "retro" | "retrograde" | "r" | "retrogrado" => (false, true),
        "ri" | "retroinv" => (true, true),
        _ => return None,
    })
}

/// Splits a repeat count off a word: `[e3 d3]x4`, `riffx4`. Only after a
/// closing bracket or a motif, so `cx4` in a drum line stays a closed hat,
/// a crash and an error.
pub fn split_times<'a>(word: &'a str, lib: &Library) -> (&'a str, u32) {
    let Some(at) = word.rfind(['x', '×']) else {
        return (word, 1);
    };
    let sign = word[at..].chars().next().map_or(1, char::len_utf8);
    let digits = &word[at + sign..];
    // `x4`, or `4x` as people also say it.
    let (core, n) = if digits.is_empty() {
        let head = &word[..at];
        let d = head.len() - head.trim_end_matches(|c: char| c.is_ascii_digit()).len();
        match head[head.len() - d..].parse::<u32>() {
            Ok(n) if d > 0 => (&head[..head.len() - d], n),
            _ => return (word, 1),
        }
    } else {
        match digits.parse::<u32>() {
            Ok(n) => (&word[..at], n),
            Err(_) => return (word, 1),
        }
    };
    let after_bracket = core.ends_with(']') || core.ends_with('}');
    if n >= 1 && (after_bracket || motif_ref(core, lib).is_some()) {
        (core, n)
    } else {
        (word, 1)
    }
}

/// Does `ops` to the notes of some beats, in order.
pub fn apply(beats: &mut [Beat], ops: &[Op], key: Key) {
    for op in ops {
        match *op {
            Op::Semitones(n) => {
                each_note(beats, &mut |v| v.iter_mut().for_each(|p| *p += n));
                each_grace(beats, &mut |v| v.iter_mut().for_each(|p| *p += n));
            }
            Op::Degrees(k) => {
                each_note(beats, &mut |v| {
                    v.iter_mut().for_each(|p| *p = by_degrees(*p, key, k))
                });
                each_grace(beats, &mut |v| {
                    v.iter_mut().for_each(|p| *p = by_degrees(*p, key, k))
                });
            }
            Op::Invert => {
                let mut pivot = None;
                each_note(beats, &mut |v| {
                    let c = *pivot.get_or_insert(v.first().copied().unwrap_or(60));
                    v.iter_mut().for_each(|p| *p = 2 * c - *p);
                });
                if let Some(c) = pivot {
                    each_grace(beats, &mut |v| v.iter_mut().for_each(|p| *p = 2 * c - *p));
                }
            }
            Op::Retrograde => {
                let mut all = Vec::new();
                each_note(beats, &mut |v| all.push(v.clone()));
                let before: Vec<i32> = all
                    .iter()
                    .map(|v| v.first().copied().unwrap_or(0))
                    .collect();
                all.reverse();
                let mut next = all.into_iter();
                each_note(beats, &mut |v| {
                    if let Some(n) = next.next() {
                        *v = n;
                    }
                });
                // A grace note keeps its distance to the note it leads into.
                let mut i = 0;
                each_hit(beats, &mut |h| {
                    let Sound::Notes(v) = &h.sound else {
                        return;
                    };
                    let moved =
                        v.first().copied().unwrap_or(0) - before.get(i).copied().unwrap_or(0);
                    i += 1;
                    for g in &mut h.grace {
                        if let Sound::Notes(gv) = g {
                            gv.iter_mut().for_each(|p| *p += moved);
                        }
                    }
                });
            }
        }
    }
}

/// Calls `f` on each note's pitches, in the order they are written.
pub fn each_note(beats: &mut [Beat], f: &mut dyn FnMut(&mut Vec<i32>)) {
    each_hit(beats, &mut |h| {
        if let Sound::Notes(v) = &mut h.sound {
            f(v);
        }
    });
}

/// Calls `f` on each grace note's pitches.
fn each_grace(beats: &mut [Beat], f: &mut dyn FnMut(&mut Vec<i32>)) {
    each_hit(beats, &mut |h| {
        for g in &mut h.grace {
            if let Sound::Notes(v) = g {
                f(v);
            }
        }
    });
}

/// Calls `f` on each hit, in the order they are written.
fn each_hit(beats: &mut [Beat], f: &mut dyn FnMut(&mut Hit)) {
    fn slots(s: &mut [Slot], f: &mut dyn FnMut(&mut Hit)) {
        for slot in s {
            match slot {
                Slot::Hit(h) => f(h),
                Slot::Group(inner, _) => slots(inner, f),
                Slot::Choice(options, _) => options.iter_mut().for_each(|o| slots(o, f)),
                Slot::Hold(_) | Slot::Rest(_) => {}
            }
        }
    }
    for b in beats {
        slots(&mut b.slots, f);
    }
}

/// Every note of some bars, in order.
pub fn each_note_in_bars(bars: &mut [Bar], f: &mut dyn FnMut(&mut Vec<i32>)) {
    for bar in bars {
        if let BarContent::Beats(beats) = &mut bar.content {
            each_note(beats, f);
        }
    }
}

const MAJOR: [i32; 7] = [0, 2, 4, 5, 7, 9, 11];
const MINOR: [i32; 7] = [0, 2, 3, 5, 7, 8, 10];

/// A pitch moved `k` degrees in a key; a note outside the scale keeps its
/// distance from the scale note below it.
pub fn by_degrees(pitch: i32, key: Key, k: i32) -> i32 {
    let scale = if key.minor { MINOR } else { MAJOR };
    let rel = pitch - key.tonic;
    let octave = rel.div_euclid(12);
    let pc = rel.rem_euclid(12);
    let i = scale.iter().rposition(|&s| s <= pc).unwrap_or(0);
    let chroma = pc - scale[i];
    let idx = octave * 7 + i as i32 + k;
    key.tonic + idx.div_euclid(7) * 12 + scale[idx.rem_euclid(7) as usize] + chroma
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Form {
    /// Prime: the row as written.
    P,
    I,
    R,
    RI,
}

/// A row's form, `n` semitones up: P0 is the row as written, I0 its
/// inversion on the same first note, R0 and RI0 those two backwards.
pub fn row_form(row: &[i32], form: Form, n: i32) -> Vec<i32> {
    let first = row.first().copied().unwrap_or(60);
    let mut out: Vec<i32> = match form {
        Form::P | Form::R => row.iter().map(|p| p + n).collect(),
        Form::I | Form::RI => row.iter().map(|p| 2 * first - p + n).collect(),
    };
    if matches!(form, Form::R | Form::RI) {
        out.reverse();
    }
    out
}

/// `serie`, `serie.P0`, `serie.I5`, `serie.R11`, `serie.RI3`: a row's name
/// and form. `Err` with the form written when it is not one.
pub fn form_ref(word: &str) -> Result<(&str, Form, i32), String> {
    let Some((name, form)) = word.split_once('.') else {
        return Ok((word, Form::P, 0));
    };
    // The same names as for motifs and lines, with the transposition after:
    // `serie.inv`, `serie.retrograde5`.
    let letters = form.trim_end_matches(|c: char| c.is_ascii_digit());
    if letters.len() > 2 {
        let f = match mirror(letters) {
            Some((true, true)) => Form::RI,
            Some((true, false)) => Form::I,
            Some((false, true)) => Form::R,
            _ if ["prime", "original"].contains(&crate::words::fold(letters).as_str()) => Form::P,
            _ => return Err(form.to_string()),
        };
        let digits = &form[letters.len()..];
        let n = if digits.is_empty() {
            Ok(0)
        } else {
            digits.parse::<i32>()
        };
        return match n {
            Ok(n) if (0..12).contains(&n) => Ok((name, f, n)),
            _ => Err(form.to_string()),
        };
    }
    let upper = form.to_uppercase();
    let (f, digits) = if let Some(d) = upper.strip_prefix("RI") {
        (Form::RI, d)
    } else if let Some(d) = upper.strip_prefix('P') {
        (Form::P, d)
    } else if let Some(d) = upper.strip_prefix('I') {
        (Form::I, d)
    } else if let Some(d) = upper.strip_prefix('R') {
        (Form::R, d)
    } else {
        return Err(form.to_string());
    };
    let n = if digits.is_empty() {
        Ok(0)
    } else {
        digits.parse::<i32>()
    };
    match n {
        Ok(n) if (0..12).contains(&n) => Ok((name, f, n)),
        _ => Err(form.to_string()),
    }
}

/// One note of a row: a pitch class 0–11 (placed in octave 4) or a note
/// name, with or without its octave (`fá#`, `eb4`).
pub fn row_note(token: &str) -> Option<i32> {
    if let Ok(n) = token.parse::<i32>() {
        return (0..12).contains(&n).then_some(60 + n);
    }
    let (pc, _, len) = note_name(token)?;
    let rest = &token[len..];
    let (acc, alen) = accidentals(rest);
    let rest = &rest[alen..];
    let octave = if rest.is_empty() {
        4
    } else {
        rest.parse::<i32>().ok().filter(|o| (-1..=9).contains(o))?
    };
    Some((octave + 1) * 12 + pc + acc)
}

/// A place in a row's text: where a token starts and how long it is.
pub type Token = (usize, usize);

/// A row's pitches, its tokens that are not notes, and the first token
/// repeating a pitch class in a twelve-note row.
pub fn read_row(text: &str) -> (Vec<i32>, Vec<Token>, Option<Token>) {
    let mut pitches = Vec::new();
    let mut bad = Vec::new();
    let mut tokens = Vec::new();
    let mut pos = 0;
    for token in text.split_whitespace() {
        let at = text[pos..].find(token).map_or(pos, |i| pos + i);
        pos = at + token.len();
        match row_note(token) {
            Some(p) => {
                pitches.push(p);
                tokens.push((at, token.len()));
            }
            None => bad.push((at, token.len())),
        }
    }
    let mut repeated = None;
    if pitches.len() == 12 {
        let mut seen = [false; 12];
        for (i, p) in pitches.iter().enumerate() {
            let pc = p.rem_euclid(12) as usize;
            if seen[pc] {
                repeated = Some(tokens[i]);
                break;
            }
            seen[pc] = true;
        }
    }
    (pitches, bad, repeated)
}

/// Puts `pitches` into the `o`s of some bars, in order, starting again from
/// the first when they run out. Returns how many `o`s there were.
pub fn fill(bars: &mut [Bar], pitches: &[i32]) -> usize {
    let mut count = 0;
    each_note_in_bars(bars, &mut |v| {
        if v.len() == 1 && v[0] == PLACEHOLDER {
            if !pitches.is_empty() {
                v[0] = pitches[count % pitches.len()];
            }
            count += 1;
        }
    });
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lib() -> Library {
        let mut l = Library::default();
        l.motifs.insert(
            "riff".into(),
            Source {
                text: "[e3 d3 c3 d3]".into(),
                base: 0,
            },
        );
        l
    }

    #[test]
    fn motif_references_read_their_moves() {
        let l = lib();
        assert_eq!(motif_ref("riff", &l), Some(("riff", vec![])));
        assert_eq!(
            motif_ref("riff+2", &l),
            Some(("riff", vec![Op::Degrees(2)]))
        );
        assert_eq!(
            motif_ref("riff-1", &l),
            Some(("riff", vec![Op::Degrees(-1)]))
        );
        assert_eq!(
            motif_ref("riff+3st", &l),
            Some(("riff", vec![Op::Semitones(3)]))
        );
        assert_eq!(
            motif_ref("riff.ri+2", &l),
            Some(("riff", vec![Op::Invert, Op::Retrograde, Op::Degrees(2)]))
        );
        assert_eq!(motif_ref("riff.foo", &l), None);
        assert_eq!(motif_ref("other", &l), None);
    }

    #[test]
    fn repeat_counts_follow_brackets_and_motifs_only() {
        let l = lib();
        assert_eq!(split_times("[e3 d3]x4", &l), ("[e3 d3]", 4));
        assert_eq!(split_times("riffx3", &l), ("riff", 3));
        assert_eq!(split_times("riff+2x2", &l), ("riff+2", 2));
        assert_eq!(split_times("cx4", &l), ("cx4", 1));
    }

    #[test]
    fn degrees_follow_the_key() {
        let c = Key {
            tonic: 0,
            minor: false,
        };
        assert_eq!(by_degrees(60, c, 1), 62);
        assert_eq!(by_degrees(64, c, 1), 65, "E to F is a degree");
        assert_eq!(by_degrees(71, c, 1), 72, "over the octave");
        assert_eq!(by_degrees(60, c, -1), 59);
        assert_eq!(by_degrees(61, c, 1), 63, "C# keeps its sharp: D#");
        let am = Key {
            tonic: 9,
            minor: true,
        };
        assert_eq!(by_degrees(52, am, 1), 53, "E to F in A minor");
    }

    #[test]
    fn row_forms() {
        let row = [60, 71, 63, 64];
        assert_eq!(row_form(&row, Form::P, 0), row);
        assert_eq!(row_form(&row, Form::P, 2), [62, 73, 65, 66]);
        assert_eq!(row_form(&row, Form::I, 0), [60, 49, 57, 56]);
        assert_eq!(row_form(&row, Form::R, 0), [64, 63, 71, 60]);
        assert_eq!(row_form(&row, Form::RI, 0), [56, 57, 49, 60]);
        assert_eq!(form_ref("serie"), Ok(("serie", Form::P, 0)));
        assert_eq!(form_ref("serie.RI3"), Ok(("serie", Form::RI, 3)));
        assert_eq!(form_ref("serie.i11"), Ok(("serie", Form::I, 11)));
        assert!(form_ref("serie.X3").is_err());
        assert!(form_ref("serie.P12").is_err());
    }

    #[test]
    fn rows_read_numbers_and_names_and_find_repeats() {
        let (p, bad, rep) = read_row("0 11 3 4 8 7 9 5 6 1 2 10");
        assert_eq!(p[..3], [60, 71, 63]);
        assert!(bad.is_empty() && rep.is_none());
        let (p, _, _) = read_row("dó4 si3 mib");
        assert_eq!(p, [60, 59, 63]);
        let (_, bad, _) = read_row("0 12 x");
        assert_eq!(bad.len(), 2);
        let (_, _, rep) = read_row("0 1 2 3 4 5 6 7 8 9 10 0");
        assert_eq!(rep, Some((23, 1)));
    }
}
