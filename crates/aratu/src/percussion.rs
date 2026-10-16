//! Brazilian percussion, and the flamenco cajón: each instrument is a line
//! named after it, with strokes of its own, written with the rhythm rules
//! like a drum line: `pandeiro = pd td pd td`.

use crate::ast::Perc;
use crate::diag::Lang;
use crate::schedule::Drum;

impl Perc {
    /// The instrument a line's name calls, in any of the three languages.
    pub fn named(line: &str) -> Option<Perc> {
        use Perc::*;
        Some(match crate::words::fold(line).as_str() {
            "pandeiro" | "pandero" => Pandeiro,
            "zabumba" => Zabumba,
            "triangulo" | "triangle" | "ferrinho" => Triangle,
            "agogo" => Agogo,
            "surdo" => Surdo,
            "tamborim" => Tamborim,
            "alfaia" => Alfaia,
            "gongue" => Gongue,
            "ganza" | "agbe" | "caxixi" | "shaker" | "chocalho" => Shaker,
            "cuica" => Cuica,
            "cajon" => Cajon,
            _ => return None,
        })
    }

    /// The instrument's strokes: the letter and what it plays.
    pub fn strokes(self) -> &'static [(char, Drum)] {
        use Drum::*;
        match self {
            Perc::Pandeiro => &[
                ('p', PandeiroThumb),
                ('d', PandeiroFingers),
                ('t', PandeiroSlap),
                ('m', PandeiroMuted),
            ],
            Perc::Zabumba => &[('o', ZabumbaOpen), ('m', ZabumbaMuted), ('b', ZabumbaStick)],
            Perc::Triangle => &[('o', TriangleOpen), ('f', TriangleClosed)],
            Perc::Agogo => &[('a', AgogoHigh), ('g', AgogoLow)],
            Perc::Surdo => &[('o', SurdoOpen), ('m', SurdoMuted)],
            Perc::Tamborim => &[('o', TamborimOpen), ('m', TamborimMuted)],
            Perc::Alfaia => &[('o', AlfaiaOpen), ('m', AlfaiaMuted)],
            Perc::Gongue => &[('o', GongueOpen), ('m', GongueMuted)],
            Perc::Shaker => &[('s', Shaker)],
            Perc::Cuica => &[('a', CuicaHigh), ('g', CuicaLow)],
            Perc::Cajon => &[('b', CajonBass), ('t', CajonSlap)],
        }
    }

    /// What a letter plays on this instrument.
    pub fn stroke(self, c: char) -> Option<Drum> {
        self.strokes()
            .iter()
            .find(|(s, _)| *s == c)
            .map(|(_, d)| *d)
    }

    /// The instrument's name and its strokes in words, for messages.
    pub fn help(self, lang: Lang) -> &'static str {
        use Lang::*;
        use Perc::*;
        match (self, lang) {
            (Pandeiro, Pt) => "do pandeiro: p polegar, d dedos, t tapa, m abafado",
            (Pandeiro, Es) => "del pandeiro: p pulgar, d dedos, t palmada, m apagado",
            (Pandeiro, En) => "on the pandeiro: p thumb, d fingers, t slap, m muted",
            (Zabumba, Pt) => "da zabumba: o aberto, m abafado, b bacalhau",
            (Zabumba, Es) => "de la zabumba: o abierto, m apagado, b bacalhau (varilla)",
            (Zabumba, En) => "on the zabumba: o open, m muted, b bacalhau (the stick)",
            (Triangle, Pt) => "do triângulo: o aberto, f fechado",
            (Triangle, Es) => "del triángulo: o abierto, f cerrado",
            (Triangle, En) => "on the triangle: o open, f closed",
            (Agogo, Pt) => "do agogô: a agudo, g grave",
            (Agogo, Es) => "del agogó: a agudo, g grave",
            (Agogo, En) => "on the agogô: a high, g low",
            (Surdo, Pt) => "do surdo: o aberto, m abafado",
            (Surdo, Es) => "del surdo: o abierto, m apagado",
            (Surdo, En) => "on the surdo: o open, m muted",
            (Tamborim, Pt) => "do tamborim: o aberto, m abafado",
            (Tamborim, Es) => "del tamborim: o abierto, m apagado",
            (Tamborim, En) => "on the tamborim: o open, m muted",
            (Alfaia, Pt) => "da alfaia: o aberto, m abafado",
            (Alfaia, Es) => "de la alfaia: o abierto, m apagado",
            (Alfaia, En) => "on the alfaia: o open, m muted",
            (Gongue, Pt) => "do gonguê: o aberto, m abafado",
            (Gongue, Es) => "del gonguê: o abierto, m apagado",
            (Gongue, En) => "on the gonguê: o open, m muted",
            (Shaker, Pt) => "do ganzá: s",
            (Shaker, Es) => "del ganzá: s",
            (Shaker, En) => "on the shaker: s",
            (Cuica, Pt) => "da cuíca: a agudo, g grave",
            (Cuica, Es) => "de la cuíca: a agudo, g grave",
            (Cuica, En) => "on the cuíca: a high, g low",
            (Cajon, Pt) => "do cajón: b grave, t tapa",
            (Cajon, Es) => "del cajón: b grave, t golpe agudo",
            (Cajon, En) => "on the cajón: b bass, t slap",
        }
    }
}
