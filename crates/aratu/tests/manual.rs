//! Every example in the manual must be read without errors, so the manual
//! and the language cannot drift apart.

use aratu::compile;
use aratu::diag::{Lang, Message};
use aratu::parse::parse;
use aratu::span::line_col;

const MANUAL: &str = include_str!("../../../docs/MANUAL.md");

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
    let blocks = code_blocks(MANUAL);
    assert!(blocks.len() > 20, "found only {} examples", blocks.len());
    let mut failures = Vec::new();
    for (line, src) in &blocks {
        let (_, diags) = parse(src);
        for d in diags.iter().filter(|d| d.is_error()) {
            let (l, _) = line_col(src, d.span.start);
            failures.push(format!(
                "MANUAL.md line {}: `{}`: {}",
                line + l - 1,
                &src[d.span.start..d.span.end],
                d.text(Lang::En)
            ));
        }
    }
    assert!(failures.is_empty(), "\n{}", failures.join("\n"));
}

/// Whether a pattern or section of this name is defined in some example.
fn defined_somewhere(blocks: &[(usize, String)], name: &str) -> bool {
    blocks.iter().any(|(_, src)| {
        src.contains(&format!("pattern {name} ")) || src.contains(&format!("section {name} "))
    })
}

/// The examples are fragments of one notebook: a name used in one example
/// may be defined in another, but nothing else may be wrong.
#[test]
fn every_example_in_the_manual_checks() {
    let blocks = code_blocks(MANUAL);
    let mut failures = Vec::new();
    for (line, src) in &blocks {
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
                "MANUAL.md line {}: `{}`: {}",
                line + l - 1,
                &src[d.span.start..d.span.end],
                d.text(Lang::En)
            ));
        }
    }
    assert!(failures.is_empty(), "\n{}", failures.join("\n"));
}
