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
  /** The open hi-hat ringing in each part, for a closed one to choke. */
  private readonly openHats = new Map<Chain, Voice>();
  /** The voices made by the event being played, for `play` to return. */
  private made: Voice[] = [];

  constructor() {
    this.ctx = new AudioContext({ latencyHint: "interactive" });
    const ctx = this.ctx;
    // The master: a gentle compressor glues the parts, then a little gain.
    // It is slow and light enough to let an accent through.
    const comp = ctx.createDynamicsCompressor();
    comp.threshold.value = -12;
    comp.knee.value = 6;
    comp.ratio.value = 2.5;
    comp.attack.value = 0.02;
    comp.release.value = 0.15;
    const out = ctx.createGain();
    out.gain.value = 1.25;
    comp.connect(out).connect(ctx.destination);
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
    // Loudness grows faster than velocity, so the step from mf to an
    // accent is heard; mf itself stays where it was.
    const g = Math.pow(e.vel / 127, 2) * 0.72;
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
  private noiseHit(dest: AudioNode, when: number, type: BiquadFilterType, freq: number, q: number, peak: number, decay: number): Voice {
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

  private drum(c: Chain, name: string, when: number, g: number): void {
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
        const open = this.openHats.get(c);
        if (open) {
          open.gain.gain.cancelScheduledValues(when);
          open.gain.gain.setTargetAtTime(0.0001, when, 0.006);
          this.openHats.delete(c);
        }
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

  /** A plucked string (Karplus-Strong), brighter when plucked harder. */
  private pluckBuffer(note: number, bright: number): AudioBuffer {
    const level = Math.round(Math.min(Math.max(bright, 0), 1) * 3);
    const key = note * 4 + level;
    const cached = this.plucks.get(key);
    if (cached) return cached;
    const rate = this.ctx.sampleRate;
    const len = Math.floor(rate * 3);
    const buf = this.ctx.createBuffer(1, len, rate);
    const d = buf.getChannelData(0);
    const period = Math.max(2, Math.round(rate / mtof(note)));
    const ring = new Float32Array(period);
    // A softer pluck starts from smoother noise.
    const smooth = [0.25, 0.45, 0.7, 0.95][level];
    let s = 0;
    for (let i = 0; i < period; i++) {
      s += smooth * (Math.random() * 2 - 1 - s);
      ring[i] = s;
    }
    const loss = 0.9965 + Math.min(0.003, period / 500000);
    let idx = 0;
    let body = 0;
    for (let i = 0; i < len; i++) {
      const next = (idx + 1) % period;
      const v = ring[idx];
      ring[idx] = loss * (0.5 * v + 0.5 * ring[next]);
      // A gentle body low-pass.
      body += 0.55 * (v - body);
      d[i] = body * 1.4;
      idx = next;
    }
    this.plucks.set(key, buf);
    return buf;
  }

  private note(c: Chain, note: number, when: number, dur: number, g: number): void {
    const ctx = this.ctx;
    const end = when + dur;
    const freq = mtof(note);
    const bright = Math.min(g / 0.6, 1);
    const gain = ctx.createGain();
    gain.connect(c.input);

    if (c.wave === "pluck") {
      const src = ctx.createBufferSource();
      src.buffer = this.pluckBuffer(note, bright);
      gain.gain.setValueAtTime(g * 1.3, when);
      gain.gain.setTargetAtTime(0.0001, end, 0.06);
      src.connect(gain);
      src.start(when);
      src.stop(end + 0.4);
      this.track(src, gain);
      return;
    }

    if (c.wave === "keys") {
      // An electric piano: a sine, its octave fading fast, and a tine.
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
      return;
    }

    // A subtractive synth: detuned oscillators into a filter with its own
    // envelope, opening on the attack and closing over the decay.
    const filter = ctx.createBiquadFilter();
    filter.type = "lowpass";
    filter.Q.value = 2;
    const base = Math.min(freq * 2, 18000);
    const peak = Math.min(freq * (3 + 10 * bright), 18000);
    filter.frequency.setValueAtTime(base, when);
    filter.frequency.linearRampToValueAtTime(peak, when + Math.max(c.attack, 0.01));
    filter.frequency.setTargetAtTime(Math.min(freq * (2 + 4 * bright), 18000), when + c.attack, 0.25);
    filter.connect(gain);
    const level = c.wave === "sine" || c.wave === "triangle" ? g : g * 0.42;
    gain.gain.setValueAtTime(0.0001, when);
    gain.gain.linearRampToValueAtTime(level, when + c.attack);
    gain.gain.setTargetAtTime(level * 0.75, when + c.attack, 0.15);
    gain.gain.setTargetAtTime(0.0001, end, Math.max(c.release / 3, 0.01));
    const stop = end + c.release * 2 + 0.05;
    const voices = c.wave === "sine" ? [0] : [-7, 0, 7];
    for (const cents of voices) {
      const o = ctx.createOscillator();
      o.type = c.wave as OscillatorType;
      o.frequency.value = freq;
      o.detune.value = cents;
      const og = ctx.createGain();
      og.gain.value = 1 / voices.length;
      o.connect(og).connect(filter);
      o.start(when);
      o.stop(stop);
      this.track(o, gain);
    }
    // Basses get a sine an octave below for weight.
    if (note < 52) {
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
  }
}
