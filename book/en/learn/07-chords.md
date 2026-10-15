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
