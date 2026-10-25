// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
// Builds the library (build-lib.mjs) into a scratch folder and checks what
// a page that holds it relies on: every file is there; the module imports
// without a document (so it touches nothing of the page until mounted) and
// gives mount and mountPlayer; no script uses eval or new Function or loads
// from blob:; every style rule is under .aratu-root, none on :root, html,
// body or a bare element, every animation's name is Aratu's own; and the
// layered sheet is the same rules in @layer aratu. Mounting itself is
// checked in a browser, on lib-test/ (npm run lib:test).
import assert from "node:assert/strict";
import { existsSync, mkdtempSync, readFileSync, readdirSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { pathToFileURL } from "node:url";
import { buildLib } from "./build-lib.mjs";

const out = mkdtempSync(join(tmpdir(), "aratu-lib-"));
try {
  await buildLib(out, { wasm: false });
  const files = [
    "aratu.js",
    "aratu.js.map",
    "aratu.d.ts",
    "aratu.css",
    "aratu.layered.css",
    "aratu.wasm",
    "manguebyte/engine.js",
    "manguebyte/bridge.js",
    "manguebyte/worklet.js",
    "manguebyte/ring.js",
    "manguebyte/scope.js",
    "manguebyte/manguebyte_web.wasm",
    "manguebyte/LICENSE-MIT",
    "samples/piano.grand/voice.sfz",
    "samples/piano.grand/index.json",
    "book-pt.json",
    "book-es.json",
    "book-en.json",
    "mascot.png",
    "VERSION",
    "README.md",
    "LICENSE",
    "NOTICE.md",
    "COMMERCIAL.md",
    "TRADEMARKS.md",
    "BOOK-LICENSE.md",
  ];
  for (const f of files) assert.ok(existsSync(join(out, f)), `dist-lib: ${f} is missing`);
  assert.match(readFileSync(join(out, "VERSION"), "utf8"), /^Aratu \S+ [0-9a-f]{40}/, "VERSION names no commit");

  // Imported where there is no document, it must not throw: nothing runs
  // on the page until mount() is called.
  const lib = await import(pathToFileURL(join(out, "aratu.js")).href);
  assert.equal(typeof lib.mount, "function", "no mount()");
  assert.equal(typeof lib.mountPlayer, "function", "no mountPlayer()");
  assert.deepEqual(lib.routeOf("#/aprender/01-bars"), { view: "learn", page: "01-bars" });

  // A page whose Content Security Policy has no 'unsafe-eval' and no blob:
  // for scripts runs all of it.
  const scripts = ["aratu.js", ...readdirSync(join(out, "manguebyte")).filter((f) => f.endsWith(".js")).map((f) => `manguebyte/${f}`)];
  for (const f of scripts) {
    const js = readFileSync(join(out, f), "utf8");
    assert.ok(!/\beval\s*\(/.test(js), `${f}: eval`);
    assert.ok(!/\bnew\s+Function\b|[^.\w]Function\s*\(/.test(js), `${f}: new Function`);
    assert.ok(!js.includes("blob:"), `${f}: blob:`);
  }
  assert.ok(
    /from\s*"\.\/manguebyte\/engine\.js"/.test(readFileSync(join(out, "aratu.js"), "utf8")),
    "aratu.js does not import Manguebyte's engine from beside it",
  );

  // Every rule under .aratu-root.
  const css = readFileSync(join(out, "aratu.css"), "utf8");
  const plain = css.replace(/\/\*[\s\S]*?\*\//g, "");
  const keyframes = [...plain.matchAll(/@keyframes\s+([\w-]+)/g)].map((m) => m[1]);
  assert.ok(keyframes.length > 0 && keyframes.every((k) => k.startsWith("aratu-")), `keyframes: ${keyframes}`);
  for (const m of plain.matchAll(/animation:\s*([\w-]+)/g)) {
    assert.ok(m[1] === "none" || keyframes.includes(m[1]), `animation ${m[1]}`);
  }
  const body = plain.replace(/@keyframes\s+[\w-]+\s*\{(?:[^{}]*\{[^{}]*\})*[^{}]*\}/g, "");
  let rules = 0;
  for (const m of body.matchAll(/([^{}@;]+)\{[^{}]*\}/g)) {
    for (const sel of m[1].split(/,(?![^(]*\))/).map((s) => s.trim())) {
      assert.ok(/^(\.aratu-root|:where\(\.aratu-root)/.test(sel), `a rule outside .aratu-root: ${sel}`);
      rules++;
    }
  }
  assert.ok(rules > 300, `only ${rules} rules read`);
  for (const bare of [":root", "html", "body"]) assert.ok(!new RegExp(`(^|[\\s,}])${bare}[\\s,{.:\\[]`).test(body), `a rule on ${bare}`);
  const layered = readFileSync(join(out, "aratu.layered.css"), "utf8");
  assert.ok(layered.includes(`@layer aratu {\n${css}\n}`), "aratu.layered.css is not aratu.css in @layer aratu");
  console.log(`lib ok: ${rules} selectors under .aratu-root, ${scripts.length} scripts`);
} finally {
  rmSync(out, { recursive: true, force: true });
}
