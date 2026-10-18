// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
//! The examples on the page for developers, in every language, compile
//! without errors, so the page and the language cannot drift apart.

use aratu::compile;
use aratu::diag::Lang;

const PAGES: [(&str, &str); 3] = [
    ("pt", include_str!("../../../book/pt/inside.md")),
    ("es", include_str!("../../../book/es/inside.md")),
    ("en", include_str!("../../../book/en/inside.md")),
];

/// The `aratu` blocks of a Markdown text (the page also shows JSON).
fn aratu_blocks(md: &str) -> Vec<String> {
    let mut blocks = Vec::new();
    let mut current: Option<String> = None;
    for line in md.lines() {
        let fence = line.trim_start().starts_with("```");
        match (&mut current, fence) {
            (Some(_), true) => blocks.push(current.take().unwrap()),
            (None, true) if line.trim_start().starts_with("```aratu") => {
                current = Some(String::new());
            }
            (None, true) => current = None,
            (Some(text), false) => {
                text.push_str(line);
                text.push('\n');
            }
            (None, false) => {}
        }
    }
    blocks
}

#[test]
fn every_example_on_the_page_for_developers_compiles() {
    for (lang, md) in PAGES {
        let blocks = aratu_blocks(md);
        assert!(blocks.len() >= 3, "{lang}: only {} examples", blocks.len());
        for src in blocks {
            let c = compile(&src);
            let errors: Vec<String> = c.errors().map(|d| d.text(Lang::En)).collect();
            assert!(errors.is_empty(), "{lang}: {src}\n{errors:?}");
        }
    }
}
