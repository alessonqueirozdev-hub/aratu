//! "Did you mean": the known word nearest to a misspelt one, and the words
//! musicians try first in Portuguese and Spanish for the frame of a piece.

/// The edit distance between two words: letters added, removed or changed.
pub fn distance(a: &str, b: &str) -> usize {
    let b: Vec<char> = b.chars().collect();
    let mut row: Vec<usize> = (0..=b.len()).collect();
    for (i, ca) in a.chars().enumerate() {
        let mut prev = row[0];
        row[0] = i + 1;
        for (j, cb) in b.iter().enumerate() {
            let cur = row[j + 1];
            row[j + 1] = (prev + usize::from(ca != *cb)).min(row[j] + 1).min(cur + 1);
            prev = cur;
        }
    }
    row[b.len()]
}

/// The candidate nearest to `word`, if it is near enough to be a slip of
/// the keyboard: one letter off in a short word, two in a longer one.
pub fn closest<'a>(word: &str, candidates: impl IntoIterator<Item = &'a str>) -> Option<&'a str> {
    let word = word.to_lowercase();
    let most = if word.chars().count() <= 4 { 1 } else { 2 };
    candidates
        .into_iter()
        .map(|c| (distance(&word, c), c))
        .filter(|(d, c)| *d > 0 && *d <= most && *d < c.chars().count())
        .min_by_key(|(d, _)| *d)
        .map(|(_, c)| c)
}

/// The words of the frame of a piece, written before a colon.
pub const HEADERS: &[&str] = &["meter", "tempo", "key", "pickup", "seed", "form"];

/// What musicians write for each of them in Portuguese and Spanish.
const HEADER_WORDS: &[(&str, &str)] = &[
    ("andamento", "tempo"),
    ("bpm", "tempo"),
    ("velocidade", "tempo"),
    ("velocidad", "tempo"),
    ("compasso", "meter"),
    ("compás", "meter"),
    ("compas", "meter"),
    ("fórmula", "meter"),
    ("formula", "meter"),
    ("métrica", "meter"),
    ("metrica", "meter"),
    ("time", "meter"),
    ("tom", "key"),
    ("tonalidade", "key"),
    ("tonalidad", "key"),
    ("tono", "key"),
    ("escala", "key"),
    ("anacruse", "pickup"),
    ("anacrusa", "pickup"),
    ("semente", "seed"),
    ("semilla", "seed"),
    ("forma", "form"),
    ("estrutura", "form"),
    ("estructura", "form"),
];

/// The header a Portuguese or Spanish word stands for: `andamento` is
/// `tempo`.
pub fn header_for(word: &str) -> Option<&'static str> {
    let w = word.to_lowercase();
    HEADER_WORDS
        .iter()
        .find(|(said, _)| *said == w)
        .map(|(_, header)| *header)
}

/// Words that start a statement.
pub const KEYWORDS: &[&str] = &[
    "play", "add", "tempo", "rit", "rall", "accel", "meter", "key", "swing", "rubato", "humanize",
    "every", "now", "at", "seed", "form", "pattern", "section", "motif", "row", "list", "pickup",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slips_are_found_and_strangers_are_not() {
        assert_eq!(closest("tempp", HEADERS.iter().copied()), Some("tempo"));
        assert_eq!(closest("ply", KEYWORDS.iter().copied()), Some("play"));
        assert_eq!(closest("patern", KEYWORDS.iter().copied()), Some("pattern"));
        assert_eq!(closest("melodia", KEYWORDS.iter().copied()), None);
        assert_eq!(
            closest("play", KEYWORDS.iter().copied()),
            None,
            "a right word needs no help"
        );
        assert_eq!(header_for("Andamento"), Some("tempo"));
        assert_eq!(header_for("tom"), Some("key"));
    }
}
