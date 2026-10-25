// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
// Serves a host page for Aratu as a library, the way an app would hold it:
// the page at a deep route (/app/…, any path under it), dist-lib/ under
// /static/aratu/v1/, and a Content Security Policy with no 'unsafe-inline',
// no 'unsafe-eval' and no blob: for scripts; styles only from files and
// from <style> elements with the page's nonce. Build the library first
// (npm run lib). Open http://localhost:5192/ (or --port=…), then run
// `await aratuTest.run()` in the console, or use the page's buttons.
import { randomBytes } from "node:crypto";
import { createReadStream, existsSync, readFileSync, statSync } from "node:fs";
import { createServer } from "node:http";
import { dirname, extname, join, normalize } from "node:path";
import { fileURLToPath } from "node:url";

const web = join(dirname(fileURLToPath(import.meta.url)), "..");
const lib = join(web, "dist-lib");
const test = join(web, "lib-test");
const port = Number(process.argv.find((a) => a.startsWith("--port="))?.slice(7) ?? 5192);

if (!existsSync(join(lib, "aratu.js"))) {
  console.error("dist-lib/ is not built: run npm run lib first.");
  process.exit(1);
}

const csp = (nonce = "") => [
  "default-src 'self'",
  "script-src 'self' 'wasm-unsafe-eval'",
  `style-src 'self'${nonce ? ` 'nonce-${nonce}'` : ""}`,
  "img-src 'self' data:",
  "font-src 'self'",
  "connect-src 'self'",
  "object-src 'none'",
  "base-uri 'self'",
].join("; ");
const CSP = csp();

const types = {
  ".html": "text/html; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".css": "text/css; charset=utf-8",
  ".wasm": "application/wasm",
  ".png": "image/png",
  ".json": "application/json; charset=utf-8",
  ".map": "application/json",
  ".sfz": "text/plain; charset=utf-8",
  ".flac": "audio/flac",
  ".ogg": "audio/ogg",
  ".wav": "audio/wav",
  ".md": "text/plain; charset=utf-8",
};

const send = (res, status, body, type = "text/plain; charset=utf-8") => {
  res.writeHead(status, { "Content-Type": type, "Content-Security-Policy": CSP, "Cache-Control": "no-store" });
  res.end(body);
};

/** A file under `dir`, or null when the path leaves it or is not there. */
const fileIn = (dir, rel) => {
  const path = normalize(join(dir, decodeURIComponent(rel)));
  if (!path.startsWith(dir) || !existsSync(path) || statSync(path).isDirectory()) return null;
  return path;
};

createServer((req, res) => {
  const url = new URL(req.url ?? "/", "http://localhost");
  const p = url.pathname;
  if (p === "/") {
    res.writeHead(302, { Location: "/app/pecas/minha-peca/42" }).end();
    return;
  }
  if (p.startsWith("/app/")) {
    // A nonce for this page, which the test hands to the editor for
    // CodeMirror's <style> element.
    const nonce = randomBytes(12).toString("base64");
    const html = readFileSync(join(test, "index.html"), "utf8").replace(
      "<head>",
      `<head>
    <meta name="test-nonce" content="${nonce}" />`,
    );
    res.writeHead(200, { "Content-Type": types[".html"], "Content-Security-Policy": csp(nonce), "Cache-Control": "no-store" });
    res.end(html);
    return;
  }
  let path = null;
  if (p.startsWith("/static/aratu/v1/")) path = fileIn(lib, p.slice("/static/aratu/v1/".length));
  else if (p.startsWith("/lib-test/")) {
    const rel = p.slice("/lib-test/".length);
    // A stylesheet put in the cascade layer "base", as Tailwind 4 has its own.
    const layered = /^(.+)\.layered\.css$/.exec(rel);
    if (layered) {
      const plain = fileIn(test, `${layered[1]}.css`);
      if (plain) return send(res, 200, `@layer base {\n${readFileSync(plain, "utf8")}\n}\n`, types[".css"]);
    }
    path = fileIn(test, rel);
  }
  if (!path) return send(res, 404, "not found");
  res.writeHead(200, {
    "Content-Type": types[extname(path)] ?? "application/octet-stream",
    "Content-Security-Policy": CSP,
    "Cache-Control": "no-store",
  });
  createReadStream(path).pipe(res);
}).listen(port, () => console.log(`Aratu in a host page: http://localhost:${port}/`));
