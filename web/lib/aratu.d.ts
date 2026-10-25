// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
// Made with Aratu, by Alesson Lucas Oliveira de Queiroz (https://aratu.live).
// The types of aratu.js, Aratu as a library (see README.md). The build
// copies this file into dist-lib/; src/lib-types.check.ts keeps it the same
// as the code.

/** The languages the editor speaks. */
export type Lang = "pt" | "es" | "en";
export declare const LANGS: Lang[];

/** What an editor shows: playing, or one of the book's three parts. */
export type View = "play" | "learn" | "manual" | "inside";
export declare const VIEWS: View[];
/** The address of each view on aratu.live and in the book's links:
 * `#/tocar`, `#/aprender/<page>`. */
export declare const ROUTES: Record<View, string>;
/** The view and page an address like `#/aprender/compassos` names. */
export declare function routeOf(hash: string): { view: View; page?: string };

/** Where things are kept between visits: `localStorage`, or anything that
 * answers the same two calls. */
export interface AratuStorage {
  getItem(key: string): string | null;
  setItem(key: string, value: string): void;
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
   * by default the one aratu.js was loaded from. */
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
  /** The address that opens a piece's code; with it, the Share button
   * copies that address. */
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

/** The whole editor, in a new `.aratu-root` inside `el`. */
export declare function mount(el: HTMLElement, opts?: EditorOptions): AratuEditor;

export interface PlayerOptions {
  /** "pt", "es" or "en" (anything else is English). */
  lang?: string;
  theme?: "light" | "dark";
  /** The folder with aratu.wasm, manguebyte/ and samples/; by default the
   * one aratu.js was loaded from. */
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

/** A piece that only plays (a button, the bar, its beats), in a new
 * `.aratu-root` inside `el`. */
export declare function mountPlayer(el: HTMLElement, code: string, opts?: PlayerOptions): AratuPlayer;

/** The code a shared link carries for a piece: `z` and the deflated text,
 * or `t` and the text. */
export declare function encodePiece(text: string): Promise<string>;
/** The piece a code holds, or null when it cannot be read. */
export declare function decodePiece(code: string): Promise<string | null>;
