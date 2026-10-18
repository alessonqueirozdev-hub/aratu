// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
// The Sounds panel: every voice by family, with the code that selects it.
// A click on a code writes it into the piece; ▶ plays a bar of it.

import type { Aratu, VoiceCatalogue, VoiceInfo } from "./aratu";
import { Engine, playsOnManguebyte } from "./audio";
import { type Lang, tr } from "./i18n";

export interface SoundsOptions {
  aratu: Aratu;
  engine: () => Engine;
  lang: () => Lang;
  /** Writes a voice's code into the piece, where the cursor is. */
  insert: (code: string) => void;
}

const el = <K extends keyof HTMLElementTagNameMap>(tag: K, cls?: string, text?: string) => {
  const e = document.createElement(tag);
  if (cls) e.className = cls;
  if (text !== undefined) e.textContent = text;
  return e;
};

export class SoundsPanel {
  private catalogue: VoiceCatalogue | null = null;
  private filter = "";
  private stopPreview: (() => void) | null = null;
  /** Counts previews asked for, so one still waiting knows another came. */
  private asked = 0;

  constructor(
    private readonly box: HTMLElement,
    private readonly o: SoundsOptions,
  ) {}

  /** Draws the list again, in the current language. */
  render(): void {
    const lang = this.o.lang();
    this.catalogue = this.o.aratu.voices(lang);
    this.box.innerHTML = "";
    const search = el("input", "sound-search");
    search.type = "search";
    search.placeholder = tr("soundSearch", lang);
    search.value = this.filter;
    search.oninput = () => {
      this.filter = search.value;
      this.list(list);
    };
    const help = el("p", "sound-help", tr("soundsHelp", lang));
    // Which engine plays samples and drum kits: Manguebyte, or (switched
    // off, a way out on stage) the stand-in for everything.
    const engineRow = el("label", "sound-engine");
    const box = el("input");
    box.type = "checkbox";
    box.checked = Engine.manguebyteWanted();
    box.onchange = () => {
      try {
        localStorage.setItem("aratu.manguebyte", box.checked ? "on" : "off");
      } catch {
        // Without storage the switch cannot be kept.
      }
      location.reload();
    };
    engineRow.append(box, el("span", "", tr("mbSwitch", lang)));
    const engineHelp = el("p", "sound-help", tr("mbHelp", lang));
    const list = el("div", "sound-list");
    this.box.append(search, help, engineRow, engineHelp, list);
    this.list(list);
  }

  /** Marks a sampled voice as loading, ready or failed. */
  mark(code: string, state: string): void {
    for (const b of this.box.querySelectorAll<HTMLElement>(`[data-voice="${CSS.escape(code)}"] .sound-state`)) {
      b.dataset.state = state;
      b.textContent = this.stateWord(state);
    }
  }

  private stateWord(state: string): string {
    const lang = this.o.lang();
    if (state === "loading") return tr("soundLoading", lang);
    if (state === "ready") return tr("soundReady", lang);
    if (state === "failed") return tr("soundFailed", lang);
    return tr("soundSampled", lang);
  }

  private list(box: HTMLElement): void {
    if (!this.catalogue) return;
    const lang = this.o.lang();
    const q = this.filter.trim().toLowerCase();
    box.innerHTML = "";
    let shown = 0;
    for (const family of this.catalogue.families) {
      const voices = family.voices.filter(
        (v) => !q || v.code.includes(q) || v.name.toLowerCase().includes(q) || family.name.toLowerCase().includes(q),
      );
      if (voices.length === 0) continue;
      box.append(el("h3", "sound-family", family.name));
      for (const v of voices) {
        box.append(this.row(v, family.code === "drums"));
        shown++;
      }
    }
    if (shown === 0) box.append(el("p", "sound-none", tr("soundNone", lang)));
  }

  private row(v: VoiceInfo, kit: boolean): HTMLElement {
    const lang = this.o.lang();
    const row = el("div", "sound-row");
    row.dataset.voice = v.code;
    const play = el("button", "sound-play", "▶");
    play.title = tr("soundHear", lang);
    play.onclick = () => void this.preview(v, kit, play);
    const code = el("button", "sound-code", v.code);
    code.title = tr("soundUse", lang);
    code.onclick = () => this.o.insert(v.code);
    const name = el("span", "sound-name", v.name);
    row.append(play, code, name);
    if (playsOnManguebyte(v.code, v.sampled) && Engine.manguebyteWanted()) {
      row.append(el("span", "sound-mb", "Manguebyte"));
    }
    if (v.sampled) {
      const state = el("span", "sound-state");
      const s = this.o.engine().sampleState(v.code);
      state.dataset.state = s;
      state.textContent = this.stateWord(s);
      row.append(state);
    }
    return row;
  }

  /** Plays one bar of a voice, twice, on its own. */
  private async preview(v: VoiceInfo, kit: boolean, button: HTMLElement): Promise<void> {
    this.stopPreview?.();
    const part = kit ? "drums" : "som";
    const body = kit ? v.demo.split("\n").map((l) => `  ${l}`).join("\n") : `  melodia = ${v.demo}`;
    const src = `tempo: 100\n${part}: ${v.code}\n${part} {\n${body}\n}`;
    const program = this.o.aratu.compile(src, this.o.lang());
    if (!program.ok) {
      this.o.aratu.release(program.handle);
      return;
    }
    const engine = this.o.engine();
    void engine.ctx.resume();
    engine.configure(program);
    const rendered = this.o.aratu.render(program.handle, 1, 3, new Map(), 1);
    this.o.aratu.release(program.handle);
    // Heard on the voice itself, once it has been downloaded.
    button.classList.add("on");
    const asked = ++this.asked;
    await engine.ready([part]);
    if (asked !== this.asked) return;
    const start = engine.ctx.currentTime + 0.25;
    const voices = rendered.events.flatMap((e) => engine.play(e, start + e.time));
    const end = Math.max(0, ...rendered.events.map((e) => e.time + e.dur)) + 0.5;
    const timer = window.setTimeout(() => button.classList.remove("on"), end * 1000);
    this.stopPreview = () => {
      window.clearTimeout(timer);
      button.classList.remove("on");
      engine.cancel(voices, engine.ctx.currentTime);
      this.stopPreview = null;
    };
  }
}
