// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
// Builds the editor into dist/: the wasm module, the bundle, the page, and
// the book's pages to read with the site map (site.mjs).
import * as esbuild from "esbuild";
import { createHash } from "node:crypto";
import { cpSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { buildBook } from "./book.mjs";
import { buildWasm } from "./build-wasm.mjs";
import { SITE_URL, buildSite } from "./site.mjs";

const web = join(dirname(fileURLToPath(import.meta.url)), "..");
const dist = join(web, "dist");

export const options = {
  entryPoints: [join(web, "src", "main.ts")],
  bundle: true,
  format: "esm",
  target: "es2022",
  outfile: join(dist, "main.js"),
  sourcemap: true,
  loader: { ".aratu": "text" },
};

export function copyStatic() {
  mkdirSync(dist, { recursive: true });
  cpSync(join(web, "public"), dist, { recursive: true });
  // The page's absolute links (canonical, Open Graph) need the site's address.
  writeFileSync(join(dist, "index.html"), readFileSync(join(web, "index.html"), "utf8").replaceAll("%SITE_URL%", SITE_URL));
  cpSync(join(web, "src", "style.css"), join(dist, "style.css"));
  buildBook(dist);
  buildSite(dist);
}

/** Stamps the page's script and style with a version made from their
 * content, so a browser that kept an older copy fetches the new one as
 * soon as the page changes. The script passes its stamp on to the wasm. */
export function stampVersions() {
  const hash = createHash("sha256");
  for (const f of ["main.js", "style.css", "aratu.wasm"]) hash.update(readFileSync(join(dist, f)));
  const v = hash.digest("hex").slice(0, 10);
  const page = join(dist, "index.html");
  const html = readFileSync(page, "utf8")
    .replace('href="style.css"', `href="style.css?v=${v}"`)
    .replace('src="main.js"', `src="main.js?v=${v}"`);
  writeFileSync(page, html);
  return v;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  buildWasm();
  rmSync(dist, { recursive: true, force: true });
  copyStatic();
  await esbuild.build({ ...options, minify: true });
  console.log("built", dist, "version", stampVersions());
}
