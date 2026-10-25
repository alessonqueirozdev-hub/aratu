// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
// A piece to hear, without its code: a button that plays and stops it, the
// bar it is in, its beats lit as they go. For a list of pieces (a feed, a
// gallery), where each one only needs to play. It shares the language and
// the sound engine with every editor and player on the page, and only one
// of them sounds at a time.

import type { Program } from "./aratu";
import { beatPips, runProgress } from "./display";
import { tr } from "./i18n";
import { Player } from "./player";
import { Session, type Sounding } from "./session";
import { type AratuStorage, EngineLease, Emitter, assetsOf, langOf, makeStore, useAratu } from "./shared";
import { icon, playerHtml } from "./template";

export interface PlayerOptions {
  /** "pt", "es" or "en" (anything else is English). */
  lang?: string;
  theme?: "light" | "dark";
  /** The folder with aratu.wasm, manguebyte/ and samples/; by default the
   * one this script was loaded from. */
  assetBase?: string | URL;
  wasmUrl?: string | URL;
  version?: string | null;
  /** Where the choice of Manguebyte is kept (see the editor's Sounds
   * panel), under `namespace`. */
  storage?: AratuStorage | null;
  namespace?: string;
}

export interface PlayerEvents extends Record<string, unknown[]> {
  play: [];
  stop: [];
}

export interface AratuPlayer {
  readonly root: HTMLElement;
  /** Settles once the language has loaded. */
  readonly ready: Promise<void>;
  readonly playing: boolean;
  getCode(): string;
  /** A new piece; while playing, it comes in at the next bar line. */
  setCode(code: string): void;
  play(): Promise<void>;
  stop(): void;
  setLang(lang: string): void;
  setTheme(theme: "light" | "dark"): void;
  on<K extends keyof PlayerEvents>(event: K, f: (...args: PlayerEvents[K]) => void): () => void;
  destroy(): void;
}

export function mountPlayer(el: HTMLElement, code: string, opts: PlayerOptions = {}): AratuPlayer {
  const store = makeStore(opts.storage, opts.namespace);
  const assets = assetsOf(opts);
  const events = new Emitter<PlayerEvents>();
  const language = useAratu(assets.wasm);
  let lang = langOf(opts.lang);
  let alive = true;
  let text = code;

  const root = document.createElement("div");
  root.className = "aratu-root aratu-player";
  root.dataset.theme = opts.theme === "dark" ? "dark" : "light";
  root.innerHTML = playerHtml();
  el.append(root);
  const $ = (name: string) => root.querySelector<HTMLElement>(`[data-ref="${name}"]`) as HTMLElement;
  const button = $("play");
  const note = $("note");
  const progress = $("bar-progress");
  beatPips($("beat"), 4, -1);

  const words = () => {
    root.lang = lang === "pt" ? "pt-BR" : lang;
    const word = player?.playing || starting ? "stop" : "play";
    button.title = tr(word, lang);
    button.setAttribute("aria-label", tr(word, lang));
  };

  const timers = new Set<number>();
  const later = (f: () => void, ms: number) => {
    const t = window.setTimeout(() => {
      timers.delete(t);
      if (alive) f();
    }, ms);
    timers.add(t);
  };
  const say = (message: string) => {
    note.textContent = message;
    note.hidden = message === "";
  };

  let player: Player | null = null;
  /** The piece compiled and not yet handed to the player. */
  let waiting: Program | null = null;
  let starting = false;
  let lease: EngineLease | null = null;
  let aratu: Awaited<typeof language.aratu> | null = null;

  const shown = (on: boolean) => {
    button.classList.toggle("on", on);
    button.innerHTML = icon(on ? "stop" : "play");
    if (!on) {
      runProgress(progress, null);
      beatPips($("beat"), $("beat").childElementCount, -1);
    }
    words();
  };

  /** The piece compiled again; the problem it has, said. */
  const compile = (): Program | null => {
    if (!aratu) return null;
    const p = aratu.compile(text, lang);
    if (!p.ok) {
      const first = p.diagnostics.find((d) => d.severity === "error") ?? p.diagnostics[0];
      say(first ? `${tr("line", lang)} ${first.line}: ${first.message}` : tr("fixFirst", lang));
      aratu.release(p.handle);
      return null;
    }
    say("");
    $("bpm").textContent = `${Math.round(p.tempo)} bpm · ${p.meter}`;
    return p;
  };

  const me: Sounding = {
    stop: () => stop(),
  };

  const stop = () => {
    const was = player?.playing || starting;
    starting = false;
    player?.stop();
    if (Session.active === me) Session.active = null;
    button.removeAttribute("aria-busy");
    shown(false);
    if (was) events.emit("stop");
  };

  const play = async () => {
    await ready;
    if (!alive || !aratu || !lease || player?.playing || starting) return;
    const p = compile();
    if (!p) return;
    if (waiting) aratu.release(waiting.handle);
    waiting = null;
    if (Session.active && Session.active !== me) Session.active.stop();
    Session.active = me;
    const engine = lease.get();
    if (player && player.engine !== engine) {
      player.dispose();
      player = null;
    }
    player ??= new Player(aratu, engine, {
      bar: (info, at) => {
        const delay = Math.max(0, (at - engine.ctx.currentTime) * 1000);
        const length = info.end - info.start;
        later(() => {
          $("bar").textContent = String(info.bar);
          $("bpm").textContent = `${Math.round(info.bpm)} bpm · ${info.meter}`;
          runProgress(progress, length);
        }, delay);
        for (let b = 0; b < info.beats; b++) {
          const offset = (info.beatTimes?.[b] ?? info.start + (b * length) / Math.max(info.beats, 1)) - info.start;
          later(() => {
            beatPips($("beat"), info.beats, b);
            later(() => $("beat").querySelector(".hit")?.classList.remove("hit"), 120);
          }, delay + offset * 1000);
        }
      },
      events: () => {},
      cut: () => {},
      swapped: () => {},
    });
    // The first play downloads the voices; the button says it is busy.
    starting = true;
    button.setAttribute("aria-busy", "true");
    shown(true);
    events.emit("play");
    await player.play(p);
    if (!alive) return;
    button.removeAttribute("aria-busy");
    // Stopped while the voices were still on their way.
    if (!starting) return player.stop();
    starting = false;
  };

  const ready = (async () => {
    const loaded = await language.aratu;
    if (!alive) return;
    aratu = loaded;
    lease = new EngineLease(
      assets,
      () => loaded.voices("en"),
      () => store.get("manguebyte") !== "off",
    );
    waiting = compile();
  })();
  ready.catch((error: unknown) => {
    console.error("Aratu: the language could not load", error);
    say(String(error));
  });

  button.onclick = () => (player?.playing || starting ? stop() : void play());
  words();

  return {
    root,
    ready,
    get playing() {
      return player?.playing ?? false;
    },
    getCode: () => text,
    setCode(next: string) {
      text = next;
      if (!aratu) return;
      const p = compile();
      if (!p) return;
      if (player?.playing) {
        player.update(p);
      } else {
        if (waiting) aratu.release(waiting.handle);
        waiting = p;
      }
    },
    play,
    stop,
    setLang(l: string) {
      lang = langOf(l);
      words();
      if (aratu && !player?.playing) {
        const p = compile();
        if (p) {
          if (waiting) aratu.release(waiting.handle);
          waiting = p;
        }
      }
    },
    setTheme(t: "light" | "dark") {
      root.dataset.theme = t === "dark" ? "dark" : "light";
    },
    on: (event, f) => events.on(event, f),
    destroy() {
      if (!alive) return;
      stop();
      alive = false;
      for (const t of timers) window.clearTimeout(t);
      timers.clear();
      player?.dispose();
      player = null;
      if (waiting && aratu) aratu.release(waiting.handle);
      waiting = null;
      lease?.release();
      language.release();
      events.clear();
      root.remove();
    },
  };
}
