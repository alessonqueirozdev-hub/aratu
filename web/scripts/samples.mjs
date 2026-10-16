// Prepares the sampled voices from the Versilian Community Sample Library
// (VCSL, CC0: https://github.com/sgossner/VCSL). It picks a few notes of
// each instrument, makes them mono, trims and shortens them, resamples them
// to a light rate and writes 16-bit WAV files with an index per voice into
// public/samples/. The editor fetches a voice the first time a piece uses
// it. Run once, by hand, and commit the result:
//
//   node scripts/samples.mjs            (downloads into .samples-cache/)
//
// No dependency: WAV reading and writing and the resampler are below.

import { existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const web = join(dirname(fileURLToPath(import.meta.url)), "..");
const cache = join(web, ".samples-cache");
const out = join(web, "public", "samples");
const RAW = "https://raw.githubusercontent.com/sgossner/VCSL/master/";

/** Pitched voices: notes picked at least `spacing` semitones apart. */
const PITCHED = [
  { code: "piano.grand", dir: "Chordophones/Zithers/Grand Piano, Steinway B", include: /^NoSus\/.*_vl3_rr1\.wav$/, spacing: 4, seconds: 4, rate: 24000, range: [24, 103] },
  { code: "piano.upright", dir: "Chordophones/Zithers/Upright Piano, Yamaha", include: /^Sustains\/.*_vl2_rr1\.wav$/, spacing: 4, seconds: 3.5, rate: 24000, range: [24, 103] },
  { code: "keys.harpsichord", dir: "Chordophones/Zithers/Harpsichord, Flemish", include: /^Sustains\/(High|Low)\/.*_rr1\.wav$/, spacing: 4, seconds: 2.5, rate: 24000 },
  { code: "organ.pipe", dir: "Aerophones/Edge-blown Aerophones/Pipe Organ", include: /^Loud\/.*\.wav$/, spacing: 3, seconds: 2.5, rate: 24000, loop: true },
  { code: "harp", dir: "Chordophones/Composite Chordophones/Concert Harp", include: /_mf1\.wav$/, spacing: 3, seconds: 3, rate: 24000 },
  { code: "mallets.marimba", dir: "Idiophones/Struck Idiophones/Marimba", include: /_med_01\.wav$/, spacing: 3, seconds: 2, rate: 24000 },
  { code: "mallets.vibraphone", dir: "Idiophones/Struck Idiophones/Vibraphone", include: /^Hard Mallets\/.*_v2_rr1_Main\.wav$/, spacing: 3, seconds: 3.5, rate: 24000 },
  { code: "mallets.xylophone", dir: "Idiophones/Struck Idiophones/Xylophone", include: /^Medium Mallets\/.*_ff_01_far\.wav$/, spacing: 3, seconds: 1.5, rate: 32000 },
  { code: "mallets.glockenspiel", dir: "Idiophones/Struck Idiophones/Glockenspiel", include: /glock_medium_/, spacing: 3, seconds: 2.5, rate: 32000 },
  { code: "bells.tubular", dir: "Idiophones/Struck Idiophones/Tubular Bells 1", include: /_(ff|f)_rr\d\.wav$/, spacing: 2, seconds: 4, rate: 24000 },
  { code: "mallets.kalimba", dir: "Idiophones/Plucked Idiophones/Kalimba, Kenya", include: /\.wav$/, spacing: 2, seconds: 2.5, rate: 24000 },
  { code: "harmonica", dir: "Aerophones/Free Aerophones/Harmonica-Hohner-Super64", include: /^Sustains\/Normal\/.*\.wav$/, spacing: 3, seconds: 2, rate: 24000, loop: true },
  { code: "recorder.alto", dir: "Aerophones/Edge-blown Aerophones/Baroque Alto Recorder", include: /^Sustain\/.*\.wav$/, spacing: 2, seconds: 2, rate: 24000, loop: true },
  { code: "sax.tenor", dir: "Aerophones/Reed Aerophones/Tenor Saxophone", include: /^Non-Vibrato\/.*_vl3_rr1\.wav$/, spacing: 3, seconds: 2.5, rate: 24000, loop: true },
];

/** The acoustic kit: one recording for each drum the language names. */
const KIT = {
  code: "drums.acoustic",
  rate: 32000,
  hits: {
    kick: ["Membranophones/Struck Membranophones/Bass Drum 1", /BDrumNew_hit_v5_rr1/, 1.2],
    snare: ["Membranophones/Struck Membranophones/Snare Drum, Modern 1", /HitSN_v5_rr1/, 0.8],
    "closed-hat": ["Idiophones/Struck Idiophones/Hi-Hat Cymbal", /HitC_v3_rr1/, 0.5],
    "open-hat": ["Idiophones/Struck Idiophones/Hi-Hat Cymbal", /HitO_rr1/, 1.5],
    ride: ["Idiophones/Struck Idiophones/Suspended Cymbal 1", /[Hh]it.*\.wav$/, 2.5],
    crash: ["Idiophones/Struck Idiophones/Clash Cymbals 1", /crash1_mf1/, 2.5],
    "high-tom": ["Membranophones/Struck Membranophones/Tom 1", /Stick\/TomH_HitS_v3_rr1/, 1],
    "mid-tom": ["Membranophones/Struck Membranophones/Tom 2", /Stick\/TomL_HitS_v3_rr1/, 1],
    "floor-tom": ["Membranophones/Struck Membranophones/Legacy Toms", /tenor_lower\/tenor_ff_rr1/, 1.2],
    "side-stick": ["Membranophones/Struck Membranophones/Legacy Snares", /drum1\/snare1_click\.wav$/, 0.4],
  },
};

const NAMES = { C: 0, D: 2, E: 4, F: 5, G: 7, A: 9, B: 11 };
function midi(name) {
  const m = name.match(/^([A-G])(#|b)?(-?\d)$/);
  if (!m) return null;
  return (Number(m[3]) + 1) * 12 + NAMES[m[1]] + (m[2] === "#" ? 1 : m[2] === "b" ? -1 : 0);
}
const noteIn = (path) => {
  for (const part of path.split(/[_ /.\-]/)) {
    const n = midi(part);
    if (n !== null) return n;
  }
  return null;
};

async function tree() {
  const file = join(cache, "tree.json");
  if (!existsSync(file)) {
    const r = await fetch("https://api.github.com/repos/sgossner/VCSL/git/trees/master?recursive=1");
    if (!r.ok) throw new Error(`GitHub API: ${r.status}`);
    writeFileSync(file, await r.text());
  }
  return JSON.parse(readFileSync(file, "utf8")).tree.filter((x) => x.type === "blob");
}

async function download(path) {
  const file = join(cache, path.replace(/[\\/:*?"<>|]/g, "_"));
  if (!existsSync(file)) {
    const url = RAW + path.split("/").map(encodeURIComponent).join("/");
    const r = await fetch(url);
    if (!r.ok) throw new Error(`${path}: ${r.status}`);
    writeFileSync(file, Buffer.from(await r.arrayBuffer()));
    process.stdout.write(".");
  }
  return readFileSync(file);
}

/** Reads a PCM or float WAV into one mono channel. */
function readWav(buf) {
  if (buf.toString("ascii", 0, 4) !== "RIFF" || buf.toString("ascii", 8, 12) !== "WAVE") throw new Error("not a WAV");
  let pos = 12;
  let fmt = null;
  while (pos + 8 <= buf.length) {
    const id = buf.toString("ascii", pos, pos + 4);
    const size = buf.readUInt32LE(pos + 4);
    const body = pos + 8;
    if (id === "fmt ") {
      let format = buf.readUInt16LE(body);
      const channels = buf.readUInt16LE(body + 2);
      const rate = buf.readUInt32LE(body + 4);
      const bits = buf.readUInt16LE(body + 14);
      if (format === 0xfffe) format = buf.readUInt16LE(body + 24);
      fmt = { format, channels, rate, bits };
    } else if (id === "data" && fmt) {
      const bytes = fmt.bits / 8;
      const frames = Math.floor(Math.min(size, buf.length - body) / (bytes * fmt.channels));
      const mono = new Float32Array(frames);
      for (let i = 0; i < frames; i++) {
        let s = 0;
        for (let c = 0; c < fmt.channels; c++) {
          const at = body + (i * fmt.channels + c) * bytes;
          if (fmt.format === 3) s += bytes === 4 ? buf.readFloatLE(at) : buf.readDoubleLE(at);
          else if (bytes === 2) s += buf.readInt16LE(at) / 32768;
          else if (bytes === 3) s += buf.readIntLE(at, 3) / 8388608;
          else if (bytes === 4) s += buf.readInt32LE(at) / 2147483648;
          else s += (buf[at] - 128) / 128;
        }
        mono[i] = s / fmt.channels;
      }
      return { rate: fmt.rate, data: mono };
    }
    pos = body + size + (size % 2);
  }
  throw new Error("no audio in WAV");
}

/** A windowed-sinc resampler, low-passed below the new Nyquist. */
function resample(data, from, to) {
  if (from === to) return data;
  const ratio = to / from;
  const n = Math.floor(data.length * ratio);
  const outData = new Float32Array(n);
  const cutoff = Math.min(1, ratio) * 0.95;
  const half = 16;
  for (let i = 0; i < n; i++) {
    const x = i / ratio;
    const c = Math.floor(x);
    let s = 0;
    let w = 0;
    for (let k = c - half + 1; k <= c + half; k++) {
      if (k < 0 || k >= data.length) continue;
      const t = x - k;
      const sinc = t === 0 ? 1 : Math.sin(Math.PI * t * cutoff) / (Math.PI * t * cutoff);
      const win = 0.5 + 0.5 * Math.cos((Math.PI * t) / half);
      const f = sinc * win;
      s += data[k] * f;
      w += f;
    }
    outData[i] = w ? s / w : 0;
  }
  return outData;
}

/** From just before the sound starts, at most `seconds` long, faded out. */
function shape(data, rate, seconds, loop) {
  let peak = 0;
  for (const v of data) peak = Math.max(peak, Math.abs(v));
  const gate = peak * 0.02;
  let start = 0;
  while (start < data.length && Math.abs(data[start]) < gate) start++;
  start = Math.max(0, start - Math.round(rate * 0.003));
  const len = Math.min(data.length - start, Math.round(rate * seconds));
  const d = data.slice(start, start + len);
  let loopStart = null;
  let loopEnd = null;
  if (loop) {
    // A loop in the steady middle, its end cross-faded into its start, so a
    // held note sustains.
    loopStart = Math.round(len * 0.35);
    loopEnd = Math.round(len * 0.9);
    const fade = Math.min(Math.round(rate * 0.12), loopStart);
    for (let i = 0; i < fade; i++) {
      const t = i / fade;
      const a = loopEnd - fade + i;
      d[a] = d[a] * (1 - t) + d[loopStart - fade + i] * t;
    }
  } else {
    const fade = Math.min(Math.round(rate * 0.3), Math.floor(len / 3));
    for (let i = 0; i < fade; i++) d[len - fade + i] *= 1 - i / fade;
  }
  return { data: d, loopStart, loopEnd };
}

function writeWav(file, data, rate, gain) {
  const buf = Buffer.alloc(44 + data.length * 2);
  buf.write("RIFF", 0, "ascii");
  buf.writeUInt32LE(36 + data.length * 2, 4);
  buf.write("WAVEfmt ", 8, "ascii");
  buf.writeUInt32LE(16, 16);
  buf.writeUInt16LE(1, 20);
  buf.writeUInt16LE(1, 22);
  buf.writeUInt32LE(rate, 24);
  buf.writeUInt32LE(rate * 2, 28);
  buf.writeUInt16LE(2, 32);
  buf.writeUInt16LE(16, 34);
  buf.write("data", 36, "ascii");
  buf.writeUInt32LE(data.length * 2, 40);
  for (let i = 0; i < data.length; i++) {
    buf.writeInt16LE(Math.max(-32767, Math.min(32767, Math.round(data[i] * gain * 32767))), 44 + i * 2);
  }
  writeFileSync(file, buf);
  return buf.length;
}

const peakOf = (d) => d.reduce((m, v) => Math.max(m, Math.abs(v)), 0);

/** The same voice as an SFZ instrument, for Manguebyte's sampler: each
 * recording covers the keys nearest to it. */
function sfz(notes, rate, loop) {
  const lines = ["// Made by Aratu's web/scripts/samples.mjs from VCSL (CC0).", "<control> default_path=./", "<global> ampeg_release=0.12"];
  notes.forEach((n, i) => {
    const lo = i === 0 ? 0 : Math.floor((notes[i - 1].note + n.note) / 2) + 1;
    const hi = i === notes.length - 1 ? 127 : Math.floor((n.note + notes[i + 1].note) / 2);
    let region = `<region> sample=${n.file} pitch_keycenter=${n.note} lokey=${lo} hikey=${hi}`;
    if (loop && n.loop) {
      region += ` loop_mode=loop_continuous loop_start=${Math.round(n.loop[0] * rate)} loop_end=${Math.round(n.loop[1] * rate)}`;
    }
    lines.push(region);
  });
  return lines.join("\n") + "\n";
}

/** The kit as SFZ: General MIDI drum keys. */
const GM = { kick: 36, snare: 38, "side-stick": 37, "closed-hat": 42, "open-hat": 46, ride: 51, crash: 49, "high-tom": 50, "mid-tom": 47, "floor-tom": 43 };
function kitSfz(hits) {
  const lines = ["// Made by Aratu's web/scripts/samples.mjs from VCSL (CC0).", "<control> default_path=./", "<group> loop_mode=one_shot"];
  for (const [name, file] of Object.entries(hits)) {
    // The closed hat (group 2) chokes the open one.
    const choke = name === "open-hat" ? " group=1 off_by=2" : name === "closed-hat" ? " group=2" : "";
    lines.push(`<region> sample=${file} key=${GM[name]}${choke}`);
  }
  return lines.join("\n") + "\n";
}

async function pitched(files, v) {
  const inDir = files
    .filter((f) => f.path.startsWith(v.dir + "/"))
    .map((f) => ({ ...f, rel: f.path.slice(v.dir.length + 1) }))
    .filter((f) => v.include.test(f.rel))
    .map((f) => ({ ...f, note: noteIn(f.rel) }))
    .filter((f) => f.note !== null && (!v.range || (f.note >= v.range[0] && f.note <= v.range[1])))
    .sort((a, b) => a.note - b.note || a.rel.localeCompare(b.rel));
  const picked = [];
  for (const f of inDir) {
    if (picked.length === 0 || f.note - picked[picked.length - 1].note >= v.spacing) picked.push(f);
  }
  if (picked.length === 0) throw new Error(`${v.code}: no files matched`);
  const shaped = [];
  for (const f of picked) {
    const wav = readWav(await download(f.path));
    const res = resample(wav.data, wav.rate, v.rate);
    shaped.push({ f, ...shape(res, v.rate, v.seconds, v.loop) });
  }
  // One gain for the voice, so its notes keep their relative levels.
  const gain = 0.9 / Math.max(...shaped.map((s) => peakOf(s.data)));
  const dir = join(out, v.code);
  mkdirSync(dir, { recursive: true });
  let bytes = 0;
  const notes = shaped.map((s) => {
    const file = `${s.f.note}.wav`;
    bytes += writeWav(join(dir, file), s.data, v.rate, gain);
    const n = { note: s.f.note, file };
    if (v.loop) Object.assign(n, { loop: [s.loopStart / v.rate, s.loopEnd / v.rate] });
    return n;
  });
  writeFileSync(join(dir, "index.json"), JSON.stringify({ kind: "pitched", source: v.dir, notes }));
  writeFileSync(join(dir, "voice.sfz"), sfz(notes, v.rate, !!v.loop));
  return { code: v.code, files: notes.length, bytes, sources: picked.map((f) => f.path) };
}

async function kit(files) {
  const dir = join(out, KIT.code);
  mkdirSync(dir, { recursive: true });
  const hits = {};
  const sources = [];
  let bytes = 0;
  for (const [name, [d, re, seconds]] of Object.entries(KIT.hits)) {
    const f = files.find((x) => x.path.startsWith(d + "/") && re.test(x.path.slice(d.length + 1)));
    if (!f) throw new Error(`${KIT.code} ${name}: no file`);
    const wav = readWav(await download(f.path));
    const s = shape(resample(wav.data, wav.rate, KIT.rate), KIT.rate, seconds, false);
    bytes += writeWav(join(dir, `${name}.wav`), s.data, KIT.rate, 0.9 / peakOf(s.data));
    hits[name] = `${name}.wav`;
    sources.push(f.path);
  }
  writeFileSync(join(dir, "index.json"), JSON.stringify({ kind: "kit", hits }));
  writeFileSync(join(dir, "voice.sfz"), kitSfz(hits));
  return { code: KIT.code, files: Object.keys(hits).length, bytes, sources };
}

mkdirSync(cache, { recursive: true });
rmSync(out, { recursive: true, force: true });
mkdirSync(out, { recursive: true });
const files = await tree();
const report = [];
for (const v of PITCHED) report.push(await pitched(files, v));
report.push(await kit(files));
process.stdout.write("\n");
let total = 0;
for (const r of report) {
  total += r.bytes;
  console.log(`${r.code.padEnd(22)} ${String(r.files).padStart(3)} files ${(r.bytes / 1e6).toFixed(2)} MB`);
}
console.log(`total ${(total / 1e6).toFixed(1)} MB`);
writeFileSync(
  join(out, "SOURCES.md"),
  `# Sampled voices\n\nThe recordings in this folder come from the Versilian Community Sample\nLibrary (VCSL), released under Creative Commons Zero (CC0):\nhttps://github.com/sgossner/VCSL and https://versilian-studios.com/vcsl/.\nThey were made mono, trimmed, shortened and resampled by\n\`web/scripts/samples.mjs\`. Thank you, Versilian Studios.\n\n` +
    report.map((r) => `## ${r.code}\n\n${r.sources.map((s) => `- ${s}`).join("\n")}\n`).join("\n"),
);
