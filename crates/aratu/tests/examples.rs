// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
//! The example pieces the editor offers compile with no problem at all, and
//! play.

use aratu::compile;
use aratu::diag::Lang;
use aratu::schedule::{Options, render};
use std::path::Path;

#[test]
fn every_example_compiles_cleanly_and_plays() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples");
    let mut count = 0;
    for entry in std::fs::read_dir(&dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_none_or(|e| e != "aratu") {
            continue;
        }
        let src = std::fs::read_to_string(&path).unwrap();
        let c = compile(&src);
        let problems: Vec<String> = c.diagnostics.iter().map(|d| d.text(Lang::En)).collect();
        assert!(problems.is_empty(), "{}: {problems:?}", path.display());
        let r = render(&c.piece, &Options::default(), 0, 17);
        assert!(
            r.events.len() > 16,
            "{} plays almost nothing",
            path.display()
        );
        count += 1;
    }
    assert!(count >= 5);
}
