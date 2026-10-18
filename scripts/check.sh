#!/usr/bin/env sh
# SPDX-License-Identifier: AGPL-3.0-or-later
# Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
# Everything CI checks, stopping at the first failure. Run before committing.
set -e
cargo fmt --all --check
cargo clippy --all-targets -- -D warnings
cargo test
(cd web && npm run check && npm run wasm && npm run smoke)
