// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
// Listens to sampled voices the way the editor plays them: on Manguebyte's
// engine, in a Chromium browser with its sound muted (--mute-audio). For
// each voice it loads voice.sfz, waits until the engine has measured it and
// plays a note soft (pp), the same note loud (ff) and the phrase the
// engine's loudness normalisation measures with (a melody, or on a kit the
// snare and a groove), recording what
// comes out. It does the same with the voice as it is in a commit (HEAD,
// or --before=<ref>), and prints both: how soft and loud differ in level
// and in brightness (the spectral centroid of their first 0.3 s), whether
// the voice is stereo, and the loudness of the phrase, which the
// normalisation should keep where it was. Run by hand after
// scripts/samples.mjs:
//
//   node scripts/voices.mjs [voice ...] [--before=<ref>]
//
// It fails if a voice does not load in Manguebyte, if the browser cannot
// decode the files index.json names for the stand-in, or if a voice with
// dynamic layers
// (more than one `lovel` in its voice.sfz) does not change its timbre from
// soft to loud, or plays its phrase more than 1.5 LU away from before.

import { execFileSync, spawn } from "node:child_process";
import { existsSync, mkdtempSync, readFileSync, rmSync } from "node:fs";
import { createServer } from "node:http";
import { tmpdir } from "node:os";
import { dirname, extname, join, normalize } from "node:path";
import { fileURLToPath } from "node:url";
import { browser } from "./og.mjs";

const web = join(dirname(fileURLToPath(import.meta.url)), "..");
const repo = join(web, "..");
const pub = join(web, "public");
const args = process.argv.slice(2);
const before = args.find((a) => a.startsWith("--before="))?.slice(9) ?? "HEAD";
const named = args.filter((a) => !a.startsWith("--"));
const voices =
  named.length > 0
    ? named
    : ["piano.grand", "piano.upright", "keys.harpsichord", "organ.pipe", "harp", "mallets.marimba", "mallets.vibraphone", "mallets.xylophone", "mallets.glockenspiel", "bells.tubular", "mallets.kalimba", "harmonica", "recorder.alto", "sax.tenor", "drums.acoustic"];

/** Whether an SFZ text is a drum kit: regions on keys, none retuned. */
const isKit = (sfz) => /\bkey=\d/.test(sfz) && !/pitch_keycenter=/.test(sfz);

/** The key a voice is tried on: middle C, or the middle of its range; on a
 * kit, the snare. */
function keyOf(sfz) {
  if (isKit(sfz)) return 38;
  const keys = [...sfz.matchAll(/pitch_keycenter=(\d+)/g)].map((m) => Number(m[1]));
  const [lo, hi] = [Math.min(...keys), Math.max(...keys)];
  return lo <= 60 && hi >= 60 ? 60 : Math.round((lo + hi) / 2);
}

const types = {
  ".html": "text/html; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".wasm": "application/wasm",
  ".json": "application/json",
  ".sfz": "text/plain; charset=utf-8",
  ".wav": "audio/wav",
  ".flac": "audio/flac",
  ".ogg": "audio/ogg",
};

const page = `<!doctype html><meta charset="utf-8"><title>voices</title><script type="module" src="/check.js"></script>`;

// Records the engine's output in blocks, from the frame it is told.
const tap = `
class Tap extends AudioWorkletProcessor {
  constructor() {
    super();
    this.from = Infinity;
    this.to = -Infinity;
    this.l = [];
    this.r = [];
    this.port.onmessage = (e) => { this.from = e.data.from; this.to = e.data.to; this.l = []; this.r = []; };
  }
  process(inputs) {
    const input = inputs[0];
    if (currentFrame >= this.from && currentFrame < this.to && input.length > 0) {
      this.l.push(input[0].slice());
      this.r.push((input[1] ?? input[0]).slice());
    }
    if (currentFrame >= this.to && this.l.length > 0) {
      this.port.postMessage({ l: this.l, r: this.r });
      this.l = [];
      this.r = [];
      this.to = -Infinity;
    }
    return true;
  }
}
registerProcessor("tap", Tap);
`;

const check = `
import { ManguebyteEngine } from "/manguebyte/engine.js";
const plan = ${"${PLAN}"};
const ctx = new AudioContext({ sampleRate: 48000 });
const results = [];
try {
  await ctx.resume();
  await ctx.audioWorklet.addModule("/tap.js");
  const tap = new AudioWorkletNode(ctx, "tap", { numberOfInputs: 1, numberOfOutputs: 1, outputChannelCount: [2] });
  tap.connect(ctx.destination);
  const module = await WebAssembly.compileStreaming(fetch("/manguebyte/manguebyte_web.wasm"));
  const engine = await ManguebyteEngine.create(ctx, { wasm: module, workletUrl: "/manguebyte/worklet.js" });
  engine.node.connect(tap);
  const record = (from, seconds) =>
    new Promise((resolve) => {
      tap.port.onmessage = (e) => {
        const join = (blocks) => {
          const out = new Float32Array(blocks.reduce((n, b) => n + b.length, 0));
          let at = 0;
          for (const b of blocks) { out.set(b, at); at += b.length; }
          return out;
        };
        resolve({ l: join(e.data.l), r: join(e.data.r) });
      };
      tap.port.postMessage({ from: Math.round(from * ctx.sampleRate), to: Math.round((from + seconds) * ctx.sampleRate) });
    });
  let part = 0;
  for (const job of plan) {
    part++;
    const result = { voice: job.voice, set: job.set };
    try {
      await engine.createPart(part, job.kit ? "sampler_kit" : "sampler", "none");
      const started = performance.now();
      const loading = engine.loadSfz(part, job.url);
      result.summary = await loading;
      result.playableMs = Math.round(performance.now() - started);
      result.makeUp = (await loading.normalized).makeUp;
      const k = job.key;
      const t0 = ctx.currentTime + 0.3;
      const taped = record(t0, 7.5);
      engine.noteOn(part, k, 33 / 127, t0);
      engine.noteOff(part, k, t0 + 1.2);
      engine.noteOn(part, k, 112 / 127, t0 + 1.5);
      engine.noteOff(part, k, t0 + 2.7);
      // The melody Manguebyte measures a sampler with (level.rs): around
      // middle C, at 120 beats a minute, held 0.9 of each note. A kit's is
      // a groove: the closed hat on every eighth, the kick on 1 and 3, the
      // snare on 2 and 4, for two bars.
      const melody = job.kit
        ? Array.from({ length: 16 }, (_, e) => [[e / 2, 0.5, 42], ...(e % 4 === 0 ? [[e / 2, 0.5, 36]] : e % 4 === 2 ? [[e / 2, 0.5, 38]] : [])]).flat()
        : [[0, 1, 60], [1, 0.5, 64], [1.5, 0.5, 67], [2, 1, 69], [3, 1, 67], [4, 0.5, 64], [4.5, 0.5, 62], [5, 0.5, 60], [5.5, 0.5, 62], [6, 1, 64], [7, 1, 60]];
      for (const [beat, beats, key] of melody) {
        engine.noteOn(part, key, 0.8, t0 + 3 + beat * 0.5);
        engine.noteOff(part, key, t0 + 3 + (beat + beats * 0.9) * 0.5);
      }
      const { l, r } = await taped;
      const both = new Float32Array(l.length + r.length);
      both.set(l);
      both.set(r, l.length);
      await fetch("/audio/" + (part - 1), { method: "POST", body: both });
      await engine.removePart(part);
      // The stand-in decodes what index.json names with the browser's own
      // decoder, when Manguebyte cannot play.
      const dir = job.url.replace(/voice\\.sfz$/, "");
      const index = await (await fetch(dir + "index.json")).json();
      const named = index.kind === "kit" ? Object.values(index.hits) : (index.notes ?? []).map((n) => n.file);
      if (named.length > 0) {
        let peak = 0;
        let channels = 0;
        for (const file of named) {
          const buffer = await ctx.decodeAudioData(await (await fetch(dir + file)).arrayBuffer());
          channels = buffer.numberOfChannels;
          for (let c = 0; c < channels; c++) for (const v of buffer.getChannelData(c)) peak = Math.max(peak, Math.abs(v));
        }
        result.standIn = { files: named.length, channels, peak };
      }
    } catch (error) {
      result.error = String(error?.message ?? error);
    }
    results.push(result);
  }
  await engine.close();
} catch (error) {
  results.push({ error: String(error?.message ?? error) });
}
// The music stops before the page says it is done.
await ctx.close();
await fetch("/result", { method: "POST", body: JSON.stringify(results) });
`;

/** A file of the voices as they were in `before`, from git. */
function fromGit(path) {
  try {
    return execFileSync("git", ["show", `${before}:web/public/${path}`], { cwd: repo, maxBuffer: 1 << 28 });
  } catch {
    return null;
  }
}

const plan = [];
for (const voice of voices) {
  const now = join(pub, "samples", voice, "voice.sfz");
  if (!existsSync(now)) throw new Error(`${voice}: no voice.sfz in public/samples`);
  const text = readFileSync(now, "utf8");
  const [key, kit] = [keyOf(text), isKit(text)];
  if (fromGit(`samples/${voice}/voice.sfz`)) plan.push({ voice, set: before, url: `/before/samples/${voice}/voice.sfz`, key, kit });
  plan.push({ voice, set: "now", url: `/samples/${voice}/voice.sfz`, key, kit });
}

let done;
const finished = new Promise((resolve) => (done = resolve));
/** What each job recorded: its left channel, then its right. */
const audio = new Map();
const server = createServer((req, res) => {
  const url = new URL(req.url ?? "/", "http://localhost");
  const headers = { "Cross-Origin-Opener-Policy": "same-origin", "Cross-Origin-Embedder-Policy": "require-corp", "Cache-Control": "no-store" };
  const send = (body, type) => res.writeHead(200, { ...headers, "Content-Type": type }).end(body);
  if (req.method === "POST") {
    const chunks = [];
    req.on("data", (c) => chunks.push(c));
    req.on("end", () => {
      res.writeHead(204).end();
      const body = Buffer.concat(chunks);
      if (url.pathname === "/result") done(JSON.parse(body.toString("utf8")));
      else audio.set(Number(url.pathname.split("/").pop()), new Float32Array(new Uint8Array(body).buffer));
    });
    return;
  }
  if (url.pathname === "/") return send(page, types[".html"]);
  if (url.pathname === "/check.js") return send(check.replace("${PLAN}", JSON.stringify(plan)), types[".js"]);
  if (url.pathname === "/tap.js") return send(tap, types[".js"]);
  const path = decodeURIComponent(url.pathname);
  const type = types[extname(path)] ?? "application/octet-stream";
  if (path.startsWith("/before/")) {
    const body = fromGit(path.slice("/before/".length));
    return body ? send(body, type) : res.writeHead(404).end();
  }
  const file = normalize(join(pub, path));
  if (!file.startsWith(pub) || !existsSync(file)) return res.writeHead(404).end();
  send(readFileSync(file), type);
});
await new Promise((resolve) => server.listen(0, "127.0.0.1", resolve));
const port = server.address().port;
const profile = mkdtempSync(join(tmpdir(), "aratu-voices-"));
const chrome = spawn(browser(), [
  "--headless=new",
  "--mute-audio",
  "--autoplay-policy=no-user-gesture-required",
  "--no-first-run",
  "--no-default-browser-check",
  "--disable-gpu",
  `--user-data-dir=${profile}`,
  `http://127.0.0.1:${port}/`,
], { stdio: "ignore" });
const timer = setTimeout(() => done([{ error: "no answer from the browser after 10 minutes" }]), 600_000);
const results = await finished;
clearTimeout(timer);
// The whole browser, its helper processes too.
if (process.platform === "win32") execFileSync("taskkill", ["/pid", String(chrome.pid), "/t", "/f"], { stdio: "ignore" });
else chrome.kill();
server.close();
await new Promise((resolve) => setTimeout(resolve, 1500));
try {
  rmSync(profile, { recursive: true, force: true, maxRetries: 10, retryDelay: 300 });
} catch {
  console.log(`left ${profile} behind`);
}

// What the recordings say.
const RATE = 48000;
const seg = (d, from, seconds) => d.subarray(Math.round(from * RATE), Math.round((from + seconds) * RATE));
const db = (x) => 10 * Math.log10(x + 1e-20);
/** K-weighted power (BS.1770) of both channels, as dB: a loudness without
 * gating, for comparing like with like. */
function kPower(l, r) {
  const filter = (b, a, d) => {
    const y = new Float64Array(d.length);
    let x1 = 0, x2 = 0, y1 = 0, y2 = 0;
    for (let i = 0; i < d.length; i++) {
      y[i] = b[0] * d[i] + b[1] * x1 + b[2] * x2 - a[1] * y1 - a[2] * y2;
      x2 = x1; x1 = d[i]; y2 = y1; y1 = y[i];
    }
    return y;
  };
  // The coefficients BS.1770 gives for 48 kHz.
  const shelf = [[1.53512485958697, -2.69169618940638, 1.19839281085285], [1, -1.69065929318241, 0.73248077421585]];
  const hp = [[1, -2, 1], [1, -1.99004745483398, 0.99007225036621]];
  let p = 0;
  for (const d of [l, r]) {
    const y = filter(...hp, filter(...shelf, d));
    p += y.reduce((s, v) => s + v * v, 0) / y.length;
  }
  return db(p) - 0.691;
}
/** The spectral centroid, in Hz, of the first 0.3 s after `from`. */
function centroid(d, from) {
  const n = 16384;
  const re = new Float64Array(n);
  const im = new Float64Array(n);
  const start = Math.round(from * RATE);
  const len = Math.round(0.3 * RATE);
  for (let i = 0; i < len; i++) re[i] = (d[start + i] ?? 0) * (0.5 - 0.5 * Math.cos((2 * Math.PI * i) / len));
  for (let i = 1, j = 0; i < n; i++) {
    let bit = n >> 1;
    for (; j & bit; bit >>= 1) j ^= bit;
    j ^= bit;
    if (i < j) [re[i], re[j], im[i], im[j]] = [re[j], re[i], im[j], im[i]];
  }
  for (let size = 2; size <= n; size <<= 1) {
    const step = (-2 * Math.PI) / size;
    for (let i = 0; i < n; i += size) {
      for (let k = 0; k < size / 2; k++) {
        const wr = Math.cos(step * k), wi = Math.sin(step * k);
        const a = i + k, b = a + size / 2;
        const tr = re[b] * wr - im[b] * wi, ti = re[b] * wi + im[b] * wr;
        re[b] = re[a] - tr; im[b] = im[a] - ti; re[a] += tr; im[a] += ti;
      }
    }
  }
  let sum = 0, weighted = 0;
  for (let k = 1; k < n / 2; k++) {
    const m = Math.hypot(re[k], im[k]);
    sum += m;
    weighted += m * ((k * RATE) / n);
  }
  return weighted / sum;
}

let failed = false;
const byVoice = new Map();
for (const [i, r] of results.entries()) {
  if (!r.voice) {
    console.log(`browser: ${r.error}`);
    failed = true;
    continue;
  }
  if (r.error || !audio.has(i)) {
    console.log(`${r.voice.padEnd(22)} ${r.set.padEnd(5)} FAILED: ${r.error ?? "nothing recorded"}`);
    failed = true;
    continue;
  }
  const both = audio.get(i);
  const l = both.subarray(0, both.length / 2);
  const rr = both.subarray(both.length / 2);
  const soft = kPower(seg(l, 0, 1.2), seg(rr, 0, 1.2));
  const loud = kPower(seg(l, 1.5, 1.2), seg(rr, 1.5, 1.2));
  const phrase = kPower(seg(l, 3, 4.5), seg(rr, 3, 4.5));
  const bright = centroid(l, 1.5) / centroid(l, 0);
  let side = 0, mid = 0;
  for (let i = 0; i < l.length; i++) {
    side += (l[i] - rr[i]) ** 2;
    mid += (l[i] + rr[i]) ** 2;
  }
  const width = db(side / mid);
  const peak = 20 * Math.log10(both.reduce((m, v) => Math.max(m, Math.abs(v)), 0));
  const m = { soft, loud, phrase, bright, width, peak, makeUp: r.makeUp, ms: r.playableMs, ...r.summary };
  (byVoice.get(r.voice) ?? byVoice.set(r.voice, {}).get(r.voice))[r.set === "now" ? "now" : "before"] = m;
  console.log(
    `${r.voice.padEnd(22)} ${r.set.slice(0, 7).padEnd(7)} ${String(m.regions).padStart(3)} regions ${(m.sample_bytes / 1e6).toFixed(0).padStart(3)} MB decoded, playable in ${String(m.ms).padStart(5)} ms, make-up ${m.makeUp?.toFixed(1) ?? "-"} dB | ` +
      `pp ${soft.toFixed(1)} ff ${loud.toFixed(1)} (${(loud - soft).toFixed(1)} dB), brightness ff/pp ${bright.toFixed(2)}, ` +
      `phrase ${phrase.toFixed(1)} LUFS, peak ${peak.toFixed(1)} dBFS, side ${width < -90 ? "none (mono)" : `${width.toFixed(1)} dB`}` +
      (r.standIn ? `, stand-in decodes ${r.standIn.files} ${r.standIn.channels}-channel files, peak ${(20 * Math.log10(r.standIn.peak)).toFixed(1)} dBFS` : ", stand-in decodes nothing"),
  );
}
for (const [voice, { now, before: was }] of byVoice) {
  if (!now) continue;
  const sfz = readFileSync(join(pub, "samples", voice, "voice.sfz"), "utf8");
  const layers = new Set([...sfz.matchAll(/lovel=(\d+)/g)].map((m) => m[1])).size;
  if (layers < 2) continue;
  const problems = [];
  if (now.bright < 1.1) problems.push(`soft and loud sound alike (brightness ff/pp ${now.bright.toFixed(2)})`);
  if (was && Math.abs(now.phrase - was.phrase) > 1.5) problems.push(`the phrase moved ${(now.phrase - was.phrase).toFixed(1)} LU from ${before}`);
  if (problems.length > 0) {
    failed = true;
    console.log(`${voice}: ${problems.join("; ")}`);
  }
}
if (failed) process.exit(1);
console.log("voices ok");
