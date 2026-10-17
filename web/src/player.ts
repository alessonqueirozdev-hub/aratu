// Plays a program bar by bar, a little ahead of time, and swaps in a new
// version of the piece at the next bar line, or at the next beat when the
// new version brings a line written with `now:`.

import type { AratuEvent, Aratu, BarInfo, Program } from "./aratu";
import type { Engine, Voice } from "./audio";

/** How far ahead of the audio clock bars are scheduled, in seconds. */
const AHEAD = 0.3;
/** How soon a beat can be for a change to still come in on it, in seconds. */
const MARGIN = 0.05;

export interface Scheduled {
  event: AratuEvent;
  /** Audio-clock time the event sounds at. */
  at: number;
  /** The program it came from, for the editor to know the positions fit. */
  handle: number;
}

export interface PlayerHooks {
  /** A bar has been scheduled to start at `at` (audio clock). */
  bar(info: BarInfo, at: number): void;
  events(list: Scheduled[]): void;
  /** Events scheduled from beat `beat` of bar `bar` on will not sound. */
  cut(bar: number, beat: number): void;
  /** A new version of the piece takes over from beat `beat` (0 is the bar
   * line) of bar `bar`. */
  swapped(program: Program, bar: number, beat: number): void;
}

/** A bar on the way, with the audio-clock time its start sounds at. */
interface BarAt {
  info: BarInfo;
  at: number;
}

/** What an event scheduled not long ago made, to take back if need be. */
interface Sound {
  bar: number;
  beat: number;
  at: number;
  voices: Voice[];
}

export class Player {
  private program: Program | null = null;
  private pending: Program | null = null;
  /** For each live line, the bar it has sounded from. */
  private readonly anchors = new Map<string, number>();
  private nextBar = 1;
  private nextTime = 0;
  private bars: BarAt[] = [];
  private sounds: Sound[] = [];
  private seed = 0;
  /** Counts stops, so a start still waiting for the engine knows it was
   * stopped meanwhile. */
  private runs = 0;
  private timer: number | undefined;

  constructor(
    private readonly aratu: Aratu,
    private readonly engine: Engine,
    private readonly hooks: PlayerHooks,
  ) {}

  get playing(): boolean {
    return this.timer !== undefined;
  }

  get current(): Program | null {
    return this.program;
  }

  async play(program: Program): Promise<void> {
    this.stop();
    const run = this.runs;
    await this.engine.ctx.resume();
    this.adopt(program, 1);
    // The first bar on Manguebyte's voices, not the stand-in's while they
    // are being made.
    await this.engine.whenReady();
    if (run !== this.runs) return;
    // The lines there when playing starts sound from the first bar, and keep
    // it through the edits that follow.
    this.anchors.clear();
    for (const l of program.live) this.anchors.set(l.key, 1);
    this.seed = (Math.random() * 2 ** 32) >>> 0;
    this.nextBar = program.pickup > 0 ? 0 : 1;
    this.nextTime = this.engine.ctx.currentTime + 0.12;
    this.timer = window.setInterval(() => this.tick(), 25);
    this.tick();
  }

  /** A new version of the piece: it takes over at the next bar line, or at
   * the next beat if it brings a new line written with `now:`. */
  update(program: Program): void {
    if (!this.playing) {
      this.adopt(program, this.nextBar);
      return;
    }
    if (this.pending && this.pending !== program) this.aratu.release(this.pending.handle);
    this.pending = null;
    const now = program.live.some((l) => l.now && !this.anchors.has(l.key));
    if (!now || !this.swapAtBeat(program)) this.pending = program;
  }

  stop(): void {
    this.runs++;
    if (this.timer !== undefined) window.clearInterval(this.timer);
    this.timer = undefined;
    this.bars = [];
    this.sounds = [];
    this.engine.allOff();
  }

  /** Makes `program` the one playing. Its new live lines sound from `bar`;
   * those written with `now:` from `nowBar`. */
  private adopt(program: Program, bar: number, nowBar = bar): void {
    const old = this.program;
    this.program = program;
    if (old && old !== program) this.aratu.release(old.handle);
    // A live line keeps the bar it first sounded from; a new one starts here.
    const keys = new Set(program.live.map((l) => l.key));
    for (const k of [...this.anchors.keys()]) if (!keys.has(k)) this.anchors.delete(k);
    if (this.playing) {
      for (const l of program.live) if (!this.anchors.has(l.key)) this.anchors.set(l.key, l.now ? nowBar : bar);
    }
    this.engine.configure(program, this.playing);
  }

  /** Swaps `program` in at the next beat already scheduled: what the old
   * version scheduled from there on is taken back, and the new one plays the
   * rest of that bar. Returns false when the next beat is the next bar line
   * not yet scheduled, where the usual swap does the same. */
  private swapAtBeat(program: Program): boolean {
    const earliest = this.engine.ctx.currentTime + MARGIN;
    for (const b of this.bars) {
      const beat = b.info.beatTimes.findIndex((t) => b.at + t - b.info.start >= earliest);
      if (beat < 0) continue;
      const bar = b.info.bar;
      const at = b.at + b.info.beatTimes[beat] - b.info.start;

      const keep: Sound[] = [];
      for (const s of this.sounds) {
        if (s.bar > bar || (s.bar === bar && s.beat >= beat)) this.engine.cancel(s.voices, at);
        else keep.push(s);
      }
      this.sounds = keep;
      this.engine.cancelParams(at);
      this.hooks.cut(bar, beat);
      this.bars = this.bars.filter((x) => x.info.bar < bar);

      // A `now:` line belongs to this bar, from this beat; other new lines
      // still wait for the bar line.
      this.adopt(program, bar + 1, bar);
      const r = this.aratu.render(program.handle, bar, bar + 1, this.anchors, this.seed, beat);
      const info = r.bars[0];
      if (!info) return false;
      // The new version may have fewer beats here: then its bar is over.
      const offset = at - (info.beatTimes[beat] ?? info.end);
      this.bars.push({ info, at: info.start + offset });
      this.hooks.events(this.schedule(r.events, offset, program.handle));
      this.hooks.swapped(program, bar, beat);
      this.nextBar = bar + 1;
      this.nextTime = info.end + offset;
      this.tick();
      return true;
    }
    return false;
  }

  private tick(): void {
    const now = this.engine.ctx.currentTime;
    this.bars = this.bars.filter((b) => b.at + b.info.end - b.info.start > now);
    this.sounds = this.sounds.filter((s) => s.at > now - 0.1);
    const ahead = now + AHEAD;
    for (let guard = 0; this.nextTime < ahead && guard < 16; guard++) this.scheduleBar();
  }

  /** Plays events rendered in piece time, `offset` seconds later on the
   * audio clock. */
  private schedule(events: AratuEvent[], offset: number, handle: number): Scheduled[] {
    const now = this.engine.ctx.currentTime;
    const list: Scheduled[] = [];
    for (const e of events) {
      const at = e.time + offset;
      if (at < now - 0.02) continue;
      const voices = this.engine.play(e, at);
      if (voices.length > 0) this.sounds.push({ bar: e.bar, beat: e.beat, at, voices });
      list.push({ event: e, at, handle });
    }
    return list;
  }

  private scheduleBar(): void {
    if (this.pending) {
      const p = this.pending;
      this.pending = null;
      this.adopt(p, this.nextBar);
      this.hooks.swapped(p, this.nextBar, 0);
    }
    const program = this.program;
    if (!program) return;
    const r = this.aratu.render(program.handle, this.nextBar, this.nextBar + 1, this.anchors, this.seed);
    const bar = r.bars[0];
    if (!bar) {
      this.nextBar++;
      return;
    }
    const offset = this.nextTime - bar.start;
    const list = this.schedule(r.events, offset, program.handle);
    this.bars.push({ info: bar, at: this.nextTime });
    this.hooks.bar(bar, this.nextTime);
    this.engine.bar(bar, this.nextTime);
    this.hooks.events(list);
    this.nextTime += Math.max(bar.end - bar.start, 0);
    this.nextBar++;
  }
}
