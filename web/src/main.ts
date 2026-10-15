// The Aratu editor: write, hear, change while it plays.

import { ChangeSet } from "@codemirror/state";
import { Aratu, type Program } from "./aratu";
import { Engine } from "./audio";
import { createEditor, showDiagnostics, showPlaying } from "./editor";
import { examples } from "./examples";
import { knobs, showKnobs } from "./knobs";
import { type Lang, isWord, tr } from "./i18n";
import { Player, type Scheduled } from "./player";

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
    // Private windows may refuse storage; the editor works without it.
  }
}

async function main(): Promise<void> {
  let lang: Lang = (load("aratu.lang") as Lang | null) ?? (navigator.language.startsWith("pt") ? "pt" : "en");
  const applyWords = () => {
    document.documentElement.lang = lang === "pt" ? "pt-BR" : "en";
    for (const el of document.querySelectorAll<HTMLElement>("[data-word]")) {
      const w = el.dataset.word ?? "";
      if (isWord(w)) el.textContent = tr(w, lang);
    }
    $("lang-pt").classList.toggle("on", lang === "pt");
    $("lang-en").classList.toggle("on", lang === "en");
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
  };
  applyWords();
  $("note").textContent = tr("loading", lang);

  const aratu = await Aratu.load("aratu.wasm");
  let engine: Engine | null = null;
  let player: Player | null = null;

  /** For each program, the edits made since it was compiled, so that the
   * positions of its events can be followed into the current text. */
  const editsSince = new Map<number, ChangeSet>();
  /** The last program without errors that is not yet with the player. */
  let ready: Program | null = null;
  /** The last program with errors, released when the next one comes. */
  let broken: Program | null = null;
  let scheduled: Scheduled[] = [];

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
        view.dispatch({ selection: { anchor: d.from, head: d.to }, scrollIntoView: true });
        view.focus();
      };
      ul.append(li);
    }
    const playing = player?.playing ?? false;
    $("note").textContent = p.ok ? "" : playing ? tr("keepsPlaying", lang) : tr("fixFirst", lang);
  };

  const compile = (): Program => {
    const p = aratu.compile(view.state.doc.toString(), lang);
    editsSince.set(p.handle, ChangeSet.empty(view.state.doc.length));
    // Only the last few programs can still have events on the way.
    for (const h of editsSince.keys()) {
      if (editsSince.size <= 8) break;
      editsSince.delete(h);
    }
    if (broken) aratu.release(broken.handle);
    broken = null;
    showDiagnostics(view, p.diagnostics);
    showKnobs(view, p.controls, (control, value) => {
      // Parameters set at a bar, and the tempo, come in with the text.
      if (control.timed || control.id === "tempo") return;
      engine?.setNow(control.part, control.id, value);
    });
    messages(p);
    if (!p.ok) {
      broken = p;
      return p;
    }
    if (player?.playing) {
      player.update(p);
    } else {
      if (ready) aratu.release(ready.handle);
      ready = p;
    }
    return p;
  };

  let timer: number | undefined;
  const view = createEditor($("editor"), load("aratu.text") ?? examples[0].text, {
    change(text, changes) {
      for (const [h, cs] of editsSince) editsSince.set(h, cs.compose(changes));
      for (const s of scheduled) {
        s.event.from = changes.mapPos(s.event.from, 1);
        s.event.to = changes.mapPos(s.event.to, -1);
      }
      save("aratu.text", text);
      window.clearTimeout(timer);
      timer = window.setTimeout(compile, 250);
    },
    run: () => void play(),
    stop: () => stop(),
  }, knobs);

  const play = async () => {
    window.clearTimeout(timer);
    const p = compile();
    if (player?.playing) return;
    if (!p.ok) return;
    if (!engine) engine = new Engine();
    if (!player) {
      player = new Player(aratu, engine, {
        bar(info, at) {
          const ctx = engine!.ctx;
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
        events(list) {
          for (const s of list) {
            const cs = editsSince.get(s.handle);
            if (cs && !cs.empty) {
              s.event.from = cs.mapPos(s.event.from, 1);
              s.event.to = cs.mapPos(s.event.to, -1);
            }
          }
          scheduled.push(...list);
        },
        swapped(_program, bar) {
          $("note").textContent = `${tr("next", lang)} ${bar}.`;
          window.setTimeout(() => ($("note").textContent = ""), 3000);
        },
      });
    }
    ready = null;
    await player.play(p);
    $("play").classList.add("on");
  };

  const stop = () => {
    player?.stop();
    scheduled = [];
    showPlaying(view, []);
    $("play").classList.remove("on");
    $("beat").classList.remove("on");
  };

  // Light up what is sounding.
  let shown = "";
  const frame = () => {
    if (engine && player?.playing) {
      const now = engine.ctx.currentTime;
      scheduled = scheduled.filter((s) => s.at + Math.max(s.event.dur, 0.12) > now - 0.05);
      const spans = scheduled
        .filter((s) => s.at <= now)
        .map((s) => ({ from: s.event.from, to: s.event.to }));
      const key = spans.map((s) => `${s.from}:${s.to}`).join(",");
      if (key !== shown) {
        shown = key;
        showPlaying(view, spans);
      }
    }
    requestAnimationFrame(frame);
  };
  requestAnimationFrame(frame);

  $("play").onclick = () => void play();
  $("stop").onclick = stop;
  $<HTMLSelectElement>("examples").onchange = (e) => {
    const id = (e.target as HTMLSelectElement).value;
    const ex = examples.find((x) => x.id === id);
    if (!ex) return;
    view.dispatch({ changes: { from: 0, to: view.state.doc.length, insert: ex.text } });
    window.clearTimeout(timer);
    compile();
  };
  const setLang = (l: Lang) => {
    lang = l;
    save("aratu.lang", l);
    applyWords();
    compile();
  };
  $("lang-pt").onclick = () => setLang("pt");
  $("lang-en").onclick = () => setLang("en");

  compile();
}

void main();
