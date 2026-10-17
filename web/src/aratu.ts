// The Aratu language, compiled to WebAssembly, behind a small typed API.

export interface Diagnostic {
  severity: "error" | "warning";
  /** UTF-16 offsets in the source, as the editor counts them. */
  from: number;
  to: number;
  line: number;
  message: string;
}

export interface ParamValue {
  id: string;
  value: number;
}

export interface PartInfo {
  name: string;
  /** "drums", "guitar", "keys" or "synth": the default sound. */
  instrument: string;
  chain: string[];
  params: ParamValue[];
}

/** A number in the text drawn as a knob. */
export interface Control {
  id: string;
  /** The part it sets, "all" for the whole mix, "" for the tempo. */
  part: string;
  /** Where the number is written. */
  from: number;
  to: number;
  /** The value in the parameter's own unit (Hz, ms, 0 to 1, bpm). */
  value: number;
  /** The written number per unit of value (0.001 for kHz, 100 for %). */
  factor: number;
  min: number;
  max: number;
  unit: string;
  log: boolean;
  /** Set at a bar: turning it changes the text, not the sound right away. */
  timed: boolean;
  /** Whole numbers only: a choice (a wave, a shape) or a count. */
  steps: boolean;
}

export interface Program {
  handle: number;
  ok: boolean;
  diagnostics: Diagnostic[];
  /** Lines written without a time, which sound from when they appear. */
  live: { key: string; now: boolean }[];
  tempo: number;
  meter: string;
  pickup: number;
  /** Bars in one time round of the longest line. */
  length: number;
  /** Melody lists (`list temas { … }`), with how many melodies each has. */
  lists: { name: string; count: number }[];
  /** Twelve-tone rows (`row serie = …`), as MIDI notes. */
  rows: { name: string; pitches: number[] }[];
  parts: PartInfo[];
  params: ParamValue[];
  /** Parameters that move (`-> 4000 Hz over 4 bars`), by part ("all" for
   * the whole mix). */
  moving: { part: string; id: string }[];
  controls: Control[];
}

export interface AratuEvent {
  bar: number;
  beat: number;
  /** Seconds from the start of the piece. */
  time: number;
  dur: number;
  part: string;
  /** The line that played it: `bass`, `drums.hh`. */
  line: string;
  vel: number;
  /** 1 when written in upper case. */
  acc?: number;
  note?: number;
  drum?: string;
  param?: string;
  value?: number;
  /** For a parameter that moves: seconds to glide to `value` from the
   * point before. */
  glide?: number;
  /** 1 when `from bar 9 to 16` is over and the parameter goes back to the
   * value its part is set up with. */
  reset?: number;
  from: number;
  to: number;
  /** For a strum: where the chord it plays is written. */
  cfrom?: number;
  cto?: number;
}

export interface BarInfo {
  bar: number;
  start: number;
  end: number;
  beats: number;
  bpm: number;
  meter: string;
  /** When each whole beat starts, in seconds from the start of the piece. */
  beatTimes: number[];
}

export interface Rendered {
  events: AratuEvent[];
  bars: BarInfo[];
}

/** The answer of a check: whether the student's piece reads, and what
 * differs from the answer, in words. */
export interface Checked {
  compiles: boolean;
  ok: boolean;
  problems: string[];
}

export type Lang = "pt" | "es" | "en";

/** A step of the rhythm map: 0 rest, 1 the sound goes on, or a sound that
 * starts here: where it is written, how many steps it lasts, and for notes
 * the text with explicit octaves. */
export type GridStep = 0 | 1 | [number, number, number] | [number, number, number, string];

export interface GridBar {
  /** Place in one time round: 0 the pickup, then 1, 2... */
  pos: number;
  /** The written beats of the bar, to rewrite (-1 when it cannot be). */
  from: number;
  to: number;
  beats: number;
  /** Steps per beat. */
  res: number;
  editable: boolean;
  steps: GridStep[];
}

export interface GridLine {
  label: string;
  kind: "drums" | "strum" | "pitched" | "chords";
  bars: GridBar[];
}

const LANG_CODE: Record<Lang, number> = { pt: 0, en: 1, es: 2 };

interface Exports {
  memory: WebAssembly.Memory;
  aratu_alloc(len: number): number;
  aratu_free(ptr: number, len: number): void;
  aratu_compile(ptr: number, len: number, lang: number): number;
  aratu_render(
    handle: number,
    from: number,
    fromBeat: number,
    to: number,
    defaultAnchor: number,
    seed: number,
    anchorsPtr: number,
    anchorsLen: number,
  ): void;
  aratu_check(
    studentPtr: number,
    studentLen: number,
    answerPtr: number,
    answerLen: number,
    bars: number,
    flags: number,
    lang: number,
  ): void;
  aratu_grid(handle: number, maxBars: number): void;
  aratu_voices(lang: number): void;
  aratu_release(handle: number): void;
  aratu_out_ptr(): number;
  aratu_out_len(): number;
}

const encoder = new TextEncoder();
const decoder = new TextDecoder();

export class Aratu {
  private constructor(private readonly x: Exports) {}

  static async load(url: string): Promise<Aratu> {
    const response = await fetch(url);
    const { instance } = await WebAssembly.instantiate(await response.arrayBuffer(), {});
    return new Aratu(instance.exports as unknown as Exports);
  }

  private withText<T>(text: string, f: (ptr: number, len: number) => T): T {
    const bytes = encoder.encode(text);
    const len = bytes.length;
    const ptr = this.x.aratu_alloc(len);
    new Uint8Array(this.x.memory.buffer, ptr, len).set(bytes);
    try {
      return f(ptr, len);
    } finally {
      this.x.aratu_free(ptr, len);
    }
  }

  private output(): string {
    const bytes = new Uint8Array(this.x.memory.buffer, this.x.aratu_out_ptr(), this.x.aratu_out_len());
    return decoder.decode(bytes);
  }

  compile(src: string, lang: Lang): Program {
    this.withText(src, (p, n) => this.x.aratu_compile(p, n, LANG_CODE[lang]));
    return JSON.parse(this.output()) as Program;
  }

  /** Renders bars `from..to`; bar `from` from its beat `fromBeat` on. */
  render(
    handle: number,
    from: number,
    to: number,
    anchors: Map<string, number>,
    seed: number,
    fromBeat = 0,
  ): Rendered {
    let text = "";
    for (const [key, bar] of anchors) text += `${bar}\t${key}\n`;
    this.withText(text, (p, n) => this.x.aratu_render(handle, from, fromBeat, to, 1, seed >>> 0, p, n));
    return JSON.parse(this.output()) as Rendered;
  }

  /** Compares a student's piece with the answer by what they play. `bars`
   * 0 compares one time round of the answer. */
  check(student: string, answer: string, lang: Lang, bars = 0, dynamics = false, lengths = true): Checked {
    const flags = (dynamics ? 1 : 0) | (lengths ? 0 : 2);
    this.withText(student, (sp, sn) =>
      this.withText(answer, (ap, an) => this.x.aratu_check(sp, sn, ap, an, bars, flags, LANG_CODE[lang])),
    );
    return JSON.parse(this.output()) as Checked;
  }

  /** The rhythm map of a program: every line's bars as steps. */
  grid(handle: number, maxBars: number): { lines: GridLine[] } {
    this.x.aratu_grid(handle, maxBars);
    return JSON.parse(this.output()) as { lines: GridLine[] };
  }

  /** Every voice a part can play, by family, named in `lang`. */
  voices(lang: Lang): VoiceCatalogue {
    this.x.aratu_voices(LANG_CODE[lang]);
    return JSON.parse(this.output()) as VoiceCatalogue;
  }

  release(handle: number): void {
    this.x.aratu_release(handle);
  }
}

export interface VoiceInfo {
  code: string;
  name: string;
  /** Recorded (fetched the first time it is used), not synthesized. */
  sampled: boolean;
  /** One bar to hear it. */
  demo: string;
}

export interface VoiceCatalogue {
  families: { code: string; name: string; voices: VoiceInfo[] }[];
  /** Short names kept from before the catalogue, and what they play. */
  aliases: Record<string, string>;
}
