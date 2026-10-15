// Builds the editor into dist/: the wasm module, the bundle, the page.
import * as esbuild from "esbuild";
import { cpSync, mkdirSync, rmSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { buildBook } from "./book.mjs";
import { buildWasm } from "./build-wasm.mjs";

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
  cpSync(join(web, "index.html"), join(dist, "index.html"));
  cpSync(join(web, "src", "style.css"), join(dist, "style.css"));
  buildBook(dist);
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  buildWasm();
  rmSync(dist, { recursive: true, force: true });
  copyStatic();
  await esbuild.build({ ...options, minify: true });
  console.log("built", dist);
}
