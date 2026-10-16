//! Aratu in the browser.
//!
//! A handful of exported functions with plain numbers and bytes, and JSON for
//! everything structured, so the module needs no binding generator. The
//! editor writes the source into memory from [`aratu_alloc`], calls
//! [`aratu_compile`], and reads the answer from [`aratu_out_ptr`] and
//! [`aratu_out_len`]. Positions in the answers are in UTF-16 code units, as
//! JavaScript strings count them.

use aratu::diag::{Lang, Severity};
use aratu::model::{At, Piece};
use aratu::ratio::Ratio;
use aratu::schedule::{EventKind, Options, render};
use aratu::{Compiled, compile, params};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Write as _;

pub mod json;
use json::Json;

/// A compiled piece kept for rendering.
pub struct Program {
    pub compiled: Compiled,
    /// The source, for the rhythm map.
    pub src: String,
    /// For each byte offset of the source, its UTF-16 offset.
    utf16: Vec<u32>,
}

impl Program {
    fn pos(&self, byte: usize) -> u32 {
        self.utf16[byte.min(self.utf16.len() - 1)]
    }
}

fn utf16_table(src: &str) -> Vec<u32> {
    let mut table = vec![0u32; src.len() + 1];
    let mut n = 0u32;
    for (i, c) in src.char_indices() {
        for b in 0..c.len_utf8() {
            table[i + b] = n;
        }
        n += c.len_utf16() as u32;
    }
    table[src.len()] = n;
    table
}

thread_local! {
    static PROGRAMS: RefCell<HashMap<u32, Program>> = RefCell::new(HashMap::new());
    static NEXT: RefCell<u32> = const { RefCell::new(1) };
    static OUT: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
}

/// What kind of instrument a part sounds like by default.
fn instrument(part: &aratu::model::Part) -> &'static str {
    use aratu::ast::LineKind;
    if part.lines.iter().any(|l| l.kind == LineKind::Strum) {
        "guitar"
    } else if !part.lines.is_empty() && part.lines.iter().all(|l| l.kind == LineKind::Drums) {
        "drums"
    } else if part.lines.iter().any(|l| l.kind == LineKind::Chords) {
        "keys"
    } else {
        "synth"
    }
}

/// Compiles `src`, keeps the program, and answers in JSON.
pub fn compile_json(src: &str, lang: Lang) -> (u32, String) {
    let compiled = compile(src);
    let program = Program {
        utf16: utf16_table(src),
        compiled,
        src: src.to_string(),
    };
    let handle = NEXT.with(|n| {
        let mut n = n.borrow_mut();
        let h = *n;
        *n += 1;
        h
    });
    let text = describe(handle, &program, src, lang);
    PROGRAMS.with(|p| p.borrow_mut().insert(handle, program));
    (handle, text)
}

fn describe(handle: u32, program: &Program, src: &str, lang: Lang) -> String {
    let c = &program.compiled;
    let piece: &Piece = &c.piece;
    let mut j = Json::new();
    j.open_obj();
    j.key("handle").num(handle as f64);
    j.key("ok").bool(!c.has_errors());
    j.key("diagnostics").open_arr();
    for d in &c.diagnostics {
        j.open_obj();
        j.key("severity").str(match d.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
        });
        j.key("from").num(program.pos(d.span.start) as f64);
        j.key("to")
            .num(program.pos(d.span.end.max(d.span.start)) as f64);
        let (line, _) = aratu::span::line_col(src, d.span.start);
        j.key("line").num(line as f64);
        j.key("message").str(&d.text(lang));
        j.close_obj();
    }
    j.close_arr();

    j.key("live").open_arr();
    for cmd in &piece.commands {
        if matches!(cmd.at, At::Live | At::Now) {
            j.open_obj();
            j.key("key").str(&cmd.key);
            j.key("now").bool(cmd.at == At::Now);
            j.close_obj();
        }
    }
    j.close_arr();

    j.key("tempo").num(piece.tempo);
    j.key("meter").str(&piece.meter.to_string());
    j.key("pickup").num(piece.pickup.to_f64());
    // One time round: the longest line, for the book's rhythm grid.
    let length = piece
        .all_parts()
        .flat_map(|p| p.lines.iter())
        .map(|l| l.order.len())
        .max()
        .unwrap_or(1)
        .max(1);
    j.key("length").num(length as f64);

    // Melody lists, for the stage's buttons, and rows, for the matrix.
    j.key("lists").open_arr();
    for (name, count) in &piece.lists {
        j.open_obj();
        j.key("name").str(name);
        j.key("count").num(*count as f64);
        j.close_obj();
    }
    j.close_arr();
    j.key("rows").open_arr();
    for (name, pitches) in &piece.rows {
        j.open_obj();
        j.key("name").str(name);
        j.key("pitches").open_arr();
        for p in pitches {
            j.num(*p as f64);
        }
        j.close_arr();
        j.close_obj();
    }
    j.close_arr();

    // Every part, from the piece, its patterns and its sections, once.
    let mut seen = Vec::new();
    j.key("parts").open_arr();
    for part in piece.all_parts() {
        if seen.contains(&part.name) {
            continue;
        }
        seen.push(part.name.clone());
        j.open_obj();
        j.key("name").str(&part.name);
        j.key("instrument").str(instrument(part));
        j.key("chain").open_arr();
        for u in &part.chain {
            j.str(u);
        }
        j.close_arr();
        j.key("params").open_arr();
        for p in &part.params {
            j.open_obj();
            j.key("id").str(&p.id);
            j.key("value").num(p.value);
            j.close_obj();
        }
        j.close_arr();
        j.close_obj();
    }
    j.close_arr();

    j.key("params").open_arr();
    for p in &piece.params {
        j.open_obj();
        j.key("id").str(&p.id);
        j.key("value").num(p.value);
        j.close_obj();
    }
    j.close_arr();

    // Parameters that move: a new version of the text leaves them where
    // they are instead of setting them back.
    j.key("moving").open_arr();
    for c in &piece.commands {
        if let aratu::model::Action::Param {
            part,
            id,
            motion: Some(_),
            ..
        } = &c.action
        {
            j.open_obj();
            j.key("part").str(part.as_deref().unwrap_or("all"));
            j.key("id").str(id);
            j.close_obj();
        }
    }
    j.close_arr();

    j.key("controls").open_arr();
    for ctl in controls(piece) {
        j.open_obj();
        j.key("id").str(&ctl.id);
        j.key("part").str(&ctl.part);
        j.key("from").num(program.pos(ctl.span.start) as f64);
        j.key("to").num(program.pos(ctl.span.end) as f64);
        j.key("value").num(ctl.value);
        j.key("factor").num(ctl.factor);
        j.key("min").num(ctl.min);
        j.key("max").num(ctl.max);
        j.key("unit").str(ctl.unit);
        j.key("log").bool(ctl.log);
        j.key("timed").bool(ctl.timed);
        j.close_obj();
    }
    j.close_arr();
    j.close_obj();
    j.finish()
}

/// A number in the text that the editor draws as a knob, shaped by the
/// parameter it sets.
pub struct Control {
    pub id: String,
    /// The part it belongs to, or "all".
    pub part: String,
    /// Where the number is written.
    pub span: aratu::span::Span,
    /// The value in the parameter's own unit.
    pub value: f64,
    /// The written number per unit of `value` (0.001 for kHz, 100 for %).
    pub factor: f64,
    pub min: f64,
    pub max: f64,
    pub unit: &'static str,
    pub log: bool,
    /// Set at a bar (`at bar 16: ...`), so turning it changes the text only.
    pub timed: bool,
}

/// Every number in the piece that sets a known parameter, plus the tempo.
pub fn controls(piece: &Piece) -> Vec<Control> {
    use aratu::model::{Action, ParamSet};
    let mut out = Vec::new();
    let mut push = |part: &str, p: &ParamSet, timed: bool| {
        if let Some(info) = params::lookup(&p.id) {
            out.push(Control {
                id: info.id.to_string(),
                part: part.to_string(),
                span: p.number_span,
                value: p.value,
                factor: p.factor,
                min: info.min,
                max: info.max,
                unit: info.unit,
                log: info.scale == params::Scale::Log,
                timed,
            });
        }
    };
    for part in piece.all_parts() {
        for p in &part.params {
            push(&part.name, p, false);
        }
    }
    for p in &piece.params {
        push("all", p, false);
    }
    for c in &piece.commands {
        if let Action::Param {
            part,
            id,
            value,
            number_span,
            factor,
            motion,
        } = &c.action
        {
            let part = part.as_deref().unwrap_or("all");
            let set = ParamSet {
                id: id.clone(),
                value: *value,
                number_span: *number_span,
                factor: *factor,
            };
            push(part, &set, true);
            // Where it moves to has a knob too.
            if let Some(m) = motion {
                let to = ParamSet {
                    id: id.clone(),
                    value: m.to,
                    number_span: m.to_span,
                    factor: m.to_factor,
                };
                push(part, &to, true);
            }
        }
    }
    if let Some(span) = piece.tempo_span {
        out.push(Control {
            id: "tempo".into(),
            part: String::new(),
            span,
            value: piece.tempo,
            factor: 1.0,
            min: 40.0,
            max: 240.0,
            unit: "bpm",
            log: false,
            timed: false,
        });
    }
    out.sort_by_key(|c| c.span.start);
    out
}

/// Renders bars `from..to` of a kept program, in JSON, bar `from` from its
/// beat `from_beat` (a change with `now:` coming in inside a bar).
///
/// `anchors` holds one live line per text line, `bar<TAB>key`.
pub fn render_json(
    handle: u32,
    from: i64,
    from_beat: i64,
    to: i64,
    default_anchor: i64,
    seed: u64,
    anchors: &str,
) -> String {
    let mut opts = Options {
        default_anchor,
        random_seed: seed,
        from_beat: Ratio::int(from_beat.max(0)),
        ..Options::default()
    };
    for l in anchors.lines() {
        if let Some((bar, key)) = l.split_once('\t') {
            if let Ok(b) = bar.parse() {
                opts.anchors.insert(key.to_string(), b);
            }
        }
    }
    PROGRAMS.with(|p| {
        let p = p.borrow();
        let Some(program) = p.get(&handle) else {
            return "{\"events\":[],\"bars\":[]}".to_string();
        };
        let r = render(&program.compiled.piece, &opts, from, to);
        let mut j = Json::new();
        j.open_obj();
        j.key("events").open_arr();
        for e in &r.events {
            j.open_obj();
            j.key("bar").num(e.bar as f64);
            j.key("beat").num(e.beat.to_f64());
            j.key("time").num(e.time);
            j.key("dur").num(e.duration);
            j.key("part").str(&e.part);
            j.key("line").str(&e.line);
            j.key("vel").num(e.velocity as f64);
            if e.accent {
                j.key("acc").num(1.0);
            }
            match &e.kind {
                EventKind::Note(n) => {
                    j.key("note").num(*n as f64);
                }
                EventKind::Drum(d) => {
                    j.key("drum").str(d.name());
                }
                EventKind::Param { id, value, glide } => {
                    j.key("param").str(id);
                    j.key("value").num(*value);
                    if *glide > 0.0 {
                        j.key("glide").num(*glide);
                    }
                }
            }
            j.key("from").num(program.pos(e.span.start) as f64);
            j.key("to").num(program.pos(e.span.end) as f64);
            if let Some(a) = e.also {
                j.key("cfrom").num(program.pos(a.start) as f64);
                j.key("cto").num(program.pos(a.end) as f64);
            }
            j.close_obj();
        }
        j.close_arr();
        j.key("bars").open_arr();
        for b in &r.bars {
            j.open_obj();
            j.key("bar").num(b.bar as f64);
            j.key("start").num(b.start_time);
            j.key("end").num(b.end_time);
            j.key("beats").num(b.beats.to_f64());
            j.key("bpm").num(b.bpm);
            j.key("beatTimes").open_arr();
            for t in &b.beat_times {
                j.num(*t);
            }
            j.close_arr();
            let mut m = String::new();
            let _ = write!(m, "{}", b.meter);
            j.key("meter").str(&m);
            j.close_obj();
        }
        j.close_arr();
        j.close_obj();
        j.finish()
    })
}

/// The rhythm map of a kept program, in JSON: every line's bars as steps.
/// A step is 0 (rest), 1 (the sound goes on) or `[from, to, len, text]`
/// for a sound starting there (`text` only for notes).
pub fn grid_json(handle: u32, max_bars: usize) -> String {
    use aratu::ast::LineKind;
    use aratu::grid::{Step, grid};
    PROGRAMS.with(|p| {
        let p = p.borrow();
        let Some(program) = p.get(&handle) else {
            return "{\"lines\":[]}".to_string();
        };
        let lines = grid(&program.compiled.piece, &program.src, max_bars);
        let mut j = Json::new();
        j.open_obj();
        j.key("lines").open_arr();
        for line in &lines {
            j.open_obj();
            j.key("label").str(&line.label);
            j.key("kind").str(match line.kind {
                LineKind::Drums => "drums",
                LineKind::Strum => "strum",
                LineKind::Pitched => "pitched",
                LineKind::Chords => "chords",
            });
            j.key("bars").open_arr();
            for bar in &line.bars {
                j.open_obj();
                j.key("pos").num(bar.position as f64);
                match bar.span {
                    Some(s) => {
                        j.key("from").num(program.pos(s.start) as f64);
                        j.key("to").num(program.pos(s.end) as f64);
                    }
                    None => {
                        j.key("from").num(-1.0);
                        j.key("to").num(-1.0);
                    }
                }
                j.key("beats").num(bar.beats as f64);
                j.key("res").num(bar.res as f64);
                j.key("editable").bool(bar.editable);
                j.key("steps").open_arr();
                for s in &bar.steps {
                    match s {
                        Step::Rest => {
                            j.num(0.0);
                        }
                        Step::Hold => {
                            j.num(1.0);
                        }
                        Step::Start { span, len, text } => {
                            j.open_arr();
                            j.num(program.pos(span.start) as f64);
                            j.num(program.pos(span.end) as f64);
                            j.num(*len as f64);
                            if let Some(t) = text {
                                j.str(t);
                            }
                            j.close_arr();
                        }
                    }
                }
                j.close_arr();
                j.close_obj();
            }
            j.close_arr();
            j.close_obj();
        }
        j.close_arr();
        j.close_obj();
        j.finish()
    })
}

/// The rhythm map of a kept program (see [`grid_json`]); the JSON answer is
/// in the output buffer.
#[unsafe(no_mangle)]
pub extern "C" fn aratu_grid(handle: u32, max_bars: u32) {
    set_out(grid_json(handle, max_bars as usize));
}

/// Every voice a part can play, by family, named in `lang`, with an example
/// to hear it: `{"families":[{"code","name","voices":[{"code","name",
/// "sampled","demo"}]}],"aliases":{...}}`.
pub fn voices_json(lang: Lang) -> String {
    use aratu::voices::{ALIASES, Family, Source, VOICES};
    let mut j = Json::new();
    j.open_obj();
    j.key("families").open_arr();
    for f in Family::ALL {
        j.open_obj();
        j.key("code").str(f.code());
        j.key("name").str(f.name(lang));
        j.key("voices").open_arr();
        for v in VOICES.iter().filter(|v| v.family == f) {
            j.open_obj();
            j.key("code").str(v.code);
            j.key("name").str(v.name(lang));
            j.key("sampled").bool(v.source == Source::Sampled);
            j.key("demo").str(v.demo);
            j.close_obj();
        }
        j.close_arr();
        j.close_obj();
    }
    j.close_arr();
    j.key("aliases").open_obj();
    for (alias, to) in ALIASES {
        j.key(alias).str(to);
    }
    j.close_obj();
    j.close_obj();
    j.finish()
}

/// The voice catalogue (see [`voices_json`]) in the output buffer.
#[unsafe(no_mangle)]
pub extern "C" fn aratu_voices(lang: u32) {
    let lang = match lang {
        1 => Lang::En,
        2 => Lang::Es,
        _ => Lang::Pt,
    };
    set_out(voices_json(lang));
}

pub fn release(handle: u32) {
    PROGRAMS.with(|p| p.borrow_mut().remove(&handle));
}

fn set_out(text: String) {
    OUT.with(|o| *o.borrow_mut() = text.into_bytes());
}

/// Reserves `len` bytes for the editor to write into.
#[unsafe(no_mangle)]
pub extern "C" fn aratu_alloc(len: usize) -> *mut u8 {
    let mut v = Vec::<u8>::with_capacity(len.max(1));
    let p = v.as_mut_ptr();
    std::mem::forget(v);
    p
}

/// Gives back memory from [`aratu_alloc`].
///
/// # Safety
/// `ptr` and `len` must come from one call to [`aratu_alloc`].
#[unsafe(no_mangle)]
pub unsafe extern "C" fn aratu_free(ptr: *mut u8, len: usize) {
    // SAFETY: the caller passes back exactly what aratu_alloc gave.
    unsafe { drop(Vec::from_raw_parts(ptr, 0, len.max(1))) };
}

/// # Safety
/// `ptr..ptr + len` must be readable memory holding text written by the
/// caller (invalid UTF-8 is replaced).
unsafe fn read_text(ptr: *const u8, len: usize) -> String {
    // SAFETY: guaranteed by the caller.
    let bytes = unsafe { std::slice::from_raw_parts(ptr, len) };
    String::from_utf8_lossy(bytes).into_owned()
}

/// Compiles a piece (`lang` 0 Portuguese, 1 English, 2 Spanish); the JSON answer is in
/// the output buffer. Returns the program's handle.
///
/// # Safety
/// `ptr..ptr + len` must be the UTF-8 source, readable.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn aratu_compile(ptr: *const u8, len: usize, lang: u32) -> u32 {
    // SAFETY: forwarded from the caller.
    let src = unsafe { read_text(ptr, len) };
    let lang = match lang {
        1 => Lang::En,
        2 => Lang::Es,
        _ => Lang::Pt,
    };
    let (handle, text) = compile_json(&src, lang);
    set_out(text);
    handle
}

/// Renders bars `from..to`, bar `from` from its beat `from_beat`; the JSON
/// answer is in the output buffer.
///
/// # Safety
/// `anchors_ptr..anchors_ptr + anchors_len` must be readable text.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn aratu_render(
    handle: u32,
    from: i32,
    from_beat: i32,
    to: i32,
    default_anchor: i32,
    seed: u32,
    anchors_ptr: *const u8,
    anchors_len: usize,
) {
    // SAFETY: forwarded from the caller.
    let anchors = unsafe { read_text(anchors_ptr, anchors_len) };
    let text = render_json(
        handle,
        from as i64,
        from_beat as i64,
        to as i64,
        default_anchor as i64,
        seed as u64,
        &anchors,
    );
    set_out(text);
}

/// Checks an exercise: the student's text against the answer's, by what
/// they play. `bars` 0 compares one time round of the answer; `flags`
/// bit 0 also compares dynamics, bit 1 leaves lengths out.
pub fn check_json(student: &str, answer: &str, bars: i64, flags: u32, lang: Lang) -> String {
    use aratu::exercise::{CheckOptions, check};
    let s = compile(student);
    let a = compile(answer);
    let mut j = Json::new();
    j.open_obj();
    j.key("compiles").bool(!s.has_errors());
    let problems: Vec<String> = if s.has_errors() || a.has_errors() {
        Vec::new()
    } else {
        let opts = CheckOptions {
            bars: (bars > 0).then_some(bars),
            lengths: flags & 2 == 0,
            dynamics: flags & 1 != 0,
        };
        check(&s.piece, &a.piece, opts)
            .iter()
            .map(|p| p.text(lang))
            .collect()
    };
    j.key("ok")
        .bool(!s.has_errors() && !a.has_errors() && problems.is_empty());
    j.key("problems").open_arr();
    for p in &problems {
        j.str(p);
    }
    j.close_arr();
    j.close_obj();
    j.finish()
}

/// Checks an exercise (see [`check_json`]); the JSON answer is in the
/// output buffer. `lang` as for [`aratu_compile`].
///
/// # Safety
/// Both ranges must be readable UTF-8 text.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn aratu_check(
    student_ptr: *const u8,
    student_len: usize,
    answer_ptr: *const u8,
    answer_len: usize,
    bars: i32,
    flags: u32,
    lang: u32,
) {
    // SAFETY: forwarded from the caller.
    let student = unsafe { read_text(student_ptr, student_len) };
    // SAFETY: forwarded from the caller.
    let answer = unsafe { read_text(answer_ptr, answer_len) };
    let lang = match lang {
        1 => Lang::En,
        2 => Lang::Es,
        _ => Lang::Pt,
    };
    set_out(check_json(&student, &answer, bars as i64, flags, lang));
}

/// Forgets a program.
#[unsafe(no_mangle)]
pub extern "C" fn aratu_release(handle: u32) {
    release(handle);
}

#[unsafe(no_mangle)]
pub extern "C" fn aratu_out_ptr() -> *const u8 {
    OUT.with(|o| o.borrow().as_ptr())
}

#[unsafe(no_mangle)]
pub extern "C" fn aratu_out_len() -> usize {
    OUT.with(|o| o.borrow().len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exercises_are_checked_by_what_they_play() {
        let ok = check_json(
            "m = | dó4 ré4 mi4 fá4 |",
            "m = | c4 d e f |",
            0,
            0,
            Lang::Pt,
        );
        assert_eq!(ok, r#"{"compiles":true,"ok":true,"problems":[]}"#);
        let wrong = check_json("m = | c4 d e e |", "m = | c4 d e f |", 0, 0, Lang::Es);
        assert!(
            wrong.contains("En el compás 1, tiempo 4: falta fa4."),
            "{wrong}"
        );
        let broken = check_json("m = | c4 d e |", "m = | c4 d e f |", 0, 0, Lang::Pt);
        assert!(
            broken.starts_with(r#"{"compiles":false,"ok":false"#),
            "{broken}"
        );
    }

    #[test]
    fn the_rhythm_map_answers_in_json() {
        let (h, _) = compile_json("hh = c.c c c -c\nm = | c4 e _ g |", Lang::En);
        let g = grid_json(h, 8);
        assert!(
            g.contains("\"label\":\"drums.hh\",\"kind\":\"drums\""),
            "{g}"
        );
        assert!(g.contains("\"res\":4"), "{g}");
        assert!(g.contains("\"e4\""), "{g}");
        release(h);
    }

    #[test]
    fn positions_are_utf16() {
        let t = utf16_table("dó𝄞x");
        // d=0, ó (2 bytes)=1, 𝄞 (4 bytes, 2 UTF-16 units)=2, x=4.
        assert_eq!(t[0], 0);
        assert_eq!(t[1], 1);
        assert_eq!(t[3], 2);
        assert_eq!(t[7], 4);
        assert_eq!(t[8], 5);
    }

    #[test]
    fn compile_and_render_answer_in_json() {
        let (h, text) = compile_json("tempo: 120\nbass = | dó3 _ _ _ |\nhh = c c c c", Lang::Pt);
        assert!(text.contains("\"ok\":true"), "{text}");
        assert!(text.contains("\"name\":\"bass\""));
        assert!(text.contains("\"instrument\":\"drums\""));
        let r = render_json(h, 1, 0, 2, 1, 0, "");
        assert!(r.contains("\"note\":48"), "{r}");
        assert!(r.contains("\"drum\":\"closed-hat\""));
        // dó3 starts after "tempo: 120\nbass = | " = 20 units.
        assert!(r.contains("\"from\":20,\"to\":23"), "{r}");
        release(h);
        assert_eq!(
            render_json(h, 1, 0, 2, 1, 0, ""),
            "{\"events\":[],\"bars\":[]}"
        );
    }

    #[test]
    fn errors_come_in_the_readers_language() {
        let (_, pt) = compile_json("bass = | c2 _ _ _ _ |", Lang::Pt);
        assert!(pt.contains("\"ok\":false"));
        assert!(pt.contains("o compasso 1 de `bass` tem 5 tempos"), "{pt}");
        let (_, en) = compile_json("bass = | c2 _ _ _ _ |", Lang::En);
        assert!(en.contains("bar 1 of `bass` has 5 beats"), "{en}");
    }

    #[test]
    fn moving_parameters_are_listed_and_glide() {
        let src = "melodia = | c4 _ _ _ |\nmelodia.lowpass.cutoff = 300 Hz <-> 5 kHz over 2 bars";
        let (h, text) = compile_json(src, Lang::En);
        assert!(
            text.contains("\"moving\":[{\"part\":\"melodia\",\"id\":\"lowpass.cutoff\"}]"),
            "{text}"
        );
        // A knob for where it starts and one for where it goes, in kHz.
        assert_eq!(
            text.matches("\"id\":\"lowpass.cutoff\",\"part\"").count(),
            2
        );
        assert!(text.contains("\"value\":5000,\"factor\":0.001"), "{text}");
        let r = render_json(h, 1, 0, 2, 1, 0, "");
        assert!(
            r.contains("\"param\":\"lowpass.cutoff\",\"value\":300,"),
            "{r}"
        );
        assert!(r.contains("\"glide\":"), "{r}");
    }

    #[test]
    fn live_lines_are_listed_for_anchoring() {
        let (h, text) = compile_json("pattern a { hh = c c c c }\nplay a\nnow: play a", Lang::En);
        assert!(
            text.contains("{\"key\":\"play a\",\"now\":false}"),
            "{text}"
        );
        assert!(
            text.contains("{\"key\":\"now: play a\",\"now\":true}"),
            "{text}"
        );
        let r = render_json(h, 1, 0, 4, 1, 0, "3\tplay a\n3\tnow: play a");
        let events = &r[..r.find("\"bars\"").unwrap()];
        assert!(events.contains("\"bar\":3"));
        assert!(!events.contains("\"bar\":1,"), "{r}");
    }
}

#[cfg(test)]
mod control_tests {
    use super::*;

    #[test]
    fn numbers_that_set_parameters_become_knobs() {
        let src = "tempo: 96\nbass = | c2 _ _ _ |\nbass {\n  reverb.mix = 0.42\n  lowpass.cutoff = 1.2 kHz\n}\nat bar 16: bass.reverb.mix = 65%\nbass { wobble = 3 }";
        let c = compile(src);
        let ctl = controls(&c.piece);
        let ids: Vec<&str> = ctl.iter().map(|c| c.id.as_str()).collect();
        assert_eq!(ids, ["tempo", "reverb.mix", "lowpass.cutoff", "reverb.mix"]);
        let text = |c: &Control| &src[c.span.start..c.span.end];
        assert_eq!(text(&ctl[0]), "96");
        assert_eq!(text(&ctl[1]), "0.42");
        assert_eq!(text(&ctl[2]), "1.2");
        assert_eq!(ctl[2].value, 1200.0);
        assert!((ctl[2].factor - 0.001).abs() < 1e-12);
        assert!(ctl[2].log);
        assert_eq!(text(&ctl[3]), "65");
        assert_eq!(ctl[3].factor, 100.0);
        assert!(ctl[3].timed);
        assert!(ctl.iter().all(|c| c.part == "bass" || c.id == "tempo"));
    }
}
