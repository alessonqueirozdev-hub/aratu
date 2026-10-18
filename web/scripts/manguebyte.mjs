// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
// Brings Manguebyte's browser engine (crates/manguebyte-web) into the
// editor: clones Manguebyte at a pinned commit, builds the WebAssembly
// module and copies it, with the three JavaScript files it runs with and
// its licences, into public/manguebyte/. Run by hand when Manguebyte ships
// something Aratu should play, then commit the result:
//
//   node scripts/manguebyte.mjs [commit]
//
// Manguebyte is MIT OR Apache-2.0, like Aratu.

import { execFileSync } from "node:child_process";
import { cpSync, existsSync, mkdirSync, rmSync, statSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const REPO = "https://github.com/alessonqueirozdev-hub/manguebyte.git";
// Waves 5c and 5d (effects, the master bus, the instruments of
// manguebyte-dsp, the keys of the Brazilian percussion), and every
// instrument and preset brought to the same loudness, -24 LUFS, with a
// sampled instrument playing as soon as its samples are decoded.
const COMMIT = process.argv[2] ?? "bb31a669b50597761381c0d15821990e37f507d8";

const web = join(dirname(fileURLToPath(import.meta.url)), "..");
const cache = join(web, ".manguebyte-cache");
const src = join(cache, "src");
const out = join(web, "public", "manguebyte");

const run = (cmd, args, cwd) => execFileSync(cmd, args, { cwd, stdio: "inherit" });

mkdirSync(cache, { recursive: true });
if (!existsSync(join(src, ".git"))) run("git", ["clone", "--filter=blob:none", "--no-checkout", REPO, src], cache);
run("git", ["fetch", "--quiet", "origin", COMMIT], src);
run("git", ["checkout", "--quiet", "--force", COMMIT], src);
run(
  "cargo",
  [
    "build",
    "--quiet",
    "-p",
    "manguebyte-web",
    "--target",
    "wasm32-unknown-unknown",
    "--release",
    "--target-dir",
    join(cache, "target"),
  ],
  src,
);

rmSync(out, { recursive: true, force: true });
mkdirSync(out, { recursive: true });
const crate = join(src, "crates", "manguebyte-web");
cpSync(join(cache, "target", "wasm32-unknown-unknown", "release", "manguebyte_web.wasm"), join(out, "manguebyte_web.wasm"));
for (const f of ["engine.js", "ring.js", "worklet.js"]) cpSync(join(crate, "js", f), join(out, f));
for (const f of ["LICENSE-MIT", "LICENSE-APACHE"]) cpSync(join(crate, f), join(out, f));
// Aratu's own types for the part of engine.js it uses.
cpSync(join(web, "scripts", "manguebyte-engine.d.ts"), join(out, "engine.d.ts"));
writeFileSync(
  join(out, "VERSION"),
  `Manguebyte ${COMMIT}\n${REPO}\nBuilt by Aratu's web/scripts/manguebyte.mjs.\n`,
);
const kb = Math.round(statSync(join(out, "manguebyte_web.wasm")).size / 1024);
console.log(`manguebyte ${COMMIT.slice(0, 7)}: manguebyte_web.wasm ${kb} KB`);
