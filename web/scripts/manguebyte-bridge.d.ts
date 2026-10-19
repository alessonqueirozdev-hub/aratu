// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
// Types for Manguebyte's js/bridge.js, the part of its API Aratu uses.
// Copied next to bridge.js by scripts/manguebyte.mjs.

import { ManguebyteEngine } from "./engine.js";

/** The DAW's transport, as the plugin last reported it. */
export interface Transport {
  /** Where the DAW is, in quarter notes. */
  beat: number;
  tempo: number;
  /** The beat the bar the DAW is in starts on. */
  barStart: number;
  loopStart: number;
  loopEnd: number;
  epoch: number;
  /** [beats in a bar, the note that is a beat]. */
  timeSignature: [number, number];
  playing: boolean;
  recording: boolean;
  /** Added by the client: the DAW jumped, or its loop went round. */
  jumped?: boolean;
  wrapped?: boolean;
}

export declare const DEFAULT_PORT: number;

export declare class AratuBridge extends ManguebyteEngine {
  static connect(options?: {
    port?: number;
    url?: string;
    timeBase?: "seconds" | "beats";
    name?: string;
    /** Seconds, for "seconds" mode: the clock times are given on. */
    clock?: () => number;
  }): Promise<AratuBridge>;
  readonly state: "connecting" | "open" | "closed" | "error";
  readonly transport: Transport | null;
  /** The beat the DAW is on now, estimated from its last transport. */
  readonly currentBeat: number;
  /** The beat a time on the clock falls on, at the DAW's tempo. */
  beatAt(time: number): number | null;
  /** The time on the clock the DAW reaches `beat`, if it plays. */
  timeAtBeat(beat: number): number | null;
  onstate: ((state: AratuBridge["state"]) => void) | null;
  ontransport: ((t: Transport) => void) | null;
  onnotice: ((n: { code: number; text: string }) => void) | null;
}
