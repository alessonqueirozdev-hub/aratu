// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
// Checks the sampled voices in public/samples/ against what SOURCES.md and
// the manual say of them: every file a voice.sfz or an index.json names is
// there; in a voice with dynamic layers the layers share out the
// velocities, 0 to 127, each covering every key, round robins take turns
// in full, and the FLAC and Ogg Vorbis files keep their source's rate and
// channels (44.1 kHz stereo for every VCSL voice re-exported so far).
import assert from "node:assert/strict";
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const web = join(dirname(fileURLToPath(import.meta.url)), "..");
const samples = join(web, "public", "samples");

/** The voices recorded with dynamic layers, and how many. */
const LAYERED = {
  "piano.grand": 3,
  "piano.upright": 3,
  harp: 2,
  "mallets.marimba": 3,
  "mallets.vibraphone": 2,
  "mallets.xylophone": 2,
  "mallets.glockenspiel": 3,
};

/** Rate and channels from a FLAC file's STREAMINFO. */
function flacInfo(b) {
  assert.equal(b.toString("ascii", 0, 4), "fLaC");
  assert.equal(b[4] & 0x7f, 0, "STREAMINFO comes first");
  const at = 8 + 10;
  const rate = (b[at] << 12) | (b[at + 1] << 4) | (b[at + 2] >> 4);
  const channels = ((b[at + 2] >> 1) & 7) + 1;
  return { rate, channels };
}

/** Rate and channels from an Ogg Vorbis file's identification header. */
function vorbisInfo(b) {
  assert.equal(b.toString("ascii", 0, 4), "OggS");
  const at = 27 + b[26];
  assert.equal(b.toString("ascii", at, at + 7), "\x01vorbis");
  return { channels: b[at + 11], rate: b.readUInt32LE(at + 12) };
}

/** The regions of an SFZ text, each with what its <global> and <group>
 * give it (the subset samples.mjs and the kept kits write). */
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

  const layers = LAYERED[code];
  if (!layers) continue;
  // The layers share out the velocities, in order, without a gap.
  const groups = [...new Map(all.map((r) => [`${r.lovel}-${r.hivel}`, [Number(r.lovel), Number(r.hivel)]])).values()];
  groups.sort((a, b) => a[0] - b[0]);
  assert.equal(groups.length, layers, `${code}: ${groups.length} layers, not ${layers}`);
  assert.equal(groups[0][0], 0, `${code}: the softest layer starts at velocity 0`);
  assert.equal(groups[groups.length - 1][1], 127, `${code}: the loudest layer reaches 127`);
  groups.slice(1).forEach(([lo], i) => assert.equal(lo, groups[i][1] + 1, `${code}: a gap or overlap in velocities at ${lo}`));
  for (const [lo, hi] of groups) {
    const layer = all.filter((r) => Number(r.lovel) === lo && Number(r.hivel) === hi);
    // Each layer plays every key, once in each turn of its round robins.
    for (let key = 0; key < 128; key++) {
      const here = layer.filter((r) => Number(r.lokey) <= key && key <= Number(r.hikey));
      assert.ok(here.length > 0, `${code}: no region for key ${key} at velocities ${lo}-${hi}`);
      const length = Number(here[0].seq_length ?? 1);
      const turns = here.map((r) => Number(r.seq_position ?? 1)).sort((a, b) => a - b);
      assert.ok(here.every((r) => Number(r.seq_length ?? 1) === length), `${code}: key ${key} mixes round robins`);
      assert.deepEqual(turns, Array.from({ length }, (_, i) => i + 1), `${code}: key ${key} at ${lo}-${hi} has turns ${turns}`);
    }
    // Different recordings from layer to layer, not one recording louder.
    assert.ok(layer.every((r) => all.filter((o) => o.sample === r.sample).length === 1), `${code}: a recording in two layers`);
  }
  // What the manual says the dynamics play (theory.rs: p 49, mf 80, f 96,
  // an accent 32 more): p plays the softest layer, mf not yet the loudest,
  // and f, ff and an accented mf the loudest.
  const layerOf = (velocity) => groups.findIndex(([lo, hi]) => lo <= velocity && velocity <= hi);
  assert.equal(layerOf(49), 0, `${code}: p plays the softest layer`);
  assert.ok(layerOf(80) < layers - 1, `${code}: mf plays a softer layer than the loudest`);
  for (const velocity of [96, 112, 80 + 32]) assert.equal(layerOf(velocity), layers - 1, `${code}: velocity ${velocity} plays the loudest layer`);
  // At the source's rate and in its channels.
  for (const r of all) {
    const bytes = readFileSync(join(dir, base, r.sample));
    const info = r.sample.endsWith(".flac") ? flacInfo(bytes) : r.sample.endsWith(".ogg") ? vorbisInfo(bytes) : null;
    assert.ok(info, `${code}: ${r.sample} is neither FLAC nor Ogg Vorbis`);
    assert.deepEqual(info, { rate: 44100, channels: 2 }, `${code}: ${r.sample}`);
  }
}
// The upright piano's notes recorded twice take turns.
assert.ok(regions(readFileSync(join(samples, "piano.upright", "voice.sfz"), "utf8")).some((r) => Number(r.seq_length) > 1), "piano.upright has round robins");
console.log("samples ok");
