// Turns the book (book/<lang>/manual.md, book/<lang>/inside.md and
// book/<lang>/learn/*.md) into one JSON file per language for the editor. Code blocks marked `aratu`
// become live examples; `aratu exercise` blocks become exercises, with the
// `aratu answer` block that follows them kept hidden as the answer.

import { marked } from "marked";
import { mkdirSync, readFileSync, readdirSync, writeFileSync, existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const web = join(dirname(fileURLToPath(import.meta.url)), "..");
const root = join(web, "..");
export const LANGS = ["pt", "es", "en"];

const escape = (s) =>
  s.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;").replace(/"/g, "&quot;");

export function slug(text) {
  return text
    .toLowerCase()
    .normalize("NFD")
    .replace(/[̀-ͯ]/g, "")
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-|-$/g, "");
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

/** Renders one page of Markdown, with its Aratu blocks made interactive. */
function page(md, pageId) {
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
    const data = Object.entries(a.attrs)
      .filter(([k]) => k !== "id")
      .map(([k, v]) => ` data-${k}="${escape(v)}"`)
      .join("");
    html +=
      `<div class="aratu-block" data-kind="${a.kind}" data-id="${escape(a.id)}"${data}>` +
      `<textarea class="aratu-code" hidden>${escape(a.code)}</textarea>` +
      (a.answer !== undefined ? `<textarea class="aratu-answer" hidden>${escape(a.answer)}</textarea>` : "") +
      `</div>`;
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
function manual(lang, name = "manual") {
  const file = join(root, "book", lang, `${name}.md`);
  if (!existsSync(file)) return [];
  const md = readFileSync(file, "utf8");
  const parts = md.split(/^(?=## )/m);
  const chapters = [];
  const intro = parts.shift();
  chapters.push({ id: "intro", title: title(intro), html: page(intro, `${name}-${lang}-intro`) });
  for (const part of parts) {
    const heading = part.match(/^##\s+(.+)$/m)[1].trim();
    // The chapter's number names it in every language, so switching
    // language keeps the page.
    const number = heading.match(/^(\d+)\./);
    const id = number ? number[1] : slug(heading);
    const body = part.replace(/^##\s+.+$/m, `# ${heading}`);
    chapters.push({ id, title: heading, html: page(body, `${name}-${lang}-${id}`) });
  }
  return chapters;
}

/** The course, one page per lesson file, in file name order. */
function learn(lang) {
  const dir = join(root, "book", lang, "learn");
  if (!existsSync(dir)) return [];
  return readdirSync(dir)
    .filter((f) => f.endsWith(".md"))
    .sort()
    .map((f) => {
      const md = readFileSync(join(dir, f), "utf8");
      const id = f.replace(/\.md$/, "");
      // Exercise ids are shared between languages: `lesson-n`.
      return { id, title: title(md), html: page(md, id) };
    });
}

export function buildBook(outDir) {
  mkdirSync(outDir, { recursive: true });
  for (const lang of LANGS) {
    const book = { lang, manual: manual(lang), inside: manual(lang, "inside"), learn: learn(lang) };
    writeFileSync(join(outDir, `book-${lang}.json`), JSON.stringify(book));
  }
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  buildBook(join(web, "dist"));
  console.log("book built");
}
