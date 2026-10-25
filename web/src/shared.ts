// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
// What every editor and player on a page shares: where Aratu's files are,
// what is kept between visits, the language compiled once, and one sound
// engine (one AudioContext, one Manguebyte worklet, the samples downloaded
// once), made on the first play and closed when the last one goes.

import { Aratu, type Lang, type VoiceCatalogue } from "./aratu";
import { Engine, type SoundProblem } from "./audio";

/** Where things are kept between visits: `localStorage`, or anything that
 * answers the same two calls. */
export interface AratuStorage {
  getItem(key: string): string | null;
  setItem(key: string, value: string): void;
}

/** The keys an editor keeps, under its namespace. */
export interface Store {
  get(key: string): string | null;
  set(key: string, value: string): void;
}

/** `storage` under `namespace`; without one, kept only while the page is
 * open. A storage that refuses (a private window) is the same as none. */
export function makeStore(storage: AratuStorage | null | undefined, namespace = "aratu."): Store {
  if (!storage) {
    const memory = new Map<string, string>();
    return { get: (k) => memory.get(k) ?? null, set: (k, v) => void memory.set(k, v) };
  }
  return {
    get(key) {
      try {
        return storage.getItem(namespace + key);
      } catch {
        return null;
      }
    },
    set(key, value) {
      try {
        storage.setItem(namespace + key, value);
      } catch {
        // Everything works without it.
      }
    },
  };
}

/** The languages the editor speaks; any other is English. */
export function langOf(asked: string | undefined | null): Lang {
  const l = (asked ?? "").toLowerCase();
  return l.startsWith("pt") ? "pt" : l.startsWith("es") ? "es" : "en";
}

/** Where Aratu's files are: `aratu.wasm`, `manguebyte/`, `samples/` and
 * the book's `book-<lang>.json`, all in one folder. */
export interface Assets {
  base: URL;
  /** The stamp a deploy puts on the wasm files, so a browser never mixes a
   * new page with an old copy it kept. */
  version: string | null;
  wasm: URL;
}

/** The folder, by default the one this script was loaded from. */
export function assetsOf(o: { assetBase?: string | URL; wasmUrl?: string | URL; version?: string | null }): Assets {
  const here = new URL(import.meta.url);
  const base = new URL(String(o.assetBase ?? new URL(".", here)), document.baseURI);
  if (!base.pathname.endsWith("/")) base.pathname += "/";
  const version = o.version !== undefined ? o.version : here.searchParams.get("v");
  const wasm = new URL(String(o.wasmUrl ?? new URL("aratu.wasm", base)), document.baseURI);
  if (version && o.wasmUrl === undefined) wasm.searchParams.set("v", version);
  return { base, version, wasm };
}

/** The language, compiled once for every editor that loads it from the
 * same address, and let go when the last of them is destroyed. */
const languages = new Map<string, { aratu: Promise<Aratu>; users: number }>();

export function useAratu(wasm: URL): { aratu: Promise<Aratu>; release(): void } {
  const key = wasm.href;
  let entry = languages.get(key);
  if (!entry) {
    entry = { aratu: Aratu.load(key), users: 0 };
    languages.set(key, entry);
    // A download that failed is tried again by the next editor.
    entry.aratu.catch(() => languages.get(key) === entry && languages.delete(key));
  }
  entry.users++;
  let released = false;
  return {
    aratu: entry.aratu,
    release() {
      if (released) return;
      released = true;
      if (--entry.users === 0 && languages.get(key) === entry) languages.delete(key);
    },
  };
}

type SampleState = "loading" | "ready" | "failed";

/** A sound engine shared by every editor and player that asks for the
 * same files and the same choice of Manguebyte. */
interface Shared {
  key: string;
  engine: Engine;
  users: number;
  sample: Set<(code: string, state: SampleState) => void>;
  loading: Set<(voices: string[]) => void>;
  problem: Set<(p: SoundProblem) => void>;
}

const engines = new Map<string, Shared>();

export interface EngineHooks {
  sample?(code: string, state: SampleState): void;
  loading?(voices: string[]): void;
  problem?(p: SoundProblem): void;
}

/** One editor's or player's hold on the shared engine. The engine is made
 * the first time `get` is called (browsers let sound start only after a
 * click), and made again when the choice of Manguebyte changes. */
export class EngineLease {
  private shared: Shared | null = null;

  constructor(
    private readonly assets: Assets,
    private readonly catalogue: () => VoiceCatalogue,
    private readonly manguebyte: () => boolean,
    private readonly hooks: EngineHooks = {},
  ) {}

  private keyNow(): string {
    return `${this.assets.base.href}\n${this.assets.version ?? ""}\n${this.manguebyte() ? "mb" : "stand-in"}`;
  }

  /** The engine, made now if need be. */
  get(): Engine {
    const key = this.keyNow();
    if (this.shared?.key === key) return this.shared.engine;
    this.release();
    let s = engines.get(key);
    if (!s) {
      const engine = new Engine(this.catalogue(), this.assets.base.href, {
        manguebyte: this.manguebyte(),
        version: this.assets.version,
      });
      const made: Shared = { key, engine, users: 0, sample: new Set(), loading: new Set(), problem: new Set() };
      engine.onSample = (code, state) => made.sample.forEach((f) => f(code, state));
      engine.onLoading = (voices) => made.loading.forEach((f) => f(voices));
      engine.onProblem = (p) => made.problem.forEach((f) => f(p));
      engines.set(key, made);
      s = made;
    }
    s.users++;
    if (this.hooks.sample) s.sample.add(this.hooks.sample);
    if (this.hooks.loading) s.loading.add(this.hooks.loading);
    if (this.hooks.problem) s.problem.add(this.hooks.problem);
    this.shared = s;
    return s.engine;
  }

  /** The engine if it has been made, without making it. */
  peek(): Engine | null {
    return this.shared?.engine ?? null;
  }

  /** Lets the engine go; the last to let it go closes it. */
  release(): void {
    const s = this.shared;
    if (!s) return;
    this.shared = null;
    if (this.hooks.sample) s.sample.delete(this.hooks.sample);
    if (this.hooks.loading) s.loading.delete(this.hooks.loading);
    if (this.hooks.problem) s.problem.delete(this.hooks.problem);
    if (--s.users > 0) return;
    if (engines.get(s.key) === s) engines.delete(s.key);
    void s.engine.destroy();
  }
}

/** A tiny event emitter for the handles `mount` and `mountPlayer` return. */
type Listener = (...args: unknown[]) => void;

export class Emitter<E extends Record<string, unknown[]>> {
  private readonly listeners = new Map<keyof E, Set<Listener>>();

  on<K extends keyof E>(event: K, f: (...args: E[K]) => void): () => void {
    let set = this.listeners.get(event);
    if (!set) this.listeners.set(event, (set = new Set()));
    const g = f as unknown as Listener;
    set.add(g);
    return () => void set.delete(g);
  }

  emit<K extends keyof E>(event: K, ...args: E[K]): void {
    for (const f of this.listeners.get(event) ?? []) {
      try {
        f(...args);
      } catch (error) {
        console.error("Aratu:", error);
      }
    }
  }

  clear(): void {
    this.listeners.clear();
  }
}

/** Listeners on the window and the document, taken away all at once. */
export class Listeners {
  private readonly off: (() => void)[] = [];

  add<K extends keyof WindowEventMap>(
    target: Window,
    type: K,
    f: (e: WindowEventMap[K]) => void,
    options?: boolean | AddEventListenerOptions,
  ): void;
  add<K extends keyof DocumentEventMap>(
    target: Document,
    type: K,
    f: (e: DocumentEventMap[K]) => void,
    options?: boolean | AddEventListenerOptions,
  ): void;
  add<K extends keyof HTMLElementEventMap>(
    target: HTMLElement,
    type: K,
    f: (e: HTMLElementEventMap[K]) => void,
    options?: boolean | AddEventListenerOptions,
  ): void;
  add(target: EventTarget, type: string, f: (e: never) => void, options?: boolean | AddEventListenerOptions): void {
    const listener = f as unknown as EventListener;
    target.addEventListener(type, listener, options);
    this.off.push(() => target.removeEventListener(type, listener, options));
  }

  /** A media query's changes. */
  media(query: MediaQueryList, f: () => void): void {
    query.addEventListener("change", f);
    this.off.push(() => query.removeEventListener("change", f));
  }

  /** Something else to undo with the rest: a timer, an observer. */
  also(f: () => void): void {
    this.off.push(f);
  }

  removeAll(): void {
    for (const f of this.off.splice(0)) f();
  }
}
