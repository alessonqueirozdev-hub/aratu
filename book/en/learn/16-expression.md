# Expression: swing, rubato and humanize

A computer plays every note in its exact place. A musician does not: they
swing the eighths, breathe in a phrase, never hit twice the same way. Three
words give that life to what you write, without changing a single note.

## Swing

In jazz, samba-jazz and the blues, the eighths are not even: the first is
longer and the second arrives a little late. That is **swing**. Write how
much:

```aratu
swing 62%
tempo: 110
hh = cc cc cc cc
bd = b - b -
sd = - s - s
```

The percentage says where the second eighth falls inside the beat. 50% is
straight, both eighths even. 66% is the triplet feel, `c_c`, the heaviest
swing. Anywhere in between is the groove you want.

> Drag the knob on the `62%` while the music plays, from 50 to 70, and hear
> the hi-hat start straight and begin to swing.

Swing can apply to one part only, too: `bass: swing 60%`.

## Rubato

**Rubato** means stolen time. The musician stretches one beat and shrinks
another, but arrives at the bar line in the right place: the stolen time is
given back. It is how a pianist plays a slow melody.

```aratu
tempo: 70
melody = | e4 d c d | e e e _ |
bd = b - - -
melody: rubato 20%
```

Listen: the melody breathes, but the bass drum still hits at the start of
each bar, together with it. `all: rubato 10%` bends the whole piece
together, like a conductor.

## Humanize

`humanize` adds small deviations, in timing and in strength, like those of
a real musician. Each hit comes out a little different:

```aratu
humanize
tempo: 100
hh = cc cc cc cc
bd = b - b -
```

`humanize 5 ms` sets the size of the deviation. The deviations are drawn
the way chance is: with the same seed, the same piece always plays the same
way.

## Exercises

These exercises also check **when** each note arrives, not only which note
it is.

Put a 66% swing on this groove.

```aratu exercise timing=1
hh = cc cc cc cc
bd = b - b -
```

```aratu answer
swing 66%
hh = cc cc cc cc
bd = b - b -
```

The swing is too heavy: make it 58%.

```aratu exercise timing=1
swing 66%
hh = cc cc cc cc
```

```aratu answer
swing 58%
hh = cc cc cc cc
```

Make the melody breathe with 20% rubato, while the bass drum keeps the
pulse.

```aratu exercise timing=1
melody = | e4 d c d |
bd = b - - -
```

```aratu answer
melody = | e4 d c d |
bd = b - - -
melody: rubato 20%
```
