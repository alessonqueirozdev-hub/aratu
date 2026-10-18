// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
//! Repeats inside a bar, motifs and their transformations, twelve-tone rows
//! and melody lists.

use aratu::compile;
use aratu::diag::{Message, Severity};
use aratu::ratio::Ratio;
use aratu::schedule::{EventKind, Options, render};

/// The notes of bars 1 to `bars`, in order, and the piece's messages.
fn notes(src: &str, bars: i64) -> (Vec<u8>, Vec<Message>) {
    let c = compile(src);
    let messages = c.diagnostics.iter().map(|d| d.message.clone()).collect();
    let r = render(&c.piece, &Options::default(), 1, bars + 1);
    let notes = r
        .events
        .iter()
        .filter_map(|e| match e.kind {
            EventKind::Note(n) => Some(n),
            _ => None,
        })
        .collect();
    (notes, messages)
}

fn clean(src: &str, bars: i64) -> Vec<u8> {
    let c = compile(src);
    let errors: Vec<_> = c
        .diagnostics
        .iter()
        .filter(|d| d.severity == Severity::Error || d.severity == Severity::Warning)
        .collect();
    assert!(errors.is_empty(), "{src}: {errors:?}");
    notes(src, bars).0
}

const RIFF: [u8; 4] = [52, 50, 48, 50];

#[test]
fn a_beat_repeats_inside_the_bar() {
    let n = clean("melodia = | [e3 d3 c3 d3]x4 |", 1);
    assert_eq!(n, RIFF.repeat(4));
    let c = compile("melodia = | [e3 d3 c3 d3]x4 |");
    let r = render(&c.piece, &Options::default(), 1, 2);
    assert_eq!(
        r.events[4].beat,
        Ratio::int(1),
        "each repeat takes its own beat"
    );
    // Too many repeats for the bar is the usual beat count.
    let (_, m) = notes("melodia = | [e3 d3 c3 d3]x5 |", 1);
    assert!(
        m.iter().any(|m| matches!(m, Message::BeatCount { .. })),
        "{m:?}"
    );
    // Drums too, and a crash after a closed hat is still two symbols.
    let c = compile("hh = | [cc]x3 co |");
    assert!(c.diagnostics.is_empty(), "{:?}", c.diagnostics);
    assert_eq!(render(&c.piece, &Options::default(), 1, 2).events.len(), 8);
}

#[test]
fn a_motif_is_used_by_name_even_above_its_definition() {
    let n = clean(
        "melodia = | riff riff riffx2 |\nmotif riff = [e3 d3 c3 d3]",
        1,
    );
    assert_eq!(n, RIFF.repeat(4));
}

#[test]
fn a_motif_moves_by_degrees_of_the_key_or_by_semitones() {
    let src = "key: C major\nmotif riff = [e3 d3 c3 d3]\nmelodia = | riff riff+1 riff-1 riff+3st |";
    let n = clean(src, 1);
    assert_eq!(n[4..8], [53, 52, 50, 52], "a degree up: F E D E");
    assert_eq!(n[8..12], [50, 48, 47, 48], "a degree down: D C B C");
    assert_eq!(n[12..16], [55, 53, 51, 53], "three semitones up");
    // In A minor, E moves to F and G to A.
    let n = clean(
        "key: A minor\nmotif m = [e3 g3]\nmelodia = | m+1 m m m |",
        1,
    );
    assert_eq!(n[..2], [53, 57]);
}

#[test]
fn inversion_retrograde_and_retrograde_inversion() {
    let src = "motif riff = [e3 d3 c3 d3]\nmelodia = | riff.inv riff.retro riff.ri riff.ri+2st |";
    let n = clean(src, 1);
    assert_eq!(n[0..4], [52, 54, 56, 54], "intervals upside down around E");
    assert_eq!(n[4..8], [50, 48, 50, 52], "the notes backwards");
    assert_eq!(n[8..12], [54, 56, 54, 52], "the inversion backwards");
    assert_eq!(n[12..16], [56, 58, 56, 54]);
}

#[test]
fn motifs_tell_their_mistakes_where_they_are_written() {
    let (_, m) = notes("motif a = b\nmotif b = a\nmelodia = | a a a a |", 1);
    assert!(
        m.iter().any(|m| matches!(m, Message::MotifLoop { .. })),
        "{m:?}"
    );
    let (_, m) = notes("motif a = | c4 d4 |\nmelodia = | c4 d4 e4 f4 |", 1);
    assert!(m.contains(&Message::MotifWithBars), "{m:?}");
    // A drum motif reads as drums.
    let c = compile("motif fill = ssss\nsd = | s s s fill |");
    assert!(c.diagnostics.is_empty(), "{:?}", c.diagnostics);
}

#[test]
fn a_twelve_tone_row_fills_a_rhythm() {
    let row = "row serie = 0 11 3 4 8 7 9 5 6 1 2 10\n";
    let rhythm = "| o o o o | o o o o | o o o o |";
    let p0 = clean(&format!("{row}melodia = {rhythm} from serie.P0"), 3);
    assert_eq!(p0, [60, 71, 63, 64, 68, 67, 69, 65, 66, 61, 62, 70]);
    let r0 = clean(&format!("{row}melodia = {rhythm} from serie.R0"), 3);
    assert_eq!(r0, [70, 62, 61, 66, 65, 69, 67, 68, 64, 63, 71, 60]);
    let i0 = clean(&format!("{row}melodia = {rhythm} from serie.I0"), 3);
    assert_eq!(i0[..4], [60, 49, 57, 56]);
    let p5 = clean(
        &format!("{row}melodia = {rhythm} from serie.P5 octave 3"),
        3,
    );
    assert_eq!(p5[..3], [53, 52, 56], "five up, every note in octave 3");
    // A rhythm with fewer places takes the next notes the next time round.
    let short = clean(&format!("{row}melodia = | o [o o] O _ | from serie"), 1);
    assert_eq!(short, [60, 71, 63, 64]);
}

#[test]
fn an_inline_vector_cycles_like_an_isorhythm() {
    let n = clean("melodia = | o o o o | o o _ _ | from [0 4 7]", 2);
    assert_eq!(n, [60, 64, 67, 60, 64, 67]);
}

#[test]
fn rows_are_checked() {
    let (_, m) = notes(
        "row s = 0 1 2 3 4 5 6 7 8 9 10 0\nmelodia = | o o o o | from s",
        1,
    );
    assert!(
        m.iter()
            .any(|m| matches!(m, Message::RowRepeats { note } if note == "0")),
        "{m:?}"
    );
    let (_, m) = notes("row s = 0 12 h\nmelodia = | o o o o | from s", 1);
    assert_eq!(
        m.iter()
            .filter(|m| matches!(m, Message::BadRowNote { .. }))
            .count(),
        2
    );
    let (_, m) = notes("melodia = | o o o o | from nada", 1);
    assert!(
        m.iter().any(|m| matches!(m, Message::UnknownRow { .. })),
        "{m:?}"
    );
    let (_, m) = notes("row s = 0 1 2\nmelodia = | o o o o | from s.X2", 1);
    assert!(
        m.iter().any(|m| matches!(m, Message::BadRowForm { .. })),
        "{m:?}"
    );
    let (_, m) = notes("row s = 0 1 2\nmelodia = | c4 d4 e4 f4 | from s", 1);
    assert!(m.contains(&Message::NoPlaceholders), "{m:?}");
}

#[test]
fn a_melody_is_picked_from_a_list_by_number() {
    let list = "list temas {\n  | c4 d4 e4 f4 |\n  | g4 a4 b4 c5 |\n}\n";
    let two = clean(&format!("{list}melodia = temas.2"), 1);
    assert_eq!(two, [67, 69, 71, 72]);
    let all = clean(&format!("{list}melodia = temas.all"), 2);
    assert_eq!(all, [60, 62, 64, 65, 67, 69, 71, 72]);
    let (_, m) = notes(&format!("{list}melodia = temas.5"), 1);
    assert!(
        m.contains(&Message::ListIndex {
            name: "temas".into(),
            index: 5,
            count: 2
        }),
        "{m:?}"
    );
    // A melody's mistake is told in the list.
    let c = compile("list t {\n  | c4 d4 h4 f4 |\n}\nmelodia = t.1");
    assert!(
        c.diagnostics
            .iter()
            .any(|d| matches!(d.message, Message::UnknownNote { .. }))
    );
}

#[test]
fn a_random_list_chooses_a_whole_melody_each_time_round() {
    let list = "list temas {\n  | c4 _ _ _ |\n  | e4 _ _ _ | f4 _ _ _ |\n  | g4 _ _ _ |\n}\n";
    let firsts = |src: &str| -> Vec<u8> {
        let c = compile(src);
        assert!(c.diagnostics.is_empty(), "{:?}", c.diagnostics);
        render(&c.piece, &Options::default(), 1, 41)
            .events
            .iter()
            .filter_map(|e| match e.kind {
                EventKind::Note(n) => Some(n),
                _ => None,
            })
            .collect()
    };
    let a = firsts(&format!("{list}melodia = temas.random"));
    assert_eq!(a.len(), 40);
    // Every melody turns up, and the two-bar one always whole.
    for n in [60, 64, 67] {
        assert!(a.contains(&n), "{n} in {a:?}");
    }
    for (i, n) in a.iter().enumerate() {
        if *n == 64 && i + 1 < a.len() {
            assert_eq!(a[i + 1], 65, "E is always followed by F: {a:?}");
        }
        if *n == 65 {
            assert_eq!(a[i - 1], 64, "F only after E: {a:?}");
        }
    }
    // The same seed plays the same order; another seed another.
    assert_eq!(a, firsts(&format!("{list}melodia = temas.random")));
    let b = firsts(&format!("seed 9\n{list}melodia = temas.random"));
    assert_ne!(a, b);
}

#[test]
fn a_list_melody_may_use_motifs_and_rows() {
    let src = "row s = 0 4 7\nmotif m = [c4 d4]\nlist t {\n  | m m m m |\n  | o o o o | from s\n}\nmelodia = t.all";
    let n = clean(src, 2);
    assert_eq!(n[..8], [60, 62, 60, 62, 60, 62, 60, 62]);
    assert_eq!(n[8..], [60, 64, 67, 60]);
}

#[test]
fn list_melodies_fill_bars_inside_a_line() {
    let list = "list temas {\n  | a2 e3 a e |\n  | a2 f3 a f |\n  | g2 c3 g c |\n}\n";
    // With or without the outer bar lines, one melody per bar.
    for line in [
        "melodia = | temas.1 | temas.3 | temas.2 |",
        "melodia = temas.1 | temas.3 | temas.2",
    ] {
        let n = clean(&format!("{list}{line}"), 3);
        assert_eq!(
            n,
            [45, 52, 57, 52, 43, 48, 55, 48, 45, 53, 57, 53],
            "{line}"
        );
    }
    // Mixed with written bars, and all of them as one bar's worth.
    let n = clean(&format!("{list}melodia = | c4 _ _ _ | temas.all |"), 4);
    assert_eq!(n.len(), 13);
    let (_, m) = notes(&format!("{list}melodia = | temas.1 | temas.9 |"), 2);
    assert!(
        m.iter().any(|m| matches!(
            m,
            Message::ListIndex {
                index: 9,
                count: 3,
                ..
            }
        )),
        "{m:?}"
    );
    // A list that names itself stops, with a message.
    let (_, m) = notes("list t {\n  | t.1 |\n}\nmelodia = | t.1 |", 1);
    assert!(
        m.iter().any(|m| matches!(m, Message::MotifLoop { .. })),
        "{m:?}"
    );
}

#[test]
fn four_x_reads_like_x_four_and_braces_alone_suggest_brackets() {
    let a = clean("melodia = | [a2 e3 a e]4x |", 1);
    let b = clean("melodia = | [a2 e3 a e]x4 |", 1);
    assert_eq!(a, b);
    assert_eq!(a.len(), 16);
    // Braces with one option still play as a group, with a hint.
    let (n, m) = notes("melodia = | {a2 e3 a e}x4 |", 1);
    assert_eq!(n, a);
    assert!(
        m.contains(&Message::BracesOneOption {
            inner: "a2 e3 a e".into()
        }),
        "{m:?}"
    );
}
