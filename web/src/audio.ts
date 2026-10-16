// Built-in sounds on the Web Audio API, standing in until Manguebyte runs
// in the browser: synthesized drum kits, synths, keyboards and plucked
// strings (Karplus-Strong, rendered once per note into a buffer), and
// sampled voices (VCSL, CC0) fetched when a piece first uses them. Each part
// has its own chain: high-pass, low-pass, volume, pan, and sends to a delay
// and a reverb. A part's voice is the code at the head of its chain
// (`bass: bass.slap -> reverb`), from the catalogue in the crate.

import type { AratuEvent, ParamValue, PartInfo, Program, VoiceCatalogue } from "./aratu";
import { ManguebyteEngine } from "../public/manguebyte/engine.js";
import { type Loaded, Sampler } from "./sampler";

/** Manguebyte's drum notes (General MIDI, and 88 for the golpe), by the
 * names Aratu's events give drums. */
const MB_DRUMS: Record<string, number> = {
  kick: 36,
  snare: 38,
  "side-stick": 37,
  "closed-hat": 42,
  "open-hat": 46,
  ride: 51,
  crash: 49,
  "high-tom": 50,
  "mid-tom": 47,
  "floor-tom": 43,
  "guitar-tap": 88,
};

/** How much a part Manguebyte plays is raised or lowered to sit with the
 * others, measured in the browser with each voice's bar in the Sounds
 * panel: a sampled instrument peaks near 0.3 like the stand-in's voices, a
 * kit near 0.72 like the stand-in's kit. */
const MB_LEVEL: Record<string, number> = { sampler: 1.6, sampler_kit: 1.45, drums: 0.75 };

const mtof = (n: number) => 440 * Math.pow(2, (n - 69) / 12);

/** Each voice's level, measured in the browser so that one line of any
 * voice peaks near the same height (about 0.3 at mf). */
const VOICE_GAIN: Record<string, number> = {
  "bass.slap": 0.3,
  "bass.808": 0.5,
  "bass.finger": 0.85,
  "bass.synth": 1.4,
  "mallets.vibraphone": 2,
  "piano.upright": 1.6,
  "organ.pipe": 1.8,
  "sax.tenor": 1.7,
  harp: 1.4,
  "bells.tubular": 1.2,
  "recorder.alto": 1.3,
  "keys.harpsichord": 1.3,
  "keys.organ": 1.6,
  "keys.bell": 1.3,
  "synth.brass": 3.5,
  "synth.saw": 2,
  "synth.square": 2,
  "synth.strings": 2.5,
  "synth.supersaw": 2.7,
  "synth.pad": 2,
  "synth.lead": 1.8,
};

/** What a sampled voice plays while its recordings are on their way. */
const STAND_IN: Record<string, string> = {
  "drums.acoustic": "drums.kit",
  "piano.grand": "keys.epiano",
  "piano.upright": "keys.epiano",
  "keys.harpsichord": "guitar.steel",
  "organ.pipe": "keys.organ",
  harp: "guitar.nylon",
  "mallets.marimba": "keys.bell",
  "mallets.vibraphone": "keys.bell",
  "mallets.xylophone": "keys.bell",
  "mallets.glockenspiel": "keys.bell",
  "mallets.kalimba": "keys.bell",
  "bells.tubular": "keys.bell",
  "sax.tenor": "synth.brass",
  harmonica: "synth.lead",
  "recorder.alto": "synth.sine",
};

/** One sounding source and the gain that shapes it. */
interface SourceVoice {
  src: AudioScheduledSourceNode;
  gain: GainNode;
}

/** What `play` made, for `cancel`: Web Audio sources, or notes sent to a
 * part's Manguebyte engine (which cancels them itself). */
export type Voice = SourceVoice | { mb: ManguebyteEngine };

/** A part that Manguebyte's engine plays: its synthesized kit, a sampled
 * kit, or a sampled instrument. One engine per part, its output going into
 * the part's chain, so Aratu's effects still apply to it. */
interface MbPart {
  voice: string;
  kind: "drums" | "sampler" | "sampler_kit";
  engine: ManguebyteEngine | null;
  /** Plays what is asked of it: the kit at once, a sampler once loaded. */
  ready: boolean;
  failed: boolean;
}

interface Chain {
  input: GainNode;
  highpass: BiquadFilterNode;
  lowpass: BiquadFilterNode;
  volume: GainNode;
  pan: StereoPannerNode;
  delaySend: GainNode;
  delay: DelayNode;
  feedback: GainNode;
  reverbSend: GainNode;
  instrument: string;
  /** The voice's code: `bass.slap`, `piano.grand`. */
  voice: string;
  attack: number;
  release: number;
  /** The value each parameter is set up with, for `from bar 9 to 16` to
   * come back to. */
  configured: Map<string, number>;
  /** Set when Manguebyte's engine plays this part. */
  mb?: MbPart;
}

export class Engine {
  readonly ctx: AudioContext;
  private readonly master: GainNode;
  private readonly reverb: ConvolverNode;
  private readonly noise: AudioBuffer;
  private readonly chains = new Map<string, Chain>();
  private readonly plucks = new Map<number, AudioBuffer>();
  private readonly voices = new Set<SourceVoice>();
  /** The open hi-hat ringing in each part, for a closed one to choke. */
  private readonly openHats = new Map<Chain, SourceVoice>();
  /** Manguebyte's module, compiled once for every part's engine; null when
   * it cannot run here, and the stand-in plays everything. */
  private mbModule: Promise<WebAssembly.Module | null> | null = null;
  /** Where each sampled voice played by Manguebyte stands, for the panel. */
  private readonly mbStates = new Map<string, "loading" | "ready" | "failed">();
  private notify: (code: string, state: "loading" | "ready" | "failed") => void = () => {};
  private readonly sampler: Sampler;
  private readonly sampled: Set<string>;
  private readonly aliases: Record<string, string>;
  /** The voices made by the event being played, for `play` to return. */
  private made: Voice[] = [];
  private readonly base: string;

  constructor(catalogue?: VoiceCatalogue, base = document.baseURI) {
    this.ctx = new AudioContext({ latencyHint: "interactive" });
    const ctx = this.ctx;
    this.base = base;
    this.sampler = new Sampler(ctx);
    this.aliases = catalogue?.aliases ?? {};
    this.sampled = new Set(
      (catalogue?.families ?? []).flatMap((f) => f.voices.filter((v) => v.sampled).map((v) => v.code)),
    );
    // The master: a gentle compressor glues the parts, then a little gain.
    // It is slow and light enough to let an accent through.
    const comp = ctx.createDynamicsCompressor();
    comp.threshold.value = -12;
    comp.knee.value = 6;
    comp.ratio.value = 2.5;
    comp.attack.value = 0.02;
    comp.release.value = 0.15;
    const out = ctx.createGain();
    out.gain.value = 1.1;
    // Last, a limiter: fast and hard, it only acts on the rare peaks (an
    // accented strum with a golpe on top) that would otherwise clip. Its
    // threshold sits just under full scale, so the automatic make-up gain
    // stays tiny and the accents keep their distance from the other beats.
    const limit = ctx.createDynamicsCompressor();
    limit.threshold.value = -1.5;
    limit.knee.value = 0;
    limit.ratio.value = 20;
    limit.attack.value = 0.001;
    limit.release.value = 0.08;
    comp.connect(out).connect(limit).connect(ctx.destination);
    this.master = ctx.createGain();
    this.master.gain.value = 0.7;
    this.master.connect(comp);

    this.reverb = ctx.createConvolver();
    this.reverb.buffer = this.impulse(2.2);
    this.reverb.connect(this.master);

    const len = ctx.sampleRate;
    this.noise = ctx.createBuffer(1, len, ctx.sampleRate);
    const d = this.noise.getChannelData(0);
    for (let i = 0; i < len; i++) d[i] = Math.random() * 2 - 1;
  }

  /** A room: stereo noise decaying exponentially, darker as it fades. */
  private impulse(seconds: number): AudioBuffer {
    const rate = this.ctx.sampleRate;
    const len = Math.floor(rate * seconds);
    const buf = this.ctx.createBuffer(2, len, rate);
    for (let c = 0; c < 2; c++) {
      const d = buf.getChannelData(c);
      let lp = 0;
      for (let i = 0; i < len; i++) {
        const t = i / len;
        // A one-pole low-pass that closes over the tail.
        const k = 0.6 - 0.5 * t;
        lp += k * (Math.random() * 2 - 1 - lp);
        d[i] = lp * Math.exp(-5 * t) * (i < rate * 0.01 ? i / (rate * 0.01) : 1);
      }
    }
    return buf;
  }
  private chain(part: string): Chain {
    let c = this.chains.get(part);
    if (c) return c;
    const ctx = this.ctx;
    const input = ctx.createGain();
    const highpass = ctx.createBiquadFilter();
    highpass.type = "highpass";
    highpass.frequency.value = 10;
    const lowpass = ctx.createBiquadFilter();
    lowpass.type = "lowpass";
    lowpass.frequency.value = 20000;
    const volume = ctx.createGain();
    volume.gain.value = 0.8;
    const pan = ctx.createStereoPanner();
    const delaySend = ctx.createGain();
    delaySend.gain.value = 0;
    const delay = ctx.createDelay(2.5);
    delay.delayTime.value = 0.25;
    const feedback = ctx.createGain();
    feedback.gain.value = 0.35;
    const reverbSend = ctx.createGain();
    reverbSend.gain.value = 0;
    input.connect(highpass).connect(lowpass).connect(volume).connect(pan);
    pan.connect(this.master);
    pan.connect(delaySend).connect(delay);
    delay.connect(feedback).connect(delay);
    delay.connect(this.master);
    pan.connect(reverbSend).connect(this.reverb);
    c = {
      input,
      highpass,
      lowpass,
      volume,
      pan,
      delaySend,
      delay,
      feedback,
      reverbSend,
      instrument: "synth",
      voice: "synth.triangle",
      attack: 0.005,
      release: 0.15,
      configured: new Map(),
    };
    this.chains.set(part, c);
    return c;
  }

  /** Sets up every part of a program: its sound, its chain, its numbers.
   * While `playing`, parameters that move are left where they are. */
  configure(program: Program, playing = false): void {
    const moving = new Set(playing ? program.moving.map((m) => `${m.part}	${m.id}`) : []);
    for (const part of program.parts) this.configurePart(part, program.params, moving);
  }

  private configurePart(part: PartInfo, globals: ParamValue[], moving: Set<string>): void {
    const c = this.chain(part.name);
    c.instrument = part.instrument;
    const written = part.chain[0] ?? "";
    const head = this.aliases[written] ?? written;
    const isBass = /bass|baixo|bajo/i.test(part.name);
    const known = head.includes(".") || head === "harp" || head === "harmonica";
    c.voice = known
      ? head
      : part.instrument === "drums"
        ? "drums.kit"
        : part.instrument === "guitar"
          ? "guitar.nylon"
          : part.instrument === "keys"
            ? "keys.epiano"
            : isBass
              ? "bass.synth"
              : "synth.triangle";
    // Samples and drum kits go to Manguebyte's engine; the rest, and
    // everything when that engine cannot run, stays in the stand-in. A
    // sampled voice starts on its way now, so it is ready sooner.
    this.route(c);
    // Defaults from the chain, then the numbers written in the piece.
    const now = this.ctx.currentTime;
    const defaults: ParamValue[] = [
      { id: "volume", value: 0.8 },
      { id: "pan", value: 0 },
      { id: "lowpass.cutoff", value: part.chain.includes("lowpass") ? 1500 : isBass && !head ? 900 : 20000 },
      { id: "lowpass.resonance", value: 0.2 },
      { id: "highpass.cutoff", value: part.chain.includes("highpass") ? 300 : 10 },
      { id: "reverb.mix", value: part.chain.includes("reverb") ? 0.3 : 0.08 },
      { id: "delay.mix", value: part.chain.includes("delay") ? 0.25 : 0 },
      { id: "delay.time", value: 250 },
      { id: "delay.feedback", value: 0.35 },
      { id: "attack", value: 5 },
      { id: "release", value: 150 },
    ];
    for (const p of [...defaults, ...globals, ...part.params]) {
      c.configured.set(p.id, p.value);
      if (moving.has(`${part.name}	${p.id}`) || moving.has(`all	${p.id}`)) continue;
      this.setParam(c, p.id, p.value, now);
    }
  }

  /** Sets a parameter at `when`; with `glide`, the sound moves to it
   * smoothly over the seconds before, as a moving parameter does. */
  private setParam(c: Chain, id: string, v: number, when: number, glide = 0): void {
    const set = (p: AudioParam, value: number): void => {
      if (glide > 0) p.setTargetAtTime(value, Math.max(when - glide, 0), glide / 3);
      else p.setTargetAtTime(value, when, 0.015);
    };
    switch (id) {
      case "volume":
        return set(c.volume.gain, v);
      case "pan":
        return set(c.pan.pan, v);
      case "lowpass.cutoff":
        return set(c.lowpass.frequency, v);
      case "lowpass.resonance":
        return set(c.lowpass.Q, 0.5 + v * 18);
      case "highpass.cutoff":
        return set(c.highpass.frequency, v);
      case "highpass.resonance":
        return set(c.highpass.Q, 0.5 + v * 18);
      case "reverb.mix":
        return set(c.reverbSend.gain, v);
      case "delay.mix":
        return set(c.delaySend.gain, v);
      case "delay.time":
        return set(c.delay.delayTime, v / 1000);
      case "delay.feedback":
        return set(c.feedback.gain, Math.min(v, 0.95));
      case "attack":
        c.attack = v / 1000;
        return;
      case "release":
        c.release = v / 1000;
        return;
    }
  }

  /** Changes one parameter now, as a knob does. */
  setNow(part: string, id: string, value: number): void {
    const targets = part === "all" ? [...this.chains.values()] : [this.chains.get(part)];
    for (const c of targets) if (c) this.setParam(c, id, value, this.ctx.currentTime);
  }

  /** What Manguebyte plays of a voice: nothing, its synthesized kit, a
   * sampled kit or a sampled instrument. */
  private mbKind(voice: string): MbPart["kind"] | null {
    if (voice === "drums.kit") return "drums";
    if (voice === "drums.acoustic") return "sampler_kit";
    if (this.sampled.has(voice)) return "sampler";
    return null;
  }

  /** Gives a part its Manguebyte engine when its voice is one Manguebyte
   * plays, and takes it away when the voice changes. */
  private route(c: Chain): void {
    const kind = this.mbKind(c.voice);
    if (c.mb && c.mb.voice === c.voice) return;
    if (c.mb) {
      const old = c.mb.engine;
      c.mb = undefined;
      if (old) {
        old.allNotesOff(0);
        window.setTimeout(() => {
          old.node.disconnect();
          void old.close();
        }, 600);
      }
    }
    if (!kind) return;
    const part: MbPart = { voice: c.voice, kind, engine: null, ready: false, failed: false };
    c.mb = part;
    void this.startMb(c, part);
  }

  private moduleOf(): Promise<WebAssembly.Module | null> {
    this.mbModule ??= (async () => {
      if (typeof AudioWorkletNode === "undefined" || !this.ctx.audioWorklet) return null;
      // The Sounds panel can turn Manguebyte off: everything then plays in
      // the stand-in, a way out on stage if something goes wrong.
      if (!Engine.manguebyteWanted()) return null;
      try {
        const version = new URL(import.meta.url).searchParams.get("v");
        const url = new URL(`manguebyte/manguebyte_web.wasm${version ? `?v=${version}` : ""}`, this.base);
        const r = await fetch(url);
        if (!r.ok) return null;
        return await WebAssembly.compile(await r.arrayBuffer());
      } catch {
        return null;
      }
    })();
    return this.mbModule;
  }

  /** Whether the person wants Manguebyte to play (the Sounds panel's
   * switch, kept in this browser). */
  static manguebyteWanted(): boolean {
    try {
      return localStorage.getItem("aratu.manguebyte") !== "off";
    } catch {
      return true;
    }
  }

  private async startMb(c: Chain, part: MbPart): Promise<void> {
    const sampled = part.kind !== "drums";
    if (part.kind === "sampler") this.setMbState(part.voice, "loading");
    try {
      const module = await this.moduleOf();
      if (!module) throw new Error("Manguebyte cannot run in this browser");
      const engine = await ManguebyteEngine.create(this.ctx, {
        wasm: module,
        workletUrl: new URL("manguebyte/worklet.js", this.base),
      });
      if (c.mb !== part) {
        void engine.close();
        return;
      }
      await engine.createPart(0, part.kind, "none");
      const level = this.ctx.createGain();
      level.gain.value = MB_LEVEL[part.kind] ?? 1;
      engine.node.connect(level).connect(c.input);
      part.engine = engine;
      // A kit plays its synthesized drums while its samples come.
      if (part.kind !== "sampler") part.ready = true;
      if (sampled) {
        if (part.kind === "sampler_kit") this.setMbState(part.voice, "loading");
        await engine.loadSfz(0, `samples/${part.voice}/voice.sfz`, { base: this.base });
        if (c.mb !== part) return;
        part.ready = true;
        this.setMbState(part.voice, "ready");
      }
    } catch (error) {
      console.warn("Manguebyte:", error);
      part.failed = true;
      part.ready = false;
      if (sampled) this.setMbState(part.voice, "failed");
      // The stand-in, with its own copy of the samples, takes over.
      if (this.sampled.has(part.voice)) void this.sampler.load(part.voice);
    }
  }

  private setMbState(code: string, state: "loading" | "ready" | "failed"): void {
    this.mbStates.set(code, state);
    this.notify(code, state);
  }

  /** Sends an event to the part's Manguebyte engine, when it plays the part. */
  private playMb(c: Chain, e: AratuEvent, when: number): boolean {
    const part = c.mb;
    const engine = part?.engine;
    if (!part || !engine || !part.ready) return false;
    const velocity = Math.max(0.01, Math.min(1, e.vel / 127));
    const accent = e.acc === 1;
    if (e.drum) {
      const note = MB_DRUMS[e.drum];
      if (note === undefined || part.kind === "sampler") return false;
      engine.noteOn(0, note, velocity, when, 0, accent);
    } else if (e.note !== undefined) {
      if (part.kind !== "sampler") return false;
      engine.noteOn(0, e.note, velocity, when, 0, accent);
      engine.noteOff(0, e.note, when + Math.max(e.dur, 0.02));
    } else {
      return false;
    }
    this.made.push({ mb: engine });
    return true;
  }

  /** Schedules an event; returns the voices it made, for `cancel`. */
  play(e: AratuEvent, when: number): Voice[] {
    this.made = [];
    if (e.param !== undefined && e.value !== undefined) {
      const targets = e.part === "all" ? [...this.chains.values()] : [this.chain(e.part)];
      for (const c of targets) {
        // A range is over: back to how the part is set up.
        const value = e.reset ? c?.configured.get(e.param) : e.value;
        if (c && value !== undefined) this.setParam(c, e.param, value, when, e.glide ?? 0);
      }
      return this.made;
    }
    const c = this.chain(e.part);
    if (this.playMb(c, e, when)) return this.made;
    // Loudness grows faster than velocity, so the step from mf to an
    // accent is heard; mf itself stays where it was.
    const g = Math.pow(e.vel / 127, 2) * 0.72;
    if (e.drum) this.drum(c, e.drum, when, g);
    else if (e.note !== undefined) this.note(c, e.note, when, Math.max(e.dur, 0.02), g, e.acc === 1);
    return this.made;
  }

  /** Where a sampled voice stands: not asked for, coming, ready, failed. */
  sampleState(code: string): "idle" | "loading" | "ready" | "failed" {
    const mb = this.mbStates.get(code);
    if (mb === "ready" || mb === "loading") return mb;
    return this.sampler.state(code);
  }

  /** Tells the Sounds panel when a sampled voice arrives. */
  set onSample(f: (code: string, state: "loading" | "ready" | "failed") => void) {
    this.notify = f;
    this.sampler.onChange = f;
  }

  /** Which engine plays a voice now, for the panel: Manguebyte or the
   * stand-in. */
  playedBy(code: string): "manguebyte" | "stand-in" {
    for (const c of this.chains.values()) if (c.mb?.voice === code && c.mb.ready) return "manguebyte";
    return "stand-in";
  }

  /** The voice that sounds now for a chain: a sampled one still on its way
   * plays its stand-in. */
  private sounding(c: Chain): { voice: string; loaded?: Loaded } {
    if (!this.sampled.has(c.voice)) return { voice: c.voice };
    const loaded = this.sampler.get(c.voice);
    if (loaded) return { voice: c.voice, loaded };
    // Manguebyte is bringing it: a synthesized stand-in meanwhile. Only
    // when Manguebyte cannot play it does the stand-in fetch the samples.
    if (!c.mb || c.mb.failed) void this.sampler.load(c.voice);
    return { voice: STAND_IN[c.voice] ?? "synth.triangle" };
  }

  /** Silences voices scheduled but not yet sounding, as when a version of
   * the piece is replaced inside a bar. */
  cancel(voices: Voice[], from: number): void {
    const engines = new Set<ManguebyteEngine>();
    for (const v of voices) {
      if ("mb" in v) {
        engines.add(v.mb);
        continue;
      }
      if (!this.voices.delete(v)) continue;
      v.gain.disconnect();
      try {
        v.src.stop(from);
      } catch {
        // Already stopped: nothing more to do.
      }
    }
    // Manguebyte drops what is left of the part and fades what sounds.
    for (const engine of engines) engine.cancel(0, from);
  }

  /** Forgets the parameter changes scheduled from `from` on. */
  cancelParams(from: number): void {
    for (const c of this.chains.values()) {
      const params = [
        c.highpass.frequency,
        c.highpass.Q,
        c.lowpass.frequency,
        c.lowpass.Q,
        c.volume.gain,
        c.pan.pan,
        c.delaySend.gain,
        c.delay.delayTime,
        c.feedback.gain,
        c.reverbSend.gain,
      ];
      for (const p of params) p.cancelScheduledValues(from);
    }
  }

  private track(src: AudioScheduledSourceNode, gain: GainNode): void {
    const v = { src, gain };
    this.voices.add(v);
    this.made.push(v);
    src.onended = () => {
      this.voices.delete(v);
      gain.disconnect();
    };
  }

  private noiseSource(when: number, length: number): AudioBufferSourceNode {
    const src = this.ctx.createBufferSource();
    src.buffer = this.noise;
    src.loop = true;
    src.loopStart = Math.random() * 0.5;
    src.start(when, Math.random() * 0.5);
    src.stop(when + length);
    return src;
  }

  /** A short burst of filtered noise into `dest`. */
  private noiseHit(dest: AudioNode, when: number, type: BiquadFilterType, freq: number, q: number, peak: number, decay: number): SourceVoice {
    const gain = this.ctx.createGain();
    const n = this.noiseSource(when, decay * 4 + 0.05);
    const f = this.ctx.createBiquadFilter();
    f.type = type;
    f.frequency.value = freq;
    f.Q.value = q;
    n.connect(f).connect(gain).connect(dest);
    gain.gain.setValueAtTime(0.0001, when);
    gain.gain.linearRampToValueAtTime(peak, when + 0.001);
    gain.gain.setTargetAtTime(0.0001, when + 0.001, decay);
    this.track(n, gain);
    return { src: n, gain };
  }

  /** The six square waves of an analogue cymbal, high-passed. */
  private metal(dest: AudioNode, when: number, peak: number, decay: number, cutoff: number): void {
    const gain = this.ctx.createGain();
    const hp = this.ctx.createBiquadFilter();
    hp.type = "highpass";
    hp.frequency.value = cutoff;
    hp.connect(gain).connect(dest);
    gain.gain.setValueAtTime(0.0001, when);
    gain.gain.linearRampToValueAtTime(peak, when + 0.002);
    gain.gain.setTargetAtTime(0.0001, when + 0.002, decay);
    const stop = when + decay * 5 + 0.05;
    for (const f of [205.3, 304.4, 369.6, 522.7, 540, 800]) {
      const o = this.ctx.createOscillator();
      o.type = "square";
      o.frequency.value = f * 1.7;
      o.connect(hp);
      o.start(when);
      o.stop(stop);
      this.track(o, gain);
    }
  }

  /** A tom: a skin tuned to `freq` that falls a little as it rings. */
  private tom(c: Chain, when: number, g: number, bright: number, freq: number, decay: number): void {
    const gain = this.ctx.createGain();
    gain.connect(c.input);
    // The skin and its first overtone.
    for (const [ratio, level] of [[1, 1], [1.6, 0.25]]) {
      const o = this.ctx.createOscillator();
      o.type = "sine";
      o.frequency.setValueAtTime(freq * ratio * (1 + 0.35 * Math.min(bright, 1.2)), when);
      o.frequency.exponentialRampToValueAtTime(freq * ratio, when + 0.04);
      o.frequency.exponentialRampToValueAtTime(freq * ratio * 0.9, when + decay * 3);
      const og = this.ctx.createGain();
      og.gain.value = level;
      o.connect(og).connect(gain);
      o.start(when);
      o.stop(when + decay * 5 + 0.05);
      this.track(o, gain);
    }
    gain.gain.setValueAtTime(0.0001, when);
    gain.gain.linearRampToValueAtTime(g * 1.3, when + 0.002);
    gain.gain.setTargetAtTime(0.0001, when + 0.01, decay);
    // The stick on the skin.
    this.noiseHit(c.input, when, "bandpass", freq * 8, 1, g * 0.3, 0.012);
  }

  /** A recorded drum hit; a closed hat chokes an open one. */
  private hit(c: Chain, name: string, buffer: AudioBuffer, when: number, g: number): void {
    const ctx = this.ctx;
    if (name === "closed-hat") this.chokeOpenHat(c, when);
    const src = ctx.createBufferSource();
    src.buffer = buffer;
    const gain = ctx.createGain();
    const level: Record<string, number> = { kick: 1.6, snare: 1.3, "closed-hat": 0.7, "open-hat": 0.7, ride: 0.6, crash: 0.8 };
    gain.gain.value = g * (level[name] ?? 1.2);
    // Softer hits are darker as well as quieter.
    const lp = ctx.createBiquadFilter();
    lp.type = "lowpass";
    lp.frequency.value = 3000 + 17000 * Math.min(g / 0.6, 1);
    src.connect(lp).connect(gain).connect(c.input);
    src.start(when);
    this.track(src, gain);
    if (name === "open-hat") this.openHats.set(c, { src, gain });
  }

  private chokeOpenHat(c: Chain, when: number): void {
    const open = this.openHats.get(c);
    if (open) {
      open.gain.gain.cancelScheduledValues(when);
      open.gain.gain.setTargetAtTime(0.0001, when, 0.006);
      this.openHats.delete(c);
    }
  }

  /** A kit after the Roland TR-808: sine kick and toms, metallic hats. */
  private drum808(c: Chain, name: string, when: number, g: number): void {
    const ctx = this.ctx;
    const boom = (from: number, to: number, decay: number, level: number) => {
      const gain = ctx.createGain();
      gain.connect(c.input);
      const o = ctx.createOscillator();
      o.frequency.setValueAtTime(from, when);
      o.frequency.exponentialRampToValueAtTime(to, when + 0.06);
      o.connect(gain);
      gain.gain.setValueAtTime(0.0001, when);
      gain.gain.linearRampToValueAtTime(g * level, when + 0.002);
      gain.gain.setTargetAtTime(0.0001, when + 0.01, decay);
      o.start(when);
      o.stop(when + decay * 6 + 0.05);
      this.track(o, gain);
    };
    switch (name) {
      case "kick":
        boom(130, 48, 0.35, 2.2);
        this.noiseHit(c.input, when, "highpass", 2500, 0.7, g * 0.25, 0.003);
        return;
      case "snare":
        boom(330, 180, 0.05, 0.6);
        this.noiseHit(c.input, when, "highpass", 1800, 0.7, g * 0.9, 0.09);
        return;
      case "closed-hat":
        this.chokeOpenHat(c, when);
        this.metal(c.input, when, g * 0.35, 0.025, 8000);
        return;
      case "open-hat": {
        const before = this.made.length;
        this.metal(c.input, when, g * 0.3, 0.22, 7500);
        const v = this.made[before];
        if (v && !("mb" in v)) this.openHats.set(c, v);
        return;
      }
      case "ride":
        this.metal(c.input, when, g * 0.18, 0.7, 5000);
        return;
      case "crash":
        this.metal(c.input, when, g * 0.2, 1.1, 4000);
        this.noiseHit(c.input, when, "highpass", 5000, 0.7, g * 0.3, 0.6);
        return;
      case "high-tom":
        boom(240, 190, 0.22, 1.4);
        return;
      case "mid-tom":
        boom(180, 140, 0.28, 1.4);
        return;
      case "floor-tom":
        boom(120, 92, 0.38, 1.5);
        return;
      default:
        // The rim and anything else: the analogue kit's own.
        this.analogue(c, name, when, g);
    }
  }

  private drum(c: Chain, name: string, when: number, g: number): void {
    if (name !== "guitar-tap") {
      const { voice, loaded } = this.sounding(c);
      if (loaded?.kind === "kit") {
        const buffer = loaded.hits.get(name);
        if (buffer) return this.hit(c, name, buffer, when, g);
      }
      if (voice === "drums.808") return this.drum808(c, name, when, g);
    }
    this.analogue(c, name, when, g);
  }

  /** The analogue kit, `drums.kit`. */
  private analogue(c: Chain, name: string, when: number, g: number): void {
    const ctx = this.ctx;
    // Harder hits are brighter, not only louder.
    const bright = 0.6 + 0.8 * Math.min(g / 0.6, 1);
    switch (name) {
      case "kick": {
        const gain = ctx.createGain();
        gain.connect(c.input);
        const o = ctx.createOscillator();
        o.frequency.setValueAtTime(150 * Math.min(bright, 1.2), when);
        o.frequency.exponentialRampToValueAtTime(50, when + 0.05);
        o.frequency.exponentialRampToValueAtTime(42, when + 0.4);
        gain.gain.setValueAtTime(0.0001, when);
        gain.gain.linearRampToValueAtTime(g * 1.9, when + 0.002);
        gain.gain.setTargetAtTime(0.0001, when + 0.02, 0.12);
        o.connect(gain);
        o.start(when);
        o.stop(when + 0.7);
        this.track(o, gain);
        // The beater's click.
        this.noiseHit(c.input, when, "highpass", 1800 * bright, 0.7, g * 0.5, 0.004);
        return;
      }
      case "snare": {
        const og = ctx.createGain();
        og.connect(c.input);
        for (const f of [185, 330]) {
          const o = ctx.createOscillator();
          o.type = "triangle";
          o.frequency.setValueAtTime(f * 1.15, when);
          o.frequency.exponentialRampToValueAtTime(f, when + 0.03);
          o.connect(og);
          o.start(when);
          o.stop(when + 0.3);
          this.track(o, og);
        }
        og.gain.setValueAtTime(0.0001, when);
        og.gain.linearRampToValueAtTime(g * 0.6, when + 0.001);
        og.gain.setTargetAtTime(0.0001, when + 0.001, 0.035);
        this.noiseHit(c.input, when, "bandpass", 3200 * bright, 0.6, g * 1.1, 0.06);
        this.noiseHit(c.input, when, "highpass", 7000, 0.7, g * 0.35, 0.03);
        return;
      }
      case "closed-hat": {
        // A closed hat chokes an open one still ringing.
        this.chokeOpenHat(c, when);
        this.noiseHit(c.input, when, "highpass", 7500 * Math.min(bright, 1.1), 0.8, g * 0.42, 0.018);
        this.noiseHit(c.input, when, "bandpass", 10000, 1.5, g * 0.25, 0.012);
        return;
      }
      case "open-hat": {
        const v = this.noiseHit(c.input, when, "highpass", 7000 * Math.min(bright, 1.1), 0.7, g * 0.38, 0.13);
        this.openHats.set(c, v);
        return;
      }
      case "ride":
        this.metal(c.input, when, g * 0.18, 0.35, 6000);
        this.noiseHit(c.input, when, "bandpass", 8000, 1.2, g * 0.12, 0.25);
        return;
      case "crash":
        this.metal(c.input, when, g * 0.16, 0.5, 4500);
        this.noiseHit(c.input, when, "highpass", 4000 * bright, 0.7, g * 0.4, 0.45);
        return;
      case "high-tom":
        this.tom(c, when, g, bright, 210, 0.16);
        return;
      case "mid-tom":
        this.tom(c, when, g, bright, 155, 0.2);
        return;
      case "floor-tom":
        this.tom(c, when, g, bright, 100, 0.28);
        return;
      case "guitar-tap": {
        // The flamenco golpe: a finger on the guitar's top. The wooden body
        // thumps, the nail clicks, and the strings stay still.
        const gain = ctx.createGain();
        gain.connect(c.input);
        for (const [f, level] of [[105, 1], [210, 0.45]]) {
          const o = ctx.createOscillator();
          o.type = "sine";
          o.frequency.setValueAtTime(f * 1.15, when);
          o.frequency.exponentialRampToValueAtTime(f, when + 0.015);
          const og = ctx.createGain();
          og.gain.value = level;
          o.connect(og).connect(gain);
          o.start(when);
          o.stop(when + 0.25);
          this.track(o, gain);
        }
        gain.gain.setValueAtTime(0.0001, when);
        gain.gain.linearRampToValueAtTime(g * 1.1, when + 0.002);
        gain.gain.setTargetAtTime(0.0001, when + 0.004, 0.045);
        this.noiseHit(c.input, when, "bandpass", 1400 * bright, 1.2, g * 0.9, 0.012);
        this.noiseHit(c.input, when, "highpass", 4000, 0.7, g * 0.35, 0.006);
        return;
      }
      case "clap": {
        // Hands clapping: a few hands a hair apart, then the room.
        for (const [dt, level] of [[0, 0.8], [0.011, 0.7], [0.022, 1]]) {
          this.noiseHit(c.input, when + dt, "bandpass", 1100 * bright, 1.4, g * level, 0.008);
        }
        this.noiseHit(c.input, when + 0.03, "bandpass", 1300, 1.1, g * 0.5, 0.07);
        return;
      }
      case "side-stick": {
        // The stick across the rim: a dry, woody click with no snare wires.
        const gain = ctx.createGain();
        gain.connect(c.input);
        const o = ctx.createOscillator();
        o.type = "triangle";
        o.frequency.setValueAtTime(1750, when);
        o.frequency.exponentialRampToValueAtTime(1500, when + 0.02);
        o.connect(gain);
        o.start(when);
        o.stop(when + 0.1);
        this.track(o, gain);
        gain.gain.setValueAtTime(0.0001, when);
        gain.gain.linearRampToValueAtTime(g * 0.5, when + 0.001);
        gain.gain.setTargetAtTime(0.0001, when + 0.001, 0.012);
        this.noiseHit(c.input, when, "bandpass", 2600 * bright, 2.5, g * 0.9, 0.01);
        this.noiseHit(c.input, when, "bandpass", 480, 4, g * 0.6, 0.02);
        return;
      }
      default:
        this.noiseHit(c.input, when, "highpass", 6000, 0.7, g * 0.4, 0.05);
    }
  }

  /** A plucked string (Karplus-Strong), brighter when plucked harder.
   * `kind`: 0 nylon, 1 steel, 2 finger bass, 3 slap bass. */
  private pluckBuffer(note: number, bright: number, kind = 0): AudioBuffer {
    const level = Math.round(Math.min(Math.max(bright, 0), 1) * 3);
    const key = (note * 4 + level) * 4 + kind;
    const cached = this.plucks.get(key);
    if (cached) return cached;
    const rate = this.ctx.sampleRate;
    const len = Math.floor(rate * 3);
    const buf = this.ctx.createBuffer(1, len, rate);
    const d = buf.getChannelData(0);
    const period = Math.max(2, Math.round(rate / mtof(note)));
    const ring = new Float32Array(period);
    // A softer pluck starts from smoother noise.
    const smooth = Math.min(0.99, [0.25, 0.45, 0.7, 0.95][level] + [0, 0.15, -0.1, 0.3][kind]);
    let s = 0;
    for (let i = 0; i < period; i++) {
      s += smooth * (Math.random() * 2 - 1 - s);
      ring[i] = s;
    }
    const loss = [0.9965 + Math.min(0.003, period / 500000), 0.9982, 0.9978, 0.997][kind];
    const tone = [0.55, 0.8, 0.3, 0.85][kind];
    let idx = 0;
    let body = 0;
    for (let i = 0; i < len; i++) {
      const next = (idx + 1) % period;
      const v = ring[idx];
      ring[idx] = loss * (0.5 * v + 0.5 * ring[next]);
      // A gentle body low-pass.
      body += tone * (v - body);
      d[i] = body * 1.4;
      idx = next;
    }
    this.plucks.set(key, buf);
    return buf;
  }

  private note(c: Chain, note: number, when: number, dur: number, g: number, accent = false): void {
    const { voice, loaded } = this.sounding(c);
    const bright = Math.min(g / 0.6, 1);
    g *= VOICE_GAIN[voice] ?? 1;
    if (loaded?.kind === "pitched") return this.sample(c, loaded, note, when, dur, g);
    switch (voice) {
      case "guitar.nylon":
      case "synth.pluck":
        return this.pluck(c, note, when, dur, g * 1.3, bright, 0);
      case "guitar.steel":
        return this.pluck(c, note, when, dur, g * 1.2, bright, 1);
      case "keys.epiano":
        return this.epiano(c, note, when, dur, g, bright);
      case "keys.organ":
        return this.organ(c, note, when, dur, g);
      case "keys.bell":
        return this.bell(c, note, when, dur, g, bright);
      case "bass.finger":
        this.pluck(c, note, when, dur, g * 1.5, bright * 0.6, 2);
        return this.sine(c, note - 12, when, dur, g * 0.35, 0.01, 0.08);
      case "bass.slap":
        return this.slap(c, note, when, dur, g, accent);
      case "bass.808":
        return this.bass808(c, note, when, dur, g);
      case "bass.acid":
        return this.acid(c, note, when, dur, g, accent);
      case "bass.synth":
        return this.subtractive(c, note, when, dur, g, bright, { wave: "sawtooth", detune: [-7, 0, 7], sub: true });
      case "synth.saw":
        return this.subtractive(c, note, when, dur, g, bright, { wave: "sawtooth", detune: [-7, 0, 7] });
      case "synth.square":
        return this.subtractive(c, note, when, dur, g, bright, { wave: "square", detune: [-7, 0, 7] });
      case "synth.sine":
        return this.subtractive(c, note, when, dur, g, bright, { wave: "sine", detune: [0] });
      case "synth.supersaw":
        return this.subtractive(c, note, when, dur, g * 0.8, bright, {
          wave: "sawtooth",
          detune: [-28, -17, -8, 0, 8, 17, 28],
          spread: true,
        });
      case "synth.pad":
        return this.subtractive(c, note, when, dur, g, bright * 0.5, {
          wave: "sawtooth",
          detune: [-12, -4, 4, 12],
          attack: 0.6,
          release: 1.2,
          cutoff: 3,
          spread: true,
        });
      case "synth.strings":
        return this.subtractive(c, note, when, dur, g, bright * 0.7, {
          wave: "sawtooth",
          detune: [-9, 0, 9],
          attack: 0.25,
          release: 0.6,
          cutoff: 5,
          vibrato: 0.004,
          spread: true,
        });
      case "synth.brass":
        return this.subtractive(c, note, when, dur, g, bright, {
          wave: "sawtooth",
          detune: [-5, 5],
          attack: 0.07,
          release: 0.15,
          swell: true,
        });
      case "synth.lead":
        return this.subtractive(c, note, when, dur, g, bright, {
          wave: "square",
          detune: [0, 6],
          vibrato: 0.006,
          release: 0.1,
        });
      default:
        return this.subtractive(c, note, when, dur, g, bright, { wave: "triangle", detune: [-7, 0, 7] });
    }
  }

  /** A recorded note: the nearest recording, retuned; a held one loops. */
  private sample(c: Chain, v: Loaded, note: number, when: number, dur: number, g: number): void {
    const z = Sampler.zoneFor(v, note);
    if (!z) return;
    const ctx = this.ctx;
    const src = ctx.createBufferSource();
    src.buffer = z.zone.buffer;
    src.playbackRate.value = z.rate;
    const held = !!z.zone.loop;
    if (z.zone.loop) {
      src.loop = true;
      src.loopStart = z.zone.loop[0];
      src.loopEnd = z.zone.loop[1];
    }
    // Harder playing is brighter as well as louder.
    const lp = ctx.createBiquadFilter();
    lp.type = "lowpass";
    lp.frequency.value = 2500 + 16000 * Math.min(g / 0.6, 1);
    const gain = ctx.createGain();
    src.connect(lp).connect(gain).connect(c.input);
    const end = when + dur;
    const level = g * 1.5;
    if (held) {
      gain.gain.setValueAtTime(0.0001, when);
      gain.gain.linearRampToValueAtTime(level, when + 0.02);
    } else {
      gain.gain.setValueAtTime(level, when);
    }
    // A key let go: the damper stops the string; a held sound fades out.
    const release = held ? Math.max(c.release, 0.08) : 0.12;
    gain.gain.setTargetAtTime(0.0001, end, release / 3);
    const ring = held ? Infinity : (z.zone.buffer.duration - 0.01) / z.rate;
    src.start(when);
    src.stop(Math.min(end + release * 3, when + ring));
    this.track(src, gain);
  }

  private pluck(c: Chain, note: number, when: number, dur: number, level: number, bright: number, kind: number): void {
    const ctx = this.ctx;
    const end = when + dur;
    const gain = ctx.createGain();
    gain.connect(c.input);
    const src = ctx.createBufferSource();
    src.buffer = this.pluckBuffer(note, bright, kind);
    gain.gain.setValueAtTime(level, when);
    gain.gain.setTargetAtTime(0.0001, end, 0.06);
    src.connect(gain);
    src.start(when);
    src.stop(end + 0.4);
    this.track(src, gain);
  }

  /** A slap bass: the thumb on the string, or (accented) a pop, the string
   * pulled and let snap on the frets. */
  private slap(c: Chain, note: number, when: number, dur: number, g: number, pop: boolean): void {
    this.pluck(c, note, when, dur, g * (pop ? 1.9 : 1.5), pop ? 1 : 0.8, 3);
    this.sine(c, note - 12, when, dur, g * 0.3, 0.005, 0.06);
    if (pop) {
      this.noiseHit(c.input, when, "bandpass", 2800, 1.5, g * 0.9, 0.012);
      this.noiseHit(c.input, when, "highpass", 6000, 0.7, g * 0.4, 0.006);
    } else {
      // The thumb's thud on the string.
      this.noiseHit(c.input, when, "bandpass", 900, 1.2, g * 0.6, 0.015);
    }
  }

  private sine(c: Chain, note: number, when: number, dur: number, level: number, attack: number, release: number): void {
    const ctx = this.ctx;
    const gain = ctx.createGain();
    gain.connect(c.input);
    const o = ctx.createOscillator();
    o.frequency.value = mtof(note);
    o.connect(gain);
    gain.gain.setValueAtTime(0.0001, when);
    gain.gain.linearRampToValueAtTime(level, when + attack);
    gain.gain.setTargetAtTime(0.0001, when + dur, release / 3);
    o.start(when);
    o.stop(when + dur + release * 3 + 0.05);
    this.track(o, gain);
  }

  /** An electric piano: a sine, its octave fading fast, and a tine. */
  private epiano(c: Chain, note: number, when: number, dur: number, g: number, bright: number): void {
    const ctx = this.ctx;
    const end = when + dur;
    const freq = mtof(note);
    const gain = ctx.createGain();
    gain.connect(c.input);
    const parts: [number, number, number][] = [
      [1, 1, 1.2],
      [2, 0.35 * (0.4 + bright), 0.35],
      [4, 0.12 * bright, 0.05],
    ];
    gain.gain.setValueAtTime(0.0001, when);
    gain.gain.linearRampToValueAtTime(g * 0.9, when + 0.004);
    gain.gain.setTargetAtTime(g * 0.35, when + 0.004, 0.6);
    gain.gain.setTargetAtTime(0.0001, end, 0.12);
    for (const [mult, level, decay] of parts) {
      const o = ctx.createOscillator();
      o.frequency.value = freq * mult;
      const og = ctx.createGain();
      og.gain.setValueAtTime(level, when);
      og.gain.setTargetAtTime(level * 0.1, when, decay);
      o.connect(og).connect(gain);
      o.start(when);
      o.stop(end + 0.8);
      this.track(o, gain);
    }
  }

  /** A drawbar organ: sines at the drawbars' footages, a slow vibrato. */
  private organ(c: Chain, note: number, when: number, dur: number, g: number): void {
    const ctx = this.ctx;
    const end = when + dur;
    const freq = mtof(note);
    const gain = ctx.createGain();
    gain.connect(c.input);
    gain.gain.setValueAtTime(0.0001, when);
    gain.gain.linearRampToValueAtTime(g * 0.5, when + 0.01);
    gain.gain.setTargetAtTime(0.0001, end, 0.03);
    const lfo = ctx.createOscillator();
    lfo.frequency.value = 6.2;
    const depth = ctx.createGain();
    depth.gain.value = 4;
    lfo.connect(depth);
    lfo.start(when);
    lfo.stop(end + 0.2);
    // 16', 8', 4', 2 2/3', 2' at 8 8 6 4 4 of 8.
    for (const [mult, level] of [
      [0.5, 0.8],
      [1, 1],
      [2, 0.6],
      [3, 0.35],
      [4, 0.35],
    ]) {
      const o = ctx.createOscillator();
      o.frequency.value = freq * mult;
      depth.connect(o.detune);
      const og = ctx.createGain();
      og.gain.value = level / 3;
      o.connect(og).connect(gain);
      o.start(when);
      o.stop(end + 0.2);
      this.track(o, gain);
    }
    this.track(lfo, gain);
    // The key click.
    this.noiseHit(c.input, when, "bandpass", 3000, 1, g * 0.15, 0.004);
  }

  /** An FM bell: a sine carrier, its modulator at 3.5 times the note, the
   * brightness fading first. */
  private bell(c: Chain, note: number, when: number, dur: number, g: number, bright: number): void {
    const ctx = this.ctx;
    const freq = mtof(note);
    const decay = 1.2;
    const gain = ctx.createGain();
    gain.connect(c.input);
    gain.gain.setValueAtTime(0.0001, when);
    gain.gain.linearRampToValueAtTime(g * 0.8, when + 0.003);
    gain.gain.setTargetAtTime(0.0001, when + 0.003, decay * 0.5);
    gain.gain.setTargetAtTime(0.0001, when + dur, 0.3);
    const carrier = ctx.createOscillator();
    carrier.frequency.value = freq;
    const mod = ctx.createOscillator();
    mod.frequency.value = freq * 3.5;
    const index = ctx.createGain();
    index.gain.setValueAtTime(freq * (1.5 + 3 * bright), when);
    index.gain.setTargetAtTime(freq * 0.2, when, 0.25);
    mod.connect(index).connect(carrier.frequency);
    carrier.connect(gain);
    const stop = when + Math.max(dur, decay * 3) + 0.5;
    for (const o of [carrier, mod]) {
      o.start(when);
      o.stop(stop);
      this.track(o, gain);
    }
  }

  /** An 808 bass: a sine that drops onto its note and rings. */
  private bass808(c: Chain, note: number, when: number, dur: number, g: number): void {
    const ctx = this.ctx;
    const freq = mtof(note);
    const gain = ctx.createGain();
    const shape = ctx.createWaveShaper();
    shape.curve = this.softClip();
    gain.connect(shape).connect(c.input);
    const o = ctx.createOscillator();
    o.frequency.setValueAtTime(freq * 2.2, when);
    o.frequency.exponentialRampToValueAtTime(freq, when + 0.05);
    o.connect(gain);
    gain.gain.setValueAtTime(0.0001, when);
    gain.gain.linearRampToValueAtTime(g * 1.6, when + 0.003);
    gain.gain.setTargetAtTime(g * 0.9, when + 0.01, 0.4);
    gain.gain.setTargetAtTime(0.0001, when + dur, 0.08);
    o.start(when);
    o.stop(when + dur + 0.5);
    this.track(o, gain);
  }

  private clip: Float32Array<ArrayBuffer> | null = null;
  private softClip(): Float32Array<ArrayBuffer> {
    if (!this.clip) {
      const n = 1024;
      this.clip = new Float32Array(new ArrayBuffer(n * 4));
      for (let i = 0; i < n; i++) {
        const x = (i / (n - 1)) * 2 - 1;
        this.clip[i] = Math.tanh(1.8 * x) / Math.tanh(1.8);
      }
    }
    return this.clip;
  }

  /** An acid bass, after the TB-303: a saw through a resonant filter that
   * snaps shut; an accent opens it wider and louder. */
  private acid(c: Chain, note: number, when: number, dur: number, g: number, accent: boolean): void {
    const ctx = this.ctx;
    const freq = mtof(note);
    const end = when + dur;
    const filter = ctx.createBiquadFilter();
    filter.type = "lowpass";
    filter.Q.value = accent ? 16 : 11;
    const top = Math.min(freq * (accent ? 24 : 12), 12000);
    filter.frequency.setValueAtTime(top, when);
    filter.frequency.setTargetAtTime(freq * 1.5, when, accent ? 0.12 : 0.08);
    const gain = ctx.createGain();
    filter.connect(gain).connect(c.input);
    const level = g * (accent ? 0.75 : 0.5);
    gain.gain.setValueAtTime(0.0001, when);
    gain.gain.linearRampToValueAtTime(level, when + 0.003);
    gain.gain.setTargetAtTime(0.0001, end, 0.02);
    const o = ctx.createOscillator();
    o.type = "sawtooth";
    o.frequency.value = freq;
    o.connect(filter);
    o.start(when);
    o.stop(end + 0.15);
    this.track(o, gain);
  }

  /** A subtractive synth: detuned oscillators into a filter with its own
   * envelope, opening on the attack and closing over the decay. */
  private subtractive(
    c: Chain,
    note: number,
    when: number,
    dur: number,
    g: number,
    bright: number,
    o: {
      wave: OscillatorType;
      detune: number[];
      sub?: boolean;
      attack?: number;
      release?: number;
      /** The filter's resting cutoff, as a multiple of the note. */
      cutoff?: number;
      vibrato?: number;
      /** Brass: the filter swells open over the attack. */
      swell?: boolean;
      /** Spread the detuned oscillators across the stereo field. */
      spread?: boolean;
    },
  ): void {
    const ctx = this.ctx;
    const end = when + dur;
    const freq = mtof(note);
    const attack = Math.max(o.attack ?? c.attack, 0.002);
    const release = o.release ?? c.release;
    const gain = ctx.createGain();
    gain.connect(c.input);
    const filter = ctx.createBiquadFilter();
    filter.type = "lowpass";
    filter.Q.value = 2;
    const rest = Math.min(freq * (o.cutoff ?? 2 + 4 * bright), 18000);
    const peak = Math.min(freq * (3 + 10 * bright), 18000);
    if (o.swell) {
      filter.frequency.setValueAtTime(freq, when);
      filter.frequency.linearRampToValueAtTime(peak, when + attack * 2);
      filter.frequency.setTargetAtTime(rest * 1.5, when + attack * 2, 0.3);
    } else {
      filter.frequency.setValueAtTime(Math.min(freq * 2, 18000), when);
      filter.frequency.linearRampToValueAtTime(peak, when + Math.max(attack, 0.01));
      filter.frequency.setTargetAtTime(rest, when + attack, 0.25);
    }
    filter.connect(gain);
    const level = o.wave === "sine" || o.wave === "triangle" ? g : g * 0.42;
    gain.gain.setValueAtTime(0.0001, when);
    gain.gain.linearRampToValueAtTime(level, when + attack);
    gain.gain.setTargetAtTime(level * 0.75, when + attack, 0.15);
    gain.gain.setTargetAtTime(0.0001, end, Math.max(release / 3, 0.01));
    const stop = end + release * 2 + 0.05;
    let lfoGain: GainNode | null = null;
    if (o.vibrato) {
      const lfo = ctx.createOscillator();
      lfo.frequency.value = 5.5;
      lfoGain = ctx.createGain();
      lfoGain.gain.setValueAtTime(0, when);
      lfoGain.gain.linearRampToValueAtTime(o.vibrato * 1200, when + 0.3);
      lfo.connect(lfoGain);
      lfo.start(when);
      lfo.stop(stop);
      this.track(lfo, gain);
    }
    o.detune.forEach((cents, i) => {
      const osc = ctx.createOscillator();
      osc.type = o.wave;
      osc.frequency.value = freq;
      osc.detune.value = cents;
      if (lfoGain) lfoGain.connect(osc.detune);
      const og = ctx.createGain();
      og.gain.value = 1 / o.detune.length;
      if (o.spread && o.detune.length > 1) {
        const pan = ctx.createStereoPanner();
        pan.pan.value = (i / (o.detune.length - 1)) * 1.4 - 0.7;
        osc.connect(og).connect(pan).connect(filter);
      } else {
        osc.connect(og).connect(filter);
      }
      osc.start(when);
      osc.stop(stop);
      this.track(osc, gain);
    });
    // Basses get a sine an octave below for weight.
    if (o.sub ?? note < 52) {
      const sub = ctx.createOscillator();
      sub.frequency.value = freq / 2;
      const sg = ctx.createGain();
      sg.gain.value = 0.5;
      sub.connect(sg).connect(gain);
      sub.start(when);
      sub.stop(stop);
      this.track(sub, gain);
    }
  }

  /** Silences everything that sounds or is scheduled. */
  allOff(): void {
    const now = this.ctx.currentTime;
    for (const v of this.voices) {
      v.gain.gain.cancelScheduledValues(now);
      v.gain.gain.setTargetAtTime(0, now, 0.01);
      try {
        v.src.stop(now + 0.05);
      } catch {
        // Not started yet: stopping before the start is allowed to throw.
      }
    }
    this.voices.clear();
    // Manguebyte's parts: what is scheduled dropped, what sounds let go
    // without a click.
    for (const c of this.chains.values()) {
      const engine = c.mb?.engine;
      if (engine) {
        engine.cancel(0, now);
        engine.allNotesOff(0, now);
      }
    }
  }
}
