// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
// Builds the pages to read into a scratch folder and checks them: every
// lesson and chapter in every language has its page, with a title, a
// description and all its text; every relative link lands on a file, even
// when the site lives in a folder; the site map lists every page; and every
// page, the editor's too, shows its language's card when shared.
import assert from "node:assert/strict";
import { cpSync, existsSync, mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { KINDS, LANGS, escape, readBook } from "./book.mjs";
import { pngSize } from "./og.mjs";
import { SHARE_IMAGE, SITE_URL, buildSite, shareImageFile } from "./site.mjs";
import { tr } from "../src/i18n.ts";

const web = join(dirname(fileURLToPath(import.meta.url)), "..");
const out = mkdtempSync(join(tmpdir(), "aratu-site-"));
try {
  // What copyStatic puts beside the pages, for the links to land on.
  const cards = LANGS.map((l) => join("public", shareImageFile(l)));
  for (const f of ["index.html", join("src", "style.css"), join("public", "mascot.png"), ...cards]) {
    cpSync(join(web, f), join(out, f.split(/[\\/]/).pop()));
  }
  const written = buildSite(out);
  const sitemap = readFileSync(join(out, "sitemap.xml"), "utf8");
  assert.match(readFileSync(join(out, "robots.txt"), "utf8"), new RegExp(`Sitemap: ${SITE_URL}sitemap.xml`));
  assert.ok(sitemap.includes(`<loc>${SITE_URL}</loc>`), "the site map lists the editor");

  const text = (html) =>
    html
      .replace(/<[^>]+>/g, " ")
      .replace(/\s+/g, " ")
      .trim();
  const attr = (html, re) => html.match(re)?.[1] ?? "";

  // The card a shared link shows: the page's language's, the same for Open
  // Graph and Twitter, on the site, there, and as big as the page says.
  const shareCard = (where, html, lang) => {
    const meta = (name) => attr(html, new RegExp(`<meta\\s+(?:property|name)="${name}"\\s+content="([^"]*)"`));
    const image = meta("og:image");
    assert.equal(image, SITE_URL + shareImageFile(lang), `${where}: og:image`);
    assert.equal(meta("twitter:image"), image, `${where}: twitter:image`);
    const file = join(out, image.slice(SITE_URL.length));
    assert.ok(existsSync(file), `${where}: ${image} is not there`);
    const { width, height } = pngSize(file);
    assert.deepEqual([width, height], [SHARE_IMAGE.width, SHARE_IMAGE.height], `${where}: ${image} is ${width}×${height}`);
    assert.deepEqual([meta("og:image:width"), meta("og:image:height")], [`${width}`, `${height}`], `${where}: og:image size`);
    const alt = escape(tr("shareImageAlt", lang));
    assert.equal(meta("og:image:alt"), alt, `${where}: og:image:alt`);
    assert.equal(meta("twitter:image:alt"), alt, `${where}: twitter:image:alt`);
    assert.equal(meta("twitter:card"), "summary_large_image", `${where}: twitter:card`);
  };
  shareCard("index.html", readFileSync(join(out, "index.html"), "utf8").replaceAll("%SITE_URL%", SITE_URL), "pt");

  let pages = 0;
  const check = (path, title, live) => {
    const file = join(out, path, "index.html");
    assert.ok(existsSync(file), `${path}: no page`);
    const html = readFileSync(file, "utf8");
    const lang = path.split("/")[0];
    assert.match(html, new RegExp(`<html lang="${lang === "pt" ? "pt-BR" : lang}"`), `${path}: language`);
    const t = attr(html, /<title>([^<]*)<\/title>/);
    assert.ok(t.length > 0, `${path}: no title`);
    if (title) assert.ok(t.includes(title.replace(/^\d+\.\s*/, "").replace(/&/g, "&amp;")), `${path}: title ${t}`);
    const d = attr(html, /<meta name="description" content="([^"]*)"/);
    assert.ok(d.length >= 40, `${path}: description "${d}"`);
    assert.ok(html.includes(`<link rel="canonical" href="${SITE_URL}${path}" />`), `${path}: canonical`);
    for (const l of [...LANGS, "x-default"]) {
      assert.match(html, new RegExp(`<link rel="alternate" hreflang="${l}" href="${SITE_URL}`), `${path}: hreflang ${l}`);
    }
    for (const p of ["og:title", "og:description", "og:url", "og:image"]) {
      assert.match(html, new RegExp(`<meta property="${p}" content="[^"]+"`), `${path}: ${p}`);
    }
    shareCard(path, html, lang);
    assert.ok(!/href="#\//.test(html), `${path}: a link into the editor's routes was left as it was`);
    assert.ok(sitemap.includes(`<loc>${SITE_URL}${path}</loc>`), `${path}: not in the site map`);
    if (live) {
      // Every paragraph of the page as the editor shows it, and every
      // example's code, is on the page to read.
      const all = text(html);
      for (const m of live.matchAll(/<p>([\s\S]*?)<\/p>/g)) {
        assert.ok(all.includes(text(m[1])), `${path}: missing "${text(m[1]).slice(0, 60)}"`);
      }
      const blocks = (live.match(/class="aratu-block"/g) ?? []).length;
      const code = (html.match(/<pre><code class="language-aratu">/g) ?? []).length;
      assert.ok(code >= blocks, `${path}: ${blocks} examples, ${code} shown`);
      for (const m of live.matchAll(/<textarea class="aratu-code" hidden>([\s\S]*?)<\/textarea>/g)) {
        assert.ok(html.includes(m[1]), `${path}: an example's code is missing`);
      }
    }
    // Relative links, read as if the site lived in a folder, stay in it and
    // land on a file.
    const base = new URL(`https://example.org/site/${path}`);
    for (const m of html.matchAll(/(?:href|src)="([^"]+)"/g)) {
      const url = new URL(m[1].replace(/&amp;/g, "&"), base);
      if (url.origin !== base.origin) continue;
      assert.ok(url.pathname.startsWith("/site/"), `${path}: ${m[1]} leaves the site`);
      let target = decodeURIComponent(url.pathname.slice("/site/".length));
      if (target === "" || target.endsWith("/")) target += "index.html";
      assert.ok(existsSync(join(out, target)), `${path}: ${m[1]} leads nowhere`);
    }
    pages++;
  };

  for (const lang of LANGS) {
    const book = readBook(lang);
    check(`${lang}/`);
    check(`${lang}/${KINDS.learn.dir[lang]}/`);
    for (const kind of Object.keys(KINDS)) {
      assert.ok(book[kind].length > 0, `${lang}: no ${kind} pages`);
      for (const p of book[kind]) check(p.path, p.title, p.html);
    }
  }
  assert.equal(pages, written.length, "pages written that were not checked");
  console.log(`site ok: ${pages} pages for ${SITE_URL}`);
} finally {
  rmSync(out, { recursive: true, force: true });
}
