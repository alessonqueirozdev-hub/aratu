// The rhythm map: every line as rows of bars, each bar striped with its
// beats and their subdivisions, the written symbols on them. Drag a symbol
// to move it, click an empty place to add one, click a symbol to take it
// away: the bar is rewritten in the text, and the music follows at the
// next bar line. While the piece plays, a needle runs across it.

import type { EditorView } from "@codemirror/view";
import type { Aratu, BarInfo, GridBar, GridLine, Program } from "./aratu";
import type { Engine } from "./audio";

/** One step of a bar while editing it. */
interface Cell {
  kind: "rest" | "hold" | "start";
  /** The written text of a sound that starts here. */
  text?: string;
  len?: number;
}

interface BarView {
  line: GridLine;
  bar: GridBar;
  /** Steps per beat drawn (the bar's own, refined by the chosen grid). */
  res: number;
  cells: Cell[];
  el: HTMLElement;
}

function gcd(a: number, b: number): number {
  return b === 0 ? a : gcd(b, a % b);
}

function lcm(a: number, b: number): number {
  return (a / gcd(a, b)) * b;
}

function el<K extends keyof HTMLElementTagNameMap>(tag: K, cls?: string, text?: string): HTMLElementTagNameMap[K] {
  const e = document.createElement(tag);
  if (cls) e.className = cls;
  if (text !== undefined) e.textContent = text;
  return e;
}

/** The steps of a bar at `res` steps per beat, with the written texts. */
function cellsOf(bar: GridBar, res: number, text: string): Cell[] {
  const f = res / bar.res;
  const cells: Cell[] = [];
  for (const s of bar.steps) {
    if (s === 0) {
      for (let k = 0; k < f; k++) cells.push({ kind: "rest" });
    } else if (s === 1) {
      for (let k = 0; k < f; k++) cells.push({ kind: "hold" });
    } else {
      const [from, to, len, notes] = s;
      cells.push({ kind: "start", text: notes ?? text.slice(from, to), len: len * f });
      for (let k = 1; k < f; k++) cells.push({ kind: "hold" });
    }
  }
  return cells;
}

/** Writes a bar's steps back as Aratu: each beat as few parts as it needs,
 * two notes in one beat in brackets. */
export function formatBar(cells: Cell[], beats: number, res: number, kind: string): string {
  const out: string[] = [];
  // A drum or a strum does not sustain: a rest after it sounds the same as
  // a hold, so the shortest writing wins (\`b\`, not \`b-\`).
  const struck = kind === "drums" || kind === "strum";
  for (let b = 0; b < beats; b++) {
    const slice = cells.slice(b * res, (b + 1) * res);
    let n = res;
    for (let d = 1; d <= res; d++) {
      if (res % d !== 0) continue;
      const g = res / d;
      let ok = true;
      for (let k = 0; k < d && ok; k++) {
        const first = slice[k * g];
        for (let j = 1; j < g && ok; j++) {
          const c = slice[k * g + j];
          ok =
            first.kind === "rest"
              ? c.kind === "rest"
              : c.kind === "hold" || (struck && first.kind === "start" && c.kind === "rest");
        }
      }
      if (ok) {
        n = d;
        break;
      }
    }
    const g = res / n;
    const tokens: string[] = [];
    let starts = 0;
    for (let k = 0; k < n; k++) {
      const c = slice[k * g];
      if (c.kind === "start") {
        starts++;
        tokens.push(c.text ?? "c");
      } else tokens.push(c.kind === "hold" ? "_" : "-");
    }
    out.push(kind === "pitched" && starts > 1 ? `[${tokens.join(" ")}]` : tokens.join(""));
  }
  return out.join(" ");
}

/** Places a sound of `len` steps at `at`, cutting what was there. */
function place(cells: Cell[], at: number, text: string, len: number): void {
  const end = Math.min(cells.length, at + Math.max(1, len));
  // A sound starting before `at` and running into it now ends there.
  for (let k = at; k < end; k++) cells[k] = { kind: k === at ? "start" : "hold" };
  cells[at] = { kind: "start", text, len: end - at };
  // What followed, if it was the held end of a cut sound, becomes rest.
  for (let k = end; k < cells.length && cells[k].kind === "hold"; k++) cells[k] = { kind: "rest" };
}

/** Takes the sound starting at `at` away. */
function remove(cells: Cell[], at: number): { text: string; len: number } | null {
  const c = cells[at];
  if (c?.kind !== "start") return null;
  const len = c.len ?? 1;
  for (let k = at; k < Math.min(cells.length, at + len); k++) cells[k] = { kind: "rest" };
  return { text: c.text ?? "c", len };
}

export interface RhythmMapOptions {
  aratu: Aratu;
  engine: () => Engine;
  /** The editor whose text the map rewrites; none for a map to look at. */
  view?: () => EditorView | null;
  /** Called after the map changed the text, to compile at once. */
  edited?: () => void;
  /** Steps per beat the grid is refined to (1 is the rhythm's own). */
  quantum?: () => number;
  /** The most bars drawn per line. */
  maxBars?: number;
}

export class RhythmMap {
  private bars: BarView[] = [];
  private times: { bar: number; at: number; dur: number }[] = [];
  private frame = 0;
  private length = 1;

  constructor(
    readonly box: HTMLElement,
    private readonly o: RhythmMapOptions,
  ) {}

  draw(program: Program, text: string): void {
    this.box.innerHTML = "";
    this.bars = [];
    if (!program.ok) return;
    this.length = program.length;
    const g = this.o.aratu.grid(program.handle, this.o.maxBars ?? 4);
    const lines = g.lines.filter((l) => l.bars.length > 0);
    if (lines.length === 0) return;
    const q = Math.max(1, this.o.quantum?.() ?? 1);
    // Columns in time: the pickup (if any line has one), then one time round
    // of the longest line; shorter lines repeat, as they loop.
    const pickup = lines.some((l) => l.bars.some((b) => b.pos === 0));
    const round = Math.max(...lines.map((l) => l.bars.filter((b) => b.pos > 0).length), 1);
    const columns: number[] = [];
    if (pickup) columns.push(0);
    for (let k = 1; k <= round; k++) columns.push(k);
    const table = el("div", "grid");
    table.style.setProperty("--bars", String(columns.length));
    const editable = !!this.o.view;

    for (const line of lines) {
      const row = el("div", "grid-row");
      row.append(el("span", "grid-label", line.label));
      const loop = line.bars.filter((b) => b.pos > 0);
      for (const col of columns) {
        const bar = col === 0 ? line.bars.find((b) => b.pos === 0) : loop[(col - 1) % Math.max(loop.length, 1)];
        if (!bar) {
          row.append(el("div", "grid-empty"));
          continue;
        }
        const repeat = col > loop.length;
        const res = lcm(bar.res, q);
        const cells = cellsOf(bar, res, text);
        const total = cells.length;
        const cell = el("div", "grid-bar");
        cell.dataset.bar = String(col);
        if (repeat) cell.classList.add("repeat");
        cell.style.setProperty("--beats", String(bar.beats));
        cell.style.setProperty("--steps", String(total));
        if (!(editable && bar.editable)) cell.classList.add("locked");
        const view: BarView = { line, bar, res, cells, el: cell };
        this.bars.push(view);
        // The beat numbers on top of the first line's bars.
        if (line === lines[0]) {
          for (let b = 0; b < bar.beats; b++) {
            const n = el("span", "grid-num", String(b + 1));
            n.style.left = `${(b / bar.beats) * 100}%`;
            cell.append(n);
          }
        }
        // A sound held over from the bar before.
        if (cells[0]?.kind === "hold") {
          let k = 0;
          while (k < total && cells[k].kind === "hold") k++;
          const tie = el("i", "grid-hit grid-tie");
          tie.style.left = "0%";
          tie.style.width = `${(k / total) * 100}%`;
          cell.append(tie);
        }
        cells.forEach((c, i) => {
          if (c.kind !== "start") return;
          const hit = el("i", "grid-hit", c.text);
          hit.dataset.step = String(i);
          hit.dataset.at = String(i / total);
          hit.style.left = `${(i / total) * 100}%`;
          if (line.kind === "pitched" || line.kind === "chords") {
            hit.style.width = `calc(${((c.len ?? 1) / total) * 100}% - 3px)`;
            hit.classList.add("long");
          }
          cell.append(hit);
        });
        if (editable && bar.editable) this.attach(view);
        row.append(cell);
      }
      table.append(row);
    }
    this.box.append(table);
  }

  /** The step under a pointer in a bar. */
  private stepAt(v: BarView, clientX: number): number {
    const r = v.el.getBoundingClientRect();
    const x = (clientX - r.left) / Math.max(r.width, 1);
    return Math.min(v.cells.length - 1, Math.max(0, Math.floor(x * v.cells.length)));
  }

  /** The bar of the same line under a pointer, if any. */
  private barAt(line: GridLine, clientX: number, clientY: number): BarView | null {
    for (const v of this.bars) {
      if (v.line !== line || !v.bar.editable) continue;
      const r = v.el.getBoundingClientRect();
      if (clientX >= r.left && clientX <= r.right && clientY >= r.top - 20 && clientY <= r.bottom + 20) return v;
    }
    return null;
  }

  private attach(v: BarView): void {
    v.el.addEventListener("pointerdown", (down) => {
      if (down.button !== 0) return;
      down.preventDefault();
      const target = down.target as HTMLElement;
      const from = target.classList.contains("grid-hit") && target.dataset.step !== undefined
        ? Number(target.dataset.step)
        : null;
      if (from === null) {
        // An empty place: add a sound there.
        this.add(v, this.stepAt(v, down.clientX));
        return;
      }
      const startX = down.clientX;
      let over: BarView = v;
      let step = from;
      try {
        target.setPointerCapture(down.pointerId);
      } catch {
        // Without capture the drag still works while the pointer stays on it.
      }
      target.classList.add("dragging");
      const move = (e: PointerEvent) => {
        const b = this.barAt(v.line, e.clientX, e.clientY);
        if (!b) return;
        over = b;
        step = this.stepAt(b, e.clientX);
        // Show where it will land.
        const r = b.el.getBoundingClientRect();
        const vr = v.el.getBoundingClientRect();
        const x = r.left - vr.left + (step / b.cells.length) * r.width;
        target.style.left = `${(x / vr.width) * 100}%`;
      };
      const up = (e: PointerEvent) => {
        target.removeEventListener("pointermove", move);
        target.removeEventListener("pointerup", up);
        target.classList.remove("dragging");
        const still = Math.abs(e.clientX - startX) < 4;
        if (still) this.take(v, from);
        else if (over !== v || step !== from) this.move(v, from, over, step);
        else target.style.left = `${(from / v.cells.length) * 100}%`;
      };
      target.addEventListener("pointermove", move);
      target.addEventListener("pointerup", up);
    });
  }

  /** The symbol a new sound gets: the line's most used one. */
  private symbolFor(v: BarView, at: number): string {
    if (v.line.kind === "pitched") {
      for (let k = at; k >= 0; k--) if (v.cells[k].kind === "start") return v.cells[k].text ?? "c4";
      const next = v.cells.find((c) => c.kind === "start");
      return next?.text ?? "c4";
    }
    const count = new Map<string, number>();
    for (const b of this.bars) {
      if (b.line !== v.line) continue;
      for (const c of b.cells) {
        if (c.kind !== "start" || !c.text) continue;
        const letter = c.text.replace(/^[a-z]+:/, "").charAt(0).toLowerCase();
        count.set(letter, (count.get(letter) ?? 0) + 1);
      }
    }
    let best = v.line.kind === "strum" ? "d" : "c";
    let most = 0;
    for (const [k, n] of count) if (n > most) [best, most] = [k, n];
    return best;
  }

  private add(v: BarView, at: number): void {
    if (v.cells[at].kind === "start") return;
    place(v.cells, at, this.symbolFor(v, at), 1);
    this.write([v]);
  }

  private take(v: BarView, at: number): void {
    if (!remove(v.cells, at)) return;
    this.write([v]);
  }

  private move(a: BarView, from: number, b: BarView, to: number): void {
    // Two copies of one written bar (a repeat, a loop) are the same bar.
    const target = b.bar === a.bar ? a : b;
    const sound = remove(a.cells, from);
    if (!sound) return;
    place(target.cells, to, sound.text, sound.len);
    this.write(target === a ? [a] : [a, target]);
  }

  /** Rewrites the given bars in the text. */
  private write(changed: BarView[]): void {
    const view = this.o.view?.();
    if (!view) return;
    const changes = changed
      .filter((v) => v.bar.from >= 0)
      .map((v) => ({
        from: v.bar.from,
        to: v.bar.to,
        insert: formatBar(v.cells, v.bar.beats, v.res, v.line.kind),
      }))
      .sort((x, y) => x.from - y.from);
    if (changes.length === 0) return;
    view.dispatch({ changes, userEvent: "input.rhythm" });
    this.o.edited?.();
  }

  // The needle.

  bar(info: BarInfo, at: number): void {
    this.times.push({ bar: info.bar, at, dur: Math.max(info.end - info.start, 0.001) });
    if (this.times.length > 12) this.times.shift();
  }

  start(): void {
    cancelAnimationFrame(this.frame);
    const tick = () => {
      this.needle();
      this.frame = requestAnimationFrame(tick);
    };
    this.frame = requestAnimationFrame(tick);
  }

  stop(): void {
    cancelAnimationFrame(this.frame);
    this.times = [];
    this.clearNeedle();
  }

  private clearNeedle(): void {
    for (const e of this.box.querySelectorAll(".grid-bar.playing")) e.classList.remove("playing");
    for (const e of this.box.querySelectorAll(".grid-hit.on")) e.classList.remove("on");
  }

  private needle(): void {
    const now = this.o.engine().ctx.currentTime;
    const b = this.times.find((x) => now >= x.at && now < x.at + x.dur);
    this.clearNeedle();
    if (!b) return;
    const k = b.bar === 0 ? 0 : ((b.bar - 1) % Math.max(this.length, 1)) + 1;
    const pos = (now - b.at) / b.dur;
    for (const cell of this.box.querySelectorAll<HTMLElement>(`.grid-bar[data-bar="${k}"]`)) {
      cell.classList.add("playing");
      cell.style.setProperty("--pos", String(pos));
      for (const hit of cell.querySelectorAll<HTMLElement>(".grid-hit[data-at]")) {
        const at = Number(hit.dataset.at);
        if (at <= pos && pos - at < 0.07) hit.classList.add("on");
      }
    }
  }
}
