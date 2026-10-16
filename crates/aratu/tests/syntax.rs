//! What the review of the whole syntax (16 October 2026) asked for: things
//! accepted that did something else, messages that misled, and the same
//! idea written differently in different places.

use aratu::compile;
use aratu::diag::{Lang, Message};
use aratu::ratio::Ratio;
use aratu::schedule::{Drum, Event, EventKind, Options, render};

fn events(src: &str, from: i64, to: i64) -> Vec<Event> {
    let c = compile(src);
    let errors: Vec<_> = c.errors().map(|d| d.text(Lang::En)).collect();
    assert!(errors.is_empty(), "{src}: {errors:?}");
    render(&c.piece, &Options::default(), from, to).events
}

fn notes(src: &str, from: i64, to: i64) -> Vec<(i64, u8)> {
    events(src, from, to)
        .iter()
        .filter_map(|e| match e.kind {
            EventKind::Note(n) => Some((e.bar, n)),
            _ => None,
        })
        .collect()
}

fn drums(src: &str) -> Vec<Drum> {
    events(src, 1, 2)
        .iter()
        .filter_map(|e| match e.kind {
            EventKind::Drum(d) => Some(d),
            _ => None,
        })
        .collect()
}

/// The first message of a piece.
fn first(src: &str) -> Message {
    compile(src)
        .diagnostics
        .first()
        .unwrap_or_else(|| panic!("{src} should say something"))
        .message
        .clone()
}

#[test]
fn drum_lines_and_parts_by_their_portuguese_and_spanish_names() {
    assert_eq!(drums("caixa = - s - s"), [Drum::Snare, Drum::Snare]);
    assert_eq!(drums("bumbo = b - b -"), [Drum::Kick, Drum::Kick]);
    assert_eq!(
        drums("chimbal = c - c -"),
        [Drum::ClosedHat, Drum::ClosedHat]
    );
    assert_eq!(drums("bombo = b - - -"), [Drum::Kick]);
    assert_eq!(drums("caja = - s - -"), [Drum::Snare]);
    // Any line inside a `bateria` block reads as drums.
    assert_eq!(drums("bateria { x = b s b s }").len(), 4);
    assert_eq!(drums("batería { x = b - - - }"), [Drum::Kick]);
}

#[test]
fn hands_clap_with_p() {
    assert_eq!(drums("palmas = - p - p"), [Drum::Clap, Drum::Clap]);
    assert_eq!(drums("cp = p p p p").len(), 4);
    // A dynamic still needs its colon, so `p:` is soft and `p` claps.
    let e = events("palmas = p: p p f: p p", 1, 2);
    assert!(e[0].velocity < e[2].velocity);
}

#[test]
fn a_header_in_portuguese_or_misspelt_is_told_with_the_right_word() {
    for (src, word, suggestion) in [
        ("andamento: 120", "andamento", "tempo"),
        ("compasso: 3/4", "compasso", "meter"),
        ("tom: lá menor", "tom", "key"),
        ("tempp: 120", "tempp", "tempo"),
        ("metter: 3/4", "metter", "meter"),
    ] {
        assert_eq!(
            first(src),
            Message::UnknownHeader {
                word: word.into(),
                suggestion: suggestion.into(),
            },
            "{src}"
        );
    }
    // A part told nothing it understands hears what it can be told.
    assert_eq!(
        first("bass = | c2 _ _ _ |\nbass: 120"),
        Message::BadPartCommand {
            part: "bass".into()
        }
    );
    // A broken chain is still told as a chain.
    assert!(matches!(
        first("bass: synth.saw ->"),
        Message::Expected { .. }
    ));
}

#[test]
fn misspelt_keywords_parameters_motifs_and_lists_get_a_suggestion() {
    let src = "pattern g { hh = c c c c }\nply g";
    assert_eq!(
        first(src),
        Message::UnknownWord {
            word: "ply".into(),
            suggestion: "play".into()
        }
    );
    assert_eq!(
        first("patern g { hh = c c c c }"),
        Message::UnknownWord {
            word: "patern".into(),
            suggestion: "pattern".into()
        }
    );
    let c = compile("bass = | c2 _ _ _ |\nbass.lowpass.cutof = 800 Hz");
    assert!(c.diagnostics.iter().any(|d| d.message
        == Message::UnknownParam {
            name: "lowpass.cutof".into(),
            closest: Some("lowpass.cutoff".into())
        }));
    assert_eq!(
        first("motif riff = [e4 d4]\nmelodia = | riff.foo riff riff riff |"),
        Message::UnknownMotifForm {
            name: "riff".into(),
            form: "foo".into()
        }
    );
    assert_eq!(
        first("list t {\n  | c4 d e f |\n}\nmelodia = | t.foo c4 d e |"),
        Message::UnknownListPick {
            name: "t".into(),
            pick: "foo".into()
        }
    );
    // `every` counts bars, and says so.
    let src = "pattern g { hh = c c c c }\nplay g\nevery 2 beats: play g";
    assert!(
        compile(src).diagnostics[0]
            .text(Lang::En)
            .contains("every counts whole bars")
    );
}

#[test]
fn a_line_changes_at_a_bar() {
    let src = "melodia = | c4 _ _ _ | d4 _ _ _ |\nat bar 3: melodia = | g4 _ _ _ |";
    let c = compile(src);
    assert!(c.diagnostics.is_empty(), "{:?}", c.diagnostics);
    assert_eq!(
        notes(src, 1, 6),
        [(1, 60), (2, 62), (3, 67), (4, 67), (5, 67)]
    );
    // It starts from its own first bar, and a later one takes over.
    let src = "melodia = | c4 _ _ _ |\nat bar 2: melodia = | e4 _ _ _ | f4 _ _ _ |\nat bar 5: melodia = | g4 _ _ _ |";
    assert_eq!(
        notes(src, 1, 6),
        [(1, 60), (2, 64), (3, 65), (4, 64), (5, 67)]
    );
    // A line written only at a bar is silent before it.
    let src = "baixo = | c2 _ _ _ |\nat bar 3: melodia = | g4 _ _ _ |";
    assert_eq!(notes(src, 1, 4), [(1, 36), (2, 36), (3, 36), (3, 67)]);
    // `every` does not change a line; it says how to vary one.
    assert_eq!(
        first("every 4 bars: melodia = | c4 _ _ _ |"),
        Message::LineChangeOnlyAtBar
    );
    let e = events(
        "melodia = | c4 _ _ _ |\nat bar 2: melodia = | g4 _ _ _ |",
        2,
        3,
    );
    assert_eq!(e[0].beat, Ratio::ZERO);
}
