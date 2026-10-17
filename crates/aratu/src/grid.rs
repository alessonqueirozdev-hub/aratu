//! The rhythm map: each line's bars as a grid of equal steps, fine enough
//! to show every subdivision written (eighths, sixteenths, triplets), with
//! where each sound is written so the editor can move it and rewrite the
//! bar.

use crate::ast::{BarContent, LineKind, Slot, Sound};
use crate::model::{Line, Piece};
use crate::ratio::Ratio;
use crate::span::Span;

/// The finest grid drawn: 48 steps per beat (sixteenth triplets and more).
pub const MAX_RES: i64 = 48;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Step {
    /// Nothing sounds.
    Rest,
    /// The sound before goes on (or, at a bar's start, the last bar's).
    Hold,
    /// A sound starts here, lasting `len` steps (holds included).
    Start {
        span: Span,
        len: i64,
        /// For notes: the written text with every octave made explicit, so
        /// the note keeps its pitch wherever it is moved.
        text: Option<String>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub struct GridBar {
    /// Where the bar sounds in one time round: 0 is the pickup, then 1, 2...
    pub position: i64,
    /// The written text of the bar's beats, to rewrite; `None` when the bar
    /// cannot be rewritten from the map (a `%`, eight bars of rest).
    pub span: Option<Span>,
    pub beats: i64,
    /// Steps per beat.
    pub res: i64,
    pub steps: Vec<Step>,
    /// Whether the map may rewrite it: no choices, no chord shares, and a
    /// grid that holds every part exactly.
    pub editable: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GridLine {
    pub label: String,
    pub kind: LineKind,
    pub bars: Vec<GridBar>,
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a.abs() } else { gcd(b, a % b) }
}

fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

#[derive(Clone, Copy)]
enum Part<'a> {
    Hit(&'a crate::ast::Hit),
    Hold,
    Rest,
}

/// Lays out slots over `[start, start + len)`, the first option of a choice
/// standing for it. Returns whether a choice was met.
fn lay<'a>(
    slots: &'a [Slot],
    start: Ratio,
    len: Ratio,
    out: &mut Vec<(Ratio, Ratio, Part<'a>)>,
) -> bool {
    if slots.is_empty() {
        return false;
    }
    let mut choice = false;
    let part = len / Ratio::int(slots.len() as i64);
    for (i, s) in slots.iter().enumerate() {
        let at = start + part * Ratio::int(i as i64);
        match s {
            Slot::Hit(h) => out.push((at, part, Part::Hit(h))),
            Slot::Hold(_) => out.push((at, part, Part::Hold)),
            Slot::Rest(_) => out.push((at, part, Part::Rest)),
            Slot::Group(g, _) => choice |= lay(g, at, part, out),
            Slot::Choice(o, _) => {
                choice = true;
                lay(&o[0], at, part, out);
            }
        }
    }
    choice
}

/// The text of a note with each octave written out: `e` after `c4` becomes
/// `e4`, `dó` becomes `dó4`. Marks before and after are kept.
pub fn explicit_octaves(text: &str, pitches: &[i32]) -> String {
    let mut out = String::new();
    let mut rest = text;
    // A dynamic before the note stays as written.
    if let Some(colon) = rest.find(':') {
        if rest[..colon].chars().all(|c| c.is_ascii_alphabetic()) {
            out.push_str(&rest[..=colon]);
            rest = &rest[colon + 1..];
        }
    }
    let mut pitches = pitches.iter();
    for (k, piece) in rest.split('+').enumerate() {
        if k > 0 {
            out.push('+');
        }
        let Some((_, _, nlen)) = crate::theory::note_name(piece) else {
            out.push_str(piece);
            continue;
        };
        let (_, alen) = crate::theory::accidentals(&piece[nlen..]);
        let name_end = nlen + alen;
        let after = &piece[name_end..];
        let has_octave = after.starts_with(|c: char| c.is_ascii_digit());
        out.push_str(&piece[..name_end]);
        if !has_octave {
            if let Some(&p) = pitches.next() {
                // The octave of the written name: undo the accidental first.
                let (acc, _) = crate::theory::accidentals(&piece[nlen..]);
                out.push_str(&((p - acc).div_euclid(12) - 1).to_string());
            }
        } else {
            pitches.next();
        }
        out.push_str(after);
    }
    out
}

/// A MIDI note in letters with its octave: 61 is `c#4`.
fn note_with_octave(n: i32) -> String {
    const NAMES: [&str; 12] = [
        "c", "c#", "d", "eb", "e", "f", "f#", "g", "ab", "a", "bb", "b",
    ];
    format!(
        "{}{}",
        NAMES[n.rem_euclid(12) as usize],
        n.div_euclid(12) - 1
    )
}

fn grid_bar(line: &Line, index: usize, position: i64, beats: i64, src: &str) -> GridBar {
    let bar = &line.bars[index];
    let mut parts: Vec<(Ratio, Ratio, Part)> = Vec::new();
    let mut choice = false;
    let mut span = None;
    match &bar.content {
        BarContent::Beats(written) => {
            let share = if line.kind == LineKind::Chords {
                Ratio::new(beats, written.len().max(1) as i64)
            } else {
                Ratio::ONE
            };
            let mut at = Ratio::ZERO;
            for beat in written {
                let len = share * Ratio::int(beat.len as i64);
                choice |= lay(&beat.slots, at, len, &mut parts);
                at += len;
            }
            if let (Some(first), Some(last)) = (written.first(), written.last()) {
                let s = first.span.join(last.span);
                // A bar copied by `%` has its beats written in another bar,
                // and a motif or a repeat (`[e3 d3]x4`) makes beats that
                // are written once or elsewhere: none can be redrawn.
                let own = written.windows(2).all(|w| w[0].span.end <= w[1].span.start)
                    && written
                        .iter()
                        .all(|b| b.span.start >= bar.span.start && b.span.end <= bar.span.end);
                if own && s.start >= bar.span.start && s.end <= bar.span.end {
                    span = Some(s);
                }
            }
        }
        BarContent::Rest(1) => {
            parts.push((Ratio::ZERO, Ratio::int(beats), Part::Rest));
            // Only a bar written as a lone `-` (not one of `- x8`).
            if bar.markers.is_empty()
                && bar.jump.is_none()
                && src[bar.span.start..bar.span.end].trim() == "-"
            {
                span = Some(bar.span);
            }
        }
        _ => parts.push((Ratio::ZERO, Ratio::int(beats), Part::Rest)),
    }

    // Steps per beat: enough for every start and length written.
    let mut res = 1;
    let mut exact = true;
    for (s, d, _) in &parts {
        for r in [*s, *d] {
            let den = r.den();
            let next = lcm(res, den);
            if next > MAX_RES {
                exact = false;
            } else {
                res = next;
            }
        }
    }
    let total = (beats * res).max(1);
    let mut steps = vec![Step::Rest; total as usize];
    let at = |r: Ratio| -> i64 { (r * Ratio::int(res)).to_f64().round() as i64 };
    let mut current: Option<usize> = None;
    for (s, d, p) in &parts {
        let (i0, i1) = (at(*s).clamp(0, total), at(*s + *d).clamp(0, total));
        match p {
            Part::Hit(h) => {
                let written = &src[h.span.start..h.span.end];
                let text = match &h.sound {
                    // An `o` shows the note the row gave it.
                    Sound::Notes(n) if n.len() == 1 && written.starts_with(['o', 'O']) => {
                        Some(note_with_octave(n[0]))
                    }
                    Sound::Notes(n) => Some(explicit_octaves(written, n)),
                    _ => None,
                };
                if i0 < total {
                    steps[i0 as usize] = Step::Start {
                        span: h.span,
                        len: (i1 - i0).max(1),
                        text,
                    };
                    for k in i0 + 1..i1 {
                        steps[k as usize] = Step::Hold;
                    }
                    current = Some(i0 as usize);
                }
            }
            Part::Hold => {
                for k in i0..i1 {
                    steps[k as usize] = Step::Hold;
                }
                if let Some(c) = current {
                    if let Step::Start { len, .. } = &mut steps[c] {
                        *len += i1 - i0;
                    }
                }
            }
            Part::Rest => {
                current = None;
                for k in i0..i1 {
                    steps[k as usize] = Step::Rest;
                }
            }
        }
    }
    // A sound written once that plays several times, as a Euclidean
    // rhythm's `b(3,8)`, cannot be moved one hit at a time.
    let mut written: Vec<usize> = parts
        .iter()
        .filter_map(|(_, _, p)| match p {
            Part::Hit(h) => Some(h.span.start),
            _ => None,
        })
        .collect();
    let hits = written.len();
    written.sort_unstable();
    written.dedup();
    let editable = exact
        && !choice
        && written.len() == hits
        && span.is_some()
        && matches!(
            line.kind,
            LineKind::Drums | LineKind::Strum | LineKind::Pitched
        );
    GridBar {
        position,
        span,
        beats,
        res,
        steps,
        editable,
    }
}

/// The map of every line, one time round, at most `max_bars` bars each.
pub fn grid(piece: &Piece, src: &str, max_bars: usize) -> Vec<GridLine> {
    let beats = piece.meter.beats().floor().max(1);
    let mut out = Vec::new();
    for part in piece.all_parts() {
        // An arpeggiator's steps are not written: nothing to draw.
        for line in part.lines.iter().filter(|l| l.kind != LineKind::Arp) {
            let mut bars = Vec::new();
            if let Some(p) = line.pickup {
                let b = line.beats(p, piece.meter).floor().max(1);
                bars.push(grid_bar(line, p, 0, b, src));
            }
            for (k, &i) in line.order.iter().take(max_bars).enumerate() {
                let b = match &line.bars[i].content {
                    BarContent::Beats(w) if line.kind != LineKind::Chords => {
                        w.iter().map(|b| b.len as i64).sum()
                    }
                    _ => beats,
                };
                bars.push(grid_bar(line, i, k as i64 + 1, b.max(1), src));
            }
            out.push(GridLine {
                label: line.label.clone(),
                kind: line.kind,
                bars,
            });
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compile;

    fn first_bar(src: &str) -> GridBar {
        let c = compile(src);
        grid(&c.piece, src, 8).remove(0).bars.remove(0)
    }

    #[test]
    fn the_grid_is_as_fine_as_the_rhythm() {
        assert_eq!(first_bar("hh = c c c c").res, 1);
        assert_eq!(first_bar("hh = cc cc cc cc").res, 2);
        assert_eq!(first_bar("hh = cccc c c c").res, 4);
        assert_eq!(first_bar("hh = ccc c c c").res, 3);
        assert_eq!(first_bar("hh = ccc cc c c").res, 6);
        let b = first_bar("hh = c.c c c c");
        assert_eq!(b.res, 4);
        assert!(matches!(b.steps[0], Step::Start { len: 3, .. }));
        assert!(matches!(b.steps[3], Step::Start { len: 1, .. }));
    }

    #[test]
    fn holds_lengthen_notes_and_rests_stay_empty() {
        let src = "m = | c4 _ - e4 |";
        let b = first_bar(src);
        assert_eq!(b.res, 1);
        assert!(matches!(&b.steps[0], Step::Start { len: 2, .. }));
        assert_eq!(b.steps[1], Step::Hold);
        assert_eq!(b.steps[2], Step::Rest);
        assert!(b.editable);
        let span = b.span.unwrap();
        assert_eq!(&src[span.start..span.end], "c4 _ - e4");
    }

    #[test]
    fn notes_get_explicit_octaves() {
        let b = first_bar("m = | c4 e g c5 |");
        let texts: Vec<String> = b
            .steps
            .iter()
            .filter_map(|s| match s {
                Step::Start { text, .. } => text.clone(),
                _ => None,
            })
            .collect();
        assert_eq!(texts, ["c4", "e4", "g4", "c5"]);
        assert_eq!(explicit_octaves("p:dó", &[48]), "p:dó3");
        assert_eq!(explicit_octaves("eb'", &[63]), "eb4'");
        assert_eq!(explicit_octaves("c4+e+g", &[60, 64, 67]), "c4+e4+g4");
        assert_eq!(explicit_octaves("b#", &[60]), "b#3");
    }

    #[test]
    fn repeats_and_motifs_are_not_editable() {
        let b = first_bar("m = | [e3 d3 c3 d3]x4 |");
        assert!(!b.editable);
        assert_eq!(b.steps.len(), 16, "but they are drawn");
        assert!(!first_bar("motif r = [e3 d3]\nm = | r r r r |").editable);
        assert!(first_bar("m = | [e3 d3] c3 d3 e3 |").editable);
        // A row's notes are drawn by name, not as `o`.
        let b = first_bar("row s = 1 4\nm = | o o - - | from s octave 3");
        let names: Vec<_> = b
            .steps
            .iter()
            .filter_map(|s| match s {
                Step::Start { text, .. } => text.clone(),
                _ => None,
            })
            .collect();
        assert_eq!(names, ["c#3", "e3"]);
    }

    #[test]
    fn choices_and_chord_shares_are_not_editable() {
        assert!(!first_bar("hh = {cc|co} c c c").editable);
        assert!(!first_bar("chords = | C G |").editable);
        assert!(first_bar("bd = | - |").editable);
        let src = "bd = | - x2 | b - b - |";
        let c = compile(src);
        let g = grid(&c.piece, src, 8);
        assert!(!g[0].bars[0].editable && !g[0].bars[1].editable && g[0].bars[2].editable);
        let src = "bd = | b - b - | % |";
        let c = compile(src);
        let g = grid(&c.piece, src, 8);
        assert!(g[0].bars[0].editable && !g[0].bars[1].editable);
    }

    #[test]
    fn bars_follow_the_playing_order() {
        let src = "m = |: c4 _ _ _ :| d4 _ _ _ |";
        let c = compile(src);
        let g = grid(&c.piece, src, 8);
        let positions: Vec<i64> = g[0].bars.iter().map(|b| b.position).collect();
        assert_eq!(positions, [1, 2, 3]);
    }
}
