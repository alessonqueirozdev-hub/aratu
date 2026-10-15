// Plays a program bar by bar, a little ahead of time, and swaps in a new
// version of the piece at the next bar line.

import type { AratuEvent, Aratu, BarInfo, Program } from "./aratu";
import type { Engine } from "./audio";

/** How far ahead of the audio clock bars are scheduled, in seconds. */
const AHEAD = 0.3;

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
  /** A new version of the piece takes over from bar `bar`. */
  swapped(program: Program, bar: number): void;
}

export class Player {
  private program: Program | null = null;
  private pending: Program | null = null;
  /** For each live line, the bar it has sounded from. */
  private readonly anchors = new Map<string, number>();
  private nextBar = 1;
  private nextTime = 0;
  private seed = 0;
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
    await this.engine.ctx.resume();
    this.adopt(program, 1);
    this.anchors.clear();
    this.seed = (Math.random() * 2 ** 32) >>> 0;
    this.nextBar = program.pickup > 0 ? 0 : 1;
    this.nextTime = this.engine.ctx.currentTime + 0.12;
    this.timer = window.setInterval(() => this.tick(), 25);
    this.tick();
  }

  /** A new version of the piece: it takes over at the next bar line. */
  update(program: Program): void {
    if (!this.playing) {
      this.adopt(program, this.nextBar);
      return;
    }
    if (this.pending && this.pending !== program) this.aratu.release(this.pending.handle);
    this.pending = program;
  }

  stop(): void {
    if (this.timer !== undefined) window.clearInterval(this.timer);
    this.timer = undefined;
    this.engine.allOff();
  }

  private adopt(program: Program, bar: number): void {
    const old = this.program;
    this.program = program;
    if (old && old !== program) this.aratu.release(old.handle);
    // A live line keeps the bar it first sounded from; a new one starts here.
    const keys = new Set(program.live.map((l) => l.key));
    for (const k of [...this.anchors.keys()]) if (!keys.has(k)) this.anchors.delete(k);
    if (this.playing) for (const k of keys) if (!this.anchors.has(k)) this.anchors.set(k, bar);
    this.engine.configure(program);
  }

  private tick(): void {
    const ahead = this.engine.ctx.currentTime + AHEAD;
    for (let guard = 0; this.nextTime < ahead && guard < 16; guard++) this.scheduleBar();
  }

  private scheduleBar(): void {
    if (this.pending) {
      const p = this.pending;
      this.pending = null;
      this.adopt(p, this.nextBar);
      this.hooks.swapped(p, this.nextBar);
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
    const now = this.engine.ctx.currentTime;
    const list: Scheduled[] = [];
    for (const e of r.events) {
      const at = e.time + offset;
      if (at < now - 0.02) continue;
      this.engine.play(e, at);
      list.push({ event: e, at, handle: program.handle });
    }
    this.hooks.bar(bar, this.nextTime);
    this.hooks.events(list);
    this.nextTime += Math.max(bar.end - bar.start, 0);
    this.nextBar++;
  }
}
