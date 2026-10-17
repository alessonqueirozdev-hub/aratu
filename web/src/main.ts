// Aratu in the browser: play, learn, look things up.

import { EditorView } from "@codemirror/view";
import { Aratu, type Program } from "./aratu";
import { Engine, type SoundProblem } from "./audio";
import { mountBlocks } from "./blocks";
import { RhythmMap } from "./rhythm";
import { examples } from "./examples";
import { LANGS, type Lang, isWord, tr } from "./i18n";
import { Session } from "./session";
import { SoundsPanel } from "./sounds";
import { RowMatrix } from "./matrix";

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

async function main(): Promise<void> {
  let lang: Lang = firstLang();
  let theme = load("aratu.theme") === "dark" ? "dark" : "light";

  const applyTheme = () => {
    document.documentElement.dataset.theme = theme;
    $("theme-light").classList.toggle("on", theme === "light");
    $("theme-dark").classList.toggle("on", theme === "dark");
    $("theme-switch").setAttribute("aria-label", tr("theme", lang));
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

  // The page stamps the script with its version (see scripts/build.mjs);
  // the wasm gets the same stamp, so a new deploy never mixes with an old
  // copy kept by the browser.
  const version = new URL(import.meta.url).searchParams.get("v");
  const aratu = await Aratu.load(version ? `aratu.wasm?v=${version}` : "aratu.wasm");
  let engine: Engine | null = null;
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
  const getEngine = (): Engine => {
    if (!engine) {
      engine = new Engine(aratu.voices("en"));
      engine.onSample = (code, state) => sounds.mark(code, state);
      engine.onLoading = () => soundStatus();
      engine.onProblem = () => soundStatus();
    }
    return engine;
  };

  // Exercises solved, kept in this browser.
  const done = new Set<string>(JSON.parse(load("aratu.done") ?? "[]") as string[]);
  const markDone = (id: string) => {
    done.add(id);
    save("aratu.done", JSON.stringify([...done]));
    renderLearnToc();
  };

  // The playground.
  let quantum = Number(load("aratu.quantum") ?? 1) || 1;
  const map = new RhythmMap($("play-grid"), {
    aratu,
    engine: getEngine,
    view: () => playground?.view ?? null,
    edited: () => playground.compile(),
    quantum: () => quantum,
    maxBars: 8,
  });
  const showQuantum = () => {
    for (const b of document.querySelectorAll<HTMLElement>("#quantum button")) {
      b.classList.toggle("on", Number(b.dataset.q) === quantum);
    }
  };
  for (const b of document.querySelectorAll<HTMLElement>("#quantum button")) {
    b.onclick = () => {
      quantum = Number(b.dataset.q);
      save("aratu.quantum", String(quantum));
      showQuantum();
      if (playground.latest) map.draw(playground.latest, playground.text);
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
        playground.view.dispatch({ selection: { anchor: d.from, head: d.to }, scrollIntoView: true });
        playground.view.focus();
      };
      ul.append(li);
    }
    $("note").textContent = p.ok ? "" : playground?.playing ? tr("keepsPlaying", lang) : tr("fixFirst", lang);
    // On stage the first problem shows over the code, in the musician's words.
    const first = p.diagnostics.find((d) => d.severity === "error") ?? p.diagnostics[0];
    const msg = $("stage-msg");
    msg.hidden = !first;
    msg.textContent = first ? `${tr("line", lang)} ${first.line}: ${first.message}` : "";
    if (p.ok) map.draw(p, playground?.text ?? "");
    if (!$("side-rows").hidden) matrix.render(p);
    stageLists();
  };

  const playground: Session = new Session({
    parent: $("editor"),
    text: load("aratu.text") ?? examples[0].text,
    aratu,
    engine: getEngine,
    lang: () => lang,
    onCompile: messages,
    onChange: (t) => {
      save("aratu.text", t);
      if (document.body.classList.contains("stage")) requestAnimationFrame(() => fitFont());
    },
    onBar: (info, at) => {
      map.bar(info, at);
      const ctx = getEngine().ctx;
      const delay = Math.max(0, (at - ctx.currentTime) * 1000);
      window.setTimeout(() => {
        $("bar").textContent = String(info.bar);
        $("stage-bar-n").textContent = String(info.bar);
        $("bpm").textContent = String(Math.round(info.bpm));
        $("meter").textContent = info.meter;
      }, delay);
      const beatLen = (info.end - info.start) / Math.max(info.beats, 1);
      for (let b = 0; b < info.beats; b++) {
        window.setTimeout(() => {
          const dots = [$("beat"), $("stage-beat")];
          for (const dot of dots) dot.classList.add("on");
          // On stage, the first beat of each bar lights the frame.
          if (b === 0) document.body.classList.add("downbeat");
          window.setTimeout(() => {
            for (const dot of dots) dot.classList.remove("on");
            document.body.classList.remove("downbeat");
          }, 120);
        }, delay + b * beatLen * 1000);
      }
    },
    onSwap: (bar, beat) => {
      const at = beat > 0 ? `${bar}, ${tr("beat", lang)} ${beat + 1}` : String(bar);
      $("note").textContent = `${tr("next", lang)} ${at}.`;
      window.setTimeout(() => ($("note").textContent = ""), 3000);
    },
    onPlaying: (on) => {
      $("play").classList.toggle("on", on);
      if (on) map.start();
      else map.stop();
    },
  });
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
    const p = playground.latest;
    if (!p?.lists?.length) return [];
    const counts = new Map(p.lists.map((l) => [l.name, l.count]));
    const text = playground.text;
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
  const setListIndex = (ref: ListRef, value: string) => {
    playground.view.dispatch({ changes: { from: ref.from, to: ref.to, insert: value } });
  };
  const stageLists = () => {
    const box = $("stage-lists");
    box.innerHTML = "";
    const refs = listRefs();
    box.title = refs.length ? tr("listHint", lang) : "";
    for (const ref of refs) {
      const group = document.createElement("span");
      group.className = "stage-list";
      const label = document.createElement("span");
      label.className = "stage-list-label";
      label.textContent = `${ref.line}: ${ref.list}`;
      group.append(label);
      const choices = [
        ...Array.from({ length: ref.count }, (_, i) => [String(i + 1), String(i + 1)]),
        ["all", tr("listAll", lang)],
        ["random", tr("listRandom", lang)],
      ];
      for (const [value, text] of choices) {
        const b = document.createElement("button");
        b.className = "btn stage-list-choice";
        b.textContent = text;
        b.classList.toggle("on", value === ref.value);
        b.onclick = () => {
          const fresh = listRefs().find((r) => r.from === ref.from) ?? ref;
          setListIndex(fresh, value);
        };
        group.append(b);
      }
      box.append(group);
    }
  };

  // The Sounds panel, beside the messages.
  const matrix = new RowMatrix($("side-rows"), {
    lang: () => lang,
    insert: (text) => {
      const view = playground.view;
      const { from, to } = view.state.selection.main;
      view.dispatch({ changes: { from, to, insert: text }, selection: { anchor: from + text.length } });
      view.focus();
    },
  });
  const sounds = new SoundsPanel($("side-sounds"), {
    aratu,
    engine: getEngine,
    lang: () => lang,
    insert: (code) => {
      const view = playground.view;
      const { from, to } = view.state.selection.main;
      view.dispatch({ changes: { from, to, insert: code }, selection: { anchor: from + code.length } });
      view.focus();
    },
  });
  const sideTab = (which: "messages" | "sounds" | "rows") => {
    for (const t of ["messages", "sounds", "rows"] as const) {
      $(`side-${t}`).hidden = which !== t;
      $(`tab-${t}`).classList.toggle("on", which === t);
    }
    if (which === "sounds") sounds.render();
    if (which === "rows") matrix.render(playground.latest ?? null);
    save("aratu.side", which);
  };
  $("tab-messages").onclick = () => sideTab("messages");
  $("tab-sounds").onclick = () => sideTab("sounds");
  $("tab-rows").onclick = () => sideTab("rows");
  const savedSide = load("aratu.side");
  if (savedSide === "sounds" || savedSide === "rows") sideTab(savedSide);

  $("play").onclick = () => void playground.play();
  $("stop").onclick = () => playground.stop();
  // The stage: only the code, full screen and big enough for a projector,
  // for playing live in front of people.
  let stageFont = Number(load("aratu.stageFont") ?? 30) || 30;
  let idleTimer = 0;
  const onStage = () => document.body.classList.contains("stage");
  // The size the person chose with A− and A+; "fit the screen" may make
  // the letters smaller than that, never larger.
  let shownFont = stageFont;
  const showFont = (px: number) => {
    shownFont = px;
    document.body.style.setProperty("--stage-font", `${px}px`);
    playground.view.requestMeasure();
    playground.mirror?.requestMeasure();
  };
  const setStageFont = (px: number) => {
    stageFont = Math.min(72, Math.max(16, px));
    save("aratu.stageFont", String(stageFont));
    showFont(stageFont);
    requestAnimationFrame(() => fitFont());
  };
  // Two columns on stage: a second view of the code that goes on where
  // the first ends, like a newspaper's columns, so more of the piece shows
  // without scrolling. Scrolling either moves both.
  let split = load("aratu.stageSplit") === "on";
  let linking = false;
  // The right column starts with the first line the left one does not
  // show whole, so no line is cut in two and none is left out. The left
  // view knows exactly where its last line is; the right one is asked to
  // bring that line to its top, measuring as it goes. Scrolling one column
  // moves the other; the move it causes is not taken for the person's.
  let mirrorQuiet = 0;
  let leftQuiet = 0;
  const followColumn = () => {
    const m = playground.mirror;
    if (!m || linking) return;
    linking = true;
    const v = playground.view;
    const s = v.scrollDOM;
    const bottom = Math.max(0, s.scrollTop + s.clientHeight - v.documentPadding.top);
    const block = v.lineBlockAtHeight(bottom);
    const pos = block.bottom > bottom + 1 ? block.from : Math.min(block.to + 1, v.state.doc.length);
    mirrorQuiet = performance.now() + 250;
    m.dispatch({ effects: EditorView.scrollIntoView(pos, { y: "start", yMargin: 8 }) });
    linking = false;
  };
  const leadColumn = () => {
    const m = playground.mirror;
    if (!m || linking || performance.now() < mirrorQuiet) return;
    linking = true;
    const v = playground.view;
    const s = v.scrollDOM;
    const top = m.lineBlockAtHeight(Math.max(0, m.scrollDOM.scrollTop - m.documentPadding.top));
    leftQuiet = performance.now() + 250;
    s.scrollTop = Math.max(0, v.lineBlockAt(top.from).top + v.documentPadding.top - s.clientHeight);
    linking = false;
  };
  // "Fit the screen": the letters shrink as the piece grows, so that all
  // of it shows in the columns without scrolling, down to a size still
  // readable; past that, the columns scroll. Off, the size stays and the
  // columns scroll.
  let fit = load("aratu.stageFit") === "on";
  const SMALLEST = 14;
  let fitting = 0;
  const fitFont = () => {
    if (!onStage()) return;
    const run = ++fitting;
    if (!fit) {
      if (shownFont !== stageFont) showFont(stageFont);
      requestAnimationFrame(followColumn);
      return;
    }
    let steps = 0;
    let grew = false;
    const step = () => {
      if (run !== fitting) return;
      const v = playground.view;
      const s = v.scrollDOM;
      const pad = v.documentPadding;
      const text = v.contentHeight - pad.top - pad.bottom;
      const columns = playground.mirror ? 2 : 1;
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
        requestAnimationFrame(() => requestAnimationFrame(followColumn));
        window.setTimeout(followColumn, 300);
      }
    };
    requestAnimationFrame(() => requestAnimationFrame(step));
  };
  playground.view.scrollDOM.addEventListener("scroll", () => {
    if (performance.now() >= leftQuiet) followColumn();
  });
  window.addEventListener("resize", () => fitFont());
  const applySplit = () => {
    const on = split && onStage();
    document.body.classList.toggle("stage-split", on);
    $("editor-mirror").hidden = !on;
    $("stage-split").classList.toggle("on", split);
    if (on && !playground.mirror) {
      const m = playground.openMirror($("editor-mirror"));
      m.scrollDOM.addEventListener("scroll", leadColumn);
    }
    if (!on && playground.mirror) playground.unmirror();
    $("stage-fit").classList.toggle("on", fit);
    playground.view.requestMeasure();
    // Once the second view has laid itself out, it starts where the first
    // ends.
    fitFont();
    for (const ms of [250, 600]) window.setTimeout(followColumn, ms);
  };
  // The controls fade when the mouse rests, and come back when it moves.
  const wake = () => {
    document.body.classList.remove("idle");
    window.clearTimeout(idleTimer);
    if (onStage()) idleTimer = window.setTimeout(() => document.body.classList.add("idle"), 2500);
  };
  const enterStage = () => {
    if (!location.hash.startsWith("#/tocar")) location.hash = "#/tocar";
    document.body.classList.add("stage");
    document.documentElement.dataset.theme = "dark";
    setStageFont(stageFont);
    applySplit();
    document.documentElement.requestFullscreen?.().catch(() => {
      // Without full screen (an iframe, an old browser) the stage still works.
    });
    wake();
    playground.view.focus();
  };
  const leaveStage = () => {
    document.body.classList.remove("stage", "stage-map", "idle");
    applySplit();
    window.clearTimeout(idleTimer);
    applyTheme();
    if (document.fullscreenElement) document.exitFullscreen().catch(() => {});
    playground.view.requestMeasure();
  };
  $("stage").onclick = enterStage;
  $("stage-exit").onclick = leaveStage;
  $("stage-split").onclick = () => {
    split = !split;
    save("aratu.stageSplit", split ? "on" : "off");
    applySplit();
  };
  $("stage-fit").onclick = () => {
    fit = !fit;
    save("aratu.stageFit", fit ? "on" : "off");
    $("stage-fit").classList.toggle("on", fit);
    fitFont();
  };
  $("stage-play").onclick = () => void playground.play();
  $("stage-stop").onclick = () => playground.stop();
  $("stage-smaller").onclick = () => setStageFont(stageFont - 4);
  $("stage-bigger").onclick = () => setStageFont(stageFont + 4);
  $("stage-map").onclick = () => {
    document.body.classList.toggle("stage-map");
    $("stage-map").classList.toggle("on", document.body.classList.contains("stage-map"));
    playground.view.requestMeasure();
  };
  document.addEventListener("fullscreenchange", () => {
    if (!document.fullscreenElement && onStage()) leaveStage();
  });
  document.addEventListener("mousemove", wake);
  window.addEventListener(
    "keydown",
    (e) => {
      const digit = /^Digit([1-9])$/.exec(e.code);
      if (e.altKey && !e.ctrlKey && !e.metaKey && digit && location.hash.startsWith("#/tocar")) {
        // Alt+N: melody N of the list on the cursor's line, or of the first.
        const refs = listRefs();
        if (refs.length) {
          const cursor = playground.view.state.selection.main.head;
          const text = playground.text;
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
      if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key.toLowerCase() === "f") {
        e.preventDefault();
        if (onStage()) leaveStage();
        else enterStage();
      } else if (e.key === "Escape" && onStage() && !document.fullscreenElement) {
        leaveStage();
      }
    },
    true,
  );

  $<HTMLSelectElement>("examples").onchange = (e) => {
    const ex = examples.find((x) => x.id === (e.target as HTMLSelectElement).value);
    if (ex) playground.text = ex.text;
  };

  // The book: the course, the manual and the page for developers.
  let book: Book | null = null;
  const loadBook = async () => {
    // Ask the server each time: a cached copy of an older book would send
    // links to pages it does not have.
    const res = await fetch(`book-${lang}.json`, { cache: "no-cache" });
    book = (await res.json()) as Book;
  };
  let pageSessions: Session[] = [];
  const blockContext = {
    aratu,
    engine: getEngine,
    lang: () => lang,
    isDone: (id: string) => done.has(id),
    markDone,
    // Lessons start from eighths, so a beat can be built by clicking.
    quantum: () => (quantum === 1 ? 2 : quantum),
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

  type Kind = "learn" | "manual" | "inside";
  const routeOf: Record<Kind, string> = { learn: "aprender", manual: "manual", inside: "por-dentro" };

  function showPage(kind: Kind, id: string | undefined): void {
    if (!book) return;
    const pages = book[kind] ?? [];
    if (pages.length === 0) return;
    const index = Math.max(0, pages.findIndex((p) => p.id === id));
    const page = pages[index];
    for (const s of pageSessions) s.destroy();
    const article = $(`${kind}-page`);
    article.innerHTML = page.html;
    pageSessions = mountBlocks(article, blockContext);

    const nav = document.createElement("nav");
    nav.className = "page-nav";
    const base = `#/${routeOf[kind]}/`;
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
    const link = document.createElement("p");
    link.className = "page-link";
    const share = document.createElement("a");
    share.href = page.path;
    share.textContent = `🔗 ${tr("pageLink", lang)}`;
    link.append(share);
    article.append(link);
    article.scrollTop = 0;
    window.scrollTo(0, 0);

    if (kind === "learn") {
      currentLearn = page.id;
      save("aratu.lesson", page.id);
      renderLearnToc(page.id);
    } else {
      const ol = $(`${kind}-toc`);
      ol.innerHTML = "";
      for (const p of pages) {
        const li = document.createElement("li");
        const a = document.createElement("a");
        a.href = base + p.id;
        a.textContent = p.title;
        if (p.id === page.id) a.classList.add("here");
        li.append(a);
        ol.append(li);
      }
    }
  }

  const route = () => {
    const [, view = "aprender", id] = location.hash.replace(/^#/, "").split("/");
    const which =
      view === "tocar" ? "play" : view === "manual" ? "manual" : view === "por-dentro" ? "inside" : "learn";
    for (const v of ["play", "learn", "manual", "inside"]) {
      $(`view-${v}`).hidden = v !== which;
      $(`tab-${v}`).classList.toggle("here", v === which);
    }
    if (which !== "play") {
      if (document.body.classList.contains("stage")) $("stage-exit").click();
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
    soundStatus();
    playground.compile();
    if (!$("side-sounds").hidden) sounds.render();
    if (!$("side-rows").hidden) matrix.render(playground.latest ?? null);
    stageLists();
    await loadBook();
    route();
  };
  for (const l of LANGS) $(`lang-${l}`).onclick = () => void setLang(l);
  const setTheme = (t: "light" | "dark") => {
    theme = t;
    save("aratu.theme", theme);
    applyTheme();
  };
  $("theme-light").onclick = () => setTheme("light");
  $("theme-dark").onclick = () => setTheme("dark");

  playground.compile();
  await loadBook();
  window.addEventListener("hashchange", route);
  route();
}

void main();
