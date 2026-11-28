// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
// Plays Manguebyte's engine, as vendored in public/manguebyte/, in Node
// through the engine.js and worklet.js the editor loads, with a stand-in
// for the browser's AudioWorklet. It checks what the editor asks of the
// engine: every voice of the Sounds panel it plays there (its instrument,
// preset and knobs) and every sampled voice makes a sound, the drum machine
// plays every drum and percussion key, every parameter of the language that
// goes to the engine is one it has, with the knob's range inside the
// engine's, and nothing the editor sends is ignored. VERSION names the
// commit scripts/manguebyte.mjs builds. The tables are read from the
// editor's and the language's sources, so they cannot drift from this check.
import assert from "node:assert/strict";
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const web = join(dirname(fileURLToPath(import.meta.url)), "..");
const pub = join(web, "public");
const vendored = join(pub, "manguebyte");

const RATE = 48000;
const QUANTUM = 128;

// The browser's side of an AudioWorklet: what worklet.js finds around it.
let nextPort = null;
let Processor = null;
globalThis.sampleRate = RATE;
globalThis.currentFrame = 0;
globalThis.registerProcessor = (_name, processor) => (Processor = processor);
globalThis.AudioWorkletProcessor = class {
  constructor() {
    this.port = nextPort;
  }
};

const { ManguebyteEngine, INSTRUMENT } = await import(pathToFileURL(join(vendored, "engine.js")).href);
await import(pathToFileURL(join(vendored, "worklet.js")).href);

let processor = null;
const context = {
  sampleRate: RATE,
  // Not "running", so the worklet decodes samples as they arrive.
  state: "suspended",
  get currentTime() {
    return globalThis.currentFrame / RATE;
  },
  audioWorklet: { addModule: async () => {} },
};
class Node {
  constructor(_context, _name, options) {
    const channel = new MessageChannel();
    this.port = channel.port1;
    nextPort = channel.port2;
    processor = new Processor(options);
  }
}

// loadSfz fetches the voice and its samples; here they are files.
globalThis.fetch = async (href) => {
  const bytes = new Uint8Array(readFileSync(fileURLToPath(href)));
  return { ok: true, status: 200, arrayBuffer: async () => bytes.buffer };
};

const engine = await ManguebyteEngine.create(context, {
  wasm: new Uint8Array(readFileSync(join(vendored, "manguebyte_web.wasm"))),
  AudioWorkletNode: Node,
});

function render(quanta) {
  const output = [new Float32Array(QUANTUM), new Float32Array(QUANTUM)];
  for (let q = 0; q < quanta; q++) {
    processor.process([], [output], {});
    globalThis.currentFrame += QUANTUM;
  }
}

/** Renders until the part sounds, for at most about five seconds. */
async function sounds(part) {
  await engine.sync();
  for (let i = 0; i < 64; i++) {
    render(32);
    const scope = await engine.partScope(part, 32 * QUANTUM);
    assert.ok(scope, `no scope for part ${part}`);
    if (scope.left.some((s) => s !== 0) || scope.right.some((s) => s !== 0)) {
      assert.ok(scope.left.every(Number.isFinite) && scope.right.every(Number.isFinite), "a sample is not finite");
      return true;
    }
  }
  return false;
}

/** A table written in the editor as an object literal, `const NAME... = {`. */
const audio = readFileSync(join(web, "src", "audio.ts"), "utf8");
function table(name) {
  const found = audio.match(new RegExp(`^const ${name}\\b[^=]*= (\\{\\n[\\s\\S]*?\\n\\});$`, "m"));
  assert.ok(found, `src/audio.ts has no table ${name}`);
  return new Function(`return ${found[1]};`)();
}
const voices = table("MB_VOICES");
const drums = table("MB_DRUMS");
const percussion = table("PERC_KEYS");
assert.ok(Object.keys(voices).length > 50, "MB_VOICES was not read");

const problems = [];
const KITS = new Set(["drums", "sampler_kit", "analog_kit", "drum_kit"]);

/** The keys to try a part on: a kit's kick and snare, a pitched part's
 * middle, high and low C. */
const keysFor = (kind) => (KITS.has(kind) ? [36, 38] : [60, 72, 48, 36]);

async function play(name, kind, { preset, knobs = {}, sfz } = {}) {
  await engine.createPart(0, kind, "none");
  if (preset !== undefined) engine.setPreset(0, kind, preset);
  const ids = new Set((await engine.params(0)).map((p) => p.id));
  for (const [id, value] of Object.entries(knobs)) {
    if (!ids.has(id)) problems.push(`${name}: the ${kind} part has no ${id}`);
    engine.setParam(0, id, value);
  }
  let keys = keysFor(kind);
  if (sfz) {
    const summary = await engine.loadSfz(0, `samples/${name}/voice.sfz`, { base: pathToFileURL(`${pub}/`).href });
    assert.ok(summary.regions > 0, `${name}: no regions`);
    if (summary.warnings.length > 0) problems.push(`${name}: ${summary.warnings.join("; ")}`);
    const text = readFileSync(sfz, "utf8");
    if (!KITS.has(kind)) {
      const centres = [...text.matchAll(/pitch_keycenter=(\d+)/g)].map((m) => Number(m[1]));
      keys = [centres[Math.floor(centres.length / 2)]];
    }
  }
  let heard = false;
  for (const key of keys) {
    engine.noteOn(0, key, 0.8);
    heard = await sounds(0);
    engine.noteOff(0, key);
    if (heard) break;
  }
  if (!heard) problems.push(`${name}: silent on keys ${keys.join(", ")}`);
  await remove();
}

/** Removes part 0 once the engine has applied what was sent to it. */
async function remove() {
  await engine.sync();
  render(1);
  await engine.removePart(0);
}

for (const [code, voice] of Object.entries(voices)) {
  if (voice.kind === "sampler" || voice.kind === "sampler_kit") continue;
  await play(code, voice.kind, voice);
}

const samples = join(pub, "samples");
let sampled = 0;
for (const code of readdirSync(samples)) {
  const sfz = join(samples, code, "voice.sfz");
  if (!existsSync(sfz)) continue;
  const kind = voices[code]?.kind ?? "sampler";
  await play(code, kind, { sfz });
  sampled += 1;
}
assert.ok(sampled > 10, "the sampled voices were not found");

// The drum machine, which also plays the percussion beside other parts.
for (const [drum, key] of Object.entries({ ...drums, ...percussion })) {
  await engine.createPart(0, "drum_kit", "none");
  engine.setPreset(0, "drum_kit", "acoustic");
  engine.noteOn(0, key, 0.8);
  if (!(await sounds(0))) problems.push(`drum_kit: ${drum} (key ${key}) is silent`);
  await remove();
}

// Every parameter the language knows that goes to the engine, under the
// names and units the editor sends it with (mbParam in src/audio.ts).
const language = readFileSync(join(web, "..", "crates", "aratu", "src", "params.rs"), "utf8");
const known = [...language.matchAll(/\bp\(\s*"([a-z0-9_.]+)",\s*(-?[\d.]+),\s*(-?[\d.]+),/g)].map(
  ([, id, min, max]) => ({ id, min: Number(min), max: Number(max) }),
);
assert.ok(known.length > 100, "the parameter table was not read");

const part = new Map();
for (const kind of Object.keys(INSTRUMENT)) {
  await engine.createPart(0, kind, "all");
  for (const p of await engine.params(0)) part.set(p.id, p);
  await engine.removePart(0);
}
const master = new Map((await engine.masterParams()).map((p) => [p.id, p]));

for (const { id, min, max } of known) {
  // The volume is the part's gain in decibels, the reverb its send, the
  // stand-in's envelope the synth's.
  if (id === "volume" || id === "reverb.mix" || id === "attack" || id === "release") continue;
  const own = id.replace(/^tube\.drive$/, "drive.amount").replace(/^tube\./, "drive.");
  const found = id.startsWith("master.") ? master.get(own) : part.get(own);
  if (!found) {
    problems.push(`${id}: the engine has no ${own}`);
    continue;
  }
  const slack = (x) => Math.abs(x) * 1e-6 + 1e-6;
  if (min < found.min - slack(found.min) || max > found.max + slack(found.max)) {
    problems.push(`${id}: knob ${min} to ${max}, engine ${found.min} to ${found.max}`);
  }
}
for (const id of ["gain", "pan"]) assert.ok(part.has(id), `the engine has no ${id}`);

const stats = await engine.stats();
for (const counter of ["ignored", "dropped", "unrouted"]) {
  if (stats[counter] !== 0) problems.push(`the engine counted ${stats[counter]} ${counter} events`);
}

await engine.close();
processor.port.close();

assert.deepEqual(problems, [], `the engine and the editor disagree:\n${problems.join("\n")}`);

const version = readFileSync(join(vendored, "VERSION"), "utf8").match(/^Manguebyte ([0-9a-f]{40})$/m)?.[1];
const script = readFileSync(join(web, "scripts", "manguebyte.mjs"), "utf8");
const pinned = script.match(/process\.argv\[2\] \?\? "([0-9a-f]{40})"/)?.[1];
assert.ok(version && pinned, "no commit in VERSION or in scripts/manguebyte.mjs");
assert.equal(version, pinned, "public/manguebyte/ is not the build of the commit scripts/manguebyte.mjs pins");

console.log(
  `engine ${version.slice(0, 7)}: ${Object.keys(voices).length} voices, ${sampled} sampled, ` +
    `${Object.keys({ ...drums, ...percussion }).length} drum keys, ${known.length} parameters`,
);
