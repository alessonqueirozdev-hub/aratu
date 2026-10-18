// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
//! The loudness of every note of a line, from its marks and hairpins.
//!
//! A mark (`p:`, `f:`) sets the level from its note on; `sfz` makes one note
//! very strong and `fp` one note loud before the level drops to piano. A
//! hairpin (`<`, `>`) moves the level evenly from the beat after its note to
//! the next mark; with no mark after it, it moves one step by the end of the
//! line. A line that loops starts each time round at the level it ended at.

use crate::ast::{BarContent, Hairpin, Hit, Slot};
use crate::diag::{Diagnostic, Message};
use crate::model::Line;
use crate::ratio::Ratio;
use crate::theory::Dyn;
use std::collections::HashMap;

/// The key of the pickup bar in [`Line::levels`].
pub const PICKUP: usize = usize::MAX;

/// Calls `f` for every hit in `slots` laid over `[start, start + len)`, with
/// its start and length. Every option of a choice is visited, each laid
/// over the choice's whole part.
pub fn visit<'a>(
    slots: &'a [Slot],
    start: Ratio,
    len: Ratio,
    f: &mut impl FnMut(&'a Hit, Ratio, Ratio),
) {
    if slots.is_empty() {
        return;
    }
    let part = len / Ratio::int(slots.len() as i64);
    for (i, s) in slots.iter().enumerate() {
        let at = start + part * Ratio::int(i as i64);
        match s {
            Slot::Hit(h) => f(h, at, part),
            Slot::Group(g, _) => visit(g, at, part, f),
            Slot::Choice(options, _) => {
                for o in options {
                    visit(o, at, part, f);
                }
            }
            Slot::Hold(_) | Slot::Rest(_) => {}
        }
    }
}

struct Seen {
    pos: Ratio,
    key: (usize, usize),
    mark: Option<Dyn>,
}

/// Works out the velocity of every hit of the line, keyed by the bar's
/// place in the playing order (or [`PICKUP`]) and the hit's source offset.
pub fn levels(line: &Line, diags: &mut Vec<Diagnostic>) -> HashMap<(usize, usize), u8> {
    let mut hits: Vec<Seen> = Vec::new();
    let mut marks: Vec<(Ratio, Dyn)> = Vec::new();
    let mut pins: Vec<(Ratio, bool, Ratio, crate::span::Span)> = Vec::new();
    let mut pos = Ratio::ZERO;

    let bars: Vec<(usize, usize)> = line
        .pickup
        .map(|p| (PICKUP, p))
        .into_iter()
        .chain(line.order.iter().copied().enumerate())
        .collect();
    for (k, i) in bars {
        let BarContent::Beats(beats) = &line.bars[i].content else {
            continue;
        };
        let mut beat_start = pos;
        for beat in beats {
            let len = Ratio::int(beat.len as i64);
            visit(&beat.slots, beat_start, len, &mut |h, at, _| {
                if let Some(d) = h.dynamic {
                    marks.push((at, d));
                }
                if let Some(pin) = h.hairpin {
                    pins.push((
                        Ratio::int(at.floor() + 1),
                        pin == Hairpin::Crescendo,
                        at,
                        h.span,
                    ));
                }
                hits.push(Seen {
                    pos: at,
                    key: (k, h.span.start),
                    mark: h.dynamic,
                });
            });
            beat_start += len;
        }
        pos = beat_start;
    }
    let end = pos;

    let initial = marks.last().map_or(Dyn::Mf.velocity(), |(_, d)| {
        d.level_after(Dyn::Mf.velocity())
    });
    // The level from marks alone, just after position x.
    let level_at = |x: Ratio| -> f64 {
        let mut level = initial;
        for (p, d) in &marks {
            if *p > x {
                break;
            }
            level = d.level_after(level);
        }
        level as f64
    };

    // Each hairpin as (start, end, start level, end level).
    let mut ramps: Vec<(Ratio, Ratio, f64, f64)> = Vec::new();
    for &(start, up, hit_pos, span) in &pins {
        let from = level_at(hit_pos);
        match marks.iter().find(|(p, _)| *p > hit_pos) {
            Some((p, d)) => ramps.push((start, *p, from, d.velocity() as f64)),
            None => {
                diags.push(Diagnostic::warning(
                    span,
                    Message::HairpinWithoutTarget {
                        line: line.label.clone(),
                    },
                ));
                let to = Dyn::step(from.round() as u8, up) as f64;
                ramps.push((start, end.max(start), from, to));
            }
        }
    }

    let mut out = HashMap::new();
    for h in &hits {
        let v = match h.mark {
            Some(d) => d.velocity() as f64,
            None => {
                let mut v = level_at(h.pos);
                for &(s, e, from, to) in &ramps {
                    if h.pos >= s && h.pos < e {
                        let t = ((h.pos - s) / (e - s)).to_f64();
                        v = from + (to - from) * t;
                    }
                }
                v
            }
        };
        out.insert(h.key, v.round().clamp(1.0, 127.0) as u8);
    }
    out
}

#[cfg(test)]
mod tests {
    use crate::compile;

    /// Velocities of a one-line piece in playing order.
    fn vels(src: &str) -> Vec<u8> {
        let c = compile(src);
        let line = &c.piece.parts[0].lines[0];
        let mut v: Vec<((usize, usize), u8)> = line.levels.iter().map(|(k, v)| (*k, *v)).collect();
        v.sort();
        v.into_iter().map(|(_, v)| v).collect()
    }

    #[test]
    fn marks_set_the_level() {
        assert_eq!(vels("m = | c4 d e f |"), [80, 80, 80, 80]);
        assert_eq!(
            vels("m = | p:c4 e4 g4 c5 | f:g4 _ _ _ |"),
            [49, 49, 49, 49, 96]
        );
        // The loop starts again at the level it ended at.
        assert_eq!(vels("m = | c4 d e f:g |"), [96, 96, 96, 96]);
    }

    #[test]
    fn sfz_and_fp() {
        assert_eq!(vels("m = | mp:c4 sfz:d4 e4 f4 |"), [64, 127, 64, 64]);
        assert_eq!(vels("m = | fp:c4 d4 e4 f4 |"), [96, 49, 49, 49]);
    }

    #[test]
    fn a_hairpin_runs_from_the_next_beat_to_the_next_mark() {
        // p (49) to f (96) from beat 1 to beat 4: 49, 64.67, 80.33, then f.
        assert_eq!(
            vels("m = | p:c4< d4 e4 f4 | f:g4 _ _ _ |"),
            [49, 49, 65, 80, 96]
        );
    }

    #[test]
    fn a_hairpin_without_a_mark_moves_one_step_and_warns() {
        let c = compile("m = | mf:c4> d4 e4 f4 |");
        assert!(matches!(
            c.diagnostics[0].message,
            crate::diag::Message::HairpinWithoutTarget { .. }
        ));
        let v = vels("m = | mf:c4> d4 e4 f4 |");
        // mf (80) down to mp (64) between beat 1 and the end (beat 4).
        assert_eq!(v, [80, 80, 75, 69]);
    }
}
