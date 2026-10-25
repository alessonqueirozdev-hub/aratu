// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
// The display of where the piece is: the bar's beats as dots, its meter, a
// line that crosses it in the time of a bar. The editor's and the player's.

/** The beats of the bar as dots: the first bigger, the one now lit. */
export function beatPips(el: HTMLElement, beats: number, now: number): void {
  const n = Math.max(1, Math.min(beats, 16));
  while (el.childElementCount > n) el.lastElementChild?.remove();
  while (el.childElementCount < n) el.append(document.createElement("i"));
  [...el.children].forEach((dot, i) => {
    dot.classList.toggle("now", i === now);
    dot.classList.toggle("hit", i === now);
  });
}

/** The meter stacked, as a score writes it: 6 over 8. */
export function showMeter(el: HTMLElement, meter: string): void {
  const m = /^(\d+)\/(\d+)$/.exec(meter);
  if (!m) {
    el.textContent = meter;
    return;
  }
  const sig = document.createElement("span");
  sig.className = "sig";
  for (const part of [m[1], m[2]]) {
    const s = document.createElement("span");
    s.textContent = part;
    sig.append(s);
  }
  el.replaceChildren(sig);
}

/** A line that crosses the display in the time of one bar. */
export function runProgress(el: HTMLElement, seconds: number | null): void {
  el.classList.remove("run");
  if (seconds === null) return;
  el.style.setProperty("--bar-len", `${seconds}s`);
  void el.offsetWidth;
  el.classList.add("run");
}
