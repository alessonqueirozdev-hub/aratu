//! Music theory the language needs: note names in letters and solfège,
//! meters, keys, dynamics and chord symbols.

use crate::ratio::Ratio;

/// Pitch class of a note letter, `c` = 0.
pub fn letter_pitch_class(c: char) -> Option<i32> {
    Some(match c.to_ascii_lowercase() {
        'c' => 0,
        'd' => 2,
        'e' => 4,
        'f' => 5,
        'g' => 7,
        'a' => 9,
        'b' => 11,
        _ => return None,
    })
}

/// Solfège syllables, longest first so that `sol` is tried before `so`.
const SOLFEGE: &[(&str, i32)] = &[
    ("sol", 7),
    ("dó", 0),
    ("do", 0),
    ("ré", 2),
    ("re", 2),
    ("mi", 4),
    ("fá", 5),
    ("fa", 5),
    ("lá", 9),
    ("la", 9),
    ("si", 11),
];

/// A note name at the start of `text`: its pitch class, whether it is
/// written with an upper-case first letter, and its length in bytes.
/// Solfège is tried first, so `fá` is F, not F followed by `á`.
pub fn note_name(text: &str) -> Option<(i32, bool, usize)> {
    let first = text.chars().next()?;
    let upper = first.is_uppercase();
    let lower: String = text.chars().take(3).collect::<String>().to_lowercase();
    for (syl, pc) in SOLFEGE {
        if lower.starts_with(syl) {
            // Byte length of the same number of characters in the original.
            let n = syl.chars().count();
            let len: usize = text.chars().take(n).map(char::len_utf8).sum();
            return Some((*pc, upper, len));
        }
    }
    letter_pitch_class(first)
        .filter(|_| first.is_ascii_alphabetic())
        .map(|pc| (pc, upper, 1))
}

/// Accidentals at the start of `text`: their sum in semitones and length.
/// At most two (`##`, `bb`).
pub fn accidentals(text: &str) -> (i32, usize) {
    let mut total = 0;
    let mut len = 0;
    for c in text.chars().take(2) {
        match c {
            '#' | '♯' => total += 1,
            'b' | '♭' => total -= 1,
            _ => break,
        }
        len += c.len_utf8();
    }
    (total, len)
}

/// A meter such as 4/4 or 6/8.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Meter {
    pub num: u32,
    pub den: u32,
}

impl Meter {
    pub const COMMON: Meter = Meter { num: 4, den: 4 };

    pub fn parse(text: &str) -> Option<Meter> {
        let (n, d) = text.split_once('/')?;
        let num: u32 = n.trim().parse().ok()?;
        let den: u32 = d.trim().parse().ok()?;
        if !(1..=64).contains(&num) || ![1, 2, 4, 8, 16, 32].contains(&den) {
            return None;
        }
        Some(Meter { num, den })
    }

    /// 6/8, 9/8, 12/8 and the like are counted in dotted beats.
    pub fn is_compound(self) -> bool {
        self.den >= 8 && self.num > 3 && self.num % 3 == 0
    }

    /// The number of beats a musician counts in one bar.
    pub fn beats(self) -> Ratio {
        if self.is_compound() {
            Ratio::int(self.num as i64 / 3)
        } else {
            Ratio::int(self.num as i64)
        }
    }

    /// The length of one beat as a fraction of a whole note.
    pub fn beat_unit(self) -> Ratio {
        if self.is_compound() {
            Ratio::new(3, self.den as i64)
        } else {
            Ratio::new(1, self.den as i64)
        }
    }
}

impl std::fmt::Display for Meter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.num, self.den)
    }
}

/// A key such as A minor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Key {
    pub tonic: i32,
    pub minor: bool,
}

impl Key {
    /// `A minor`, `F# major`, `lá menor`, `Am`, `C`.
    pub fn parse(text: &str) -> Option<Key> {
        let text = text.trim();
        let (pc, _, len) = note_name(text)?;
        let rest = &text[len..];
        let (acc, alen) = accidentals(rest);
        let mode = rest[alen..].trim().to_lowercase();
        let minor = match mode.as_str() {
            "" | "major" | "maior" | "maj" => false,
            "minor" | "menor" | "min" | "m" => true,
            _ => return None,
        };
        Some(Key {
            tonic: (pc + acc).rem_euclid(12),
            minor,
        })
    }
}

/// A dynamic mark.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dyn {
    Ppp,
    Pp,
    P,
    Mp,
    Mf,
    F,
    Ff,
    Fff,
    /// Sforzando: this note very strong, then back.
    Sfz,
    /// Forte-piano: this note loud, then soft.
    Fp,
}

impl Dyn {
    pub fn parse(word: &str) -> Option<Dyn> {
        Some(match word {
            "ppp" => Dyn::Ppp,
            "pp" => Dyn::Pp,
            "p" => Dyn::P,
            "mp" => Dyn::Mp,
            "mf" => Dyn::Mf,
            "f" => Dyn::F,
            "ff" => Dyn::Ff,
            "fff" => Dyn::Fff,
            "sfz" | "sf" | "sffz" => Dyn::Sfz,
            "fp" => Dyn::Fp,
            _ => return None,
        })
    }

    /// The fixed velocity of the mark, as notation programs use.
    pub fn velocity(self) -> u8 {
        match self {
            Dyn::Ppp => 16,
            Dyn::Pp => 33,
            Dyn::P => 49,
            Dyn::Mp => 64,
            Dyn::Mf => 80,
            Dyn::F => 96,
            Dyn::Ff => 112,
            Dyn::Fff => 127,
            Dyn::Sfz => 127,
            Dyn::Fp => 96,
        }
    }

    /// The level that continues after the note carrying the mark.
    pub fn level_after(self, before: u8) -> u8 {
        match self {
            Dyn::Sfz => before,
            Dyn::Fp => Dyn::P.velocity(),
            d => d.velocity(),
        }
    }

    /// The next mark up (`up`) or down the scale ppp..fff.
    pub fn step(level: u8, up: bool) -> u8 {
        const LEVELS: [u8; 8] = [16, 33, 49, 64, 80, 96, 112, 127];
        if up {
            LEVELS.iter().copied().find(|&l| l > level).unwrap_or(127)
        } else {
            LEVELS
                .iter()
                .rev()
                .copied()
                .find(|&l| l < level)
                .unwrap_or(16)
        }
    }
}

/// The velocity an accent adds: two marks up, so an accent in `mf` sounds
/// like `ff` and stands out clearly from the beats around it.
pub const ACCENT: u8 = 32;

/// A chord symbol, read as a lead sheet writes it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Chord {
    pub root: i32,
    /// Semitones above the root, the root (0) included, ascending.
    pub intervals: Vec<i32>,
    /// The bass note of a slash chord, as a pitch class.
    pub bass: Option<i32>,
}

impl Chord {
    /// `Am`, `F`, `G7`, `Cmaj7`, `Dm7b5`, `E/G#`, `C7M`, `Bº`, `G7(b9)`.
    pub fn parse(text: &str) -> Option<Chord> {
        let first = text.chars().next()?;
        if !first.is_ascii_uppercase() {
            return None;
        }
        let root = letter_pitch_class(first)?;
        let mut rest = &text[1..];
        let (acc, alen) = accidentals(rest);
        let root = (root + acc).rem_euclid(12);
        rest = &rest[alen..];

        let mut bass = None;
        if let Some(i) = rest.find('/') {
            let b = &rest[i + 1..];
            let bc = b.chars().next()?;
            let bpc = letter_pitch_class(bc).filter(|_| bc.is_ascii_uppercase())?;
            let (bacc, balen) = accidentals(&b[1..]);
            if 1 + balen != b.len() {
                return None;
            }
            bass = Some((bpc + bacc).rem_euclid(12));
            rest = &rest[..i];
        }

        let mut third = Some(4);
        let mut fifth = 7;
        let mut extra: Vec<i32> = Vec::new();
        let mut seventh: Option<i32> = None;
        let mut q = rest.replace(['(', ')', ','], "");
        // Longest spellings first.
        const TOKENS: &[&str] = &[
            "maj13", "maj11", "maj9", "maj7", "add11", "add13", "add9", "add2", "add4", "sus2",
            "sus4", "dim7", "m7b5", "min", "maj", "dim", "aug", "sus", "7M", "M7", "Δ7", "°7",
            "º7", "ø7", "b13", "#11", "b9", "#9", "b5", "#5", "13", "11", "m", "-", "Δ", "°", "º",
            "ø", "+", "M", "9", "7", "6", "5", "4", "2",
        ];
        while !q.is_empty() {
            let tok = TOKENS.iter().find(|t| q.starts_with(*t))?;
            match *tok {
                "m" | "min" | "-" => third = Some(3),
                "maj" | "M" => {}
                "maj7" | "7M" | "M7" | "Δ7" | "Δ" => seventh = Some(11),
                "maj9" => {
                    seventh = Some(11);
                    extra.push(14);
                }
                "maj11" => {
                    seventh = Some(11);
                    extra.extend([14, 17]);
                }
                "maj13" => {
                    seventh = Some(11);
                    extra.extend([14, 21]);
                }
                "dim" | "°" | "º" => {
                    third = Some(3);
                    fifth = 6;
                }
                "dim7" | "°7" | "º7" => {
                    third = Some(3);
                    fifth = 6;
                    seventh = Some(9);
                }
                "m7b5" | "ø" | "ø7" => {
                    third = Some(3);
                    fifth = 6;
                    seventh = Some(10);
                }
                "aug" | "+" | "#5" => fifth = 8,
                "b5" => fifth = 6,
                "sus" | "sus4" | "4" => third = Some(5),
                "sus2" | "2" => third = Some(2),
                "5" => third = None,
                "6" => extra.push(9),
                "7" => {
                    if seventh.is_none() {
                        seventh = Some(10)
                    }
                }
                "9" => {
                    seventh.get_or_insert(10);
                    extra.push(14);
                }
                "11" => {
                    seventh.get_or_insert(10);
                    extra.extend([14, 17]);
                }
                "13" => {
                    seventh.get_or_insert(10);
                    extra.extend([14, 21]);
                }
                "add9" | "add2" => extra.push(14),
                "add4" | "add11" => extra.push(17),
                "add13" => extra.push(21),
                "b9" => extra.push(13),
                "#9" => extra.push(15),
                "#11" => extra.push(18),
                "b13" => extra.push(20),
                _ => unreachable!(),
            }
            q = q[tok.len()..].to_string();
        }
        let mut intervals = vec![0];
        intervals.extend(third);
        intervals.push(fifth);
        intervals.extend(seventh);
        intervals.extend(extra);
        intervals.sort_unstable();
        intervals.dedup();
        Some(Chord {
            root,
            intervals,
            bass,
        })
    }

    /// Pitch classes of the chord, root first.
    pub fn pitch_classes(&self) -> Vec<i32> {
        self.intervals
            .iter()
            .map(|i| (self.root + i).rem_euclid(12))
            .collect()
    }

    /// A keyboard voicing: the bass in the second octave, the chord in close
    /// position from the third.
    pub fn block_voicing(&self) -> Vec<i32> {
        let bass = 36 + self.bass.unwrap_or(self.root);
        let mut notes = vec![bass];
        notes.extend(self.intervals.iter().map(|i| 48 + self.root + i));
        notes
    }

    /// A guitar-like voicing of up to six strings, low to high: the bass
    /// between E2 and D#3, then chord tones upwards, at least a minor third
    /// apart, up to E5.
    pub fn guitar_voicing(&self) -> Vec<i32> {
        let pcs = self.pitch_classes();
        let bass_pc = self.bass.unwrap_or(self.root);
        let bass = 40 + (bass_pc - 4).rem_euclid(12);
        let mut notes = vec![bass];
        let mut n = bass + 3;
        while notes.len() < 6 && n <= 76 {
            if pcs.contains(&n.rem_euclid(12)) {
                notes.push(n);
                n += 3;
            } else {
                n += 1;
            }
        }
        notes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solfege_and_letters() {
        assert_eq!(note_name("dó4"), Some((0, false, 3)));
        assert_eq!(note_name("Ré"), Some((2, true, 3)));
        assert_eq!(note_name("sol#3"), Some((7, false, 3)));
        assert_eq!(note_name("fá"), Some((5, false, 3)));
        assert_eq!(note_name("fa"), Some((5, false, 2)));
        assert_eq!(note_name("si"), Some((11, false, 2)));
        assert_eq!(note_name("e4"), Some((4, false, 1)));
        assert_eq!(note_name("C"), Some((0, true, 1)));
        assert_eq!(note_name("h4"), None);
        assert_eq!(accidentals("bb4"), (-2, 2));
        assert_eq!(accidentals("#4"), (1, 1));
        assert_eq!(accidentals("♭"), (-1, 3));
    }

    #[test]
    fn meters() {
        let m = Meter::parse("6/8").unwrap();
        assert!(m.is_compound());
        assert_eq!(m.beats(), Ratio::int(2));
        assert_eq!(m.beat_unit(), Ratio::new(3, 8));
        let m = Meter::parse("7/8").unwrap();
        assert_eq!(m.beats(), Ratio::int(7));
        assert_eq!(Meter::parse("3/4").unwrap().beats(), Ratio::int(3));
        assert_eq!(Meter::parse("4/5"), None);
        assert_eq!(Meter::parse("0/4"), None);
    }

    #[test]
    fn keys() {
        assert_eq!(
            Key::parse("A minor"),
            Some(Key {
                tonic: 9,
                minor: true
            })
        );
        assert_eq!(
            Key::parse("F# major"),
            Some(Key {
                tonic: 6,
                minor: false
            })
        );
        assert_eq!(
            Key::parse("lá menor"),
            Some(Key {
                tonic: 9,
                minor: true
            })
        );
        assert_eq!(
            Key::parse("Bb"),
            Some(Key {
                tonic: 10,
                minor: false
            })
        );
        assert_eq!(Key::parse("H minor"), None);
    }

    fn pcs(s: &str) -> Vec<i32> {
        Chord::parse(s).unwrap().intervals
    }

    #[test]
    fn chord_symbols() {
        assert_eq!(pcs("C"), [0, 4, 7]);
        assert_eq!(pcs("Am"), [0, 3, 7]);
        assert_eq!(pcs("G7"), [0, 4, 7, 10]);
        assert_eq!(pcs("Cmaj7"), [0, 4, 7, 11]);
        assert_eq!(pcs("C7M"), [0, 4, 7, 11]);
        assert_eq!(pcs("Dm7b5"), [0, 3, 6, 10]);
        assert_eq!(pcs("Bº"), [0, 3, 6]);
        assert_eq!(pcs("Bdim7"), [0, 3, 6, 9]);
        assert_eq!(pcs("Csus4"), [0, 5, 7]);
        assert_eq!(pcs("G7(b9)"), [0, 4, 7, 10, 13]);
        assert_eq!(pcs("Am7"), [0, 3, 7, 10]);
        assert_eq!(pcs("C6"), [0, 4, 7, 9]);
        assert_eq!(pcs("D9"), [0, 4, 7, 10, 14]);
        assert_eq!(pcs("E5"), [0, 7]);
        assert_eq!(pcs("Caug"), [0, 4, 8]);
        let c = Chord::parse("E/G#").unwrap();
        assert_eq!((c.root, c.bass), (4, Some(8)));
        assert_eq!(Chord::parse("Bb").unwrap().root, 10);
        assert_eq!(Chord::parse("F#m").unwrap().root, 6);
        assert!(Chord::parse("Hm").is_none());
        assert!(Chord::parse("Cxyz").is_none());
        assert!(Chord::parse("am").is_none());
    }

    #[test]
    fn dynamics() {
        assert_eq!(Dyn::parse("mf").unwrap().velocity(), 80);
        assert_eq!(Dyn::parse("fff").unwrap().velocity(), 127);
        assert_eq!(Dyn::Fp.level_after(80), 49);
        assert_eq!(Dyn::Sfz.level_after(64), 64);
        assert_eq!(Dyn::step(80, true), 96);
        assert_eq!(Dyn::step(80, false), 64);
        assert_eq!(Dyn::step(127, true), 127);
        assert!(Dyn::parse("mff").is_none());
    }
}
