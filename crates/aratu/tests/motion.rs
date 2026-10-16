//! Parameters that move: `lowpass.cutoff = 400 Hz -> 4000 Hz over 4 bars`.

use aratu::compile;
use aratu::diag::{Message, Thing};
use aratu::ratio::Ratio;
use aratu::schedule::{EventKind, Options, render};

const LINE: &str = "tempo: 120\nmelodia = | c4 _ _ _ |\n";

/// The points of a moving parameter: bar, beat, value and glide.
fn points(src: &str, from: i64, to: i64, opts: &Options) -> Vec<(i64, Ratio, f64, f64)> {
    let c = compile(src);
    let errors: Vec<_> = c.errors().collect();
    assert!(errors.is_empty(), "{src}: {errors:?}");
    render(&c.piece, opts, from, to)
        .events
        .iter()
        .filter_map(|e| match e.kind {
            EventKind::Param { value, glide, .. } => Some((e.bar, e.beat, value, glide)),
            _ => None,
        })
        .collect()
}

fn at(p: &[(i64, Ratio, f64, f64)], bar: i64, beat: Ratio) -> f64 {
    p.iter()
        .find(|x| x.0 == bar && x.1 == beat)
        .unwrap_or_else(|| panic!("no point at {bar} {beat}"))
        .2
}

fn close(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-6, "{a} != {b}");
}

#[test]
fn a_cutoff_rises_over_four_bars_and_stays() {
    let src = format!("{LINE}melodia.lowpass.cutoff = 400 Hz -> 4000 Hz over 4 bars");
    let p = points(&src, 1, 9, &Options::default());
    close(at(&p, 1, Ratio::ZERO), 400.0);
    // A cutoff moves by equal ratios, as its knob turns: halfway in time is
    // the geometric middle, not 2200 Hz.
    close(at(&p, 3, Ratio::ZERO), 400.0 * 10f64.sqrt());
    close(at(&p, 5, Ratio::ZERO), 4000.0);
    assert!(p.iter().all(|x| x.0 <= 5), "nothing after the top");
    // A point every sixteenth of a beat, each gliding from the one before.
    assert_eq!(p.iter().filter(|x| x.0 == 2).count(), 64);
    assert_eq!(p[0].3, 0.0, "the first point sets it");
    close(p[1].3, 0.5 / 16.0);
    for w in p.windows(2) {
        assert!(w[1].2 >= w[0].2, "it only goes up");
    }
}

#[test]
fn there_and_back_again_and_again() {
    let src = format!("{LINE}melodia.reverb.mix = 0 <-> 1 over 2 beats");
    let p = points(&src, 1, 4, &Options::default());
    close(at(&p, 1, Ratio::int(1)), 0.5);
    close(at(&p, 1, Ratio::int(2)), 1.0);
    close(at(&p, 1, Ratio::int(3)), 0.5);
    close(at(&p, 2, Ratio::ZERO), 0.0);
    close(at(&p, 3, Ratio::int(2)), 1.0);
    // Rendering one bar alone gives the same points.
    let alone = points(&src, 3, 4, &Options::default());
    assert_eq!(
        alone,
        p.iter().filter(|x| x.0 == 3).copied().collect::<Vec<_>>()
    );
}

#[test]
fn another_length_back_and_lengths_in_eighths_and_sixteenths() {
    // Up in a beat, down in two beats, again and again.
    let src =
        format!("{LINE}melodia.reverb.mix = 0 -> 1 over 4 sixteenths, back over 4 eighths, loop");
    let p = points(&src, 1, 3, &Options::default());
    close(at(&p, 1, Ratio::int(1)), 1.0);
    close(at(&p, 1, Ratio::int(2)), 0.5);
    close(at(&p, 1, Ratio::int(3)), 0.0);
    close(at(&p, 2, Ratio::ZERO), 1.0);
    // Without `loop` it goes there and back once.
    let src = format!("{LINE}melodia.reverb.mix = 0 -> 1 over 1 beat, back over 1 beat");
    let p = points(&src, 1, 3, &Options::default());
    close(p.last().unwrap().2, 0.0);
    assert_eq!(p.last().unwrap().1, Ratio::int(2));
    // In 6/8 a beat is a dotted quarter: three eighths.
    let src = "meter: 6/8\nmelodia = | c4 _ |\nmelodia.reverb.mix = 0 -> 1 over 3 eighths";
    let p = points(src, 1, 2, &Options::default());
    close(at(&p, 1, Ratio::int(1)), 1.0);
}

#[test]
fn loop_alone_rises_and_starts_again() {
    let src = format!("{LINE}melodia.reverb.mix = 0 -> 1 over 1 bar, loop");
    let p = points(&src, 1, 3, &Options::default());
    let turn: Vec<_> = p
        .iter()
        .filter(|x| x.0 == 2 && x.1 == Ratio::ZERO)
        .collect();
    assert_eq!(turn.len(), 2, "the top, then straight back: {turn:?}");
    close(turn[0].2, 1.0);
    assert!(turn[0].3 > 0.0);
    close(turn[1].2, 0.0);
    assert_eq!(turn[1].3, 0.0);
}

#[test]
fn it_starts_at_its_bar_or_where_it_was_written_live() {
    let src = format!("{LINE}at bar 3: melodia.reverb.mix = 0 -> 1 over 2 bars");
    let p = points(&src, 1, 6, &Options::default());
    assert!(p.iter().all(|x| x.0 >= 3));
    close(at(&p, 4, Ratio::ZERO), 0.5);
    // Written while playing, it starts at the bar it came in.
    let line = "melodia.reverb.mix = 0 -> 1 over 2 bars";
    let c = compile(&format!("{LINE}{line}"));
    let key = &c.piece.commands[0].key;
    // Only what it moves names it, so turning a knob keeps its place.
    assert_eq!(key, "melodia.reverb.mix = ->");
    let mut opts = Options::default();
    opts.anchors.insert(key.clone(), 7);
    let p = points(&format!("{LINE}{line}"), 7, 10, &opts);
    close(at(&p, 7, Ratio::ZERO), 0.0);
    close(at(&p, 8, Ratio::ZERO), 0.5);
    // Every eight bars, first: it starts again at the first of each eight.
    let src = format!("{LINE}every 8 bars, first: melodia.reverb.mix = 0 -> 1 over 4 bars");
    let p = points(&src, 1, 12, &Options::default());
    close(at(&p, 5, Ratio::ZERO), 1.0);
    close(at(&p, 9, Ratio::ZERO), 0.0);
    close(at(&p, 11, Ratio::ZERO), 0.5);
    // Without first it ends where the eight end, like a fill: a rise into
    // the next phrase.
    let src = format!("{LINE}every 8 bars: melodia.reverb.mix = 0 -> 1 over 4 bars");
    let p = points(&src, 1, 18, &Options::default());
    assert!(p.iter().all(|x| x.0 >= 5), "{:?}", p[0]);
    close(at(&p, 5, Ratio::ZERO), 0.0);
    close(at(&p, 7, Ratio::ZERO), 0.5);
    close(at(&p, 9, Ratio::ZERO), 1.0);
    close(at(&p, 13, Ratio::ZERO), 0.0);
}

#[test]
fn a_later_change_of_the_same_parameter_takes_over() {
    let src = format!(
        "{LINE}melodia.reverb.mix = 0 <-> 1 over 1 bar\nat bar 4: melodia.reverb.mix = 0.3"
    );
    let p = points(&src, 1, 7, &Options::default());
    let after: Vec<_> = p.iter().filter(|x| x.0 >= 4).collect();
    assert_eq!(after.len(), 1, "{after:?}");
    close(after[0].2, 0.3);
}

#[test]
fn the_way_it_is_written_is_checked() {
    let fails = |src: &str| -> Message {
        let c = compile(src);
        c.diagnostics
            .first()
            .unwrap_or_else(|| panic!("{src} should fail"))
            .message
            .clone()
    };
    for src in [
        "melodia.reverb.mix = 0 -> 1",
        "melodia.reverb.mix = 0 -> 1 over 4 apples",
        "melodia.reverb.mix = 0 -> 1 over 0 bars",
        "melodia.reverb.mix = 0 -> 1 over 2 bars, back",
    ] {
        assert!(
            matches!(
                fails(&format!("{LINE}{src}")),
                Message::Expected {
                    what: Thing::Length,
                    ..
                }
            ),
            "{src}"
        );
    }
    // A plain number may be followed by `over`: it is no unit.
    let src = format!("{LINE}melodia.reverb.mix = 0.2 -> 0.8 over 2 bars");
    let p = points(&src, 1, 2, &Options::default());
    close(p[0].2, 0.2);
    // Where it goes is checked like any value.
    let c = compile(&format!(
        "{LINE}melodia.lowpass.cutoff = 400 Hz -> 40 kHz over 2 bars"
    ));
    assert!(
        c.diagnostics
            .iter()
            .any(|d| matches!(d.message, Message::ParamOutOfRange { .. }))
    );
}
