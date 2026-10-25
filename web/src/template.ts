// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
// The editor's markup, made for each editor mounted: no ids (a page may
// hold several), the icons drawn in place, the words filled in by
// `data-word` and `data-title` (see app.ts), and only the parts its
// features ask for.

/** The icons, drawn in a 24 × 24 box. */
const ICONS: Record<string, string> = {
  play: '<path d="M8 5.2v13.6L18.8 12z" fill="currentColor" />',
  stop: '<rect x="6.5" y="6.5" width="11" height="11" rx="1.2" fill="currentColor" />',
  rec: '<circle cx="12" cy="12" r="5.6" fill="currentColor" />',
  stage:
    '<g fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="4" width="18" height="12" rx="1.5" /><path d="M12 16v4M8 20h8M7 8.5h6M7 11.5h9" /></g>',
  share:
    '<g fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><path d="M10 14a4 4 0 0 0 5.66 0l3-3a4 4 0 0 0-5.66-5.66l-1 1" /><path d="M14 10a4 4 0 0 0-5.66 0l-3 3a4 4 0 0 0 5.66 5.66l1-1" /></g>',
  cols: '<g fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="4" width="18" height="16" rx="1.5" /><path d="M12 4v16M6 8h3M6 11h3M6 14h2M15 8h3M15 11h3" /></g>',
  fit: '<path d="M4 9V4h5M20 9V4h-5M4 15v5h5M20 15v5h-5" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round" />',
  map: '<g fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"><path d="M3 7h18M3 12h18M3 17h18" /><circle cx="7" cy="7" r="1.7" fill="currentColor" /><circle cx="15" cy="12" r="1.7" fill="currentColor" /><circle cx="10" cy="17" r="1.7" fill="currentColor" /></g>',
  exit: '<path d="M6 6l12 12M18 6L6 18" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" />',
  follow:
    '<g fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" stroke-linejoin="round"><path d="M3 6h10M3 10h7M3 14h10M3 18h6" /><path d="M18 6v11M14.5 13.5 18 17l3.5-3.5" /></g>',
  daw: '<g fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round"><path d="M6 4v16M12 4v16M18 4v16" /><rect x="4" y="13" width="4" height="3" rx="1" fill="currentColor" /><rect x="10" y="7" width="4" height="3" rx="1" fill="currentColor" /><rect x="16" y="11" width="4" height="3" rx="1" fill="currentColor" /></g>',
  down: '<path d="M6 9l6 6 6-6" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />',
};

export function icon(name: keyof typeof ICONS): string {
  return `<svg class="i" viewBox="0 0 24 24" aria-hidden="true">${ICONS[name]}</svg>`;
}

/** What an editor shows. */
export interface Parts {
  daw: boolean;
  stage: boolean;
  book: boolean;
  share: boolean;
  record: boolean;
  scenes: boolean;
  examples: boolean;
}

const when = (on: boolean, html: string): string => (on ? html : "");

function playView(p: Parts): string {
  return `
<section class="view play-view" data-view="play" hidden>
  <div class="toolbar">
    <div class="transport">
      <button type="button" data-ref="play" class="btn btn-play big" data-title="play">${icon("play")}<span data-word="play"></span></button>
      <button type="button" data-ref="stop" class="btn big btn-stop" data-title="stop">${icon("stop")}<span data-word="stop"></span></button>
      ${when(
        p.record,
        `<button type="button" data-ref="record" class="btn big record" data-title="record">${icon("rec")}<span data-ref="record-word" data-word="record"></span></button>`,
      )}
    </div>
    <div class="status" data-ref="status">
      <span class="stat stat-bar"><b data-word="bar"></b><span data-ref="bar">–</span></span>
      <span data-ref="beat" class="beat"></span>
      <span class="stat stat-tempo"><b data-word="tempo"></b><span data-ref="bpm">–</span></span>
      <span class="stat stat-meter"><b data-word="meter"></b><span data-ref="meter">–</span></span>
      <canvas data-ref="scope" class="scope" data-mode="wave" data-title="scopeHint"></canvas>
      <span data-ref="bar-progress" class="bar-progress"></span>
    </div>
    <div class="tools">
      <button type="button" data-ref="follow" class="btn follow" data-title="followHint" aria-pressed="false">${icon("follow")}<span data-word="follow"></span></button>
      ${when(
        p.daw,
        `<button type="button" data-ref="daw" class="btn daw-open" data-title="dawHint" aria-pressed="false">${icon("daw")}<span data-word="daw"></span></button>`,
      )}
      ${when(
        p.stage,
        `<button type="button" data-ref="stage" class="btn stage-open" data-title="stage">${icon("stage")}<span data-word="stage"></span></button>`,
      )}
      ${when(
        p.share,
        `<button type="button" data-ref="share" class="btn" data-title="share">${icon("share")}<span data-word="share"></span></button>`,
      )}
      ${when(p.scenes, `<span data-ref="scenes" class="scenes"></span>`)}
      ${when(
        p.examples,
        `<label class="examples"><span data-word="examples"></span><select data-ref="examples"></select></label>`,
      )}
    </div>
  </div>
  <div class="work">
    <section class="editor sheet" data-ref="editor"></section>
    <section class="editor sheet" data-ref="editor-mirror" hidden></section>
    <aside class="side">
      <div class="side-tabs" role="tablist">
        <button type="button" data-ref="tab-messages" class="on" role="tab" data-word="messages"></button>
        <button type="button" data-ref="tab-sounds" role="tab" data-word="sounds"></button>
        <button type="button" data-ref="tab-rows" role="tab" data-word="rows"></button>
      </div>
      <div data-ref="side-messages">
        <ul data-ref="messages" class="messages"></ul>
        <p class="note" data-ref="note"></p>
      </div>
      <div data-ref="side-sounds" class="sounds" hidden></div>
      <div data-ref="side-rows" class="rows" hidden></div>
    </aside>
  </div>
  <section class="rhythm sheet">
    <div class="rhythm-head">
      <h2 data-word="rhythmMap"></h2>
      <div class="seg quantum" data-ref="quantum" role="group">
        <span class="quantum-label" data-word="grid"></span><button type="button" data-q="1" data-word="gridAuto"></button><button type="button" data-q="2">♪ <span data-word="eighths"></span></button><button type="button" data-q="4">♬ <span data-word="sixteenths"></span></button><button type="button" data-q="3">3 <span data-word="triplets"></span></button>
      </div>
      <p data-word="rhythmHelp"></p>
    </div>
    <div data-ref="play-grid" class="rhythm-grid"></div>
  </section>
  <p class="hint" data-word="hint"></p>
  ${when(p.stage, stageBar(p))}
</section>`;
}

/** The stage's rail: where the piece is, always; the controls, which fade
 * while the mouse rests. */
function stageBar(p: Parts): string {
  return `
  <div class="stage-bar" data-ref="stage-bar">
    <div class="stage-hud">
      <span class="stage-count"><span data-ref="stage-bar-n">–</span><span data-ref="stage-beat" class="beat"></span></span>
      <span class="stage-tempo" data-ref="stage-bpm"></span>
    </div>
    <div class="stage-controls">
      <button type="button" data-ref="stage-play" class="btn btn-play" data-title="play">${icon("play")}</button>
      <button type="button" data-ref="stage-stop" class="btn" data-title="stop">${icon("stop")}</button>
      <span data-ref="stage-lists" class="stage-lists"></span>
      ${when(p.scenes, `<span data-ref="stage-scenes" class="scenes"></span>`)}
      <span class="stage-tools">
        <button type="button" data-ref="stage-smaller" class="btn font-step" data-title="fontSmaller">A<small>−</small></button>
        <button type="button" data-ref="stage-bigger" class="btn font-step" data-title="fontBigger">A<small>+</small></button>
        <button type="button" data-ref="stage-follow" class="btn" data-title="followHint" aria-pressed="false">${icon("follow")}<span data-word="follow"></span></button>
        <button type="button" data-ref="stage-split" class="btn" data-title="stageSplit">${icon("cols")}<span data-word="stageSplit"></span></button>
        <button type="button" data-ref="stage-fit" class="btn" data-title="stageFit">${icon("fit")}<span data-word="stageFit"></span></button>
        <button type="button" data-ref="stage-map" class="btn" data-title="rhythmMap">${icon("map")}<span data-word="rhythmMap"></span></button>
      </span>
    </div>
    <button type="button" data-ref="stage-exit" class="btn stage-exit" data-title="stageExit">${icon("exit")}<span data-word="stageExit"></span></button>
    <span data-ref="stage-progress" class="bar-progress"></span>
  </div>
  <p data-ref="stage-msg" class="stage-msg" hidden></p>`;
}

function bookView(kind: "learn" | "manual" | "inside"): string {
  const heading = kind === "learn" ? "lessons" : "chapters";
  return `
<section class="view book-view" data-view="${kind}" hidden>
  <aside class="toc">
    <button type="button" class="toc-toggle" aria-expanded="false"><span class="toc-kind" data-word="${heading}"></span><span class="toc-here"></span>${icon("down")}</button>
    <h2 data-word="${heading}"></h2>
    ${kind === "learn" ? `<p class="progress" data-ref="learn-progress"></p>` : ""}
    <ol data-ref="${kind}-toc"></ol>
  </aside>
  <article class="page" data-ref="${kind}-page"></article>
</section>`;
}

/** Everything an editor shows, for its root. */
export function editorHtml(p: Parts): string {
  return (
    playView(p) +
    (p.book ? bookView("learn") + bookView("manual") + bookView("inside") : "") +
    `<div data-ref="sound-status" class="sound-status" role="status" aria-live="polite" hidden></div>`
  );
}

/** A player's markup: play or stop, where the piece is, and what it says. */
export function playerHtml(): string {
  return `
<div class="player">
  <button type="button" data-ref="play" class="btn btn-play player-play" data-title="play">${icon("play")}</button>
  <span class="player-count"><span data-ref="bar">–</span><span data-ref="beat" class="beat"></span></span>
  <span class="player-tempo" data-ref="bpm"></span>
  <span data-ref="bar-progress" class="bar-progress"></span>
</div>
<p class="player-note" data-ref="note" hidden></p>`;
}
