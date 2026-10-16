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
