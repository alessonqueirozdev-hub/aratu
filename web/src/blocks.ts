// The live blocks of the book: examples to hear and change, and exercises
// to solve. Each has a small editor, buttons, and a rhythm grid that draws
// what is written on the beats of each bar, the way a teacher would on the
// board.

import type { Aratu } from "./aratu";
import type { Engine } from "./audio";
import { type Lang, tr } from "./i18n";
import { RhythmMap } from "./rhythm";
import { Session } from "./session";

export interface BlockContext {
  aratu: Aratu;
  engine: () => Engine;
  lang: () => Lang;
  isDone(id: string): boolean;
  markDone(id: string): void;
  openInPlay(text: string): void;
  /** Steps per beat the rhythm maps are refined to. */
  quantum(): number;
}

function el<K extends keyof HTMLElementTagNameMap>(tag: K, cls?: string, text?: string): HTMLElementTagNameMap[K] {
  const e = document.createElement(tag);
  if (cls) e.className = cls;
  if (text !== undefined) e.textContent = text;
  return e;
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
    let session: Session;
    const map = new RhythmMap(grid, {
      aratu: ctx.aratu,
      engine: ctx.engine,
      view: () => session?.view ?? null,
      edited: () => session.compile(),
      quantum: ctx.quantum,
    });
    session = new Session({
      parent: editor,
      text: code,
      aratu: ctx.aratu,
      engine: ctx.engine,
      lang: ctx.lang,
      compact: true,
      onCompile: (p) => {
        map.draw(p, session.text);
        const errors = p.diagnostics.filter((d) => d.severity === "error");
        result.className = "block-result";
        result.innerHTML = "";
        if (errors.length > 0) {
          result.classList.add("bad");
          for (const d of errors.slice(0, 2)) result.append(el("p", "", d.message));
        }
      },
      onBar: (info, at) => map.bar(info, at),
      onPlaying: (on) => {
        if (on) map.start();
        else map.stop();
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
