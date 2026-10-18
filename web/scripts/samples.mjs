// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
// Prepares the sampled voices from the Versilian Community Sample Library
// (VCSL, CC0: https://github.com/sgossner/VCSL) and writes them, with an
// index per voice, into public/samples/. The editor fetches a voice the
// first time a piece uses it. Every voice keeps its recordings as they were
// made, at their own rate and in their own channels: every dynamic layer,
// mapped to a range of velocities, and every round robin, taking turns,
// trimmed and faded, or looped where they hold steady, and written as FLAC
// or Ogg Vorbis. The voices in KEPT were made elsewhere: their folders are
// left as they are. Run by hand and commit the result:
//
//   node scripts/samples.mjs [voice ...]   (downloads into .samples-cache/)
//
// Naming voices remakes only those and leaves SOURCES.md as it is.
//
// WAV reading and writing are below. FLAC and Ogg Vorbis are written by
// Manguebyte's encoders (manguebyte-formats, MIT OR Apache-2.0), which the
// script builds into the cache with cargo.

import { execFileSync } from "node:child_process";
import { existsSync, mkdirSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const web = join(dirname(fileURLToPath(import.meta.url)), "..");
const cache = join(web, ".samples-cache");
const out = join(web, "public", "samples");
const RAW = "https://raw.githubusercontent.com/sgossner/VCSL/master/";

/** Pitched voices: notes picked at least `spacing` semitones apart, at the
 * source's rate and in its channels. `layers` are patterns of its files,
 * softest first, one for each dynamic it was recorded at (one layer when
 * it has none); `rr` finds the round robin a file is. `seconds` may run
 * from the lowest note to the highest; a voice that sustains (`loop`)
 * loops where its notes hold steady, as long as a key is held. `format` is
 * "flac" or a Vorbis quality ("q7"). Every voice is FLAC for now: on the
 * mallets, Manguebyte's Vorbis encoder at quality 7 took out everything
 * above 16 kHz and most of 8 to 16 kHz, the brightness the layers are for,
 * which FLAC keeps for four times the bytes. `normalized` marks a source
 * whose layers come at one level already (the grand's NORMALIZED.txt asks
 * that their gains be left alone).
 *
 * Many sources name their files an octave below the pitch they sound (a
 * file "C4" that sounds C5, 523 Hz): `shift` moves their notes up to the
 * pitch heard, 12 semitones. Every voice is measured as it is made
 * (`heardPitch`), and a layer whose notes do not sound within a semitone
 * of where they are mapped stops the script; the lowest tubular bell, named
 * C3, is left out, heard about a fourth above it in both layers. Several
 * sources were tuned to A 430, 40 cents under the A 440 every other voice
 * plays at: a voice whose
 * layers agree that it sounds 20 cents or more off is tuned back to A 440
 * (`tune` in its SFZ and its index.json).
 *
 * Some sources have no dynamics to split: the harpsichord's High and Low
 * are its two registers at one pitch, the recorder's other folders a
 * vibrato and a staccato. The organ's Quiet and Loud are two registrations
 * of its third manual, played as an organist adds stops for forte; the
 * harmonica's Accented notes are its loud layer. */
const PITCHED = [
  { code: "piano.grand", dir: "Chordophones/Zithers/Grand Piano, Steinway B", include: /^NoSus\//, layers: [/_vl2_/, /_vl3_/, /_vl4_/], rr: /_rr(\d+)/, normalized: true, spacing: 4, seconds: [5, 2], fade: 1, format: "flac", range: [24, 103] },
  { code: "piano.upright", dir: "Chordophones/Zithers/Upright Piano, Yamaha", include: /^Sustains\//, layers: [/_vl1_/, /_vl2_/, /_vl3_/], rr: /_rr(\d+)/, shift: 12, spacing: 4, seconds: [4, 1.8], fade: 1, format: "flac", range: [24, 103] },
  { code: "keys.harpsichord", dir: "Chordophones/Zithers/Harpsichord, Flemish", include: /^Sustains\/(High|Low)\/.*_rr1\.wav$/, shift: 12, spacing: 4, seconds: [2.5, 1.2], fade: 0.8, format: "flac" },
  { code: "organ.pipe", dir: "Aerophones/Edge-blown Aerophones/Pipe Organ", include: /^(Quiet|Loud)\//, layers: [/^Quiet\//, /^Loud\//], shift: 12, spacing: 6, seconds: 1.5, loop: true, format: "flac" },
  { code: "harp", dir: "Chordophones/Composite Chordophones/Concert Harp", include: /\.wav$/, layers: [/_(p|mp|mf)\d\.wav$/, /_f\d\.wav$/], spacing: 3, seconds: [4, 1.5], fade: 0.8, format: "flac" },
  { code: "mallets.marimba", dir: "Idiophones/Struck Idiophones/Marimba", include: /\.wav$/, layers: [/_soft_/, /_med_/, /_loud_/], shift: 12, spacing: 3, seconds: [2.5, 1.2], fade: 0.5, format: "flac" },
  { code: "mallets.vibraphone", dir: "Idiophones/Struck Idiophones/Vibraphone", include: /^Hard Mallets\/.*_Main\.wav$/, layers: [/_v2_/, /_v3_/], rr: /_rr(\d+)/, shift: 12, spacing: 3, seconds: [4, 2.5], fade: 1, format: "flac" },
  { code: "mallets.xylophone", dir: "Idiophones/Struck Idiophones/Xylophone", include: /^Medium Mallets\/.*_far\.wav$/, layers: [/_pp_/, /_ff_/], shift: 12, spacing: 3, seconds: 1.5, fade: 0.4, format: "flac" },
  { code: "mallets.glockenspiel", dir: "Idiophones/Struck Idiophones/Glockenspiel", include: /\.wav$/, layers: [/_soft_/, /_medium_/, /_loud_/], shift: 12, spacing: 3, seconds: [3, 2], fade: 0.8, format: "flac" },
  { code: "bells.tubular", dir: "Idiophones/Struck Idiophones/Tubular Bells 1", include: /\.wav$/, layers: [/_(pp|p)_/, /_(f|ff|fff)_/], shift: 12, spacing: 4, seconds: 4, fade: 2, format: "flac", range: [64, 127] },
  { code: "mallets.kalimba", dir: "Idiophones/Plucked Idiophones/Kalimba, Kenya", include: /\.wav$/, shift: 12, spacing: 2, seconds: [2.5, 1.5], fade: 0.8, format: "flac" },
  { code: "harmonica", dir: "Aerophones/Free Aerophones/Harmonica-Hohner-Super64", include: /^Sustains\/(Normal|Accented)\//, layers: [/\/Normal\//, /\/Accented\//], shift: 12, spacing: 3, seconds: 1.2, loop: true, format: "flac" },
  { code: "recorder.alto", dir: "Aerophones/Edge-blown Aerophones/Baroque Alto Recorder", include: /^Sustain\//, shift: 12, spacing: 2, seconds: 1.5, loop: true, format: "flac" },
  { code: "sax.tenor", dir: "Aerophones/Reed Aerophones/Tenor Saxophone", include: /^Non-Vibrato\//, layers: [/_vl2_/, /_vl3_/], rr: /_rr(\d+)/, shift: 12, spacing: 3, seconds: 1.1, loop: true, format: "flac" },
];

/** The acoustic kit: each drum the language names, with its dynamics and
 * round robins (at most `takes` of each), on its General MIDI key. The
 * middle of three layers, which mf plays, is the stroke the kick, snare,
 * hi-hat, crash and high and mid toms had when the kit had one; the ride
 * is the suspended cymbal's bell, as it was. */
const KIT = {
  code: "drums.acoustic",
  format: "flac",
  drums: {
    kick: { dir: "Membranophones/Struck Membranophones/Bass Drum 1", include: /^BDrumNew_hit_/, layers: [/_v3_/, /_v5_/, /_v7_/], rr: /_rr(\d+)/, seconds: 1.2 },
    snare: { dir: "Membranophones/Struck Membranophones/Snare Drum, Modern 1", include: /^Snare2_HitSN_/, layers: [/_v3_/, /_v5_/, /_v7_/], rr: /_rr(\d+)/, seconds: 0.8 },
    "closed-hat": { dir: "Idiophones/Struck Idiophones/Hi-Hat Cymbal", include: /^HiHat_HitC_/, layers: [/_v1_/, /_v3_/, /_v4_/], rr: /_rr(\d+)/, seconds: 0.5 },
    "open-hat": { dir: "Idiophones/Struck Idiophones/Hi-Hat Cymbal", include: /^HiHat_HitO_rr/, rr: /_rr(\d+)/, seconds: 1.5 },
    ride: { dir: "Idiophones/Struck Idiophones/Suspended Cymbal 1", include: /^susCymb1_hit_bell_/, layers: [/_pp\d/, /_mf\d/, /_fff\d/], seconds: 2.5 },
    crash: { dir: "Idiophones/Struck Idiophones/Clash Cymbals 1", include: /^cymbal_crash1_/, layers: [/_pp\d/, /_mf\d/, /_ff\d/], rr: /(\d)\.wav$/, takes: 1, seconds: 2.5 },
    "high-tom": { dir: "Membranophones/Struck Membranophones/Tom 1", include: /^Stick\/TomH_HitS_/, layers: [/_v2_/, /_v3_/, /_v4_/], rr: /_rr(\d+)/, seconds: 1 },
    "mid-tom": { dir: "Membranophones/Struck Membranophones/Tom 2", include: /^Stick\/TomL_HitS_/, layers: [/_v2_/, /_v3_/, /_v4_/], rr: /_rr(\d+)/, seconds: 1 },
    "floor-tom": { dir: "Membranophones/Struck Membranophones/Legacy Toms", include: /^tenor_lower\//, layers: [/_pp_/, /_mf_/, /_ff_/], rr: /_rr(\d+)/, takes: 2, seconds: 1.2 },
    "side-stick": { dir: "Membranophones/Struck Membranophones/Legacy Snares", include: /^drum1\/snare1_click\d*\.wav$/, rr: /click(\d*)\.wav$/, seconds: 0.4 },
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

/** Reads a PCM or float WAV into its channels. */
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
      const channels = Array.from({ length: fmt.channels }, () => new Float32Array(frames));
      for (let i = 0; i < frames; i++) {
        for (let c = 0; c < fmt.channels; c++) {
          const at = body + (i * fmt.channels + c) * bytes;
          let v;
          if (fmt.format === 3) v = bytes === 4 ? buf.readFloatLE(at) : buf.readDoubleLE(at);
          else if (bytes === 2) v = buf.readInt16LE(at) / 32768;
          else if (bytes === 3) v = buf.readIntLE(at, 3) / 8388608;
          else if (bytes === 4) v = buf.readInt32LE(at) / 2147483648;
          else v = (buf[at] - 128) / 128;
          channels[c][i] = v;
        }
      }
      return { rate: fmt.rate, channels };
    }
    pos = body + size + (size % 2);
  }
  throw new Error("no audio in WAV");
}

/** The frame just before a recording's sound starts: 3 ms before the
 * first that comes to 2% of its peak, in any channel. */
function onset(channels, rate) {
  const loudest = (i) => channels.reduce((m, d) => Math.max(m, Math.abs(d[i])), 0);
  let peak = 0;
  for (let i = 0; i < channels[0].length; i++) peak = Math.max(peak, loudest(i));
  let start = 0;
  while (start < channels[0].length && loudest(start) < peak * 0.02) start++;
  return Math.max(0, start - Math.round(rate * 0.003));
}

/** The channels of a recording, from just before the sound starts, at most
 * `seconds` long, the last `fade` seconds faded out. */
function shapeChannels(channels, rate, seconds, fade) {
  const start = onset(channels, rate);
  const len = Math.min(channels[0].length - start, Math.round(rate * seconds));
  const n = Math.min(Math.round(rate * fade), Math.floor(len / 2));
  return channels.map((d) => {
    const s = d.slice(start, start + len);
    // A raised-cosine fade: the decay goes on into silence, without a corner.
    for (let i = 0; i < n; i++) s[len - n + i] *= 0.5 + 0.5 * Math.cos((Math.PI * i) / n);
    return s;
  });
}

/** A sustained recording from just before the sound starts, at most
 * `seconds` long, looped where it holds steady: from 40% of the way in to
 * an end, among the last 40 ms, whose frames before it best match, in
 * every channel at once, the frames before the loop's start. Those frames
 * then fade into the ones before the start, so that the loop goes round
 * without a click; the recording stops where the loop ends. The loop is
 * [start, end), in frames. */
function loopChannels(channels, rate, seconds) {
  const begin = onset(channels, rate);
  const len = Math.min(channels[0].length - begin, Math.round(rate * seconds));
  const d = channels.map((c) => c.slice(begin, begin + len));
  const from = Math.round(len * 0.4);
  const window = Math.round(rate * 0.02);
  let end = len;
  let best = -Infinity;
  for (let e = len - Math.round(rate * 0.04); e <= len; e++) {
    let dot = 0, a = 0, b = 0;
    for (const c of d) {
      for (let i = 1; i <= window; i++) {
        dot += c[e - i] * c[from - i];
        a += c[e - i] * c[e - i];
        b += c[from - i] * c[from - i];
      }
    }
    const score = dot / Math.sqrt(a * b + 1e-30);
    if (score > best) {
      best = score;
      end = e;
    }
  }
  const fade = Math.min(Math.round(rate * 0.1), Math.floor((end - from) / 3), from);
  for (const c of d) {
    for (let i = 0; i < fade; i++) {
      const t = (i + 1) / fade;
      const at = end - fade + i;
      c[at] = c[at] * (1 - t) + c[from - fade + i] * t;
    }
  }
  return { channels: d.map((c) => c.slice(0, end)), loop: [from, end] };
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

/** The highest velocity of each layer but the loudest, from Aratu's
 * dynamics (theory.rs: ppp 16, pp 33, p 49, mp 64, mf 80, f 96, ff 112,
 * fff 127, an accent 32 more; a ghost note a third of its dynamic). Of two
 * layers the soft one plays up to mf; of three, the softest plays p and
 * under, the middle one mp and mf, the loudest f and over, and an accent on
 * mf. */
const SPLITS = { 1: [], 2: [88], 3: [59, 95] };

/** The velocities of layer `l` of `count`. */
function velocities(count, l) {
  const tops = [...SPLITS[count], 127];
  return [l === 0 ? 0 : tops[l - 1] + 1, tops[l]];
}

/** The layer an unmarked note (mf, 80) plays, which the stand-in plays. */
const mfLayer = (count) => [...SPLITS[count], 127].findIndex((t) => 80 <= t);

/** How many semitones a layer's notes sound above their names. */
const shiftOf = (v) => v.shift ?? 0;

/** The magnitudes of a stretch's spectrum, Hann-windowed (a radix-2 FFT;
 * the length is a power of two). */
function magnitudes(x) {
  const n = x.length;
  const re = Float64Array.from(x, (v, i) => v * (0.5 - 0.5 * Math.cos((2 * Math.PI * i) / n)));
  const im = new Float64Array(n);
  for (let i = 1, j = 0; i < n; i++) {
    let bit = n >> 1;
    for (; j & bit; bit >>= 1) j ^= bit;
    j ^= bit;
    if (i < j) {
      [re[i], re[j]] = [re[j], re[i]];
      [im[i], im[j]] = [im[j], im[i]];
    }
  }
  for (let len = 2; len <= n; len <<= 1) {
    const half = len >> 1;
    const wr = Math.cos((-2 * Math.PI) / len);
    const wi = Math.sin((-2 * Math.PI) / len);
    for (let i = 0; i < n; i += len) {
      let cr = 1;
      let ci = 0;
      for (let k = 0; k < half; k++) {
        const a = i + k;
        const b = a + half;
        const tr = re[b] * cr - im[b] * ci;
        const ti = re[b] * ci + im[b] * cr;
        re[b] = re[a] - tr;
        im[b] = im[a] - ti;
        re[a] += tr;
        im[a] += ti;
        const t = cr * wr - ci * wi;
        ci = cr * wi + ci * wr;
        cr = t;
      }
    }
  }
  return Float64Array.from({ length: n / 2 }, (_, k) => Math.hypot(re[k], im[k]));
}

/** The pitch a recording is heard at, as a MIDI note with cents, 50 ms
 * in: the fundamental whose harmonics best explain its spectrum
 * (subharmonic summation, Hermes 1988). Unlike an autocorrelation it hears
 * the strike note of a bell or a bar, whose partials are not harmonics,
 * and the organ's main stop over a weaker one an octave below it. It hears
 * the grand piano, named right, at its names, within 0.2 of a semitone. */
function heardPitch(channels, rate) {
  const start = Math.round(rate * 0.05);
  let n = 16384;
  while (n > 2048 && start + n > channels[0].length) n >>= 1;
  let x = null;
  let energy = -1;
  for (const c of channels) {
    const s = c.subarray(start, start + n);
    if (s.length < n) continue;
    const e = s.reduce((a, v) => a + v * v, 0);
    if (e > energy) {
      energy = e;
      x = s;
    }
  }
  if (!x) return null;
  const mag = magnitudes(x);
  const bin = rate / n;
  // The strongest bin within a quarter tone of f.
  const near = (f) => {
    const lo = Math.max(1, Math.floor((f * Math.pow(2, -1 / 24)) / bin));
    const hi = Math.min(mag.length - 1, Math.ceil((f * Math.pow(2, 1 / 24)) / bin));
    let m = 0;
    for (let k = lo; k <= hi; k++) m = Math.max(m, mag[k]);
    return m;
  };
  let best = 0;
  let heard = 0;
  for (let f0 = 25; f0 <= 4500; f0 *= Math.pow(2, 1 / 48)) {
    let s = 0;
    for (let k = 1; k <= 10 && k * f0 < rate / 2; k++) s += Math.pow(0.84, k - 1) * near(k * f0);
    if (s > best) {
      best = s;
      heard = f0;
    }
  }
  // Then finer, around the best.
  const coarse = heard;
  for (let f0 = coarse * Math.pow(2, -1 / 48); f0 <= coarse * Math.pow(2, 1 / 48); f0 *= Math.pow(2, 1 / 1200)) {
    let s = 0;
    for (let k = 1; k <= 10 && k * f0 < rate / 2; k++) s += Math.pow(0.84, k - 1) * near(k * f0);
    if (s > best) {
      best = s;
      heard = f0;
    }
  }
  return 69 + 12 * Math.log2(heard / 440);
}

const median = (list) => [...list].sort((a, b) => a - b)[Math.floor(list.length / 2)];

/** The cents that bring a voice to A 440: when its notes sound 20 cents
 * or more off, every layer agrees, within 30 cents, on how far, and in
 * every layer at least 70% of the notes were heard within a semitone of
 * where they are mapped. Where the layers of one recorded instrument
 * disagree, or a layer was heard here and there (a bell struck softly),
 * what is off is the measure, not the instrument. 0 when it plays in tune. */
function tuneOf(heard) {
  const off = median(heard.map((h) => h.heard - h.note));
  const layers = [...new Set(heard.map((h) => h.layer))].map((l) => median(heard.filter((h) => h.layer === l).map((h) => h.heard - h.note)));
  const steady = [...new Set(heard.map((h) => h.layer))].every((l) => {
    const here = heard.filter((h) => h.layer === l);
    return here.filter((h) => Math.abs(h.heard - h.note) <= 1).length >= 0.7 * here.length;
  });
  const agree = steady && Math.max(...layers) - Math.min(...layers) <= 0.3;
  return agree && Math.abs(off) >= 0.2 ? -Math.round(off * 100) : 0;
}

/** The recordings of a voice, or of a drum, that `include` takes: their
 * note, their layer and their take, in that order. A note's takes in a
 * layer are numbered from 1 in the order `rr` finds them, at most `takes`
 * of them. */
function recordingsOf(files, v, noteOf) {
  const layers = v.layers ?? [/(?:)/];
  const found = files
    .filter((f) => f.path.startsWith(v.dir + "/"))
    .map((f) => ({ ...f, rel: f.path.slice(v.dir.length + 1) }))
    .filter((f) => v.include.test(f.rel) && f.rel.endsWith(".wav"))
    .map((f) => ({ ...f, layer: layers.findIndex((re) => re.test(f.rel)), turn: v.rr ? Number(f.rel.match(v.rr)?.[1] || 1) : 1 }))
    .map((f) => {
      const named = noteOf(f.rel);
      return { ...f, note: named === null ? null : named + shiftOf(v) };
    })
    .filter((f) => f.note !== null && f.layer >= 0 && (!v.range || (f.note >= v.range[0] && f.note <= v.range[1])))
    .sort((a, b) => a.note - b.note || a.layer - b.layer || a.turn - b.turn || a.rel.localeCompare(b.rel))
    // Takes that are not round robins (a second take of a note): the first.
    .filter((f, i, all) => i === 0 || f.note !== all[i - 1].note || f.layer !== all[i - 1].layer || f.turn !== all[i - 1].turn);
  let take = 0;
  return found
    .map((f, i) => {
      take = i > 0 && f.note === found[i - 1].note && f.layer === found[i - 1].layer ? take + 1 : 1;
      return { ...f, take };
    })
    .filter((f) => !v.takes || f.take <= v.takes);
}

/** Downloads `picked`, at one rate, and trims each, or loops it. Each note
 * of a softer layer is brought to the loudness of the loudest layer's
 * nearest note, its takes by one gain, so that they keep their
 * differences; nothing moves in a source whose layers were brought to one
 * level already, and that says not to move them. Then one gain for them
 * all, for their loudest peak. */
async function prepare(picked, v, secondsAt) {
  const shaped = [];
  let rate = null;
  for (const f of picked) {
    const wav = readWav(await download(f.path));
    if (rate !== null && wav.rate !== rate) throw new Error(`${v.code ?? v.dir}: ${f.rel} is ${wav.rate} Hz, not ${rate}`);
    rate = wav.rate;
    const channels = wav.channels.slice(0, 2);
    const s = v.loop ? loopChannels(channels, rate, secondsAt(f.note)) : { channels: shapeChannels(channels, rate, secondsAt(f.note), v.fade ?? 0.3), loop: null };
    shaped.push({ f, ...s, loudness: loudnessOf(s.channels, rate) });
  }
  const top = (v.layers ?? [null]).length - 1;
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
  const raw = (s) => Math.pow(10, gainDb.get(s) / 20);
  const peak = Math.max(...shaped.map((s) => raw(s) * Math.max(...s.channels.map(peakOf))));
  const gainOf = (s) => (raw(s) * 0.9) / peak;
  const gainsDb = Array.from({ length: top + 1 }, (_, l) => mean(shaped.filter((s) => s.f.layer === l).map((s) => gainDb.get(s))).toFixed(1));
  return { shaped, rate, gainOf, gainsDb };
}

/** A layered voice as SFZ: each layer a group of velocities, its notes
 * covering the keys nearest to them, round robins taking turns, a held
 * note going round its loop (`loop_end` is the loop's last frame). */
function layeredSfz(layers, tune) {
  const lines = ["// Made by Aratu's web/scripts/samples.mjs from VCSL (CC0).", "<control> default_path=./", `<global> ampeg_release=0.12${tune ? ` tune=${tune}` : ""}`];
  layers.forEach((notes, l) => {
    const [lovel, hivel] = velocities(layers.length, l);
    lines.push(`<group> lovel=${lovel} hivel=${hivel}`);
    notes.forEach((n, i) => {
      const lo = i === 0 ? 0 : Math.floor((notes[i - 1].note + n.note) / 2) + 1;
      const hi = i === notes.length - 1 ? 127 : Math.floor((n.note + notes[i + 1].note) / 2);
      n.takes.forEach(({ file, loop }, t) => {
        const turns = n.takes.length > 1 ? ` seq_length=${n.takes.length} seq_position=${t + 1}` : "";
        const held = loop ? ` loop_mode=loop_continuous loop_start=${loop[0]} loop_end=${loop[1] - 1}` : "";
        lines.push(`<region> sample=${file} pitch_keycenter=${n.note} lokey=${lo} hikey=${hi}${turns}${held}`);
      });
    });
  });
  return lines.join("\n") + "\n";
}

/** The kit as SFZ: each drum on its General MIDI key, a group of
 * velocities for each of its layers, its round robins taking turns. */
// "guitar-tap" (the flamenco golpe) is 88 in the contract with Manguebyte, past
// every GM and GM2 percussion key; VCSL has no recording of it yet.
const GM = { kick: 36, snare: 38, "side-stick": 37, "closed-hat": 42, "open-hat": 46, ride: 51, crash: 49, "high-tom": 50, "mid-tom": 47, "floor-tom": 43, "guitar-tap": 88 };
function kitSfz(drums) {
  const lines = ["// Made by Aratu's web/scripts/samples.mjs from VCSL (CC0).", "<control> default_path=./", "<global> loop_mode=one_shot"];
  for (const { name, layers } of drums) {
    // The closed hat (group 2) chokes the open one.
    const choke = name === "open-hat" ? " group=1 off_by=2" : name === "closed-hat" ? " group=2" : "";
    layers.forEach((takes, l) => {
      const [lovel, hivel] = velocities(layers.length, l);
      lines.push(`<group> lovel=${lovel} hivel=${hivel}`);
      takes.forEach((file, t) => {
        const turns = takes.length > 1 ? ` seq_length=${takes.length} seq_position=${t + 1}` : "";
        lines.push(`<region> sample=${file} key=${GM[name]}${choke}${turns}`);
      });
    });
  }
  return lines.join("\n") + "\n";
}

/** A voice with every dynamic layer and round robin of its notes, at the
 * source's rate and channels. A softer layer's notes are as loud as the
 * loudest layer's, so that velocity sets the level as it does in every
 * sampler part (the SFZ square law; Manguebyte's loudness normalisation
 * measures at velocity 0.8, in the loudest layer) and the layer only
 * changes the timbre; the loudest layer's notes keep their relative
 * levels. */
async function pitched(files, v) {
  const inDir = recordingsOf(files, v, noteIn);
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
  const { shaped, rate, gainOf, gainsDb } = await prepare(picked, v, secondsAt);
  // Each note's first take in each layer, heard: the median of a layer must
  // be within a semitone of its notes, or their names are off (see `shift`).
  const heard = shaped
    .filter((s) => s.f.take === 1)
    .map((s) => ({ note: s.f.note, layer: s.f.layer, heard: Math.round(heardPitch(s.channels, rate) * 100) / 100 }));
  for (let l = 0; l < (v.layers ?? [null]).length; l++) {
    const off = heard
      .filter((h) => h.layer === l)
      .map((h) => h.heard - h.note)
      .sort((a, b) => a - b);
    const median = off[Math.floor(off.length / 2)];
    if (!(Math.abs(median) <= 1)) {
      throw new Error(`${v.code}: layer ${l + 1} sounds ${median.toFixed(2)} semitones from the notes it is mapped to; its files are named off (set its shift)`);
    }
  }
  const tune = tuneOf(heard);
  const dir = join(out, v.code);
  mkdirSync(dir, { recursive: true });
  const ext = v.format === "flac" ? "flac" : "ogg";
  let bytes = 0;
  const layers = (v.layers ?? [null]).map(() => []);
  for (const s of shaped) {
    const { note, layer, take } = s.f;
    const file = `${note}_v${layer + 1}${take > 1 ? `_rr${take}` : ""}.${ext}`;
    bytes += writeEncoded(join(dir, file), s.channels, rate, gainOf(s), v.format);
    const list = layers[layer];
    if (list.length === 0 || list[list.length - 1].note !== note) list.push({ note, takes: [] });
    list[list.length - 1].takes.push({ file, loop: s.loop });
  }
  // The stand-in plays one layer, the one an unmarked note (mf) plays.
  const mf = layers[mfLayer(layers.length)].map((n) => {
    const { file, loop } = n.takes[0];
    return loop ? { note: n.note, file, loop: [loop[0] / rate, loop[1] / rate] } : { note: n.note, file };
  });
  writeFileSync(join(dir, "index.json"), JSON.stringify({ kind: "pitched", source: v.dir, notes: mf, tune, heard }));
  writeFileSync(join(dir, "voice.sfz"), layeredSfz(layers, tune));
  return { code: v.code, files: shaped.length, bytes, sources: picked.map((f) => f.path), rate, channels: shaped[0].channels.length, gainsDb, layers: layers.map((l) => l.length), tune };
}

/** The kit, each drum with every dynamic layer and round robin it was
 * recorded with (at most `takes`), at the source's rate and channels; its
 * softer layers as loud as its loudest, then one gain for the drum, its
 * loudest peak at 0.9 as every drum's was. */
async function kit(files) {
  const dir = join(out, KIT.code);
  mkdirSync(dir, { recursive: true });
  const ext = KIT.format === "flac" ? "flac" : "ogg";
  const drums = [];
  const sources = [];
  let bytes = 0;
  let count = 0;
  let rate = null;
  let channels = null;
  for (const [name, d] of Object.entries(KIT.drums)) {
    const picked = recordingsOf(files, d, () => 0);
    if (picked.length === 0) throw new Error(`${KIT.code} ${name}: no files matched`);
    const made = await prepare(picked, d, () => d.seconds);
    if (rate !== null && made.rate !== rate) throw new Error(`${KIT.code} ${name}: ${made.rate} Hz, not ${rate}`);
    rate = made.rate;
    channels = made.shaped[0].channels.length;
    const layers = (d.layers ?? [null]).map(() => []);
    for (const s of made.shaped) {
      const file = `${name}_v${s.f.layer + 1}${s.f.take > 1 ? `_rr${s.f.take}` : ""}.${ext}`;
      bytes += writeEncoded(join(dir, file), s.channels, rate, made.gainOf(s), KIT.format);
      layers[s.f.layer].push(file);
    }
    drums.push({ name, layers });
    sources.push(...picked.map((f) => f.path));
    count += picked.length;
  }
  // The stand-in plays one take of each drum, the one an unmarked hit plays.
  const hits = Object.fromEntries(drums.map(({ name, layers }) => [name, layers[mfLayer(layers.length)][0]]));
  writeFileSync(join(dir, "index.json"), JSON.stringify({ kind: "kit", hits }));
  writeFileSync(join(dir, "voice.sfz"), kitSfz(drums));
  return { code: KIT.code, files: count, bytes, sources, rate, channels, drums: drums.map(({ name, layers }) => ({ name, layers: layers.map((l) => l.length) })) };
}

/** What SOURCES.md says of how a voice was made. */
function madeOf(v, r) {
  const n = r.layers.length;
  const ranges = Array.from({ length: n }, (_, l) => velocities(n, l).join("–")).join(", ");
  const layers = n === 1 ? "one dynamic layer" : `${n} dynamic layers (velocities ${ranges})`;
  const format = v.format === "flac" ? "FLAC" : `Ogg Vorbis at quality ${v.format.slice(1)}`;
  const turns = r.files > r.layers.reduce((a, b) => a + b, 0) ? ", with the round robins recorded taking turns" : "";
  const held = v.loop ? ", each looped where it holds steady" : "";
  const named = v.shift ? " Its files are named an octave below the pitch they sound, and are mapped to the pitch heard." : "";
  const tuned = r.tune ? ` It was tuned ${Math.abs(r.tune)} cents ${r.tune > 0 ? "under" : "over"} A 440, and is tuned back to it.` : "";
  return `${r.rate / 1000} kHz, ${r.channels === 2 ? "stereo" : "mono"}, ${format}: ${layers} of ${r.layers.join(", ")} notes${turns}${held}, ${r.files} recordings.${named}${tuned}\n\n`;
}

/** What SOURCES.md says of how the kit was made. */
function madeOfKit(r) {
  const format = KIT.format === "flac" ? "FLAC" : `Ogg Vorbis at quality ${KIT.format.slice(1)}`;
  const counts = [...new Set(r.drums.map((d) => d.layers.length))].filter((n) => n > 1).sort();
  const ranges = counts.map((n) => `${n} on velocities ${Array.from({ length: n }, (_, l) => velocities(n, l).join("–")).join(", ")}`).join("; ");
  const drums = r.drums.map(({ name, layers }) => {
    const n = layers.length;
    const takes = Math.max(...layers);
    return `${name} (key ${GM[name]}) ${n === 1 ? "one layer" : `${n} layers`}${takes > 1 ? ` of ${takes} round robins taking turns` : ""}`;
  });
  return `${r.rate / 1000} kHz, ${r.channels === 2 ? "stereo" : "mono"}, ${format}: ${r.files} recordings on General MIDI keys, each drum's dynamic layers (${ranges}) brought to one loudness. ${drums.join("; ")}.\n\n`;
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
  const what = r.drums
    ? r.drums.map(({ name, layers }) => `${name} ${layers.join("/")}`).join(", ")
    : `layers ${r.layers.join("/")} notes, gains ${r.gainsDb.join("/")} dB`;
  console.log(`${r.code.padEnd(22)} ${String(r.files).padStart(3)} files ${(r.bytes / 1e6).toFixed(2)} MB ${r.rate} Hz ${r.channels} ch, ${what}`);
}
console.log(`total ${(total / 1e6).toFixed(1)} MB`);
if (only.length > 0) {
  console.log("SOURCES.md was left as it is: run without naming voices to write it.");
} else {
  const formats = [...new Set([...PITCHED, KIT].map((v) => (v.format === "flac" ? "FLAC" : "Ogg Vorbis")))];
  writeFileSync(
    join(out, "SOURCES.md"),
    `# Sampled voices\n\nMost recordings in this folder come from the Versilian Community Sample\nLibrary (VCSL), released under Creative Commons Zero (CC0):\nhttps://github.com/sgossner/VCSL and https://versilian-studios.com/vcsl/.\n\`web/scripts/samples.mjs\` made them, keeping them as they were recorded,\nat their rate and in their channels: every dynamic layer mapped to a range\nof velocities and brought to one loudness, so that the velocity sets the\nlevel and the layer the timbre, and the round robins taking turns. Trimmed\nand faded, or looped where they hold steady, they are written as\n${formats.join(" and ")} by Manguebyte's encoder${formats.length > 1 ? "s" : ""}. Thank you, Versilian Studios.\nThe voices at the end were recorded for Aratu.\n\n` +
      [
        ...report.map((r) => {
          const v = PITCHED.find((p) => p.code === r.code);
          return `## ${r.code}\n\n${v ? madeOf(v, r) : madeOfKit(r)}${r.sources.map((s) => `- ${s}`).join("\n")}\n`;
        }),
        ...KEPT.map((k) => `## ${k.code}\n\n${k.credit}\n`),
      ].join("\n"),
  );
}
