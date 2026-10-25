// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
// The effects of a chain, drawn where they are named: beside `eq`, `tube`,
// `drive`, `lowpass` and `highpass` in `bass: synth.saw -> eq -> tube`, a
// small curve worked out from the part's numbers, which follows them as the
// knobs turn. The equalizer's opens, on a click, into a larger one whose
// three bands are dragged: the numbers are rewritten in the text, or written
// under the chain when the piece has none yet.

import { type Range, StateEffect, StateField } from "@codemirror/state";
import { Decoration, type DecorationSet, EditorView, WidgetType } from "@codemirror/view";
import type { Control, Lang, PartInfo } from "./aratu";
import { tr } from "./i18n";

/** What the drawings need from the last compile. */
interface FxState {
  parts: PartInfo[];
  controls: Control[];
  lang: Lang;
}

/** The effects drawn, by the names a chain may give them. */
const NAMES: Record<string, string> = {
  eq: "eq",
  tube: "tube",
  valvula: "tube",
  "válvula": "tube",
  drive: "drive",
  lowpass: "lowpass",
  highpass: "highpass",
};

/** Each effect's numbers when the piece does not set them (Manguebyte's). */
const DEFAULTS: Record<string, number> = {
  "eq.low_gain": 0,
  "eq.mid_gain": 0,
  "eq.high_gain": 0,
  "eq.low_freq": 120,
  "eq.mid_freq": 1000,
  "eq.mid_q": 0.7,
  "eq.high_freq": 8000,
  "tube.drive": 8,
  "tube.output": -4,
  "tube.mix": 1,
  "drive.amount": 12,
  "drive.output": -6,
  "drive.mix": 1,
  "drive.shape": 0,
  "lowpass.cutoff": 2000,
  "lowpass.resonance": 0.2,
  "highpass.cutoff": 200,
  "highpass.resonance": 0,
};

type Values = (id: string) => number;

function valuesOf(state: FxState, part: string): Values {
  const set = state.parts.find((p) => p.name === part)?.params ?? [];
  return (id) => set.find((p) => p.id === id)?.value ?? DEFAULTS[id] ?? 0;
}

// The curves, as points from 0 to 1 across and up.

const SPAN = [20, 20000];
const fx = (t: number) => SPAN[0] * Math.pow(SPAN[1] / SPAN[0], t);

/** The response in decibels of an RBJ biquad at frequency f. */
function biquadDb(b: number[], a: number[], f: number, rate = 48000): number {
  const w = (2 * Math.PI * f) / rate;
  const re = (c: number[]) => c[0] + c[1] * Math.cos(w) + c[2] * Math.cos(2 * w);
  const im = (c: number[]) => -(c[1] * Math.sin(w) + c[2] * Math.sin(2 * w));
  const num = Math.hypot(re(b), im(b));
  const den = Math.hypot(re(a), im(a));
  return 20 * Math.log10(num / den);
}

function shelf(f0: number, db: number, high: boolean): [number[], number[]] {
  const A = Math.pow(10, db / 40);
  const w = (2 * Math.PI * f0) / 48000;
  const alpha = (Math.sin(w) / 2) * Math.SQRT2;
  const c = Math.cos(w);
  const s = 2 * Math.sqrt(A) * alpha;
  const sign = high ? -1 : 1;
  const b = [
    A * (A + 1 - sign * (A - 1) * c + s),
    sign * 2 * A * (A - 1 - sign * (A + 1) * c),
    A * (A + 1 - sign * (A - 1) * c - s),
  ];
  const a = [A + 1 + sign * (A - 1) * c + s, -sign * 2 * (A - 1 + sign * (A + 1) * c), A + 1 + sign * (A - 1) * c - s];
  return [b, a];
}

function peak(f0: number, db: number, q: number): [number[], number[]] {
  const A = Math.pow(10, db / 40);
  const w = (2 * Math.PI * f0) / 48000;
  const alpha = Math.sin(w) / (2 * Math.max(q, 0.05));
  const c = Math.cos(w);
  return [
    [1 + alpha * A, -2 * c, 1 - alpha * A],
    [1 + alpha / A, -2 * c, 1 - alpha / A],
  ];
}

function pass(f0: number, resonance: number, high: boolean): [number[], number[]] {
  const q = 0.707 + resonance * 6;
  const w = (2 * Math.PI * Math.min(f0, 20000)) / 48000;
  const alpha = Math.sin(w) / (2 * q);
  const c = Math.cos(w);
  const b = high ? [(1 + c) / 2, -(1 + c), (1 + c) / 2] : [(1 - c) / 2, 1 - c, (1 - c) / 2];
  return [b, [1 + alpha, -2 * c, 1 - alpha]];
}

/** The equalizer's response at f, in dB. */
function eqDb(v: Values, f: number): number {
  const bands = [
    shelf(v("eq.low_freq"), v("eq.low_gain"), false),
    peak(v("eq.mid_freq"), v("eq.mid_gain"), v("eq.mid_q")),
    shelf(v("eq.high_freq"), v("eq.high_gain"), true),
  ];
  return bands.reduce((sum, [b, a]) => sum + biquadDb(b, a, f), 0);
}

/** A drive's curve: what comes out for what goes in, -1 to 1. */
function transfer(kind: string, v: Values): (x: number) => number {
  const prefix = kind === "tube" ? "tube" : "drive";
  const gain = Math.pow(10, v(kind === "tube" ? "tube.drive" : "drive.amount") / 20);
  const mix = v(`${prefix}.mix`);
  const shape = kind === "tube" ? 4 : Math.round(v("drive.shape"));
  const curve = (x: number) => {
    const g = x * gain;
    switch (shape) {
      case 1:
        return Math.abs(g) < 1 ? g - (g * g * g) / 3 : Math.sign(g) * (2 / 3);
      case 2:
        return Math.max(-1, Math.min(1, g));
      case 3:
        return Math.sin(g);
      case 4:
        // Manguebyte's tube: zero and unity slope at rest, flatter above.
        return 0.75 * (Math.tanh(1.5 * g + Math.LN2 / 2) - 1 / 3);
      default:
        return Math.tanh(g);
    }
  };
  const top = Math.max(Math.abs(curve(1)), Math.abs(curve(-1)), 1e-6);
  return (x) => (1 - mix) * x + (mix * curve(x)) / top;
}

/** The curve of an effect as points, x and y from 0 to 1 (y up). */
function points(kind: string, v: Values, n = 48): [number, number][] {
  const out: [number, number][] = [];
  for (let i = 0; i <= n; i++) {
    const t = i / n;
    if (kind === "eq") {
      out.push([t, 0.5 + Math.max(-1, Math.min(1, eqDb(v, fx(t)) / 18)) / 2]);
    } else if (kind === "lowpass" || kind === "highpass") {
      const [b, a] = pass(v(`${kind}.cutoff`), v(`${kind}.resonance`), kind === "highpass");
      const db = biquadDb(b, a, fx(t));
      out.push([t, Math.max(0, Math.min(1, (db + 30) / 42))]);
    } else {
      const x = t * 2 - 1;
      out.push([t, 0.5 + transfer(kind, v)(x) / 2]);
    }
  }
  return out;
}

function path(pts: [number, number][], w: number, h: number, pad = 1.5): string {
  return pts
    .map(([x, y], i) => `${i ? "L" : "M"}${(pad + x * (w - 2 * pad)).toFixed(1)},${(pad + (1 - y) * (h - 2 * pad)).toFixed(1)}`)
    .join("");
}

const SVG = "http://www.w3.org/2000/svg";

function svg(w: number, h: number): SVGSVGElement {
  const s = document.createElementNS(SVG, "svg");
  s.setAttribute("viewBox", `0 0 ${w} ${h}`);
  s.setAttribute("width", String(w));
  s.setAttribute("height", String(h));
  return s;
}

function line(d: string, cls: string): SVGPathElement {
  const p = document.createElementNS(SVG, "path");
  p.setAttribute("d", d);
  p.setAttribute("class", cls);
  return p;
}

// The small curve beside the effect's name.

class FxWidget extends WidgetType {
  constructor(
    readonly kind: string,
    readonly part: string,
    readonly sig: string,
    readonly state: FxState,
  ) {
    super();
  }

  eq(other: FxWidget): boolean {
    return other.kind === this.kind && other.part === this.part && other.sig === this.sig;
  }

  toDOM(view: EditorView): HTMLElement {
    const box = document.createElement("span");
    box.className = `cm-fx cm-fx-${this.kind}`;
    const v = valuesOf(this.state, this.part);
    const s = svg(46, 16);
    if (this.kind === "eq") s.append(line("M0,8H46", "cm-fx-zero"));
    s.append(line(path(points(this.kind, v, 32), 46, 16), "cm-fx-curve"));
    box.append(s);
    if (this.kind === "tube") {
      // The valve glows as hard as it is driven.
      const heat = Math.min(1, v("tube.drive") / 24);
      box.style.setProperty("--heat", heat.toFixed(2));
    }
    if (this.kind === "eq") {
      box.title = tr("eqOpen", this.state.lang);
      box.onmousedown = (e) => {
        e.preventDefault();
        openEq(view, this.part, this.state, box);
      };
    }
    return box;
  }

  ignoreEvent(): boolean {
    return true;
  }
}

const setFx = StateEffect.define<FxState>();

/** Where each effect a chain names is written, with its part. */
function scan(doc: string): { kind: string; part: string; at: number }[] {
  const found: { kind: string; part: string; at: number }[] = [];
  const re = /^([ \t]*)([\p{L}_][\p{L}\p{N}_]*)[ \t]*:[ \t]*([^\n]*->[^\n]*)$/gmu;
  for (const m of doc.matchAll(re)) {
    const start = (m.index ?? 0) + m[0].length - m[3].length;
    const units = /[^\s>-][^\s>]*/g;
    for (const u of m[3].matchAll(units)) {
      const word = u[0].replace(/-+$/, "");
      const kind = NAMES[word.toLowerCase()];
      if (kind) found.push({ kind, part: m[2], at: start + (u.index ?? 0) + word.length });
    }
  }
  return found;
}

function build(doc: string, state: FxState | null): DecorationSet {
  if (!state) return Decoration.none;
  const marks: Range<Decoration>[] = [];
  for (const f of scan(doc)) {
    const v = valuesOf(state, f.part);
    const ids = Object.keys(DEFAULTS).filter((id) => id.startsWith(`${f.kind}.`));
    const sig = ids.map((id) => v(id)).join(",");
    marks.push(Decoration.widget({ widget: new FxWidget(f.kind, f.part, sig, state), side: 1 }).range(f.at));
  }
  return Decoration.set(marks, true);
}

const fxField = StateField.define<{ state: FxState | null; set: DecorationSet }>({
  create: () => ({ state: null, set: Decoration.none }),
  update(value, tr) {
    let state = value.state;
    for (const e of tr.effects) if (e.is(setFx)) state = e.value;
    if (state === value.state && !tr.docChanged) return value;
    return { state, set: build(tr.state.doc.toString(), state) };
  },
  provide: (f) => EditorView.decorations.from(f, (v) => v.set),
});

export const effects = [fxField];

export function showFx(view: EditorView, state: FxState): void {
  view.dispatch({ effects: setFx.of(state) });
}

// The equalizer, opened.

const BANDS = [
  { name: "low", word: "eqLow" as const },
  { name: "mid", word: "eqMid" as const },
  { name: "high", word: "eqHigh" as const },
];

/** The equalizer open on the page, if any, and how to take away the
 * listeners that close it. */
let open: { panel: HTMLElement; off: () => void } | null = null;

/** Closes the equalizer, if one is open (under `root`, when given). */
export function closeEq(root?: HTMLElement): void {
  if (root && open && !root.contains(open.panel)) return;
  open?.off();
  open?.panel.remove();
  open = null;
}

function number(v: number): string {
  const r = Math.abs(v) >= 100 ? Math.round(v) : Math.round(v * 10) / 10;
  return String(r);
}

function openEq(view: EditorView, part: string, state: FxState, anchor: HTMLElement): void {
  closeEq();
  const lang = state.lang;
  const W = 320;
  const H = 150;
  const panel = document.createElement("div");
  panel.className = "eq-panel";
  const head = document.createElement("div");
  head.className = "eq-head";
  const title = document.createElement("b");
  title.textContent = `${tr("eqTitle", lang)} · ${part}`;
  const close = document.createElement("button");
  close.className = "eq-close";
  close.textContent = "✕";
  close.setAttribute("aria-label", tr("soundClose", lang));
  close.onclick = () => closeEq();
  head.append(title, close);
  const s = svg(W, H);
  s.classList.add("eq-graph");
  // Lines at 100 Hz, 1 kHz and 10 kHz, and at 0 dB.
  const xOf = (f: number) => (Math.log(f / SPAN[0]) / Math.log(SPAN[1] / SPAN[0])) * W;
  const yOf = (db: number) => H / 2 - (db / 24) * (H / 2 - 8);
  for (const f of [100, 1000, 10000]) s.append(line(`M${xOf(f).toFixed(1)},0V${H}`, "eq-grid"));
  s.append(line(`M0,${H / 2}H${W}`, "eq-grid eq-zero"));
  const curve = line("", "eq-curve");
  s.append(curve);
  const values = new Map<string, number>();
  const v: Values = (id) => values.get(id) ?? valuesOf(state, part)(id);
  const draw = () => {
    const pts: [number, number][] = [];
    for (let i = 0; i <= 96; i++) {
      const t = i / 96;
      pts.push([t, 0.5 + Math.max(-1, Math.min(1, eqDb(v, fx(t)) / 24)) * ((H / 2 - 8) / H)]);
    }
    curve.setAttribute("d", path(pts, W, H, 0));
    for (const b of BANDS) {
      const dot = handles.get(b.name);
      if (!dot) continue;
      dot.setAttribute("cx", xOf(v(`eq.${b.name}_freq`)).toFixed(1));
      dot.setAttribute("cy", yOf(v(`eq.${b.name}_gain`)).toFixed(1));
    }
    legend.textContent = BANDS.map(
      (b) => `${tr(b.word, lang)} ${number(v(`eq.${b.name}_gain`))} dB · ${number(v(`eq.${b.name}_freq`))} Hz`,
    ).join("\n");
  };
  const legend = document.createElement("p");
  legend.className = "eq-legend";
  const handles = new Map<string, SVGCircleElement>();
  // Where each number is written, kept up to date as the text changes.
  const written = new Map<string, { from: number; to: number; factor: number }>();
  for (const c of state.controls) {
    if (c.part === part && c.id.startsWith("eq.")) written.set(c.id, { from: c.from, to: c.to, factor: c.factor });
  }
  const write = (set: Record<string, number>) => {
    const doc = view.state.doc;
    const changes: { from: number; to: number; insert: string }[] = [];
    const missing: string[] = [];
    for (const [id, value] of Object.entries(set)) {
      values.set(id, value);
      const w = written.get(id);
      if (w) changes.push({ from: w.from, to: w.to, insert: number(value * w.factor) });
      else missing.push(id);
    }
    // Numbers not written yet go under the part's chain.
    let at = -1;
    if (missing.length > 0) {
      const chain = scan(doc.toString()).find((f) => f.part === part && f.kind === "eq");
      if (chain) at = doc.lineAt(chain.at).to;
    }
    const added: { id: string; offset: number; length: number }[] = [];
    let text = "";
    if (at >= 0) {
      for (const id of missing) {
        const unit = id.endsWith("_freq") ? " Hz" : id.endsWith("_gain") ? " dB" : "";
        const head = `\n${part}.${id} = `;
        const n = number(set[id]);
        added.push({ id, offset: text.length + head.length, length: n.length });
        text += `${head}${n}${unit}`;
      }
      changes.push({ from: at, to: at, insert: text });
    }
    changes.sort((a, b) => a.from - b.from);
    const tx = view.state.update({ changes, userEvent: "input.eq" });
    for (const [id, w] of written) {
      const replaced = changes.find((c) => c.from === w.from && c.to === w.to);
      const from = tx.changes.mapPos(w.from, -1);
      const to = replaced ? from + replaced.insert.length : tx.changes.mapPos(w.to, 1);
      written.set(id, { ...w, from, to });
    }
    if (at >= 0) {
      const base = tx.changes.mapPos(at, -1);
      for (const a of added) written.set(a.id, { from: base + a.offset, to: base + a.offset + a.length, factor: 1 });
    }
    view.dispatch(tx);
  };
  for (const b of BANDS) {
    const dot = document.createElementNS(SVG, "circle");
    dot.setAttribute("r", "7");
    dot.setAttribute("class", `eq-band eq-${b.name}`);
    const label = document.createElementNS(SVG, "title");
    label.textContent = tr(b.word, lang);
    dot.append(label);
    dot.addEventListener("pointerdown", (down) => {
      down.preventDefault();
      dot.setPointerCapture(down.pointerId);
      const rect = s.getBoundingClientRect();
      const range: Record<string, [number, number]> = { low: [20, 1000], mid: [100, 10000], high: [1000, 20000] };
      const move = (e: PointerEvent) => {
        const x = Math.max(0, Math.min(1, (e.clientX - rect.left) / rect.width));
        const y = (e.clientY - rect.top) / rect.height;
        const [lo, hi] = range[b.name];
        const f = Math.max(lo, Math.min(hi, fx(x)));
        const db = Math.max(-24, Math.min(24, ((0.5 - y) * H) / (H / 2 - 8) * 24));
        write({ [`eq.${b.name}_freq`]: Math.round(f), [`eq.${b.name}_gain`]: Math.round(db * 2) / 2 });
        draw();
      };
      const up = () => {
        dot.removeEventListener("pointermove", move);
        dot.removeEventListener("pointerup", up);
      };
      dot.addEventListener("pointermove", move);
      dot.addEventListener("pointerup", up);
    });
    // The wheel narrows or widens the middle band.
    if (b.name === "mid") {
      dot.addEventListener("wheel", (e) => {
        e.preventDefault();
        const q = Math.max(0.1, Math.min(10, v("eq.mid_q") * (e.deltaY < 0 ? 1.15 : 1 / 1.15)));
        write({ "eq.mid_q": Math.round(q * 100) / 100 });
        draw();
      });
    }
    handles.set(b.name, dot);
    s.append(dot);
  }
  panel.append(head, s, legend);
  const hint = document.createElement("p");
  hint.className = "eq-hint";
  hint.textContent = tr("eqHint", lang);
  panel.append(hint);
  // In the editor's root, so its styles (and full screen, on stage) reach
  // it; placed in the root's own terms, inside the window.
  const host = view.dom.closest<HTMLElement>(".aratu-root") ?? document.body;
  host.append(panel);
  const r = anchor.getBoundingClientRect();
  const box = host === document.body ? { left: -window.scrollX, top: -window.scrollY } : host.getBoundingClientRect();
  panel.style.left = `${Math.max(8, Math.min(window.innerWidth - W - 40, r.left)) - box.left}px`;
  panel.style.top = `${Math.min(window.innerHeight - 260, r.bottom + 6) - box.top}px`;
  const away = (e: Event) => {
    if (!panel.contains(e.target as Node)) closeEq();
  };
  const key = (e: KeyboardEvent) => {
    if (e.key === "Escape") closeEq();
  };
  // Not the click that opened it.
  const later = window.setTimeout(() => document.addEventListener("pointerdown", away, true), 0);
  document.addEventListener("keydown", key);
  open = {
    panel,
    off: () => {
      window.clearTimeout(later);
      document.removeEventListener("pointerdown", away, true);
      document.removeEventListener("keydown", key);
    },
  };
  draw();
}
