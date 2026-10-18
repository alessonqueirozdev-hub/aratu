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
    /// Bowed strings.
    Strings,
    /// Sung voices.
    Voices,
}

impl Family {
    pub const ALL: [Family; 9] = [
        Family::Drums,
        Family::Keys,
        Family::Mallets,
        Family::Plucked,
        Family::Strings,
        Family::Bass,
        Family::Winds,
        Family::Voices,
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
            Family::Strings => "strings",
            Family::Voices => "voices",
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
            (Family::Strings, Lang::Pt) => "Cordas friccionadas",
            (Family::Strings, Lang::Es) => "Cuerdas frotadas",
            (Family::Strings, Lang::En) => "Bowed strings",
            (Family::Voices, Lang::Pt) => "Vozes",
            (Family::Voices, Lang::Es) => "Voces",
            (Family::Voices, Lang::En) => "Voices",
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
const LEAD: &str = "| c5 _ g4 c5 |";
const PAD: &str = "| [c4 e4 g4] _ _ _ |";
const STAB: &str = "| [c4 e4 g4] - [c4 e4 g4] - |";
const HIT: &str = "| c4 _ _ _ |";
const FOUR: &str = "hh = -c -c -c -c\nbd = b b b b\nclap = - p - p";
const TRAP: &str = "hh = cccc cccc cccc cccc\nbd = b - - b\nclap = - p - p";
/// For the instruments that start above middle C (the xylophone from G4, the
/// alto recorder from F4): they play at the pitch written.
const HIGH: &str = "| c5 e5 g5 c6 |";

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
        HIGH,
    ),
    v(
        "mallets.glockenspiel",
        Mallets,
        Sampled,
        ["Glockenspiel", "Glockenspiel", "Glockenspiel"],
        "| c6 e6 g6 c7 |",
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
        HIGH,
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
    // Manguebyte's wave 5e: drum kits, the modelled instruments and the
    // poly synth's presets.
    v(
        "drums.909",
        Drums,
        Synthesized,
        ["Bateria 909", "Batería 909", "909 kit"],
        BEAT,
    ),
    v(
        "drums.606",
        Drums,
        Synthesized,
        ["Bateria 606", "Batería 606", "606 kit"],
        BEAT,
    ),
    v(
        "drums.cr78",
        Drums,
        Synthesized,
        ["Bateria CR-78", "Batería CR-78", "CR-78 kit"],
        BEAT,
    ),
    v(
        "drums.trap",
        Drums,
        Synthesized,
        ["Bateria de trap", "Batería de trap", "Trap kit"],
        TRAP,
    ),
    v(
        "drums.techno",
        Drums,
        Synthesized,
        ["Bateria de techno", "Batería de techno", "Techno kit"],
        FOUR,
    ),
    v(
        "drums.house",
        Drums,
        Synthesized,
        ["Bateria de house", "Batería de house", "House kit"],
        FOUR,
    ),
    v(
        "drums.dnb",
        Drums,
        Synthesized,
        [
            "Bateria de drum and bass",
            "Batería de drum and bass",
            "Drum and bass kit",
        ],
        BEAT,
    ),
    v(
        "drums.lofi",
        Drums,
        Synthesized,
        ["Bateria lo-fi", "Batería lo-fi", "Lo-fi kit"],
        BEAT,
    ),
    v(
        "drums.modeled",
        Drums,
        Synthesized,
        [
            "Bateria acústica modelada",
            "Batería acústica modelada",
            "Modelled acoustic kit",
        ],
        BEAT,
    ),
    v(
        "mallets.celesta",
        Mallets,
        Synthesized,
        ["Celesta", "Celesta", "Celesta"],
        "| c5 e5 g5 c6 |",
    ),
    v(
        "mallets.steelpan",
        Mallets,
        Synthesized,
        ["Steel pan", "Steel pan", "Steel pan"],
        RUN,
    ),
    v(
        "strings.violin",
        Strings,
        Synthesized,
        ["Violino", "Violín", "Violin"],
        "| g4 _ d5 _ |",
    ),
    v(
        "strings.viola",
        Strings,
        Synthesized,
        ["Viola", "Viola", "Viola"],
        "| c4 _ g4 _ |",
    ),
    v(
        "strings.cello",
        Strings,
        Synthesized,
        ["Violoncelo", "Violonchelo", "Cello"],
        "| c3 _ g3 _ |",
    ),
    v(
        "strings.bass",
        Strings,
        Synthesized,
        ["Contrabaixo", "Contrabajo", "Double bass"],
        "| c2 _ g2 _ |",
    ),
    v(
        "strings.section",
        Strings,
        Synthesized,
        ["Naipe de cordas", "Sección de cuerdas", "String section"],
        "| [c4 e4 g4] _ _ _ |",
    ),
    v(
        "strings.violin_pizz",
        Strings,
        Synthesized,
        [
            "Violino em pizzicato",
            "Violín en pizzicato",
            "Violin pizzicato",
        ],
        "| c5 e5 g5 c6 |",
    ),
    v(
        "strings.cello_pizz",
        Strings,
        Synthesized,
        [
            "Violoncelo em pizzicato",
            "Violonchelo en pizzicato",
            "Cello pizzicato",
        ],
        "| c3 e3 g3 c4 |",
    ),
    v(
        "strings.bass_pizz",
        Strings,
        Synthesized,
        [
            "Contrabaixo em pizzicato",
            "Contrabajo en pizzicato",
            "Double bass pizzicato",
        ],
        LOW,
    ),
    v(
        "flute",
        Winds,
        Synthesized,
        ["Flauta transversal", "Flauta traversa", "Flute"],
        "| c5 e5 g5 c6 |",
    ),
    v(
        "clarinet",
        Winds,
        Synthesized,
        ["Clarinete", "Clarinete", "Clarinet"],
        RUN,
    ),
    v(
        "sax",
        Winds,
        Synthesized,
        [
            "Saxofone modelado",
            "Saxofón modelado",
            "Modelled saxophone",
        ],
        LONG,
    ),
    v(
        "trumpet",
        Winds,
        Synthesized,
        ["Trompete", "Trompeta", "Trumpet"],
        "| c5 _ g4 _ |",
    ),
    v(
        "trombone",
        Winds,
        Synthesized,
        ["Trombone", "Trombón", "Trombone"],
        "| c3 _ g3 _ |",
    ),
    v(
        "choir.aah",
        Voices,
        Synthesized,
        ["Coro em \"a\"", "Coro en \"a\"", "\"Aah\" choir"],
        "| [c4 e4 g4] _ _ _ |",
    ),
    v(
        "choir.ooh",
        Voices,
        Synthesized,
        ["Coro em \"u\"", "Coro en \"u\"", "\"Ooh\" choir"],
        "| [c4 e4 g4] _ _ _ |",
    ),
    v(
        "voice.solo",
        Voices,
        Synthesized,
        ["Voz solo", "Voz solista", "Solo voice"],
        "| c4 _ e4 g4 |",
    ),
    v(
        "poly.moog_bass",
        Bass,
        Synthesized,
        ["Baixo Moog", "Bajo Moog", "Moog bass"],
        LOW,
    ),
    v(
        "poly.reese",
        Bass,
        Synthesized,
        ["Baixo Reese", "Bajo Reese", "Reese bass"],
        LOW,
    ),
    v(
        "poly.wobble_bass",
        Bass,
        Synthesized,
        ["Baixo wobble", "Bajo wobble", "Wobble bass"],
        LOW,
    ),
    v(
        "poly.deep_sub",
        Bass,
        Synthesized,
        ["Subgrave", "Subgrave", "Deep sub"],
        LOW,
    ),
    v(
        "poly.fm_growl",
        Bass,
        Synthesized,
        ["Rosnado FM", "Gruñido FM", "FM growl"],
        LOW,
    ),
    v(
        "poly.house_organ_bass",
        Bass,
        Synthesized,
        [
            "Baixo de órgão house",
            "Bajo de órgano house",
            "House organ bass",
        ],
        LOW,
    ),
    v(
        "poly.acid_line",
        Bass,
        Synthesized,
        ["Linha ácida", "Línea ácida", "Acid line"],
        LOW,
    ),
    v(
        "poly.garage_square",
        Bass,
        Synthesized,
        [
            "Baixo quadrado garage",
            "Bajo cuadrado garage",
            "Garage square bass",
        ],
        LOW,
    ),
    v(
        "poly.wavetable_growl",
        Bass,
        Synthesized,
        ["Rosnado wavetable", "Gruñido wavetable", "Wavetable growl"],
        LOW,
    ),
    v(
        "poly.synthpop_bass",
        Bass,
        Synthesized,
        ["Baixo synthpop", "Bajo synthpop", "Synthpop bass"],
        LOW,
    ),
    v(
        "poly.funk_bass",
        Bass,
        Synthesized,
        ["Baixo funk", "Bajo funk", "Funk bass"],
        LOW,
    ),
    v(
        "poly.808_glide",
        Bass,
        Synthesized,
        ["808 deslizante", "808 deslizante", "808 glide"],
        LOW,
    ),
    v(
        "poly.rolling_bass",
        Bass,
        Synthesized,
        ["Baixo rolante", "Bajo rodante", "Rolling bass"],
        LOW,
    ),
    v(
        "poly.rubber_bass",
        Bass,
        Synthesized,
        ["Baixo de borracha", "Bajo de goma", "Rubber bass"],
        LOW,
    ),
    v(
        "poly.talking_bass",
        Bass,
        Synthesized,
        ["Baixo falante", "Bajo parlante", "Talking bass"],
        LOW,
    ),
    v(
        "poly.trance_supersaw",
        Synth,
        Synthesized,
        [
            "Supersaw de trance",
            "Supersaw de trance",
            "Trance supersaw",
        ],
        LEAD,
    ),
    v(
        "poly.square_lead",
        Synth,
        Synthesized,
        ["Solo quadrado", "Solista cuadrado", "Square lead"],
        LEAD,
    ),
    v(
        "poly.sync_lead",
        Synth,
        Synthesized,
        ["Solo sync", "Solista sync", "Sync lead"],
        LEAD,
    ),
    v(
        "poly.chiptune",
        Synth,
        Synthesized,
        ["Chiptune", "Chiptune", "Chiptune"],
        LEAD,
    ),
    v(
        "poly.hoover",
        Synth,
        Synthesized,
        ["Hoover", "Hoover", "Hoover"],
        LEAD,
    ),
    v(
        "poly.mono_saw_lead",
        Synth,
        Synthesized,
        [
            "Solo dente de serra",
            "Solista diente de sierra",
            "Mono saw lead",
        ],
        LEAD,
    ),
    v(
        "poly.whistle_lead",
        Synth,
        Synthesized,
        ["Solo assobio", "Solista silbido", "Whistle lead"],
        LEAD,
    ),
    v(
        "poly.digital_lead",
        Synth,
        Synthesized,
        ["Solo digital", "Solista digital", "Digital lead"],
        LEAD,
    ),
    v(
        "poly.talking_lead",
        Synth,
        Synthesized,
        ["Solo falante", "Solista parlante", "Talking lead"],
        LEAD,
    ),
    v(
        "poly.screamer",
        Synth,
        Synthesized,
        ["Grito", "Grito", "Screamer"],
        LEAD,
    ),
    v(
        "poly.future_lead",
        Synth,
        Synthesized,
        ["Solo future", "Solista future", "Future lead"],
        LEAD,
    ),
    v(
        "poly.warm_analog_pad",
        Synth,
        Synthesized,
        [
            "Pad analógico quente",
            "Pad analógico cálido",
            "Warm analog pad",
        ],
        PAD,
    ),
    v(
        "poly.string_machine",
        Synth,
        Synthesized,
        ["String machine", "String machine", "String machine"],
        PAD,
    ),
    v(
        "poly.glass_pad",
        Synth,
        Synthesized,
        ["Pad de vidro", "Pad de cristal", "Glass pad"],
        PAD,
    ),
    v(
        "poly.dark_drone",
        Synth,
        Synthesized,
        ["Drone escuro", "Drone oscuro", "Dark drone"],
        PAD,
    ),
    v(
        "poly.choir_pad",
        Synth,
        Synthesized,
        ["Pad de coro", "Pad de coro", "Choir pad"],
        PAD,
    ),
    v(
        "poly.formant_pad",
        Synth,
        Synthesized,
        ["Pad de formantes", "Pad de formantes", "Formant pad"],
        PAD,
    ),
    v(
        "poly.pwm_pad",
        Synth,
        Synthesized,
        ["Pad PWM", "Pad PWM", "PWM pad"],
        PAD,
    ),
    v(
        "poly.shimmer_pad",
        Synth,
        Synthesized,
        ["Pad cintilante", "Pad brillante", "Shimmer pad"],
        PAD,
    ),
    v(
        "poly.evolving_spectra",
        Synth,
        Synthesized,
        [
            "Espectro em movimento",
            "Espectro en movimiento",
            "Evolving spectra",
        ],
        PAD,
    ),
    v(
        "poly.soft_sine_pad",
        Synth,
        Synthesized,
        ["Pad senoidal suave", "Pad senoidal suave", "Soft sine pad"],
        PAD,
    ),
    v(
        "poly.comb_wash",
        Synth,
        Synthesized,
        ["Lavagem em pente", "Lavado en peine", "Comb wash"],
        PAD,
    ),
    v(
        "poly.sweep_pad",
        Synth,
        Synthesized,
        ["Pad varredura", "Pad de barrido", "Sweep pad"],
        PAD,
    ),
    v(
        "poly.house_pluck",
        Synth,
        Synthesized,
        ["Pluck house", "Pluck house", "House pluck"],
        RUN,
    ),
    v(
        "poly.bell_pluck",
        Synth,
        Synthesized,
        ["Pluck de sino", "Pluck de campana", "Bell pluck"],
        RUN,
    ),
    v(
        "poly.marimba",
        Synth,
        Synthesized,
        ["Marimba sintética", "Marimba sintética", "Synth marimba"],
        RUN,
    ),
    v(
        "poly.plucked_string",
        Synth,
        Synthesized,
        ["Corda pinçada", "Cuerda pulsada", "Plucked string"],
        RUN,
    ),
    v(
        "poly.future_pluck",
        Synth,
        Synthesized,
        ["Pluck future", "Pluck future", "Future pluck"],
        RUN,
    ),
    v(
        "poly.glass_pluck",
        Synth,
        Synthesized,
        ["Pluck de vidro", "Pluck de cristal", "Glass pluck"],
        RUN,
    ),
    v(
        "poly.trance_pluck",
        Synth,
        Synthesized,
        ["Pluck de trance", "Pluck de trance", "Trance pluck"],
        RUN,
    ),
    v(
        "poly.pizzicato",
        Synth,
        Synthesized,
        [
            "Pizzicato sintético",
            "Pizzicato sintético",
            "Synth pizzicato",
        ],
        RUN,
    ),
    v(
        "poly.kalimba",
        Synth,
        Synthesized,
        ["Kalimba sintética", "Kalimba sintética", "Synth kalimba"],
        RUN,
    ),
    v(
        "poly.polysynth_piano",
        Keys,
        Synthesized,
        [
            "Piano polissintetizado",
            "Piano polisintetizado",
            "Polysynth piano",
        ],
        CHORD,
    ),
    v(
        "poly.clavinet",
        Keys,
        Synthesized,
        ["Clavinete", "Clavinet", "Clavinet"],
        CHORD,
    ),
    v(
        "poly.fm_electric_piano",
        Keys,
        Synthesized,
        [
            "Piano elétrico FM",
            "Piano eléctrico FM",
            "FM electric piano",
        ],
        CHORD,
    ),
    v(
        "poly.drawbar_organ",
        Keys,
        Synthesized,
        ["Órgão de drawbar", "Órgano de drawbar", "Drawbar organ"],
        CHORD,
    ),
    v(
        "poly.juno_keys",
        Keys,
        Synthesized,
        ["Teclado Juno", "Teclado Juno", "Juno keys"],
        CHORD,
    ),
    v(
        "poly.wurly",
        Keys,
        Synthesized,
        ["Wurlitzer", "Wurlitzer", "Wurly"],
        CHORD,
    ),
    v(
        "poly.vibraphone",
        Keys,
        Synthesized,
        [
            "Vibrafone sintético",
            "Vibráfono sintético",
            "Synth vibraphone",
        ],
        CHORD,
    ),
    v(
        "poly.synth_brass",
        Synth,
        Synthesized,
        ["Metais sintéticos", "Metales sintéticos", "Synth brass"],
        STAB,
    ),
    v(
        "poly.brass_stab",
        Synth,
        Synthesized,
        ["Ataque de metais", "Golpe de metales", "Brass stab"],
        STAB,
    ),
    v(
        "poly.jump_brass",
        Synth,
        Synthesized,
        ["Metais de jump", "Metales de jump", "Jump brass"],
        STAB,
    ),
    v(
        "poly.rave_chord_stab",
        Synth,
        Synthesized,
        ["Acorde de rave", "Acorde de rave", "Rave chord stab"],
        STAB,
    ),
    v(
        "poly.soft_horns",
        Synth,
        Synthesized,
        ["Trompas suaves", "Trompas suaves", "Soft horns"],
        PAD,
    ),
    v(
        "poly.fanfare",
        Synth,
        Synthesized,
        ["Fanfarra", "Fanfarria", "Fanfare"],
        STAB,
    ),
    v(
        "poly.riser",
        Synth,
        Synthesized,
        ["Subida", "Subida", "Riser"],
        HIT,
    ),
    v(
        "poly.downlifter",
        Synth,
        Synthesized,
        ["Descida", "Bajada", "Downlifter"],
        HIT,
    ),
    v(
        "poly.noise_sweep",
        Synth,
        Synthesized,
        ["Varredura de ruído", "Barrido de ruido", "Noise sweep"],
        HIT,
    ),
    v(
        "poly.laser_zap",
        Synth,
        Synthesized,
        ["Laser", "Láser", "Laser zap"],
        HIT,
    ),
    v(
        "poly.random_bleeps",
        Synth,
        Synthesized,
        ["Bipes aleatórios", "Pitidos aleatorios", "Random bleeps"],
        HIT,
    ),
    v(
        "poly.wind",
        Synth,
        Synthesized,
        ["Vento", "Viento", "Wind"],
        HIT,
    ),
    v(
        "poly.impact_boom",
        Synth,
        Synthesized,
        ["Impacto", "Impacto", "Impact boom"],
        HIT,
    ),
    v(
        "poly.siren",
        Synth,
        Synthesized,
        ["Sirene", "Sirena", "Siren"],
        HIT,
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
