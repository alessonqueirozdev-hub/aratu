// Types for Manguebyte's js/engine.js, the part of its API Aratu uses.
// Copied next to engine.js by scripts/manguebyte.mjs.

export type Instrument = "synth" | "drums" | "sampler" | "sampler_kit";

export interface Latency {
  engineFrames: number;
  outputFrames: number;
  frames: number;
  seconds: number;
}

export interface SfzSummary {
  regions: number;
  samples: number;
  sample_bytes: number;
  warnings: string[];
}

export declare const DRUM_NOTES: Readonly<Record<string, number>>;

export declare class ManguebyteEngine {
  static create(
    context: BaseAudioContext,
    options: { wasm: string | URL | ArrayBuffer | WebAssembly.Module; workletUrl?: string | URL; shared?: boolean },
  ): Promise<ManguebyteEngine>;
  readonly node: AudioWorkletNode;
  readonly mode: "shared" | "message";
  noteOn(part: number, note: number, velocity?: number, time?: number, channel?: number, accent?: boolean): boolean;
  noteOff(part: number, note: number, time?: number, channel?: number): boolean;
  setParam(part: number, id: string | number, value: number, time?: number): boolean;
  allNotesOff(part: number, time?: number): boolean;
  choke(part: number, time?: number): boolean;
  cancel(part: number, time?: number): boolean;
  latency(): Promise<Latency>;
  createPart(part: number, kind?: Instrument, chain?: "full" | "none" | "lowpass" | "reverb" | number): Promise<void>;
  loadSfz(part: number, url: string, options?: { base?: string }): Promise<SfzSummary>;
  removePart(part: number): Promise<void>;
  close(): Promise<void>;
}
