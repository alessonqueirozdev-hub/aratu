// Drawings under a line of the code, where `bass.wave` or `arp.roll` is
// written: the part's wave, as an oscilloscope shows it, or its roll, the
// notes of the bar passing under a playhead, a row for each pitch or drum.
//
// Manguebyte mixes the parts inside its engine and hands over the whole mix,
// so a part's own sound cannot be measured yet: the wave is drawn from what
// the part plays (each note's pitch, strength and length, a drum's kind),
// held still on the lowest note as a scope holds a steady tone. The roll is
// the piece's own events.

import { type Range, StateEffect, StateField } from "@codemirror/state";
import { Decoration, type DecorationSet, EditorView, WidgetType } from "@codemirror/view";
import type { AratuEvent, BarInfo, Visual } from "./aratu";

/** A sounding event, with the audio-clock time it starts at. */
export interface Heard {
  event: AratuEvent;
  at: number;
}

/** What a session lends its drawings while it plays. */
export interface VisualSource {
  /** The audio clock, or null when not playing. */
  now(): number | null;
  /** The bar sounding at \`now\`, with its start on the audio clock, and
   * every event of it and of the bars around it. */
  sounding(now: number): { info: BarInfo; at: number; events: Heard[] } | null;
  /** A part's sound (\`synth.saw\`, \`drums.808\`), for the wave's shape. */
  voice(part: string): string;
}

const sources = new WeakMap<EditorView, VisualSource>();
const canvases = new Map<HTMLCanvasElement, EditorView>();

export function lendVisuals(view: EditorView, source: VisualSource): void {
  sources.set(view, source);
}

class VisualWidget extends WidgetType {
  constructor(
    readonly target: string,
    readonly kind: "wave" | "roll",
  ) {
    super();
  }

  eq(other: VisualWidget): boolean {
    return other.target === this.target && other.kind === this.kind;
  }

  toDOM(view: EditorView): HTMLElement {
    const box = document.createElement("div");
    box.className = `cm-visual cm-visual-${this.kind}`;
    const canvas = document.createElement("canvas");
    canvas.dataset.target = this.target;
    canvas.dataset.kind = this.kind;
    box.append(canvas);
    canvases.set(canvas, view);
    return box;
  }

  destroy(dom: HTMLElement): void {
    const canvas = dom.querySelector("canvas");
    if (canvas) canvases.delete(canvas);
  }

  get estimatedHeight(): number {
    return this.kind === "wave" ? 50 : 66;
  }

  ignoreEvent(): boolean {
    return true;
  }
}

const setVisuals = StateEffect.define<Visual[]>();

const visualField = StateField.define<DecorationSet>({
  create: () => Decoration.none,
  update(set, tr) {
    set = set.map(tr.changes);
    for (const e of tr.effects) {
      if (!e.is(setVisuals)) continue;
      const doc = tr.state.doc;
      const marks: Range<Decoration>[] = [];
      for (const v of e.value) {
        if (v.to > doc.length) continue;
        const line = doc.lineAt(v.to);
        marks.push(
          Decoration.widget({ widget: new VisualWidget(v.target, v.kind), block: true, side: 1 }).range(line.to),
        );
      }
      set = Decoration.set(marks, true);
    }
    return set;
  },
  provide: (f) => EditorView.decorations.from(f),
});

export const visuals = [visualField];

export function showVisuals(view: EditorView, list: Visual[]): void {
  view.dispatch({ effects: setVisuals.of(list) });
}

// One loop draws every drawing on the page while something plays.
let frame = 0;

export function startVisuals(): void {
  if (!frame) frame = requestAnimationFrame(draw);
}

export function stopVisuals(): void {
  cancelAnimationFrame(frame);
  frame = 0;
  for (const c of canvases.keys()) c.getContext("2d")?.clearRect(0, 0, c.width, c.height);
}

function draw(): void {
  frame = requestAnimationFrame(draw);
  for (const [c, view] of canvases) {
    if (!c.isConnected) {
      canvases.delete(c);
      continue;
    }
    const source = sources.get(view);
    const now = source?.now() ?? null;
    const g = sized(c);
    if (!g) continue;
    g.ctx.clearRect(0, 0, g.w, g.h);
    if (!source || now === null) continue;
    const bar = source.sounding(now);
    if (!bar) continue;
    const target = c.dataset.target ?? "";
    const mine = bar.events.filter(
      (e) => e.event.param === undefined && (e.event.part === target || e.event.line === target),
    );
    const color = getComputedStyle(c).color;
    g.ctx.strokeStyle = color;
    g.ctx.fillStyle = color;
    if (c.dataset.kind === "roll") roll(g, bar, mine, now, color);
    else wave(g, mine, now, source.voice(mine[0]?.event.part ?? target));
  }
}

interface Sized {
  ctx: CanvasRenderingContext2D;
  w: number;
  h: number;
  dpr: number;
}

function sized(c: HTMLCanvasElement): Sized | null {
  if (c.clientWidth === 0) return null;
  const dpr = Math.min(2, window.devicePixelRatio || 1);
  const w = Math.round(c.clientWidth * dpr);
  const h = Math.round(c.clientHeight * dpr);
  if (c.width !== w || c.height !== h) {
    c.width = w;
    c.height = h;
  }
  const ctx = c.getContext("2d");
  return ctx ? { ctx, w, h, dpr } : null;
}

// The wave.

/** Noise that is the same for the same instant. */
function noise(t: number): number {
  const n = Math.sin(Math.floor(t * 44100) * 12.9898) * 43758.5453;
  return (n - Math.floor(n)) * 2 - 1;
}

/** How loud a note is, \`t\` seconds after it starts. */
function envelope(t: number, dur: number): number {
  if (t < 0) return 0;
  if (t < 0.006) return t / 0.006;
  const held = 0.65 + 0.35 * Math.exp(-(t - 0.006) / 0.15);
  if (t < dur) return held;
  return held * Math.exp(-(t - dur) / 0.12);
}

type Shape = (cycles: number) => number;

const sine: Shape = (x) => Math.sin(2 * Math.PI * x);
const saw: Shape = (x) => 2 * (x - Math.floor(x + 0.5));
const square: Shape = (x) => (x - Math.floor(x) < 0.5 ? 1 : -1);
const rich: Shape = (x) => 0.7 * sine(x) + 0.35 * sine(2 * x) + 0.2 * sine(3 * x);

function shapeOf(voice: string): Shape {
  if (/supersaw/.test(voice)) return (x) => (saw(x) + saw(x * 1.006) + saw(x * 0.994)) / 2.2;
  if (/saw|brass|strings|pad|acid|bass\.synth|lead/.test(voice)) return saw;
  if (/square/.test(voice)) return square;
  if (/sine|808|organ|recorder|flute/.test(voice)) return sine;
  return rich;
}

/** A drum's sound at \`t\` seconds after the hit. */
function drumAt(drum: string, t: number, abs: number): number {
  if (t < 0) return 0;
  if (/kick|bd|alfaia|surdo|bombo|zabumba/.test(drum)) {
    const f = 50 + 110 * Math.exp(-t / 0.04);
    return Math.sin(2 * Math.PI * f * t) * Math.exp(-t / 0.25);
  }
  if (/snare|sd|clap|caixa|rim|stick|tarol/.test(drum)) {
    return (0.6 * noise(abs) + 0.4 * Math.sin(2 * Math.PI * 190 * t)) * Math.exp(-t / 0.12);
  }
  if (/tom/.test(drum)) return Math.sin(2 * Math.PI * (110 + 60 * Math.exp(-t / 0.05)) * t) * Math.exp(-t / 0.3);
  const long = /open|crash|ride|gongue|agog/.test(drum);
  return noise(abs) * Math.exp(-t / (long ? 0.3 : 0.05)) * 0.8;
}

const freq = (note: number) => 440 * Math.pow(2, (note - 69) / 12);

function wave(g: Sized, events: Heard[], now: number, voice: string): void {
  const { ctx, w, h, dpr } = g;
  const live = events.filter((e) => e.at <= now && now < e.at + e.event.dur + 0.6);
  const notes = live.filter((e) => e.event.note !== undefined);
  // Held on the lowest note: about three of its cycles across.
  const low = notes.reduce<Heard | null>((a, e) => (!a || (e.event.note ?? 0) < (a.event.note ?? 0) ? e : a), null);
  const f0 = low?.event.note !== undefined ? freq(low.event.note) : 0;
  const span = f0 > 0 ? Math.min(0.05, Math.max(0.008, 3 / f0)) : 0.03;
  const start = low && f0 > 0 ? low.at + Math.ceil((now - low.at) * f0) / f0 : now;
  const shape = shapeOf(voice);
  const steps = Math.max(2, Math.floor(w / dpr));
  const ys = new Float32Array(steps + 1);
  let peak = 0;
  for (let i = 0; i <= steps; i++) {
    const t = start + (i / steps) * span;
    let s = 0;
    for (const e of live) {
      const since = t - e.at;
      // Velocities run 0 to 127, as in MIDI.
      const level = (e.event.vel / 127) * (e.event.acc ? 1.2 : 1);
      if (e.event.note !== undefined) {
        s += level * envelope(since, e.event.dur) * shape(freq(e.event.note) * since);
      } else if (e.event.drum !== undefined) {
        s += 0.7 * level * drumAt(e.event.drum, since, t);
      }
    }
    // Several notes add up; a soft limit keeps them inside, as a mix would.
    s = Math.tanh((1.3 * s) / Math.max(1, Math.sqrt(live.length)));
    ys[i] = s;
    peak = Math.max(peak, Math.abs(s));
  }
  // The height follows the part's level, as a scope's gain is set to it:
  // up at once, down slowly, and a quiet part stays small.
  const held = Math.max(peak, (levels.get(g.ctx) ?? 0) * 0.94);
  levels.set(g.ctx, held);
  const scale = 0.85 / Math.max(held, 0.12);
  ctx.lineWidth = 1.6 * dpr;
  ctx.lineJoin = "round";
  ctx.beginPath();
  for (let i = 0; i <= steps; i++) {
    const y = h / 2 - ys[i] * scale * (h / 2);
    const x = (i / steps) * w;
    if (i === 0) ctx.moveTo(x, y);
    else ctx.lineTo(x, y);
  }
  ctx.stroke();
}

/** Each wave's height, held a little while. */
const levels = new WeakMap<CanvasRenderingContext2D, number>();

// The roll.

function roll(g: Sized, bar: { info: BarInfo; at: number }, events: Heard[], now: number, color: string): void {
  const { ctx, w, h, dpr } = g;
  const len = bar.info.end - bar.info.start;
  if (len <= 0) return;
  const inBar = events.filter((e) => e.event.bar === bar.info.bar);
  const x = (t: number) => ((t - bar.at) / len) * w;
  // The beats, faint.
  ctx.globalAlpha = 0.18;
  for (const bt of bar.info.beatTimes) {
    const bx = Math.round(x(bar.at + bt - bar.info.start));
    ctx.fillRect(bx, 0, dpr, h);
  }
  ctx.globalAlpha = 1;
  const keyOf = (e: Heard) => (e.event.note !== undefined ? `n${e.event.note}` : `d${e.event.drum ?? ""}`);
  const keys = [...new Set(inBar.map(keyOf))].sort((a, b) => {
    // Notes high to low, then the drums in the order they come.
    if (a[0] === "n" && b[0] === "n") return Number(b.slice(1)) - Number(a.slice(1));
    return a[0] === b[0] ? 0 : a[0] === "n" ? -1 : 1;
  });
  const rows = Math.max(1, keys.length);
  const rowH = h / rows;
  const gap = Math.min(2 * dpr, rowH * 0.25);
  for (const e of inBar) {
    const r = keys.indexOf(keyOf(e));
    const x0 = x(e.at);
    const x1 = Math.max(x0 + 3 * dpr, x(e.at + Math.max(e.event.dur, 0.05)));
    const on = e.at <= now && now < e.at + Math.max(e.event.dur, 0.08);
    ctx.globalAlpha = on ? 1 : e.at > now ? 0.55 : 0.3;
    ctx.fillStyle = color;
    ctx.fillRect(x0, r * rowH + gap / 2, x1 - x0 - dpr, Math.max(dpr, rowH - gap));
  }
  ctx.globalAlpha = 1;
  // The playhead.
  ctx.fillRect(Math.round(x(now)), 0, 2 * dpr, h);
}
