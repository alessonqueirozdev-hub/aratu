// Loads the built WebAssembly module in Node and checks the editor's
// protocol end to end: compile, diagnostics in both languages, render.
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import assert from "node:assert/strict";

const web = join(dirname(fileURLToPath(import.meta.url)), "..");
const bytes = readFileSync(join(web, "public", "aratu.wasm"));
const { instance } = await WebAssembly.instantiate(bytes, {});
const x = instance.exports;
const enc = new TextEncoder();
const dec = new TextDecoder();

function call(text, f) {
  const b = enc.encode(text);
  const p = x.aratu_alloc(b.length);
  new Uint8Array(x.memory.buffer, p, b.length).set(b);
  const r = f(p, b.length);
  x.aratu_free(p, b.length);
  return r;
}
const out = () => JSON.parse(dec.decode(new Uint8Array(x.memory.buffer, x.aratu_out_ptr(), x.aratu_out_len())));

const src = "tempo: 120\nbass = | dó3 _ _ _ |\nhh = c c c c";
const h = call(src, (p, n) => x.aratu_compile(p, n, 0));
const program = out();
assert.equal(program.ok, true);
assert.deepEqual(program.parts.map((p) => p.name), ["bass", "drums"]);

call("", (p, n) => x.aratu_render(h, 1, 3, 1, 0, p, n));
const r = out();
assert.equal(r.bars.length, 2);
assert.equal(r.events.filter((e) => e.drum === "closed-hat").length, 8);
const bass = r.events.filter((e) => e.note === 48);
assert.equal(bass.length, 2);
assert.equal(bass[1].time, 2);
assert.equal(src.slice(bass[0].from, bass[0].to), "dó3");

call("bass = | c2 _ _ _ _ |", (p, n) => x.aratu_compile(p, n, 0));
assert.match(out().diagnostics[0].message, /tem 5 tempos, mas a fórmula de compasso é 4\/4/);
call("bass = | c2 _ _ _ _ |", (p, n) => x.aratu_compile(p, n, 1));
assert.match(out().diagnostics[0].message, /has 5 beats, but the meter is 4\/4/);

// Many compiles and renders do not grow memory without bound.
const before = x.memory.buffer.byteLength;
for (let i = 0; i < 300; i++) {
  const hh = call(src, (p, n) => x.aratu_compile(p, n, 0));
  call("", (p, n) => x.aratu_render(hh, i + 1, i + 2, 1, 0, p, n));
  x.aratu_release(hh);
}
assert.ok(x.memory.buffer.byteLength - before < 4 << 20, "memory keeps growing");
console.log("smoke ok");
