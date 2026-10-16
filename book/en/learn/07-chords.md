# Chords and strumming

A progression is written as a lead sheet, on a line called `chords`. Each
bar between bar lines has its chord:

```aratu
tempo: 90
chords = | Am | F | C | G |
```

On its own, the chords line plays each chord whole, like a keyboard.

## Chords sharing a bar

Two chords in one bar split it in half. For other splits, use `_` to say
how long each chord lasts:

```aratu
chords = | C G | Am _ _ F | C |
```

In the second bar, Am lasts three beats and F one.

Chord symbols keep their usual spelling: `Am`, `G7`, `Cmaj7`, `Dm7`,
`Bdim`, `E/G#`, `G7(b9)`.

## The strum

A line called `strum` says how the hand hits the strings, with the same
rhythm rules:

| Symbol | Strum |
|---|---|
| `d` | down |
| `u` | up |
| `m` | muted |
| `g` | golpe: the fingers tap the guitar's top, not the strings |
| `t` | down strum and golpe together |
| `-` | the hand passes without striking |
| upper case | accented |

Put both lines in a block named after the instrument:

```aratu
tempo: 90
guitar {
  chords = | Am | F | C | G |
  strum  = D du -u du
}
```

The strum repeats every bar, and the chords change on their own. A down
strum sounds the strings from low to high, an up strum from high to low,
as on a real guitar.

> Change the strum to `D -u D -u` and hear the difference while it plays.

## The golpe on the top

In flamenco the hand also taps the guitar's top: the *golpe*. It is
written `g`, and `G` is an accented golpe. The strings do not sound on a
golpe; the wood does:

```aratu
tempo: 100
guitar {
  chords = | Am | G | F | E |
  strum  = dgdu dgdu dGdu dgdu
}
```

> Each beat has four sixteenths: down, golpe, down, up. The golpe on the
> third beat is accented.

Often the golpe falls **together** with a down strum: the hand sweeps down
the strings and the ring finger taps the top at the same moment. That is
written `t`, for *together*, and `T` is the same with an accent:

```aratu
tempo: 100
guitar {
  chords = | Am | G | F | E |
  strum  = T-du g-du T-du g-du
}
```

## Exercises

Write the progression C, Am, F, G, one chord per bar.

```aratu exercise
guitar {
  chords = | C | C | C | C |
  strum  = d d d d
}
```

```aratu answer
guitar {
  chords = | C | Am | F | G |
  strum  = d d d d
}
```

Now change the strum to `D du D du`.

```aratu exercise
guitar {
  chords = | Am | Dm | E7 | Am |
  strum  = d d d d
}
```

```aratu answer
guitar {
  chords = | Am | Dm | E7 | Am |
  strum  = D du D du
}
```
