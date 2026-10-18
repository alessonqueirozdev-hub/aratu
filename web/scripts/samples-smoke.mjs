// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
// Checks the sampled voices in public/samples/ against what SOURCES.md and
// the manual say of them: every file a voice.sfz or an index.json names is
// there; in a voice made from its layers the layers share out the
// velocities, 0 to 127, on every key it plays (every key of a pitched
// voice, each drum's key in a kit), round robins take turns in full, the
// FLAC and Ogg Vorbis files keep their source's rate and channels, a
// voice that sustains loops inside its recordings, and a pitched voice's
// notes sound where they are mapped (the pitches samples.mjs heard, with
// the voice's `tune`, within a semitone for every layer and, for a voice
// that is tuned, within 25 cents of A 440 overall).
import assert from "node:assert/strict";
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const web = join(dirname(fileURLToPath(import.meta.url)), "..");
const samples = join(web, "public", "samples");

/** The VCSL voices, made from their dynamic layers: how many layers (a
 * kit's for each drum's key), the rate and channels of the source, and
 * whether they loop. */
const LAYERED = {
  "piano.grand": { layers: 3, rate: 44100, channels: 2 },
  "piano.upright": { layers: 3, rate: 44100, channels: 2 },
  "keys.harpsichord": { layers: 1, rate: 44100, channels: 2 },
  "organ.pipe": { layers: 2, rate: 44100, channels: 2, loop: true },
  harp: { layers: 2, rate: 44100, channels: 2 },
  "mallets.marimba": { layers: 3, rate: 44100, channels: 2 },
  "mallets.vibraphone": { layers: 2, rate: 44100, channels: 2 },
  "mallets.xylophone": { layers: 2, rate: 44100, channels: 2 },
  "mallets.glockenspiel": { layers: 3, rate: 44100, channels: 2 },
  "bells.tubular": { layers: 2, rate: 44100, channels: 2 },
  "mallets.kalimba": { layers: 1, rate: 48000, channels: 2 },
  harmonica: { layers: 2, rate: 44100, channels: 1, loop: true },
  "recorder.alto": { layers: 1, rate: 48000, channels: 2, loop: true },
  "sax.tenor": { layers: 2, rate: 48000, channels: 2, loop: true },
  // Kick, snare, side stick, closed and open hat, ride, crash, high, mid
  // and floor tom.
  "drums.acoustic": { layers: { 36: 3, 38: 3, 37: 1, 42: 3, 46: 1, 51: 3, 49: 3, 50: 3, 47: 3, 43: 3 }, rate: 44100, channels: 2 },
};

/** Rate, channels and length in frames from a FLAC file's STREAMINFO. */
function flacInfo(b) {
  assert.equal(b.toString("ascii", 0, 4), "fLaC");
  assert.equal(b[4] & 0x7f, 0, "STREAMINFO comes first");
  const at = 8 + 10;
  const rate = (b[at] << 12) | (b[at + 1] << 4) | (b[at + 2] >> 4);
  const channels = ((b[at + 2] >> 1) & 7) + 1;
  const frames = (b[at + 3] & 0x0f) * 2 ** 32 + b.readUInt32BE(at + 4);
  return { rate, channels, frames };
}

/** Rate and channels from an Ogg Vorbis file's identification header. */
function vorbisInfo(b) {
  assert.equal(b.toString("ascii", 0, 4), "OggS");
  const at = 27 + b[26];
  assert.equal(b.toString("ascii", at, at + 7), "\x01vorbis");
  return { channels: b[at + 11], rate: b.readUInt32LE(at + 12), frames: null };
}

/** The regions of an SFZ text, each with what its <global> and <group>
 * give it (the subset samples.mjs and the kept kits write); `key` sets
 * both ends of the keys. */
function regions(text) {
  const out = [];
  let global = {};
  let group = {};
  let scope = null;
  for (const raw of text.split("\n")) {
    const line = raw.replace(/\/\/.*$/, "").trim();
    if (!line) continue;
    for (const token of line.split(/\s+/)) {
      const header = token.match(/^<(\w+)>$/);
      if (header) {
        scope = header[1];
        if (scope === "group") group = {};
        if (scope === "region") out.push({ ...global, ...group });
        continue;
      }
      const [key, value] = token.split("=");
      if (scope === "global") global[key] = value;
      else if (scope === "group") group[key] = value;
      else if (scope === "region") out[out.length - 1][key] = value;
    }
  }
  for (const r of out) if (r.key !== undefined) r.lokey = r.hikey = r.key;
  return out;
}

const voices = readdirSync(samples, { withFileTypes: true }).filter((d) => d.isDirectory()).map((d) => d.name);
for (const code of Object.keys(LAYERED)) assert.ok(voices.includes(code), `${code} is missing`);
for (const code of voices) {
  const dir = join(samples, code);
  const sfz = readFileSync(join(dir, "voice.sfz"), "utf8");
  const base = sfz.match(/default_path=(\S+)/)?.[1] ?? "";
  const all = regions(sfz);
  assert.ok(all.length > 0, `${code}: no regions`);
  for (const r of all) assert.ok(existsSync(join(dir, base, r.sample)), `${code}: ${r.sample} is missing`);
  const index = JSON.parse(readFileSync(join(dir, "index.json"), "utf8"));
  const named = index.kind === "kit" ? Object.values(index.hits) : index.notes.map((n) => n.file);
  for (const f of named) assert.ok(existsSync(join(dir, f)), `${code}: index.json names ${f}, which is missing`);

  const voice = LAYERED[code];
  if (!voice) continue;
  // The keys it plays: every key, or each drum's.
  const kit = typeof voice.layers === "object";
  const keys = kit ? Object.keys(voice.layers).map(Number) : Array.from({ length: 128 }, (_, k) => k);
  if (kit) assert.deepEqual([...new Set(all.map((r) => Number(r.key)))].sort((a, b) => a - b), [...keys].sort((a, b) => a - b), `${code}: its drums' keys`);
  if (!kit) {
    // Each layer's notes sound where they are mapped: many VCSL files are
    // named an octave below what they sound.
    assert.ok(Array.isArray(index.heard) && index.heard.length > 0, `${code}: index.json has no pitches heard`);
    for (let l = 0; l < voice.layers; l++) {
      const off = index.heard
        .filter((h) => h.layer === l)
        .map((h) => h.heard + (index.tune ?? 0) / 100 - h.note)
        .sort((a, b) => a - b);
      assert.ok(off.length > 0, `${code}: layer ${l + 1} was not heard`);
      const median = off[Math.floor(off.length / 2)];
      assert.ok(Math.abs(median) <= 1, `${code}: layer ${l + 1} sounds ${median.toFixed(2)} semitones from its notes`);
    }
    if (index.tune) {
      // A voice recorded at another A is brought to A 440, in its SFZ too.
      const all = index.heard.map((h) => h.heard + index.tune / 100 - h.note).sort((a, b) => a - b);
      const median = all[Math.floor(all.length / 2)];
      assert.ok(Math.abs(median) <= 0.25, `${code}: tuned ${index.tune} cents, it is still ${median.toFixed(2)} semitones off`);
      assert.match(sfz, new RegExp(`<global>[^\\n]*tune=${index.tune}\\b`), `${code}: its SFZ is not tuned ${index.tune} cents`);
    }
    const heard = new Set(index.heard.map((h) => h.note));
    for (const r of all) assert.ok(heard.has(Number(r.pitch_keycenter)), `${code}: ${r.sample} is on ${r.pitch_keycenter}, a note that was not heard`);
  }
  for (const key of keys) {
    const here = all.filter((r) => Number(r.lokey) <= key && key <= Number(r.hikey));
    const layers = kit ? voice.layers[key] : voice.layers;
    // The layers share out the velocities, in order, without a gap.
    const groups = [...new Map(here.map((r) => [`${r.lovel}-${r.hivel}`, [Number(r.lovel), Number(r.hivel)]])).values()];
    groups.sort((a, b) => a[0] - b[0]);
    assert.equal(groups.length, layers, `${code}: key ${key} has ${groups.length} layers, not ${layers}`);
    assert.equal(groups[0][0], 0, `${code}: key ${key}'s softest layer starts at velocity 0`);
    assert.equal(groups[groups.length - 1][1], 127, `${code}: key ${key}'s loudest layer reaches 127`);
    groups.slice(1).forEach(([lo], i) => assert.equal(lo, groups[i][1] + 1, `${code}: key ${key} has a gap or overlap in velocities at ${lo}`));
    for (const [lo, hi] of groups) {
      // Each layer plays the key once in each turn of its round robins.
      const layer = here.filter((r) => Number(r.lovel) === lo && Number(r.hivel) === hi);
      const length = Number(layer[0].seq_length ?? 1);
      const turns = layer.map((r) => Number(r.seq_position ?? 1)).sort((a, b) => a - b);
      assert.ok(layer.every((r) => Number(r.seq_length ?? 1) === length), `${code}: key ${key} mixes round robins`);
      assert.deepEqual(turns, Array.from({ length }, (_, i) => i + 1), `${code}: key ${key} at ${lo}-${hi} has turns ${turns}`);
      // Different recordings from layer to layer, not one recording louder.
      assert.ok(layer.every((r) => all.filter((o) => o.sample === r.sample).length === 1), `${code}: a recording in two layers`);
    }
    if (layers < 2) continue;
    // What the manual says the dynamics play (theory.rs: p 49, mf 80, f 96,
    // an accent 32 more): p plays the softest layer, mf not yet the
    // loudest, and f, ff and an accented mf the loudest.
    const layerOf = (velocity) => groups.findIndex(([lo, hi]) => lo <= velocity && velocity <= hi);
    assert.equal(layerOf(49), 0, `${code}: p plays key ${key}'s softest layer`);
    assert.ok(layerOf(80) < layers - 1, `${code}: mf plays a softer layer of key ${key} than the loudest`);
    for (const velocity of [96, 112, 80 + 32]) assert.equal(layerOf(velocity), layers - 1, `${code}: velocity ${velocity} plays key ${key}'s loudest layer`);
    // On a kit, a ghost note (schedule.rs: 0.35 of its velocity) plays the
    // softest, even at fff.
    if (kit) assert.equal(layerOf(Math.round(127 * 0.35)), 0, `${code}: a ghost note plays key ${key}'s softest layer`);
  }
  // At the source's rate and in its channels; a voice that sustains loops
  // inside each recording, and the stand-in with it.
  const frames = new Map();
  for (const r of all) {
    const bytes = readFileSync(join(dir, base, r.sample));
    const info = r.sample.endsWith(".flac") ? flacInfo(bytes) : r.sample.endsWith(".ogg") ? vorbisInfo(bytes) : null;
    assert.ok(info, `${code}: ${r.sample} is neither FLAC nor Ogg Vorbis`);
    assert.deepEqual({ rate: info.rate, channels: info.channels }, { rate: voice.rate, channels: voice.channels }, `${code}: ${r.sample}`);
    frames.set(r.sample, info.frames);
    if (!voice.loop) {
      assert.ok(r.loop_start === undefined, `${code}: ${r.sample} loops`);
      continue;
    }
    assert.equal(r.loop_mode, "loop_continuous", `${code}: ${r.sample} does not loop`);
    const [start, end] = [Number(r.loop_start), Number(r.loop_end)];
    assert.ok(start > 0 && start < end, `${code}: ${r.sample} loops from ${start} to ${end}`);
    if (info.frames !== null) assert.ok(end < info.frames, `${code}: ${r.sample}'s loop ends at ${end}, past its ${info.frames} frames`);
  }
  if (voice.loop) {
    for (const n of index.notes) {
      assert.ok(n.loop && n.loop[0] < n.loop[1], `${code}: the stand-in's ${n.file} does not loop`);
      const length = frames.get(n.file);
      if (length) assert.ok(Math.round(n.loop[1] * voice.rate) <= length, `${code}: the stand-in's loop of ${n.file} ends past it`);
    }
  }
}
// Notes recorded more than once take turns: the upright piano's, and the
// acoustic kit's drums.
for (const code of ["piano.upright", "drums.acoustic"]) {
  assert.ok(regions(readFileSync(join(samples, code, "voice.sfz"), "utf8")).some((r) => Number(r.seq_length) > 1), `${code} has round robins`);
}
// The acoustic kit's closed hat chokes its open hat, at every velocity.
const acoustic = regions(readFileSync(join(samples, "drums.acoustic", "voice.sfz"), "utf8"));
assert.ok(acoustic.filter((r) => r.key === "42").every((r) => r.group === "2"), "drums.acoustic: the closed hat is in group 2");
assert.ok(acoustic.filter((r) => r.key === "46").every((r) => r.off_by === "2"), "drums.acoustic: the closed hat cuts the open hat");
console.log("samples ok");
