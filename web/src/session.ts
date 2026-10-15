// One piece of Aratu being written and heard: an editor, its diagnostics and
// knobs, a player, and the symbols lit up as they sound. The playground has
// one; every example and exercise of the book has its own. Only one plays
// at a time.

import { ChangeSet } from "@codemirror/state";
import type { EditorView } from "@codemirror/view";
import type { Aratu, BarInfo, Lang, Program } from "./aratu";
import type { Engine } from "./audio";
import { createEditor, showDiagnostics, showPlaying } from "./editor";
import { knobs, showKnobs } from "./knobs";
import { Player, type Scheduled } from "./player";

export interface SessionOptions {
  parent: HTMLElement;
  text: string;
  aratu: Aratu;
  /** The shared sound engine, made on the first play (browsers ask for a click). */
  engine: () => Engine;
  lang: () => Lang;
  /** A smaller editor, for the book. */
  compact?: boolean;
  onCompile?(program: Program): void;
  onChange?(text: string): void;
  onBar?(info: BarInfo, at: number): void;
  onSwap?(bar: number, beat: number): void;
  onPlaying?(playing: boolean): void;
}

export class Session {
  /** The session that is sounding, if any. */
  static active: Session | null = null;

  readonly view: EditorView;
  /** The last compiled version, with errors or not. */
  latest: Program | null = null;
  private player: Player | null = null;
  private ready: Program | null = null;
  private broken: Program | null = null;
  private scheduled: Scheduled[] = [];
  private readonly editsSince = new Map<number, ChangeSet>();
  private timer: number | undefined;
  private frame = 0;
  private shown = "";

  constructor(private readonly o: SessionOptions) {
    this.view = createEditor(
      o.parent,
      o.text,
      {
        change: (text, changes) => {
          for (const [h, cs] of this.editsSince) this.editsSince.set(h, cs.compose(changes));
          for (const s of this.scheduled) {
            s.event.from = changes.mapPos(s.event.from, 1);
            s.event.to = changes.mapPos(s.event.to, -1);
          }
          o.onChange?.(text);
          window.clearTimeout(this.timer);
          this.timer = window.setTimeout(() => this.compile(), 250);
        },
        run: () => void this.play(),
        stop: () => this.stop(),
      },
      [knobs],
      o.compact,
    );
  }

  get text(): string {
    return this.view.state.doc.toString();
  }

  set text(t: string) {
    this.view.dispatch({ changes: { from: 0, to: this.view.state.doc.length, insert: t } });
    window.clearTimeout(this.timer);
    this.compile();
  }

  get playing(): boolean {
    return this.player?.playing ?? false;
  }

  /** Reads the text again, shows its problems and, while playing, hands the
   * new version to the player for the next bar line. */
  compile(): Program {
    window.clearTimeout(this.timer);
    const { aratu } = this.o;
    const p = aratu.compile(this.text, this.o.lang());
    this.editsSince.set(p.handle, ChangeSet.empty(this.view.state.doc.length));
    for (const h of this.editsSince.keys()) {
      if (this.editsSince.size <= 8) break;
      this.editsSince.delete(h);
    }
    if (this.broken) aratu.release(this.broken.handle);
    this.broken = null;
    this.latest = p;
    showDiagnostics(this.view, p.diagnostics);
    showKnobs(this.view, p.controls, (control, value) => {
      // Parameters set at a bar, and the tempo, come in with the text.
      if (control.timed || control.id === "tempo" || !this.playing) return;
      this.o.engine().setNow(control.part, control.id, value);
    });
    this.o.onCompile?.(p);
    if (!p.ok) {
      this.broken = p;
      return p;
    }
    if (this.player?.playing) {
      this.player.update(p);
    } else {
      if (this.ready) aratu.release(this.ready.handle);
      this.ready = p;
    }
    return p;
  }

  async play(): Promise<void> {
    const p = this.compile();
    if (this.playing || !p.ok) return;
    if (Session.active && Session.active !== this) Session.active.stop();
    Session.active = this;
    const engine = this.o.engine();
    if (!this.player) {
      this.player = new Player(this.o.aratu, engine, {
        bar: (info, at) => this.o.onBar?.(info, at),
        events: (list) => {
          for (const s of list) {
            const cs = this.editsSince.get(s.handle);
            if (cs && !cs.empty) {
              s.event.from = cs.mapPos(s.event.from, 1);
              s.event.to = cs.mapPos(s.event.to, -1);
            }
          }
          this.scheduled.push(...list);
        },
        cut: (bar, beat) => {
          this.scheduled = this.scheduled.filter(
            (s) => s.event.bar < bar || (s.event.bar === bar && s.event.beat < beat),
          );
        },
        swapped: (_p, bar, beat) => this.o.onSwap?.(bar, beat),
      });
    }
    this.ready = null;
    await this.player.play(p);
    this.o.onPlaying?.(true);
    const tick = () => {
      if (!this.playing) return;
      const now = engine.ctx.currentTime;
      this.scheduled = this.scheduled.filter((s) => s.at + Math.max(s.event.dur, 0.12) > now - 0.05);
      const spans = this.scheduled.filter((s) => s.at <= now).map((s) => ({ from: s.event.from, to: s.event.to }));
      const key = spans.map((s) => `${s.from}:${s.to}`).join(",");
      if (key !== this.shown) {
        this.shown = key;
        showPlaying(this.view, spans);
      }
      this.frame = requestAnimationFrame(tick);
    };
    this.frame = requestAnimationFrame(tick);
  }

  stop(): void {
    const was = this.playing;
    this.player?.stop();
    cancelAnimationFrame(this.frame);
    this.scheduled = [];
    this.shown = "";
    showPlaying(this.view, []);
    if (Session.active === this) Session.active = null;
    if (was) this.o.onPlaying?.(false);
  }

  destroy(): void {
    this.stop();
    window.clearTimeout(this.timer);
    this.view.destroy();
  }
}
