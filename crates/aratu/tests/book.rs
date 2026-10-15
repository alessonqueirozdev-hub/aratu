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
