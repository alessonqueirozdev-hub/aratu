# Numbers that move

A DJ turns the filter knob slowly and the whole dance floor feels the music
opening up. A guitarist rocks the pedal and the sound sweeps back and
forth. In Aratu a number can move on its own too, from one value to
another, while the music plays.

## From one value to another: `->`

Write the starting value, an arrow, the value to reach, and how long it
takes, with `over`:

```aratu
tempo: 100
bass: synth.saw -> lowpass
bass = | c2 c3 c2 c3 | eb2 eb3 eb2 eb3 |
bass.lowpass.cutoff = 300 Hz -> 4000 Hz over 4 bars
```

The filter opens over four bars and stays open. Both numbers get a knob,
and while it plays a second needle shows where the number is now.

The cutoff moves on the ear's scale: halfway between 300 and 4000 Hz it is
at about 1100 Hz, not 2150, and that is why the sweep sounds even from
start to end.

## There and back: `<->`

With `<->` in place of the arrow, the number goes there and back without
stopping: it opens over four bars, closes over four, opens again.

```aratu
tempo: 100
bass: synth.saw -> lowpass
bass = | c2 c3 c2 c3 | eb2 eb3 eb2 eb3 |
bass.lowpass.cutoff = 300 Hz <-> 4000 Hz over 4 bars
```

## A different length on the way back, and starting over

| Write | What it does |
|---|---|
| `A -> B over 4 bars` | goes from A to B over 4 bars and stays at B |
| `A <-> B over 4 bars` | there and back, 4 bars each way, again and again |
| `A -> B over 4 bars, back over 2 bars` | up over 4, back over 2, once |
| `A <-> B over 4 bars, back over 1 bar` | up over 4, back over 1, again and again |
| `A -> B over 1 bar, loop` | up, then from A again, like a ramp |

A short `loop` makes rhythmic effects. Here the pad's volume rises on each
beat and starts again, like a compressor "breathing":

```aratu
tempo: 120
pad: synth.pad
pad = | c4+eb4+g4 _ _ _ |
drums { bd = b b b b }
pad.volume = 0.1 -> 0.8 over 1 beat, loop
```

The length can be in bars (`bars`), beats (`beats`), eighth notes
(`eighths`) or sixteenths (`sixteenths`).

## When it starts

- Written on its own, the line starts at the start of the piece, or at the
  next bar if you write it while the music plays. Turning one of the knobs
  changes the way without starting it over.
- With `at bar 9:` in front, it starts at bar 9.
- With `every 8 bars:`, it ends at the end of every eight bars, like a fill
  leading back in.
- With `every 8 bars, first:`, it starts at the first of every eight.

```aratu
tempo: 120
bass: synth.saw -> lowpass
bass = | c2 c3 c2 c3 |
drums { hh = cc cc cc cc; bd = b b b b }
bass.lowpass.cutoff = 400 Hz
every 4 bars: bass.lowpass.cutoff = 400 Hz -> 6000 Hz over 2 bars
```

## Exercises

Make the filter open from 300 Hz to 4000 Hz over 4 bars.

```aratu exercise sound=1 bars=4
bass: synth.saw -> lowpass
bass = | c2 c3 c2 c3 |
bass.lowpass.cutoff = 300 Hz
```

```aratu answer
bass: synth.saw -> lowpass
bass = | c2 c3 c2 c3 |
bass.lowpass.cutoff = 300 Hz -> 4000 Hz over 4 bars
```

Now the filter goes there and back without stopping, two bars each way.

```aratu exercise sound=1 bars=8
bass: synth.saw -> lowpass
bass = | c2 c3 c2 c3 |
bass.lowpass.cutoff = 300 Hz -> 4000 Hz over 2 bars
```

```aratu answer
bass: synth.saw -> lowpass
bass = | c2 c3 c2 c3 |
bass.lowpass.cutoff = 300 Hz <-> 4000 Hz over 2 bars
```

At bar 3, the keys' reverb jumps to 0.6, all at once.

```aratu exercise sound=1 bars=4
keys: keys.epiano -> reverb
keys = | c4+e4+g4 _ _ _ |
```

```aratu answer
keys: keys.epiano -> reverb
keys = | c4+e4+g4 _ _ _ |
at bar 3: keys.reverb.mix = 0.6
```
