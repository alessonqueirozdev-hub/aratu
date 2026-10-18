// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
//! The voices a part can play, by code: `bass: bass.slap -> reverb`.
//!
//! One list serves the checker (an unknown voice is reported, with the
//! closest code) and the editor's Sounds panel. Synthesized voices are made
//! in the browser; sampled ones play recordings from the Versilian Community
//! Sample Library (CC0), or recorded for Aratu (`drums.alesson`, CC0),
//! fetched the first time a piece uses them.

use crate::diag::Lang;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Family {
    Drums,
    Keys,
    Mallets,
    Plucked,
    Bass,
    Winds,
    Synth,
}

impl Family {
    pub const ALL: [Family; 7] = [
        Family::Drums,
        Family::Keys,
        Family::Mallets,
        Family::Plucked,
        Family::Bass,
        Family::Winds,
        Family::Synth,
    ];

    pub fn code(self) -> &'static str {
        match self {
            Family::Drums => "drums",
            Family::Keys => "keys",
            Family::Mallets => "mallets",
            Family::Plucked => "plucked",
            Family::Bass => "bass",
            Family::Winds => "winds",
            Family::Synth => "synth",
        }
    }

    pub fn name(self, lang: Lang) -> &'static str {
        match (self, lang) {
            (Family::Drums, Lang::Pt) => "Baterias",
            (Family::Drums, Lang::Es) => "Baterías",
            (Family::Drums, Lang::En) => "Drum kits",
            (Family::Keys, Lang::Pt) => "Teclados",
            (Family::Keys, Lang::Es) => "Teclados",
            (Family::Keys, Lang::En) => "Keyboards",
            (Family::Mallets, Lang::Pt) => "Teclados de percussão e sinos",
            (Family::Mallets, Lang::Es) => "Láminas y campanas",
            (Family::Mallets, Lang::En) => "Mallets and bells",
            (Family::Plucked, Lang::Pt) => "Cordas dedilhadas",
            (Family::Plucked, Lang::Es) => "Cuerdas pulsadas",
            (Family::Plucked, Lang::En) => "Plucked strings",
            (Family::Bass, Lang::Pt) => "Baixos",
            (Family::Bass, Lang::Es) => "Bajos",
            (Family::Bass, Lang::En) => "Basses",
            (Family::Winds, Lang::Pt) => "Sopros",
            (Family::Winds, Lang::Es) => "Vientos",
            (Family::Winds, Lang::En) => "Winds",
            (Family::Synth, Lang::Pt) => "Sintetizadores",
            (Family::Synth, Lang::Es) => "Sintetizadores",
            (Family::Synth, Lang::En) => "Synthesizers",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Source {
    /// Made in the browser: nothing to fetch.
    Synthesized,
    /// Recordings, fetched the first time a piece uses the voice.
    Sampled,
}

#[derive(Clone, Copy, Debug)]
pub struct Voice {
    pub code: &'static str,
    pub family: Family,
    pub source: Source,
    /// The name in Portuguese, Spanish and English.
    names: [&'static str; 3],
    /// A short example to hear it, in Aratu: one bar.
    pub demo: &'static str,
}

impl Voice {
    pub fn name(&self, lang: Lang) -> &'static str {
        match lang {
            Lang::Pt => self.names[0],
            Lang::Es => self.names[1],
            Lang::En => self.names[2],
        }
    }
}

const fn v(
    code: &'static str,
    family: Family,
    source: Source,
    names: [&'static str; 3],
    demo: &'static str,
) -> Voice {
    Voice {
        code,
        family,
        source,
        names,
        demo,
    }
}

use Family::*;
use Source::*;

const BEAT: &str = "hh = cc cc co cc\nbd = b - bb -\nsd = - s - S";
const CHORD: &str = "| c4 e4 g4 [c5 g4] |";
const RUN: &str = "| c4 e4 g4 c5 |";
const LOW: &str = "| c2 _ g2 c3 |";
const LONG: &str = "| c4 _ g4 _ |";

pub const VOICES: &[Voice] = &[
    v(
        "drums.kit",
        Drums,
        Synthesized,
        ["Bateria analógica", "Batería analógica", "Analogue kit"],
        BEAT,
    ),
    v(
        "drums.808",
        Drums,
        Synthesized,
        ["Bateria 808", "Batería 808", "808 kit"],
        BEAT,
    ),
    v(
        "drums.acoustic",
        Drums,
        Sampled,
        ["Bateria acústica", "Batería acústica", "Acoustic kit"],
        BEAT,
    ),
    v(
        "drums.alesson",
        Drums,
        Sampled,
        [
            "Bateria gravada do Alesson",
            "Batería grabada de Alesson",
            "Alesson's recorded kit",
        ],
        BEAT,
    ),
    v(
        "piano.grand",
        Keys,
        Sampled,
        ["Piano de cauda", "Piano de cola", "Grand piano"],
        CHORD,
    ),
    v(
        "piano.upright",
        Keys,
        Sampled,
        ["Piano de armário", "Piano vertical", "Upright piano"],
        CHORD,
    ),
    v(
        "keys.epiano",
        Keys,
        Synthesized,
        ["Piano elétrico", "Piano eléctrico", "Electric piano"],
        CHORD,
    ),
    v(
        "keys.organ",
        Keys,
        Synthesized,
        ["Órgão elétrico", "Órgano eléctrico", "Drawbar organ"],
        LONG,
    ),
    v(
        "keys.harpsichord",
        Keys,
        Sampled,
        ["Cravo", "Clave", "Harpsichord"],
        RUN,
    ),
    v(
        "organ.pipe",
        Keys,
        Sampled,
        ["Órgão de tubos", "Órgano de tubos", "Pipe organ"],
        LONG,
    ),
    v(
        "mallets.marimba",
        Mallets,
        Sampled,
        ["Marimba", "Marimba", "Marimba"],
        RUN,
    ),
    v(
        "mallets.vibraphone",
        Mallets,
        Sampled,
        ["Vibrafone", "Vibráfono", "Vibraphone"],
        RUN,
    ),
    v(
        "mallets.xylophone",
        Mallets,
        Sampled,
        ["Xilofone", "Xilófono", "Xylophone"],
        RUN,
    ),
    v(
        "mallets.glockenspiel",
        Mallets,
        Sampled,
        ["Glockenspiel", "Glockenspiel", "Glockenspiel"],
        "| c5 e5 g5 c6 |",
    ),
    v(
        "mallets.kalimba",
        Mallets,
        Sampled,
        ["Kalimba", "Kalimba", "Kalimba"],
        RUN,
    ),
    v(
        "bells.tubular",
        Mallets,
        Sampled,
        ["Sinos tubulares", "Campanas tubulares", "Tubular bells"],
        LONG,
    ),
    v(
        "keys.bell",
        Mallets,
        Synthesized,
        ["Sino FM", "Campana FM", "FM bell"],
        RUN,
    ),
    v(
        "guitar.nylon",
        Plucked,
        Synthesized,
        ["Violão de nylon", "Guitarra de nylon", "Nylon guitar"],
        CHORD,
    ),
    v(
        "guitar.steel",
        Plucked,
        Synthesized,
        ["Violão de aço", "Guitarra de acero", "Steel guitar"],
        CHORD,
    ),
    v("harp", Plucked, Sampled, ["Harpa", "Arpa", "Harp"], RUN),
    v(
        "bass.synth",
        Bass,
        Synthesized,
        ["Baixo sintetizado", "Bajo sintetizado", "Synth bass"],
        LOW,
    ),
    v(
        "bass.finger",
        Bass,
        Synthesized,
        ["Baixo de dedo", "Bajo con dedos", "Finger bass"],
        LOW,
    ),
    v(
        "bass.slap",
        Bass,
        Synthesized,
        ["Baixo slap", "Bajo slap", "Slap bass"],
        "| e2 E3 e2 E3 |",
    ),
    v(
        "bass.808",
        Bass,
        Synthesized,
        ["Baixo 808", "Bajo 808", "808 bass"],
        LOW,
    ),
    v(
        "bass.acid",
        Bass,
        Synthesized,
        ["Baixo ácido", "Bajo ácido", "Acid bass"],
        "| c2 c3 c2 eb2 |",
    ),
    v(
        "sax.tenor",
        Winds,
        Sampled,
        ["Sax tenor", "Saxo tenor", "Tenor sax"],
        LONG,
    ),
    v(
        "harmonica",
        Winds,
        Sampled,
        ["Gaita", "Armónica", "Harmonica"],
        LONG,
    ),
    v(
        "recorder.alto",
        Winds,
        Sampled,
        ["Flauta doce", "Flauta dulce", "Recorder"],
        RUN,
    ),
    v(
        "synth.saw",
        Synth,
        Synthesized,
        ["Dente de serra", "Diente de sierra", "Saw"],
        LONG,
    ),
    v(
        "synth.square",
        Synth,
        Synthesized,
        ["Onda quadrada", "Onda cuadrada", "Square"],
        LONG,
    ),
    v(
        "synth.triangle",
        Synth,
        Synthesized,
        ["Onda triangular", "Onda triangular", "Triangle"],
        LONG,
    ),
    v(
        "synth.sine",
        Synth,
        Synthesized,
        ["Senoide", "Senoidal", "Sine"],
        LONG,
    ),
    v(
        "synth.pluck",
        Synth,
        Synthesized,
        ["Corda sintética", "Cuerda sintética", "Pluck"],
        RUN,
    ),
    v(
        "synth.supersaw",
        Synth,
        Synthesized,
        ["Supersaw", "Supersaw", "Supersaw"],
        CHORD,
    ),
    v(
        "synth.pad",
        Synth,
        Synthesized,
        ["Pad", "Pad", "Pad"],
        "| [c4 e4 g4] _ _ _ |",
    ),
    v(
        "synth.strings",
        Synth,
        Synthesized,
        ["Cordas sintéticas", "Cuerdas sintéticas", "Synth strings"],
        "| [c4 e4 g4] _ _ _ |",
    ),
    v(
        "synth.brass",
        Synth,
        Synthesized,
        ["Metais sintéticos", "Metales sintéticos", "Synth brass"],
        LONG,
    ),
    v(
        "synth.lead",
        Synth,
        Synthesized,
        ["Solo", "Solista", "Lead"],
        RUN,
    ),
];

/// Short names kept from before the catalogue, and what they play.
pub const ALIASES: &[(&str, &str)] = &[
    ("synth.sawtooth", "synth.saw"),
    ("pluck", "synth.pluck"),
    ("keys", "keys.epiano"),
    ("piano", "piano.grand"),
    ("organ", "keys.organ"),
    ("guitar", "guitar.nylon"),
    ("drums", "drums.kit"),
];

/// The effects a chain may name: Manguebyte's, in the order it runs them.
/// `tube` is Manguebyte's drive set up as a valve: its smooth curve, a
/// moderate gain and the level brought back down.
pub const EFFECTS: &[&str] = &[
    "highpass",
    "lowpass",
    "eq",
    "drive",
    "tube",
    "bitcrush",
    "compressor",
    "chorus",
    "flanger",
    "phaser",
    "tremolo",
    "autopan",
    "delay",
    "reverb",
];

/// The effect a word names, in English: `valvula` and `válvula` are `tube`.
pub fn effect(name: &str) -> Option<&'static str> {
    EFFECTS.iter().copied().find(|e| crate::words::is(name, e))
}

/// A parameter's id with its effect in English: `valvula.drive` is
/// `tube.drive`.
pub fn param_id(id: String) -> String {
    match id.split_once('.') {
        Some((head, rest)) => match effect(head) {
            Some(e) if e != head => format!("{e}.{rest}"),
            _ => id,
        },
        None => id,
    }
}

/// The voice a code names, aliases included.
pub fn find(code: &str) -> Option<&'static Voice> {
    let code = ALIASES
        .iter()
        .find(|(a, _)| *a == code)
        .map_or(code, |(_, to)| to);
    VOICES.iter().find(|v| v.code == code)
}

/// The known code closest to `code`, for "did you mean".
pub fn closest(code: &str) -> Option<&'static str> {
    VOICES
        .iter()
        .map(|v| v.code)
        .chain(EFFECTS.iter().copied())
        .map(|c| (crate::suggest::distance(code, c), c))
        .filter(|(d, c)| *d <= 3.max(c.len() / 3))
        .min_by_key(|(d, _)| *d)
        .map(|(_, c)| c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codes_are_unique_and_named_in_every_language() {
        for (i, v) in VOICES.iter().enumerate() {
            assert!(
                VOICES[i + 1..].iter().all(|w| w.code != v.code),
                "{}",
                v.code
            );
            for lang in [Lang::Pt, Lang::Es, Lang::En] {
                assert!(!v.name(lang).is_empty());
            }
        }
        for (alias, to) in ALIASES {
            assert!(find(alias).is_some_and(|v| v.code == *to), "{alias}");
        }
    }

    #[test]
    fn a_misspelt_voice_finds_its_closest_code() {
        assert_eq!(closest("bass.slapp"), Some("bass.slap"));
        assert_eq!(closest("piano.grande"), Some("piano.grand"));
        assert_eq!(closest("zzzzzzzzzzzz"), None);
    }

    #[test]
    fn every_voice_can_be_written_in_a_chain() {
        for v in VOICES {
            let src = format!("som: {} -> reverb\nsom {{\n  m = | c4 _ _ _ |\n}}", v.code);
            let c = crate::compile(&src);
            assert!(c.diagnostics.is_empty(), "{}: {:?}", v.code, c.diagnostics);
            assert_eq!(c.piece.parts[0].chain[0], v.code);
        }
        let c = crate::compile("som: bass.slapp\nsom {\n  m = | c2 _ _ _ |\n}");
        assert!(matches!(
            &c.diagnostics[0].message,
            crate::diag::Message::UnknownSound { closest: Some(x), .. } if x == "bass.slap"
        ));
    }
}
