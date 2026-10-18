// One piece of Aratu being written and heard: an editor, its diagnostics and
// knobs, a player, and the symbols lit up as they sound. The playground has
// one; every example and exercise of the book has its own. Only one plays
// at a time.

import { ChangeSet } from "@codemirror/state";
import type { EditorView } from "@codemirror/view";
import type { Aratu, BarInfo, Lang, Program } from "./aratu";
import type { Engine } from "./audio";
import { createEditor, narrowScreen, setWrapping, showDiagnostics, showPlaying } from "./editor";
import { knobs, showKnobs, showMoving } from "./knobs";
import { Player, type Scheduled } from "./player";
import { type Heard, lendVisuals, showVisuals, startVisuals, stopVisuals, visuals } from "./visuals";

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
  /** The symbols lit as they sound, each time they change. */
  onLit?(spans: { from: number; to: number }[]): void;
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
  /** Whether a knob shows where a moving parameter is now. */
  private movingShown = false;
  /** A second view of the same text (the stage's second column). */
  mirror: EditorView | null = null;
  /** The bars scheduled and their events, kept a while after they sound,
   * for the drawings under the lines (`bass.wave`, `arp.roll`). */
  private heardBars: { info: BarInfo; at: number }[] = [];
  private heard: Heard[] = [];
  /** Set while a change is copied from one view to the other. */
  private syncing = false;

  constructor(private readonly o: SessionOptions) {
    this.view = createEditor(
      o.parent,
      o.text,
      {
        change: (text, changes) => {
          // Into the second view, unless it came from there.
          if (this.mirror && !this.syncing) {
            this.syncing = true;
            this.mirror.dispatch({ changes });
            this.syncing = false;
          }
          for (const [h, cs] of this.editsSince) this.editsSince.set(h, cs.compose(changes));
          for (const s of this.scheduled) {
            s.event.from = changes.mapPos(s.event.from, 1);
            s.event.to = changes.mapPos(s.event.to, -1);
            if (s.event.cfrom !== undefined && s.event.cto !== undefined) {
              s.event.cfrom = changes.mapPos(s.event.cfrom, 1);
              s.event.cto = changes.mapPos(s.event.cto, -1);
            }
          }
          o.onChange?.(text);
          window.clearTimeout(this.timer);
          this.timer = window.setTimeout(() => this.compile(), 250);
        },
        run: () => void this.play(),
        stop: () => this.stop(),
      },
      [knobs, visuals],
      o.compact,
    );
    lendVisuals(this.view, this.visualSource);
  }

  /** What the drawings under the lines draw from. */
  private readonly visualSource = {
    now: (): number | null => (this.playing ? this.o.engine().ctx.currentTime : null),
    sounding: (now: number) => {
      const bar = this.heardBars.find((b) => b.at <= now && now < b.at + b.info.end - b.info.start);
      return bar ? { info: bar.info, at: bar.at, events: this.heard } : null;
    },
    voice: (part: string): string => this.latest?.parts.find((p) => p.name === part)?.chain[0] ?? "",
  };

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
    const moved = (control: Program["controls"][number], value: number) => {
      // Parameters set at a bar, and the tempo, come in with the text.
      if (control.timed || control.id === "tempo" || !this.playing) return;
      this.o.engine().setNow(control.part, control.id, value);
    };
    for (const view of this.views()) {
      showDiagnostics(view, p.diagnostics);
      showKnobs(view, p.controls, moved);
      showVisuals(view, p.visuals ?? []);
    }
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
        bar: (info, at) => {
          this.heardBars.push({ info, at });
          this.o.onBar?.(info, at);
        },
        events: (list) => {
          for (const s of list) {
            const cs = this.editsSince.get(s.handle);
            if (cs && !cs.empty) {
              s.event.from = cs.mapPos(s.event.from, 1);
              s.event.to = cs.mapPos(s.event.to, -1);
              if (s.event.cfrom !== undefined && s.event.cto !== undefined) {
                s.event.cfrom = cs.mapPos(s.event.cfrom, 1);
                s.event.cto = cs.mapPos(s.event.cto, -1);
              }
            }
          }
          this.scheduled.push(...list);
          this.heard.push(...list);
        },
        cut: (bar, beat) => {
          this.scheduled = this.scheduled.filter(
            (s) => s.event.bar < bar || (s.event.bar === bar && s.event.beat < beat),
          );
          this.heard = this.heard.filter((s) => s.event.bar < bar || (s.event.bar === bar && s.event.beat < beat));
          this.heardBars = this.heardBars.filter((b) => b.info.bar < bar || (b.info.bar === bar && beat > 0));
        },
        swapped: (_p, bar, beat) => this.o.onSwap?.(bar, beat),
      });
    }
    this.ready = null;
    await this.player.play(p);
    this.o.onPlaying?.(true);
    startVisuals();
    const tick = () => {
      if (!this.playing) return;
      const now = engine.ctx.currentTime;
      this.scheduled = this.scheduled.filter((s) => s.at + Math.max(s.event.dur, 0.12) > now - 0.05);
      // A bar's events stay while the bar after it sounds.
      this.heardBars = this.heardBars.filter((b) => b.at + 2 * (b.info.end - b.info.start) > now);
      const oldest = this.heardBars[0]?.info.bar ?? 0;
      if (this.heard.length > 0 && this.heard[0].event.bar < oldest) {
        this.heard = this.heard.filter((s) => s.event.bar >= oldest);
      }
      // A strum lights its chord symbol too.
      const lit = this.scheduled.filter((s) => s.at <= now);
      const spans = lit.flatMap((s) => {
          const own = { from: s.event.from, to: s.event.to };
          return s.event.cfrom !== undefined && s.event.cto !== undefined
            ? [own, { from: s.event.cfrom, to: s.event.cto }]
            : [own];
        });
      const key = spans.map((s) => `${s.from}:${s.to}`).join(",");
      if (key !== this.shown) {
        this.shown = key;
        for (const view of this.views()) showPlaying(view, spans);
        // What sounds, for following the music: the notes, not the numbers
        // that move.
        this.o.onLit?.(lit.filter((s) => s.event.param === undefined).map((s) => ({ from: s.event.from, to: s.event.to })));
      }
      // Where each moving parameter is now, on its knobs.
      const moving = new Set((this.player?.current?.moving ?? []).map((m) => `${m.part}	${m.id}`));
      if (moving.size > 0 || this.movingShown) {
        const now = new Map<string, number>();
        for (const s of this.scheduled) {
          const e = s.event;
          if (s.at > engine.ctx.currentTime || e.param === undefined || e.value === undefined) continue;
          const k = `${e.part}	${e.param}`;
          if (moving.has(k)) now.set(k, e.value);
        }
        this.movingShown = now.size > 0;
        for (const view of this.views()) showMoving(view, now);
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
    this.heard = [];
    this.heardBars = [];
    if (was) stopVisuals();
    this.shown = "";
    for (const view of this.views()) showPlaying(view, []);
    for (const view of this.views()) showMoving(view, new Map());
    this.movingShown = false;
    if (Session.active === this) Session.active = null;
    if (was) this.o.onPlaying?.(false);
  }

  destroy(): void {
    this.stop();
    window.clearTimeout(this.timer);
    this.unmirror();
    this.view.destroy();
  }

  private views(): EditorView[] {
    return this.mirror ? [this.view, this.mirror] : [this.view];
  }

  /** Opens a second view of the text in `parent`: what is typed in either
   * is the same piece, and both show what sounds, the messages and the
   * knobs. Long lines wrap in both. */
  openMirror(parent: HTMLElement): EditorView {
    if (this.mirror) return this.mirror;
    const mirror = createEditor(
      parent,
      this.text,
      {
        change: (_text, changes) => {
          if (this.syncing) return;
          this.syncing = true;
          this.view.dispatch({ changes });
          this.syncing = false;
        },
        run: () => void this.play(),
        stop: () => this.stop(),
      },
      [knobs, visuals],
      this.o.compact,
    );
    lendVisuals(mirror, this.visualSource);
    this.mirror = mirror;
    setWrapping(this.view, true);
    setWrapping(mirror, true);
    if (this.latest) {
      showDiagnostics(mirror, this.latest.diagnostics);
      showVisuals(mirror, this.latest.visuals ?? []);
    }
    // Knobs come with the next compile; ask for it now.
    this.compile();
    return mirror;
  }

  unmirror(): void {
    if (!this.mirror) return;
    this.mirror.destroy();
    this.mirror = null;
    setWrapping(this.view, narrowScreen());
  }
}
