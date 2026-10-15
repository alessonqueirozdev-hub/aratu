// A development server: rebuilds on change and serves dist/ with the
// cross-origin isolation headers the audio engine will need.
import * as esbuild from "esbuild";
import { createReadStream, existsSync, statSync, watch } from "node:fs";
import { createServer } from "node:http";
import { extname, join, normalize } from "node:path";
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { buildWasm } from "./build-wasm.mjs";
import { copyStatic, options } from "./build.mjs";

const web = join(dirname(fileURLToPath(import.meta.url)), "..");
const dist = join(web, "dist");
const port = Number(process.env.PORT ?? 5173);

buildWasm();
copyStatic();
const ctx = await esbuild.context(options);
await ctx.watch();
const rebuild = () => {
  try {
    copyStatic();
  } catch (e) {
    console.error(e.message);
  }
};
for (const f of ["index.html", join("src", "style.css")]) {
  watch(join(web, f), rebuild);
}
watch(join(web, "..", "book"), { recursive: true }, rebuild);

const types = {
  ".html": "text/html; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".css": "text/css; charset=utf-8",
  ".wasm": "application/wasm",
  ".png": "image/png",
  ".svg": "image/svg+xml",
  ".map": "application/json",
  ".json": "application/json; charset=utf-8",
};

createServer((req, res) => {
  const url = new URL(req.url ?? "/", "http://localhost");
  let path = normalize(join(dist, decodeURIComponent(url.pathname)));
  if (!path.startsWith(dist)) {
    res.writeHead(403).end();
    return;
  }
  if (existsSync(path) && statSync(path).isDirectory()) path = join(path, "index.html");
  if (!existsSync(path)) {
    res.writeHead(404).end("not found");
    return;
  }
  res.writeHead(200, {
    "Content-Type": types[extname(path)] ?? "application/octet-stream",
    "Cross-Origin-Opener-Policy": "same-origin",
    "Cross-Origin-Embedder-Policy": "require-corp",
    "Cache-Control": "no-store",
  });
  createReadStream(path).pipe(res);
}).listen(port, () => console.log(`Aratu: http://localhost:${port}`));
