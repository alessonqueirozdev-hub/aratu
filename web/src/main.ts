// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
/*! Aratu, by Alesson Lucas Oliveira de Queiroz (https://aratu.live). Free software under the
 * GNU AGPL, version 3 or later, with the terms in NOTICE.md; source:
 * https://github.com/alessonqueirozdev-hub/aratu */

// Aratu in the browser: play, learn, look things up. The site is a page
// that mounts the editor (app.ts) between its header and its footer, and
// keeps the view in the address: `#/tocar`, `#/aprender/<lesson>`.

import { ROUTES, type View, VIEWS, mount, routeOf } from "./app";
import { LANGS, type Lang, isWord, tr } from "./i18n";
import { codeInAddress, linkFor } from "./share";

const $ = <T extends HTMLElement = HTMLElement>(id: string) => document.getElementById(id) as T;

function browserStorage(): Storage | null {
  try {
    return window.localStorage;
  } catch {
    return null;
  }
}

const storage = browserStorage();

function load(key: string): string | null {
  try {
    return storage?.getItem(key) ?? null;
  } catch {
    return null;
  }
}

function save(key: string, value: string): void {
  try {
    storage?.setItem(key, value);
  } catch {
    // Private windows may refuse storage; everything works without it.
  }
}

function firstLang(): Lang {
  // The pages to read open the editor in their own language.
  const url = new URL(location.href);
  const asked = url.searchParams.get("lang");
  if (asked !== null) {
    url.searchParams.delete("lang");
    history.replaceState(null, "", url);
    if ((LANGS as string[]).includes(asked)) {
      save("aratu.lang", asked);
      return asked as Lang;
    }
  }
  const saved = load("aratu.lang");
  if (saved && (LANGS as string[]).includes(saved)) return saved as Lang;
  const nav = navigator.language.toLowerCase();
  return nav.startsWith("pt") ? "pt" : nav.startsWith("es") ? "es" : "en";
}

function main(): void {
  let lang: Lang = firstLang();
  let theme: "light" | "dark" = load("aratu.theme") === "dark" ? "dark" : "light";
  const page = document.body;
  // The header and the footer are the site's; the editor says its own words.
  const chrome = [...document.querySelectorAll<HTMLElement>("body > header, body > footer")];

  const applyTheme = () => {
    page.dataset.theme = theme;
    $("theme-light").classList.toggle("on", theme === "light");
    $("theme-dark").classList.toggle("on", theme === "dark");
    $("theme-switch").setAttribute("aria-label", tr("theme", lang));
  };
  const applyWords = () => {
    document.documentElement.lang = lang === "pt" ? "pt-BR" : lang;
    for (const part of chrome) {
      for (const el of part.querySelectorAll<HTMLElement>("[data-word]")) {
        const w = el.dataset.word ?? "";
        if (isWord(w)) el.textContent = tr(w, lang);
      }
      // Buttons that show only their icon say what they do.
      for (const el of part.querySelectorAll<HTMLElement>("[data-title]")) {
        const w = el.dataset.title ?? "";
        if (!isWord(w)) continue;
        el.title = tr(w, lang);
        el.setAttribute("aria-label", tr(w, lang));
      }
    }
    for (const l of LANGS) $(`lang-${l}`).classList.toggle("on", l === lang);
    applyTheme();
  };
  applyWords();

  const tabs = (view: View) => {
    for (const v of VIEWS) $(`tab-${v}`).classList.toggle("here", v === view);
  };

  // A shared link opens its piece in Play; the address goes back to the
  // editor alone, so a reload does not open the link again over what was
  // written since.
  const takeShared = (): string | null => {
    const code = codeInAddress();
    if (code) history.replaceState(null, "", `${location.pathname}${location.search}#/${ROUTES.play}`);
    return code;
  };
  const shared = takeShared();
  const start = routeOf(location.hash);

  const editor = mount($("aratu"), {
    lang,
    theme,
    view: start.view,
    page: start.page,
    storage,
    namespace: "aratu.",
    shareLink: linkFor,
    pageLinks: document.baseURI,
    keys: "page",
    // The header stays at the top while the page scrolls; on a phone the
    // tabs stay at the foot.
    viewportInsets: () => {
      const top = document.querySelector(".top");
      const bar = document.querySelector(".tabs");
      const high = top && getComputedStyle(top).position === "sticky" ? top.getBoundingClientRect().bottom : 0;
      const low = bar && getComputedStyle(bar).position === "fixed" ? window.innerHeight - bar.getBoundingClientRect().top : 0;
      return { top: Math.max(0, high), bottom: Math.max(0, low) };
    },
  });
  tabs(start.view);
  if (shared) void editor.openShared(shared);

  // The editor going somewhere by itself (a link in the book, the stage):
  // the address follows, as a link would have it.
  editor.on("navigate", (view, id) => {
    const hash = `#/${ROUTES[view]}${id ? `/${id}` : ""}`;
    if (location.hash !== hash) location.hash = hash;
    tabs(view);
  });
  editor.on("stage", (on) => page.classList.toggle("staged", on));
  window.addEventListener("hashchange", () => {
    const code = takeShared();
    if (code) void editor.openShared(code);
    const r = routeOf(location.hash);
    editor.setView(r.view, r.page);
    tabs(r.view);
  });

  for (const l of LANGS) {
    $(`lang-${l}`).onclick = () => {
      lang = l;
      save("aratu.lang", l);
      applyWords();
      editor.setLang(l);
    };
  }
  const setTheme = (t: "light" | "dark") => {
    theme = t;
    save("aratu.theme", theme);
    applyTheme();
    editor.setTheme(t);
  };
  $("theme-light").onclick = () => setTheme("light");
  $("theme-dark").onclick = () => setTheme("dark");
}

main();
