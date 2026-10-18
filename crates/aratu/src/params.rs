//! The parameters Aratu knows, with the range, unit and scale its knobs use.
//!
//! The effects', instruments' and master bus's are Manguebyte's own, with
//! its identifiers, ranges, units and defaults (crates/manguebyte-web's
//! README); Aratu's own are the part's volume (Manguebyte's gain is in
//! decibels), the stand-in's envelope, and the reverb, which is the send to
//! Manguebyte's shared reverb.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scale {
    Linear,
    /// Equal knob turns multiply the value (frequencies, times).
    Log,
    /// Whole numbers only: a choice (a wave, a shape) or a count.
    Steps,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ParamInfo {
    pub id: &'static str,
    pub min: f64,
    pub max: f64,
    pub default: f64,
    /// The unit values are kept in: "Hz", "ms", or "" for a plain number.
    pub unit: &'static str,
    pub scale: Scale,
}

const fn p(
    id: &'static str,
    min: f64,
    max: f64,
    default: f64,
    unit: &'static str,
    scale: Scale,
) -> ParamInfo {
    ParamInfo {
        id,
        min,
        max,
        default,
        unit,
        scale,
    }
}

pub const PARAMS: &[ParamInfo] = &[
    // The part.
    p("volume", 0.0, 1.0, 0.8, "", Scale::Linear),
    p("pan", -1.0, 1.0, 0.0, "", Scale::Linear),
    p("attack", 1.0, 2000.0, 5.0, "ms", Scale::Log),
    p("release", 5.0, 5000.0, 250.0, "ms", Scale::Log),
    p("reverb.mix", 0.0, 1.0, 0.3, "", Scale::Linear),
    // Manguebyte's effects, in the order its chain runs them.
    p("highpass.cutoff", 20.0, 20000.0, 200.0, "Hz", Scale::Log),
    p("highpass.resonance", 0.0, 1.0, 0.0, "", Scale::Linear),
    p("lowpass.cutoff", 20.0, 20000.0, 2000.0, "Hz", Scale::Log),
    p("lowpass.resonance", 0.0, 1.0, 0.2, "", Scale::Linear),
    p("eq.low_gain", -24.0, 24.0, 0.0, "dB", Scale::Linear),
    p("eq.mid_gain", -24.0, 24.0, 0.0, "dB", Scale::Linear),
    p("eq.high_gain", -24.0, 24.0, 0.0, "dB", Scale::Linear),
    p("eq.low_freq", 20.0, 1000.0, 120.0, "Hz", Scale::Log),
    p("eq.mid_freq", 100.0, 10000.0, 1000.0, "Hz", Scale::Log),
    p("eq.mid_q", 0.1, 10.0, 0.7, "", Scale::Log),
    p("eq.high_freq", 1000.0, 20000.0, 8000.0, "Hz", Scale::Log),
    p("drive.amount", 0.0, 36.0, 12.0, "dB", Scale::Linear),
    p("drive.output", -24.0, 12.0, -6.0, "dB", Scale::Linear),
    p("drive.mix", 0.0, 1.0, 1.0, "", Scale::Linear),
    p("drive.shape", 0.0, 3.0, 0.0, "", Scale::Steps),
    // The valve: Manguebyte's drive with its smooth curve.
    p("tube.drive", 0.0, 24.0, 8.0, "dB", Scale::Linear),
    p("tube.output", -24.0, 12.0, -4.0, "dB", Scale::Linear),
    p("tube.mix", 0.0, 1.0, 1.0, "", Scale::Linear),
    p("bitcrush.bits", 1.0, 16.0, 8.0, "", Scale::Linear),
    p("bitcrush.rate", 100.0, 48000.0, 48000.0, "Hz", Scale::Log),
    p("bitcrush.mix", 0.0, 1.0, 1.0, "", Scale::Linear),
    p(
        "compressor.threshold",
        -60.0,
        0.0,
        -18.0,
        "dB",
        Scale::Linear,
    ),
    p("compressor.ratio", 1.0, 20.0, 4.0, "", Scale::Log),
    p("compressor.attack", 0.1, 200.0, 10.0, "ms", Scale::Log),
    p("compressor.release", 5.0, 2000.0, 120.0, "ms", Scale::Log),
    p("compressor.knee", 0.0, 24.0, 6.0, "dB", Scale::Linear),
    p("compressor.makeup", 0.0, 24.0, 0.0, "dB", Scale::Linear),
    p("compressor.mix", 0.0, 1.0, 1.0, "", Scale::Linear),
    p("chorus.mix", 0.0, 1.0, 0.5, "", Scale::Linear),
    p("chorus.rate", 0.05, 10.0, 0.8, "Hz", Scale::Log),
    p("chorus.depth", 0.0, 10.0, 2.5, "ms", Scale::Linear),
    p("chorus.delay", 1.0, 40.0, 15.0, "ms", Scale::Linear),
    p("chorus.voices", 1.0, 8.0, 3.0, "", Scale::Steps),
    p("chorus.spread", 0.0, 1.0, 1.0, "", Scale::Linear),
    p("flanger.mix", 0.0, 1.0, 0.5, "", Scale::Linear),
    p("flanger.rate", 0.02, 10.0, 0.25, "Hz", Scale::Log),
    p("flanger.depth", 0.0, 5.0, 1.5, "ms", Scale::Linear),
    p("flanger.feedback", -0.95, 0.95, 0.7, "", Scale::Linear),
    p("flanger.delay", 0.5, 10.0, 2.0, "ms", Scale::Linear),
    p("phaser.mix", 0.0, 1.0, 0.5, "", Scale::Linear),
    p("phaser.rate", 0.02, 10.0, 0.5, "Hz", Scale::Log),
    p("phaser.depth", 0.0, 6.0, 2.0, "", Scale::Linear),
    p("phaser.center", 100.0, 10000.0, 1000.0, "Hz", Scale::Log),
    p("phaser.feedback", -0.95, 0.95, 0.3, "", Scale::Linear),
    p("phaser.stages", 2.0, 12.0, 4.0, "", Scale::Steps),
    p("phaser.spread", 0.0, 1.0, 0.0, "", Scale::Linear),
    p("tremolo.rate", 0.1, 30.0, 5.0, "Hz", Scale::Log),
    p("tremolo.depth", 0.0, 1.0, 0.5, "", Scale::Linear),
    p("tremolo.shape", 0.0, 4.0, 0.0, "", Scale::Steps),
    p("tremolo.sync", 0.0, 16.0, 0.0, "", Scale::Linear),
    p("autopan.rate", 0.05, 20.0, 0.5, "Hz", Scale::Log),
    p("autopan.depth", 0.0, 1.0, 1.0, "", Scale::Linear),
    p("autopan.shape", 0.0, 4.0, 0.0, "", Scale::Steps),
    p("autopan.sync", 0.0, 16.0, 0.0, "", Scale::Linear),
    p("delay.time", 0.001, 4.0, 0.375, "s", Scale::Log),
    p("delay.feedback", 0.0, 0.95, 0.35, "", Scale::Linear),
    p("delay.mix", 0.0, 1.0, 0.3, "", Scale::Linear),
    p("delay.damping", 0.0, 1.0, 0.3, "", Scale::Linear),
    p("delay.pingpong", 0.0, 1.0, 0.0, "", Scale::Linear),
    p("delay.sync", 0.0, 16.0, 0.0, "", Scale::Linear),
    // Manguebyte's instruments.
    p("synth.wave", 0.0, 3.0, 0.0, "", Scale::Steps),
    p("synth.detune", 0.0, 50.0, 9.0, "", Scale::Linear),
    p("synth.sub", 0.0, 1.0, 0.35, "", Scale::Linear),
    p("synth.cutoff", 40.0, 16000.0, 900.0, "Hz", Scale::Log),
    p("synth.resonance", 0.0, 1.0, 0.25, "", Scale::Linear),
    p("synth.env_amount", 0.0, 6.0, 2.5, "", Scale::Linear),
    p("synth.filter_decay", 0.01, 4.0, 0.35, "s", Scale::Log),
    p("synth.decay", 0.01, 8.0, 0.6, "s", Scale::Log),
    p("synth.sustain", 0.0, 1.0, 0.5, "", Scale::Linear),
    p("plucked.decay", 0.1, 20.0, 3.0, "s", Scale::Log),
    p(
        "plucked.brightness",
        100.0,
        20000.0,
        2800.0,
        "Hz",
        Scale::Log,
    ),
    p("plucked.position", 0.02, 0.5, 0.16, "", Scale::Linear),
    p("plucked.tone", 200.0, 20000.0, 6500.0, "Hz", Scale::Log),
    p("plucked.release", 0.01, 2.0, 0.18, "s", Scale::Log),
    p("fm.brightness", 0.0, 2.0, 1.0, "", Scale::Linear),
    p("fm.decay", 0.25, 4.0, 1.0, "", Scale::Log),
    p("fm.release", 0.25, 4.0, 1.0, "", Scale::Log),
    p("organ.rotary", 0.0, 2.0, 1.0, "", Scale::Steps),
    p("organ.percussion", 0.0, 4.0, 3.0, "", Scale::Steps),
    p("organ.click", 0.0, 1.0, 0.5, "", Scale::Linear),
    p("organ.leakage", 0.0, 1.0, 0.4, "", Scale::Linear),
    p("organ.velocity", 0.0, 1.0, 0.5, "", Scale::Linear),
    p("organ.speaker", 0.0, 1.0, 1.0, "", Scale::Linear),
    p("acid.cutoff", 30.0, 8000.0, 320.0, "Hz", Scale::Log),
    p("acid.resonance", 0.0, 1.0, 0.72, "", Scale::Linear),
    p("acid.env_mod", 0.0, 1.0, 0.6, "", Scale::Linear),
    p("acid.decay", 0.05, 4.0, 0.5, "s", Scale::Log),
    p("acid.accent", 0.0, 1.0, 0.7, "", Scale::Linear),
    p("acid.slide", 0.005, 1.0, 0.06, "s", Scale::Log),
    p("acid.drive", -12.0, 24.0, 6.0, "dB", Scale::Linear),
    p("kick_bass.tune", -12.0, 12.0, 0.0, "", Scale::Linear),
    p("kick_bass.decay", 0.05, 20.0, 2.5, "s", Scale::Log),
    p("kick_bass.sweep", 0.0, 48.0, 12.0, "", Scale::Linear),
    p("kick_bass.drive", 0.0, 1.0, 0.3, "", Scale::Linear),
    p("kick_bass.glide", 0.001, 2.0, 0.08, "s", Scale::Log),
    p("kit.tune", -12.0, 12.0, 0.0, "", Scale::Linear),
    p("kit.decay", 0.25, 4.0, 1.0, "", Scale::Log),
    p("analog.tone", 0.0, 1.0, 0.5, "", Scale::Linear),
    p("analog.snap", 0.0, 1.0, 0.5, "", Scale::Linear),
    p("analog.kick_decay", 0.25, 8.0, 1.0, "", Scale::Log),
    // Manguebyte's master bus, for the whole mix.
    p("master.gain", -60.0, 12.0, 0.0, "dB", Scale::Linear),
    p("master.reverb.level", -60.0, 6.0, 0.0, "dB", Scale::Linear),
    p("master.reverb.decay", 0.2, 10.0, 2.2, "s", Scale::Log),
    p("master.reverb.size", 0.0, 1.0, 0.7, "", Scale::Linear),
    p("master.reverb.damping", 0.0, 1.0, 0.5, "", Scale::Linear),
    p("master.glue.on", 0.0, 1.0, 0.0, "", Scale::Steps),
    p(
        "master.glue.threshold",
        -40.0,
        0.0,
        -12.0,
        "dB",
        Scale::Linear,
    ),
    p("master.glue.ratio", 1.0, 10.0, 2.0, "", Scale::Log),
    p("master.glue.makeup", 0.0, 18.0, 0.0, "dB", Scale::Linear),
    p(
        "master.limiter.ceiling",
        -12.0,
        -0.3,
        -0.3,
        "dB",
        Scale::Linear,
    ),
    p(
        "master.limiter.release",
        10.0,
        1000.0,
        150.0,
        "ms",
        Scale::Log,
    ),
];

pub fn lookup(id: &str) -> Option<&'static ParamInfo> {
    PARAMS.iter().find(|p| p.id.eq_ignore_ascii_case(id))
}

/// The known parameter nearest to a misspelt one: `lowpass.cutof`.
pub fn closest(id: &str) -> Option<&'static str> {
    crate::suggest::closest(id, PARAMS.iter().map(|p| p.id))
}

/// Converts a written value to the parameter's own unit: `1.2 kHz` to 1200
/// Hz, `0.25 s` to 250 ms, `40%` to 0.4 for a plain 0-to-1 parameter.
/// Returns `None` for a unit that does not fit the parameter.
pub fn normalize(number: f64, unit: Option<&str>, info: &ParamInfo) -> Option<f64> {
    let unit = unit.map(str::to_lowercase);
    match (unit.as_deref(), info.unit) {
        (None, _) => Some(number),
        (Some(u), own) if u == own.to_lowercase() => Some(number),
        (Some("ms"), "s") => Some(number / 1000.0),
        (Some("hz"), "Hz") => Some(number),
        (Some("khz"), "Hz") => Some(number * 1000.0),
        (Some("ms"), "ms") => Some(number),
        (Some("s"), "ms") => Some(number * 1000.0),
        (Some("%"), "") => Some(number / 100.0),
        _ => None,
    }
}

/// Position of a value on its knob, 0 to 1.
pub fn to_knob(value: f64, info: &ParamInfo) -> f64 {
    let v = value.clamp(info.min, info.max);
    let t = match info.scale {
        Scale::Linear | Scale::Steps => (v - info.min) / (info.max - info.min),
        Scale::Log => (v / info.min).ln() / (info.max / info.min).ln(),
    };
    t.clamp(0.0, 1.0)
}

/// The value at a knob position, 0 to 1.
pub fn from_knob(t: f64, info: &ParamInfo) -> f64 {
    let t = t.clamp(0.0, 1.0);
    match info.scale {
        Scale::Linear => info.min + t * (info.max - info.min),
        Scale::Steps => (info.min + t * (info.max - info.min)).round(),
        Scale::Log => info.min * (info.max / info.min).powf(t),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn units_and_knobs() {
        let cutoff = lookup("lowpass.cutoff").unwrap();
        assert_eq!(normalize(1.2, Some("kHz"), cutoff), Some(1200.0));
        assert_eq!(normalize(800.0, Some("Hz"), cutoff), Some(800.0));
        assert_eq!(normalize(800.0, Some("ms"), cutoff), None);
        let mix = lookup("reverb.mix").unwrap();
        assert_eq!(normalize(40.0, Some("%"), mix), Some(0.4));
        // A log knob puts 632 Hz (the geometric middle of 20..20000) at 0.5.
        let mid = from_knob(0.5, cutoff);
        assert!((mid - 632.455).abs() < 0.01);
        assert!((to_knob(mid, cutoff) - 0.5).abs() < 1e-12);
        assert!((to_knob(0.42, mix) - 0.42).abs() < 1e-12);
    }
}
