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
  part: string;
  from: number;
  to: number;
  value: number;
  min?: number;
  max?: number;
  unit?: string;
  log?: boolean;
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
  parts: PartInfo[];
  params: ParamValue[];
  controls: Control[];
}

export interface AratuEvent {
  bar: number;
  beat: number;
  /** Seconds from the start of the piece. */
  time: number;
  dur: number;
  part: string;
  vel: number;
  note?: number;
  drum?: string;
  param?: string;
  value?: number;
  from: number;
  to: number;
}

export interface BarInfo {
  bar: number;
  start: number;
  end: number;
  beats: number;
  bpm: number;
  meter: string;
}

export interface Rendered {
  events: AratuEvent[];
  bars: BarInfo[];
}

interface Exports {
  memory: WebAssembly.Memory;
  aratu_alloc(len: number): number;
  aratu_free(ptr: number, len: number): void;
  aratu_compile(ptr: number, len: number, lang: number): number;
  aratu_render(
    handle: number,
    from: number,
    to: number,
    defaultAnchor: number,
    seed: number,
    anchorsPtr: number,
    anchorsLen: number,
  ): void;
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

  compile(src: string, lang: "pt" | "en"): Program {
    this.withText(src, (p, n) => this.x.aratu_compile(p, n, lang === "en" ? 1 : 0));
    return JSON.parse(this.output()) as Program;
  }

  render(handle: number, from: number, to: number, anchors: Map<string, number>, seed: number): Rendered {
    let text = "";
    for (const [key, bar] of anchors) text += `${bar}\t${key}\n`;
    this.withText(text, (p, n) => this.x.aratu_render(handle, from, to, 1, seed >>> 0, p, n));
    return JSON.parse(this.output()) as Rendered;
  }

  release(handle: number): void {
    this.x.aratu_release(handle);
  }
}
