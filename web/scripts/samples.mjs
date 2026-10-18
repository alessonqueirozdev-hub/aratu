// Prepares the sampled voices from the Versilian Community Sample Library
// (VCSL, CC0: https://github.com/sgossner/VCSL) and writes them, with an
// index per voice, into public/samples/. The editor fetches a voice the
// first time a piece uses it. A voice with `layers` keeps its recordings as
// they were made, in stereo at their own rate: every dynamic layer, mapped
// to a range of velocities, and every round robin, taking turns, trimmed
// and faded and written as FLAC or Ogg Vorbis. The others pick one take of
// a few notes, make them mono, trim and shorten them, resample them to a
// light rate and write 16-bit WAV files. The voices in KEPT were made
// elsewhere: their folders are left as they are. Run by hand and commit the
// result:
//
//   node scripts/samples.mjs [voice ...]   (downloads into .samples-cache/)
//
// Naming voices remakes only those and leaves SOURCES.md as it is.
//
// WAV reading and writing and the resampler are below. FLAC and Ogg Vorbis
// are written by Manguebyte's encoders (manguebyte-formats, MIT OR
// Apache-2.0), which the script builds into the cache with cargo.

import { execFileSync } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const web = join(dirname(fileURLToPath(import.meta.url)), "..");
const cache = join(web, ".samples-cache");
const out = join(web, "public", "samples");
const RAW = "https://raw.githubusercontent.com/sgossner/VCSL/master/";

/** Pitched voices: notes picked at least `spacing` semitones apart. A voice
 * with `layers` (patterns of its files, softest first) keeps the source's
 * rate and channels; its `seconds` may run from the lowest note to the
 * highest, and `rr` finds the round robin a file is. `format` is "flac" or
 * a Vorbis quality ("q7"). Every voice is FLAC for now: on the mallets,
 * Manguebyte's Vorbis encoder at quality 7 took out everything above
 * 16 kHz and most of 8 to 16 kHz, the brightness the layers are for, which
 * FLAC keeps for four times the bytes. `normalized` marks a source whose
 * layers come at one level already (the grand's NORMALIZED.txt asks that
 * their gains be left alone). */
const PITCHED = [
  { code: "piano.grand", dir: "Chordophones/Zithers/Grand Piano, Steinway B", include: /^NoSus\//, layers: [/_vl2_/, /_vl3_/, /_vl4_/], rr: /_rr(\d+)/, normalized: true, spacing: 4, seconds: [5, 2], fade: 1, format: "flac", range: [24, 103] },
  { code: "piano.upright", dir: "Chordophones/Zithers/Upright Piano, Yamaha", include: /^Sustains\//, layers: [/_vl1_/, /_vl2_/, /_vl3_/], rr: /_rr(\d+)/, spacing: 4, seconds: [4, 1.8], fade: 1, format: "flac", range: [24, 103] },
  { code: "keys.harpsichord", dir: "Chordophones/Zithers/Harpsichord, Flemish", include: /^Sustains\/(High|Low)\/.*_rr1\.wav$/, spacing: 4, seconds: 2.5, rate: 24000 },
  { code: "organ.pipe", dir: "Aerophones/Edge-blown Aerophones/Pipe Organ", include: /^Loud\/.*\.wav$/, spacing: 3, seconds: 2.5, rate: 24000, loop: true },
  { code: "harp", dir: "Chordophones/Composite Chordophones/Concert Harp", include: /\.wav$/, layers: [/_(p|mp|mf)\d\.wav$/, /_f\d\.wav$/], spacing: 3, seconds: [4, 1.5], fade: 0.8, format: "flac" },
  { code: "mallets.marimba", dir: "Idiophones/Struck Idiophones/Marimba", include: /\.wav$/, layers: [/_soft_/, /_med_/, /_loud_/], spacing: 3, seconds: [2.5, 1.2], fade: 0.5, format: "flac" },
  { code: "mallets.vibraphone", dir: "Idiophones/Struck Idiophones/Vibraphone", include: /^Hard Mallets\/.*_Main\.wav$/, layers: [/_v2_/, /_v3_/], rr: /_rr(\d+)/, spacing: 3, seconds: [4, 2.5], fade: 1, format: "flac" },
  { code: "mallets.xylophone", dir: "Idiophones/Struck Idiophones/Xylophone", include: /^Medium Mallets\/.*_far\.wav$/, layers: [/_pp_/, /_ff_/], spacing: 3, seconds: 1.5, fade: 0.4, format: "flac" },
  { code: "mallets.glockenspiel", dir: "Idiophones/Struck Idiophones/Glockenspiel", include: /\.wav$/, layers: [/_soft_/, /_medium_/, /_loud_/], spacing: 3, seconds: [3, 2], fade: 0.8, format: "flac" },
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

/** Voices recorded for Aratu rather than taken from VCSL: this script
 * neither deletes nor rewrites their folders, which hold their own
 * README.md and LICENSE.txt, and credits them at the end of SOURCES.md. */
const KEPT = [
  {
    code: "drums.alesson",
    credit: `Recorded by Alesson Queiroz and dedicated to the public domain under
Creative Commons Zero (CC0 1.0):
https://creativecommons.org/publicdomain/zero/1.0/. Kept as it came, not
made by \`web/scripts/samples.mjs\`: 22 hits, 48 kHz, 24-bit, mono, mapped
to General MIDI keys in \`voice.sfz\` (kick, snare in three velocity layers,
an alternative snare on key 40, crash), with round robins. They come from
takes of a snare, a crash and a whole groove. Each hit was cut from just
before its attack to where its tail meets the noise floor, with one gain
per instrument and no compression or EQ. The kick and the alternative
snare were separated from the groove (score-informed NMF and a Wiener
mask) and kept only where the other drums leak in below -25 dB; the
hi-hats did not come out clean, so they stay synthesized.
\`drums.alesson/README.md\` tells how every sample was made, and
\`LICENSE.txt\` holds the dedication. Thank you, Alesson.`,
  },
];

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

/** Reads a PCM or float WAV into one mono channel, and with `keep` each of
 * its channels too. */
function readWav(buf, keep = false) {
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
      const channels = keep ? Array.from({ length: fmt.channels }, () => new Float32Array(frames)) : [];
      for (let i = 0; i < frames; i++) {
        let s = 0;
        for (let c = 0; c < fmt.channels; c++) {
          const at = body + (i * fmt.channels + c) * bytes;
          let v;
          if (fmt.format === 3) v = bytes === 4 ? buf.readFloatLE(at) : buf.readDoubleLE(at);
          else if (bytes === 2) v = buf.readInt16LE(at) / 32768;
          else if (bytes === 3) v = buf.readIntLE(at, 3) / 8388608;
          else if (bytes === 4) v = buf.readInt32LE(at) / 2147483648;
          else v = (buf[at] - 128) / 128;
          s += v;
          if (keep) channels[c][i] = v;
        }
        mono[i] = s / fmt.channels;
      }
      return { rate: fmt.rate, data: mono, channels };
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

/** The channels of a recording, from just before the sound starts, at most
 * `seconds` long, the last `fade` seconds faded out. */
function shapeChannels(channels, rate, seconds, fade) {
  const loudest = (i) => channels.reduce((m, d) => Math.max(m, Math.abs(d[i])), 0);
  let peak = 0;
  for (let i = 0; i < channels[0].length; i++) peak = Math.max(peak, loudest(i));
  let start = 0;
  while (start < channels[0].length && loudest(start) < peak * 0.02) start++;
  start = Math.max(0, start - Math.round(rate * 0.003));
  const len = Math.min(channels[0].length - start, Math.round(rate * seconds));
  const n = Math.min(Math.round(rate * fade), Math.floor(len / 2));
  return channels.map((d) => {
    const s = d.slice(start, start + len);
    // A raised-cosine fade: the decay goes on into silence, without a corner.
    for (let i = 0; i < n; i++) s[len - n + i] *= 0.5 + 0.5 * Math.cos((Math.PI * i) / n);
    return s;
  });
}

/** How loud a recording is, in dB: its first second through the K filter
 * of ITU-R BS.1770 (a high shelf and a high-pass), the channels' powers
 * added, as the loudness meters that normalise the voices hear it. */
function loudnessOf(channels, rate) {
  const biquad = (b, a, d) => {
    const y = new Float64Array(d.length);
    let x1 = 0, x2 = 0, y1 = 0, y2 = 0;
    for (let i = 0; i < d.length; i++) {
      y[i] = b[0] * d[i] + b[1] * x1 + b[2] * x2 - a[1] * y1 - a[2] * y2;
      x2 = x1; x1 = d[i]; y2 = y1; y1 = y[i];
    }
    return y;
  };
  // The analog prototypes of BS.1770, made digital for this rate.
  let K = Math.tan((Math.PI * 1681.974450955533) / rate);
  const Q1 = 0.7071752369554196;
  const Vh = Math.pow(10, 3.999843853973347 / 20);
  const Vb = Math.pow(Vh, 0.4996667741545416);
  let a0 = 1 + K / Q1 + K * K;
  const shelf = [
    [(Vh + (Vb * K) / Q1 + K * K) / a0, (2 * (K * K - Vh)) / a0, (Vh - (Vb * K) / Q1 + K * K) / a0],
    [1, (2 * (K * K - 1)) / a0, (1 - K / Q1 + K * K) / a0],
  ];
  K = Math.tan((Math.PI * 38.13547087602444) / rate);
  const Q2 = 0.5003270373238773;
  a0 = 1 + K / Q2 + K * K;
  const highpass = [
    [1, -2, 1],
    [1, (2 * (K * K - 1)) / a0, (1 - K / Q2 + K * K) / a0],
  ];
  let power = 0;
  for (const d of channels) {
    const y = biquad(...highpass, biquad(...shelf, d.subarray(0, Math.min(d.length, rate))));
    power += y.reduce((s, v) => s + v * v, 0) / y.length;
  }
  return 10 * Math.log10(power + 1e-20);
}

function writeWav(file, data, rate, gain) {
  const channels = Array.isArray(data) ? data : [data];
  const n = channels.length;
  const frames = channels[0].length;
  const buf = Buffer.alloc(44 + frames * n * 2);
  buf.write("RIFF", 0, "ascii");
  buf.writeUInt32LE(36 + frames * n * 2, 4);
  buf.write("WAVEfmt ", 8, "ascii");
  buf.writeUInt32LE(16, 16);
  buf.writeUInt16LE(1, 20);
  buf.writeUInt16LE(n, 22);
  buf.writeUInt32LE(rate, 24);
  buf.writeUInt32LE(rate * 2 * n, 28);
  buf.writeUInt16LE(2 * n, 32);
  buf.writeUInt16LE(16, 34);
  buf.write("data", 36, "ascii");
  buf.writeUInt32LE(frames * n * 2, 40);
  for (let i = 0; i < frames; i++) {
    for (let c = 0; c < n; c++) {
      const v = Math.max(-32767, Math.min(32767, Math.round(channels[c][i] * gain * 32767)));
      buf.writeInt16LE(v, 44 + (i * n + c) * 2);
    }
  }
  writeFileSync(file, buf);
  return buf.length;
}

// The FLAC and Ogg Vorbis files are written by Manguebyte's encoders, from
// this commit: a small program, built once into the cache, that reads a WAV
// file and writes either. FLAC at level 8 is lossless; Vorbis is coded at
// the quality asked for, from 0 to 10.
const ENCODER = {
  commit: "1846d039f62b4ae8b710229e793439353f58b87e",
  repo: "https://github.com/alessonqueirozdev-hub/manguebyte.git",
};
let encoderExe = null;
function encoder() {
  if (encoderExe) return encoderExe;
  const dir = join(cache, "encoder");
  mkdirSync(join(dir, "src"), { recursive: true });
  writeFileSync(
    join(dir, "Cargo.toml"),
    `# Written by Aratu's web/scripts/samples.mjs.
[package]
name = "aratu-samples-encoder"
version = "0.0.0"
edition = "2024"
publish = false

[dependencies]
manguebyte-formats = { git = "${ENCODER.repo}", rev = "${ENCODER.commit}" }

# Its own workspace, not Aratu's.
[workspace]
`,
  );
  writeFileSync(
    join(dir, "src", "main.rs"),
    `// Written by Aratu's web/scripts/samples.mjs: encoder <flac|quality> <in.wav> <out>
use manguebyte_formats::{flac, vorbis, wav};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let [_, how, input, output] = args.as_slice() else {
        return Err("usage: encoder <flac|0-10> <in.wav> <out>".into());
    };
    let file = wav::read_file(input)?;
    if how == "flac" {
        let bits = u32::from(file.bits_per_sample);
        let options = flac::FlacOptions::new(bits).with_level(8).with_padding(0).with_seek_points(0);
        flac::write_file(output, &file.buffer, file.sample_rate, &options)?;
    } else {
        let options = vorbis::VorbisOptions::new(how.parse()?);
        vorbis::write_file(output, &file.buffer, file.sample_rate, &options)?;
    }
    Ok(())
}
`,
  );
  execFileSync("cargo", ["build", "--quiet", "--release"], { cwd: dir, stdio: "inherit" });
  encoderExe = join(dir, "target", "release", `aratu-samples-encoder${process.platform === "win32" ? ".exe" : ""}`);
  return encoderExe;
}

/** Writes channels as `file` in `format` ("flac" or "q7"), through a WAV. */
function writeEncoded(file, channels, rate, gain, format) {
  const wav = join(cache, "encoding.wav");
  writeWav(wav, channels, rate, gain);
  execFileSync(encoder(), [format === "flac" ? "flac" : format.slice(1), wav, file], { stdio: "inherit" });
  rmSync(wav);
  return statSync(file).size;
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

/** The highest velocity of each layer but the loudest, from Aratu's
 * dynamics (theory.rs: ppp 16, pp 33, p 49, mp 64, mf 80, f 96, ff 112,
 * fff 127, an accent 32 more). Of two layers the soft one plays up to mf;
 * of three, the softest plays p and under, the middle one mp and mf, the
 * loudest f and over, and an accent on mf. */
const SPLITS = { 1: [], 2: [88], 3: [59, 95] };

/** A layered voice as SFZ: each layer a group of velocities, its notes
 * covering the keys nearest to them, round robins taking turns. */
function layeredSfz(layers) {
  const tops = SPLITS[layers.length];
  const lines = ["// Made by Aratu's web/scripts/samples.mjs from VCSL (CC0).", "<control> default_path=./", "<global> ampeg_release=0.12"];
  layers.forEach((notes, l) => {
    const lovel = l === 0 ? 0 : tops[l - 1] + 1;
    const hivel = l === layers.length - 1 ? 127 : tops[l];
    lines.push(`<group> lovel=${lovel} hivel=${hivel}`);
    notes.forEach((n, i) => {
      const lo = i === 0 ? 0 : Math.floor((notes[i - 1].note + n.note) / 2) + 1;
      const hi = i === notes.length - 1 ? 127 : Math.floor((n.note + notes[i + 1].note) / 2);
      n.takes.forEach((file, t) => {
        const turns = n.takes.length > 1 ? ` seq_length=${n.takes.length} seq_position=${t + 1}` : "";
        lines.push(`<region> sample=${file} pitch_keycenter=${n.note} lokey=${lo} hikey=${hi}${turns}`);
      });
    });
  });
  return lines.join("\n") + "\n";
}

/** The kit as SFZ: General MIDI drum keys. */
// "guitar-tap" (the flamenco golpe) is 88 in the contract with Manguebyte, past
// every GM and GM2 percussion key; VCSL has no recording of it yet.
const GM = { kick: 36, snare: 38, "side-stick": 37, "closed-hat": 42, "open-hat": 46, ride: 51, crash: 49, "high-tom": 50, "mid-tom": 47, "floor-tom": 43, "guitar-tap": 88 };
function kitSfz(hits) {
  const lines = ["// Made by Aratu's web/scripts/samples.mjs from VCSL (CC0).", "<control> default_path=./", "<group> loop_mode=one_shot"];
  for (const [name, file] of Object.entries(hits)) {
    // The closed hat (group 2) chokes the open one.
    const choke = name === "open-hat" ? " group=1 off_by=2" : name === "closed-hat" ? " group=2" : "";
    lines.push(`<region> sample=${file} key=${GM[name]}${choke}`);
  }
  return lines.join("\n") + "\n";
}

/** A voice with every dynamic layer and round robin of its notes, at the
 * source's rate and channels. Each note of a softer layer is brought to the
 * loudness of the loudest layer's, so that velocity sets the level as it
 * does in every sampler part (the SFZ square law; Manguebyte's loudness
 * normalisation measures at velocity 0.8, in the loudest layer) and the
 * layer only changes the timbre; the loudest layer's notes keep their
 * relative levels. */
async function layered(files, v) {
  const inDir = files
    .filter((f) => f.path.startsWith(v.dir + "/"))
    .map((f) => ({ ...f, rel: f.path.slice(v.dir.length + 1) }))
    .filter((f) => v.include.test(f.rel) && f.rel.endsWith(".wav"))
    .map((f) => ({ ...f, note: noteIn(f.rel), layer: v.layers.findIndex((re) => re.test(f.rel)) }))
    .filter((f) => f.note !== null && f.layer >= 0 && (!v.range || (f.note >= v.range[0] && f.note <= v.range[1])))
    .map((f) => ({ ...f, take: v.rr ? Number(f.rel.match(v.rr)?.[1] ?? 1) : 1 }))
    .sort((a, b) => a.note - b.note || a.layer - b.layer || a.take - b.take || a.rel.localeCompare(b.rel))
    // Takes that are not round robins (a second take of a note): the first.
    .filter((f, i, all) => i === 0 || f.note !== all[i - 1].note || f.layer !== all[i - 1].layer || f.take !== all[i - 1].take);
  const notes = [];
  for (const n of new Set(inDir.map((f) => f.note))) {
    if (notes.length === 0 || n - notes[notes.length - 1] >= v.spacing) notes.push(n);
  }
  const picked = inDir.filter((f) => notes.includes(f.note));
  if (picked.length === 0) throw new Error(`${v.code}: no files matched`);
  const [low, high] = [notes[0], notes[notes.length - 1]];
  const secondsAt = (note) => {
    if (!Array.isArray(v.seconds)) return v.seconds;
    const t = high === low ? 0 : (note - low) / (high - low);
    return v.seconds[0] + (v.seconds[1] - v.seconds[0]) * t;
  };
  const shaped = [];
  let rate = null;
  for (const f of picked) {
    const wav = readWav(await download(f.path), true);
    if (rate !== null && wav.rate !== rate) throw new Error(`${v.code}: ${f.rel} is ${wav.rate} Hz, not ${rate}`);
    rate = wav.rate;
    const channels = shapeChannels(wav.channels.slice(0, 2), rate, secondsAt(f.note), v.fade ?? 0.3);
    shaped.push({ f, channels, loudness: loudnessOf(channels, rate) });
  }
  // Each note of a softer layer is brought to the loudness of the loudest
  // layer's nearest note, its takes by one gain, so that they keep their
  // differences; nothing moves in a source whose layers were brought to one
  // level already, and that says not to move them.
  const top = v.layers.length - 1;
  const mean = (list) => list.reduce((a, b) => a + b, 0) / list.length;
  const groupOf = (s) => shaped.filter((o) => o.f.note === s.f.note && o.f.layer === s.f.layer);
  const loudest = shaped.filter((s) => s.f.layer === top && s.f.take === 1);
  const gainDb = new Map();
  for (const s of shaped) {
    if (v.normalized || s.f.layer === top) {
      gainDb.set(s, 0);
      continue;
    }
    const ref = loudest.reduce((a, b) => (Math.abs(b.f.note - s.f.note) < Math.abs(a.f.note - s.f.note) ? b : a));
    gainDb.set(s, mean(groupOf(ref).map((o) => o.loudness)) - mean(groupOf(s).map((o) => o.loudness)));
  }
  const gainOf = (s) => Math.pow(10, gainDb.get(s) / 20);
  // Then one gain for the voice, for its loudest peak.
  const peak = Math.max(...shaped.map((s) => gainOf(s) * Math.max(...s.channels.map(peakOf))));
  const dir = join(out, v.code);
  mkdirSync(dir, { recursive: true });
  const ext = v.format === "flac" ? "flac" : "ogg";
  let bytes = 0;
  const layers = v.layers.map(() => []);
  for (const s of shaped) {
    const { note, layer, take } = s.f;
    const file = `${note}_v${layer + 1}${take > 1 ? `_rr${take}` : ""}.${ext}`;
    bytes += writeEncoded(join(dir, file), s.channels, rate, (gainOf(s) * 0.9) / peak, v.format);
    const list = layers[layer];
    if (list.length === 0 || list[list.length - 1].note !== note) list.push({ note, takes: [] });
    list[list.length - 1].takes.push(file);
  }
  // The stand-in plays one layer, the one an unmarked note (mf) plays.
  const tops = [...SPLITS[v.layers.length], 127];
  const mf = layers[tops.findIndex((t) => 80 <= t)];
  const index = { kind: "pitched", source: v.dir, notes: mf.map((n) => ({ note: n.note, file: n.takes[0] })) };
  writeFileSync(join(dir, "index.json"), JSON.stringify(index));
  writeFileSync(join(dir, "voice.sfz"), layeredSfz(layers));
  const gainsDb = v.layers.map((_, l) => mean(shaped.filter((s) => s.f.layer === l).map((s) => gainDb.get(s))).toFixed(1));
  return { code: v.code, files: shaped.length, bytes, sources: picked.map((f) => f.path), rate, channels: shaped[0].channels.length, gainsDb, layers: layers.map((l) => l.length) };
}

async function pitched(files, v) {
  if (v.layers) return layered(files, v);
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

/** What SOURCES.md says of how a layered voice was made. */
function madeOf(v, r) {
  const tops = [...SPLITS[v.layers.length], 127];
  const ranges = tops.map((t, l) => `${l === 0 ? 0 : tops[l - 1] + 1}–${t}`).join(", ");
  const format = v.format === "flac" ? "FLAC" : `Ogg Vorbis at quality ${v.format.slice(1)}`;
  const turns = r.files > r.layers.reduce((a, b) => a + b, 0) ? ", with the round robins recorded taking turns" : "";
  return `${r.rate / 1000} kHz, ${r.channels === 2 ? "stereo" : "mono"}, ${format}: ${r.layers.length} dynamic layers (velocities ${ranges}) of ${r.layers.join(", ")} notes${turns}, ${r.files} recordings.\n\n`;
}

const only = process.argv.slice(2);
for (const code of only) {
  if (![...PITCHED, KIT].some((v) => v.code === code)) throw new Error(`${code}: no such voice is made here`);
}
const wanted = (v) => only.length === 0 || only.includes(v.code);
mkdirSync(cache, { recursive: true });
// Only the folders made here start over; the kept voices stay.
for (const v of [...PITCHED, KIT]) {
  if (KEPT.some((k) => k.code === v.code)) throw new Error(`${v.code} is both made and kept`);
  if (wanted(v)) rmSync(join(out, v.code), { recursive: true, force: true });
}
for (const k of KEPT) {
  if (!existsSync(join(out, k.code, "voice.sfz"))) throw new Error(`${k.code}: its folder is missing from ${out}`);
}
mkdirSync(out, { recursive: true });
const files = await tree();
const report = [];
for (const v of PITCHED.filter(wanted)) report.push(await pitched(files, v));
if (wanted(KIT)) report.push(await kit(files));
process.stdout.write("\n");
let total = 0;
for (const r of report) {
  total += r.bytes;
  const layers = r.layers ? ` ${r.rate} Hz ${r.channels} ch, layers ${r.layers.join("/")} notes, gains ${r.gainsDb.join("/")} dB` : "";
  console.log(`${r.code.padEnd(22)} ${String(r.files).padStart(3)} files ${(r.bytes / 1e6).toFixed(2)} MB${layers}`);
}
console.log(`total ${(total / 1e6).toFixed(1)} MB`);
if (only.length > 0) {
  console.log("SOURCES.md was left as it is: run without naming voices to write it.");
} else {
  const formats = [...new Set(PITCHED.filter((v) => v.layers).map((v) => (v.format === "flac" ? "FLAC" : "Ogg Vorbis")))];
  writeFileSync(
    join(out, "SOURCES.md"),
    `# Sampled voices\n\nMost recordings in this folder come from the Versilian Community Sample\nLibrary (VCSL), released under Creative Commons Zero (CC0):\nhttps://github.com/sgossner/VCSL and https://versilian-studios.com/vcsl/.\n\`web/scripts/samples.mjs\` made them. The voices whose recordings it\ndescribes keep them as they were recorded, at their rate and in their\nchannels, every dynamic layer mapped to a range of velocities and brought\nto one loudness, so that the velocity sets the level and the layer the\ntimbre; trimmed and faded, they are written as ${formats.join(" and ")} by\nManguebyte's encoder${formats.length > 1 ? "s" : ""}. The others were made mono, trimmed, shortened and\nresampled. Thank you, Versilian Studios. The voices at the end were\nrecorded for Aratu.\n\n` +
      [
        ...report.map((r) => {
          const v = PITCHED.find((p) => p.code === r.code);
          return `## ${r.code}\n\n${v?.layers ? madeOf(v, r) : ""}${r.sources.map((s) => `- ${s}`).join("\n")}\n`;
        }),
        ...KEPT.map((k) => `## ${k.code}\n\n${k.credit}\n`),
      ].join("\n"),
  );
}
