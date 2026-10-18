// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
//! Every example in the manual, in every language, must be read without
//! errors, so the manual and the language cannot drift apart.

use aratu::compile;
use aratu::diag::{Lang, Message};
use aratu::parse::parse;
use aratu::span::line_col;

const MANUALS: [(&str, &str); 3] = [
    ("pt", include_str!("../../../book/pt/manual.md")),
    ("es", include_str!("../../../book/es/manual.md")),
    ("en", include_str!("../../../book/en/manual.md")),
];

/// Every block of every manual, with where it comes from.
fn all_blocks() -> Vec<(String, usize, String)> {
    MANUALS
        .iter()
        .flat_map(|(lang, md)| {
            code_blocks(md)
                .into_iter()
                .map(move |(line, src)| (lang.to_string(), line, src))
        })
        .collect()
}

/// The fenced code blocks of a Markdown text, with the line each starts on.
pub fn code_blocks(md: &str) -> Vec<(usize, String)> {
    let mut blocks = Vec::new();
    let mut current: Option<(usize, String)> = None;
    for (i, line) in md.lines().enumerate() {
        if line.trim_start().starts_with("```") {
            match current.take() {
                Some(b) => blocks.push(b),
                None => current = Some((i + 2, String::new())),
            }
        } else if let Some((_, text)) = current.as_mut() {
            text.push_str(line);
            text.push('\n');
        }
    }
    blocks
}

#[test]
fn every_example_in_the_manual_parses() {
    let blocks = all_blocks();
    assert!(blocks.len() >= 60, "found only {} examples", blocks.len());
    let mut failures = Vec::new();
    for (lang, line, src) in &blocks {
        let (_, diags) = parse(src);
        for d in diags.iter().filter(|d| d.is_error()) {
            let (l, _) = line_col(src, d.span.start);
            failures.push(format!(
                "book/{lang}/manual.md line {}: `{}`: {}",
                line + l - 1,
                &src[d.span.start..d.span.end],
                d.text(Lang::En)
            ));
        }
    }
    assert!(failures.is_empty(), "\n{}", failures.join("\n"));
}

/// Whether a pattern or section of this name is defined in some example.
fn defined_somewhere(blocks: &[(String, usize, String)], name: &str) -> bool {
    blocks.iter().any(|(_, _, src)| {
        src.contains(&format!("pattern {name} ")) || src.contains(&format!("section {name} "))
    })
}

/// The examples are fragments of one notebook: a name used in one example
/// may be defined in another, but nothing else may be wrong.
#[test]
fn every_example_in_the_manual_checks() {
    let blocks = all_blocks();
    let mut failures = Vec::new();
    for (lang, line, src) in &blocks {
        let compiled = compile(src);
        for d in compiled.errors() {
            let elsewhere = match &d.message {
                Message::UnknownPattern { name } | Message::UnknownSection { name } => {
                    defined_somewhere(&blocks, name)
                }
                _ => false,
            };
            if elsewhere {
                continue;
            }
            let (l, _) = line_col(src, d.span.start);
            failures.push(format!(
                "book/{lang}/manual.md line {}: `{}`: {}",
                line + l - 1,
                &src[d.span.start..d.span.end],
                d.text(Lang::En)
            ));
        }
    }
    assert!(failures.is_empty(), "\n{}", failures.join("\n"));
}
