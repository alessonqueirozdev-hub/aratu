# Sounds, effects and knobs

Each part has a sound and a chain of effects, written as the path of the
signal, from left to right, like a pedalboard:

```aratu
tempo: 100
bass: synth.saw -> lowpass -> reverb
bass = | c2 _ c2 eb2 | f2 _ g2 bb1 |
```

The sound is the code at the head of the chain. There are dozens: open the
**Sounds** panel, beside the messages in Play, to hear each one and put its
code in the text with a click. Change `synth.saw` to `bass.slap`,
`bass.808` or `piano.grand` and listen:

```aratu
tempo: 100
bass: bass.slap
bass = | e2 E3 e2 E3 | a2 A3 a2 A3 |
```

> In `bass.slap`, an upper-case note is a *pop*: the string pulled so it
> snaps.

## Numbers become knobs

Parameters are set with numbers, and each number gets a round knob beside
it. Drag the knob up or down: the number in the text changes and the sound
changes with it, at once.

```aratu
tempo: 100
bass: synth.saw -> lowpass -> reverb
bass = | c2 _ c2 eb2 | f2 _ g2 bb1 |
bass {
  lowpass.cutoff = 800 Hz
  lowpass.resonance = 0.3
  reverb.mix = 0.2
}
```

The `lowpass.cutoff` knob turns on a logarithmic scale in hertz, as the
ear hears; the `reverb.mix` knob goes from 0 to 1. Hold Shift for finer
steps.

| Parameter | Range |
|---|---|
| `volume` | 0 to 1 |
| `pan` | -1 (left) to 1 (right) |
| `lowpass.cutoff`, `highpass.cutoff` | 20 to 20000 Hz |
| `lowpass.resonance` | 0 to 1 |
| `reverb.mix` | 0 to 1 |
| `delay.time` | 10 to 2000 ms |
| `delay.feedback`, `delay.mix` | 0 to 0.95, 0 to 1 |

A parameter can also change at a bar: `at bar 9: bass.reverb.mix = 0.6`.

> These are simple sounds built into the browser. Manguebyte's engine will
> bring real instruments and effects, and the same texts will sound better
> without changing anything.
