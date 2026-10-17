//! The book (the course and the manual, in every language) must never show
//! a broken example or an exercise that cannot be solved.
//!
//! - Every example compiles without errors, unless marked `broken` on
//!   purpose (an example of an error for the student to fix).
//! - Every answer compiles without a single warning.
//! - An exercise's starting code does not already sound like its answer.
//! - The same exercise sounds the same in every language.

use aratu::compile;
use aratu::diag::Lang;
use aratu::exercise::{CheckOptions, check};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

const LANGS: [&str; 3] = ["pt", "es", "en"];

#[derive(Debug, Clone)]
struct Block {
    kind: String,
    attrs: BTreeMap<String, String>,
    code: String,
    answer: Option<String>,
    line: usize,
}

fn book_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../book")
}

/// The Aratu blocks of a Markdown page, answers joined to their exercises.
fn blocks(md: &str) -> Vec<Block> {
    let mut out: Vec<Block> = Vec::new();
    let mut lines = md.lines().enumerate();
    while let Some((n, line)) = lines.next() {
        let Some(info) = line.trim_start().strip_prefix("```aratu") else {
            continue;
        };
        let mut words = info.split_whitespace();
        let mut kind = "example".to_string();
        let mut attrs = BTreeMap::new();
        for w in words.by_ref() {
            match w.split_once('=') {
                Some((k, v)) => {
                    attrs.insert(k.to_string(), v.to_string());
                }
                None if ["example", "exercise", "answer"].contains(&w) => kind = w.to_string(),
                None => {
                    attrs.insert(w.to_string(), "1".to_string());
                }
            }
        }
        let mut code = String::new();
        for (_, l) in lines.by_ref() {
            if l.trim_start().starts_with("```") {
                break;
            }
            code.push_str(l);
            code.push('\n');
        }
        if kind == "answer" {
            let prev = out.last_mut().expect("an answer follows an exercise");
            assert_eq!(
                prev.kind,
                "exercise",
                "line {}: an answer must follow an exercise",
                n + 1
            );
            prev.answer = Some(code);
            continue;
        }
        out.push(Block {
            kind,
            attrs,
            code,
            answer: None,
            line: n + 1,
        });
    }
    out
}

/// Every page of the book in one language: (name, text).
fn pages(lang: &str) -> Vec<(String, String)> {
    let dir = book_dir().join(lang);
    let mut out = Vec::new();
    let manual = dir.join("manual.md");
    if manual.exists() {
        out.push((
            "manual".to_string(),
            std::fs::read_to_string(manual).unwrap(),
        ));
    }
    let learn = dir.join("learn");
    if learn.exists() {
        let mut files: Vec<PathBuf> = std::fs::read_dir(learn)
            .unwrap()
            .map(|e| e.unwrap().path())
            .filter(|p| p.extension().is_some_and(|e| e == "md"))
            .collect();
        files.sort();
        for f in files {
            let name = f.file_stem().unwrap().to_string_lossy().to_string();
            out.push((name, std::fs::read_to_string(f).unwrap()));
        }
    }
    out
}

fn options(b: &Block) -> CheckOptions {
    CheckOptions {
        bars: b.attrs.get("bars").and_then(|v| v.parse().ok()),
        lengths: b.attrs.get("lengths").is_none_or(|v| v != "0"),
        dynamics: b.attrs.get("dynamics").is_some_and(|v| v == "1"),
        timing: b.attrs.get("timing").is_some_and(|v| v == "1"),
        sound: b.attrs.get("sound").is_some_and(|v| v == "1"),
    }
}

fn errors(src: &str) -> Vec<String> {
    compile(src).errors().map(|d| d.text(Lang::En)).collect()
}

#[test]
fn every_block_of_the_book_works() {
    let mut failures = Vec::new();
    let mut count = 0;
    for lang in LANGS {
        for (page, md) in pages(lang) {
            for b in blocks(&md) {
                count += 1;
                let at = format!("book/{lang}/{page} line {}", b.line);
                let broken = b.attrs.contains_key("broken");
                let errs = errors(&b.code);
                if broken && errs.is_empty() {
                    failures.push(format!("{at}: marked broken but has no error"));
                }
                if !broken && !errs.is_empty() {
                    failures.push(format!("{at}: {errs:?}"));
                }
                if b.kind == "exercise" {
                    let Some(answer) = &b.answer else {
                        failures.push(format!("{at}: exercise without an answer"));
                        continue;
                    };
                    let a = compile(answer);
                    let problems: Vec<String> =
                        a.diagnostics.iter().map(|d| d.text(Lang::En)).collect();
                    if !problems.is_empty() {
                        failures.push(format!("{at}: the answer has problems: {problems:?}"));
                        continue;
                    }
                    if !broken {
                        let s = compile(&b.code);
                        if check(&s.piece, &a.piece, options(&b)).is_empty() {
                            failures.push(format!(
                                "{at}: the starting code already sounds like the answer"
                            ));
                        }
                    }
                }
            }
        }
    }
    assert!(count > 0, "the book has no blocks");
    assert!(failures.is_empty(), "\n{}", failures.join("\n"));
}

#[test]
fn every_exercise_sounds_the_same_in_every_language() {
    // Exercises by page and position.
    let mut by_lang: BTreeMap<&str, BTreeMap<(String, usize), Block>> = BTreeMap::new();
    for lang in LANGS {
        let mut map = BTreeMap::new();
        for (page, md) in pages(lang) {
            for (i, b) in blocks(&md)
                .into_iter()
                .filter(|b| b.kind == "exercise")
                .enumerate()
            {
                map.insert((page.clone(), i), b);
            }
        }
        by_lang.insert(lang, map);
    }
    let reference = &by_lang["pt"];
    let mut failures = Vec::new();
    for lang in ["es", "en"] {
        let other = &by_lang[lang];
        let pages_of = |m: &BTreeMap<(String, usize), Block>| -> Vec<String> {
            let mut v: Vec<String> = m.keys().map(|k| k.0.clone()).collect();
            v.dedup();
            v
        };
        // Only pages written in both languages are compared.
        let shared: Vec<String> = pages_of(reference)
            .into_iter()
            .filter(|p| pages_of(other).contains(p))
            .collect();
        for page in shared {
            let ours: Vec<_> = reference.iter().filter(|(k, _)| k.0 == page).collect();
            let theirs: Vec<_> = other.iter().filter(|(k, _)| k.0 == page).collect();
            if ours.len() != theirs.len() {
                failures.push(format!(
                    "{page}: {} exercises in pt, {} in {lang}",
                    ours.len(),
                    theirs.len()
                ));
                continue;
            }
            for ((k, a), (_, b)) in ours.iter().zip(theirs.iter()) {
                let (Some(x), Some(y)) = (&a.answer, &b.answer) else {
                    continue;
                };
                let (x, y) = (compile(x), compile(y));
                let opts = CheckOptions {
                    dynamics: true,
                    ..options(a)
                };
                let diff = check(&y.piece, &x.piece, opts);
                if !diff.is_empty() {
                    failures.push(format!(
                        "{page} exercise {}: {lang} differs from pt: {}",
                        k.1 + 1,
                        diff[0].text(Lang::En)
                    ));
                }
            }
        }
    }
    assert!(failures.is_empty(), "\n{}", failures.join("\n"));
}

/// A feature, and how to tell a lesson's code shows it.
type Taught = (&'static str, fn(&str) -> bool);

/// Every piece of the language is taught in the course, in an example or an
/// exercise, in every language: a feature only the manual shows is one that
/// students never meet. A feature added to the language gets a line here,
/// and a lesson.
const TAUGHT: &[Taught] = &[
    ("dividing a beat", |c| c.contains("cccc")),
    ("dotted rhythm", |c| c.contains("c.c")),
    ("bars of rest", |c| c.contains("| - x")),
    ("compound meter", |c| c.contains("6/8")),
    ("five beats", |c| c.contains("5/4")),
    ("additive meter", |c| c.contains("= 2+2+3")),
    ("tuplet across beats", |c| c.contains("2[")),
    ("accent", |c| c.contains(" S ") || c.contains("- S")),
    ("open hi-hat", |c| c.contains("co")),
    ("toms", |c| c.contains("tom ")),
    ("side stick", |c| c.contains("rim")),
    ("roll", |c| c.contains('~')),
    ("clap", |c| c.contains("palmas") || c.contains("clap")),
    ("pandeiro", |c| {
        c.contains("pandeiro") || c.contains("pandero")
    }),
    ("zabumba", |c| c.contains("zabumba")),
    ("triangle", |c| {
        c.contains("triângulo") || c.contains("triángulo") || c.contains("triangle")
    }),
    ("agogô", |c| {
        c.contains("agogô") || c.contains("agogó") || c.contains("agogo")
    }),
    ("samba surdo", |c| c.contains("surdo")),
    ("alfaia", |c| c.contains("alfaia")),
    ("gonguê", |c| c.contains("gonguê") || c.contains("gongue")),
    ("cajón", |c| c.contains("cajón") || c.contains("cajon")),
    ("shaker", |c| {
        c.contains("ganzá") || c.contains("agbê") || c.contains("shaker")
    }),
    ("notes and octaves", |c| c.contains("4 ")),
    ("sharps", |c| c.contains('#')),
    ("notes together", |c| c.contains("4+")),
    ("tie over the bar", |c| c.contains("| _")),
    ("staccato", |c| c.contains("4'")),
    ("long lines", |c| {
        c.lines()
            .skip(1)
            .any(|l| l.starts_with(' ') && l.trim_start().starts_with('|'))
    }),
    ("chord symbols", |c| c.contains("| Am |")),
    ("solfège chords", |c| {
        c.contains("Lám") || c.contains("Lam ")
    }),
    ("no chord", |c| c.contains("N.C.")),
    ("strum", |c| c.contains("du")),
    ("golpe", |c| c.contains("dgdu")),
    ("roman numerals", |c| c.contains("| vi |")),
    ("secondary dominant", |c| c.contains("V7/V")),
    ("borrowed chord", |c| c.contains("bVII")),
    ("arpeggio", |c| c.contains(":arp")),
    ("dynamic marks", |c| c.contains("p:")),
    ("hairpins", |c| c.contains("4<")),
    ("a part's level", |c| c.contains("cresc to")),
    ("marcato", |c| c.contains("S!") || c.contains("s!")),
    ("ghost notes", |c| c.contains("(s)")),
    ("trill", |c| c.contains(":tr ")),
    ("mordent", |c| c.contains(":mord ")),
    ("grace note", |c| {
        c.contains("`ré5") || c.contains("`re5") || c.contains("`d5")
    }),
    ("flam", |c| c.contains(" `s") || c.contains(" `S")),
    ("drag", |c| c.contains("``s") || c.contains("``S")),
    ("repeat signs", |c| c.contains("|:")),
    ("first and second endings", |c| c.contains("|1")),
    ("repeat the bar", |c| c.contains('%')),
    ("jumps", |c| c.contains("D.S.")),
    ("sections", |c| {
        c.contains("section ") || c.contains("seção ") || c.contains("sección ")
    }),
    ("form", |c| c.contains("form:") || c.contains("forma:")),
    ("repeat a beat", |c| c.contains("]x4")),
    ("motifs", |c| c.contains("motif ") || c.contains("motivo ")),
    ("sequences", |c| c.contains("+1")),
    ("inversion", |c| c.contains(".inv")),
    ("twelve-tone rows", |c| {
        c.contains(".P0") || c.contains(".R0")
    }),
    ("melody lists", |c| {
        c.contains("list ") || c.contains("lista ")
    }),
    ("transforming a line", |c| {
        c.contains(".transpose") || c.contains(".transpor") || c.contains(".transponer")
    }),
    ("ritardando", |c| c.contains("rit to")),
    ("tempo ramp", |c| c.contains("90 -> 140")),
    ("fermata", |c| c.contains("4^")),
    ("fermata's length", |c| c.contains("^3")),
    ("pickup", |c| {
        c.contains("pickup:") || c.contains("anacruse:") || c.contains("anacrusa:")
    }),
    ("swing", |c| c.contains("swing")),
    ("rubato", |c| c.contains("rubato")),
    ("humanize", |c| {
        c.contains("humanize") || c.contains("humanizar")
    }),
    ("patterns", |c| {
        c.contains("pattern ") || c.contains("padrão ") || c.contains("patrón ")
    }),
    ("every", |c| c.contains("every ") || c.contains("cada ")),
    ("once", |c| c.contains("once")),
    ("a beat of a bar", |c| {
        c.contains("beat 3") || c.contains("tempo 3:") || c.contains("tiempo 3")
    }),
    ("a stretch of bars", |c| {
        c.contains("from bar")
            || c.contains("do compasso")
            || c.contains("del compás")
            || c.contains("desde el compás")
    }),
    ("a line changing at a bar", |c| {
        c.contains(": baixo = |") || c.contains(": bass = |") || c.contains(": bajo = |")
    }),
    ("out and in", |c| {
        c.contains(": out") || c.contains(": fora") || c.contains(": fuera")
    }),
    ("sometimes", |c| c.contains("?30%")),
    ("one or the other", |c| {
        c.contains('{') && c.contains('|') && c.contains("}")
    }),
    ("seed", |c| {
        c.contains("seed") || c.contains("semente") || c.contains("semilla")
    }),
    ("a voice", |c| c.contains("bass.slap")),
    ("an effect chain", |c| c.contains("-> lowpass")),
    ("a number with a knob", |c| c.contains(".cutoff =")),
    ("a number that moves", |c| c.contains(" -> 4000 Hz")),
    ("back and forth", |c| c.contains("<->")),
    ("loop", |c| {
        c.contains("loop") || c.contains("repete") || c.contains("repite")
    }),
    ("volume", |c| c.contains(".volume =")),
    ("pan", |c| c.contains(".pan =")),
    ("a number for every part", |c| c.contains("\nreverb.mix =")),
    ("effects for every part", |c| {
        c.contains("todos: chorus") || c.contains("all: chorus")
    }),
    ("the master", |c| c.contains("master.")),
    ("keywords in other languages", |c| {
        c.contains("compasso:") || c.contains("compás:")
    }),
];

#[test]
fn every_feature_is_taught_in_every_language() {
    let mut failures = Vec::new();
    for lang in LANGS {
        let code: Vec<String> = pages(lang)
            .into_iter()
            .filter(|(page, _)| page != "manual")
            .flat_map(|(_, md)| blocks(&md))
            .flat_map(|b| [Some(b.code), b.answer].into_iter().flatten())
            .collect();
        for (feature, taught) in TAUGHT {
            if !code.iter().any(|c| taught(c)) {
                failures.push(format!("{lang}: no lesson shows {feature}"));
            }
        }
    }
    assert!(failures.is_empty(), "\n{}", failures.join("\n"));
}
