// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
// Sampled voices: recordings from the Versilian Community Sample Library
// (CC0), prepared by scripts/samples.mjs into samples/<voice>/, and kits
// recorded for Aratu, kept beside them. The stand-in fetches a voice when
// Manguebyte cannot play it; the voice sounds once it has arrived, and only
// if it cannot be downloaded does a synthesized voice like it play instead.

import { detailOf, limited } from "./download";

interface PitchedIndex {
  kind: "pitched";
  notes: { note: number; file: string; loop?: [number, number] }[];
}

interface KitIndex {
  kind: "kit";
  hits: Record<string, string>;
}

interface Zone {
  note: number;
  buffer: AudioBuffer;
  loop?: [number, number];
}

export interface Loaded {
  kind: "pitched" | "kit";
  /** Pitched: the recordings by root note, low to high. */
  zones: Zone[];
  /** Kit: one recording per drum name. */
  hits: Map<string, AudioBuffer>;
}

export class Sampler {
  private readonly loaded = new Map<string, Loaded>();
  private readonly pending = new Map<string, Promise<Loaded | null>>();
  /** Why a voice could not be downloaded (see `detailOf`). */
  private readonly errors = new Map<string, string>();
  /** Told when a voice arrives or fails, for the Sounds panel. */
  onChange: (code: string, state: "loading" | "ready" | "failed") => void = () => {};

  constructor(
    private readonly ctx: BaseAudioContext,
    private readonly base = "samples",
  ) {}

  get(code: string): Loaded | undefined {
    return this.loaded.get(code);
  }

  state(code: string): "idle" | "loading" | "ready" | "failed" {
    if (this.loaded.has(code)) return "ready";
    if (this.pending.has(code)) return "loading";
    if (this.errors.has(code)) return "failed";
    return "idle";
  }

  /** Why a voice could not be downloaded, if it could not. */
  error(code: string): string | undefined {
    return this.errors.get(code);
  }

  /** Starts fetching a voice; resolves when it can play. */
  load(code: string): Promise<Loaded | null> {
    const done = this.loaded.get(code);
    if (done) return Promise.resolve(done);
    let p = this.pending.get(code);
    if (p) return p;
    this.errors.delete(code);
    this.onChange(code, "loading");
    p = limited(this.fetchVoice(code)).then(
      (v) => {
        this.loaded.set(code, v);
        this.pending.delete(code);
        this.onChange(code, "ready");
        return v;
      },
      (error: unknown) => {
        this.pending.delete(code);
        this.errors.set(code, detailOf(error));
        this.onChange(code, "failed");
        return null;
      },
    );
    this.pending.set(code, p);
    return p;
  }

  private async buffer(url: string): Promise<AudioBuffer> {
    const r = await fetch(url);
    if (!r.ok) throw new Error(`${url}: ${r.status}`);
    return this.ctx.decodeAudioData(await r.arrayBuffer());
  }

  private async fetchVoice(code: string): Promise<Loaded> {
    const dir = `${this.base}/${code}`;
    const r = await fetch(`${dir}/index.json`);
    if (!r.ok) throw new Error(`${code}: ${r.status}`);
    const index = (await r.json()) as PitchedIndex | KitIndex;
    if (index.kind === "kit") {
      const hits = new Map<string, AudioBuffer>();
      await Promise.all(
        Object.entries(index.hits).map(async ([name, file]) => hits.set(name, await this.buffer(`${dir}/${file}`))),
      );
      return { kind: "kit", zones: [], hits };
    }
    const zones = await Promise.all(
      index.notes.map(async (n) => ({ note: n.note, loop: n.loop, buffer: await this.buffer(`${dir}/${n.file}`) })),
    );
    zones.sort((a, b) => a.note - b.note);
    return { kind: "pitched", zones, hits: new Map() };
  }

  /** The recording nearest to a note, and how far to retune it. */
  static zoneFor(v: Loaded, note: number): { zone: Zone; rate: number } | null {
    if (v.zones.length === 0) return null;
    let best = v.zones[0];
    for (const z of v.zones) if (Math.abs(z.note - note) < Math.abs(best.note - note)) best = z;
    return { zone: best, rate: Math.pow(2, (note - best.note) / 12) };
  }
}
