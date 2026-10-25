// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
// Builds Aratu as a library, for a page that holds the editor (see
// lib/README.md), into dist-lib/:
//
//   aratu.js, aratu.js.map   mount() and mountPlayer(), one ES module
//   aratu.d.ts               its types
//   aratu.css                its styles, every rule under .aratu-root
//   aratu.layered.css        the same in @layer aratu, for a page that
//                            puts its own resets in layers too
//   aratu.wasm               the language
//   manguebyte/              Manguebyte's engine, worklet and wasm, as they
//                            are: aratu.js imports engine.js from there, and
//                            the worklet is loaded from there by its address
//   samples/                 the recorded voices
//   book-<lang>.json         the course, the manual, the pages for developers
//   mascot.png, VERSION, and the licences.
//
// Everything but aratu.js is found by its address under the folder given
// as `assetBase`: the folder is served as it is, from the same site.
import * as esbuild from "esbuild";
import { execFileSync } from "node:child_process";
import { copyFileSync, cpSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { basename, dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { buildBook } from "./book.mjs";
import { buildWasm } from "./build-wasm.mjs";

const web = join(dirname(fileURLToPath(import.meta.url)), "..");
const root = join(web, "..");
export const distLib = join(web, "dist-lib");

/** Manguebyte's modules stay beside the library, imported by their
 * address, so a bundler takes them as they are and the worklet's own
 * imports (ring.js, scope.js) still find their files. */
const manguebyte = {
  name: "manguebyte",
  setup(build) {
    build.onResolve({ filter: /[\\/]public[\\/]manguebyte[\\/][\w-]+\.js$/ }, (args) => ({
      path: `./manguebyte/${basename(args.path)}`,
      external: true,
    }));
  },
};

function git(...args) {
  try {
    return execFileSync("git", args, { cwd: root, encoding: "utf8" }).trim();
  } catch {
    return "";
  }
}

/** Which Aratu this is: the commit, and whether the files differ from it. */
function version() {
  const commit = git("rev-parse", "HEAD") || "unknown";
  const changed = git("status", "--porcelain").length > 0;
  const mb = readFileSync(join(web, "public", "manguebyte", "VERSION"), "utf8").split("\n")[0];
  const pkg = JSON.parse(readFileSync(join(web, "package.json"), "utf8"));
  return [
    `Aratu ${pkg.version} ${commit}${changed ? " (with changes not committed)" : ""}`,
    "https://github.com/alessonqueirozdev-hub/aratu",
    `built ${new Date().toISOString()}`,
    mb,
    "",
  ].join("\n");
}

export async function buildLib(out = distLib, { wasm = true } = {}) {
  if (wasm) buildWasm();
  rmSync(out, { recursive: true, force: true });
  mkdirSync(out, { recursive: true });
  await esbuild.build({
    entryPoints: [join(web, "src", "lib.ts")],
    bundle: true,
    format: "esm",
    target: "es2022",
    outfile: join(out, "aratu.js"),
    sourcemap: true,
    minify: true,
    legalComments: "eof",
    loader: { ".aratu": "text" },
    plugins: [manguebyte],
  });
  const css = readFileSync(join(web, "src", "style.css"), "utf8");
  writeFileSync(join(out, "aratu.css"), css);
  writeFileSync(
    join(out, "aratu.layered.css"),
    `/* Aratu's styles in the cascade layer "aratu": for a page whose own resets\n` +
      `   are in layers too (Tailwind 4, say), declared before this one. A reset\n` +
      `   outside any layer (Tailwind 3's preflight) wins over every layer: such a\n` +
      `   page takes aratu.css instead. */\n` +
      `@layer aratu {\n${css}\n}\n`,
  );
  copyFileSync(join(web, "public", "aratu.wasm"), join(out, "aratu.wasm"));
  cpSync(join(web, "public", "manguebyte"), join(out, "manguebyte"), { recursive: true });
  cpSync(join(web, "public", "samples"), join(out, "samples"), { recursive: true });
  copyFileSync(join(web, "public", "mascot.png"), join(out, "mascot.png"));
  buildBook(out);
  copyFileSync(join(web, "lib", "aratu.d.ts"), join(out, "aratu.d.ts"));
  copyFileSync(join(web, "lib", "README.md"), join(out, "README.md"));
  for (const f of ["LICENSE", "NOTICE.md", "COMMERCIAL.md", "TRADEMARKS.md"]) copyFileSync(join(root, f), join(out, f));
  copyFileSync(join(root, "book", "LICENSE.md"), join(out, "BOOK-LICENSE.md"));
  writeFileSync(join(out, "VERSION"), version());
  return out;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const out = await buildLib();
  console.log("built", out);
  console.log(readFileSync(join(out, "VERSION"), "utf8").trim());
}
