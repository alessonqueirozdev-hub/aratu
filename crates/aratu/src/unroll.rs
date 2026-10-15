//! Unrolling a line as a musician reads it: similes written out, repeats
//! taken, endings chosen, jumps followed.
//!
//! The conventions are the common ones: a passage between `|:` and `:|` is
//! played twice unless a count says otherwise; `:|` with no `|:` repeats
//! from the start (or from just after the previous `:|`); the bars of an
//! ending are played only on their pass; after a `D.C.` or `D.S.` repeats are
//! not taken again and only the last ending is played.

use crate::ast::{Al, Bar, BarContent, Jump, Marker};
use crate::diag::{Diagnostic, Message};
use std::collections::{HashMap, HashSet};

/// The longest a line may unroll to before it is taken as endless.
pub const MAX_BARS: usize = 4096;

/// Writes out `%` and `%%` (each becomes a copy of the bar, or two bars,
/// before it) and `| - x8 |` (eight bars of rest). The written bar's repeat
/// signs and markers go on the first bar written out, its `:|` and jump on
/// the last.
pub fn expand_similes(bars: &[Bar], diags: &mut Vec<Diagnostic>) -> Vec<Bar> {
    let mut out: Vec<Bar> = Vec::with_capacity(bars.len());
    for bar in bars {
        let contents: Vec<BarContent> = match bar.content {
            BarContent::Simile(n) => {
                let n = n as usize;
                if out.len() < n {
                    diags.push(Diagnostic::error(bar.span, Message::NothingToRepeat));
                    continue;
                }
                out[out.len() - n..]
                    .iter()
                    .map(|b| b.content.clone())
                    .collect()
            }
            BarContent::Rest(n) if n > 1 => vec![BarContent::Rest(1); n as usize],
            _ => {
                out.push(bar.clone());
                continue;
            }
        };
        let n = contents.len();
        for (k, content) in contents.into_iter().enumerate() {
            let mut b = Bar {
                span: bar.span,
                content,
                repeat_start: false,
                ending: None,
                repeat_end: None,
                markers: Vec::new(),
                jump: None,
            };
            if k == 0 {
                b.repeat_start = bar.repeat_start;
                b.ending = bar.ending;
                b.markers = bar.markers.clone();
            }
            if k == n - 1 {
                b.repeat_end = bar.repeat_end;
                b.jump = bar.jump;
            }
            out.push(b);
        }
    }
    out
}

/// The order in which the bars are played, as indices into `bars`.
///
/// `label` names the line in messages; `first_number` is the number of
/// `bars[0]` as a musician counts it.
pub fn unroll(
    bars: &[Bar],
    label: &str,
    first_number: usize,
    diags: &mut Vec<Diagnostic>,
) -> Vec<usize> {
    let n = bars.len();
    if n == 0 {
        return Vec::new();
    }
    let number = |i: usize| i + first_number;

    // Where the passage that ends at each `:|` starts.
    let mut region_start: HashMap<usize, usize> = HashMap::new();
    let mut open = 0;
    for (i, b) in bars.iter().enumerate() {
        if b.repeat_start {
            open = i;
        }
        if b.repeat_end.is_some() {
            region_start.insert(i, open);
            open = i + 1;
        }
    }
    // For each bar inside a repeated passage, the end of that passage, and
    // the ending it belongs to (an ending runs from `|1` to the `:|`).
    let mut region_end: Vec<Option<usize>> = vec![None; n];
    let mut ending: Vec<Option<u32>> = vec![None; n];
    for (&end, &start) in &region_start {
        let mut current = None;
        for i in start..=end {
            region_end[i] = Some(end);
            if bars[i].ending.is_some() {
                current = bars[i].ending;
            }
            ending[i] = current;
        }
    }

    let segno = bars
        .iter()
        .position(|b| b.markers.iter().any(|(m, _)| *m == Marker::Segno));
    let has_marker = |m: Marker| bars.iter().any(|b| b.markers.iter().any(|(x, _)| *x == m));
    let coda = bars
        .iter()
        .position(|b| b.markers.iter().any(|(m, _)| *m == Marker::Coda));
    let has_to_coda = bars
        .iter()
        .any(|b| matches!(b.jump, Some((Jump::ToCoda, _))));

    // Check every jump before following any.
    let mut broken: HashSet<usize> = HashSet::new();
    for (i, b) in bars.iter().enumerate() {
        let Some((Jump::Back { to_segno, al }, span)) = b.jump else {
            continue;
        };
        let jump = b.jump.unwrap().0.words().to_string();
        let mut missing = |marker: &str| {
            diags.push(Diagnostic::error(
                span,
                Message::JumpWithoutMarker {
                    line: label.to_string(),
                    jump: jump.clone(),
                    bar: number(i),
                    marker: marker.to_string(),
                },
            ));
        };
        if to_segno && !segno.is_some_and(|s| s <= i) {
            missing("segno");
            broken.insert(i);
        }
        match al {
            Al::Fine if !has_marker(Marker::Fine) => {
                missing("fine:");
                broken.insert(i);
            }
            Al::Coda if coda.is_none() => {
                missing("coda:");
                broken.insert(i);
            }
            Al::Coda if !has_to_coda => {
                missing("to coda");
                broken.insert(i);
            }
            _ => {}
        }
    }

    let mut out = Vec::new();
    let mut pass: HashMap<usize, u32> = HashMap::new();
    let mut taken: HashSet<usize> = HashSet::new();
    let mut jumped = false;
    let mut stop_at_fine = false;
    let mut to_coda = false;
    let mut i = 0;
    while i < n {
        if out.len() >= MAX_BARS {
            diags.push(Diagnostic::error(
                bars[0].span,
                Message::RepeatsNeverEnd {
                    line: label.to_string(),
                },
            ));
            break;
        }
        let bar = &bars[i];
        if let (Some(e), Some(end)) = (ending[i], region_end[i]) {
            let p = if jumped {
                bars[end].repeat_end.unwrap_or(2)
            } else {
                *pass.get(&end).unwrap_or(&1)
            };
            if e != p {
                i += 1;
                continue;
            }
        }
        out.push(i);
        if jumped && stop_at_fine && bar.markers.iter().any(|(m, _)| *m == Marker::Fine) {
            break;
        }
        if jumped && to_coda && matches!(bar.jump, Some((Jump::ToCoda, _))) {
            to_coda = false;
            i = coda.expect("checked above");
            continue;
        }
        if let (Some(count), false) = (bar.repeat_end, jumped) {
            let p = pass.entry(i).or_insert(1);
            if *p < count {
                *p += 1;
                i = region_start[&i];
                continue;
            }
        }
        if let Some((Jump::Back { to_segno, al }, _)) = bar.jump {
            if !taken.contains(&i) && !broken.contains(&i) {
                taken.insert(i);
                jumped = true;
                stop_at_fine = al == Al::Fine;
                to_coda = al == Al::Coda;
                i = if to_segno { segno.unwrap() } else { 0 };
                continue;
            }
        }
        i += 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::LineKind;
    use crate::music::parse_music;

    /// Unrolls a chord line and returns the chord names in playing order.
    fn play(text: &str) -> (Vec<String>, Vec<Diagnostic>) {
        let mut d = Vec::new();
        let bars = parse_music(text, 0, LineKind::Chords, &mut d);
        let bars = expand_similes(&bars, &mut d);
        let order = unroll(&bars, "chords", 1, &mut d);
        let names = order
            .iter()
            .map(|&i| {
                let BarContent::Beats(b) = &bars[i].content else {
                    panic!()
                };
                b.iter()
                    .map(|beat| text[beat.span.start..beat.span.end].to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect();
        (names, d)
    }

    fn ok(text: &str) -> Vec<String> {
        let (names, d) = play(text);
        assert!(d.is_empty(), "{text}: {d:?}");
        names
    }

    #[test]
    fn plain_bars() {
        assert_eq!(ok("| Am | F | C G | G |"), ["Am", "F", "C G", "G"]);
    }

    #[test]
    fn repeats_and_counts() {
        assert_eq!(ok("|: Am | F :|"), ["Am", "F", "Am", "F"]);
        assert_eq!(ok("|: Am | F :| x4").len(), 8);
        assert_eq!(ok("| C |: Am | F :| G |"), ["C", "Am", "F", "Am", "F", "G"]);
        // `:|` without `|:` repeats from the start.
        assert_eq!(ok("| Am | F :| G |"), ["Am", "F", "Am", "F", "G"]);
        // Two passages one after the other.
        assert_eq!(ok("| Am :| F :|"), ["Am", "Am", "F", "F"]);
    }

    #[test]
    fn first_and_second_endings() {
        assert_eq!(
            ok("|: Am | F |1 C | G :|2 C | C |"),
            ["Am", "F", "C", "G", "Am", "F", "C", "C"]
        );
        assert_eq!(ok("|: A |1 B :|2 C |"), ["A", "B", "A", "C"]);
    }

    #[test]
    fn similes() {
        assert_eq!(ok("| Am | % | % | F |"), ["Am", "Am", "Am", "F"]);
        assert_eq!(ok("| Am | F | %% |"), ["Am", "F", "Am", "F"]);
        let (_, d) = play("| % | Am |");
        assert_eq!(d[0].message, Message::NothingToRepeat);
    }

    #[test]
    fn dal_segno_al_fine() {
        // The example from the manual, with chords in place of notes.
        assert_eq!(
            ok("| C | segno: D | E | fine: F | G D.S. al fine |"),
            ["C", "D", "E", "F", "G", "D", "E", "F"]
        );
    }

    #[test]
    fn da_capo_and_coda() {
        assert_eq!(ok("| A | B D.C. |"), ["A", "B", "A", "B"]);
        assert_eq!(
            ok("| A | B to coda | C D.C. al coda | coda: E |"),
            ["A", "B", "C", "A", "B", "E"]
        );
    }

    #[test]
    fn repeats_are_not_taken_after_a_jump() {
        assert_eq!(
            ok("|: A |1 B :|2 C | D D.C. |"),
            ["A", "B", "A", "C", "D", "A", "C", "D"]
        );
    }

    #[test]
    fn jumps_without_their_marker() {
        let (_, d) = play("| A | B | C D.S. |");
        assert_eq!(
            d[0].message,
            Message::JumpWithoutMarker {
                line: "chords".into(),
                jump: "D.S.".into(),
                bar: 3,
                marker: "segno".into()
            }
        );
        let (_, d) = play("| A | B D.C. al fine |");
        assert!(
            matches!(&d[0].message, Message::JumpWithoutMarker { marker, .. } if marker == "fine:")
        );
        let (_, d) = play("| A | coda: B D.C. al coda |");
        assert!(
            matches!(&d[0].message, Message::JumpWithoutMarker { marker, .. } if marker == "to coda")
        );
        // A broken jump is not followed.
        let (names, _) = play("| A | B D.S. |");
        assert_eq!(names, ["A", "B"]);
    }
}
