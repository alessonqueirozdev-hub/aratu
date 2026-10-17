// The book as pages to read: one plain HTML page per lesson and chapter in
// each language, which needs no script, for search engines, for reading and
// for sharing a link. Each page sends to the same page in the editor, where
// its examples play. Also the site map and robots.txt.

import { createHash } from "node:crypto";
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { KINDS, LANGS, escape, readBook } from "./book.mjs";
import { tr } from "../src/i18n.ts";

const web = join(dirname(fileURLToPath(import.meta.url)), "..");

/** The site's address, the one setting to change when it moves. Only what
 * must be absolute comes from it (canonical links, alternates, the site
 * map); the pages reach each other and the editor by relative links. */
export const SITE_URL = (process.env.ARATU_SITE_URL || "https://aratu.live/").replace(/\/*$/, "/");

/** The picture a link shows when shared (Open Graph, Twitter): a card per
 * language, drawn by scripts/og.mjs into public/. */
export const SHARE_IMAGE = { width: 1200, height: 630 };
export const shareImageFile = (lang) => `og-${lang}.png`;

const HTML_LANG = { pt: "pt-BR", es: "es", en: "en" };
const LOCALE = { pt: "pt_BR", es: "es_LA", en: "en_US" };
const LANG_NAME = { pt: "Português", es: "Español", en: "English" };
/** The page for readers whose language is none of the three. */
const DEFAULT_LANG = "en";

/** From a page's path to the site's root: `pt/manual/` → `../../`. */
const up = (path) => "../".repeat(path.split("/").filter(Boolean).length);

/** A link from the page at `from` to the editor, in the page's language,
 * at `route` (`aprender/01-bars`). */
const editor = (from, lang, route) => `${up(from)}?lang=${lang}#/${route}`;

const decode = (s) =>
  s
    .replace(/&lt;/g, "<")
    .replace(/&gt;/g, ">")
    .replace(/&quot;/g, '"')
    .replace(/&#39;/g, "'")
    .replace(/&amp;/g, "&");

/** Plain text: a space between cells, items and paragraphs, none where an
 * inline tag (`code`, `strong`) was. */
const text = (html) =>
  decode(html.replace(/<\/(td|th|tr|li|p|h\d|pre|div)>|<br\s*\/?>/g, " ").replace(/<[^>]+>/g, ""))
    .replace(/\s+/g, " ")
    .trim();

/** Cuts under `max` characters: after a sentence if one ends late enough,
 * else at a word. */
function clip(s, max = 160) {
  if (s.length <= max) return s;
  const cut = s.slice(0, max - 1);
  const sentence = cut.lastIndexOf(". ");
  if (sentence >= 90) return cut.slice(0, sentence + 1);
  return `${cut.slice(0, cut.lastIndexOf(" ")).replace(/[\s,;:.]+$/, "")}…`;
}

/** The meta description: the page's first paragraphs. */
function describe(html) {
  let d = "";
  for (const m of html.matchAll(/<p>([\s\S]*?)<\/p>/g)) {
    d = `${d} ${text(m[1])}`.trim();
    if (d.length >= 120) break;
  }
  return clip(d || text(html.replace(/<h1[\s\S]*?<\/h1>/, "")));
}

/** An Aratu block on a page to read: its code, a link to hear it in the
 * editor and, for an exercise, an answer to open. */
function readBlock(a, where) {
  const { lang, kind, id, path } = where;
  const label = tr(a.kind === "exercise" ? "exercise" : "example", lang);
  const answer =
    a.answer === undefined
      ? ""
      : `<details class="block-answer"><summary>${escape(tr("showAnswer", lang))}</summary>` +
        `<pre><code class="language-aratu">${escape(a.answer)}</code></pre></details>`;
  return (
    `<div class="aratu-block read-block" data-kind="${a.kind}">` +
    `<div class="block-top"><span class="block-label">${escape(label)}</span>` +
    `<span class="block-buttons"><a class="btn btn-play" href="${editor(path, lang, `${KINDS[kind].route}/${id}`)}">` +
    `▶ ${escape(tr("hearInAratu", lang))}</a></span></div>` +
    `<pre><code class="language-aratu">${escape(a.code)}</code></pre>${answer}</div>`
  );
}

/** Every page to read, in every language: `key` names a page the same in
 * all of them, for the links between languages. */
function pages() {
  const all = [];
  for (const lang of LANGS) {
    const book = readBook(lang, readBlock);
    all.push({ key: "home", lang, kind: "home", path: `${lang}/`, book });
    all.push({ key: "course", lang, kind: "learn", path: `${lang}/${KINDS.learn.dir[lang]}/`, book });
    for (const kind of Object.keys(KINDS)) {
      book[kind].forEach((page, index) => {
        all.push({ key: `${kind}/${page.id}`, lang, kind, path: page.path, page, index, book });
      });
    }
  }
  return all;
}

/** Turns the book's links into the editor (`#/por-dentro`, `#/manual/3`)
 * into links to the pages to read. */
function relink(html, p) {
  return html.replace(/href="#\/([^"]*)"/g, (_m, route) => {
    const [view, id] = route.split("/");
    if (view === "tocar") return `href="${editor(p.path, p.lang, "tocar")}"`;
    const kind = Object.keys(KINDS).find((k) => KINDS[k].route === view);
    const list = kind ? p.book[kind] : [];
    const target = kind === "learn" && !id ? { path: `${p.lang}/${KINDS.learn.dir[p.lang]}/` } : list.find((x) => x.id === (id ?? "intro"));
    if (!target) throw new Error(`${p.path}: no page for the link #/${route}`);
    return `href="${up(p.path)}${target.path}"`;
  });
}

function head(p, { title, description, type }, alternates, style) {
  const url = SITE_URL + p.path;
  const image = SITE_URL + shareImageFile(p.lang);
  const links = Object.entries(alternates)
    .map(([lang, path]) => `<link rel="alternate" hreflang="${lang}" href="${SITE_URL}${path}" />`)
    .join("\n    ");
  const locales = LANGS.filter((l) => l !== p.lang && alternates[l])
    .map((l) => `<meta property="og:locale:alternate" content="${LOCALE[l]}" />`)
    .join("\n    ");
  return `<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover" />
    <title>${escape(title)}</title>
    <meta name="description" content="${escape(description)}" />
    <link rel="canonical" href="${url}" />
    ${links}
    <meta property="og:type" content="${type}" />
    <meta property="og:site_name" content="Aratu" />
    <meta property="og:title" content="${escape(title)}" />
    <meta property="og:description" content="${escape(description)}" />
    <meta property="og:url" content="${url}" />
    <meta property="og:image" content="${image}" />
    <meta property="og:image:type" content="image/png" />
    <meta property="og:image:width" content="${SHARE_IMAGE.width}" />
    <meta property="og:image:height" content="${SHARE_IMAGE.height}" />
    <meta property="og:image:alt" content="${escape(tr("shareImageAlt", p.lang))}" />
    <meta property="og:locale" content="${LOCALE[p.lang]}" />
    ${locales}
    <meta name="twitter:card" content="summary_large_image" />
    <meta name="twitter:title" content="${escape(title)}" />
    <meta name="twitter:description" content="${escape(description)}" />
    <meta name="twitter:image" content="${image}" />
    <meta name="twitter:image:alt" content="${escape(tr("shareImageAlt", p.lang))}" />
    <link rel="icon" href="${up(p.path)}mascot.png" />
    <link rel="preconnect" href="https://fonts.googleapis.com" />
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin />
    <link rel="stylesheet" href="${FONTS}" />
    <link rel="stylesheet" href="${up(p.path)}style.css?v=${style}" />
    <script>
      // The editor's dark theme, if chosen there; the page reads the same without it.
      try {
        if (localStorage.getItem("aratu.theme") === "dark") document.documentElement.dataset.theme = "dark";
      } catch {}
    </script>
  </head>`;
}

// The editor's faces (index.html loads the same).
const FONTS =
  "https://fonts.googleapis.com/css2?family=Atkinson+Hyperlegible+Mono:ital,wght@0,400;0,600;1,400&family=Atkinson+Hyperlegible+Next:ital,wght@0,400;0,600;0,700;1,400&family=Gloock&display=swap";

function header(p, alternates, homes) {
  const lang = p.lang;
  const top = up(p.path);
  const tab = (kind, word, href) =>
    `<a href="${href}" data-icon="${kind}"${p.kind === kind ? ' class="here"' : ""}>${escape(tr(word, lang))}</a>`;
  const langs = LANGS.map(
    (l) =>
      `<a href="${top}${alternates[l] ?? homes[l]}" hreflang="${l}" lang="${HTML_LANG[l]}" title="${LANG_NAME[l]}"${
        l === lang ? ' class="on" aria-current="true"' : ""
      }>${l.toUpperCase()}</a>`,
  ).join("");
  return `<header class="top">
      <a class="brand" href="${top}${lang}/">
        <img src="${top}mascot.png" alt="" width="44" height="44" />
        <span class="brand-text">
          <span class="name">Aratu</span>
          <span class="tagline">${escape(tr("tagline", lang))}</span>
        </span>
      </a>
      <nav class="tabs" aria-label="Aratu">
        ${tab("play", "navPlay", editor(p.path, lang, "tocar"))}
        ${tab("learn", "navLearn", `${top}${lang}/${KINDS.learn.dir[lang]}/`)}
        ${tab("manual", "navManual", `${top}${lang}/${KINDS.manual.dir[lang]}/`)}
        ${tab("inside", "navInside", `${top}${lang}/${KINDS.inside.dir[lang]}/`)}
      </nav>
      <div class="prefs">
        <nav class="lang" aria-label="${escape(tr("language", lang))}">${langs}</nav>
      </div>
    </header>`;
}

function footer(lang) {
  return `<footer class="bottom">
      <span class="links">
        ${escape(tr("ecosystem", lang))}
        <a href="https://github.com/alessonqueirozdev-hub/manguebyte">Manguebyte</a>
        ·
        <a href="https://github.com/alessonqueirozdev-hub/aratu">${escape(tr("source", lang))}</a>
      </span>
    </footer>`;
}

/** The button to the editor, under a page's title. */
function openIn(p, route, word) {
  return (
    `<p class="open-in"><a class="btn btn-play big" href="${editor(p.path, p.lang, route)}">` +
    `▶ ${escape(tr("openInAratu", p.lang))}</a> <span>${escape(tr(word, p.lang))}</span></p>`
  );
}

/** A list of pages with their descriptions, for the course's index and the
 * home page. */
function list(p, kind) {
  const items = p.book[kind]
    .map((x) => `<li><a href="${up(p.path)}${x.path}">${escape(x.title)}</a><p>${escape(describe(x.html))}</p></li>`)
    .join("\n        ");
  return `<ol class="read-list">\n        ${items}\n      </ol>`;
}

/** What a page says: its title, description and body. */
function content(p) {
  const lang = p.lang;
  if (p.kind === "home") {
    const section = (kind, word) =>
      `<h2><a href="${up(p.path)}${lang}/${KINDS[kind].dir[lang]}/">${escape(tr(word, lang))}</a></h2>\n      ${list(p, kind)}`;
    return {
      title: `Aratu · ${tr("tagline", lang)}`,
      description: clip(tr("siteIntro", lang)),
      type: "website",
      main: `<main class="view read-view">
    <article class="page">
      <h1>Aratu</h1>
      <p>${escape(tr("siteIntro", lang))}</p>
      <p class="open-in"><a class="btn btn-play big" href="${editor(p.path, lang, "tocar")}">▶ ${escape(
        tr("openEditor", lang),
      )}</a></p>
      ${section("learn", "courseName")}
      ${section("manual", "manualName")}
      ${section("inside", "insideName")}
    </article>
  </main>`,
    };
  }
  if (p.key === "course") {
    return {
      title: tr("courseName", lang),
      description: clip(tr("courseIntro", lang)),
      type: "website",
      main: `<main class="view read-view">
    <article class="page">
      <h1>${escape(tr("courseName", lang))}</h1>
      <p>${escape(tr("courseIntro", lang))}</p>
      ${openIn(p, KINDS.learn.route, "openHelpLesson")}
      ${list(p, "learn")}
    </article>
  </main>`,
    };
  }
  const { page, index, kind } = p;
  const pages = p.book[kind];
  const base = up(p.path);
  const toc = pages
    .map(
      (x) =>
        `<li><a href="${base}${x.path}"${x.id === page.id ? ' class="here" aria-current="page"' : ""}>${escape(x.title)}</a></li>`,
    )
    .join("\n          ");
  const nav = [
    index > 0 ? `<a href="${base}${pages[index - 1].path}">← ${escape(pages[index - 1].title)}</a>` : "",
    index < pages.length - 1
      ? `<a class="next" href="${base}${pages[index + 1].path}">${escape(pages[index + 1].title)} →</a>`
      : "",
  ].join("");
  const name = { learn: "courseName", manual: "manualName", inside: "insideName" }[kind];
  const word = kind === "learn" ? "openHelpLesson" : "openHelp";
  const body = relink(page.html, p).replace("</h1>", `</h1>\n${openIn(p, `${KINDS[kind].route}/${page.id}`, word)}`);
  return {
    title: page.id === "intro" ? page.title : `${page.title.replace(/^\d+\.\s*/, "")} · ${tr(name, lang)}`,
    description: describe(page.html),
    type: "article",
    main: `<main class="view book-view">
    <aside class="toc">
      <h2>${escape(tr(kind === "learn" ? "lessons" : "chapters", lang))}</h2>
      <ol>
          ${toc}
      </ol>
    </aside>
    <article class="page">
${body}
      <nav class="page-nav">${nav}</nav>
    </article>
  </main>`,
  };
}

/** Writes every page to read into `outDir`, with sitemap.xml and
 * robots.txt, and returns the pages written: their path, title and
 * description. */
export function buildSite(outDir) {
  const style = createHash("sha256")
    .update(readFileSync(join(web, "src", "style.css")))
    .digest("hex")
    .slice(0, 10);
  const all = pages();
  const homes = Object.fromEntries(LANGS.map((l) => [l, `${l}/`]));
  const byKey = new Map();
  for (const p of all) byKey.set(p.key, { ...byKey.get(p.key), [p.lang]: p.path });
  const alternatesOf = (key) => {
    const paths = byKey.get(key);
    return { ...paths, "x-default": paths[DEFAULT_LANG] ?? paths[LANGS[0]] };
  };
  const written = [];
  for (const p of all) {
    const alternates = alternatesOf(p.key);
    const c = content(p);
    const html = `<!doctype html>
<html lang="${HTML_LANG[p.lang]}" data-theme="light">
  ${head(p, c, alternates, style)}
  <body>
    ${header(p, alternates, homes)}
  ${c.main}
    ${footer(p.lang)}
  </body>
</html>
`;
    mkdirSync(join(outDir, p.path), { recursive: true });
    writeFileSync(join(outDir, p.path, "index.html"), html);
    written.push({ key: p.key, lang: p.lang, path: p.path, title: c.title, description: c.description });
  }

  // The site map: the editor and every page to read, each with its
  // versions in the other languages.
  const urls = [`  <url>\n    <loc>${SITE_URL}</loc>\n  </url>`];
  for (const p of all) {
    const alt = Object.entries(alternatesOf(p.key))
      .map(([l, path]) => `\n    <xhtml:link rel="alternate" hreflang="${l}" href="${SITE_URL}${path}" />`)
      .join("");
    urls.push(`  <url>\n    <loc>${SITE_URL}${p.path}</loc>${alt}\n  </url>`);
  }
  writeFileSync(
    join(outDir, "sitemap.xml"),
    `<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9" xmlns:xhtml="http://www.w3.org/1999/xhtml">
${urls.join("\n")}
</urlset>
`,
  );
  writeFileSync(join(outDir, "robots.txt"), `User-agent: *\nAllow: /\n\nSitemap: ${SITE_URL}sitemap.xml\n`);
  return written;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const out = join(web, "dist");
  console.log("site built:", buildSite(out).length, "pages for", SITE_URL);
}
