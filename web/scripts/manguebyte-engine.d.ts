// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
// Types for Manguebyte's js/engine.js, the part of its API Aratu uses.
// Copied next to engine.js by scripts/manguebyte.mjs.

export type Instrument =
  | "synth"
  | "drums"
  | "sampler"
  | "sampler_kit"
  | "plucked"
  | "fm"
  | "organ"
  | "analog_kit"
  | "kick_bass"
  | "acid"
  | "drum_kit"
  | "poly"
  | "mallet"
  | "bowed"
  | "wind"
  | "choir";

/** The instruments that have presets (`<kind>.preset`). */
export type PresetKind = "plucked" | "fm" | "organ" | "acid" | "drum_kit" | "poly" | "mallet" | "bowed" | "wind" | "choir";

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

/** An effect's settings: by name, in its parameters' order, its first
 * parameter, `{}` as it is set, or `false` to switch it off. */
export type EffectSettings = Record<string, number | string> | (number | string)[] | number | false;

export declare const DRUM_NOTES: Readonly<Record<string, number>>;
export declare const PRESETS: Readonly<Record<PresetKind, readonly string[]>>;
export declare const EFFECT_ORDER: readonly string[];

/** The latest frames a part put into the master bus, after its effects,
 * gain and pan: `frame` is the engine frame of the first sample, `time`
 * its AudioContext time. */
export interface PartScope {
  frame: number;
  time: number;
  left: Float32Array;
  right: Float32Array;
}

/** A part's levels, linear, 1 at full scale, [left, right]. */
export interface PartLevels {
  part: number;
  frame: number;
  peak: [number, number];
  rms: [number, number];
}

export declare class ManguebyteEngine {
  static create(
    context: BaseAudioContext,
    options: {
      wasm: string | URL | ArrayBuffer | WebAssembly.Module;
      workletUrl?: string | URL;
      shared?: boolean;
      scopes?: boolean;
    },
  ): Promise<ManguebyteEngine>;
  readonly node: AudioWorkletNode;
  readonly mode: "shared" | "message";
  /** How partScope and partLevels read: null for a module without them. */
  readonly scopeMode: "shared" | "message" | null;
  noteOn(part: number, note: number, velocity?: number, time?: number, channel?: number, accent?: boolean): boolean;
  noteOff(part: number, note: number, time?: number, channel?: number): boolean;
  setParam(part: number, id: string | number, value: number, time?: number): boolean;
  allNotesOff(part: number, time?: number): boolean;
  choke(part: number, time?: number): boolean;
  cancel(part: number, time?: number): boolean;
  latency(): Promise<Latency>;
  createPart(part: number, kind?: Instrument, chain?: "full" | "none" | "lowpass" | "reverb" | number): Promise<void>;
  setInstrument(part: number, kind: Instrument): Promise<void>;
  setPreset(part: number, kind: PresetKind, preset: string | number, time?: number): boolean;
  setRotary(part: number, speed: "stop" | "slow" | "fast" | number, time?: number): boolean;
  setEffect(part: number, name: string, settings?: EffectSettings, time?: number): Promise<void>;
  setEffects(part: number, effects: Record<string, EffectSettings>, time?: number): Promise<void>;
  removeEffect(part: number, name: string, time?: number): void;
  setSend(part: number, amount: number, time?: number): boolean;
  setMaster(settings: Record<string, unknown>, time?: number): void;
  loadSfz(part: number, url: string, options?: { base?: string }): Promise<SfzSummary>;
  removePart(part: number): Promise<void>;
  limiterReduction(): Promise<number>;
  /** The newest `frames` frames of a part (at most SCOPE_FRAMES), or null
   * when there is no such part. */
  partScope(part: number, frames?: number): Promise<PartScope | null>;
  partLevels(part: number): Promise<PartLevels | null>;
  partLevels(): Promise<PartLevels[]>;
  close(): Promise<void>;
}
