// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
// The whole editor, mounted in an element of any page: the code with its
// diagnostics and knobs, the rhythm map, the Sounds panel, scenes, the
// stage and the book. Everything it makes lives under its root and goes
// with `destroy()`: no ids, no styles on the page around it, no listener
// left behind, no address changed. The site, aratu.live, is one more page
// that mounts it (main.ts).

import { EditorView } from "@codemirror/view";
import type { Program } from "./aratu";
import type { Engine, SoundProblem } from "./audio";
import { mountBlocks } from "./blocks";
import { setupDaw } from "./daw";
import { beatPips, runProgress, showMeter } from "./display";
import { PHONE, narrowScreen, setWrapping } from "./editor";
import { closeEq } from "./effects";
import { examples } from "./examples";
import { type Lang, isWord, tr } from "./i18n";
import { RowMatrix } from "./matrix";
import { RhythmMap } from "./rhythm";
import { Scope } from "./scope";
import { Session } from "./session";
import { decodePiece, encodePiece } from "./share";
import {
  type AratuStorage,
  EngineLease,
  Emitter,
  Listeners,
  assetsOf,
  langOf,
  makeStore,
  useAratu,
} from "./shared";
import { SoundsPanel } from "./sounds";
import { type Parts, editorHtml } from "./template";

/** What an editor shows: playing, or one of the book's three parts. */
export type View = "play" | "learn" | "manual" | "inside";

export const VIEWS: View[] = ["play", "learn", "manual", "inside"];

/** The address of each view in the site and in the book's links:
 * `#/tocar`, `#/aprender/<page>`. */
export const ROUTES: Record<View, string> = { play: "tocar", learn: "aprender", manual: "manual", inside: "por-dentro" };

/** The view and page an address like `#/aprender/compassos` names; a view
 * that is not one is the course. */
export function routeOf(hash: string): { view: View; page?: string } {
  const [, name = "aprender", page] = hash.replace(/^#/, "").split("?")[0].split("/");
  const view = (Object.keys(ROUTES) as View[]).find((v) => ROUTES[v] === name) ?? "learn";
  return { view, page: page || undefined };
}

/** The parts of the editor that can be left out. All are in by default. */
export interface Features {
  /** Playing in a DAW through the Aratu Bridge plugin (`ws://127.0.0.1`). */
  daw: boolean;
  /** The stage: the code full screen, for playing live. */
  stage: boolean;
  /** The course, the manual and the pages for developers. */
  book: boolean;
  /** Recording what plays into a file. */
  record: boolean;
  /** Eight places to keep versions of the piece and bring them back. */
  scenes: boolean;
  /** The example pieces, to open. */
  examples: boolean;
}

export interface EditorOptions {
  /** "pt", "es" or "en" (anything else is English). */
  lang?: string;
  theme?: "light" | "dark";
  /** The piece to open; without it, the one kept in `storage`, or the
   * first example. */
  code?: string;
  /** The view to open, and for the book the page (its id). */
  view?: View;
  page?: string;
  /** The folder with aratu.wasm, manguebyte/, samples/ and book-*.json;
   * by default the one this script was loaded from. */
  assetBase?: string | URL;
  /** aratu.wasm somewhere else (as a bundler gives its address). */
  wasmUrl?: string | URL;
  /** The deploy's stamp for the wasm files (`?v=`). */
  version?: string | null;
  /** Where to keep the piece and the choices between visits, under
   * `namespace` (by default "aratu."); without it, nothing is kept. */
  storage?: AratuStorage | null;
  namespace?: string;
  features?: Partial<Features>;
  /** The address that opens a piece's code (see share.ts); with it, the
   * Share button copies that address. */
  shareLink?: (code: string) => string;
  /** Where the book's pages to read without the editor are (aratu.live);
   * with it, each page links to its own. */
  pageLinks?: string | URL;
  /** What the page around covers of the window (a fixed header, a bar of
   * tabs), for following the music to keep the code in sight. */
  viewportInsets?: () => { top: number; bottom: number };
  /** The nonce of the page's Content Security Policy, for the styles
   * CodeMirror puts in a <style> element: needed only when style-src has
   * no 'unsafe-inline'. */
  nonce?: string;
  /** Where the keys for the stage, scenes and melody lists work: while the
   * editor (or nothing) has the focus, by default; or on the whole page,
   * for a page that is the editor's. */
  keys?: "editor" | "page";
}

export interface EditorEvents extends Record<string, unknown[]> {
  /** The piece was edited. */
  change: [text: string];
  play: [];
  stop: [];
  /** The editor went to another view by itself (a link in the book, "Open
   * in Play", the stage). */
  navigate: [view: View, page: string | undefined];
  /** The stage opened or closed. */
  stage: [on: boolean];
}

export interface AratuEditor {
  /** The element everything is in, with the class `aratu-root`. */
  readonly root: HTMLElement;
  /** Settles once the language has loaded and the editor can play. */
  readonly ready: Promise<void>;
  readonly playing: boolean;
  getCode(): string;
  /** Puts in a piece, as an edit (Ctrl+Z brings back the one before). */
  setCode(text: string): void;
  play(): Promise<void>;
  stop(): void;
  setLang(lang: string): void;
  setTheme(theme: "light" | "dark"): void;
  setView(view: View, page?: string): void;
  /** Opens the piece a shared link's code holds; false when it cannot be
   * read. */
  openShared(code: string): Promise<boolean>;
  on<K extends keyof EditorEvents>(event: K, f: (...args: EditorEvents[K]) => void): () => void;
  destroy(): void;
}

interface Page {
  id: string;
  title: string;
  html: string;
  /** Its page to read, without the editor, from the site's root. */
  path: string;
}

interface Book {
  lang: Lang;
  manual: Page[];
  /** For developers: the language's words, grammar and workings. */
  inside: Page[];
  learn: Page[];
}

type Kind = Exclude<View, "play">;

/** The nearest box around `el` that scrolls, or null for the page. */
function scrollerOf(el: HTMLElement): HTMLElement | null {
  for (let p = el.parentElement; p && p !== document.body && p !== document.documentElement; p = p.parentElement) {
    const o = getComputedStyle(p).overflowY;
    if ((o === "auto" || o === "scroll") && p.scrollHeight > p.clientHeight + 1) return p;
  }
  return null;
}

export function mount(el: HTMLElement, opts: EditorOptions = {}): AratuEditor {
  const features: Features = {
    daw: true,
    stage: true,
    book: true,
    record: true,
    scenes: true,
    examples: true,
    ...opts.features,
  };
  const parts: Parts = { ...features, share: !!opts.shareLink };
  const store = makeStore(opts.storage, opts.namespace);
  const assets = assetsOf(opts);
  const events = new Emitter<EditorEvents>();
  const listeners = new Listeners();
  let alive = true;

  const root = document.createElement("div");
  root.className = "aratu-root aratu-editor";
  root.innerHTML = editorHtml(parts);
  el.append(root);

  const find = <T extends HTMLElement = HTMLElement>(name: string): T | null =>
    root.querySelector<T>(`[data-ref="${name}"]`);
  const $ = <T extends HTMLElement = HTMLElement>(name: string): T => {
    const found = find<T>(name);
    if (!found) throw new Error(`Aratu: no ${name}`);
    return found;
  };
  const viewEl = (v: View) => root.querySelector<HTMLElement>(`[data-view="${v}"]`);

  let lang: Lang = langOf(opts.lang);
  let theme: "light" | "dark" = opts.theme === "dark" ? "dark" : "light";
  const onStage = () => root.classList.contains("stage");

  const applyTheme = () => {
    // On stage it is always dark, for playing in a dim room.
    root.dataset.theme = onStage() ? "dark" : theme;
  };

  const applyWords = () => {
    root.lang = lang === "pt" ? "pt-BR" : lang;
    for (const e of root.querySelectorAll<HTMLElement>("[data-word]")) {
      const w = e.dataset.word ?? "";
      if (isWord(w)) e.textContent = tr(w, lang);
    }
    // Buttons that may show only their icon say what they do.
    for (const e of root.querySelectorAll<HTMLElement>("[data-title]")) {
      const w = e.dataset.title ?? "";
      if (!isWord(w)) continue;
      e.title = tr(w, lang);
      e.setAttribute("aria-label", tr(w, lang));
    }
    const select = find<HTMLSelectElement>("examples");
    if (select) {
      const chosen = select.value;
      select.innerHTML = "";
      for (const ex of examples) {
        const o = document.createElement("option");
        o.value = ex.id;
        o.textContent = isWord(ex.id) ? tr(ex.id, lang) : ex.id;
        select.append(o);
      }
      select.value = chosen || examples[0].id;
    }
    applyTheme();
  };
  // The view on show; until the language has loaded, the rest is set
  // aside (see `start`).
  let view: View = features.book ? (opts.view ?? "play") : "play";
  let page: string | undefined = view === "play" ? undefined : opts.page;
  let show: (v: View, id: string | undefined) => void = () => {};
  let relang: () => void = () => {};
  let openShared: (code: string) => Promise<boolean> = async () => false;

  applyWords();
  for (const v of VIEWS) {
    const e = viewEl(v);
    if (e) e.hidden = v !== view;
  }

  // Before the language has loaded, the piece waits here.
  let pendingText = opts.code ?? store.get("text") ?? examples[0].text;
  let playground: Session | null = null;
  let teardown: () => void = () => {};
  const language = useAratu(assets.wasm);

  const ready = (async () => {
    const aratu = await language.aratu;
    if (!alive) return;
    teardown = start(aratu);
  })();
  ready.catch((error: unknown) => console.error("Aratu: the language could not load", error));

  /** Everything that needs the language, once it has loaded. */
  function start(aratu: Awaited<typeof language.aratu>): () => void {
    let engine: Engine | null = null;
    const lease = new EngineLease(
      assets,
      () => aratu.voices("en"),
      () => store.get("manguebyte") !== "off",
      {
        sample: (code, state) => sounds.mark(code, state),
        loading: () => soundStatus(),
        problem: () => soundStatus(),
      },
    );
    const getEngine = (): Engine => (engine = lease.get());
    const timers = new Set<number>();
    const later = (f: () => void, ms: number): number => {
      const t = window.setTimeout(() => {
        timers.delete(t);
        if (alive) f();
      }, ms);
      timers.add(t);
      return t;
    };

    // What is downloading, and the sounds that could not be downloaded, in
    // a box every view shows (the course, the manual, the stage).
    const dismissed = new Set<number>();
    const detailText = (d: string): string => {
      const [kind, ...rest] = d.split(":");
      const value = rest.join(":");
      if (kind === "timeout") return tr("detailTimeout", lang).replace("{n}", value);
      if (kind === "status") return tr("detailStatus", lang).replace("{code}", value);
      if (kind === "network") return tr("detailNetwork", lang);
      if (value === "AudioWorklet") return tr("detailWorklet", lang);
      return value || d;
    };
    const problemText = (p: SoundProblem): string => {
      const word =
        p.fallback === "engine"
          ? "problemEngine"
          : p.fallback === "copy"
            ? "problemCopy"
            : p.instead === p.voice
              ? "problemStandIn"
              : "problemSynth";
      return tr(word, lang)
        .replace("{voice}", p.voice)
        .replace("{instead}", p.instead ?? "")
        .replace("{detail}", detailText(p.detail));
    };
    const soundStatus = (): void => {
      const box = $("sound-status");
      box.innerHTML = "";
      engine = lease.peek();
      if (engine) {
        const loading = engine.loadingVoices();
        if (loading.length > 0) {
          const line = document.createElement("p");
          line.className = "sound-loading";
          line.textContent = `${tr("soundsDownloading", lang)} ${loading.join(", ")}`;
          box.append(line);
        }
        engine.problems.forEach((p, i) => {
          if (dismissed.has(i)) return;
          const line = document.createElement("p");
          line.className = "sound-problem";
          const text = document.createElement("span");
          text.textContent = problemText(p);
          const close = document.createElement("button");
          close.type = "button";
          close.textContent = "✕";
          close.title = tr("soundClose", lang);
          close.setAttribute("aria-label", tr("soundClose", lang));
          close.onclick = () => {
            dismissed.add(i);
            soundStatus();
          };
          line.append(text, close);
          box.append(line);
        });
      }
      box.hidden = box.childElementCount === 0;
    };

    // Exercises solved, kept between visits.
    const done = new Set<string>();
    try {
      for (const id of JSON.parse(store.get("done") ?? "[]") as string[]) done.add(id);
    } catch {
      // A broken entry starts empty.
    }
    const markDone = (id: string) => {
      done.add(id);
      store.set("done", JSON.stringify([...done]));
      renderLearnToc();
    };

    // The playground.
    let quantum = Number(store.get("quantum") ?? 1) || 1;
    const map = new RhythmMap($("play-grid"), {
      aratu,
      engine: getEngine,
      view: () => playground?.view ?? null,
      edited: () => session.compile(),
      quantum: () => quantum,
      maxBars: 64,
      timeline: true,
    });
    const quantumButtons = [...$("quantum").querySelectorAll<HTMLElement>("button")];
    const showQuantum = () => {
      for (const b of quantumButtons) b.classList.toggle("on", Number(b.dataset.q) === quantum);
    };
    for (const b of quantumButtons) {
      b.onclick = () => {
        quantum = Number(b.dataset.q);
        store.set("quantum", String(quantum));
        showQuantum();
        if (session.latest) map.draw(session.latest, session.text);
      };
    }
    showQuantum();
    const messages = (p: Program) => {
      const ul = $("messages");
      ul.innerHTML = "";
      if (p.diagnostics.length === 0) {
        const li = document.createElement("li");
        li.className = "good";
        li.textContent = tr("allGood", lang);
        ul.append(li);
      }
      for (const d of p.diagnostics) {
        const li = document.createElement("li");
        li.className = d.severity;
        const small = document.createElement("small");
        small.textContent = `${tr(d.severity, lang)} · ${tr("line", lang)} ${d.line}`;
        li.append(small, document.createTextNode(d.message));
        li.onclick = () => {
          session.view.dispatch({ selection: { anchor: d.from, head: d.to }, scrollIntoView: true });
          session.view.focus();
        };
        ul.append(li);
      }
      $("note").textContent = p.ok ? "" : playground?.playing ? tr("keepsPlaying", lang) : tr("fixFirst", lang);
      // On stage the first problem shows over the code, in the musician's words.
      const msg = find("stage-msg");
      if (msg) {
        const first = p.diagnostics.find((d) => d.severity === "error") ?? p.diagnostics[0];
        msg.hidden = !first;
        msg.textContent = first ? `${tr("line", lang)} ${first.line}: ${first.message}` : "";
      }
      if (p.ok) map.draw(p, playground?.text ?? "");
      if (!$("side-rows").hidden) matrix.render(p);
      stageLists();
    };

    const pips = ["beat", "stage-beat"].flatMap((n) => find(n) ?? []);
    const progress = ["bar-progress", "stage-progress"].flatMap((n) => find(n) ?? []);
    const session: Session = new Session({
      parent: $("editor"),
      text: pendingText,
      aratu,
      engine: getEngine,
      lang: () => lang,
      nonce: opts.nonce,
      onCompile: messages,
      onChange: (t) => {
        store.set("text", t);
        if (onStage()) requestAnimationFrame(() => fitFont());
        events.emit("change", t);
      },
      onBar: (info, at) => {
        map.bar(info, at);
        const ctx = getEngine().ctx;
        const delay = Math.max(0, (at - ctx.currentTime) * 1000);
        const length = info.end - info.start;
        later(() => {
          $("bar").textContent = String(info.bar);
          const n = find("stage-bar-n");
          if (n) n.textContent = String(info.bar);
          $("bpm").textContent = String(Math.round(info.bpm));
          showMeter($("meter"), info.meter);
          const bpm = find("stage-bpm");
          if (bpm) bpm.textContent = `${Math.round(info.bpm)} bpm · ${info.meter}`;
          for (const p of progress) runProgress(p, length);
        }, delay);
        for (let b = 0; b < info.beats; b++) {
          // Beats may be uneven (2+2+3): each lights when it starts.
          const offset = (info.beatTimes?.[b] ?? info.start + (b * length) / Math.max(info.beats, 1)) - info.start;
          later(() => {
            for (const p of pips) beatPips(p, info.beats, b);
            // On stage, the first beat of each bar lights the frame.
            if (b === 0) root.classList.add("downbeat");
            later(() => {
              for (const p of pips) p.querySelector(".hit")?.classList.remove("hit");
              root.classList.remove("downbeat");
            }, 120);
          }, delay + offset * 1000);
        }
      },
      onSwap: (bar, beat) => {
        const at = beat > 0 ? `${bar}, ${tr("beat", lang)} ${beat + 1}` : String(bar);
        $("note").textContent = `${tr("next", lang)} ${at}.`;
        later(() => ($("note").textContent = ""), 3000);
      },
      onLit: (spans) => followMusic(spans),
      onPlaying: (on) => {
        $("play").classList.toggle("on", on);
        if (on) scope.start();
        else scope.stop();
        find("stage-play")?.classList.toggle("on", on);
        if (!on) {
          for (const p of progress) runProgress(p, null);
          for (const p of pips) beatPips(p, p.childElementCount, -1);
        }
        if (on) map.start();
        else map.stop();
        events.emit(on ? "play" : "stop");
      },
    });
    playground = session;
    for (const p of pips) beatPips(p, 4, -1);

    // The sound drawn as it plays, on the display: the wave or (a click) its
    // frequencies.
    const scopeCanvas = $<HTMLCanvasElement>("scope");
    const scope = new Scope(
      () => [scopeCanvas],
      () => lease.peek()?.analyser ?? null,
    );
    scopeCanvas.dataset.mode = store.get("scope") === "bars" ? "bars" : "wave";
    scopeCanvas.onclick = () => {
      scopeCanvas.dataset.mode = scopeCanvas.dataset.mode === "bars" ? "wave" : "bars";
      store.set("scope", scopeCanvas.dataset.mode);
    };
    // Melody lists in the piece: `temas.2`, `temas.all`, `temas.random`,
    // where the number can be changed by a button or Alt+1…9 while it plays.
    interface ListRef {
      from: number;
      to: number;
      list: string;
      count: number;
      line: string;
      value: string;
    }
    const listRefs = (): ListRef[] => {
      const p = session.latest;
      if (!p?.lists?.length) return [];
      const counts = new Map(p.lists.map((l) => [l.name, l.count]));
      const text = session.text;
      const out: ListRef[] = [];
      for (const m of text.matchAll(/([\p{L}_][\p{L}\p{N}_]*)\.(\d+|all|random|todas|todos|tudo|acaso|aleat[oó]ri[oa]|sorteio)(?![\p{L}\p{N}_])/gu)) {
        const count = counts.get(m[1]);
        if (count === undefined || m.index === undefined) continue;
        const lineStart = text.lastIndexOf("\n", m.index) + 1;
        const head = text.slice(lineStart, m.index);
        if (!head.includes("=")) continue;
        const from = m.index + m[1].length + 1;
        out.push({ from, to: from + m[2].length, list: m[1], count, line: head.split("=")[0].trim(), value: m[2] });
      }
      return out;
    };
    const setListIndex = (r: ListRef, value: string) => {
      session.view.dispatch({ changes: { from: r.from, to: r.to, insert: value } });
    };
    const stageLists = () => {
      const box = find("stage-lists");
      if (!box) return;
      box.innerHTML = "";
      const refs = listRefs();
      box.title = refs.length ? tr("listHint", lang) : "";
      for (const r of refs) {
        const group = document.createElement("span");
        group.className = "stage-list";
        const label = document.createElement("span");
        label.className = "stage-list-label";
        label.textContent = `${r.line}: ${r.list}`;
        group.append(label);
        const choices = [
          ...Array.from({ length: r.count }, (_, i) => [String(i + 1), String(i + 1)]),
          ["all", tr("listAll", lang)],
          ["random", tr("listRandom", lang)],
        ];
        for (const [value, text] of choices) {
          const b = document.createElement("button");
          b.type = "button";
          b.className = "btn stage-list-choice";
          b.textContent = text;
          b.classList.toggle("on", value === r.value);
          b.onclick = () => {
            const fresh = listRefs().find((x) => x.from === r.from) ?? r;
            setListIndex(fresh, value);
          };
          group.append(b);
        }
        box.append(group);
      }
    };

    // The Sounds panel, beside the messages.
    const insert = (text: string) => {
      const view = session.view;
      const { from, to } = view.state.selection.main;
      view.dispatch({ changes: { from, to, insert: text }, selection: { anchor: from + text.length } });
      view.focus();
    };
    const matrix = new RowMatrix($("side-rows"), { lang: () => lang, insert });
    const sounds = new SoundsPanel($("side-sounds"), {
      aratu,
      engine: getEngine,
      lang: () => lang,
      insert,
      manguebyte: () => store.get("manguebyte") !== "off",
      setManguebyte: (on) => {
        // What sounds stops; the next play is on an engine made for the
        // new choice (another editor still using the old one keeps it).
        session.stop();
        for (const s of pageSessions) s.stop();
        store.set("manguebyte", on ? "on" : "off");
        lease.release();
        engine = null;
        dismissed.clear();
        soundStatus();
      },
    });
    const sideTab = (which: "messages" | "sounds" | "rows") => {
      for (const t of ["messages", "sounds", "rows"] as const) {
        $(`side-${t}`).hidden = which !== t;
        $(`tab-${t}`).classList.toggle("on", which === t);
      }
      if (which === "sounds") sounds.render();
      if (which === "rows") matrix.render(session.latest ?? null);
      store.set("side", which);
    };
    $("tab-messages").onclick = () => sideTab("messages");
    $("tab-sounds").onclick = () => sideTab("sounds");
    $("tab-rows").onclick = () => sideTab("rows");
    const savedSide = store.get("side");
    if (savedSide === "sounds" || savedSide === "rows") sideTab(savedSide);

    $("play").onclick = () => void session.play();
    $("stop").onclick = () => session.stop();

    // The stage: only the code, full screen and big enough for a projector,
    // for playing live in front of people.
    // A phone keeps its own size: what reads on a projector is too big there.
    const fontKey = () => (narrowScreen() ? "stageFontPhone" : "stageFont");
    const savedFont = () => Number(store.get(fontKey())) || (narrowScreen() ? 18 : 30);
    let stageFont = savedFont();
    let idleTimer = 0;
    // The size the person chose with A− and A+; "fit the screen" may make
    // the letters smaller than that, never larger.
    let shownFont = stageFont;
    const showFont = (px: number) => {
      shownFont = px;
      root.style.setProperty("--stage-font", `${px}px`);
      session.view.requestMeasure();
      session.mirror?.requestMeasure();
    };
    const setStageFont = (px: number) => {
      stageFont = Math.min(72, Math.max(narrowScreen() ? 12 : 16, px));
      store.set(fontKey(), String(stageFont));
      showFont(stageFont);
      attached = true;
      requestAnimationFrame(() => fitFont());
    };
    // Two columns on stage: a second view of the code that goes on where
    // the first ends, like a newspaper's columns, so more of the piece shows
    // without scrolling. The right column starts with the first line the
    // left one does not show whole, so no line is cut in two and none is
    // left out, and keeps to it while the piece is typed. Each column
    // scrolls on its own: scrolled by hand, a column moves alone, and from
    // then on each stays where it was put, until the letters change size,
    // the window does, or the stage opens again.
    let split = store.get("stageSplit") === "on";
    let attached = true;
    // The right column's own moves, made here, are not taken for the
    // person's.
    let placing = 0;
    // The left view knows exactly where its last line is; the right one is
    // asked to bring that line to its top, measuring as it goes.
    const followColumn = () => {
      const m = session.mirror;
      if (!m) return;
      const v = session.view;
      const s = v.scrollDOM;
      const bottom = Math.max(0, s.scrollTop + s.clientHeight - v.documentPadding.top);
      const block = v.lineBlockAtHeight(bottom);
      const pos = block.bottom > bottom + 1 ? block.from : Math.min(block.to + 1, v.state.doc.length);
      placing = performance.now() + 250;
      m.dispatch({ effects: EditorView.scrollIntoView(pos, { y: "start", yMargin: 8 }) });
    };
    // Not while the right column is being written in: there the cursor leads.
    const place = () => {
      if (alive && attached && !session.mirror?.hasFocus) followColumn();
    };
    // A scroll the person made: the wheel, a finger, the scroll bar, Page Up
    // and Page Down. A wheel's glide goes on a little after the wheel stops.
    const scrolledByHand = (e: HTMLElement): (() => boolean) => {
      let until = 0;
      const mark = () => (until = performance.now() + 500);
      e.addEventListener("wheel", mark, { passive: true });
      e.addEventListener("touchmove", mark, { passive: true });
      e.addEventListener("pointerdown", (ev) => {
        if (ev.target === e) mark();
      });
      e.addEventListener("keydown", (ev) => {
        if (ev.key === "PageUp" || ev.key === "PageDown") mark();
      });
      e.addEventListener(
        "scroll",
        () => {
          if (performance.now() < until) mark();
        },
        { passive: true },
      );
      return () => performance.now() < until;
    };
    // "Fit the screen": the letters shrink as the piece grows, so that all
    // of it shows in the columns without scrolling, down to a size still
    // readable; past that, the columns scroll. Off, the size stays and the
    // columns scroll.
    let fit = store.get("stageFit") === "on";
    const SMALLEST = 14;
    let fitting = 0;
    const fitFont = () => {
      if (!alive || !onStage()) return;
      const run = ++fitting;
      if (!fit) {
        if (shownFont !== stageFont) showFont(stageFont);
        requestAnimationFrame(place);
        return;
      }
      let steps = 0;
      let grew = false;
      const step = () => {
        if (!alive || run !== fitting) return;
        const v = session.view;
        const s = v.scrollDOM;
        const pad = v.documentPadding;
        const text = v.contentHeight - pad.top - pad.bottom;
        const columns = session.mirror ? 2 : 1;
        const room = columns * (s.clientHeight - pad.top - window.innerHeight * 0.04);
        let next = shownFont;
        if (text > room && grew) {
          next = shownFont - 1;
          steps = 99;
        } else if (text > room && shownFont > SMALLEST) {
          next = Math.max(SMALLEST, Math.floor(shownFont * Math.min(0.97, Math.sqrt(room / text))));
        } else if (text < room * 0.85 && shownFont < stageFont) {
          next = shownFont + 1;
          grew = true;
        }
        if (next !== shownFont && steps++ < 40) {
          showFont(next);
          requestAnimationFrame(() => requestAnimationFrame(step));
        } else {
          if (next !== shownFont) showFont(next);
          // Once the letters have their size and the lines their places.
          requestAnimationFrame(() => requestAnimationFrame(place));
          later(place, 300);
        }
      };
      requestAnimationFrame(() => requestAnimationFrame(step));
    };
    // The left column scrolled by hand moves alone; moved by the cursor as
    // the piece is typed, the right one follows.
    const leftByHand = scrolledByHand(session.view.scrollDOM);
    session.view.scrollDOM.addEventListener("scroll", () => {
      if (leftByHand()) attached = false;
      else place();
    });
    listeners.add(window, "resize", () => {
      attached = true;
      fitFont();
    });
    const applySplit = () => {
      // A phone is too narrow for two columns.
      const on = split && onStage() && !narrowScreen();
      root.classList.toggle("stage-split", on);
      $("editor-mirror").hidden = !on;
      find("stage-split")?.classList.toggle("on", split);
      if (on && !session.mirror) {
        const m = session.openMirror($("editor-mirror"));
        // Moved by the person, or by the cursor while typing there: the right
        // column stays where it went. Lines typed in the left column move it
        // too, keeping what it shows in place; that is not the person's move.
        const rightByHand = scrolledByHand(m.scrollDOM);
        m.scrollDOM.addEventListener("scroll", () => {
          if (performance.now() > placing && (rightByHand() || m.hasFocus)) attached = false;
        });
      }
      if (!on && session.mirror) session.unmirror();
      find("stage-fit")?.classList.toggle("on", fit);
      session.view.requestMeasure();
      // Once the second view has laid itself out, it starts where the first
      // ends.
      attached = true;
      fitFont();
      for (const ms of [250, 600]) later(place, ms);
    };
    // Following the music: while it plays, the code that sounds stays in
    // sight, in Play and on stage. Of the symbols lit at once, the screen
    // goes to where most of them are, and only when they are out of sight
    // (in either column). It waits while the musician types, clicks or
    // scrolls in the code.
    let follow = store.get("follow") === "on";
    let touched = 0;
    let followed = 0;
    const followButtons = ["follow", "stage-follow"].flatMap((n) => find(n) ?? []);
    const showFollow = () => {
      for (const b of followButtons) {
        b.classList.toggle("on", follow);
        b.setAttribute("aria-pressed", String(follow));
      }
    };
    showFollow();

    // Playing in a DAW, through the Aratu Bridge plugin (see daw.ts).
    const dawButton = find("daw");
    const daw = dawButton
      ? setupDaw({ button: dawButton, root, engine: getEngine, session: () => session, lang: () => lang, store })
      : null;
    for (const b of followButtons) {
      b.onclick = () => {
        follow = !follow;
        store.set("follow", follow ? "on" : "off");
        touched = 0;
        showFollow();
      };
    }
    const playView = viewEl("play") as HTMLElement;
    const touch = () => (touched = performance.now());
    for (const kind of ["keydown", "pointerdown", "wheel", "touchmove"] as const) {
      listeners.add(playView, kind, touch, { capture: true, passive: true });
    }
    // The part of the window the code can be seen in: under what the page
    // covers (its header, a phone's tabs) and under the transport while it
    // stays at the top; all of it on stage.
    const windowRoom = (): { top: number; bottom: number } => {
      if (onStage()) return { top: 0, bottom: window.innerHeight };
      const insets = opts.viewportInsets?.() ?? { top: 0, bottom: 0 };
      const toolbar = root.querySelector(".toolbar");
      const sticky = toolbar !== null && getComputedStyle(toolbar).position === "sticky";
      const high = Math.max(0, insets.top, sticky && toolbar ? toolbar.getBoundingClientRect().bottom : 0);
      return { top: high, bottom: window.innerHeight - insets.bottom };
    };
    /** Where a view shows text, on the screen. */
    const viewRoom = (v: EditorView): { top: number; bottom: number } => {
      const r = v.scrollDOM.getBoundingClientRect();
      const w = windowRoom();
      return { top: Math.max(r.top, w.top), bottom: Math.min(r.bottom, w.bottom) };
    };
    /** Where the text from `a` to `b` is, on the screen. */
    const onScreen = (v: EditorView, a: number, b: number): { top: number; bottom: number } => ({
      top: v.documentTop + v.lineBlockAt(a).top,
      bottom: v.documentTop + v.lineBlockAt(b).bottom,
    });
    const seen = (v: EditorView, a: number, b: number): boolean => {
      const text = onScreen(v, a, b);
      const room = viewRoom(v);
      return text.top >= room.top && text.bottom <= room.bottom;
    };
    function followMusic(spans: { from: number; to: number }[]): void {
      if (!follow || spans.length === 0 || view !== "play") return;
      const now = performance.now();
      if (now - touched < 2500 || now - followed < 400) return;
      const v = session.view;
      const end = v.state.doc.length;
      const at = spans.map((s) => Math.min(s.from, end)).sort((a, b) => a - b);
      const blocks = at.map((pos) => v.lineBlockAt(pos));
      // The stretch, at most most of a screen tall, with the most lit symbols.
      const sight = viewRoom(v);
      const room = Math.max(100, (sight.bottom - sight.top) * 0.7);
      let first = 0;
      let last = 0;
      for (let i = 0, j = 0; i < blocks.length; i++) {
        while (blocks[i].bottom - blocks[j].top > room) j++;
        if (i - j > last - first) {
          first = j;
          last = i;
        }
      }
      if (seen(v, at[first], at[last])) return;
      const m = session.mirror;
      if (m && seen(m, at[first], at[last])) return;
      followed = now;
      // The right column comes along, starting where the left one ends.
      attached = true;
      const text = onScreen(v, at[first], at[last]);
      const s = v.scrollDOM;
      if (s.scrollHeight > s.clientHeight + 1) {
        // The code scrolls in its own box: on stage, and on a phone.
        const move = (text.top + text.bottom) / 2 - (sight.top + sight.bottom) / 2;
        s.scrollTo({ top: Math.max(0, s.scrollTop + move), behavior: "smooth" });
      } else {
        // The box is as tall as the code, and what holds the editor scrolls.
        const w = windowRoom();
        const move = (text.top + text.bottom) / 2 - (w.top + w.bottom) / 2;
        const box = scrollerOf(root);
        if (box) box.scrollTo({ top: Math.max(0, box.scrollTop + move), behavior: "smooth" });
        else window.scrollTo({ top: Math.max(0, window.scrollY + move), behavior: "smooth" });
      }
    }
    // The controls fade when the mouse rests, and come back when it moves.
    const wake = () => {
      root.classList.remove("idle");
      window.clearTimeout(idleTimer);
      if (onStage()) idleTimer = window.setTimeout(() => alive && root.classList.add("idle"), 2500);
    };
    const enterStage = () => {
      if (!features.stage || onStage()) return;
      if (view !== "play") go("play");
      root.classList.add("stage");
      applyTheme();
      setStageFont(savedFont());
      applySplit();
      root.requestFullscreen?.().catch(() => {
        // Without full screen (an iframe, an old browser, a phone) the stage
        // still covers the window.
      });
      wake();
      session.view.focus();
      events.emit("stage", true);
    };
    const leaveStage = () => {
      if (!onStage()) return;
      root.classList.remove("stage", "stage-map", "idle");
      applySplit();
      window.clearTimeout(idleTimer);
      applyTheme();
      if (document.fullscreenElement === root) document.exitFullscreen().catch(() => {});
      session.view.requestMeasure();
      events.emit("stage", false);
    };
    if (features.stage) {
      $("stage").onclick = enterStage;
      $("stage-exit").onclick = leaveStage;
      $("stage-split").onclick = () => {
        split = !split;
        store.set("stageSplit", split ? "on" : "off");
        applySplit();
      };
      $("stage-fit").onclick = () => {
        fit = !fit;
        store.set("stageFit", fit ? "on" : "off");
        $("stage-fit").classList.toggle("on", fit);
        attached = true;
        fitFont();
      };
      $("stage-play").onclick = () => void session.play();
      $("stage-stop").onclick = () => session.stop();
      $("stage-smaller").onclick = () => setStageFont(stageFont - 4);
      $("stage-bigger").onclick = () => setStageFont(stageFont + 4);
      $("stage-map").onclick = () => {
        root.classList.toggle("stage-map");
        $("stage-map").classList.toggle("on", root.classList.contains("stage-map"));
        session.view.requestMeasure();
      };
      listeners.add(document, "fullscreenchange", () => {
        if (document.fullscreenElement !== root && onStage() && wasFull) leaveStage();
        wasFull = document.fullscreenElement === root;
      });
      listeners.add(root, "mousemove", wake);
      listeners.add(root, "pointerdown", wake);
    }
    // Full screen left by the browser (Esc, F11) closes the stage too.
    let wasFull = false;
    // Turning a tablet, or narrowing the window: long lines wrap on a phone,
    // and the stage goes to one column.
    listeners.media(window.matchMedia(PHONE), () => {
      if (!session.mirror) setWrapping(session.view, narrowScreen());
      if (onStage()) {
        setStageFont(savedFont());
        applySplit();
      }
    });
    // The keys work while the editor has the focus, or nothing on the page
    // has it, or on stage; or anywhere, on a page that is the editor's.
    const ours = (e: Event) => {
      const t = e.target as Node | null;
      if (opts.keys === "page" || onStage()) return true;
      return t === document.body || t === document.documentElement || (!!t && root.contains(t));
    };
    listeners.add(
      window,
      "keydown",
      (e) => {
        if (!ours(e)) return;
        const digit = /^Digit([1-9])$/.exec(e.code);
        if (e.altKey && !e.ctrlKey && !e.metaKey && digit && view === "play") {
          // Alt+N: melody N of the list on the cursor's line, or of the first.
          const refs = listRefs();
          if (refs.length) {
            const cursor = session.view.state.selection.main.head;
            const text = session.text;
            const lineStart = text.lastIndexOf("\n", cursor - 1) + 1;
            const lineEnd = text.indexOf("\n", cursor);
            const here = refs.find((r) => r.from >= lineStart && (lineEnd < 0 || r.from <= lineEnd)) ?? refs[0];
            const n = Number(digit[1]);
            if (n <= here.count) {
              e.preventDefault();
              setListIndex(here, String(n));
            }
          }
          return;
        }
        if (!features.stage) return;
        if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key.toLowerCase() === "f") {
          e.preventDefault();
          if (onStage()) leaveStage();
          else enterStage();
        } else if (e.key === "Escape" && onStage() && document.fullscreenElement !== root) {
          leaveStage();
        }
      },
      true,
    );

    const examplesSelect = find<HTMLSelectElement>("examples");
    if (examplesSelect) {
      examplesSelect.onchange = () => {
        const ex = examples.find((x) => x.id === examplesSelect.value);
        if (ex) session.text = ex.text;
      };
    }

    // A note under the messages, for a while.
    const note = (text: string, ms = 6000) => {
      $("note").textContent = text;
      later(() => {
        if ($("note").textContent === text) $("note").textContent = "";
      }, ms);
    };
    // Share: the piece goes in a link, in the part after #, so it never
    // reaches a server; whoever opens the link gets it in the editor.
    const shareButton = find("share");
    if (shareButton && opts.shareLink) {
      const linkFor = opts.shareLink;
      shareButton.onclick = async () => {
        const link = linkFor(await encodePiece(session.text));
        const long = link.length > 8000 ? ` ${tr("shareLong", lang)}` : "";
        try {
          await navigator.clipboard.writeText(link);
          note(tr("shareCopied", lang) + long);
        } catch {
          window.prompt(tr("shareShown", lang) + long, link);
        }
      };
    }
    openShared = async (code: string) => {
      const text = await decodePiece(code);
      if (!alive) return false;
      if (text === null) {
        note(tr("sharedBroken", lang), 10000);
        return false;
      }
      if (text === session.text) return true;
      // Set as an edit, so Ctrl+Z brings back the piece that was here.
      session.text = text;
      note(tr("sharedOpened", lang), 10000);
      return true;
    };

    // Record: what reaches the speakers, until pressed again; then the file
    // downloads, as WAV when the browser can make one.
    const recordButton = find("record");
    const recordShown = (on: boolean) => {
      if (!recordButton) return;
      const word = find("record-word");
      recordButton.classList.toggle("on", on);
      if (word) {
        word.dataset.word = on ? "recordStop" : "record";
        word.textContent = tr(on ? "recordStop" : "record", lang);
      }
      recordButton.dataset.title = on ? "recordStop" : "record";
      recordButton.title = tr(on ? "recordStop" : "record", lang);
      recordButton.setAttribute("aria-label", tr(on ? "recordStop" : "record", lang));
    };
    const download = (blob: Blob, name: string) => {
      const a = document.createElement("a");
      a.href = URL.createObjectURL(blob);
      a.download = name;
      a.hidden = true;
      root.append(a);
      a.click();
      a.remove();
      window.setTimeout(() => URL.revokeObjectURL(a.href), 60_000);
    };
    if (recordButton) {
      recordButton.onclick = async () => {
        const engine = getEngine();
        if (engine.recording) {
          recordShown(false);
          const made = await engine.stopRecording();
          if (!made || !alive) return;
          const stamp = new Date().toISOString().slice(0, 19).replace(/[T:]/g, "-");
          const ext = made.wav
            ? "wav"
            : made.recorded.type.includes("ogg")
              ? "ogg"
              : made.recorded.type.includes("mp4")
                ? "m4a"
                : "webm";
          download(made.wav ?? made.recorded, `aratu-${stamp}.${ext}`);
          note(tr("recordSaved", lang));
          return;
        }
        await engine.ctx.resume();
        if (!engine.startRecording()) return note(tr("recordUnsupported", lang));
        recordShown(true);
        if (!session.playing) void session.play();
      };
    }

    // Scenes: the piece as it is, kept in one of eight places and brought back
    // with one click (or Ctrl+Shift+1 to 8) while playing; the change comes in
    // at the next bar line, like any edit, and Ctrl+Z undoes it. Kept between
    // visits.
    const SCENES = 8;
    const loadScenes = (): (string | null)[] => {
      try {
        const v = JSON.parse(store.get("scenes") ?? "[]") as unknown;
        if (Array.isArray(v)) {
          return Array.from({ length: SCENES }, (_, i) => (typeof v[i] === "string" ? (v[i] as string) : null));
        }
      } catch {
        // A broken entry starts empty.
      }
      return Array<string | null>(SCENES).fill(null);
    };
    const scenes = loadScenes();
    let sceneOn = -1;
    const renderScenes = () => {
      for (const box of ["scenes", "stage-scenes"].flatMap((n) => find(n) ?? [])) {
        box.innerHTML = "";
        const label = document.createElement("span");
        label.className = "scenes-label";
        label.textContent = tr("scenes", lang);
        box.append(label);
        scenes.forEach((text, i) => {
          const b = document.createElement("button");
          b.type = "button";
          b.className = "btn scene";
          b.textContent = String(i + 1);
          b.classList.toggle("filled", text !== null);
          b.classList.toggle("on", i === sceneOn);
          const first = text?.split("\n").find((l) => l.trim()) ?? "";
          b.title =
            text === null
              ? tr("sceneEmpty", lang)
              : `${first.slice(0, 60)}\n${tr("sceneHint", lang).replace("{n}", String(i + 1))}`;
          b.onclick = (e) => (e.shiftKey || text === null ? saveScene(i) : recallScene(i));
          box.append(b);
        });
      }
    };
    const saveScene = (i: number) => {
      scenes[i] = session.text;
      sceneOn = i;
      store.set("scenes", JSON.stringify(scenes));
      note(tr("sceneSaved", lang).replace("{n}", String(i + 1)));
      renderScenes();
    };
    const recallScene = (i: number) => {
      const text = scenes[i];
      if (text === null) return;
      sceneOn = i;
      if (text !== session.text) session.text = text;
      note(tr(session.playing ? "sceneComing" : "sceneRecalled", lang).replace("{n}", String(i + 1)));
      renderScenes();
    };
    if (features.scenes) {
      listeners.add(document, "keydown", (e) => {
        if (!e.ctrlKey || !e.shiftKey || e.altKey || view !== "play" || !ours(e)) return;
        const n = Number(e.code.replace(/^(Digit|Numpad)/, ""));
        if (Number.isInteger(n) && n >= 1 && n <= SCENES) {
          e.preventDefault();
          recallScene(n - 1);
        }
      });
      renderScenes();
    }

    // The book: the course, the manual and the page for developers.
    const books = new Map<Lang, Promise<Book | null>>();
    const bookOf = (l: Lang): Promise<Book | null> => {
      let b = books.get(l);
      if (!b) {
        // Ask the server each time: a cached copy of an older book would
        // send links to pages it does not have.
        b = fetch(new URL(`book-${l}.json`, assets.base), { cache: "no-cache" })
          .then((r) => {
            if (!r.ok) throw new Error(`book-${l}.json: ${r.status}`);
            return r.json() as Promise<Book>;
          })
          .catch((error: unknown) => {
            console.warn("Aratu: the book could not be loaded", error);
            books.delete(l);
            return null;
          });
        books.set(l, b);
      }
      return b;
    };
    let book: Book | null = null;
    let pageSessions: Session[] = [];
    const blockContext = {
      aratu,
      engine: getEngine,
      lang: () => lang,
      isDone: (id: string) => done.has(id),
      markDone,
      // Lessons start from eighths, so a beat can be built by clicking.
      quantum: () => (quantum === 1 ? 2 : quantum),
      nonce: opts.nonce,
      openInPlay: (text: string) => {
        session.text = text;
        go("play");
      },
    };

    const exerciseIds = (html: string) => [...html.matchAll(/data-kind="exercise" data-id="([^"]+)"/g)].map((m) => m[1]);

    const tocLink = (href: string, title: string, n: string, here: boolean): HTMLAnchorElement => {
      const a = document.createElement("a");
      a.href = href;
      const num = document.createElement("span");
      num.className = "toc-n";
      num.textContent = n;
      const text = document.createElement("span");
      text.textContent = title;
      a.append(num, text);
      if (here) a.classList.add("here");
      return a;
    };
    for (const toc of root.querySelectorAll<HTMLElement>(".toc")) {
      const toggle = toc.querySelector<HTMLButtonElement>(".toc-toggle");
      if (!toggle) continue;
      toggle.onclick = () => {
        const open = !toc.classList.contains("open");
        toc.classList.toggle("open", open);
        toggle.setAttribute("aria-expanded", String(open));
        const ol = toc.querySelector("ol");
        const here = toc.querySelector("a.here");
        if (open && ol && here) {
          ol.scrollTop += here.getBoundingClientRect().top - ol.getBoundingClientRect().top - ol.clientHeight / 3;
        }
      };
    }

    function renderLearnToc(current?: string): void {
      if (!book) return;
      const ol = find("learn-toc");
      if (!ol) return;
      ol.innerHTML = "";
      let total = 0;
      let solved = 0;
      for (const [i, p] of book.learn.entries()) {
        const ids = exerciseIds(p.html);
        total += ids.length;
        const n = ids.filter((id) => done.has(id)).length;
        solved += n;
        const li = document.createElement("li");
        // The welcome page has no number; the lessons count from 1.
        const number = i === 0 ? "" : String(i).padStart(2, "0");
        li.append(tocLink(`#/${ROUTES.learn}/${p.id}`, p.title, number, p.id === (current ?? currentLearn)));
        if (ids.length > 0 && n === ids.length) li.classList.add("done");
        ol.append(li);
      }
      $("learn-progress").textContent = total > 0 ? `${solved} / ${total} ${tr("doneCount", lang)}` : "";
    }

    let currentLearn = store.get("lesson") ?? "";
    /** The book's page on show, and in what language. */
    let shownPage = "";

    function showPage(kind: Kind, id: string | undefined): void {
      if (!book) return;
      const pages = book[kind] ?? [];
      if (pages.length === 0) return;
      const index = Math.max(0, pages.findIndex((p) => p.id === id));
      const page = pages[index];
      const key = `${book.lang}/${kind}/${page.id}`;
      if (key === shownPage) return;
      shownPage = key;
      for (const s of pageSessions) s.destroy();
      const article = $(`${kind}-page`);
      article.innerHTML = page.html;
      article.classList.remove("turn");
      void article.offsetWidth;
      article.classList.add("turn");
      // On a narrow screen the contents fold away, showing where you are.
      const toc = viewEl(kind)?.querySelector(".toc");
      toc?.classList.remove("open");
      toc?.querySelector(".toc-toggle")?.setAttribute("aria-expanded", "false");
      const hereLabel = toc?.querySelector(".toc-here");
      if (hereLabel) hereLabel.textContent = page.title;
      pageSessions = mountBlocks(article, blockContext);

      const nav = document.createElement("nav");
      nav.className = "page-nav";
      const base = `#/${ROUTES[kind]}/`;
      if (index > 0) {
        const a = document.createElement("a");
        a.href = base + pages[index - 1].id;
        a.textContent = `← ${pages[index - 1].title}`;
        nav.append(a);
      }
      if (index < pages.length - 1) {
        const a = document.createElement("a");
        a.href = base + pages[index + 1].id;
        a.className = "next";
        a.textContent = `${pages[index + 1].title} →`;
        nav.append(a);
      }
      article.append(nav);
      // The same page without the editor, at an address to share.
      if (opts.pageLinks !== undefined) {
        const link = document.createElement("p");
        link.className = "page-link";
        const share = document.createElement("a");
        share.href = new URL(page.path, new URL(String(opts.pageLinks), document.baseURI)).href;
        share.textContent = `${tr("pageLink", lang)} ↗`;
        link.append(share);
        article.append(link);
      }
      article.scrollTop = 0;
      // A new page starts at its top: the editor comes into sight if it was
      // scrolled past.
      const margin = parseFloat(getComputedStyle(root).scrollMarginTop) || 0;
      if (root.getBoundingClientRect().top < margin) root.scrollIntoView({ block: "start" });

      if (kind === "learn") {
        currentLearn = page.id;
        store.set("lesson", page.id);
        renderLearnToc(page.id);
      } else {
        const ol = $(`${kind}-toc`);
        ol.innerHTML = "";
        for (const p of pages) {
          const li = document.createElement("li");
          // "3. Drums": the number goes in its own column.
          const m = /^(\d+)\.\s+(.*)$/.exec(p.title);
          li.append(tocLink(base + p.id, m ? m[2] : p.title, m ? m[1].padStart(2, "0") : "", p.id === page.id));
          ol.append(li);
        }
      }
    }

    show = (v: View, id: string | undefined) => {
      for (const name of VIEWS) {
        const e = viewEl(name);
        if (e) e.hidden = name !== v;
      }
      if (v !== "play") {
        leaveStage();
        session.stop();
        const asked = lang;
        void bookOf(asked).then((b) => {
          if (!alive || view !== v || lang !== asked || !b) return;
          book = b;
          showPage(v, id ?? (v === "learn" ? currentLearn : undefined));
        });
      } else {
        for (const s of pageSessions) s.stop();
        session.view.requestMeasure();
      }
    };

    // The book's own links (`#/aprender/…`, `#/tocar`): the editor goes
    // there itself, and says so, for a page that keeps the address.
    listeners.add(root, "click", (e) => {
      if (e.defaultPrevented || e.button !== 0 || e.metaKey || e.ctrlKey || e.shiftKey || e.altKey) return;
      const a = (e.target as Element | null)?.closest?.("a[href^='#/']");
      if (!a || !root.contains(a)) return;
      e.preventDefault();
      const r = routeOf(a.getAttribute("href") ?? "");
      go(r.view, r.page);
    });

    relang = () => {
      applyWords();
      daw?.words();
      soundStatus();
      if (features.scenes) renderScenes();
      session.compile();
      if (!$("side-sounds").hidden) sounds.render();
      if (!$("side-rows").hidden) matrix.render(session.latest ?? null);
      stageLists();
      shownPage = "";
      if (view !== "play") show(view, page);
    };

    session.compile();
    show(view, page);

    return () => {
      leaveStage();
      for (const t of timers) window.clearTimeout(t);
      timers.clear();
      window.clearTimeout(idleTimer);
      daw?.dispose();
      closeEq(root);
      sounds.dispose();
      map.stop();
      scope.stop();
      for (const s of pageSessions) s.destroy();
      pageSessions = [];
      const e = lease.peek();
      if (e?.recording) void e.stopRecording();
      session.destroy();
      playground = null;
      lease.release();
    };
  }

  const setView = (v: View, id?: string) => {
    if (!features.book) v = "play";
    view = v;
    page = v === "play" ? undefined : id;
    show(v, page);
  };
  /** The editor going to a view by itself: the page is told. */
  const go = (v: View, id?: string) => {
    setView(v, id);
    events.emit("navigate", view, page);
  };

  return {
    root,
    ready,
    get playing() {
      return playground?.playing ?? false;
    },
    getCode: () => playground?.text ?? pendingText,
    setCode(text: string) {
      if (playground) playground.text = text;
      else pendingText = text;
    },
    async play() {
      await ready;
      await playground?.play();
    },
    stop: () => playground?.stop(),
    setLang(l: string) {
      lang = langOf(l);
      if (playground) relang();
      else applyWords();
    },
    setTheme(t: "light" | "dark") {
      theme = t === "dark" ? "dark" : "light";
      applyTheme();
    },
    setView,
    async openShared(code: string) {
      await ready;
      return openShared(code);
    },
    on: (event, f) => events.on(event, f),
    destroy() {
      if (!alive) return;
      alive = false;
      teardown();
      teardown = () => {};
      listeners.removeAll();
      language.release();
      events.clear();
      root.remove();
    },
  };
}
