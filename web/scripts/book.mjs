// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
// Turns the book (book/<lang>/manual.md, book/<lang>/inside.md and
// book/<lang>/learn/*.md) into one JSON file per language for the editor. Code blocks marked `aratu`
// become live examples; `aratu exercise` blocks become exercises, with the
// `aratu answer` block that follows them kept hidden as the answer. The same
// pages, drawn without the editor, are the site's pages to read (site.mjs).

import { marked } from "marked";
import { mkdirSync, readFileSync, readdirSync, writeFileSync, existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const web = join(dirname(fileURLToPath(import.meta.url)), "..");
const root = join(web, "..");
export const LANGS = ["pt", "es", "en"];

/** The book's three parts: their route in the editor, and the folder of
 * their pages to read in each language. */
export const KINDS = {
  learn: { route: "aprender", dir: { pt: "aprender", es: "aprender", en: "learn" } },
  manual: { route: "manual", dir: { pt: "manual", es: "manual", en: "manual" } },
  inside: { route: "por-dentro", dir: { pt: "por-dentro", es: "por-dentro", en: "inside" } },
};

export const escape = (s) =>
  s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;").replace(/"/g, "&quot;");

export function slug(text) {
  return text
    .toLowerCase()
    .normalize("NFD")
    .replace(/[̀-ͯ]/g, "")
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-|-$/g, "");
}

/** Where a page to read lives, from the site's root, such as
 * `pt/aprender/dividindo-o-tempo/`. The name comes from the title without
 * the chapter's number, so a page keeps its address when chapters move;
 * the text before the first chapter is the folder's own page. */
function pathOf(lang, kind, id, title) {
  const dir = `${lang}/${KINDS[kind].dir[lang]}/`;
  return id === "intro" ? dir : `${dir}${slug(title.replace(/^\d+\.\s*/, ""))}/`;
}

/** Reads `key=value` and bare `flag` words after `aratu`. */
function info(lang) {
  const words = lang.trim().split(/\s+/).slice(1);
  let kind = "example";
  const attrs = {};
  for (const w of words) {
    if (["example", "exercise", "answer"].includes(w)) {
      kind = w;
      continue;
    }
    const [k, v] = w.split("=");
    attrs[k] = v ?? "1";
  }
  return { kind, attrs };
}

/** An Aratu block as the editor wants it: the code in hidden text areas,
 * for the editor to make a live block of. */
function liveBlock(a) {
  const data = Object.entries(a.attrs)
    .filter(([k]) => k !== "id")
    .map(([k, v]) => ` data-${k}="${escape(v)}"`)
    .join("");
  return (
    `<div class="aratu-block" data-kind="${a.kind}" data-id="${escape(a.id)}"${data}>` +
    `<textarea class="aratu-code" hidden>${escape(a.code)}</textarea>` +
    (a.answer !== undefined ? `<textarea class="aratu-answer" hidden>${escape(a.answer)}</textarea>` : "") +
    `</div>`
  );
}

/** Renders one page of Markdown, each Aratu block drawn by `block`. */
function page(md, pageId, block) {
  const tokens = marked.lexer(md);
  const out = [];
  let count = 0;
  for (const t of tokens) {
    if (t.type === "code" && (t.lang ?? "").startsWith("aratu")) {
      const { kind, attrs } = info(t.lang);
      if (kind === "answer") {
        const prev = [...out].reverse().find((x) => x.type !== "space");
        if (!prev || prev.aratu?.kind !== "exercise") {
          throw new Error(`${pageId}: an answer must follow an exercise`);
        }
        prev.aratu.answer = t.text;
        continue;
      }
      count++;
      out.push({ aratu: { kind, attrs, code: t.text, id: attrs.id ?? `${pageId}-${count}` } });
      continue;
    }
    out.push(t);
  }
  let html = "";
  let pending = [];
  const flush = () => {
    if (pending.length) {
      pending.links = tokens.links;
      html += marked.parser(pending);
      pending = [];
    }
  };
  for (const t of out) {
    if (!t.aratu) {
      pending.push(t);
      continue;
    }
    flush();
    const a = t.aratu;
    if (a.kind === "exercise" && a.answer === undefined) {
      throw new Error(`${pageId}: exercise ${a.id} has no answer`);
    }
    html += block(a);
  }
  flush();
  // Ids on headings, for links inside the page.
  html = html.replace(/<h([23])>(.*?)<\/h\1>/g, (_m, n, inner) => {
    const text = inner.replace(/<[^>]+>/g, "");
    return `<h${n} id="${slug(text)}">${inner}</h${n}>`;
  });
  return html;
}

function title(md) {
  const m = md.match(/^#\s+(.+)$/m);
  return m ? m[1].trim() : "";
}

/** The manual, or the page for developers (`inside`), one page per `## `
 * chapter, the text before them first. */
function manual(lang, block, kind = "manual") {
  const file = join(root, "book", lang, `${kind}.md`);
  if (!existsSync(file)) return [];
  const md = readFileSync(file, "utf8");
  const parts = md.split(/^(?=## )/m);
  const chapters = [];
  const one = (id, heading, body) => {
    const path = pathOf(lang, kind, id, heading);
    const where = { lang, kind, id, path };
    chapters.push({ id, title: heading, path, html: page(body, `${kind}-${lang}-${id}`, (a) => block(a, where)) });
  };
  const intro = parts.shift();
  one("intro", title(intro), intro);
  for (const part of parts) {
    const heading = part.match(/^##\s+(.+)$/m)[1].trim();
    // The chapter's number names it in every language, so switching
    // language keeps the page.
    const number = heading.match(/^(\d+)\./);
    one(number ? number[1] : slug(heading), heading, part.replace(/^##\s+.+$/m, `# ${heading}`));
  }
  return chapters;
}

/** The course, one page per lesson file, in file name order. */
function learn(lang, block) {
  const dir = join(root, "book", lang, "learn");
  if (!existsSync(dir)) return [];
  return readdirSync(dir)
    .filter((f) => f.endsWith(".md"))
    .sort()
    .map((f) => {
      const md = readFileSync(join(dir, f), "utf8");
      const id = f.replace(/\.md$/, "");
      const path = pathOf(lang, "learn", id, title(md));
      const where = { lang, kind: "learn", id, path };
      // Exercise ids are shared between languages: `lesson-n`.
      return { id, title: title(md), path, html: page(md, id, (a) => block(a, where)) };
    });
}

/** One language's book: the manual, `inside` and the course (`learn`),
 * each a list of pages with an id, a title, the path of its page to read
 * and its HTML. `block(a, where)` draws each Aratu block; `where` says
 * which page it is on. */
export function readBook(lang, block = liveBlock) {
  const book = { lang, manual: manual(lang, block), inside: manual(lang, block, "inside"), learn: learn(lang, block) };
  const seen = new Set();
  for (const kind of Object.keys(KINDS)) {
    for (const p of book[kind]) {
      if (seen.has(p.path)) throw new Error(`two pages would live at ${p.path}`);
      seen.add(p.path);
    }
  }
  return book;
}

export function buildBook(outDir) {
  mkdirSync(outDir, { recursive: true });
  for (const lang of LANGS) {
    writeFileSync(join(outDir, `book-${lang}.json`), JSON.stringify(readBook(lang)));
  }
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  buildBook(join(web, "dist"));
  console.log("book built");
}
