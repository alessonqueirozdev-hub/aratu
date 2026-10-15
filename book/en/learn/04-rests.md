# Rests and long rests

Silence is music too. In Aratu every rest is written with the same sign,
`-`, and its length comes from the same rules as sounds: a `-` takes one
part of the beat, as a note would.

| Rest | Written in 4/4 |
|---|---|
| quarter | `-` alone in a beat |
| eighth | `-c` (rest, then an eighth) or `c-` |
| sixteenth | `-ccc`, `c-cc` |
| dotted eighth | `-__c` or `-.c` |
| half | `- -` |

Listen to the eighth rest at the start of each beat, the off-beat:

```aratu
tempo: 100
hh = -c -c -c -c
bd = b b b b
```

A `_` after a rest lengthens the silence: `- _` is also a half rest.

## A whole bar of rest

A `-` alone between bar lines is a whole bar of rest in any meter, like the
score's whole rest. And `- x4` is four bars of rest: the `x` means
"times".

```aratu
hh = | c c c c |
bd = | - x2 | b - b - |
```

The hi-hat plays from the start; the bass drum waits two bars and comes in
on the third. Since the bass drum line is three bars long, it repeats
every three.

> Long rests let an instrument come in later, as in a band where the drums
> start alone.

## Exercises

Hi-hat only on the off-beats: an eighth rest and an eighth on every beat.

```aratu exercise
hh = cc cc cc cc
```

```aratu answer
hh = -c -c -c -c
```

The bass drum waits **two bars** in silence, then plays on beats 1 and 3.

```aratu exercise
bd = | b - b - |
```

```aratu answer
bd = | - x2 | b - b - |
```
