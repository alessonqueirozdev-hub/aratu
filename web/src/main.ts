// Aratu in the browser: play, learn, look things up.

import { Aratu, type Program } from "./aratu";
import { Engine } from "./audio";
import { drawGrid, mountBlocks } from "./blocks";
import { examples } from "./examples";
import { LANGS, type Lang, isWord, tr } from "./i18n";
import { Session } from "./session";

const $ = <T extends HTMLElement = HTMLElement>(id: string) => document.getElementById(id) as T;

function load(key: string): string | null {
  try {
    return localStorage.getItem(key);
  } catch {
    return null;
  }
}

function save(key: string, value: string): void {
  try {
    localStorage.setItem(key, value);
  } catch {
    // Private windows may refuse storage; everything works without it.
  }
}

interface Page {
  id: string;
  title: string;
  html: string;
}

interface Book {
  lang: Lang;
  manual: Page[];
  learn: Page[];
}

function firstLang(): Lang {
  const saved = load("aratu.lang");
  if (saved && (LANGS as string[]).includes(saved)) return saved as Lang;
  const nav = navigator.language.toLowerCase();
  return nav.startsWith("pt") ? "pt" : nav.startsWith("es") ? "es" : "en";
}

async function main(): Promise<void> {
  let lang: Lang = firstLang();
  let theme = load("aratu.theme") === "dark" ? "dark" : "light";

  const applyTheme = () => {
    document.documentElement.dataset.theme = theme;
    const t = $("theme");
    t.textContent = theme === "dark" ? "☀" : "☾";
    t.title = tr(theme === "dark" ? "themeLight" : "themeDark", lang);
  };

  const applyWords = () => {
    document.documentElement.lang = lang === "pt" ? "pt-BR" : lang;
    for (const el of document.querySelectorAll<HTMLElement>("[data-word]")) {
      const w = el.dataset.word ?? "";
      if (isWord(w)) el.textContent = tr(w, lang);
    }
    for (const l of LANGS) $(`lang-${l}`).classList.toggle("on", l === lang);
    const select = $<HTMLSelectElement>("examples");
    const chosen = select.value;
    select.innerHTML = "";
    for (const ex of examples) {
      const o = document.createElement("option");
      o.value = ex.id;
      o.textContent = isWord(ex.id) ? tr(ex.id, lang) : ex.id;
      select.append(o);
    }
    select.value = chosen || examples[0].id;
    applyTheme();
  };
  applyWords();

  const aratu = await Aratu.load("aratu.wasm");
  let engine: Engine | null = null;
  const getEngine = () => (engine ??= new Engine());

  // Exercises solved, kept in this browser.
  const done = new Set<string>(JSON.parse(load("aratu.done") ?? "[]") as string[]);
  const markDone = (id: string) => {
    done.add(id);
    save("aratu.done", JSON.stringify([...done]));
    renderLearnToc();
  };

  // The playground.
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
        playground.view.dispatch({ selection: { anchor: d.from, head: d.to }, scrollIntoView: true });
        playground.view.focus();
      };
      ul.append(li);
    }
    $("note").textContent = p.ok ? "" : playground?.playing ? tr("keepsPlaying", lang) : tr("fixFirst", lang);
    if (p.ok) drawGrid($("play-grid"), aratu, p, playground?.text ?? "");
  };

  const playground: Session = new Session({
    parent: $("editor"),
    text: load("aratu.text") ?? examples[0].text,
    aratu,
    engine: getEngine,
    lang: () => lang,
    onCompile: messages,
    onChange: (t) => save("aratu.text", t),
    onBar: (info, at) => {
      const ctx = getEngine().ctx;
      const delay = Math.max(0, (at - ctx.currentTime) * 1000);
      window.setTimeout(() => {
        $("bar").textContent = String(info.bar);
        $("bpm").textContent = String(Math.round(info.bpm));
        $("meter").textContent = info.meter;
      }, delay);
      const beatLen = (info.end - info.start) / Math.max(info.beats, 1);
      for (let b = 0; b < info.beats; b++) {
        window.setTimeout(() => {
          const dot = $("beat");
          dot.classList.add("on");
          window.setTimeout(() => dot.classList.remove("on"), 90);
        }, delay + b * beatLen * 1000);
      }
    },
    onSwap: (bar, beat) => {
      const at = beat > 0 ? `${bar}, ${tr("beat", lang)} ${beat + 1}` : String(bar);
      $("note").textContent = `${tr("next", lang)} ${at}.`;
      window.setTimeout(() => ($("note").textContent = ""), 3000);
    },
    onPlaying: (on) => $("play").classList.toggle("on", on),
  });
  $("play").onclick = () => void playground.play();
  $("stop").onclick = () => playground.stop();
  $<HTMLSelectElement>("examples").onchange = (e) => {
    const ex = examples.find((x) => x.id === (e.target as HTMLSelectElement).value);
    if (ex) playground.text = ex.text;
  };

  // The book: the course and the manual.
  let book: Book | null = null;
  const loadBook = async () => {
    const res = await fetch(`book-${lang}.json`);
    book = (await res.json()) as Book;
  };
  let pageSessions: Session[] = [];
  const blockContext = {
    aratu,
    engine: getEngine,
    lang: () => lang,
    isDone: (id: string) => done.has(id),
    markDone,
    openInPlay: (text: string) => {
      playground.text = text;
      location.hash = "#/tocar";
    },
  };

  const exerciseIds = (html: string) => [...html.matchAll(/data-kind="exercise" data-id="([^"]+)"/g)].map((m) => m[1]);

  function renderLearnToc(current?: string): void {
    if (!book) return;
    const ol = $("learn-toc");
    ol.innerHTML = "";
    let total = 0;
    let solved = 0;
    for (const p of book.learn) {
      const ids = exerciseIds(p.html);
      total += ids.length;
      const n = ids.filter((id) => done.has(id)).length;
      solved += n;
      const li = document.createElement("li");
      const a = document.createElement("a");
      a.href = `#/aprender/${p.id}`;
      a.textContent = p.title;
      if (p.id === (current ?? currentLearn)) a.classList.add("here");
      li.append(a);
      if (ids.length > 0 && n === ids.length) li.classList.add("done");
      ol.append(li);
    }
    $("learn-progress").textContent = total > 0 ? `${solved} / ${total} ${tr("doneCount", lang)}` : "";
  }

  let currentLearn = load("aratu.lesson") ?? "";

  function showPage(kind: "learn" | "manual", id: string | undefined): void {
    if (!book) return;
    const pages = kind === "learn" ? book.learn : book.manual;
    if (pages.length === 0) return;
    const index = Math.max(0, pages.findIndex((p) => p.id === id));
    const page = pages[index];
    for (const s of pageSessions) s.destroy();
    const article = $(kind === "learn" ? "learn-page" : "manual-page");
    article.innerHTML = page.html;
    pageSessions = mountBlocks(article, blockContext);

    const nav = document.createElement("nav");
    nav.className = "page-nav";
    const base = kind === "learn" ? "#/aprender/" : "#/manual/";
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
    article.scrollTop = 0;
    window.scrollTo(0, 0);

    if (kind === "learn") {
      currentLearn = page.id;
      save("aratu.lesson", page.id);
      renderLearnToc(page.id);
    } else {
      const ol = $("manual-toc");
      ol.innerHTML = "";
      for (const p of pages) {
        const li = document.createElement("li");
        const a = document.createElement("a");
        a.href = `#/manual/${p.id}`;
        a.textContent = p.title;
        if (p.id === page.id) a.classList.add("here");
        li.append(a);
        ol.append(li);
      }
    }
  }

  const route = () => {
    const [, view = "aprender", id] = location.hash.replace(/^#/, "").split("/");
    const which = view === "tocar" ? "play" : view === "manual" ? "manual" : "learn";
    for (const v of ["play", "learn", "manual"]) {
      $(`view-${v}`).hidden = v !== which;
      $(`tab-${v}`).classList.toggle("here", v === which);
    }
    if (which !== "play") {
      Session.active?.stop();
      showPage(which, id ?? (which === "learn" ? currentLearn : undefined));
    } else {
      for (const s of pageSessions) s.stop();
      playground.view.requestMeasure();
    }
  };

  const setLang = async (l: Lang) => {
    lang = l;
    save("aratu.lang", l);
    applyWords();
    playground.compile();
    await loadBook();
    route();
  };
  for (const l of LANGS) $(`lang-${l}`).onclick = () => void setLang(l);
  $("theme").onclick = () => {
    theme = theme === "dark" ? "light" : "dark";
    save("aratu.theme", theme);
    applyTheme();
  };

  playground.compile();
  await loadBook();
  window.addEventListener("hashchange", route);
  route();
}

void main();
