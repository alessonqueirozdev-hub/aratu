// The live blocks of the book: examples to hear and change, and exercises
// to solve. Each has a small editor, buttons, and a rhythm grid that draws
// what is written on the beats of each bar, the way a teacher would on the
// board.

import type { Aratu, BarInfo, Program } from "./aratu";
import type { Engine } from "./audio";
import { type Lang, tr } from "./i18n";
import { Session } from "./session";

export interface BlockContext {
  aratu: Aratu;
  engine: () => Engine;
  lang: () => Lang;
  isDone(id: string): boolean;
  markDone(id: string): void;
  openInPlay(text: string): void;
}

function el<K extends keyof HTMLElementTagNameMap>(tag: K, cls?: string, text?: string): HTMLElementTagNameMap[K] {
  const e = document.createElement(tag);
  if (cls) e.className = cls;
  if (text !== undefined) e.textContent = text;
  return e;
}

/** Draws the first time round of a piece as rows of beats with its symbols. */
export function drawGrid(box: HTMLElement, aratu: Aratu, program: Program, text: string): void {
  box.innerHTML = "";
  if (!program.ok) return;
  const from = program.pickup > 0 ? 0 : 1;
  const bars = Math.min(program.length, 4);
  const r = aratu.render(program.handle, from, 1 + bars, new Map(), 1);
  if (r.bars.length === 0 || r.events.length === 0) return;
  const lines = [...new Set(r.events.filter((e) => e.param === undefined).map((e) => e.line))];
  const table = el("div", "grid");
  table.style.setProperty("--bars", String(r.bars.length));
  table.dataset.length = String(program.length);
  table.dataset.shown = String(bars);

  const head = el("div", "grid-row grid-head");
  head.append(el("span", "grid-label"));
  for (const b of r.bars) {
    const cell = el("div", "grid-bar");
    cell.style.setProperty("--beats", String(b.beats));
    for (let i = 0; i < b.beats; i++) cell.append(el("span", "grid-beat", String(i + 1)));
    head.append(cell);
  }
  table.append(head);

  for (const line of lines) {
    const row = el("div", "grid-row");
    row.append(el("span", "grid-label", line));
    for (const b of r.bars) {
      const cell = el("div", "grid-bar");
      cell.dataset.bar = String(b.bar);
      cell.style.setProperty("--beats", String(b.beats));
      for (let i = 0; i < b.beats; i++) cell.append(el("span", "grid-tick"));
      const seen = new Set<string>();
      for (const e of r.events) {
        if (e.line !== line || e.bar !== b.bar) continue;
        const key = `${e.beat}:${e.from}`;
        if (seen.has(key)) continue;
        seen.add(key);
        const hit = el("i", "grid-hit", text.slice(e.from, e.to));
        const at = e.beat / Math.max(b.beats, 1);
        hit.dataset.at = String(at);
        hit.style.left = `${at * 100}%`;
        cell.append(hit);
      }
      row.append(cell);
    }
    table.append(row);
  }
  box.append(table);
}

/** The needle that runs across a rhythm grid while its piece plays, and
 * the symbols lighting up as it passes them. */
export class Needle {
  private bars: { bar: number; at: number; dur: number }[] = [];
  private frame = 0;

  constructor(
    private readonly box: HTMLElement,
    private readonly engine: () => Engine,
  ) {}

  bar(info: BarInfo, at: number): void {
    this.bars.push({ bar: info.bar, at, dur: Math.max(info.end - info.start, 0.001) });
    if (this.bars.length > 12) this.bars.shift();
  }

  start(): void {
    cancelAnimationFrame(this.frame);
    const tick = () => {
      this.draw();
      this.frame = requestAnimationFrame(tick);
    };
    this.frame = requestAnimationFrame(tick);
  }

  stop(): void {
    cancelAnimationFrame(this.frame);
    this.bars = [];
    this.clear();
  }

  private clear(): void {
    for (const e of this.box.querySelectorAll(".grid-bar.playing")) e.classList.remove("playing");
    for (const e of this.box.querySelectorAll(".grid-hit.on")) e.classList.remove("on");
  }

  private draw(): void {
    const grid = this.box.querySelector<HTMLElement>(".grid");
    if (!grid) return;
    const now = this.engine().ctx.currentTime;
    const b = this.bars.find((x) => now >= x.at && now < x.at + x.dur);
    this.clear();
    if (!b) return;
    const length = Number(grid.dataset.length ?? 1);
    const shown = Number(grid.dataset.shown ?? 1);
    // The grid draws one time round: find this bar's place in it.
    const k = b.bar === 0 ? 0 : ((b.bar - 1) % Math.max(length, 1)) + 1;
    if (k > shown) return;
    const pos = (now - b.at) / b.dur;
    for (const cell of grid.querySelectorAll<HTMLElement>(`.grid-bar[data-bar="${k}"]`)) {
      cell.classList.add("playing");
      cell.style.setProperty("--pos", String(pos));
      for (const hit of cell.querySelectorAll<HTMLElement>(".grid-hit")) {
        const at = Number(hit.dataset.at ?? -1);
        if (at <= pos && pos - at < 0.07) hit.classList.add("on");
      }
    }
  }
}

/** Turns every `.aratu-block` under `root` into a live block. */
export function mountBlocks(root: HTMLElement, ctx: BlockContext): Session[] {
  const sessions: Session[] = [];
  for (const block of root.querySelectorAll<HTMLElement>(".aratu-block")) {
    const kind = block.dataset.kind ?? "example";
    const id = block.dataset.id ?? "";
    const code = block.querySelector<HTMLTextAreaElement>(".aratu-code")?.value ?? "";
    const answer = block.querySelector<HTMLTextAreaElement>(".aratu-answer")?.value;
    const lang = ctx.lang();
    block.innerHTML = "";

    const top = el("div", "block-top");
    const label = el("span", "block-label", tr(kind === "exercise" ? "exercise" : "example", lang));
    if (kind === "exercise" && ctx.isDone(id)) label.classList.add("done");
    const buttons = el("div", "block-buttons");
    top.append(label, buttons);
    const editor = el("div", "block-editor");
    const grid = el("div", "block-grid");
    const result = el("div", "block-result");
    block.append(top, editor, grid, result);

    const listen = el("button", "btn btn-play", `▶ ${tr("listen", lang)}`);
    const needle = new Needle(grid, ctx.engine);
    const session = new Session({
      parent: editor,
      text: code,
      aratu: ctx.aratu,
      engine: ctx.engine,
      lang: ctx.lang,
      compact: true,
      onCompile: (p) => {
        drawGrid(grid, ctx.aratu, p, session.text);
        const errors = p.diagnostics.filter((d) => d.severity === "error");
        result.className = "block-result";
        result.innerHTML = "";
        if (errors.length > 0) {
          result.classList.add("bad");
          for (const d of errors.slice(0, 2)) result.append(el("p", "", d.message));
        }
      },
      onBar: (info, at) => needle.bar(info, at),
      onPlaying: (on) => {
        if (on) needle.start();
        else needle.stop();
        listen.textContent = on ? `■ ${tr("stop", ctx.lang())}` : `▶ ${tr("listen", ctx.lang())}`;
        listen.classList.toggle("on", on);
      },
    });
    sessions.push(session);
    listen.onclick = () => (session.playing ? session.stop() : void session.play());
    buttons.append(listen);

    const restart = el("button", "btn btn-quiet", `↺ ${tr("restart", lang)}`);
    restart.onclick = () => {
      session.stop();
      session.text = code;
    };

    if (kind !== "exercise" || answer === undefined) {
      const open = el("button", "btn btn-quiet", tr("openInPlay", lang));
      open.onclick = () => {
        session.stop();
        ctx.openInPlay(session.text);
      };
      buttons.append(restart, open);
      session.compile();
      continue;
    }

    // An exercise: hear the goal, try, check.
    const goal = el("div", "block-goal");
    goal.hidden = true;
    block.append(goal);
    const target = new Session({
      parent: goal,
      text: answer,
      aratu: ctx.aratu,
      engine: ctx.engine,
      lang: ctx.lang,
      compact: true,
    });
    sessions.push(target);
    const hear = el("button", "btn btn-goal", `♪ ${tr("listenAnswer", lang)}`);
    hear.onclick = () => (target.playing ? target.stop() : void target.play());
    const check = el("button", "btn btn-check", `✓ ${tr("check", lang)}`);
    const reveal = el("button", "btn btn-quiet", tr("showAnswer", lang));
    reveal.hidden = !ctx.isDone(id);
    reveal.onclick = () => {
      goal.hidden = !goal.hidden;
    };
    buttons.append(hear, check, restart, reveal);
    session.compile();

    check.onclick = () => {
      const l = ctx.lang();
      const bars = Number(block.dataset.bars ?? 0);
      const checked = ctx.aratu.check(
        session.text,
        answer,
        l,
        bars,
        block.dataset.dynamics === "1",
        block.dataset.lengths !== "0",
      );
      result.innerHTML = "";
      result.className = "block-result";
      if (!checked.compiles) {
        result.classList.add("bad");
        result.append(el("p", "", tr("doesNotRead", l)));
        return;
      }
      if (checked.ok) {
        result.classList.add("good");
        result.append(el("p", "", `🦀 ${tr("right", l)}`));
        label.classList.add("done");
        reveal.hidden = false;
        ctx.markDone(id);
        return;
      }
      result.classList.add("almost");
      result.append(el("p", "", tr("almost", l)));
      const ul = el("ul");
      for (const p of checked.problems.slice(0, 4)) ul.append(el("li", "", p));
      if (checked.problems.length > 4) {
        ul.append(el("li", "more", `${tr("andMore", l)} ${checked.problems.length - 4} ${tr("moreProblems", l)}`));
      }
      result.append(ul);
      reveal.hidden = false;
    };
  }
  return sessions;
}
