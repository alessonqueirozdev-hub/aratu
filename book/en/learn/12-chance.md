# Chance

A loop that repeats exactly the same every time soon sounds mechanical. A
drummer never plays two bars quite alike: a hi-hat note drops out, another
appears. Chance gives a loop that life without you touching it: the
computer decides, a little at random, what to play each time round. It is
the *ad lib* of a score.

## Sometimes

A `?` after a symbol plays it only some of the time, half of the time:

```aratu
tempo: 100
hh = cc cc c? cc
bd = b - b -
```

A percentage says how often:

```aratu
tempo: 100
hh = cc cc c?30% cc
sd = - s - s?80%
```

## One or the other

Braces choose one of several options, separated by `|`, each time the loop
comes round:

```aratu
tempo: 100
bass = | {c2|g1} _ c2 _ |
hh = {cc|co} cc cc cc
```

The bass starts on C or on G; the hi-hat sometimes opens on the first
beat.

## The same chance again

The choices are drawn again every time round, but from a fixed seed: the
same piece improvises the same way each time it is played from the start.
`seed 7` picks another improvisation; `seed random` makes every
performance different.

```aratu
seed 7
hh = c? c? c? c?
```

> Changing one line does not change how the others improvise: each line
> draws on its own.

This lesson has no exercise to check, because chance has no right answer.
Play with the examples as much as you like.
