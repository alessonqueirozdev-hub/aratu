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

## Parameters that move

A parameter can go from one value to another little by little, as in a
crescendo. `->` goes once and stays there; `over` says how long it takes:

```aratu
tempo: 100
bass: synth.saw -> lowpass
bass = | c2 c3 c2 c3 | eb2 eb3 eb2 eb3 |
bass.lowpass.cutoff = 300 Hz -> 4000 Hz over 4 bars
```

With `<->` in place of the arrow, it goes there and back without stopping:
up over four bars, down over four, up again.

```aratu
tempo: 100
bass: synth.saw -> lowpass
bass = | c2 c3 c2 c3 | eb2 eb3 eb2 eb3 |
bass.lowpass.cutoff = 300 Hz <-> 4000 Hz over 4 bars
```

| Write | What it does |
|---|---|
| `A -> B over 4 bars` | goes from A to B over 4 bars and stays at B |
| `A <-> B over 4 bars` | there and back, 4 bars each way, again and again |
| `A -> B over 4 bars, back over 2 bars` | up over 4, back over 2, once |
| `A <-> B over 4 bars, back over 1 bar` | up over 4, back over 1, again and again |
| `A -> B over 1 bar, loop` | up, then from A again, like a ramp |

The length can be in bars, `beats`, `eighths` or `sixteenths`:
`over 2 beats`, `over 3 eighths`.

The cutoff moves on the same scale as its knob, the ear's: halfway between
300 and 4000 Hz it is at about 1100 Hz, not 2150. Both numbers get a knob,
and while it plays a second needle shows where the parameter is now.

> Written while the music plays, the line starts moving at the next bar, and
> turning one of the knobs changes the way without starting it over. With
> `at bar 9:` in front, it starts at bar 9; with `every 8 bars:`, it ends
> at the end of every eight bars, like a fill; with `every 8 bars, first:`,
> it starts at the first of every eight.

> The recorded sounds and the analog drum kit already play on Manguebyte's
> engine. The effects will move to it too, and the same texts will sound
> better without changing anything.
