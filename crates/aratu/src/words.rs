//! The keywords of the language in English, Portuguese and Spanish. The
//! English word is the one the code and the messages use; the others stand
//! for it wherever it is expected, so `no compasso 9: toque fill` and
//! `en el compás 9: toca fill` read as `at bar 9: play fill`.

/// A word folded for comparison: lower case, without accents, so
/// `padrão`, `Padrao` and `PADRÃO` are the same word.
pub fn fold(word: &str) -> String {
    word.chars()
        .flat_map(char::to_lowercase)
        .map(|c| match c {
            'á' | 'à' | 'â' | 'ã' | 'ä' => 'a',
            'é' | 'è' | 'ê' | 'ë' => 'e',
            'í' | 'ì' | 'î' | 'ï' => 'i',
            'ó' | 'ò' | 'ô' | 'õ' | 'ö' => 'o',
            'ú' | 'ù' | 'û' | 'ü' => 'u',
            'ç' => 'c',
            'ñ' => 'n',
            c => c,
        })
        .collect()
}

/// For each keyword, the Portuguese and Spanish words that stand for it,
/// folded. A word may stand for more than one keyword where they are
/// expected in different places (`compasso:` is the meter, `no compasso 9`
/// the bar; `tempo 3` after a bar number is the beat).
pub const SYNONYMS: &[(&str, &[&str])] = &[
    // The frame of the piece.
    ("meter", &["compasso", "formula", "metrica", "compas"]),
    ("tempo", &["andamento", "pulso", "bpm"]),
    ("key", &["tom", "tonalidade", "tonalidad", "tono"]),
    ("pickup", &["anacruse", "anacrusa"]),
    ("seed", &["semente", "semilla"]),
    ("form", &["forma", "estrutura", "estructura"]),
    // When.
    ("at", &["no", "na", "em", "en"]),
    ("bar", &["compasso", "compas"]),
    ("bars", &["compassos", "compases"]),
    ("beat", &["tempo", "tiempo"]),
    ("beats", &["tempos", "tiempos"]),
    ("eighth", &["colcheia", "corchea"]),
    ("eighths", &["colcheias", "corcheas"]),
    ("sixteenth", &["semicolcheia", "semicorchea"]),
    ("sixteenths", &["semicolcheias", "semicorcheas"]),
    ("every", &["cada"]),
    ("first", &["primeiro", "primeira", "primero", "primera"]),
    ("last", &["ultimo", "ultima"]),
    ("now", &["agora", "ahora", "ja", "ya"]),
    ("from", &["de", "do", "dos", "desde", "del"]),
    ("to", &["a", "ao", "ate", "al", "hasta", "para"]),
    ("over", &["em", "en", "durante"]),
    ("back", &["volta", "voltando", "vuelta", "volviendo"]),
    ("loop", &["repete", "repetindo", "repite", "repitiendo"]),
    // What plays.
    ("play", &["toque", "toca", "tocar"]),
    (
        "add",
        &["adicione", "acrescente", "junte", "anade", "agrega"],
    ),
    ("pattern", &["padrao", "patron"]),
    ("section", &["secao", "seccion"]),
    ("motif", &["motivo"]),
    ("row", &["serie"]),
    ("list", &["lista"]),
    ("octave", &["oitava", "octava"]),
    ("all", &["todos", "todas", "tudo", "todo"]),
    (
        "random",
        &["acaso", "aleatorio", "aleatoria", "sorteio", "azar"],
    ),
    ("out", &["fora", "sai", "sale", "fuera"]),
    ("in", &["dentro", "entra"]),
    ("humanize", &["humanizar", "humaniza"]),
    // Lines that read their own symbols.
    ("chords", &["acordes", "cifra", "cifras"]),
    ("strum", &["batida", "levada", "rasgueo", "rasgueado"]),
    // Transformations.
    ("rotate", &["girar", "gira", "rotar", "rodar"]),
    ("density", &["densidade", "densidad"]),
    ("velocity", &["intensidade", "intensidad"]),
    ("transpose", &["transpor", "transponer", "transporta"]),
    ("augment", &["aumentar", "aumenta"]),
    ("diminish", &["diminuir", "diminui", "disminuir"]),
];

/// Whether `word` is the keyword `kw` in any of the three languages.
pub fn is(word: &str, kw: &str) -> bool {
    let w = fold(word);
    w == kw
        || SYNONYMS
            .iter()
            .any(|(k, said)| *k == kw && said.iter().any(|s| *s == w))
}

/// Which of the keywords `kws` the word is, if any.
pub fn which<'a>(word: &str, kws: &[&'a str]) -> Option<&'a str> {
    kws.iter().copied().find(|kw| is(word, kw))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn three_languages_one_keyword() {
        assert!(is("Padrão", "pattern"));
        assert!(is("PATRÓN", "pattern"));
        assert!(is("compasso", "bar"));
        assert!(is("compasso", "meter"));
        assert!(is("Pulso", "tempo"));
        assert!(!is("tempo", "bar"));
        assert_eq!(which("toca", &["play", "add"]), Some("play"));
        assert_eq!(which("melodia", &["play", "add"]), None);
        // Every synonym is folded already, or it could never match.
        for (_, said) in SYNONYMS {
            for s in *said {
                assert_eq!(fold(s), *s, "{s}");
            }
        }
    }
}
