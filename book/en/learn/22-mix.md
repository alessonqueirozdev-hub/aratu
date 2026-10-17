# Mixing the whole piece

With several parts playing together, there is still the job of the person
at the mixing desk: putting each instrument at the right volume, each in
its place between the speakers, all of them in the same room. That is
**mixing**.

## Volume: who stands out

Each part has a `volume`, from 0 (silent) to 1. Drums that are too loud
hide the melody; turn them down:

```aratu
tempo: 100
drums { hh = cc cc cc cc; bd = b - b -; sd = - s - s }
melody: keys.epiano
melody = | e4 g a g | e d c _ |
drums.volume = 0.5
melody.volume = 0.9
```

> Drag both knobs while the music plays, as if they were the faders on a
> mixing desk.

## Pan: where each one sits

`pan` places the part between the speakers: -1 is all the way left, 1 all
the way right, 0 in the middle. Spreading the instruments to the sides
makes each one clearer:

```aratu
tempo: 100
guitar: guitar.nylon
guitar { chords = | Am | F | C | G |; strum = D du -u du }
keys: keys.epiano
keys = | a4 _ c5 _ | a4 _ f4 _ | g4 _ e4 _ | d4 _ g4 _ |
guitar.pan = -0.6
keys.pan = 0.6
```

Listen with headphones: the guitar on the left, the keys on the right. The
bass and the bass drum usually stay in the middle.

## The same room for everyone

Aratu's mix has a shared reverb, like the room the band plays in. Each
part's `reverb.mix` says how much of it goes into that room.

A number written **with no part** holds for every part. To put the whole
piece in the room, one line is enough:

```aratu
tempo: 90
melody: piano.grand
melody = | e4 g a g | e d c _ |
bass: bass.finger
bass = | c2 _ g1 _ | a1 _ f1 _ |
reverb.mix = 0.5
```

A part can have its own value, which holds in place of the general one:
`bass.reverb.mix = 0.1` makes the bass drier, closer to the listener.

## Effects for everyone: `all:`

A chain written for `all` puts its effects on every part, after each part's
own effects:

```aratu
tempo: 90
melody: keys.epiano -> tremolo
melody = | e4 g a g | e d c _ |
bass: bass.finger
bass = | c2 _ g1 _ | a1 _ f1 _ |
all: chorus -> delay
delay.mix = 0.2
```

The melody goes through tremolo, chorus and delay; the bass through chorus
and delay. And `delay.mix = 0.2`, with no part, sets the delay of them all.

## The master

Everything ends in one last channel, the **master**: the shared room, a
compressor that "glues" the parts together and a limiter that keeps the
sound from clipping. Its numbers start with `master.`:

```aratu
tempo: 90
melody: piano.grand
melody = | e4 g a g | e d c _ |
reverb.mix = 0.5
master.reverb.decay = 4 s
master.glue.on = 1
master.limiter.ceiling = -1
```

| Number | What it is |
|---|---|
| `master.reverb.decay` | how long the room rings after the note, in seconds |
| `master.reverb.size` | the size of the room, from 0 to 1 |
| `master.glue.on` | 1 turns on the glue compressor |
| `master.gain` | the volume of everything, in dB |
| `master.limiter.ceiling` | the highest volume, from -12 to -0.3 dB |

## Making room for the kick

In electronic music the bass drum leads, and the other parts make room for
it: at each kick the pad's volume drops at once and comes back before the
next one, and the whole track seems to breathe. In the studio that is
called **ducking**, or *sidechain*. In Aratu it is one line:

```aratu
tempo: 124
drums { bd = b b b b; hh = -c -c -c -c }
pad: synth.pad
pad = | a3+c4+e4 _ _ _ | f3+a3+c4 _ _ _ |
pad: duck by kick
```

`pad: duck by kick` lowers the pad by 70% at each kick and brings it back
over one beat. Say how much and how fast: `pad: duck by kick 50% over 1
eighth`. After `by` comes the part or the line to make room for (`drums`,
`bd`), or a drum by its name, `kick`, `snare` or `clap`, whichever line
plays it.

> Delete the line and listen: the pad stands still, on top of the kick. Put
> it back and the music pumps again.

In Portuguese it is `pad: abaixa com o bumbo`, in Spanish
`pad: agacha con el bombo`, and the studio's word, `sidechain`, works in
all three.

## Exercises

Turn the drums down to 0.4, so the melody stands out.

```aratu exercise sound=1
drums { hh = cc cc cc cc; bd = b - b - }
melody = | e4 g a g |
```

```aratu answer
drums { hh = cc cc cc cc; bd = b - b - }
melody = | e4 g a g |
drums.volume = 0.4
```

Put the guitar on the left (-0.5) and the keys on the right (0.5).

```aratu exercise sound=1
guitar = | e3 _ g3 _ |
keys = | c5 _ e5 _ |
```

```aratu answer
guitar = | e3 _ g3 _ |
keys = | c5 _ e5 _ |
guitar.pan = -0.5
keys.pan = 0.5
```

Put the whole piece in the reverb, at 0.6, with a single line.

```aratu exercise sound=1
melody = | e4 g a g |
bass = | c2 _ g1 _ |
```

```aratu answer
melody = | e4 g a g |
bass = | c2 _ g1 _ |
reverb.mix = 0.6
```

Put a chorus on every part, with a single line.

```aratu exercise sound=1
melody: keys.epiano
melody = | e4 g a g |
bass: bass.finger
bass = | c2 _ g1 _ |
```

```aratu answer
melody: keys.epiano
melody = | e4 g a g |
bass: bass.finger
bass = | c2 _ g1 _ |
all: chorus
```

Make the master's room ring for 5 seconds.

```aratu exercise sound=1
melody = | e4 g a g |
reverb.mix = 0.5
```

```aratu answer
melody = | e4 g a g |
reverb.mix = 0.5
master.reverb.decay = 5 s
```

Make the pad duck by half (50%) at each kick.

```aratu exercise sound=1
drums { bd = b b b b }
pad = | c4+e4+g4 _ _ _ |
```

```aratu answer
drums { bd = b b b b }
pad = | c4+e4+g4 _ _ _ |
pad: duck by kick 50%
```
