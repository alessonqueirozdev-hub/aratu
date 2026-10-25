# Aratu as a library

Made with Aratu, by Alesson Lucas Oliveira de Queiroz (https://aratu.live).

This folder is Aratu's editor built to be mounted in any page: the code with
its diagnostics and knobs, the rhythm map, the Sounds panel, scenes, the
stage and the book (`mount`), or a piece that only plays (`mountPlayer`).
`npm run lib` in `web/` builds it into `web/dist-lib/`; `VERSION` says from
which commit.

## What is here

| File | What it is |
| --- | --- |
| `aratu.js` (and `.map`) | One ES module: `mount`, `mountPlayer`, `encodePiece`, `decodePiece`, `routeOf`. |
| `aratu.d.ts` | Its types. |
| `aratu.css` | Its styles. Every rule is under `.aratu-root`. |
| `aratu.layered.css` | The same, inside `@layer aratu`. |
| `aratu.wasm` | The language. |
| `manguebyte/` | Manguebyte's sound engine: `engine.js` (imported by `aratu.js`), `worklet.js` (loaded by the browser from its address), `manguebyte_web.wasm`. |
| `samples/` | The recorded voices, fetched the first time a voice is played. |
| `book-pt.json`, `book-es.json`, `book-en.json` | The course, the manual and the pages for developers. |
| `mascot.png` | The crab that gives the book's tips. |

Serve the whole folder as it is, from the same site as the page, and give
its address as `assetBase`. Only `aratu.js` (and `aratu.css`) may go
through a bundler; the rest is found by its address under `assetBase`.

## The editor

```js
import { mount } from "/aratu/aratu.js"; // or the path a bundler resolves

const editor = mount(document.querySelector("#editor"), {
  lang: "pt", // "pt", "es" or "en"; anything else is English
  theme: "dark", // or "light"
  assetBase: "/aratu/", // the folder above, served as it is
  code: savedPiece, // the piece to open
  storage: localStorage, // where choices are kept; without it, nothing is
  namespace: "aratu.", // the keys' prefix in storage
  features: { daw: false, stage: true, book: false },
});

editor.on("change", (code) => save(code));
editor.on("play", () => {});
editor.on("stop", () => {});
await editor.ready; // the language has loaded
editor.setCode("tempo: 100\n…"); // as an edit: Ctrl+Z brings back the one before
editor.getCode();
await editor.play();
editor.stop();
editor.setLang("en");
editor.setTheme("light");
editor.setView("learn", "intro"); // "play", "learn", "manual", "inside"
editor.destroy(); // everything it made goes: elements, listeners, sound
```

`features` leaves parts out; all are in by default:

- `daw`: playing in a DAW through Manguebyte's Aratu Bridge plugin, over
  `ws://127.0.0.1`;
- `stage`: the code full screen, for playing live (`Ctrl+Shift+F`);
- `book`: the course, the manual and the pages for developers;
- `record`, `scenes`, `examples`.

Other options: `view` and `page` (what opens first), `wasmUrl` (the wasm
somewhere else, as a bundler gives its address), `version` (a stamp put on
the wasm files as `?v=`), `shareLink(code)` (with it, a Share button copies
the address it returns; `encodePiece` and `decodePiece` turn a piece into a
link's code and back), `pageLinks` (where the book's pages to read are; with
it, each page links to its own), `viewportInsets()` (what the page covers of
the window, for following the music), `nonce` (see below), and `keys`: the
keys of the stage, the scenes and the melody lists (`Ctrl+Shift+F`,
`Ctrl+Shift+1…8`, `Alt+1…9`) work while the editor, or nothing, has the
focus; with `keys: "page"`, wherever the focus is, for a page that is the
editor's.

The editor changes nothing outside its root: not the address, not the
history, not `<html>` or `<body>`, not the title. When a link in the book,
"Open in Play" or the stage takes it to another view, it says so with
`navigate` (`(view, page) => …`), for a page that keeps the view in its
address. `stage` (`(on) => …`) says when the stage opens or closes.

## A player

```js
import { mountPlayer } from "/aratu/aratu.js";

const player = mountPlayer(card, piece.code, { lang: "pt", assetBase: "/aratu/" });
player.on("play", () => {});
await player.play();
player.stop();
player.setCode(other); // while playing, it comes in at the next bar line
player.destroy();
```

Every editor and player on a page shares one copy of the language and one
sound engine (one AudioContext, one Manguebyte worklet, every voice
downloaded once), and only one of them sounds at a time: playing one stops
the other. The last one destroyed closes the engine.

## Styles

Take `aratu.css`. Its rules are all under `.aratu-root`, the element each
editor and player is mounted in: none reaches the page around it. And none
of the page's reaches in: the root starts from the initial value of
everything it would inherit, and the elements in it from the browser's own
styles, so a reset such as Tailwind's preflight, or a page's rules for every
`button`, `a` or `h2`, change nothing inside.

Inside the root the browser's own styles also come back over the
presentational attributes of HTML (`width` and `height` on an `<img>`, say):
anything a page puts in there itself is sized with CSS.

`aratu.layered.css` is the same in `@layer aratu`, for a page that puts its
own resets in cascade layers too (Tailwind 4, say), declared before this
one. A reset outside any layer wins over every layer, whatever its
selector: with Tailwind 3, whose preflight is outside any layer, take
`aratu.css`. (CodeMirror's own styles are outside any layer too; the few
rules of Aratu's that must hold over them are `!important`.)

Both were checked against a page with Tailwind 3's preflight and loud rules
of its own for every `button`, `a`, `p`, `h2`, `div` and `span`, with
`aratu.css` loaded before them and after them, and against the same in
layers: the editor, the book and the player are drawn exactly as on a page
with no styles of its own (`lib-test/`, `npm run lib:test`).

The page may set, on any element around the root:

- `--aratu-offset-top`: what a fixed header covers at the top, for the
  transport and the book's contents to stay in sight under it;
- `--aratu-offset-bottom`: what a bar at the foot covers, for the box that
  says what is downloading;
- `--aratu-stage-z` (on `.aratu-root`): the stage's `z-index`, 1000 by
  default.

The faces are Gloock and Atkinson Hyperlegible (SIL Open Font License); the
page loads them if it wants them, from Google Fonts or its own copy.
Without them the editor falls back to the system's serif, sans and mono,
and reads the same.

## What the browser is asked for

Nothing uses `eval` or `new Function`, and no script or worklet comes from
`blob:`. A Content Security Policy needs:

- `script-src 'self' 'wasm-unsafe-eval'`: the module, the worklet (from
  `assetBase`, by its address) and the two wasm modules;
- `connect-src 'self'`: the wasm, the samples and the book are fetched from
  `assetBase`; with `features.daw`, also `ws://127.0.0.1:*`;
- `img-src 'self' data:`: the paper's grain and a few marks are drawn in
  the styles as `data:` images, and the book's tips show `mascot.png`.

A recording downloads as a file, through a `blob:` link the editor clicks.

CodeMirror puts its own styles in one `<style>` element of the document,
shared by every editor on the page (its rules are only for the class names
it makes up). A `style-src` without `'unsafe-inline'` lets it in only with
a nonce: pass it as `nonce` to `mount`.

## Licence

Aratu is free software under the GNU AGPL, version 3 or later, with the
additional terms in `NOTICE.md`, among them the author's attribution, which
anyone conveying it keeps: "Made with Aratu, by Alesson Lucas Oliveira de
Queiroz (https://aratu.live)". It is also offered under a commercial licence
(`COMMERCIAL.md`). The name, the logo and the crab are trademarks
(`TRADEMARKS.md`). The book is under CC BY-SA 4.0 (`BOOK-LICENSE.md`);
Manguebyte under MIT or Apache-2.0 (its licences are in `manguebyte/`); the
recorded voices are public domain (`samples/SOURCES.md`).
