// Draws the picture a link to the site shows when shared (Open Graph,
// Twitter): one 1200×630 card per language, written to public/og-<lang>.png
// and committed, so the build only copies it. Run it again after changing
// the card: `node scripts/og.mjs`. It draws the card as a page and takes a
// picture of it with a Chromium browser (Chrome or Edge) found on the
// machine, or the one ARATU_BROWSER names.
import { execFileSync } from "node:child_process";
import { copyFileSync, existsSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { LANGS, escape } from "./book.mjs";
import { tr } from "../src/i18n.ts";
import { SHARE_IMAGE, shareImageFile } from "./site.mjs";

const web = join(dirname(fileURLToPath(import.meta.url)), "..");
const mascot = join(web, "..", "docs", "images", "aratu-mascot.png");

/** Social sites take a card up to a few hundred kilobytes without fuss. */
const MAX_BYTES = 300 * 1024;

/** The maracatu from the course's last lesson, in each language's words. */
const SNIPPET = {
  pt: ["tempo: 92", "alfaia = O -o o_oo o", "caixa  = Ssss Ssss Ssss Ssss", "gonguê = o -o -m o"],
  es: ["tempo: 92", "alfaia = O -o o_oo o", "caja   = Ssss Ssss Ssss Ssss", "gonguê = o -o -m o"],
  en: ["tempo: 92", "alfaia = O -o o_oo o", "snare  = Ssss Ssss Ssss Ssss", "gongue = o -o -m o"],
};

/** A line of Aratu in the editor's colours. */
function highlight(line) {
  const setting = line.match(/^(\w+:)(\s*)(.*)$/);
  if (setting) return `<b class="kw">${setting[1]}</b>${setting[2]}<span class="num">${escape(setting[3])}</span>`;
  const [, name, gap, pattern] = line.match(/^(\S+)(\s*=\s*)(.*)$/);
  const hits = escape(pattern).replace(/[-_]+/g, (m) => `<span class="hold">${m}</span>`);
  return `<b class="part">${escape(name)}</b><span class="op">${gap}</span>${hits}`;
}

function card(lang) {
  const style = readFileSync(join(web, "src", "style.css"), "utf8");
  // The light theme's colours, as the editor has them.
  const colours = style.match(/:root,\s*:root\[data-theme="light"\]\s*\{[^}]*\}/)[0];
  return `<!doctype html>
<html lang="${lang}">
<meta charset="utf-8" />
<style>
${colours}
* { box-sizing: border-box; margin: 0; }
html, body { width: ${SHARE_IMAGE.width}px; height: ${SHARE_IMAGE.height}px; overflow: hidden; }
body {
  position: relative;
  background-color: var(--bg);
  color: var(--text);
  font-family: system-ui, -apple-system, "Segoe UI", Roboto, "Helvetica Neue", sans-serif;
}
.mascot { position: absolute; left: 12px; top: 18px; width: 570px; height: 570px; }
.text { position: absolute; left: 604px; right: 56px; top: 0; bottom: 0;
  display: flex; flex-direction: column; justify-content: center; gap: 22px; }
.name { font-family: "Iowan Old Style", "Palatino Linotype", Palatino, "Book Antiqua", Georgia, serif;
  font-size: 128px; font-weight: 700; line-height: 0.9; color: var(--accent); letter-spacing: -1px; }
.tagline { font-size: 33px; line-height: 1.22; color: var(--soft); }
pre { font-family: "JetBrains Mono", "Cascadia Code", "Fira Code", Consolas, monospace;
  font-size: 21px; line-height: 1.5; padding: 16px 22px; color: var(--text);
  background: var(--paper); border: 1px solid var(--line); border-left: 4px solid var(--accent);
  border-radius: 10px; box-shadow: var(--shadow); }
.kw { color: var(--keyword); font-weight: 600; }
.num { color: var(--number); }
.part { color: var(--name); font-weight: 600; }
.op { color: var(--operator); }
.hold { color: var(--hold); }
.url { font-size: 28px; font-weight: 700; color: var(--accent); letter-spacing: 0.5px; }
</style>
<body>
  <img class="mascot" src="mascot.png" alt="" />
  <div class="text">
    <div class="name">Aratu</div>
    <div class="tagline">${escape(tr("tagline", lang))}</div>
    <pre>${SNIPPET[lang].map(highlight).join("\n")}</pre>
    <div class="url">aratu.live</div>
  </div>
</body>
</html>
`;
}

/** A Chromium browser on this machine, or the one ARATU_BROWSER names. */
export function browser() {
  const found = [
    process.env.ARATU_BROWSER,
    "C:/Program Files/Google/Chrome/Application/chrome.exe",
    "C:/Program Files (x86)/Microsoft/Edge/Application/msedge.exe",
    "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
    "/usr/bin/google-chrome",
    "/usr/bin/chromium",
    "/usr/bin/chromium-browser",
  ].find((p) => p && existsSync(p));
  if (!found) throw new Error("no Chrome, Edge or Chromium found: set ARATU_BROWSER to one");
  return found;
}

/** A PNG's width and height, from its header. */
export function pngSize(file) {
  const b = readFileSync(file);
  return { width: b.readUInt32BE(16), height: b.readUInt32BE(20) };
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const exe = browser();
  const scratch = mkdtempSync(join(tmpdir(), "aratu-og-"));
  try {
    copyFileSync(mascot, join(scratch, "mascot.png"));
    for (const lang of LANGS) {
      const page = join(scratch, `${lang}.html`);
      writeFileSync(page, card(lang));
      const out = join(web, "public", shareImageFile(lang));
      execFileSync(exe, [
        "--headless=new",
        "--disable-gpu",
        "--hide-scrollbars",
        "--no-first-run",
        "--no-default-browser-check",
        `--user-data-dir=${join(scratch, "profile")}`,
        "--force-device-scale-factor=1",
        `--window-size=${SHARE_IMAGE.width},${SHARE_IMAGE.height}`,
        `--screenshot=${out}`,
        pathToFileURL(page).href,
      ], { stdio: "ignore" });
      const { width, height } = pngSize(out);
      const bytes = readFileSync(out).length;
      if (width !== SHARE_IMAGE.width || height !== SHARE_IMAGE.height) {
        throw new Error(`${out}: ${width}×${height}, not ${SHARE_IMAGE.width}×${SHARE_IMAGE.height}`);
      }
      if (bytes > MAX_BYTES) throw new Error(`${out}: ${bytes} bytes, more than ${MAX_BYTES}`);
      console.log(`${out}: ${width}×${height}, ${Math.round(bytes / 1024)} KB`);
    }
  } finally {
    rmSync(scratch, { recursive: true, force: true });
  }
}
