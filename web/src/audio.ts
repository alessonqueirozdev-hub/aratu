// Simple built-in sounds on the Web Audio API, until Manguebyte runs in the
// browser: synthesized drums, a subtractive synth, a keyboard and a plucked
// string (Karplus-Strong, rendered once per note into a buffer). Each part
// has its own chain: high-pass, low-pass, volume, pan, and sends to a delay
// and a reverb.

import type { AratuEvent, ParamValue, PartInfo, Program } from "./aratu";

const mtof = (n: number) => 440 * Math.pow(2, (n - 69) / 12);

type Wave = OscillatorType | "pluck" | "keys";

/** One sounding source and the gain that shapes it. */
export interface Voice {
  src: AudioScheduledSourceNode;
  gain: GainNode;
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
  wave: Wave;
  attack: number;
  release: number;
}

export class Engine {
  readonly ctx: AudioContext;
  private readonly master: GainNode;
  private readonly reverb: ConvolverNode;
  private readonly noise: AudioBuffer;
  private readonly chains = new Map<string, Chain>();
  private readonly plucks = new Map<number, AudioBuffer>();
  private readonly voices = new Set<Voice>();
  /** The voices made by the event being played, for `play` to return. */
  private made: Voice[] = [];

  constructor() {
    this.ctx = new AudioContext({ latencyHint: "interactive" });
    const ctx = this.ctx;
    const comp = ctx.createDynamicsCompressor();
    comp.threshold.value = -10;
    comp.ratio.value = 4;
    comp.connect(ctx.destination);
    this.master = ctx.createGain();
    this.master.gain.value = 0.7;
    this.master.connect(comp);

    this.reverb = ctx.createConvolver();
    this.reverb.buffer = this.impulse(2.4);
    this.reverb.connect(this.master);

    const len = ctx.sampleRate;
    this.noise = ctx.createBuffer(1, len, ctx.sampleRate);
    const d = this.noise.getChannelData(0);
    for (let i = 0; i < len; i++) d[i] = Math.random() * 2 - 1;
  }

  private impulse(seconds: number): AudioBuffer {
    const rate = this.ctx.sampleRate;
    const len = Math.floor(rate * seconds);
    const buf = this.ctx.createBuffer(2, len, rate);
    for (let c = 0; c < 2; c++) {
      const d = buf.getChannelData(c);
      for (let i = 0; i < len; i++) d[i] = (Math.random() * 2 - 1) * Math.pow(1 - i / len, 3);
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
      wave: "triangle",
      attack: 0.005,
      release: 0.15,
    };
    this.chains.set(part, c);
    return c;
  }

  /** Sets up every part of a program: its sound, its chain, its numbers. */
  configure(program: Program): void {
    for (const part of program.parts) this.configurePart(part, program.params);
  }

  private configurePart(part: PartInfo, globals: ParamValue[]): void {
    const c = this.chain(part.name);
    c.instrument = part.instrument;
    const head = part.chain[0] ?? "";
    const named: Record<string, Wave> = {
      "synth.saw": "sawtooth",
      "synth.sawtooth": "sawtooth",
      "synth.square": "square",
      "synth.sine": "sine",
      "synth.triangle": "triangle",
      "synth.pluck": "pluck",
      pluck: "pluck",
      keys: "keys",
      piano: "keys",
    };
    const isBass = /bass|baixo/i.test(part.name);
    c.wave =
      named[head] ??
      (part.instrument === "guitar" ? "pluck" : part.instrument === "keys" ? "keys" : isBass ? "sawtooth" : "triangle");
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
    for (const p of [...defaults, ...globals, ...part.params]) this.setParam(c, p.id, p.value, now);
  }

  private setParam(c: Chain, id: string, v: number, when: number): void {
    const set = (p: AudioParam, value: number): void => {
      p.setTargetAtTime(value, when, 0.015);
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

  /** Schedules an event; returns the voices it made, for `cancel`. */
  play(e: AratuEvent, when: number): Voice[] {
    this.made = [];
    if (e.param !== undefined && e.value !== undefined) {
      const targets = e.part === "all" ? [...this.chains.values()] : [this.chain(e.part)];
      for (const c of targets) this.setParam(c, e.param, e.value, when);
      return this.made;
    }
    const c = this.chain(e.part);
    const g = Math.pow(e.vel / 127, 1.6) * 0.6;
    if (e.drum) this.drum(c, e.drum, when, g);
    else if (e.note !== undefined) this.note(c, e.note, when, Math.max(e.dur, 0.02), g);
    return this.made;
  }

  /** Silences voices scheduled but not yet sounding, as when a version of
   * the piece is replaced inside a bar. */
  cancel(voices: Voice[], from: number): void {
    for (const v of voices) {
      if (!this.voices.delete(v)) continue;
      v.gain.disconnect();
      try {
        v.src.stop(from);
      } catch {
        // Already stopped: nothing more to do.
      }
    }
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

  private env(gain: GainNode, when: number, peak: number, attack: number, decay: number): void {
    gain.gain.setValueAtTime(0.0001, when);
    gain.gain.linearRampToValueAtTime(peak, when + attack);
    gain.gain.exponentialRampToValueAtTime(0.0001, when + attack + decay);
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

  private drum(c: Chain, name: string, when: number, g: number): void {
    const ctx = this.ctx;
    const gain = ctx.createGain();
    gain.connect(c.input);
    switch (name) {
      case "kick": {
        const o = ctx.createOscillator();
        o.frequency.setValueAtTime(150, when);
        o.frequency.exponentialRampToValueAtTime(45, when + 0.12);
        this.env(gain, when, g * 1.6, 0.002, 0.35);
        o.connect(gain);
        o.start(when);
        o.stop(when + 0.4);
        this.track(o, gain);
        return;
      }
      case "snare": {
        const n = this.noiseSource(when, 0.25);
        const hp = ctx.createBiquadFilter();
        hp.type = "highpass";
        hp.frequency.value = 1500;
        n.connect(hp).connect(gain);
        this.env(gain, when, g, 0.001, 0.18);
        const o = ctx.createOscillator();
        o.type = "triangle";
        o.frequency.value = 185;
        const og = ctx.createGain();
        this.env(og, when, g * 0.7, 0.001, 0.09);
        o.connect(og).connect(c.input);
        o.start(when);
        o.stop(when + 0.15);
        this.track(n, gain);
        this.track(o, og);
        return;
      }
      default: {
        const decay = { "closed-hat": 0.05, "open-hat": 0.35, ride: 0.9, crash: 1.6 }[name] ?? 0.1;
        const freq = { "closed-hat": 7000, "open-hat": 7000, ride: 5000, crash: 3500 }[name] ?? 6000;
        const level = { "closed-hat": 0.45, "open-hat": 0.4, ride: 0.3, crash: 0.5 }[name] ?? 0.4;
        const n = this.noiseSource(when, decay + 0.05);
        const f = ctx.createBiquadFilter();
        f.type = name === "ride" ? "bandpass" : "highpass";
        f.frequency.value = freq;
        n.connect(f).connect(gain);
        this.env(gain, when, g * level, 0.001, decay);
        this.track(n, gain);
      }
    }
  }

  private pluckBuffer(note: number): AudioBuffer {
    const cached = this.plucks.get(note);
    if (cached) return cached;
    const rate = this.ctx.sampleRate;
    const len = Math.floor(rate * 2.5);
    const buf = this.ctx.createBuffer(1, len, rate);
    const d = buf.getChannelData(0);
    const period = Math.max(2, Math.round(rate / mtof(note)));
    const ring = new Float32Array(period);
    for (let i = 0; i < period; i++) ring[i] = Math.random() * 2 - 1;
    // Brighter low strings decay slower; the averaging filter is the string.
    const loss = 0.996 + Math.min(0.0035, period / 400000);
    let idx = 0;
    for (let i = 0; i < len; i++) {
      const next = (idx + 1) % period;
      const v = ring[idx];
      ring[idx] = loss * 0.5 * (v + ring[next]);
      d[i] = v;
      idx = next;
    }
    this.plucks.set(note, buf);
    return buf;
  }

  private note(c: Chain, note: number, when: number, dur: number, g: number): void {
    const ctx = this.ctx;
    const gain = ctx.createGain();
    gain.connect(c.input);
    const end = when + dur;
    if (c.wave === "pluck") {
      const src = ctx.createBufferSource();
      src.buffer = this.pluckBuffer(note);
      gain.gain.setValueAtTime(g * 1.2, when);
      gain.gain.setTargetAtTime(0.0001, end, 0.05);
      src.connect(gain);
      src.start(when);
      src.stop(end + 0.3);
      this.track(src, gain);
      return;
    }
    if (c.wave === "keys") {
      const o1 = ctx.createOscillator();
      o1.type = "triangle";
      o1.frequency.value = mtof(note);
      const o2 = ctx.createOscillator();
      o2.frequency.value = mtof(note) * 2;
      const g2 = ctx.createGain();
      g2.gain.value = 0.25;
      o1.connect(gain);
      o2.connect(g2).connect(gain);
      gain.gain.setValueAtTime(0.0001, when);
      gain.gain.linearRampToValueAtTime(g, when + 0.004);
      gain.gain.setTargetAtTime(g * 0.25, when + 0.004, 0.5);
      gain.gain.setTargetAtTime(0.0001, end, 0.08);
      for (const o of [o1, o2]) {
        o.start(when);
        o.stop(end + 0.5);
      }
      this.track(o1, gain);
      return;
    }
    const o = ctx.createOscillator();
    o.type = c.wave;
    o.frequency.value = mtof(note);
    const peak = c.wave === "sine" || c.wave === "triangle" ? g : g * 0.5;
    gain.gain.setValueAtTime(0.0001, when);
    gain.gain.linearRampToValueAtTime(peak, when + c.attack);
    gain.gain.setTargetAtTime(peak * 0.7, when + c.attack, 0.1);
    gain.gain.setTargetAtTime(0.0001, end, c.release / 3);
    o.connect(gain);
    o.start(when);
    o.stop(end + c.release * 2 + 0.05);
    this.track(o, gain);
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
  }
}
