// Numbers drawn as knobs. Each number that sets a parameter gets a knob
// beside it, shaped by the parameter (a cutoff turns on a logarithmic scale
// in hertz, a mix from 0 to 1). Dragging the knob up or down rewrites the
// number in the text, so the text stays the one source of truth, and the
// sound follows at once. A parameter that moves shows where it is now with a
// second needle that turns as the sound does.

import { type Range, StateEffect, StateField } from "@codemirror/state";
import { Decoration, type DecorationSet, EditorView, WidgetType } from "@codemirror/view";
import type { Control } from "./aratu";

export type KnobMoved = (control: Control, value: number) => void;

function toKnob(c: Control, v: number): number {
  const x = Math.min(Math.max(v, c.min), c.max);
  const t = c.log ? Math.log(x / c.min) / Math.log(c.max / c.min) : (x - c.min) / (c.max - c.min);
  return Math.min(Math.max(t, 0), 1);
}

function fromKnob(c: Control, t: number): number {
  t = Math.min(Math.max(t, 0), 1);
  const v = c.log ? c.min * Math.pow(c.max / c.min, t) : c.min + t * (c.max - c.min);
  return c.steps ? Math.round(v) : v;
}

/** The number as it is written back into the text. */
function written(c: Control, value: number): string {
  const w = value * c.factor;
  const span = (c.max - c.min) * c.factor;
  const decimals = c.steps || span >= 100 ? 0 : span >= 10 ? 1 : 2;
  return w.toFixed(decimals);
}

const NUMBER_CHAR = /[0-9.+-]/;

class KnobWidget extends WidgetType {
  constructor(
    readonly control: Control,
    readonly moved: KnobMoved,
  ) {
    super();
  }

  eq(other: KnobWidget): boolean {
    return other.control.id === this.control.id && other.control.part === this.control.part;
  }

  updateDOM(dom: HTMLElement): boolean {
    (dom as HTMLElement & { knob?: KnobWidget }).knob = this;
    this.draw(dom, toKnob(this.control, this.control.value));
    return true;
  }

  private draw(dom: HTMLElement, t: number): void {
    const angle = -135 + 270 * t;
    const needle = dom.querySelector("line");
    needle?.setAttribute("transform", `rotate(${angle} 10 10)`);
    const arc = dom.querySelector<SVGPathElement>("path.value");
    if (arc) {
      const a = ((angle - 90) * Math.PI) / 180;
      const start = ((-135 - 90) * Math.PI) / 180;
      const large = angle - -135 > 180 ? 1 : 0;
      const r = 8;
      arc.setAttribute(
        "d",
        `M ${10 + r * Math.cos(start)} ${10 + r * Math.sin(start)} A ${r} ${r} 0 ${large} 1 ${10 + r * Math.cos(a)} ${10 + r * Math.sin(a)}`,
      );
    }
    const c = this.control;
    dom.title = `${c.part ? c.part + "." : ""}${c.id} = ${written(c, fromKnob(c, t))}${c.unit && c.factor === 1 ? " " + c.unit : ""}`;
  }

  /** The second needle: where a moving parameter is now. */
  drawNow(dom: HTMLElement, t: number): void {
    const angle = -135 + 270 * t;
    dom.querySelector("line.now")?.setAttribute("transform", `rotate(${angle} 10 10)`);
  }

  toDOM(view: EditorView): HTMLElement {
    const dom = document.createElement("span");
    dom.className = "cm-knob";
    dom.setAttribute("aria-label", this.control.id);
    dom.innerHTML =
      '<svg width="20" height="20" viewBox="0 0 20 20"><circle cx="10" cy="10" r="8" class="track"/>' +
      '<path class="value" d=""/><line x1="10" y1="10" x2="10" y2="3"/>' +
      '<line class="now" x1="10" y1="10" x2="10" y2="2"/></svg>';
    (dom as HTMLElement & { knob?: KnobWidget }).knob = this;
    this.draw(dom, toKnob(this.control, this.control.value));

    dom.addEventListener("pointerdown", (down) => {
      down.preventDefault();
      const me = (dom as HTMLElement & { knob?: KnobWidget }).knob ?? this;
      const c = me.control;
      // The number ends where the knob sits; find where it starts.
      const end = view.posAtDOM(dom);
      let start = end;
      while (start > 0 && NUMBER_CHAR.test(view.state.doc.sliceString(start - 1, start))) start--;
      let range = { from: start, to: end };
      const startT = toKnob(c, c.value);
      const startY = down.clientY;
      dom.setPointerCapture(down.pointerId);
      dom.classList.add("turning");
      const move = (e: PointerEvent) => {
        const travel = e.shiftKey ? 600 : 160;
        const t = Math.min(Math.max(startT + (startY - e.clientY) / travel, 0), 1);
        const value = fromKnob(c, t);
        const text = written(c, value);
        if (view.state.doc.sliceString(range.from, range.to) !== text) {
          view.dispatch({ changes: { ...range, insert: text }, userEvent: "input.knob" });
          range = { from: range.from, to: range.from + text.length };
        }
        me.draw(dom, t);
        me.control.value = value;
        me.moved(c, value);
      };
      const up = () => {
        dom.classList.remove("turning");
        dom.removeEventListener("pointermove", move);
        dom.removeEventListener("pointerup", up);
        dom.removeEventListener("pointercancel", up);
      };
      dom.addEventListener("pointermove", move);
      dom.addEventListener("pointerup", up);
      dom.addEventListener("pointercancel", up);
    });
    return dom;
  }

  ignoreEvent(): boolean {
    return true;
  }
}

const setKnobs = StateEffect.define<{ controls: Control[]; moved: KnobMoved }>();

const knobField = StateField.define<DecorationSet>({
  create: () => Decoration.none,
  update(deco, tr) {
    deco = deco.map(tr.changes);
    for (const e of tr.effects) {
      if (e.is(setKnobs)) {
        const len = tr.state.doc.length;
        const ranges: Range<Decoration>[] = e.value.controls
          .filter((c) => c.to <= len)
          .map((c) =>
            Decoration.widget({ widget: new KnobWidget(c, e.value.moved), side: 1 }).range(c.to),
          );
        deco = Decoration.set(ranges, true);
      }
    }
    return deco;
  },
  provide: (f) => EditorView.decorations.from(f),
});

export const knobs = [knobField];

export function showKnobs(view: EditorView, controls: Control[], moved: KnobMoved): void {
  view.dispatch({ effects: setKnobs.of({ controls, moved }) });
}

/** Turns the second needle of the knobs of each moving parameter to where it
 * is now, by `part<TAB>id`; knobs not in `now` hide it. */
export function showMoving(view: EditorView, now: Map<string, number>): void {
  for (const dom of view.dom.querySelectorAll<HTMLElement>(".cm-knob")) {
    const knob = (dom as HTMLElement & { knob?: KnobWidget }).knob;
    if (!knob) continue;
    const c = knob.control;
    const v = now.get(`${c.part}	${c.id}`);
    dom.classList.toggle("moving", v !== undefined);
    if (v !== undefined) knob.drawNow(dom, toKnob(c, v));
  }
}
