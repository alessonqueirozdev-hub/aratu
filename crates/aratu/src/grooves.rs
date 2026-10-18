// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
//! Ready-made beats: `groove house` (`levada house`, `ritmo house`) puts the
//! lines of a beat in the piece, written in Aratu itself, so a performer
//! starts from a groove and changes it line by line. A line the piece
//! writes with the same name, in the same part, takes the groove's place.
//! The beats are in 4/4, the common shapes of each style.

/// Each groove by name, with its lines.
pub const GROOVES: &[(&str, &str)] = &[
    (
        "house",
        "drums {\n  bd = b b b b\n  oh = -o -o -o -o\n  clap = - p - p\n  hh = cccc cccc cccc cccc\n}",
    ),
    (
        "techno",
        "drums {\n  bd = b b b b\n  oh = -o -o -o -o\n  hh = cCcc cCcc cCcc cCcc\n  rim = --k- -k-- --k- ---k\n}",
    ),
    (
        "trap",
        "drums {\n  bd = b --b- -b-- ----\n  clap = - - p -\n  hh = cccc cccc cccccc cccc\n}",
    ),
    (
        "dnb",
        "drums {\n  bd = b--- ---- --b- ----\n  sd = - s - s\n  hh = cCcc cCcc cCcc cCcc\n}",
    ),
    (
        "breakbeat",
        "drums {\n  bd = b- -b -- b-\n  sd = - s - s\n  hh = cc cc cc co\n}",
    ),
    (
        "hiphop",
        "drums {\n  bd = b --b- b -\n  sd = - s - s\n  hh = cc cc cc cc\n}",
    ),
    (
        "funk",
        "drums {\n  bd = b- -b -b --\n  sd = - s(s) -(s) s\n  hh = cccc cccc cccc cccc\n}",
    ),
    (
        "rock",
        "drums {\n  bd = b - bb -\n  sd = - s - s\n  hh = cc cc cc cc\n}",
    ),
    (
        "reggaeton",
        "drums {\n  bd = b b b b\n  sd = ---s --s- ---s --s-\n  hh = cc cc cc cc\n}",
    ),
    (
        "funkcarioca",
        "drums {\n  bd = b--b --b- b--b --b-\n  clap = - p - p\n  hh = cc cc cc cc\n}",
    ),
    (
        "bossa",
        "drums {\n  bd = b -b b -b\n  rim = | k- -k - k | - k -k - |\n  hh = cc cc cc cc\n}",
    ),
    (
        "samba",
        "surdo = m O m O\ntamborim = o-oo -oo- o-oo -oo-\npandeiro = pdtd pdtd pdtd pdtd\nganzá = Ssss Ssss Ssss Ssss",
    ),
    (
        "baiao",
        "zabumba = | o - -o m | o - b - |\ntriângulo = fo fo fo fo",
    ),
    (
        "maracatu",
        "alfaia = O -o o_oo o\ncaixa = Ssss Ssss Ssss Ssss\ngonguê = o -o -m o\nagbê = Ss Ss Ss Ss",
    ),
];

/// A groove's name as it may be written: `drum-and-bass`, `hip hop`,
/// `baião`, `funk carioca`.
fn key(name: &str) -> String {
    let k: String = crate::words::fold(name)
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect();
    match k.as_str() {
        "drumandbass" | "drumnbass" => "dnb".to_string(),
        "boombap" => "hiphop".to_string(),
        "baile" | "bailefunk" | "tamborzao" => "funkcarioca".to_string(),
        "bossanova" => "bossa".to_string(),
        _ => k,
    }
}

/// The lines of a groove, by any of the ways its name is written.
pub fn find(name: &str) -> Option<&'static str> {
    let k = key(name);
    GROOVES.iter().find(|(n, _)| *n == k).map(|(_, text)| *text)
}

/// The groove closest to a name not known, for "did you mean".
pub fn closest(name: &str) -> Option<&'static str> {
    let k = key(name);
    GROOVES
        .iter()
        .map(|(n, _)| (crate::suggest::distance(&k, n), *n))
        .filter(|(d, n)| *d <= 2.max(n.len() / 3))
        .min_by_key(|(d, _)| *d)
        .map(|(_, n)| n)
}

/// Every groove's name, for messages.
pub fn names() -> String {
    GROOVES
        .iter()
        .map(|(n, _)| *n)
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_groove_is_a_clean_bar_of_four_four() {
        for (name, text) in GROOVES {
            let c = crate::compile(text);
            let problems: Vec<String> = c
                .diagnostics
                .iter()
                .map(|d| d.text(crate::diag::Lang::En))
                .collect();
            assert!(problems.is_empty(), "{name}: {problems:?}");
        }
        assert_eq!(find("Drum and Bass"), find("dnb"));
        assert!(find("baião").is_some());
        assert_eq!(closest("hous"), Some("house"));
    }
}
