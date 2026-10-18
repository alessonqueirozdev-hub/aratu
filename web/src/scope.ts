// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
// The sound drawn as it plays: the wave, held still on its rising edge, or
// its frequencies as bars, the low ones on the left, on the display in
// Play. Each canvas says how in its data: `mode`
// ("wave" or "bars"), `line` (the wave's thickness) and `gain` (how tall).
// The colour is the canvas's CSS colour.

export class Scope {
  private frame = 0;
  private wave: Float32Array<ArrayBuffer> | null = null;
  private bins: Uint8Array<ArrayBuffer> | null = null;

  constructor(
    private readonly canvases: () => HTMLCanvasElement[],
    private readonly analyser: () => AnalyserNode | null,
  ) {}

  start(): void {
    if (!this.frame) this.frame = requestAnimationFrame(this.draw);
  }

  stop(): void {
    cancelAnimationFrame(this.frame);
    this.frame = 0;
    for (const c of this.canvases()) c.getContext("2d")?.clearRect(0, 0, c.width, c.height);
  }

  private readonly draw = (): void => {
    this.frame = requestAnimationFrame(this.draw);
    const a = this.analyser();
    if (!a) return;
    for (const c of this.canvases()) {
      // Hidden canvases are skipped.
      if (c.getClientRects().length === 0 || c.clientWidth === 0) continue;
      const dpr = Math.min(2, window.devicePixelRatio || 1);
      const w = Math.round(c.clientWidth * dpr);
      const h = Math.round(c.clientHeight * dpr);
      if (c.width !== w || c.height !== h) {
        c.width = w;
        c.height = h;
      }
      const g = c.getContext("2d");
      if (!g) continue;
      g.clearRect(0, 0, w, h);
      const color = getComputedStyle(c).color;
      g.strokeStyle = color;
      g.fillStyle = color;
      if (c.dataset.mode === "bars") this.bars(a, g, w, h, dpr);
      else this.line(a, g, w, h, dpr, Number(c.dataset.line ?? 1.5), Number(c.dataset.gain ?? 1));
    }
  };

  private line(a: AnalyserNode, g: CanvasRenderingContext2D, w: number, h: number, dpr: number, width: number, gain: number): void {
    if (this.wave?.length !== a.fftSize) this.wave = new Float32Array(a.fftSize);
    const data = this.wave;
    a.getFloatTimeDomainData(data);
    // Held on the first rising zero crossing, so a steady tone stands still.
    const span = Math.floor(data.length / 2);
    let start = 0;
    for (let i = 1; i < span; i++) {
      if (data[i - 1] < 0 && data[i] >= 0) {
        start = i;
        break;
      }
    }
    g.lineWidth = width * dpr;
    g.lineJoin = "round";
    g.lineCap = "round";
    g.beginPath();
    for (let x = 0; x <= w; x += dpr) {
      const v = Math.max(-1, Math.min(1, data[start + Math.floor((x / w) * (span - 1))] * gain));
      const y = h / 2 - v * (h / 2) * 0.9;
      if (x === 0) g.moveTo(x, y);
      else g.lineTo(x, y);
    }
    g.stroke();
  }

  private bars(a: AnalyserNode, g: CanvasRenderingContext2D, w: number, h: number, dpr: number): void {
    if (this.bins?.length !== a.frequencyBinCount) this.bins = new Uint8Array(a.frequencyBinCount);
    const bins = this.bins;
    a.getByteFrequencyData(bins);
    // From 30 Hz to 16 kHz on a log scale, as an equalizer shows them.
    const n = Math.max(8, Math.floor(w / (7 * dpr)));
    const nyquist = a.context.sampleRate / 2;
    const step = w / n;
    for (let i = 0; i < n; i++) {
      const f0 = 30 * Math.pow(16000 / 30, i / n);
      const f1 = 30 * Math.pow(16000 / 30, (i + 1) / n);
      const b0 = Math.floor((f0 / nyquist) * bins.length);
      const b1 = Math.max(b0 + 1, Math.floor((f1 / nyquist) * bins.length));
      let peak = 0;
      for (let b = b0; b < b1 && b < bins.length; b++) peak = Math.max(peak, bins[b]);
      const bh = Math.max(dpr, (peak / 255) * h);
      g.fillRect(i * step + dpr, h - bh, Math.max(dpr, step - 2 * dpr), bh);
    }
  }
}
