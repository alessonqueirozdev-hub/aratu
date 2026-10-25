// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
// Playing in a DAW. Manguebyte's Aratu Bridge plugin, on a track of the DAW,
// runs the same engine there; the page sends it what it would play here,
// and the sound comes out of the track. The DAW leads: its play starts the
// piece on its next bar line, its stop stops it, and its tempo is written in
// the piece's `tempo:` line, as a knob would write it. While the DAW is
// stopped the piece can still be played from here, and is heard there.

import type { Transport } from "../public/manguebyte/bridge.js";
import type { Engine } from "./audio";
import { type Lang, tr } from "./i18n";
import type { Session } from "./session";
import { Listeners, type Store } from "./shared";

export interface DawOptions {
  button: HTMLElement;
  /** Where the panel goes: the editor's root, so its styles reach it. */
  root: HTMLElement;
  engine: () => Engine;
  session: () => Session;
  lang: () => Lang;
  /** Keeps the port between visits. */
  store: Store;
}

const PORT = 47870;

/** When a piece that starts at `beat` on the DAW's timeline first meets one
 * of the DAW's bar lines, at least `room` beats ahead. */
export function nextBarLine(t: Pick<Transport, "barStart" | "timeSignature">, beat: number, room: number): number {
  const [n, d] = t.timeSignature;
  const bar = n > 0 && d > 0 ? (n * 4) / d : 4;
  const passed = Math.ceil((beat + room - t.barStart) / bar - 1e-9);
  return t.barStart + Math.max(0, passed) * bar;
}

/** The DAW button's panel; `dispose` takes it away with its listeners and
 * lets the DAW go. */
export function setupDaw(o: DawOptions): { words(): void; dispose(): void } {
  const listeners = new Listeners();
  const panel = document.createElement("div");
  panel.className = "daw-panel";
  panel.hidden = true;
  const title = document.createElement("h3");
  const about = document.createElement("p");
  const row = document.createElement("label");
  row.className = "daw-port";
  const portWord = document.createElement("span");
  const port = document.createElement("input");
  port.type = "number";
  port.min = "1024";
  port.max = "65535";
  port.value = o.store.get("daw.port") ?? String(PORT);
  row.append(portWord, port);
  const connect = document.createElement("button");
  connect.className = "btn daw-connect";
  const state = document.createElement("p");
  state.className = "daw-state";
  state.setAttribute("role", "status");
  panel.append(title, about, row, connect, state);
  o.root.append(panel);

  let connected = false;
  let busy = false;
  let shown: { word: Parameters<typeof tr>[0]; detail?: string } = { word: "dawIdle" };
  let wasPlaying = false;
  let tempoTimer = 0;

  const words = () => {
    const lang = o.lang();
    title.textContent = tr("dawTitle", lang);
    about.textContent = tr("dawAbout", lang);
    portWord.textContent = tr("dawPort", lang);
    connect.textContent = tr(connected ? "dawDisconnect" : "dawConnect", lang);
    connect.disabled = busy;
    state.textContent = tr(shown.word, lang) + (shown.detail ? ` (${shown.detail})` : "");
    o.button.classList.toggle("on", connected);
    o.button.setAttribute("aria-pressed", String(connected));
  };
  const tell = (word: Parameters<typeof tr>[0], detail?: string) => {
    shown = { word, detail };
    words();
  };

  // Under the button, inside the window, placed in the root's own terms.
  const place = () => {
    const r = o.button.getBoundingClientRect();
    const box = o.root.getBoundingClientRect();
    const left = Math.max(8, Math.min(window.innerWidth - panel.offsetWidth - 8, r.right - panel.offsetWidth));
    panel.style.top = `${r.bottom + 8 - box.top}px`;
    panel.style.left = `${left - box.left}px`;
  };
  listeners.add(o.button, "click", () => {
    panel.hidden = !panel.hidden;
    words();
    if (!panel.hidden) place();
  });
  listeners.add(document, "pointerdown", (e) => {
    if (!panel.hidden && !panel.contains(e.target as Node) && !o.button.contains(e.target as Node)) panel.hidden = true;
  });
  listeners.add(window, "resize", () => !panel.hidden && place());

  // The DAW leads the piece.
  const transport = (t: Transport) => {
    const session = o.session();
    if (t.playing && (!wasPlaying || t.jumped)) start(t);
    else if (t.playing && t.tempo > 0 && Math.abs((session.tempo ?? t.tempo) - t.tempo) > 0.05) {
      // The DAW's tempo moved while it plays: written once it settles.
      window.clearTimeout(tempoTimer);
      tempoTimer = window.setTimeout(() => session.setTempo(t.tempo), 300);
    }
    if (!t.playing && wasPlaying) session.stop();
    wasPlaying = t.playing;
  };
  const start = (t: Transport) => {
    const engine = o.engine();
    const bridge = engine.bridge;
    if (!bridge) return;
    const session = o.session();
    if (Math.abs((session.tempo ?? t.tempo) - t.tempo) > 0.05) session.setTempo(t.tempo);
    // Far enough ahead for the piece to be sent before the DAW gets there.
    const now = bridge.currentBeat;
    const beat = nextBarLine(t, now, (0.4 * t.tempo) / 60);
    const at = engine.ctx.currentTime + ((beat - now) * 60) / t.tempo;
    session.stop();
    void session.play(at);
  };

  listeners.add(connect, "click", async () => {
    const engine = o.engine();
    busy = true;
    if (connected) {
      engine.dawHooks = {};
      o.session().stop();
      await engine.useDaw(null).catch(() => {});
      connected = false;
      busy = false;
      tell("dawIdle");
      return;
    }
    const n = Math.round(Number(port.value));
    if (!(n >= 1024 && n <= 65535)) {
      busy = false;
      tell("dawBadPort");
      return;
    }
    o.store.set("daw.port", String(n));
    tell("dawConnecting");
    o.session().stop();
    engine.dawHooks = {
      state: (s) => {
        if (s === "closed" || s === "error") {
          if (connected) {
            engine.dawHooks = {};
            connected = false;
            wasPlaying = false;
            o.session().stop();
            void engine.useDaw(null).catch(() => {});
            tell("dawLost");
          }
        }
      },
      transport,
    };
    try {
      await engine.useDaw(n);
      connected = true;
      tell("dawOpen");
    } catch (error) {
      connected = false;
      tell("dawFailed", error instanceof Error ? error.message : String(error));
    }
    busy = false;
    words();
  });
  words();
  return {
    words,
    dispose() {
      listeners.removeAll();
      window.clearTimeout(tempoTimer);
      panel.remove();
      if (connected) {
        connected = false;
        const engine = o.engine();
        engine.dawHooks = {};
        void engine.useDaw(null).catch(() => {});
      }
    },
  };
}
