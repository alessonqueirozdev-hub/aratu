//! The parameters Aratu knows, with the range, unit and scale its knobs use.
//!
//! This table stands in for the parameter metadata Manguebyte will export as
//! data; it is keyed by the same string ids (`reverb.mix`,
//! `lowpass.cutoff`), so switching to Manguebyte's description is a lookup.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scale {
    Linear,
    /// Equal knob turns multiply the value (frequencies, times).
    Log,
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
    p("volume", 0.0, 1.0, 0.8, "", Scale::Linear),
    p("pan", -1.0, 1.0, 0.0, "", Scale::Linear),
    p("attack", 1.0, 2000.0, 5.0, "ms", Scale::Log),
    p("release", 5.0, 5000.0, 250.0, "ms", Scale::Log),
    p("lowpass.cutoff", 20.0, 20000.0, 2000.0, "Hz", Scale::Log),
    p("lowpass.resonance", 0.0, 1.0, 0.2, "", Scale::Linear),
    p("highpass.cutoff", 20.0, 20000.0, 200.0, "Hz", Scale::Log),
    p("highpass.resonance", 0.0, 1.0, 0.2, "", Scale::Linear),
    p("reverb.mix", 0.0, 1.0, 0.3, "", Scale::Linear),
    p("reverb.size", 0.0, 1.0, 0.5, "", Scale::Linear),
    p("delay.time", 10.0, 2000.0, 250.0, "ms", Scale::Log),
    p("delay.feedback", 0.0, 0.95, 0.35, "", Scale::Linear),
    p("delay.mix", 0.0, 1.0, 0.25, "", Scale::Linear),
    p("drive.amount", 0.0, 1.0, 0.3, "", Scale::Linear),
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
        Scale::Linear => (v - info.min) / (info.max - info.min),
        Scale::Log => (v / info.min).ln() / (info.max / info.min).ln(),
    };
    t.clamp(0.0, 1.0)
}

/// The value at a knob position, 0 to 1.
pub fn from_knob(t: f64, info: &ParamInfo) -> f64 {
    let t = t.clamp(0.0, 1.0);
    match info.scale {
        Scale::Linear => info.min + t * (info.max - info.min),
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
