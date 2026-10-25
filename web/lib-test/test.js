// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
// The test page's own script: buttons that mount and destroy an editor and
// players, and `window.aratuTest`, which does it all and says what it saw:
// the listeners, elements, AudioContexts and page left after `destroy()`,
// where every file came from, what the CSP refused, and the styles drawn
// inside the editor (to compare with a page without the host's styles).
import { mount, mountPlayer } from "/static/aratu/v1/aratu.js";

const probe = window.__probe;
const ASSETS = "/static/aratu/v1/";
// The page's CSP lets in only <style> elements with this nonce.
const NONCE = document.querySelector('meta[name="test-nonce"]')?.content;
const PIECE = "tempo: 96\nbass: piano.grand\nbass = | dó3 mi3 sol3 dó4 |\nhh = c c c c";

let editor = null;
let players = [];

const $ = (id) => document.getElementById(id);

function mountEditor(opts = {}) {
  editor?.destroy();
  editor = mount($("editor"), {
    lang: "pt",
    assetBase: ASSETS,
    code: PIECE,
    features: { daw: false },
    nonce: NONCE,
    ...opts,
  });
  return editor;
}

function mountPlayers() {
  for (const p of players) p.destroy();
  // These find their files by where aratu.js was loaded from.
  players = [0, 1, 2].map((i) => mountPlayer($("players-slot"), PIECE.replace("96", String(90 + i * 10)), { lang: "pt" }));
  return players;
}

$("mount").onclick = () => mountEditor();
$("destroy").onclick = () => {
  editor?.destroy();
  editor = null;
};
$("players").onclick = () => mountPlayers();
$("unplayers").onclick = () => {
  for (const p of players) p.destroy();
  players = [];
};
$("dark").onclick = () => editor?.setTheme("dark");
$("es").onclick = () => editor?.setLang("es");

const PROPS = [
  "display",
  "position",
  "box-sizing",
  "margin-top",
  "margin-right",
  "margin-bottom",
  "margin-left",
  "padding-top",
  "padding-right",
  "padding-bottom",
  "padding-left",
  "border-top-width",
  "border-right-width",
  "border-bottom-width",
  "border-left-width",
  "border-top-style",
  "border-left-style",
  "border-top-color",
  "border-top-left-radius",
  "color",
  "background-color",
  "background-image",
  "font-family",
  "font-size",
  "font-style",
  "font-weight",
  "line-height",
  "letter-spacing",
  "word-spacing",
  "text-align",
  "text-transform",
  "text-decoration-line",
  "list-style-type",
  "min-width",
  "min-height",
  "vertical-align",
  "scrollbar-color",
];

/** How every element under `root` is drawn: its box and the styles that
 * a page's CSS could change. */
function fingerprint(root) {
  // As drawn once still: a page in the background may hold an animation or
  // a transition halfway.
  for (const a of document.getAnimations()) a.finish();
  const out = [];
  const origin = root.getBoundingClientRect();
  for (const el of [root, ...root.querySelectorAll("*")]) {
    const cs = getComputedStyle(el);
    const r = el.getBoundingClientRect();
    const name = `${el.tagName.toLowerCase()}${el.dataset.ref ? `[${el.dataset.ref}]` : ""}.${[...el.classList].join(".")}`;
    const box = `${Math.round(r.left - origin.left)},${Math.round(r.top - origin.top)} ${Math.round(r.width)}×${Math.round(r.height)}`;
    out.push([name, box, ...PROPS.map((p) => cs.getPropertyValue(p))].join(" | "));
  }
  return out;
}

const sleep = (ms) => new Promise((done) => setTimeout(done, ms));

/** The whole check, without sound (that needs a click). */
async function run() {
  const report = { ok: true, problems: [] };
  const fail = (what) => {
    report.ok = false;
    report.problems.push(what);
  };
  const page0 = probe.page();
  const listeners0 = new Set(probe.listeners);
  const host0 = ["host-heading", "host-text", "host-list", "mount"].map((id) => {
    const cs = getComputedStyle($(id));
    return PROPS.map((p) => cs.getPropertyValue(p)).join("|");
  });

  // The editor, at a deep route, its files under ASSETS.
  const e = mountEditor();
  await e.ready;
  await sleep(600);
  report.fingerprintPlay = fingerprint(e.root);
  if (document.documentElement.dataset.theme !== "host") fail("the editor changed <html data-theme>");
  e.setTheme("dark");
  e.setLang("es");
  if (e.root.dataset.theme !== "dark") fail("setTheme did not reach the root");
  if (document.documentElement.lang !== "en") fail("the editor changed <html lang>");
  e.setView("learn");
  await sleep(1500);
  report.fingerprintLearn = fingerprint(e.root);
  e.setTheme("light");
  e.setLang("pt");
  e.setView("play");
  await sleep(300);

  // Three players, finding their files by where aratu.js is.
  const ps = mountPlayers();
  await Promise.all(ps.map((p) => p.ready));
  report.fingerprintPlayer = fingerprint(ps[0].root);

  // Mounted and destroyed again and again: nothing piles up.
  for (let i = 0; i < 5; i++) {
    const again = mountEditor({ features: { daw: true } });
    await again.ready;
  }
  editor.destroy();
  editor = null;
  for (const p of players) p.destroy();
  players = [];
  await sleep(800);

  const left = [...probe.listeners].filter((l) => !listeners0.has(l));
  if (left.length) fail(`listeners left: ${left.join(", ")}`);
  if (document.querySelector(".aratu-root")) fail("an .aratu-root was left in the page");
  const page1 = probe.page();
  for (const k of Object.keys(page0)) if (page0[k] !== page1[k]) fail(`the page changed: ${k} ${page0[k]} → ${page1[k]}`);
  const host1 = ["host-heading", "host-text", "host-list", "mount"].map((id) => {
    const cs = getComputedStyle($(id));
    return PROPS.map((p) => cs.getPropertyValue(p)).join("|");
  });
  host0.forEach((s, i) => s !== host1[i] && fail(`a host element's style changed: #${["host-heading", "host-text", "host-list", "mount"][i]}`));
  const open = probe.contexts.filter((c) => c.state !== "closed");
  if (open.length) fail(`${open.length} AudioContext(s) not closed`);
  const elsewhere = probe.fetches.filter((u) => !new URL(u).pathname.startsWith(ASSETS));
  if (elsewhere.length) fail(`fetched outside ${ASSETS}: ${elsewhere.join(", ")}`);
  if (probe.worklets.some((u) => !u.startsWith(location.origin + ASSETS))) fail(`a worklet from elsewhere: ${probe.worklets}`);
  if (probe.violations.length) fail(`CSP refused: ${probe.violations.join(", ")}`);
  report.fetches = [...new Set(probe.fetches.map((u) => new URL(u).pathname))];
  report.contexts = probe.contexts.length;
  report.worklets = probe.worklets;
  return report;
}

/** What differs between two runs' drawings, leaving out what cannot be
 * seen: where a hidden element (no box) is, and the style of a border
 * that has no width; and what only the screen's scale changes: a border
 * of 1px drawn as 0.8, a box a pixel or two off. */
function compare(a, b) {
  const col = (name) => PROPS.indexOf(name) + 2;
  const widths = ["border-top-width", "border-right-width", "border-bottom-width", "border-left-width"].map(col);
  const unseen = ["border-top-style", "border-left-style", "border-top-color"].map(col);
  // An auto margin is what is left of a line, rounded to the screen.
  const margins = ["margin-top", "margin-right", "margin-bottom", "margin-left"].map(col);
  const px = (v) => Math.round(parseFloat(v));
  const near = (p, q) => {
    const m = (s) => (s.match(/-?\d+/g) ?? []).map(Number);
    const [u, w] = [m(p), m(q)];
    return u.length === w.length && u.every((n, i) => Math.abs(n - w[i]) <= 3);
  };
  const out = {};
  for (const k of Object.keys(a)) {
    const rows = [];
    for (let i = 0; i < Math.max(a[k].length, b[k].length); i++) {
      const x = (a[k][i] ?? "").split(" | ");
      const y = (b[k][i] ?? "").split(" | ");
      if (x[0] !== y[0]) {
        rows.push(`${i} ${x[0]} is ${y[0]}`);
        continue;
      }
      const hidden = / 0×0$/.test(x[1]) && / 0×0$/.test(y[1]);
      const noBorder = widths.every((j) => px(x[j]) === 0 && px(y[j]) === 0);
      const cols = [];
      x.forEach((v, j) => {
        if (v === y[j] || (j === 1 && (hidden || near(v, y[j])))) return;
        if (unseen.includes(j) && noBorder) return;
        if (widths.includes(j) && Math.abs(parseFloat(v) - parseFloat(y[j])) <= 1) return;
        if (margins.includes(j) && Math.abs(parseFloat(v) - parseFloat(y[j])) <= 3) return;
        cols.push(`${PROPS[j - 2] ?? "box"}: ${v} ≠ ${y[j]}`);
      });
      if (cols.length) rows.push(`${i} ${x[0]}: ${cols.join("; ")}`);
    }
    out[k] = { elements: a[k].length, differ: rows.length, first: rows.slice(0, 12) };
  }
  return out;
}

/** Runs, keeps the drawings under `name`, and compares them with those
 * kept under `against`. */
async function runAndCompare(name, against = "none") {
  const r = await run();
  const fp = { play: r.fingerprintPlay, learn: r.fingerprintLearn, player: r.fingerprintPlayer };
  localStorage.setItem(`fp:${name}`, JSON.stringify(fp));
  const other = JSON.parse(localStorage.getItem(`fp:${against}`) ?? "null");
  return { ok: r.ok, problems: r.problems, diff: other && name !== against ? compare(other, fp) : null };
}

window.aratuTest = {
  run,
  runAndCompare,
  compare,
  fingerprint,
  mountEditor,
  mountPlayers,
  probe,
  get editor() {
    return editor;
  },
  get players() {
    return players;
  },
};
