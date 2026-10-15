//! From beats to seconds: the tempo as it changes, ramps included, and the
//! clock of each bar.
//!
//! A ritardando or accelerando changes the tempo evenly beat by beat, as the
//! score reads. With the tempo `T(x) = T0 + k (x - x0)` in beats per minute
//! at beat `x`, the time from beat `a` to beat `b` is the integral of
//! `60 / T(x)`, which has a closed form:
//!
//! ```text
//! t = 60 (b - a) / T0                 when k = 0
//! t = (60 / k) ln(T(b) / T(a))        otherwise
//! ```
//!
//! so every note lands on its exact time instead of on a stepped tempo.

use crate::ast::TempoChange;
use crate::ratio::Ratio;

/// A tempo ramp between two global beat positions.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Ramp {
    start: Ratio,
    end: Ratio,
    from: f64,
    to: f64,
}

/// The tempo at any beat from the last change on.
#[derive(Clone, Debug, PartialEq)]
pub struct Tempo {
    /// The tempo outside the ramp (before it starts it is `ramp.from`).
    bpm: f64,
    ramp: Option<Ramp>,
    /// The tempo before the last rit or accel, for `a tempo`.
    before_ramp: f64,
    /// The first tempo of the piece, for `tempo primo`.
    primo: f64,
}

impl Tempo {
    pub fn new(bpm: f64) -> Tempo {
        Tempo {
            bpm,
            ramp: None,
            before_ramp: bpm,
            primo: bpm,
        }
    }

    /// The tempo at global beat `x`.
    pub fn bpm_at(&self, x: Ratio) -> f64 {
        match self.ramp {
            Some(r) if x <= r.start => r.from,
            Some(r) if x < r.end => {
                let t = ((x - r.start) / (r.end - r.start)).to_f64();
                r.from + (r.to - r.from) * t
            }
            Some(r) => r.to,
            None => self.bpm,
        }
    }

    /// Applies a change at global beat `at`; a ramp lasts `length` beats.
    pub fn change(&mut self, change: TempoChange, at: Ratio, length: Ratio) {
        let now = self.bpm_at(at);
        match change {
            TempoChange::Set(bpm) => {
                self.bpm = bpm;
                self.ramp = None;
            }
            TempoChange::Ramp { from, to, .. } => {
                let from = from.unwrap_or(now);
                self.before_ramp = match self.ramp {
                    Some(_) => self.before_ramp,
                    None => now,
                };
                if length.is_zero() {
                    self.bpm = to;
                    self.ramp = None;
                } else {
                    self.ramp = Some(Ramp {
                        start: at,
                        end: at + length,
                        from,
                        to,
                    });
                    self.bpm = to;
                }
            }
            TempoChange::ATempo => {
                self.bpm = self.before_ramp;
                self.ramp = None;
            }
            TempoChange::Primo => {
                self.bpm = self.primo;
                self.ramp = None;
            }
        }
    }

    /// Seconds from global beat `a` to `b` (`a <= b`).
    pub fn seconds(&self, a: Ratio, b: Ratio) -> f64 {
        if b <= a {
            return 0.0;
        }
        let Some(r) = self.ramp else {
            return 60.0 * (b - a).to_f64() / self.bpm;
        };
        let mut total = 0.0;
        // Before the ramp, the ramp's start tempo; inside it, the curve;
        // after it, the end tempo.
        if a < r.start {
            let e = b.min(r.start);
            total += 60.0 * (e - a).to_f64() / r.from;
        }
        let s = a.max(r.start);
        let e = b.min(r.end);
        if s < e {
            let k = (r.to - r.from) / (r.end - r.start).to_f64();
            let ts = self.bpm_at(s);
            let te = self.bpm_at(e);
            total += if k.abs() < 1e-12 {
                60.0 * (e - s).to_f64() / ts
            } else {
                60.0 / k * (te / ts).ln()
            };
        }
        if b > r.end {
            let s = a.max(r.end);
            total += 60.0 * (b - s).to_f64() / r.to;
        }
        total
    }
}

/// When the beats of one bar sound.
#[derive(Clone, Debug, PartialEq)]
pub struct BarClock {
    pub bar: i64,
    /// Global beat position of the bar's start.
    pub start_beat: Ratio,
    pub beats: Ratio,
    pub start_time: f64,
    tempo: Tempo,
    /// Spans held by a fermata, as offsets in the bar: they last twice as long.
    fermatas: Vec<(Ratio, Ratio)>,
}

impl BarClock {
    pub fn new(
        bar: i64,
        start_beat: Ratio,
        beats: Ratio,
        start_time: f64,
        tempo: Tempo,
        mut fermatas: Vec<(Ratio, Ratio)>,
    ) -> BarClock {
        // Merge overlapping spans so a beat is never stretched twice.
        fermatas.sort_by(|a, b| a.0.cmp(&b.0));
        let mut merged: Vec<(Ratio, Ratio)> = Vec::new();
        for (s, e) in fermatas {
            let (s, e) = (s.max(Ratio::ZERO), e.min(beats));
            if s >= e {
                continue;
            }
            match merged.last_mut() {
                Some(last) if s <= last.1 => last.1 = last.1.max(e),
                _ => merged.push((s, e)),
            }
        }
        BarClock {
            bar,
            start_beat,
            beats,
            start_time,
            tempo,
            fermatas: merged,
        }
    }

    /// The time of an offset in the bar, in seconds from the piece's start.
    pub fn time(&self, offset: Ratio) -> f64 {
        let offset = offset.max(Ratio::ZERO);
        let mut t = self.start_time;
        let mut at = Ratio::ZERO;
        let g = |x: Ratio| self.start_beat + x;
        for &(s, e) in &self.fermatas {
            if s >= offset {
                break;
            }
            t += self.tempo.seconds(g(at), g(s));
            let e2 = e.min(offset);
            t += 2.0 * self.tempo.seconds(g(s), g(e2));
            at = e2;
        }
        if at < offset {
            t += self.tempo.seconds(g(at), g(offset));
        }
        t
    }

    pub fn end_time(&self) -> f64 {
        self.time(self.beats)
    }

    /// The tempo at an offset in the bar.
    pub fn bpm_at(&self, offset: Ratio) -> f64 {
        self.tempo.bpm_at(self.start_beat + offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn r(n: i64, d: i64) -> Ratio {
        Ratio::new(n, d)
    }

    fn close(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-9, "{a} != {b}");
    }

    #[test]
    fn steady_tempo() {
        let t = Tempo::new(120.0);
        close(t.seconds(Ratio::ZERO, Ratio::int(4)), 2.0);
        close(t.seconds(Ratio::ZERO, r(1, 3)), 1.0 / 6.0);
    }

    #[test]
    fn a_ramp_matches_the_closed_form_and_a_fine_sum() {
        // rit from 120 to 60 over 8 beats starting at beat 4.
        let mut t = Tempo::new(120.0);
        t.change(
            TempoChange::Ramp {
                from: None,
                to: 60.0,
                bars: 2,
            },
            Ratio::int(4),
            Ratio::int(8),
        );
        close(t.bpm_at(Ratio::int(8)), 90.0);
        // k = -7.5 bpm per beat: t = 60/k * ln(60/120) = 5.545...
        let exact = 60.0 / -7.5 * (60.0f64 / 120.0).ln();
        close(t.seconds(Ratio::int(4), Ratio::int(12)), exact);
        // A Riemann sum with a million steps agrees to the microsecond.
        let n = 1_000_000;
        let mut sum = 0.0;
        for i in 0..n {
            let x = 4.0 + 8.0 * (i as f64 + 0.5) / n as f64;
            let bpm = 120.0 - 7.5 * (x - 4.0);
            sum += 60.0 / bpm * 8.0 / n as f64;
        }
        assert!((sum - exact).abs() < 1e-6);
        // Before the ramp at 120, after it at 60.
        close(t.seconds(Ratio::ZERO, Ratio::int(4)), 2.0);
        close(t.seconds(Ratio::int(12), Ratio::int(16)), 4.0);
        // Split integrals add up exactly.
        close(
            t.seconds(Ratio::int(2), r(29, 3)) + t.seconds(r(29, 3), Ratio::int(14)),
            t.seconds(Ratio::int(2), Ratio::int(14)),
        );
    }

    #[test]
    fn a_tempo_and_tempo_primo() {
        let mut t = Tempo::new(96.0);
        t.change(TempoChange::Set(130.0), Ratio::int(4), Ratio::ZERO);
        t.change(
            TempoChange::Ramp {
                from: None,
                to: 80.0,
                bars: 2,
            },
            Ratio::int(8),
            Ratio::int(8),
        );
        t.change(TempoChange::ATempo, Ratio::int(16), Ratio::ZERO);
        close(t.bpm_at(Ratio::int(20)), 130.0);
        t.change(TempoChange::Primo, Ratio::int(20), Ratio::ZERO);
        close(t.bpm_at(Ratio::int(24)), 96.0);
    }

    #[test]
    fn a_fermata_doubles_its_span_only() {
        let t = Tempo::new(60.0);
        // A 4/4 bar at 60: one second per beat; a fermata on beat 3.
        let c = BarClock::new(
            1,
            Ratio::int(0),
            Ratio::int(4),
            10.0,
            t,
            vec![(Ratio::int(2), Ratio::int(3))],
        );
        close(c.time(Ratio::int(2)), 12.0);
        close(c.time(r(5, 2)), 13.0);
        close(c.time(Ratio::int(3)), 14.0);
        close(c.end_time(), 15.0);
    }
}
