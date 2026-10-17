// The code editor: CodeMirror 6 with Aratu's colours, diagnostics in the
// musician's words, and the symbols that are sounding lit up as they play.

import { defaultKeymap, history, historyKeymap, indentWithTab } from "@codemirror/commands";
import { HighlightStyle, StreamLanguage, bracketMatching, syntaxHighlighting } from "@codemirror/language";
import { type Diagnostic as CmDiagnostic, lintGutter, setDiagnostics } from "@codemirror/lint";
import {
  type ChangeSet,
  Compartment,
  EditorState,
  type Extension,
  RangeSetBuilder,
  StateEffect,
  StateField,
} from "@codemirror/state";
import {
  Decoration,
  type DecorationSet,
  EditorView,
  drawSelection,
  highlightActiveLine,
  highlightActiveLineGutter,
  keymap,
  lineNumbers,
} from "@codemirror/view";
import { tags as t } from "@lezer/highlight";
import type { Diagnostic } from "./aratu";

// Folded (lower case, no accents), in English, Portuguese and Spanish, as
// the language reads them. The short Portuguese and Spanish words that are
// also notes or everyday words (`do`, `de`, `a`, `no`, `em`) are left
// plain, so a melody does not light up as keywords.
const KEYWORDS = new Set([
  "meter", "tempo", "key", "pickup", "seed", "form", "pattern", "section", "at", "bar", "bars",
  "every", "first", "last", "play", "add", "once", "now", "rit", "rall", "accel", "to", "over", "primo",
  "out", "in", "cresc", "dim", "fade", "swing", "rubato", "humanize", "random", "beat",
  "fine", "coda", "ms", "hz", "khz", "motif", "row", "list", "from", "octave", "back",
  "loop", "beats", "eighth", "eighths", "sixteenth", "sixteenths", "all",
  "compasso", "compassos", "compas", "compases", "andamento", "pulso", "tonalidade", "tonalidad",
  "anacruse", "anacrusa", "semente", "semilla", "forma", "cada", "primeiro", "primeira", "primero",
  "primera", "ultimo", "ultima", "agora", "ahora", "desde", "ate", "hasta", "durante", "volta",
  "vuelta", "repete", "repite", "toque", "toca", "adicione", "acrescente", "anade", "agrega",
  "padrao", "patron", "secao", "seccion", "motivo", "serie", "lista", "oitava", "octava", "todos",
  "todas", "tudo", "acaso", "aleatorio", "aleatoria", "fora", "fuera", "dentro", "tempos",
  "tiempo", "tiempos", "colcheia", "colcheias", "corchea", "corcheas", "semicolcheia",
  "semicolcheias", "semicorchea", "semicorcheas", "vez", "uma", "una", "com", "con",
  "breakdown", "quebra", "quiebre", "drop", "queda", "caida", "build", "subida", "solo", "off",
  "desliga", "desligado", "apagado", "apaga", "groove", "levada", "ritmo",
  // The arpeggiator, ducking, and the quick transformations.
  "up", "down", "updown", "triplets", "octaves", "sobe", "subindo", "sube", "subiendo", "desce",
  "descendo", "baja", "bajando", "sobedesce", "subeybaja", "tercinas", "quialteras", "tresillos",
  "oitavas", "octavas", "duck", "by", "sidechain", "abaixa", "abaixar", "agacha", "agachar",
  "pelo", "pela", "fast", "slow", "echo", "rapido", "depressa", "deprisa", "lento", "devagar",
  "despacio", "eco",
]);
const fold = (w: string): string =>
  w
    .toLowerCase()
    .normalize("NFD")
    .replace(/\p{M}/gu, "");
const DYNAMICS = new Set(["ppp", "pp", "p", "mp", "mf", "f", "ff", "fff", "sfz", "fp"]);
// The same names the language reads as drum lines and drum parts.
const DRUM_LINES = new Set([
  "hh", "oh", "bd", "sd", "rd", "cr", "cy", "rim", "cp", "tom", "perc", "kick", "snare", "hat",
  "hihat", "ride", "crash", "clap", "ht", "mt", "lt", "ft", "toms", "floor", "stick",
  "chimbal", "bumbo", "caixa", "prato", "pratos", "condução", "conducao", "ataque", "aro",
  "surdo", "tons", "palma", "palmas", "charles", "bombo", "caja", "platillo", "platillos",
  "pandeiro", "pandero", "zabumba", "triangulo", "triângulo", "triangle", "ferrinho", "agogo",
  "agogô", "tamborim", "alfaia", "gongue", "gonguê", "ganza", "ganzá", "agbe", "caxixi", "shaker",
  "chocalho", "cuica", "cuíca", "cajon", "cajón",
]);
const DRUM_PARTS = new Set(["drums", "drum", "kit", "bateria", "batería"]);

interface State {
  lineStart: boolean;
  /** Braces open, and the depth of the drums block we are in (-1: none). */
  depth: number;
  drums: number;
  /** A drums block opens at the next brace. */
  opening: boolean;
  /** This line is struck (drums or strum): one letter is one sound. */
  struck: boolean;
}

const aratuLanguage = StreamLanguage.define<State>({
  name: "aratu",
  // Ctrl+/ comments lines out: the quickest way to silence one line.
  languageData: { commentTokens: { line: "//" } },
  startState: () => ({ lineStart: true, depth: 0, drums: -1, opening: false, struck: false }),
  copyState: (s) => ({ ...s }),
  token(stream, state) {
    if (stream.sol()) {
      state.lineStart = true;
      state.struck = false;
    }
    if (stream.eatSpace()) return null;
    if (stream.match("//")) {
      stream.skipToEnd();
      return "comment";
    }
    if (stream.match(/^(\|:|:\|:?|\|\d|\|\||\|)/)) return "meta";
    if (stream.match(/^(segno|fine|coda):/i) || stream.match(/^(D\.S\.|D\.C\.)/)) return "labelName";
    if (stream.match(/^(ppp|pp|mp|mf|fff|ff|sfz|fp|p|f):/)) return "atom";
    if (stream.match(/^[0-9]+(\.[0-9]+)?(%| ?(Hz|kHz|ms|bpm))?/)) return "number";
    if (stream.match(/^[0-9]+\/[0-9]+/)) return "number";
    if (stream.match(/^->/) || stream.match(/^[=:]/)) return "operator";
    if (stream.match(/^[_\-.]/)) return "punctuation";
    if (stream.match("{")) {
      state.depth++;
      if (state.opening && state.drums < 0) state.drums = state.depth;
      state.opening = false;
      return "bracket";
    }
    if (stream.match("}")) {
      if (state.depth === state.drums) state.drums = -1;
      state.depth = Math.max(0, state.depth - 1);
      return "bracket";
    }
    if (stream.match(/^[[\]()]/)) return "bracket";
    // In a drum or strum line each letter is a sound, and upper case is an
    // accent, drawn so it stands out.
    if (state.struck) {
      const ch = stream.match(/^\p{L}/u) as RegExpMatchArray | null;
      if (ch) return /\p{Lu}/u.test(ch[0]) ? "strong" : "variableName";
    }
    if (stream.match(/^[?'^<>+%~!`]/)) return "operator";
    const word = stream.match(/^[\p{L}_][\p{L}\p{N}_#]*/u) as RegExpMatchArray | null;
    if (word) {
      const w = word[0];
      const first = state.lineStart;
      state.lineStart = false;
      if (KEYWORDS.has(fold(w))) return "keyword";
      if (DYNAMICS.has(w)) return "atom";
      if (first && stream.match(/^\s*(=|\{|:)/, false)) {
        if (stream.match(/^\s*\{/, false) && DRUM_PARTS.has(w)) state.opening = true;
        if (stream.match(/^\s*=/, false)) {
          state.struck =
            state.drums >= 0 ||
            DRUM_LINES.has(w) ||
            ["strum", "batida", "levada", "rasgueo", "rasgueado"].includes(fold(w));
        }
        return "definition";
      }
      if (/^[A-G](#|b)?(m|maj|min|dim|aug|sus|°|º|ø|\d|M)*/.test(w) && /^[A-G]/.test(w)) return "typeName";
      return "variableName";
    }
    stream.next();
    return null;
  },
});

const style = HighlightStyle.define([
  { tag: t.comment, color: "var(--muted)", fontStyle: "italic" },
  { tag: t.keyword, color: "var(--keyword)", fontWeight: "600" },
  { tag: t.atom, color: "var(--dynamic)", fontWeight: "600" },
  { tag: t.number, color: "var(--number)" },
  { tag: t.meta, color: "var(--barline)", fontWeight: "700" },
  { tag: t.labelName, color: "var(--marker)", fontWeight: "600" },
  { tag: t.definition(t.variableName), color: "var(--name)", fontWeight: "600" },
  { tag: t.typeName, color: "var(--chord)" },
  { tag: t.punctuation, color: "var(--hold)" },
  { tag: t.bracket, color: "var(--barline)" },
  { tag: t.operator, color: "var(--operator)" },
  { tag: t.variableName, color: "var(--text)" },
  { tag: t.strong, color: "var(--accent)", fontWeight: "800" },
]);

// Spans that are sounding right now.
const setPlaying = StateEffect.define<{ from: number; to: number }[]>();
const playingField = StateField.define<DecorationSet>({
  create: () => Decoration.none,
  update(deco, tr) {
    deco = deco.map(tr.changes);
    for (const e of tr.effects) {
      if (e.is(setPlaying)) {
        const b = new RangeSetBuilder<Decoration>();
        const spans = [...e.value]
          .filter((s) => s.to > s.from && s.to <= tr.state.doc.length)
          .sort((a, b) => a.from - b.from || a.to - b.to);
        let last = -1;
        for (const s of spans) {
          if (s.from < last) continue;
          b.add(s.from, s.to, Decoration.mark({ class: "cm-playing" }));
          last = s.to;
        }
        deco = b.finish();
      }
    }
    return deco;
  },
  provide: (f) => EditorView.decorations.from(f),
});

export interface EditorHooks {
  change(text: string, changes: ChangeSet): void;
  run(): void;
  stop(): void;
}

/** A phone's width: long lines wrap there, and the stage has one column. */
export function narrowScreen(): boolean {
  return typeof window !== "undefined" && window.matchMedia("(max-width: 720px)").matches;
}

/** Long lines wrapped or not; on in the stage's two columns and on phones. */
const wrapping = new Compartment();

export function setWrapping(view: EditorView, on: boolean): void {
  view.dispatch({ effects: wrapping.reconfigure(on ? EditorView.lineWrapping : []) });
}

export function createEditor(
  parent: HTMLElement,
  doc: string,
  hooks: EditorHooks,
  extra: Extension[] = [],
  compact = false,
): EditorView {
  return new EditorView({
    parent,
    state: EditorState.create({
      doc,
      extensions: [
        wrapping.of(narrowScreen() ? EditorView.lineWrapping : []),
        compact ? [] : [lineNumbers(), highlightActiveLineGutter()],
        history(),
        drawSelection(),
        highlightActiveLine(),
        bracketMatching(),
        aratuLanguage,
        syntaxHighlighting(style),
        compact ? [] : lintGutter(),
        playingField,
        keymap.of([
          { key: "Mod-Enter", run: () => (hooks.run(), true) },
          { key: "Mod-.", run: () => (hooks.stop(), true) },
          { key: "Alt-m", run: (v) => toggleCommand(v, "mute") },
          { key: "Alt-s", run: (v) => toggleCommand(v, "solo") },
          indentWithTab,
          ...defaultKeymap,
          ...historyKeymap,
        ]),
        EditorView.updateListener.of((u) => {
          if (u.docChanged) hooks.change(u.state.doc.toString(), u.changes);
        }),
        ...extra,
      ],
    }),
  });
}

// Silencing and soloing from the keyboard while playing: Alt+M writes
// `part: out` for the part of the line the cursor is on, or takes it away;
// Alt+S writes `solo part`, or takes it away. The text says what happens,
// so the change comes in at the next bar line like any other, and a shared
// link keeps it.

const SECTION_WORDS = new Set(["section", "secao", "seccion", "pattern", "padrao", "patron", "list", "lista"]);

/** The part a line of the text belongs to: the line's own name, the part
 * whose block it is written in, or the drum part for a drum line. */
export function partAt(text: string, pos: number): string | null {
  const lineStart = text.lastIndexOf("\n", pos - 1) + 1;
  const next = text.indexOf("\n", pos);
  const line = text.slice(lineStart, next === -1 ? text.length : next);
  // The blocks open where the line starts; `section verse {` is a section,
  // not a part.
  const blocks: { name: string; part: boolean }[] = [];
  for (const m of text.slice(0, lineStart).matchAll(/(?:([\p{L}\w-]+)\s+)?([\p{L}\w.-]+)\s*\{|\}/gu)) {
    if (m[0] === "}") blocks.pop();
    else blocks.push({ name: m[2], part: !(m[1] && SECTION_WORDS.has(fold(m[1]))) });
  }
  const enclosing = [...blocks].reverse().find((b) => b.part)?.name ?? null;
  // A part's block opened on this very line: `drums { hh = … }`.
  const own = /^\s*(?:([\p{L}\w-]+)\s+)?([\p{L}\w-]+)\s*\{/u.exec(line);
  if (own && !(own[1] && SECTION_WORDS.has(fold(own[1])))) return own[2];
  const head = /^\s*([\p{L}\w-]+)\s*[=:.]/u.exec(line);
  const word = head ? head[1] : null;
  if (word && DRUM_LINES.has(word.toLowerCase())) return enclosing ?? "drums";
  if (enclosing) return enclosing;
  if (!word || KEYWORDS.has(fold(word))) return null;
  return word;
}

/** Writes or takes away `part: out` (mute) or `solo part` (solo) for the
 * part of the line the cursor is on. */
function toggleCommand(view: EditorView, what: "mute" | "solo"): boolean {
  const text = view.state.doc.toString();
  const part = partAt(text, view.state.selection.main.head);
  if (!part) return false;
  const esc = part.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const mine =
    what === "mute"
      ? new RegExp(`^${esc}\\s*:\\s*(out|fora|fuera|sai|sale)\\s*$`, "imu")
      : new RegExp(`^solo\\s+${esc}\\s*$`, "imu");
  // A line taken away goes with the line break before it.
  const without = (index: number, length: number) => ({
    from: Math.max(0, index - (text[index - 1] === "\n" ? 1 : 0)),
    to: index + length,
  });
  const changes: { from: number; to: number; insert?: string }[] = [];
  const found = mine.exec(text);
  if (found) {
    changes.push(without(found.index, found[0].length));
  } else {
    // One solo at a time: another part's goes.
    if (what === "solo") {
      for (const m of text.matchAll(/^solo\s+\S+\s*$/gimu)) changes.push(without(m.index, m[0].length));
    }
    const sep = text.length === 0 || text.endsWith("\n") ? "" : "\n";
    const line = what === "mute" ? `${part}: out` : `solo ${part}`;
    changes.push({ from: text.length, to: text.length, insert: sep + line });
  }
  view.dispatch({ changes });
  return true;
}

export function showDiagnostics(view: EditorView, list: Diagnostic[]): void {
  const len = view.state.doc.length;
  const cm: CmDiagnostic[] = list.map((d) => ({
    from: Math.min(d.from, len),
    to: Math.min(Math.max(d.to, d.from), len),
    severity: d.severity,
    message: d.message,
  }));
  view.dispatch(setDiagnostics(view.state, cm));
}

export function showPlaying(view: EditorView, spans: { from: number; to: number }[]): void {
  view.dispatch({ effects: setPlaying.of(spans) });
}
