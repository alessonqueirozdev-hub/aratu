// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
//! What the review of the whole syntax (16 October 2026) asked for: things
//! accepted that did something else, messages that misled, and the same
//! idea written differently in different places.

use aratu::compile;
use aratu::diag::{Lang, Message};
use aratu::ratio::Ratio;
use aratu::schedule::{Drum, Event, EventKind, Options, render};
use aratu::span::Span;

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
fn a_misspelt_header_is_told_with_the_right_word() {
    for (src, word, suggestion) in [
        ("tempp: 120", "tempp", "tempo"),
        ("metter: 3/4", "metter", "meter"),
        ("velocidade: 120", "velocidade", "tempo"),
        ("escala: lá menor", "escala", "key"),
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

/// The drums of bars `from` to `to`, as (bar, beat, drum).
fn hits(src: &str, from: i64, to: i64) -> Vec<(i64, Ratio, Drum)> {
    events(src, from, to)
        .iter()
        .filter_map(|e| match e.kind {
            EventKind::Drum(d) => Some((e.bar, e.beat, d)),
            _ => None,
        })
        .collect()
}

fn params(src: &str, from: i64, to: i64) -> Vec<(i64, Ratio, f64, bool)> {
    events(src, from, to)
        .iter()
        .filter_map(|e| match e.kind {
            EventKind::Param { value, reset, .. } => Some((e.bar, e.beat, value, reset)),
            _ => None,
        })
        .collect()
}

const GROOVE: &str = "tempo: 120\npattern g { hh = c c c c }\npattern f { sd = s s s s }\nplay g\n";

#[test]
fn a_change_at_a_beat_cuts_the_bar_there() {
    // The fill comes in at the third beat of bar 2.
    let h = hits(&format!("{GROOVE}at bar 2 beat 3: play f"), 2, 3);
    let kinds: Vec<Drum> = h.iter().map(|x| x.2).collect();
    assert_eq!(
        kinds,
        [Drum::ClosedHat, Drum::ClosedHat, Drum::Snare, Drum::Snare]
    );
    assert_eq!(h[2].1, Ratio::int(2));
    // A part goes out at a beat.
    let h = hits("hh = c c c c\nat bar 1 beat 3: drums: out", 1, 3);
    assert_eq!(h.len(), 2);
    // A transformation from a beat on.
    let n = notes(
        "baixo = | c2 c2 c2 c2 |\nat bar 1 beat 3: baixo.transpose 12",
        1,
        2,
    );
    assert_eq!(n.iter().map(|x| x.1).collect::<Vec<_>>(), [36, 36, 48, 48]);
    // A parameter at its beat, and between beats with a decimal.
    let p = params(
        "bass = | c2 _ _ _ |\nat bar 1 beat 2.5: bass.reverb.mix = 0.5",
        1,
        2,
    );
    assert_eq!(p[0].1, Ratio::new(3, 2));
    // The tempo changes there: two beats at 120, two at 60.
    let e = events("tempo: 120\nhh = c c c c\nat bar 1 beat 3: tempo 60", 1, 2);
    let t: Vec<f64> = e.iter().map(|x| x.time).collect();
    assert_eq!(t, [0.0, 0.5, 1.0, 2.0]);
    // A level from that beat.
    let v: Vec<u8> = events(
        "piano = | c4 c c c |\npiano: pp\nat bar 1 beat 3: piano: ff",
        1,
        2,
    )
    .iter()
    .map(|x| x.velocity)
    .collect();
    assert!(v[1] < v[2], "{v:?}");
}

#[test]
fn a_range_holds_for_its_bars_then_what_was_comes_back() {
    let h = hits("hh = c c c c\nfrom bar 2 to 3: drums: out", 1, 5);
    let bars: Vec<i64> = h.iter().map(|x| x.0).collect();
    assert_eq!(bars, [1, 1, 1, 1, 4, 4, 4, 4]);
    // A pattern for two bars; the one before kept counting meanwhile.
    let h = hits(&format!("{GROOVE}from bar 2 to 3: play f"), 1, 5);
    let snares: Vec<i64> = h
        .iter()
        .filter(|x| x.2 == Drum::Snare)
        .map(|x| x.0)
        .collect();
    assert_eq!(snares, [2, 2, 2, 2, 3, 3, 3, 3]);
    assert!(h.iter().any(|x| x.0 == 4 && x.2 == Drum::ClosedHat));
    // A transformation for one bar.
    let n = notes(
        "baixo = | c2 _ _ _ |\nfrom bar 2 to 2: baixo.transpose 12",
        1,
        4,
    );
    assert_eq!(n, [(1, 36), (2, 48), (3, 36)]);
    // A line for two bars, then the line as it was, in its place.
    let n = notes(
        "melodia = | c4 _ _ _ | d4 _ _ _ |\nfrom bar 2 to 3: melodia = | g4 _ _ _ |",
        1,
        6,
    );
    assert_eq!(n, [(1, 60), (2, 67), (3, 67), (4, 62), (5, 60)]);
}

#[test]
fn a_parameter_for_some_bars_goes_back_to_what_was_set() {
    // Back to the value written at a bar before.
    let src = "bass = | c2 _ _ _ |\nat bar 1: bass.reverb.mix = 0.3\nfrom bar 2 to 3: bass.reverb.mix = 0.8";
    let p = params(src, 1, 5);
    assert_eq!(
        p,
        [
            (1, Ratio::ZERO, 0.3, false),
            (2, Ratio::ZERO, 0.8, false),
            (4, Ratio::ZERO, 0.3, false)
        ]
    );
    // With nothing written at a bar, back to how the part is set up.
    let src = "bass = | c2 _ _ _ |\nbass.reverb.mix = 0.2\nfrom bar 2 to 3: bass.reverb.mix = 0.8";
    let p = params(src, 1, 5);
    assert_eq!(p.last().unwrap(), &(4, Ratio::ZERO, 0.0, true));
    // A motion for some bars stops, and the value goes back.
    let src =
        "bass = | c2 _ _ _ |\nfrom bar 2 to 3: bass.lowpass.cutoff = 300 Hz -> 3000 Hz over 2 bars";
    let p = params(src, 1, 7);
    assert!(p.iter().all(|x| x.0 >= 2 && x.0 <= 4), "{:?}", p.last());
    assert!(p.last().unwrap().3, "reset at bar 4");
    // A motion taken over for some bars goes on afterwards, from where it
    // would be by then.
    let src = "bass = | c2 _ _ _ |\nbass.reverb.mix = 0 <-> 1 over 2 bars\nfrom bar 2 to 2: bass.reverb.mix = 0.5";
    let p = params(src, 1, 4);
    let bar2: Vec<_> = p.iter().filter(|x| x.0 == 2).collect();
    assert_eq!(bar2.len(), 1);
    assert_eq!(bar2[0].2, 0.5);
    assert!(
        p.iter()
            .any(|x| x.0 == 3 && x.1 == Ratio::ZERO && (x.2 - 1.0).abs() < 1e-9)
    );
}

#[test]
fn times_that_cannot_be_are_told() {
    assert_eq!(first("from bar 2 to 3: tempo 90"), Message::FrameInRange);
    assert_eq!(first("at bar 2 beat 3: meter 3/4"), Message::FrameAtBeat);
    assert!(matches!(
        first(&format!("{GROOVE}at bar 2 beat 5: play f")),
        Message::BeatOutOfBar { bar: 2, .. }
    ));
    assert_eq!(
        first(&format!("{GROOVE}from bar 5 to 2: play f")),
        Message::BadRange { from: 5, to: 2 }
    );
}

#[test]
fn the_same_piece_in_english_portuguese_and_spanish() {
    let en = "meter: 4/4
tempo: 100
key: A minor
seed 7
pattern groove { hh = cc cc cc cc; bd = b - b -; sd = - s - s }
pattern fill { sd = ss ss SS SS }
motif riff = [e4 d4 c4 d4]
row serie = 0 4 7 11
list temas {
  | a4 _ _ _ |
  | g4 _ _ _ |
}
melodia = | riff riff+1 riff.inv riff |
bass = | o o o o | from serie octave 2
tune = temas.all
play groove x3, fill
at bar 4 beat 3: add fill once
every 8 bars, first: bass.transpose 2
from bar 5 to 6: drums: out
at bar 7: melodia: cresc to ff over 6 beats
at bar 9: tempo 120 -> 132 over 2 bars
at bar 2: bass.lowpass.cutoff = 300 Hz <-> 3000 Hz over 4 sixteenths, back over 2 eighths, loop
chords = | Am | F |
strum = d du -u du
all: swing 60%";
    let pt = "compasso: 4/4
andamento: 100
tom: lá menor
semente 7
padrão groove { hh = cc cc cc cc; bd = b - b -; sd = - s - s }
padrão fill { sd = ss ss SS SS }
motivo riff = [e4 d4 c4 d4]
série serie = 0 4 7 11
lista temas {
  | a4 _ _ _ |
  | g4 _ _ _ |
}
melodia = | riff riff+1 riff.inversão riff |
bass = | o o o o | com serie oitava 2
tune = temas.todas
toque groove 3x, fill
no compasso 4 tempo 3: adicione fill uma vez
a cada 8 compassos, primeiro: bass.transpor 2
do compasso 5 ao 6: bateria: fora
no compasso 7: melodia: cresc até ff em 6 tempos
no compasso 9: andamento 120 -> 132 em 2 compassos
no compasso 2: bass.lowpass.cutoff = 300 Hz <-> 3000 Hz em 4 semicolcheias, volta em 2 colcheias, repete
acordes = | Am | F |
batida = d du -u du
todos: swing 60%";
    let es = "compás: 4/4
pulso: 100
tonalidad: la menor
semilla 7
patrón groove { hh = cc cc cc cc; bd = b - b -; sd = - s - s }
patrón fill { sd = ss ss SS SS }
motivo riff = [e4 d4 c4 d4]
serie serie = 0 4 7 11
lista temas {
  | a4 _ _ _ |
  | g4 _ _ _ |
}
melodia = | riff riff+1 riff.invertir riff |
bass = | o o o o | desde serie octava 2
tune = temas.todos
toca groove x3, fill
en el compás 4, tiempo 3: añade fill una vez
cada 8 compases, primero: bass.transponer 2
desde el compás 5 hasta el 6: batería: fuera
en el compás 7: melodia: cresc hasta ff en 6 tiempos
en el compás 9: pulso 120 -> 132 en 2 compases
en compás 2: bass.lowpass.cutoff = 300 Hz <-> 3000 Hz en 4 semicorcheas, vuelta en 2 corcheas, repite
acordes = | Am | F |
rasgueo = d du -u du
todos: swing 60%";
    let play = |src: &str| -> Vec<String> {
        events(src, 1, 13)
            .iter()
            .map(|e| {
                format!(
                    "{} {} {:?} {} {:.4}",
                    e.bar, e.beat, e.kind, e.velocity, e.time
                )
            })
            .collect()
    };
    let english = play(en);
    assert!(english.len() > 300);
    for (lang, src) in [("Portuguese", pt), ("Spanish", es)] {
        let other = play(src);
        let at = other
            .iter()
            .zip(&english)
            .position(|(a, b)| a != b)
            .unwrap_or(other.len().min(english.len()));
        assert!(
            other.len() == english.len() && at == other.len(),
            "{lang}: from event {at}: {:?} against {:?}",
            &other[at..(at + 3).min(other.len())],
            &english[at..(at + 3).min(english.len())]
        );
    }
}

#[test]
fn a_long_line_goes_on_on_the_next_lines() {
    let one = notes("melodia = | c4 d e f | g a b c5 |", 1, 3);
    for src in [
        "melodia = | c4 d e f |\n          | g a b c5 |",
        "melodia = | c4 d e f | // the first bar\n  | g a b c5 |",
        "piano {\n  melodia = | c4 d e f |\n    | g a b c5 |\n}",
    ] {
        let c = compile(src);
        assert!(c.diagnostics.is_empty(), "{src}: {:?}", c.diagnostics);
        assert_eq!(notes(src, 1, 3), one, "{src}");
    }
    // Chords too, and the light falls on the second line's symbols.
    let src = "chords = | Am | F |\n         | C | G |";
    let e = events(src, 4, 5);
    let n = e
        .iter()
        .find(|x| matches!(x.kind, EventKind::Note(_)))
        .unwrap();
    assert_eq!(&src[n.span.start..n.span.end], "G");
    // In a list each line is a melody of its own.
    let src = "list t {\n  | c4 _ _ _ |\n  | g4 _ _ _ |\n}\nmelodia = t.2";
    assert_eq!(notes(src, 1, 2), [(1, 67)]);
    // A bar line after something that is not music is still a mistake.
    assert_eq!(first("tempo: 100\n| c4 d e f |"), Message::UnknownLine);
}

fn beats_of(src: &str) -> Vec<Ratio> {
    events(src, 1, 2).iter().map(|e| e.beat).collect()
}

#[test]
fn a_tuplet_may_span_several_beats() {
    // Three notes in two beats: a half-note triplet's worth of quarters.
    let b = beats_of("melodia = | 2[c4 d e] f g |");
    assert_eq!(
        b,
        [
            Ratio::ZERO,
            Ratio::new(2, 3),
            Ratio::new(4, 3),
            Ratio::int(2),
            Ratio::int(3)
        ]
    );
    // Four in three, then a beat's rest; drums the same way.
    let b = beats_of("hh = | 3[c c c c] - |");
    assert_eq!(
        b,
        [
            Ratio::ZERO,
            Ratio::new(3, 4),
            Ratio::new(3, 2),
            Ratio::new(9, 4)
        ]
    );
    // It counts its beats in the bar.
    assert!(matches!(
        first("melodia = | 2[c4 d e] f |"),
        Message::BeatCount { .. }
    ));
}

#[test]
fn an_additive_meter_groups_its_units() {
    // One word per group: the groups' lengths.
    let b = beats_of("meter: 7/8 = 2+2+3\nhh = | cc cc ccc |");
    assert_eq!(b, (0..7).map(Ratio::int).collect::<Vec<_>>());
    let b = beats_of("meter: 7/8 = 2+2+3\nbd = | b b b |");
    assert_eq!(b, [Ratio::ZERO, Ratio::int(2), Ratio::int(4)]);
    // Or one word per unit, as any 7/8.
    let b = beats_of("meter: 7/8 = 2+2+3\nhh = | c c c c c c c |");
    assert_eq!(b.len(), 7);
    // In Portuguese too, and the groups must add up.
    let b = beats_of("compasso: 7/8 = 3+2+2\nbumbo = | b b b |");
    assert_eq!(b, [Ratio::ZERO, Ratio::int(3), Ratio::int(5)]);
    assert_eq!(
        first("meter: 7/8 = 2+3\nhh = | c c c c c c c |"),
        Message::BadMeterGroups {
            text: "2+3".into(),
            units: 7
        }
    );
    assert_eq!(
        compile("meter: 7/8 = 2+2+3").piece.meter.to_string(),
        "7/8 = 2+2+3"
    );
}

#[test]
fn marcato_ghost_notes_and_a_fermata_s_length() {
    let v: Vec<u8> = events("sd = s S s! (s)", 1, 2)
        .iter()
        .map(|e| e.velocity)
        .collect();
    assert!(v[0] < v[1] && v[1] < v[2], "marcato above an accent: {v:?}");
    assert!(
        f64::from(v[3]) < f64::from(v[0]) * 0.5,
        "a ghost note barely sounds: {v:?}"
    );
    let e = events("sd = s S s! -", 1, 2);
    assert!(e[2].accent, "marcato tells the engine it is accented");
    // On notes too.
    let v: Vec<u8> = events("melodia = | (c4) d e! f |", 1, 2)
        .iter()
        .map(|e| e.velocity)
        .collect();
    assert!(v[0] < v[1] && v[2] > v[1], "{v:?}");
    // A fermata holds twice as long, or as many times as written.
    let t = |src: &str| events(src, 1, 2).iter().map(|e| e.time).collect::<Vec<_>>();
    assert_eq!(t("tempo: 60\nmelodia = | c4^ d e f |")[1], 2.0);
    assert_eq!(t("tempo: 60\nmelodia = | c4^3 d e f |")[1], 3.0);
    assert_eq!(
        first("sd = (s s - -"),
        Message::UnclosedBracket { open: '(' }
    );
}

#[test]
fn chords_by_their_degree_in_the_key() {
    let pcs = |src: &str, bar: i64| -> Vec<u8> {
        let mut n: Vec<u8> = events(src, bar, bar + 1)
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
    let same = |a: &str, b: &str, bars: i64| {
        for bar in 1..=bars {
            assert_eq!(pcs(a, bar), pcs(b, bar), "{a} / {b}, bar {bar}");
        }
    };
    same(
        "key: C major\nchords = | I | vi | IV | V7 |",
        "chords = | C | Am | F | G7 |",
        4,
    );
    same(
        "key: A minor\nchords = | i | iv | V7 | VI |",
        "chords = | Am | Dm | E7 | F |",
        4,
    );
    same(
        "key: C major\nchords = | ii7 | vii° | bVII | V7/V |",
        "chords = | Dm7 | Bdim | Bb | D7 |",
        4,
    );
    // In Portuguese, with the key in solfège.
    same(
        "tom: Sol maior\nacordes = | I | V |",
        "chords = | G | D |",
        2,
    );
}

#[test]
fn a_rolled_chord_goes_up_from_the_lowest_note() {
    for src in ["chords = | Am:arp |", "melodia = | a3+c4+e4:arp _ _ _ |"] {
        let e = events(src, 1, 2);
        let mut last = (0u8, -1.0);
        for x in &e {
            let EventKind::Note(n) = x.kind else { continue };
            assert!(n > last.0 && x.time > last.1, "{src}: {e:?}");
            last = (n, x.time);
        }
    }
}

#[test]
fn brazilian_percussion_lines_have_strokes_of_their_own() {
    use Drum::*;
    let cases: [(&str, &[Drum]); 11] = [
        (
            "pandeiro = pd td pd td",
            &[
                PandeiroThumb,
                PandeiroFingers,
                PandeiroSlap,
                PandeiroFingers,
                PandeiroThumb,
                PandeiroFingers,
                PandeiroSlap,
                PandeiroFingers,
            ],
        ),
        (
            "zabumba = o - b m",
            &[ZabumbaOpen, ZabumbaStick, ZabumbaMuted],
        ),
        (
            "triângulo = o f o f",
            &[TriangleOpen, TriangleClosed, TriangleOpen, TriangleClosed],
        ),
        (
            "agogô = a g a g",
            &[AgogoHigh, AgogoLow, AgogoHigh, AgogoLow],
        ),
        ("surdo = - o - m", &[SurdoOpen, SurdoMuted]),
        (
            "tamborim = o o m o",
            &[TamborimOpen, TamborimOpen, TamborimMuted, TamborimOpen],
        ),
        ("alfaia = o - m -", &[AlfaiaOpen, AlfaiaMuted]),
        (
            "gonguê = o m o m",
            &[GongueOpen, GongueMuted, GongueOpen, GongueMuted],
        ),
        ("ganzá = ssss ssss ssss ssss", &[Shaker; 16]),
        ("cuíca = a - g -", &[CuicaHigh, CuicaLow]),
        (
            "cajón = b t b t",
            &[CajonBass, CajonSlap, CajonBass, CajonSlap],
        ),
    ];
    for (src, want) in cases {
        assert_eq!(drums(src), want, "{src}");
    }
    // Names in Spanish and English, inside a block, accents in upper case.
    assert_eq!(drums("pandero = p - - -"), [PandeiroThumb]);
    assert_eq!(drums("triangle = o - - -"), [TriangleOpen]);
    let e = events(
        "percussão {\n  pandeiro = P d t d\n  agogô = a g a g\n}",
        1,
        2,
    );
    assert!(e[0].accent);
    assert_eq!(e.len(), 8);
    // A letter the instrument does not play says which it does.
    assert_eq!(
        first("pandeiro = p x t d"),
        Message::UnknownStroke {
            ch: 'x',
            instrument: aratu::ast::Perc::Pandeiro
        }
    );
    assert!(
        compile("pandeiro = p x t d").diagnostics[0]
            .text(Lang::Pt)
            .contains("p polegar, d dedos, t tapa, m abafado")
    );
}

#[test]
fn effects_and_numbers_for_the_whole_piece() {
    // Effects written for all go on every part, after its own.
    let src = "bass: synth.saw -> lowpass\nbass = | c2 _ _ _ |\nhh = c c c c\nall: chorus -> delay";
    let c = compile(src);
    assert!(c.diagnostics.is_empty(), "{:?}", c.diagnostics);
    let chain = |name: &str| {
        c.piece
            .parts
            .iter()
            .find(|p| p.name == name)
            .unwrap()
            .chain
            .clone()
    };
    assert_eq!(chain("bass"), ["synth.saw", "lowpass", "chorus", "delay"]);
    assert_eq!(chain("drums"), ["chorus", "delay"]);
    // In Portuguese too; a number with no part is for the whole piece, and
    // Manguebyte's master bus has its own.
    let c = compile(
        "baixo = | c2 _ _ _ |\ntodos: chorus\nreverb.mix = 0.5\nmaster.glue.on = 1\nmaster.reverb.decay = 3 s",
    );
    assert!(c.diagnostics.is_empty(), "{:?}", c.diagnostics);
    assert_eq!(c.piece.parts[0].chain, ["chorus"]);
    let ids: Vec<(&str, f64)> = c
        .piece
        .params
        .iter()
        .map(|p| (p.id.as_str(), p.value))
        .collect();
    assert_eq!(
        ids,
        [
            ("reverb.mix", 0.5),
            ("master.glue.on", 1.0),
            ("master.reverb.decay", 3.0)
        ]
    );
    // The new effects are known, and their numbers checked.
    let c = compile(
        "bass: synth.saw -> eq -> compressor -> phaser\nbass = | c2 _ _ _ |\nbass.eq.low_gain = 30 dB",
    );
    assert!(
        c.diagnostics
            .iter()
            .any(|d| matches!(d.message, Message::ParamOutOfRange { .. }))
    );
    // A delay's time may be written in milliseconds or seconds.
    let c = compile("bass = | c2 _ _ _ |\nbass.delay.time = 375 ms");
    assert!((c.piece.parts[0].params[0].value - 0.375).abs() < 1e-9);
}

#[test]
fn out_names_the_part_when_a_line_is_written() {
    let c = compile("hh = c c c c\nbd = b - b -\nhh: out");
    let texts: Vec<String> = c.diagnostics.iter().map(|d| d.text(Lang::Pt)).collect();
    assert!(
        texts
            .iter()
            .any(|t| t.starts_with("`hh` é uma linha da parte `drums`")),
        "{texts:?}"
    );
}

#[test]
fn a_part_bends_the_same_whatever_it_is_called() {
    let times = |src: &str| -> Vec<f64> {
        let c = compile(src);
        render(&c.piece, &Options::default(), 1, 4)
            .events
            .iter()
            .map(|e| e.time)
            .collect()
    };
    let a = times("melodia = | e4 d c d |\nmelodia: rubato 20%");
    let b = times("melody = | e4 d c d |\nmelody: rubato 20%");
    assert_eq!(a, b);
    // And `all: rubato` bends every part together.
    let c = compile("m = | c4 d e f |\nbass = | c2 d e f |\nall: rubato 20%");
    let r = render(&c.piece, &Options::default(), 1, 3);
    for e in r.events.iter().filter(|e| e.part == "m") {
        assert!(
            r.events
                .iter()
                .any(|f| f.part == "bass" && (f.time - e.time).abs() < 1e-9)
        );
    }
}

#[test]
fn a_part_that_plays_only_in_sections_is_set_up_from_outside() {
    // Numbers and a chain written outside, for a part written only inside
    // sections: it has them wherever it plays.
    let src = "section baque { alfaia = O -o o_oo o; caixa = Ssss Ssss Ssss Ssss }\nsection virada { alfaia = OO OO OO OO }\nform: baque x2 virada\nalfaia.volume = 1\nalfaia: reverb\nat bar 3: alfaia.volume = 0.5";
    let c = compile(src);
    assert!(c.diagnostics.is_empty(), "{:?}", c.diagnostics);
    assert!(c.piece.parts.is_empty(), "no part of its own outside");
    let alfaias: Vec<_> = c.piece.all_parts().filter(|p| p.name == "alfaia").collect();
    assert_eq!(alfaias.len(), 2);
    for a in alfaias {
        let set: Vec<(&str, f64)> = a.params.iter().map(|p| (p.id.as_str(), p.value)).collect();
        assert_eq!(set, [("volume", 1.0)]);
        assert_eq!(a.chain, ["reverb"]);
    }
    // At a bar, and moving, it reaches the part.
    let changes = |src: &str, from: i64, to: i64| -> Vec<(i64, String, f64)> {
        events(src, from, to)
            .into_iter()
            .filter_map(|e| match e.kind {
                EventKind::Param { value, .. } => Some((e.bar, e.part, value)),
                _ => None,
            })
            .collect()
    };
    assert_eq!(changes(src, 1, 5), [(3, "alfaia".to_string(), 0.5)]);
    let moving = changes(
        "section baque { alfaia = O -o o_oo o }\nform: baque\nalfaia.volume = 0 -> 1 over 2 bars",
        1,
        3,
    );
    assert!(moving.len() > 2 && moving.iter().all(|x| x.1 == "alfaia"));
    assert!(moving.last().unwrap().2 > 0.9, "{:?}", moving.last());
    // Written as a block, and for a pattern's drums by another name.
    let c =
        compile("pattern g { hh = c c c c }\nplay g\ndrums { volume = 0.5 }\nbateria.pan = -0.5");
    assert!(c.diagnostics.is_empty(), "{:?}", c.diagnostics);
    let drums = &c.piece.patterns[0].parts[0];
    let set: Vec<(&str, f64)> = drums
        .params
        .iter()
        .map(|p| (p.id.as_str(), p.value))
        .collect();
    assert_eq!(set, [("volume", 0.5), ("pan", -0.5)]);
    // A chain written inside stays.
    let c = compile("section a { baixo = | c2 _ _ _ |; baixo: synth.saw }\nform: a\nbaixo: reverb");
    assert!(c.diagnostics.is_empty(), "{:?}", c.diagnostics);
    assert_eq!(c.piece.sections[0].parts[0].chain, ["synth.saw"]);
    // An exercise hears it, once, though the part plays in two sections.
    let without = compile(&src.replace("alfaia.volume = 1\n", ""));
    let opts = aratu::exercise::CheckOptions {
        sound: true,
        ..Default::default()
    };
    let problems = aratu::exercise::check(&without.piece, &compile(src).piece, opts);
    let volume: Vec<_> = problems
        .iter()
        .filter(|p| matches!(p, aratu::exercise::Problem::Number { part: Some(a), id, .. } if a == "alfaia" && id == "volume"))
        .collect();
    assert_eq!(volume.len(), 1, "{problems:?}");
}

/// The notes of bars `from` to `to`, as (time, pitch, velocity).
fn timed(src: &str, from: i64, to: i64) -> Vec<(f64, u8, u8)> {
    events(src, from, to)
        .iter()
        .filter_map(|e| match e.kind {
            EventKind::Note(n) => Some((e.time, n, e.velocity)),
            _ => None,
        })
        .collect()
}

fn pitches(src: &str) -> Vec<u8> {
    timed(src, 1, 2).iter().map(|x| x.1).collect()
}

#[test]
fn a_trill_turns_with_the_note_above_in_the_key() {
    // Eighths of a beat over four beats: an odd number of strokes, so it
    // starts and ends on the note.
    let p = pitches("key: C major\nm = | c5:tr _ _ _ |");
    assert_eq!(p.len(), 31);
    assert!(p.iter().step_by(2).all(|&n| n == 72));
    assert!(p.iter().skip(1).step_by(2).all(|&n| n == 74));
    // The note above in the key: a half step from B in C major, a whole
    // step from B in A major.
    assert_eq!(pitches("key: C major\nm = | b4:tr _ _ _ |")[1], 72);
    assert_eq!(pitches("key: A major\nm = | b4:tr _ _ _ |")[1], 73);
    // `#` and `b` move the note above, as over a score's tr.
    assert_eq!(pitches("m = | e5:tr# _ _ _ |")[1], 78);
    assert_eq!(pitches("m = | d5:trb _ _ _ |")[1], 75);
    // In Portuguese and Spanish too, and the key changes the trill.
    assert_eq!(pitches("m = | dó5:trinado _ _ _ |")[1], 74);
    assert_eq!(pitches("m = | do5:trino _ _ _ |")[1], 74);
    // A note with its octave carried from the one before.
    assert_eq!(pitches("m = | c5 d:tr _ _ |")[2], 76);
    let n = notes("m = | b4:tr _ _ _ |\nat bar 2: key A major", 1, 3);
    assert!(n.iter().any(|x| x.0 == 1 && x.1 == 72));
    assert!(n.iter().any(|x| x.0 == 2 && x.1 == 73));
}

#[test]
fn a_mordent_is_the_note_its_neighbour_and_the_note() {
    let t = timed("tempo: 60\nm = | c5:mord _ _ _ |", 1, 2);
    let p: Vec<u8> = t.iter().map(|x| x.1).collect();
    assert_eq!(p, [72, 74, 72]);
    // An eighth of a beat each, then the note to the end of its length.
    assert_eq!(t[1].0, 0.125);
    assert_eq!(t[2].0, 0.25);
    assert_eq!(pitches("m = | c5:mordi _ _ _ |"), [72, 71, 72]);
    // On a short note, the three share it.
    let t = timed("tempo: 60\nm = | [c5:mord d5 e5 f5] _ _ _ |", 1, 2);
    assert_eq!(t[1].0, 0.25 / 3.0);
}

#[test]
fn grace_notes_come_just_before_and_take_no_beat() {
    let t = timed("tempo: 60\nm = | c5 `d5 c5 b4 c5 |", 1, 2);
    let p: Vec<u8> = t.iter().map(|x| x.1).collect();
    assert_eq!(p, [72, 74, 72, 71, 72]);
    // The grace note is a little before the beat, and softer.
    assert!((t[1].0 - 0.94).abs() < 1e-9, "{t:?}");
    assert_eq!(t[2].0, 1.0);
    assert!(t[1].2 < t[2].2);
    // Several, glued or not, inside brackets, and over the bar line.
    assert_eq!(pitches("m = | `b4 `c5 d5 _ _ _ |"), [71, 72, 74]);
    assert_eq!(pitches("m = | `b4`c5 d5 _ _ _ |"), [71, 72, 74]);
    assert_eq!(pitches("m = | [c5 `e5 d5] _ _ _ |"), [72, 76, 74]);
    let n = notes("m = | c5 _ _ _ `b4 | c5 _ _ _ |", 1, 3);
    assert_eq!(n, [(1, 72), (2, 71), (2, 72)]);
    // A motif moved by degrees takes its grace notes along; backwards,
    // each keeps its distance to its note.
    let n = notes(
        "key: C major\nmotif m = [`d5 c5 e5]\nmelodia = | m m+1 m.retro _ |",
        1,
        2,
    );
    let p: Vec<u8> = n.iter().map(|x| x.1).collect();
    assert_eq!(p, [74, 72, 76, 76, 74, 77, 78, 76, 72]);
}

#[test]
fn a_flam_and_a_drag_are_grace_strokes_of_the_same_drum() {
    let e = events("tempo: 60\nsd = `s - ``S -", 1, 2);
    let snares: Vec<(f64, u8)> = e
        .iter()
        .filter(|e| matches!(e.kind, EventKind::Drum(Drum::Snare)))
        .map(|e| (e.time, e.velocity))
        .collect();
    assert_eq!(snares.len(), 5);
    assert!((snares[0].0 + 0.03).abs() < 1e-9 && snares[1].0 == 0.0);
    assert!(snares[0].1 < snares[1].1);
    assert!((snares[2].0 - 1.94).abs() < 1e-9 && (snares[3].0 - 1.97).abs() < 1e-9);
    assert_eq!(snares[4].0, 2.0);
    // On the Brazilian drums too.
    assert_eq!(events("alfaia = `O - - -", 1, 2).len(), 2);
    // An exercise tells a flam from a single stroke, and says so.
    let problems = aratu::exercise::check(
        &compile("sd = - s - s").piece,
        &compile("sd = - `s - s").piece,
        aratu::exercise::CheckOptions::default(),
    );
    assert_eq!(problems.len(), 1);
    assert_eq!(
        problems[0].text(Lang::Pt),
        "No compasso 1, tempo 2: faltou uma nota de graça logo antes (a caixa)."
    );
}

#[test]
fn ornaments_and_grace_notes_written_wrong_are_told() {
    assert_eq!(first("m = | c5 _ _ _ `d5 |"), Message::GraceBeforeNothing);
    assert_eq!(first("m = | `d5 _ _ _ |"), Message::GraceBeforeNothing);
    assert_eq!(
        first("m = | c5:trinadoo _ _ _ |"),
        Message::UnknownTechnique {
            word: "trinadoo".into(),
            closest: Some("trinado".into())
        }
    );
    assert!(matches!(
        first("sd = s:tr - - -"),
        Message::OrnamentNeedsNote { .. }
    ));
    assert!(
        compile("m = | c5:xyz _ _ _ |").diagnostics[0]
            .text(Lang::Pt)
            .contains("`:tr` trinado")
    );
}

#[test]
fn a_statement_that_stops_at_the_equals_sign_says_what_is_missing() {
    // A parameter: its number, with one that fits.
    let volume = Message::MissingNumber {
        name: "volume".into(),
        example: Some("0.8".into()),
        every_part: true,
    };
    let c = compile("bass = | c2 _ _ _ |\nvolume =");
    assert!(c.has_errors());
    assert_eq!(c.diagnostics.len(), 1);
    assert_eq!(c.diagnostics[0].message, volume);
    // The editor marks the whole statement.
    assert_eq!(c.diagnostics[0].span, Span::new(20, 28));
    assert_eq!(first("volume = "), volume);
    assert_eq!(first("at bar 2: volume ="), volume);
    assert_eq!(
        volume.text(Lang::Pt),
        "falta o número depois de `volume =`; por exemplo `volume = 0.8` (sem nome de parte, vale para todas as partes)"
    );
    assert_eq!(
        volume.text(Lang::Es),
        "falta el número después de `volume =`; por ejemplo `volume = 0.8` (sin nombre de parte, vale para todas las partes)"
    );
    assert_eq!(
        volume.text(Lang::En),
        "a number is missing after `volume =`; for example `volume = 0.8` (with no part name, it holds for every part)"
    );
    assert_eq!(
        first("lowpass.cutoff ="),
        Message::MissingNumber {
            name: "lowpass.cutoff".into(),
            example: Some("2000 Hz".into()),
            every_part: true,
        }
    );
    // A part named, or the part's own block: the number is for that part.
    let part = Message::MissingNumber {
        name: "bass.reverb.mix".into(),
        example: Some("0.3".into()),
        every_part: false,
    };
    assert_eq!(first("bass.reverb.mix ="), part);
    assert_eq!(
        part.text(Lang::Pt),
        "falta o número depois de `bass.reverb.mix =`; por exemplo `bass.reverb.mix = 0.3`"
    );
    assert_eq!(
        first("bass {\n  volume =\n}"),
        Message::MissingNumber {
            name: "volume".into(),
            example: Some("0.8".into()),
            every_part: false,
        }
    );

    // A line: its music.
    let line = Message::EmptyLine {
        name: "bass".into(),
    };
    assert_eq!(first("bass ="), line);
    assert_eq!(
        line.text(Lang::Pt),
        "a linha `bass` está vazia: escreva a música depois do `=`"
    );
    assert_eq!(
        line.text(Lang::Es),
        "la línea `bass` está vacía: escribe la música después del `=`"
    );
    assert_eq!(
        line.text(Lang::En),
        "the line `bass` is empty: write its music after the `=`"
    );
    assert_eq!(
        first("pattern groove {\n  hh =\n}\nplay groove"),
        Message::EmptyLine { name: "hh".into() }
    );
    assert_eq!(
        first("section verse { bass = }"),
        Message::EmptyLine {
            name: "bass".into()
        }
    );

    // A motif or a row: its notes.
    assert_eq!(
        first("motif riff ="),
        Message::EmptyDefinition {
            name: "riff".into(),
            row: false,
        }
    );
    assert_eq!(
        first("row serie ="),
        Message::EmptyDefinition {
            name: "serie".into(),
            row: true,
        }
    );
}

#[test]
fn a_line_may_start_its_bars_below_the_equals_sign() {
    let src = "bass =\n  | c2 _ _ _ |\n  | d2 _ _ _ |";
    assert!(compile(src).diagnostics.is_empty());
    assert_eq!(notes(src, 1, 3), [(1, 36), (2, 38)]);
    // Bars below bars, as before.
    assert_eq!(
        notes("bass = | c2 _ _ _ | // one\n       | d2 _ _ _ |", 1, 3),
        [(1, 36), (2, 38)]
    );
    // In a pattern's block.
    assert_eq!(
        notes("pattern p {\n  bass =\n    | c2 _ _ _ |\n}\nplay p", 1, 2),
        [(1, 36)]
    );
    // In a list each line is a melody of its own.
    let list = "list temas {\n  | c4 _ _ _ |\n  | d4 _ _ _ |\n}\n";
    assert_eq!(notes(&format!("{list}melody = temas.2"), 1, 2), [(1, 62)]);
}

/// The sounds of a piece from bar 1 to bar `to` (exclusive), by part.
fn by_part(src: &str, to: i64) -> std::collections::BTreeMap<String, Vec<Event>> {
    let c = compile(src);
    let problems: Vec<String> = c.diagnostics.iter().map(|d| d.text(Lang::En)).collect();
    assert!(problems.is_empty(), "{problems:?}");
    let mut out: std::collections::BTreeMap<String, Vec<Event>> = Default::default();
    for e in render(&c.piece, &Options::default(), 1, to).events {
        out.entry(e.part.clone()).or_default().push(e);
    }
    out
}

const BAND: &str =
    "drums { bd = b b b b; hh = cc cc cc cc }\nbass = | c2 _ _ _ |\npad = | c4+e4+g4 _ _ _ |\n";

#[test]
fn a_breakdown_takes_the_drums_and_the_bass_out_and_a_drop_brings_them_back() {
    let parts = by_part(&format!("{BAND}at bar 3: breakdown\nat bar 5: drop"), 7);
    let bars = |p: &str| -> Vec<i64> {
        let mut b: Vec<i64> = parts[p]
            .iter()
            .filter(|e| !matches!(e.kind, EventKind::Param { .. }))
            .map(|e| e.bar)
            .collect();
        b.dedup();
        b
    };
    assert_eq!(bars("bass"), [1, 2, 5, 6]);
    assert_eq!(bars("pad"), [1, 2, 3, 4, 5, 6]);
    assert!(bars("drums").iter().all(|b| ![3, 4].contains(b)));
    // The drop comes in on a crash.
    assert!(parts["drums"].iter().any(|e| e.bar == 5
        && e.beat == Ratio::ZERO
        && matches!(e.kind, EventKind::Drum(Drum::Crash))));
    // In Portuguese, for a stretch: back on its own afterwards.
    let parts = by_part(&format!("{BAND}do compasso 2 ao 3: quebra"), 5);
    let bass: Vec<i64> = parts["bass"].iter().map(|e| e.bar).collect();
    assert_eq!(bass, [1, 4]);
}

#[test]
fn a_build_rolls_faster_and_louder_and_opens_a_highpass_to_its_end() {
    let parts = by_part(&format!("{BAND}at bar 2: build over 4 bars"), 7);
    let rolls: Vec<&Event> = parts["drums"]
        .iter()
        .filter(|e| e.line == "build")
        .collect();
    let per_bar = |b: i64| rolls.iter().filter(|e| e.bar == b).count();
    assert_eq!(
        [per_bar(2), per_bar(3), per_bar(4), per_bar(5)],
        [4, 8, 16, 32]
    );
    assert_eq!(per_bar(6), 0);
    assert!(rolls.first().unwrap().velocity < rolls.last().unwrap().velocity);
    let cutoff: Vec<(i64, f64, bool)> = parts["all"]
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::Param {
                id, value, reset, ..
            } if id == "highpass.cutoff" => Some((e.bar, *value, *reset)),
            _ => None,
        })
        .collect();
    assert!(
        cutoff.windows(2).take(15).all(|w| w[1].1 > w[0].1),
        "{cutoff:?}"
    );
    assert_eq!(cutoff.last().map(|c| (c.0, c.2)), Some((6, true)));
    // `subida em 2 compassos`, and four bars when no length is said.
    assert!(
        compile(&format!("{BAND}no compasso 2: subida em 2 compassos"))
            .diagnostics
            .is_empty()
    );
    assert!(compile(&format!("{BAND}now: build")).diagnostics.is_empty());
}

#[test]
fn a_solo_leaves_one_part_and_solo_off_brings_the_others_back() {
    let parts = by_part(&format!("{BAND}at bar 2: solo bass\nat bar 3: solo off"), 4);
    let bars = |p: &str| -> Vec<i64> {
        let mut b: Vec<i64> = parts[p].iter().map(|e| e.bar).collect();
        b.dedup();
        b
    };
    assert_eq!(bars("bass"), [1, 2, 3]);
    assert_eq!(bars("pad"), [1, 3]);
    assert_eq!(bars("drums"), [1, 3]);
    // The drum part answers to its name in any language.
    let parts = by_part(&format!("{BAND}solo bateria"), 2);
    assert!(parts.contains_key("drums") && !parts.contains_key("pad"));
}

#[test]
fn a_groove_writes_its_lines_and_a_line_written_over_it_wins() {
    let c = compile("groove house");
    assert!(c.diagnostics.is_empty(), "{:?}", c.diagnostics);
    let drums = &c.piece.parts[0];
    let names: Vec<&str> = drums.lines.iter().map(|l| l.name.as_str()).collect();
    assert_eq!(names, ["bd", "oh", "clap", "hh"]);
    // Its sounds light the word that wrote it.
    let r = render(&c.piece, &Options::default(), 1, 2);
    assert!(
        r.events
            .iter()
            .all(|e| e.span.start == 0 && e.span.end == "groove house".len())
    );
    // The hi-hat written here takes the groove's place, in either order.
    for src in ["levada house\nhh = c c c c", "hh = c c c c\nlevada house"] {
        let c = compile(src);
        assert!(c.diagnostics.is_empty(), "{src}: {:?}", c.diagnostics);
        let hh = c.piece.parts[0]
            .lines
            .iter()
            .find(|l| l.name == "hh")
            .unwrap();
        assert!(hh.lit.is_none(), "{src}");
    }
    // Percussion grooves write the instruments' own lines.
    let c = compile("ritmo samba");
    let parts: Vec<&str> = c.piece.parts.iter().map(|p| p.name.as_str()).collect();
    assert_eq!(parts, ["surdo", "tamborim", "pandeiro", "ganzá"]);
    // Unknown names and other meters are told.
    let texts = |src: &str| -> Vec<String> {
        compile(src)
            .diagnostics
            .iter()
            .map(|d| d.text(Lang::Pt))
            .collect()
    };
    assert!(
        texts("levada hous")[0].contains("seria `house`?"),
        "{:?}",
        texts("levada hous")
    );
    assert!(texts("meter: 3/4\nlevada house")[0].starts_with("as levadas são em 4/4"));
    // `drop` and `build` may still name sections.
    assert!(
        compile("section drop { bass = | c2 _ _ _ | }\nform: drop")
            .diagnostics
            .is_empty()
    );
}

/// Where each drum starts in bar 1, in beats.
fn starts(src: &str) -> Vec<f64> {
    hits(src, 1, 2)
        .iter()
        .map(|(_, beat, _)| beat.to_f64())
        .collect()
}

#[test]
fn a_euclidean_rhythm_spreads_its_hits_over_the_bar() {
    // Alone in its bar it fills the bar, as a rest alone does: eight
    // eighths in 4/4, the tresillo.
    assert_eq!(starts("bd = b(3,8)"), [0.0, 1.5, 3.0]);
    assert_eq!(starts("bd = | b(3,8) |"), [0.0, 1.5, 3.0]);
    // The cinquillo, and the same turned two steps later.
    assert_eq!(starts("bd = b(5,8)"), [0.0, 1.0, 1.5, 2.5, 3.0]);
    assert_eq!(starts("bd = b(3,8,2)"), [0.0, 1.0, 2.5]);
    assert_eq!(starts("bd = b(3,8,-1)"), [1.0, 2.5, 3.5]);
    // Spaces inside are allowed.
    assert_eq!(starts("bd = b(3, 8)"), starts("bd = b(3,8)"));
    // In any meter: the steps share whatever the bar has.
    assert_eq!(starts("meter: 3/4\nbd = b(3,6)"), [0.0, 1.0, 2.0]);
    assert_eq!(starts("meter: 6/8\nrd = r(7,12)").len(), 7);
    assert_eq!(starts("meter: 7/8 = 2+2+3\nbd = b(3,7)").len(), 3);
    // Beside other beats it is one beat, divided in its steps; with a
    // number before brackets, it spans that many beats.
    assert_eq!(starts("bd = | b(2,4) - - - |"), [0.0, 0.5]);
    assert_eq!(
        starts("bd = | 2[b(3,8)] 2[b(3,8)] |"),
        [0.0, 0.75, 1.5, 2.0, 2.75, 3.5]
    );
    // Upper case accents every hit; a repeat count after the parenthesis.
    let e = events("sd = S(2,4)", 1, 2);
    assert!(e.iter().all(|e| e.accent) && e.len() == 2);
    assert_eq!(starts("bd = | b(1,2)x4 |"), [0.0, 1.0, 2.0, 3.0]);
    // On percussion, a strum, and a note.
    assert_eq!(hits("alfaia = o(3,8)", 1, 2)[0].2, Drum::AlfaiaOpen);
    assert_eq!(notes("bass = c2(3,8)", 1, 2), [(1, 36), (1, 36), (1, 36)]);
    let bass = events("bass = c2(3,8)", 1, 2);
    assert!((bass[0].duration - 0.25).abs() < 1e-9, "an eighth at 120");
    // The two lines of a groove line up.
    let src = "drums {\n  bd = b(4,16)\n  hh = c(3,8,1)\n}";
    assert_eq!(hits(src, 1, 2).len(), 7);
    // Each hit draws its own chance: never all or none, bar after bar.
    let counts: Vec<usize> = (1..9)
        .map(|b| hits("hh = c(8,8)?", b, b + 1).len())
        .collect();
    assert!(counts.iter().any(|n| (1..8).contains(n)), "{counts:?}");
}

#[test]
fn a_euclidean_rhythm_written_wrong_is_told() {
    assert_eq!(
        first("bd = b(9,8)"),
        Message::BadEuclid { hits: 9, steps: 8 }
    );
    assert_eq!(
        first("bd = b(1,0)"),
        Message::BadEuclid { hits: 1, steps: 0 }
    );
    assert!(matches!(first("bd = b(3)"), Message::BadEuclidForm { .. }));
    assert!(matches!(
        first("bd = b(3,x)"),
        Message::BadEuclidForm { .. }
    ));
    assert!(matches!(first("bd = b(3,8"), Message::BadEuclidForm { .. }));
    let m = Message::BadEuclid { hits: 9, steps: 8 };
    assert_eq!(
        m.text(Lang::Pt),
        "9 toques não cabem em 8 passos: o primeiro número são os toques, o segundo os passos"
    );
    assert!(m.text(Lang::Es).starts_with("9 golpes no caben en 8 pasos"));
    assert!(m.text(Lang::En).starts_with("9 hits do not fit in 8 steps"));
    // A ghost note is still a ghost note.
    assert_eq!(starts("sd = (s) s (s) s").len(), 4);
}

/// The notes of bars `from` to `to`, as (bar, beat, pitch).
fn placed(src: &str, from: i64, to: i64) -> Vec<(i64, f64, u8)> {
    events(src, from, to)
        .iter()
        .filter_map(|e| match e.kind {
            EventKind::Note(n) => Some((e.bar, e.beat.to_f64(), n)),
            _ => None,
        })
        .collect()
}

#[test]
fn an_arpeggiator_plays_the_chords_in_steps() {
    let src = "chords = | Am | F |\narp = up 16ths";
    let bar1: Vec<u8> = placed(src, 1, 2).iter().map(|n| n.2).collect();
    // A3 C4 E4 again and again, sixteen sixteenths; the chord itself does
    // not sound, as under a strum.
    assert_eq!(bar1.len(), 16);
    assert_eq!(&bar1[..7], [57, 60, 64, 57, 60, 64, 57]);
    let bar2: Vec<u8> = placed(src, 2, 3).iter().map(|n| n.2).collect();
    assert_eq!(&bar2[..3], [53, 57, 60]);
    let way = |w: &str| -> Vec<u8> {
        placed(&format!("chords = | Am |\narp = {w}"), 1, 2)
            .iter()
            .map(|n| n.2)
            .collect()
    };
    assert_eq!(way("down 8ths"), [64, 60, 57, 64, 60, 57, 64, 60]);
    assert_eq!(way("updown 8ths"), [57, 60, 64, 60, 57, 60, 64, 60]);
    assert_eq!(way("up and down 8ths"), way("updown 8ths"));
    assert_eq!(way("up 8ths 2 octaves"), [57, 60, 64, 69, 72, 76, 57, 60]);
    assert_eq!(way("up triplets").len(), 12);
    assert_eq!(way("up 32nds").len(), 32);
    assert_eq!(way("up beats"), [57, 60, 64, 57]);
    assert_eq!(way("up").len(), 16, "sixteenths when not said");
    assert!(way("random 16ths").iter().all(|n| [57, 60, 64].contains(n)));
    // In Portuguese and Spanish.
    assert_eq!(
        placed("acordes = | Am |\narpejo = sobe e desce 8ths", 1, 2),
        placed("chords = | Am |\narp = updown 8ths", 1, 2)
    );
    assert_eq!(
        placed("acordes = | Am |\narpegio = baja corcheas", 1, 2),
        placed("chords = | Am |\narp = down eighths", 1, 2)
    );
    // In 6/8 an eighth is a third of a beat: six a bar.
    assert_eq!(
        placed("meter: 6/8\nchords = | Am |\narp = up 8ths", 1, 2).len(),
        6
    );
    // Each chord starts from its root.
    let two: Vec<u8> = placed("chords = | C G |\narp = up 8ths", 1, 2)
        .iter()
        .map(|n| n.2)
        .collect();
    assert_eq!(two, [48, 52, 55, 48, 55, 59, 62, 55]);
    // In a part, with the part's transformations.
    let piano = "piano {\n  chords = | Am |\n  arp = up 8ths\n}\npiano.transpose +12";
    assert_eq!(placed(piano, 1, 2)[0].2, 69);
    // A line called `arp` with notes is a melody, as before.
    assert_eq!(
        notes("arp = | c4 e g c5 |", 1, 2),
        [(1, 60), (1, 64), (1, 67), (1, 72)]
    );
}

#[test]
fn an_arpeggiator_written_wrong_is_told() {
    assert_eq!(
        first("piano { arp = up 16ths }"),
        Message::ArpWithoutChords {
            part: "piano".into()
        }
    );
    assert!(matches!(
        first("chords = | Am |\narp = up 12ths"),
        Message::Expected {
            what: aratu::diag::Thing::ArpStep,
            ..
        }
    ));
    assert_eq!(
        first("chords = | Am |\narp = up 9 octaves"),
        Message::ArpOctaves { count: 9 }
    );
    assert!(
        Message::ArpWithoutChords {
            part: "piano".into()
        }
        .text(Lang::Pt)
        .starts_with("o arpejador de `piano` não tem acordes")
    );
}

/// The volume of `part` set while bar 1 plays, as (beat, value).
fn volume(src: &str, part: &str) -> Vec<(f64, f64)> {
    events(src, 1, 2)
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::Param { id, value, .. } if id == "volume" && e.part == part => {
                Some((e.beat.to_f64(), *value))
            }
            _ => None,
        })
        .collect()
}

fn near(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-6
}

#[test]
fn a_part_ducks_under_the_kick() {
    let base = "drums { bd = b - b -; hh = cc cc cc cc }\npad = | c4 _ _ _ |\n";
    let v = volume(&format!("{base}pad: duck by kick"), "pad");
    // Down by 70% at each kick, back over a beat, from the written 0.8.
    assert!(near(v[0].0, 0.0) && near(v[0].1, 0.8 * 0.3), "{v:?}");
    assert!(v.iter().any(|(b, x)| near(*b, 1.0) && near(*x, 0.8)));
    assert!(v.iter().any(|(b, x)| near(*b, 2.0) && near(*x, 0.24)));
    // Halfway back it has come most of the way: the curve eases in.
    let half = v.iter().find(|(b, _)| near(*b, 0.5)).unwrap().1;
    assert!(half > 0.24 + (0.8 - 0.24) / 2.0, "{half}");
    // Never below the dip, never above the volume.
    assert!(v.iter().all(|(_, x)| *x >= 0.24 - 1e-9 && *x <= 0.8 + 1e-9));
    // How much and how fast, and the volume the part is set to.
    let v = volume(
        &format!("{base}pad.volume = 0.5\npad: duck by bd 50% over 1 eighth"),
        "pad",
    );
    assert!(near(v[0].1, 0.25), "{v:?}");
    assert!(v.iter().any(|(b, x)| near(*b, 0.5) && near(*x, 0.5)));
    // The line's name, the part's, and only the kicks: not the hi-hat.
    assert_eq!(
        volume(&format!("{base}pad: duck by bd"), "pad"),
        volume(&format!("{base}pad: duck by kick"), "pad")
    );
    let all = volume(&format!("{base}pad: duck by drums"), "pad");
    assert!(all.iter().any(|(b, x)| near(*b, 0.5) && near(*x, 0.24)));
    // In Portuguese and Spanish.
    let en = volume(&format!("{base}pad: duck by kick 50%"), "pad");
    assert_eq!(
        volume(&format!("{base}pad: abaixa com o bumbo 50%"), "pad"),
        en
    );
    assert_eq!(
        volume(&format!("{base}pad: agacha con el bombo 50%"), "pad"),
        en
    );
    assert_eq!(
        volume(&format!("{base}pad: sidechain by kick 50%"), "pad"),
        en
    );
    // Only where it is in force.
    let later = format!("{base}from bar 2 to 3: pad: duck by kick");
    assert!(volume(&later, "pad").is_empty());
    assert!(!params(&later, 2, 3).is_empty());
    // The drums themselves do not move.
    assert!(volume(&format!("{base}pad: duck by kick"), "drums").is_empty());
    // `all:` ducks every other part; `every` only in its bars.
    let both = format!("{base}lead = | e5 _ _ _ |\nall: duck by kick");
    assert_eq!(volume(&both, "pad"), volume(&both, "lead"));
    assert!(!volume(&both, "pad").is_empty());
    assert!(volume(&both, "drums").is_empty());
    let every = format!("{base}every 2 bars: pad: duck by kick");
    assert!(volume(&every, "pad").is_empty());
    assert!(!params(&every, 2, 3).is_empty());
    // A drum by its name in Portuguese, from a line of another name.
    assert_eq!(
        volume(&format!("{base}pad: duck by caixa"), "pad"),
        volume(&format!("{base}pad: duck by snare"), "pad")
    );
}

#[test]
fn ducking_written_wrong_is_told() {
    let base = "drums { bd = b - b - }\npad = | c4 _ _ _ |\n";
    assert_eq!(
        first(&format!("{base}pad: duck by kick 150%")),
        Message::DuckAmount { amount: 1.5 }
    );
    assert!(matches!(
        first(&format!("{base}pad: duck kick")),
        Message::Expected {
            what: aratu::diag::Thing::DuckSource,
            ..
        }
    ));
    assert_eq!(
        first(&format!("{base}pad: duck by tuba")),
        Message::UnknownTarget {
            name: "tuba".into()
        }
    );
}

#[test]
fn fast_and_slow_play_a_line_in_more_or_fewer_bars() {
    let hh = "hh = | c - - - | o - - - |\n";
    let fast = hits(&format!("{hh}hh.fast 2"), 1, 2);
    assert_eq!(
        fast,
        [
            (1, Ratio::ZERO, Drum::ClosedHat),
            (1, Ratio::int(2), Drum::OpenHat)
        ]
    );
    assert_eq!(hits(&format!("{hh}hh.fast 3"), 1, 2).len(), 3);
    // `diminish` is `fast 2`, and the two together are `fast 4`.
    assert_eq!(
        hits(&format!("{hh}hh.diminish"), 1, 3),
        hits(&format!("{hh}hh.fast 2"), 1, 3)
    );
    assert_eq!(hits(&format!("{hh}hh.fast 2\nhh.diminish"), 1, 2).len(), 4);
    // Slow: each bar over two, held across the bar line.
    let bass = "bass = | c2 _ _ _ | g1 _ _ _ |\n";
    assert_eq!(
        notes(&format!("{bass}bass.slow 2"), 1, 5),
        [(1, 36), (3, 31)]
    );
    let held = events(&format!("{bass}bass.slow 2"), 1, 3);
    assert!(
        (held[0].duration - 4.0).abs() < 1e-6,
        "{}",
        held[0].duration
    );
    assert_eq!(
        notes(&format!("{bass}bass.slow 2"), 1, 5),
        notes(&format!("{bass}bass.augment"), 1, 5)
    );
    // They cancel, and they take a whole part.
    assert_eq!(
        hits(&format!("{hh}hh.fast 2\nhh.slow 2"), 1, 3),
        hits(hh, 1, 3)
    );
    let drums = "drums { bd = b - - -; sd = - - s - }\ndrums.fast 2";
    assert_eq!(hits(drums, 1, 2).len(), 4);
    // In Portuguese and Spanish.
    assert_eq!(
        hits(&format!("{hh}hh.rápido 2"), 1, 3),
        hits(&format!("{hh}hh.fast 2"), 1, 3)
    );
    assert_eq!(
        notes(&format!("{bass}bass.lento 2"), 1, 5),
        notes(&format!("{bass}bass.slow 2"), 1, 5)
    );
    // Whole numbers only.
    assert_eq!(
        first(&format!("{hh}hh.fast 1.5")),
        Message::BadSpeed {
            verb: "fast".into(),
            times: 1.5
        }
    );
}

#[test]
fn an_echo_plays_each_sound_again_softer() {
    let lead = "lead = | c4 - - - |\n";
    let e = placed(&format!("{lead}lead.echo 1 beat"), 1, 2);
    assert_eq!(e, [(1, 0.0, 60), (1, 1.0, 60)]);
    let v: Vec<u8> = events(&format!("{lead}lead.echo 1 beat x3"), 1, 2)
        .iter()
        .map(|e| e.velocity)
        .collect();
    assert_eq!(v.len(), 4);
    assert!(v.windows(2).all(|w| w[1] < w[0]), "{v:?}");
    // In eighths, and over the bar line.
    let e = placed(&format!("{lead}lead.echo 3 eighths x2"), 1, 2);
    assert_eq!(e, [(1, 0.0, 60), (1, 1.5, 60), (1, 3.0, 60)]);
    let late = "lead = | - - - c4 |\nlead.echo 1 beat";
    // The echo of bar 1 opens bar 2, before bar 2's own note.
    assert_eq!(placed(late, 2, 3), [(2, 0.0, 60), (2, 3.0, 60)]);
    // Drums echo too, and in Portuguese and Spanish.
    assert_eq!(hits("sd = - s - -\nsd.echo 1 eighth", 1, 2).len(), 2);
    assert_eq!(
        placed(&format!("{lead}lead.eco 1 tempo"), 1, 2),
        placed(&format!("{lead}lead.echo 1 beat"), 1, 2)
    );
    assert_eq!(
        placed(&format!("{lead}lead.eco 2 corcheas"), 1, 2),
        placed(&format!("{lead}lead.echo 1 beat"), 1, 2)
    );
}

/// The notes a piece plays in bar 1, in order, chords low to high.
fn notes_of(src: &str) -> Vec<u8> {
    let c = compile(src);
    let problems: Vec<String> = c.diagnostics.iter().map(|d| d.text(Lang::En)).collect();
    assert!(problems.is_empty(), "{src}: {problems:?}");
    let mut ev: Vec<(Ratio, u8)> = render(&c.piece, &Options::default(), 1, 2)
        .events
        .iter()
        .filter_map(|e| match e.kind {
            EventKind::Note(n) => Some((e.beat, n)),
            _ => None,
        })
        .collect();
    ev.sort();
    ev.into_iter().map(|(_, n)| n).collect()
}

#[test]
fn a_melody_may_be_written_by_the_degrees_of_its_key() {
    // 1 is the tonic in octave 4; 8 is it an octave higher.
    assert_eq!(notes_of("key: C major\nm = | 1 3 5 8 |"), [60, 64, 67, 72]);
    assert_eq!(notes_of("key: A minor\nm = | 1 3 5 7 |"), [69, 72, 76, 79]);
    assert_eq!(
        notes_of("tom: Sol maior\nm = | 1 2 3 4 |"),
        [67, 69, 71, 72]
    );
    // No key: C major.
    assert_eq!(notes_of("m = | 1 _ 5 _ |"), [60, 67]);
    // Accidentals after the number, and a comma for each octave down.
    assert_eq!(
        notes_of("key: C major\nm = | 4# 7b 5, 1,, |"),
        [66, 70, 55, 36]
    );
    // Everything a note can do: groups, chords, holds, staccato, fermata,
    // repeats and tuplets.
    assert_eq!(
        notes_of("m = | [1 2 3 4] 1+3+5 _ 5'^ |"),
        [60, 62, 64, 65, 60, 64, 67, 67]
    );
    assert_eq!(notes_of("m = | [1 3]x4 |").len(), 8);
    assert_eq!(notes_of("m = | 2[1 2 3] 5 _ |"), [60, 62, 64, 67]);
    // A degree no melody reaches is told.
    let c = compile("m = | 1 0 22 1 |");
    let texts: Vec<String> = c.diagnostics.iter().map(|d| d.text(Lang::Pt)).collect();
    assert_eq!(texts.len(), 2, "{texts:?}");
    assert!(texts[0].starts_with("`0` não é um grau"), "{texts:?}");
}

#[test]
fn a_part_or_a_line_shows_its_wave_or_its_roll_under_the_line() {
    let src = "bass = | c2 c c c |\ndrums {\n  hh = cccc cccc cccc cccc\n}\nbass.wave\ndrums.hh.roll\nbaixo = | c2 c c c |\nbaixo.onda\nbajo = | c2 c c c |\nbajo.rollo\n";
    let c = compile(src);
    assert!(!c.has_errors(), "{:?}", c.diagnostics);
    let shown: Vec<(&str, &str)> = c
        .piece
        .visuals
        .iter()
        .map(|v| (v.target.as_str(), v.visual.name()))
        .collect();
    assert_eq!(
        shown,
        [
            ("bass", "wave"),
            ("drums.hh", "roll"),
            ("baixo", "wave"),
            ("bajo", "roll")
        ]
    );
    // Each is placed on its own line.
    let line = |i: usize| &src[c.piece.visuals[i].span.start..c.piece.visuals[i].span.end];
    assert_eq!(line(0), "bass.wave");
    assert_eq!(line(1), "drums.hh.roll");
    // A drawing changes nothing that sounds.
    assert_eq!(
        events(src, 1, 2).len(),
        events(&src.replace("bass.wave\n", ""), 1, 2).len()
    );
    // A name that is no part nor line is told.
    let c = compile("bass = | c2 c c c |\nbas.wave\n");
    assert!(
        c.diagnostics
            .iter()
            .any(|d| matches!(&d.message, Message::UnknownTarget { name } if name == "bas"))
    );
}

#[test]
fn a_tube_is_the_drive_stage_set_up_as_a_valve() {
    let src = "guitarra: guitar.steel -> válvula -> reverb\nguitarra = | e3 _ _ _ |\nguitarra.valvula.drive = 10\n";
    let c = compile(src);
    assert!(!c.has_errors(), "{:?}", c.diagnostics);
    assert!(c.diagnostics.is_empty(), "{:?}", c.diagnostics);
    let part = c.piece.parts.iter().find(|p| p.name == "guitarra").unwrap();
    assert_eq!(part.chain, ["guitar.steel", "tube", "reverb"]);
    assert!(
        part.params
            .iter()
            .any(|p| p.id == "tube.drive" && (p.value - 10.0).abs() < 1e-9)
    );
    // The valve and the drive are one stage.
    let c = compile("g: synth.saw -> drive -> tube\ng = | c4 _ _ _ |\n");
    assert!(
        c.diagnostics
            .iter()
            .any(|d| matches!(d.message, Message::TubeAndDrive))
    );
}
