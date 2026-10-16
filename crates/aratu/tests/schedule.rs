//! The scheduler: what sounds, when, and how loud.

use aratu::compile;
use aratu::ratio::Ratio;
use aratu::schedule::{Drum, Event, EventKind, Options, render};

fn events(src: &str, from: i64, to: i64) -> Vec<Event> {
    let c = compile(src);
    let errors: Vec<_> = c.errors().collect();
    assert!(errors.is_empty(), "{src}: {errors:?}");
    render(&c.piece, &Options::default(), from, to).events
}

fn close(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-9, "{a} != {b}");
}

fn times(ev: &[Event]) -> Vec<f64> {
    ev.iter().map(|e| e.time).collect()
}

fn notes(ev: &[Event]) -> Vec<u8> {
    ev.iter()
        .filter_map(|e| match e.kind {
            EventKind::Note(n) => Some(n),
            _ => None,
        })
        .collect()
}

#[test]
fn toms_and_the_rim_have_their_own_symbols() {
    let ev = events("drums {\n  toms = t m f -\n  rim = - k - K\n}", 1, 2);
    let kinds: Vec<_> = ev.iter().map(|e| (e.beat, e.kind.clone())).collect();
    for (beat, drum) in [
        (0, Drum::HighTom),
        (1, Drum::MidTom),
        (2, Drum::FloorTom),
        (1, Drum::SideStick),
        (3, Drum::SideStick),
    ] {
        assert!(
            kinds.contains(&(Ratio::from(beat), EventKind::Drum(drum))),
            "{drum:?} on {beat}: {kinds:?}"
        );
    }
    assert_eq!(Drum::FloorTom.name(), "floor-tom");
    assert_eq!(Drum::SideStick.name(), "side-stick");
    // A line named after a tom is a drum line without a `drums` block.
    assert_eq!(events("ft = f f f f", 1, 2).len(), 4);
}

#[test]
fn a_roll_fills_its_length_with_strokes() {
    // A three-beat roll closing on an accent.
    let ev = events("sd = s~ _ _ S", 1, 2);
    let roll: Vec<&Event> = ev.iter().filter(|e| e.beat < Ratio::int(3)).collect();
    assert_eq!(roll.len(), 24, "eight strokes a beat for three beats");
    for (i, e) in roll.iter().enumerate() {
        assert_eq!(e.beat, Ratio::new(i as i64, 8));
        assert_eq!(e.kind, EventKind::Drum(Drum::Snare));
    }
    assert!(roll[0].velocity > roll[1].velocity, "the hands alternate");
    let last = ev.last().unwrap();
    assert_eq!(last.beat, Ratio::int(3));
    assert!(last.accent && last.velocity > roll[0].velocity);
    // Under a hairpin the roll grows into the next mark.
    let ev = events("sd = p:s~< _ _ f:S", 1, 2);
    let evens: Vec<u8> = ev
        .iter()
        .filter(|e| e.beat < Ratio::int(3))
        .step_by(2)
        .map(|e| e.velocity)
        .collect();
    assert!(evens.windows(2).all(|w| w[0] <= w[1]), "{evens:?}");
    assert!(evens.last().unwrap() > &(evens[0] + 30), "{evens:?}");
    // A note rolls as a tremolo.
    let ev = events("m = | c4~ _ d4 e4 |", 1, 2);
    assert_eq!(
        ev.iter().filter(|e| e.kind == EventKind::Note(60)).count(),
        16
    );
}

#[test]
fn eighth_note_hi_hat_at_120() {
    let ev = events("tempo: 120\nhh = cc cc co cc", 1, 2);
    assert_eq!(ev.len(), 8);
    for (i, e) in ev.iter().enumerate() {
        close(e.time, i as f64 * 0.25);
        close(e.duration, 0.25);
    }
    assert_eq!(ev[5].kind, EventKind::Drum(Drum::OpenHat));
    assert_eq!(ev[0].kind, EventKind::Drum(Drum::ClosedHat));
    assert_eq!(ev[0].beat, Ratio::ZERO);
    assert_eq!(ev[5].beat, Ratio::new(5, 2));
}

#[test]
fn the_rhythm_rules_place_parts_exactly() {
    // A triplet, sixteenths, and a dotted eighth with a sixteenth, at 60 bpm.
    let ev = events("tempo: 60\nhh = ccc cccc c.c -c", 1, 2);
    let t = times(&ev);
    let expect = [
        0.0,
        1.0 / 3.0,
        2.0 / 3.0,
        1.0,
        1.25,
        1.5,
        1.75,
        2.0,
        2.75,
        3.5,
    ];
    assert_eq!(t.len(), expect.len());
    for (a, b) in t.iter().zip(expect) {
        close(*a, b);
    }
    close(ev[7].duration, 0.75);
}

#[test]
fn holds_tie_over_the_bar_line() {
    let ev = events("tempo: 60\nbass = | c2 _ _ _ | _ _ e2 _ |", 1, 3);
    assert_eq!(notes(&ev), [36, 40]);
    close(ev[0].duration, 6.0);
    close(ev[1].time, 6.0);
    close(ev[1].duration, 2.0);
}

#[test]
fn a_rest_cuts_the_note_and_staccato_shortens_it() {
    let ev = events("tempo: 60\nm = | c4 _ - d4' |", 1, 2);
    close(ev[0].duration, 2.0);
    close(ev[1].time, 3.0);
    close(ev[1].duration, 0.5);
}

#[test]
fn lines_loop_over_their_own_length() {
    let src = "tempo: 120\nhh = c c c c\nbass = | c2 _ _ _ | g1 _ _ _ |";
    let ev = events(src, 1, 5);
    let bass = notes(&ev);
    assert_eq!(bass, [36, 31, 36, 31]);
    assert_eq!(
        ev.iter()
            .filter(|e| matches!(e.kind, EventKind::Drum(_)))
            .count(),
        16
    );
}

#[test]
fn rendering_any_range_gives_the_same_events() {
    let src = "tempo: 100\nat bar 3: rit to 60 over 2 bars\nhh = cc c? {cc|co} c\nbass = | c2 _ e2 _ | g2 _ _ _ |";
    let all = events(src, 1, 7);
    let part: Vec<Event> = all
        .iter()
        .filter(|e| e.bar >= 4 && e.bar < 6)
        .cloned()
        .collect();
    assert_eq!(events(src, 4, 6), part);
}

#[test]
fn a_ritardando_lands_on_the_closed_form() {
    // Bar 1 at 120, then a rit to 60 over bar 2: 4 beats from 120 to 60.
    let ev = events(
        "tempo: 120\nat bar 2: rit to 60 over 1 bars\nhh = c c c c",
        1,
        4,
    );
    let k = -15.0; // bpm per beat
    let bar2 = 60.0 / k * (60.0f64 / 120.0).ln();
    close(ev[4].time, 2.0);
    close(ev[8].time, 2.0 + bar2);
    // After the rit, 60 bpm: one beat per second.
    close(ev[9].time - ev[8].time, 1.0);
    // Inside the rit, beat 2 of bar 2 is at the integral up to tempo 105.
    close(ev[5].time, 2.0 + 60.0 / k * (105.0f64 / 120.0).ln());
}

#[test]
fn a_tempo_returns_after_the_rit() {
    let src = "tempo: 120\nat bar 2: rit to 60 over 1 bars\nat bar 3: a tempo\nhh = c c c c";
    let c = compile(src);
    let r = render(&c.piece, &Options::default(), 1, 4);
    let bpm: Vec<f64> = r.bars.iter().map(|b| b.bpm).collect();
    assert_eq!(bpm, [120.0, 120.0, 120.0]);
    close(r.bars[2].end_time - r.bars[2].start_time, 2.0);
}

#[test]
fn a_pickup_comes_before_bar_one() {
    let src = "tempo: 60\npickup: beat 4\nmelody = | g4 | c5 _ _ _ |\nhh = c c c c";
    let ev = events(src, 0, 2);
    assert_eq!(ev[0].bar, 0);
    assert_eq!(ev[0].kind, EventKind::Note(67));
    close(ev[0].time, 0.0);
    // Bar 1 starts after the one-beat pickup.
    let c5 = ev.iter().find(|e| e.kind == EventKind::Note(72)).unwrap();
    close(c5.time, 1.0);
    assert_eq!(ev.iter().filter(|e| e.bar == 0).count(), 1);
}

#[test]
fn a_fermata_makes_everyone_wait() {
    let src = "tempo: 60\nm = | c4 d4^ e4 f4 |\nhh = c c c c";
    let ev = events(src, 1, 2);
    let hh: Vec<f64> = ev
        .iter()
        .filter(|e| matches!(e.kind, EventKind::Drum(_)))
        .map(|e| e.time)
        .collect();
    assert_eq!(hh, [0.0, 1.0, 3.0, 4.0]);
    let d = ev.iter().find(|e| e.kind == EventKind::Note(62)).unwrap();
    close(d.duration, 2.0);
}

#[test]
fn chance_is_repeatable_and_about_right() {
    let src = "hh = c? c?30% c c";
    let a = events(src, 1, 401);
    assert_eq!(a, events(src, 1, 401));
    let first = a.iter().filter(|e| e.beat == Ratio::ZERO).count() as f64 / 400.0;
    let second = a.iter().filter(|e| e.beat == Ratio::ONE).count() as f64 / 400.0;
    assert!((first - 0.5).abs() < 0.08, "{first}");
    assert!((second - 0.3).abs() < 0.08, "{second}");
    // Another seed improvises differently.
    let b = events(&format!("seed 7\n{src}"), 1, 401);
    assert_ne!(a, b);
}

#[test]
fn choices_pick_one_option_each_time_round() {
    let ev = events("bass = | {c2|g1} _ c2 _ |", 1, 201);
    let firsts: Vec<u8> = ev
        .iter()
        .filter(|e| e.beat == Ratio::ZERO)
        .map(|e| match e.kind {
            EventKind::Note(n) => n,
            _ => 0,
        })
        .collect();
    assert_eq!(firsts.len(), 200);
    let cs = firsts.iter().filter(|n| **n == 36).count();
    assert!(cs > 70 && cs < 130, "{cs}");
    assert!(firsts.iter().all(|n| *n == 36 || *n == 31));
}

#[test]
fn editing_one_line_keeps_the_others_improvisation() {
    let a = events("hh = c? c? c? c?\nsd = - s - s", 1, 50);
    let b = events("hh = c? c? c? c?\nsd = - s - S", 1, 50);
    let hats = |e: &[Event]| -> Vec<f64> {
        e.iter()
            .filter(|x| x.kind == EventKind::Drum(Drum::ClosedHat))
            .map(|x| x.time)
            .collect()
    };
    assert_eq!(hats(&a), hats(&b));
}

#[test]
fn repeats_and_jumps_play_in_order() {
    let ev = events(
        "m = |: c4 _ _ _ | d4 _ _ _ :| e4 _ _ _ D.C. al fine | fine: f4 _ _ _ |",
        1,
        9,
    );
    assert_eq!(notes(&ev), [60, 62, 60, 62, 64, 60, 62, 64]);
}

#[test]
fn a_golpe_taps_the_top_without_the_strings() {
    let src = "guitar {\n  chords = | Am |\n  strum = d g dg G\n}";
    let ev = events(src, 1, 2);
    let taps: Vec<(Ratio, u8)> = ev
        .iter()
        .filter(|e| e.kind == EventKind::Drum(Drum::GuitarTap))
        .map(|e| (e.beat, e.velocity))
        .collect();
    assert_eq!(taps.len(), 3, "{taps:?}");
    assert_eq!(taps[0].0, Ratio::int(1));
    assert_eq!(taps[1].0, Ratio::new(5, 2));
    assert_eq!(taps[2].0, Ratio::int(3));
    assert!(taps[2].1 > taps[1].1, "an upper-case golpe is accented");
    // No strings sound on a golpe.
    assert!(
        !ev.iter()
            .any(|e| e.beat == Ratio::int(1) && matches!(e.kind, EventKind::Note(_)))
    );
    assert_eq!(Drum::GuitarTap.name(), "guitar-tap");
}

#[test]
fn t_strums_down_and_taps_the_top_together() {
    let src = "guitar {
  chords = | Am |
  strum = t d T -
}";
    let ev = events(src, 1, 2);
    let at =
        |beat: i64| -> Vec<&Event> { ev.iter().filter(|e| e.beat == Ratio::int(beat)).collect() };
    for beat in [0, 2] {
        let here = at(beat);
        let taps = here
            .iter()
            .filter(|e| e.kind == EventKind::Drum(Drum::GuitarTap))
            .count();
        let strings: Vec<u8> = here
            .iter()
            .filter_map(|e| match e.kind {
                EventKind::Note(n) => Some(n),
                _ => None,
            })
            .collect();
        assert_eq!(taps, 1, "one golpe on beat {beat}");
        assert!(strings.len() >= 5, "the down strum sounds on beat {beat}");
        assert!(strings.windows(2).all(|w| w[0] < w[1]), "low to high");
        // The golpe lands with the first string.
        let tap = here
            .iter()
            .find(|e| e.kind == EventKind::Drum(Drum::GuitarTap))
            .unwrap();
        let first = here
            .iter()
            .filter(|e| matches!(e.kind, EventKind::Note(_)))
            .map(|e| e.time)
            .fold(f64::MAX, f64::min);
        close(tap.time, first);
    }
    // A plain down strum has no golpe.
    assert!(
        at(1)
            .iter()
            .all(|e| e.kind != EventKind::Drum(Drum::GuitarTap))
    );
    assert!(at(2)[0].velocity > at(0)[0].velocity, "T is accented");
}

#[test]
fn chords_strum_and_change_at_the_right_bar() {
    let src = "tempo: 60\nguitar {\n  chords = | Am | C G |\n  strum = D du -u du\n}";
    let ev = events(src, 1, 3);
    // Bar 1: six strokes of A minor; the down strum low to high.
    let first: Vec<&Event> = ev
        .iter()
        .filter(|e| e.bar == 1 && e.beat == Ratio::ZERO)
        .collect();
    assert!(first.len() >= 5);
    assert!(first.windows(2).all(|w| w[0].time < w[1].time));
    let pcs = |b: i64, beat: Ratio| -> Vec<u8> {
        let mut v: Vec<u8> = ev
            .iter()
            .filter(|e| e.bar == b && e.beat == beat)
            .filter_map(|e| match e.kind {
                EventKind::Note(n) => Some(n % 12),
                _ => None,
            })
            .collect();
        v.sort();
        v.dedup();
        v
    };
    assert_eq!(pcs(1, Ratio::ZERO), [0, 4, 9]);
    // Bar 2: C on beats 1-2, G on beats 3-4 (strummed on the "and" of 3
    // and on 4).
    assert_eq!(pcs(2, Ratio::ZERO), [0, 4, 7]);
    assert_eq!(pcs(2, Ratio::new(5, 2)), [2, 7, 11]);
    assert_eq!(pcs(2, Ratio::int(3)), [2, 7, 11]);
    // An up strum plays the top strings high to low.
    let up: Vec<&Event> = ev
        .iter()
        .filter(|e| e.bar == 1 && e.beat == Ratio::new(3, 2))
        .collect();
    assert!(up.windows(2).all(|w| {
        let (EventKind::Note(a), EventKind::Note(b)) = (&w[0].kind, &w[1].kind) else {
            return false;
        };
        a > b
    }));
}

#[test]
fn chords_alone_play_as_block_chords() {
    let ev = events("tempo: 60\nchords = | Am _ _ F |", 1, 2);
    let am: Vec<&Event> = ev.iter().filter(|e| e.beat == Ratio::ZERO).collect();
    assert_eq!(am.len(), 4);
    close(am[0].duration, 3.0);
}

#[test]
fn dynamics_accents_and_live_levels() {
    let ev = events("m = | p:c4 d4 E4 f4 |", 1, 2);
    let v: Vec<u8> = ev.iter().map(|e| e.velocity).collect();
    assert_eq!(v, [49, 49, 81, 49]);
    let ev = events("m = | c4 d4 e4 f4 |\nm: ff", 1, 2);
    assert!(ev.iter().all(|e| e.velocity == 112));
    let ev = events("m = | c4 d4 e4 f4 |\nm: fade out over 2 bars", 1, 4);
    let v: Vec<u8> = ev.iter().map(|e| e.velocity).collect();
    assert!(v.windows(2).all(|w| w[0] >= w[1]), "{v:?}");
    assert!(ev.iter().all(|e| e.bar < 3), "silent after the fade");
}

#[test]
fn a_live_crescendo_reaches_the_dynamic_it_names() {
    // Written in p, a crescendo to ff over 4 bars arrives at ff.
    let ev = events(
        "melodia = | p:dó4 mi sol Mi |\nmelodia: cresc to ff over 4 bars",
        1,
        7,
    );
    let plain: Vec<u8> = ev
        .iter()
        .filter(|e| e.beat != Ratio::int(3))
        .map(|e| e.velocity)
        .collect();
    assert_eq!(plain[0], 49, "it starts as written");
    assert!(plain.windows(2).all(|w| w[0] <= w[1]), "{plain:?}");
    let last = |bar: i64, beat: i64| {
        ev.iter()
            .find(|e| e.bar == bar && e.beat == Ratio::int(beat))
            .unwrap()
            .velocity
    };
    assert_eq!(last(5, 0), 112, "ff after four bars");
    assert_eq!(last(6, 0), 112, "and it stays there");
    assert!(last(6, 3) > last(6, 0), "the accent stays above");
    // A dim goes down to its mark the same way.
    let ev = events("m = | f:c4 d4 e4 f4 |\nm: dim to pp over 2 bars", 1, 5);
    assert_eq!(ev[0].velocity, 96);
    assert_eq!(ev.last().unwrap().velocity, 33);
}

#[test]
fn swing_delays_the_second_eighth() {
    let ev = events("tempo: 60\nswing 60%\nhh = cc cc cc cc", 1, 2);
    close(ev[1].time, 0.6);
    close(ev[2].time, 1.0);
}

#[test]
fn humanize_moves_notes_a_little() {
    let ev = events("tempo: 60\nhumanize 10 ms\nhh = c c c c", 1, 2);
    for (i, e) in ev.iter().enumerate() {
        assert!((e.time - i as f64).abs() <= 0.010 + 1e-9);
    }
    assert!(ev.iter().any(|e| (e.time - e.time.round()).abs() > 1e-6));
}

#[test]
fn rubato_bends_inside_the_bar_and_arrives_on_time() {
    let src = "tempo: 60\nm: rubato 30%\nm = | c4 d4 e4 f4 |\nhh = c c c c";
    let ev = events(src, 1, 3);
    let m: Vec<f64> = ev
        .iter()
        .filter(|e| matches!(e.kind, EventKind::Note(_)))
        .map(|e| e.time)
        .collect();
    // Downbeats stay on time, the inner beats move.
    close(m[0], 0.0);
    close(m[4], 4.0);
    assert!((m[1] - 1.0).abs() > 0.01);
    let hh: Vec<f64> = ev
        .iter()
        .filter(|e| matches!(e.kind, EventKind::Drum(_)))
        .map(|e| e.time)
        .collect();
    assert_eq!(hh, [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]);
}

#[test]
fn transformations() {
    let n = |src: &str| notes(&events(src, 1, 2));
    assert_eq!(n("m = | c4 d4 e4 g4 |\nm.transpose +12"), [72, 74, 76, 79]);
    assert_eq!(n("m = | c4 d4 e4 g4 |\nm.invert"), [60, 58, 56, 53]);
    assert_eq!(n("m = | c4 d4 e4 g4 |\nm.retrograde"), [67, 64, 62, 60]);
    let ev = events("m = | c4 d4 e4 g4 |\nm.velocity 0.5", 1, 2);
    assert!(ev.iter().all(|e| e.velocity == 40));
    // Rotate by one part: the open hi-hat moves one eighth later.
    let ev = events("hh = co cc cc cc\nhh.rotate 1", 1, 2);
    let open = ev
        .iter()
        .find(|e| e.kind == EventKind::Drum(Drum::OpenHat))
        .unwrap();
    assert_eq!(open.beat, Ratio::ONE);
    // Augment: one bar of the line lasts two.
    let ev = events("tempo: 60\nm = | c4 d4 e4 g4 |\nm.augment", 1, 3);
    assert_eq!(notes(&ev), [60, 62, 64, 67]);
    close(ev[1].time, 2.0);
    close(ev[3].time, 6.0);
    // Diminish: two bars in one.
    let ev = events(
        "tempo: 60\nm = | c4 d4 e4 g4 | a4 b4 c5 d5 |\nm.diminish",
        1,
        2,
    );
    assert_eq!(notes(&ev), [60, 62, 64, 67, 69, 71, 72, 74]);
    close(ev[4].time, 2.0);
}

#[test]
fn patterns_play_in_turn_every_and_add() {
    let src = "tempo: 120\npattern groove { hh = c c c c }\npattern fill { sd = s s s s }\npattern crash { cr = x - - - }\nplay groove x3, fill\nevery 8 bars, first: add crash";
    let ev = events(src, 1, 9);
    let kinds_in = |b: i64| -> Vec<&str> {
        let mut v: Vec<&str> = ev
            .iter()
            .filter(|e| e.bar == b)
            .filter_map(|e| match e.kind {
                EventKind::Drum(d) => Some(d.name()),
                _ => None,
            })
            .collect();
        v.sort();
        v.dedup();
        v
    };
    assert_eq!(kinds_in(1), ["closed-hat", "crash"]);
    assert_eq!(kinds_in(2), ["closed-hat"]);
    assert_eq!(kinds_in(4), ["snare"]);
    assert_eq!(kinds_in(8), ["snare"]);
    assert_eq!(kinds_in(9).len(), 0, "bar 9 is outside the range");

    let src = "pattern groove { hh = c c c c }\npattern fill { sd = s s s s }\nplay groove\nevery 4 bars: play fill";
    let ev = events(src, 1, 9);
    let snare_bars: Vec<i64> = ev
        .iter()
        .filter(|e| e.kind == EventKind::Drum(Drum::Snare))
        .map(|e| e.bar)
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect();
    assert_eq!(snare_bars, [4, 8]);
}

#[test]
fn scheduled_play_and_live_anchors() {
    let src = "pattern a { hh = c c c c }\npattern b { sd = s s s s }\nplay a\nat bar 3: play b";
    let ev = events(src, 1, 5);
    assert!(
        ev.iter()
            .filter(|e| e.bar < 3)
            .all(|e| e.kind == EventKind::Drum(Drum::ClosedHat))
    );
    assert!(
        ev.iter()
            .filter(|e| e.bar >= 3)
            .all(|e| e.kind == EventKind::Drum(Drum::Snare))
    );

    // Written live, `play b` sounds from the bar the editor remembers.
    let c = compile("pattern a { hh = c c c c }\npattern b { sd = s s s s }\nplay a\nplay b");
    let mut opts = Options::default();
    opts.anchors.insert("play b".into(), 5);
    let ev = render(&c.piece, &opts, 1, 7).events;
    let first_snare = ev
        .iter()
        .find(|e| e.kind == EventKind::Drum(Drum::Snare))
        .unwrap();
    assert_eq!(first_snare.bar, 5);
}

#[test]
fn a_now_line_comes_in_at_a_beat_inside_the_bar() {
    // The editor anchors `now: play b` to the bar it was written in (2) and
    // renders that bar from the next beat (2): the beats before it belong to
    // the version that was playing.
    let src = "tempo: 120\npattern a { hh = c c c c }\npattern b { sd = s s s s }\nplay a\nnow: play b\nnow: hh.volume = 0.5";
    let c = compile(src);
    assert!(c.errors().next().is_none());
    let mut opts = Options {
        from_beat: Ratio::int(2),
        ..Options::default()
    };
    opts.anchors.insert("now: play b".into(), 2);
    opts.anchors.insert("now: hh.volume = 0.5".into(), 2);
    let r = render(&c.piece, &opts, 2, 3);
    let sounds: Vec<&Event> = r
        .events
        .iter()
        .filter(|e| !matches!(e.kind, EventKind::Param { .. }))
        .collect();
    assert_eq!(sounds.len(), 2);
    assert!(
        sounds
            .iter()
            .all(|e| e.kind == EventKind::Drum(Drum::Snare))
    );
    assert_eq!(sounds[0].beat, Ratio::int(2));
    close(sounds[0].time, 3.0);
    close(sounds[1].time, 3.5);
    // A parameter set at that bar line is set at the beat instead.
    let param = r
        .events
        .iter()
        .find(|e| matches!(e.kind, EventKind::Param { .. }))
        .unwrap();
    assert_eq!(param.beat, Ratio::int(2));
    close(param.time, 3.0);
    // Each whole beat's time, for the editor to find the next one.
    assert_eq!(r.bars.len(), 1);
    let bt = &r.bars[0].beat_times;
    assert_eq!(bt.len(), 4);
    for (i, t) in bt.iter().enumerate() {
        close(*t, 2.0 + i as f64 * 0.5);
    }

    // The bars after it play whole.
    opts.from_beat = Ratio::ZERO;
    let next = render(&c.piece, &opts, 3, 4).events;
    let snares = next
        .iter()
        .filter(|e| e.kind == EventKind::Drum(Drum::Snare))
        .count();
    assert_eq!(snares, 4);
}

#[test]
fn once_plays_a_pattern_through_and_the_loop_goes_on() {
    // A four-bar phrase, and a fill played once in its second bar: the
    // phrase does not lose its place, so its own fill still comes in bar 4.
    let src = "pattern groove { sd = - s - s }\npattern fill { sd = s s s s }\npattern crash { cr = x - - - }\nplay groove x3, fill\nnow: play fill once\nnow: add crash once";
    let c = compile(src);
    let errors: Vec<_> = c.errors().collect();
    assert!(errors.is_empty(), "{errors:?}");
    let mut opts = Options::default();
    opts.anchors.insert("now: play fill once".into(), 2);
    opts.anchors.insert("now: add crash once".into(), 2);
    let ev = render(&c.piece, &opts, 1, 9).events;
    let per_bar = |kind: EventKind| -> Vec<usize> {
        (1..9)
            .map(|b| ev.iter().filter(|e| e.bar == b && e.kind == kind).count())
            .collect()
    };
    assert_eq!(
        per_bar(EventKind::Drum(Drum::Snare)),
        [2, 4, 2, 4, 2, 2, 2, 4]
    );
    assert_eq!(
        per_bar(EventKind::Drum(Drum::Crash)),
        [0, 1, 0, 0, 0, 0, 0, 0]
    );

    // Without `once`, the fill stays.
    let c = compile(
        "pattern groove { sd = - s - s }\npattern fill { sd = s s s s }\nplay groove x3, fill\nnow: play fill",
    );
    let mut opts = Options::default();
    opts.anchors.insert("now: play fill".into(), 2);
    let ev = render(&c.piece, &opts, 1, 6).events;
    let snares = (1..6)
        .map(|b| ev.iter().filter(|e| e.bar == b).count())
        .collect::<Vec<_>>();
    assert_eq!(snares, [2, 4, 4, 4, 4]);
}

#[test]
fn the_form_plays_sections_in_order() {
    let src =
        "section a { m = | c4 _ _ _ | }\nsection b { m = | d4 _ _ _ | e4 _ _ _ | }\nform: a b x2 a";
    let ev = events(src, 1, 8);
    assert_eq!(notes(&ev), [60, 62, 64, 62, 64, 60, 60]);
}

#[test]
fn meter_changes_move_the_bar_lines() {
    let src = "tempo: 60\nat bar 2: meter 3/4\nhh = c c c c\nat bar 2: hh = c c c";
    // The second assignment replaces the first everywhere; check bar lengths.
    let c = compile("tempo: 60\nat bar 2: meter 3/4\nm = | c4 _ _ _ | d4 _ _ |");
    let r = render(&c.piece, &Options::default(), 1, 4);
    let lens: Vec<f64> = r.bars.iter().map(|b| b.end_time - b.start_time).collect();
    assert_eq!(lens, [4.0, 3.0, 3.0]);
    let _ = src;
}

#[test]
fn every_example_in_the_manual_schedules() {
    let manual = [
        include_str!("../../../book/pt/manual.md"),
        include_str!("../../../book/es/manual.md"),
        include_str!("../../../book/en/manual.md"),
    ]
    .concat();
    let mut in_code = false;
    let mut block = String::new();
    for line in manual.lines() {
        if line.trim_start().starts_with("```") {
            if in_code {
                let c = compile(&block);
                let r = render(&c.piece, &Options::default(), 0, 9);
                for e in &r.events {
                    assert!(e.time.is_finite() && e.time >= -0.05, "{block}");
                    assert!(e.duration >= 0.0);
                }
                block.clear();
            }
            in_code = !in_code;
        } else if in_code {
            block.push_str(line);
            block.push('\n');
        }
    }
}

#[test]
fn whole_bar_and_long_rests() {
    // `| - |` is a whole bar of rest in any meter; `- x8` is eight bars.
    let src = "tempo: 60\nmeter: 3/4\nmelody = | - x2 | c4 _ _ | - | d4 _ _ |";
    let c = compile(src);
    assert!(c.diagnostics.is_empty(), "{:?}", c.diagnostics);
    let line = &c.piece.parts[0].lines[0];
    assert_eq!(line.order.len(), 5);
    let ev = render(&c.piece, &Options::default(), 1, 6).events;
    assert_eq!(notes(&ev), [60, 62]);
    assert_eq!(ev[0].bar, 3);
    close(ev[0].time, 6.0);
    assert_eq!(ev[1].bar, 5);
    // A rest cuts a note held into it.
    let ev = events("tempo: 60\nm = | c4 _ _ _ | - | _ _ _ _ |", 1, 4);
    assert_eq!(ev.len(), 1);
    close(ev[0].duration, 4.0);
}

#[test]
fn parts_go_out_and_come_back_in() {
    let src =
        "drums { hh = c c c c }\nbass = | c2 _ _ _ |\nat bar 2: drums: out\nat bar 4: drums: in";
    let ev = events(src, 1, 6);
    let hat_bars: std::collections::BTreeSet<i64> = ev
        .iter()
        .filter(|e| matches!(e.kind, EventKind::Drum(_)))
        .map(|e| e.bar)
        .collect();
    assert_eq!(hat_bars.into_iter().collect::<Vec<_>>(), [1, 4, 5]);
    assert_eq!(notes(&ev).len(), 5, "the bass keeps playing");

    let ev = events("bass = | c2 _ _ _ |\nevery 4 bars, first: bass: out", 1, 9);
    let bars: Vec<i64> = ev.iter().map(|e| e.bar).collect();
    assert_eq!(bars, [2, 3, 4, 6, 7, 8]);

    let ev = events("bass = | c2 _ _ _ |\nhh = c c c c\nall: out", 1, 3);
    assert!(ev.is_empty());
}

#[test]
fn a_strum_points_at_the_chord_it_plays() {
    let src = "seed 7\nguitar {\n  chords = | Am | Dm {E|G} |\n  strum = d d d d\n}";
    let c = compile(src);
    assert!(c.diagnostics.is_empty(), "{:?}", c.diagnostics);
    let ev = render(&c.piece, &Options::default(), 1, 13).events;
    let written = |e: &Event| e.also.map(|s| &src[s.start..s.end]);
    // Bar 1: every strum lights Am.
    assert!(
        ev.iter()
            .filter(|e| e.bar == 1)
            .all(|e| written(e) == Some("Am"))
    );
    // Bar 2's second half lights whichever of E and G was chosen, and
    // both are chosen over the bars.
    let mut chosen = std::collections::HashSet::new();
    for e in ev
        .iter()
        .filter(|e| e.bar % 2 == 0 && e.beat >= Ratio::int(2))
    {
        let w = written(e).unwrap();
        let pcs = match e.kind {
            EventKind::Note(n) => n % 12,
            _ => continue,
        };
        // The chord sounding and the chord lit agree.
        if w == "E" {
            assert!([4, 8, 11].contains(&pcs), "{w} {pcs}");
        } else {
            assert_eq!(w, "G");
            assert!([7, 11, 2].contains(&pcs), "{w} {pcs}");
        }
        chosen.insert(w);
    }
    assert_eq!(chosen.len(), 2, "{chosen:?}");
}
