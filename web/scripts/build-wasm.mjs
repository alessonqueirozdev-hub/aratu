// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
// Builds the language crate for the browser and copies it next to the page.
import { execFileSync } from "node:child_process";
import { copyFileSync, mkdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const web = join(dirname(fileURLToPath(import.meta.url)), "..");
const root = join(web, "..");

export function buildWasm() {
  execFileSync(
    "cargo",
    ["build", "--release", "--target", "wasm32-unknown-unknown", "-p", "aratu-wasm"],
    { cwd: root, stdio: "inherit" },
  );
  mkdirSync(join(web, "public"), { recursive: true });
  copyFileSync(
    join(root, "target", "wasm32-unknown-unknown", "release", "aratu_wasm.wasm"),
    join(web, "public", "aratu.wasm"),
  );
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  buildWasm();
}
