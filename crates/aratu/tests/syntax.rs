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

#[test]
fn inversion_and_retrograde_have_the_same_names_everywhere() {
    // Motifs take the names lines and rows use, and the other way round.
    let motif = |form: &str| {
        notes(
            &format!("motif riff = [e4 d4 c4 d4]\nmelodia = | riff.{form} - - - |"),
            1,
            2,
        )
    };
    for (a, b) in [
        ("inv", "invert"),
        ("inv", "I"),
        ("retro", "retrograde"),
        ("retro", "R"),
        ("ri", "RI"),
    ] {
        assert_eq!(motif(a), motif(b), "{a} {b}");
    }
    let line = |t: &str| notes(&format!("baixo = | c2 d2 e2 f2 |\nbaixo.{t}"), 1, 2);
    assert_eq!(line("inv"), line("invert"));
    assert_eq!(line("retro"), line("retrograde"));
    let ri: Vec<u8> = line("ri").iter().map(|n| n.1).collect();
    assert_eq!(ri, [31, 32, 34, 36], "inverted around C, then backwards");
    let row = |f: &str| {
        notes(
            &format!("row s = 0 4 7 11\nmelodia = | o o o o | from s.{f}"),
            1,
            2,
        )
    };
    assert_eq!(row("I0"), row("inv"));
    assert_eq!(row("R5"), row("retrograde5"));
    // `transpose` may say it counts semitones, as `+3st` does.
    assert_eq!(
        notes("baixo = | c2 _ _ _ |\nbaixo.transpose 3st", 1, 2),
        [(1, 39)]
    );
}

#[test]
fn three_x_reads_like_x_three_in_play_and_form() {
    let play = |s: &str| {
        events(
            &format!("pattern g {{ hh = c c c c }}\npattern f {{ sd = s s s s }}\nplay g {s}, f"),
            1,
            5,
        )
        .iter()
        .filter(|e| e.kind == EventKind::Drum(Drum::Snare))
        .map(|e| e.bar)
        .collect::<Vec<_>>()
    };
    assert_eq!(play("3x"), play("x3"));
    assert_eq!(play("3x")[0], 4);
    let form = |s: &str| {
        notes(
            &format!(
                "section a {{ m = | c4 _ _ _ | }}\nsection b {{ m = | g4 _ _ _ | }}\nform: a {s} b"
            ),
            1,
            5,
        )
    };
    assert_eq!(form("2x"), form("x2"));
}

#[test]
fn crescendos_and_tempo_ramps_take_the_lengths_parameters_take() {
    // Two beats of 4/4 are half a bar.
    let level = |len: &str| {
        events(
            &format!(
                "piano = | c4 c c c | c c c c |\npiano: pp\nat bar 1: piano: cresc to ff over {len}"
            ),
            1,
            2,
        )
        .iter()
        .map(|e| e.velocity)
        .collect::<Vec<_>>()
    };
    assert_eq!(level("2 beats"), level("4 eighths"));
    assert_eq!(level("2 beats"), level("8 sixteenths"));
    assert_ne!(level("2 beats"), level("1 bar"));
    let times = |len: &str| {
        events(
            &format!("tempo: 120\nhh = c c c c\nat bar 2: rit to 60 over {len}"),
            1,
            4,
        )
        .iter()
        .map(|e| e.time)
        .collect::<Vec<_>>()
    };
    assert_eq!(times("4 beats"), times("1 bar"));
    assert_ne!(times("2 beats"), times("1 bar"));
}

#[test]
fn chords_in_solfege_no_chord_six_nine_and_altered() {
    let chord = |c: &str| {
        let mut n: Vec<u8> = events(&format!("piano {{ chords = | {c} | }}"), 1, 2)
            .iter()
            .filter_map(|e| match e.kind {
                EventKind::Note(n) => Some(n % 12),
                _ => None,
            })
            .collect();
        n.sort_unstable();
        n.dedup();
        n
    };
    assert_eq!(chord("Lám"), chord("Am"));
    assert_eq!(chord("Sol7"), chord("G7"));
    assert_eq!(chord("Fá#m7"), chord("F#m7"));
    assert_eq!(chord("Dó/Mi"), chord("C/E"));
    assert_eq!(chord("Sib"), chord("Bb"));
    // Letters stay letters: F augmented, not `Fa` and `ug`.
    assert_eq!(chord("Faug"), [1, 5, 9]);
    assert_eq!(chord("C6/9"), [0, 2, 4, 7, 9]);
    assert_eq!(chord("E7alt"), [0, 2, 4, 7, 8]);
    for nc in ["N.C.", "NC", "-"] {
        assert!(chord(nc).is_empty(), "{nc}");
    }
}
