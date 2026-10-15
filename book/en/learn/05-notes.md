# Notes and melodies

So far there were only drums. Now, notes. A note is written with its name
and its octave: `c4` is middle C on the piano. Names are letters, as in
chord symbols, and the solfège names work too:

| Letter | c | d | e | f | g | a | b |
|---|---|---|---|---|---|---|---|
| Solfège | do | re | mi | fa | sol | la | si |

Listen to the C major scale going up:

```aratu
tempo: 100
melody = | c4 d4 e4 f4 | g4 a4 b4 c5 |
```

A line that is not a drum line plays with a synthesizer sound. Give it any
name you like: `melody`, `flute`, `voice`.

## The octave carries over until it changes

There is no need to repeat the octave on every note: it holds until another
one appears. The scale above can be written like this:

```aratu
melody = | c4 d e f | g a b c5 |
```

## Sharps and flats

A sharp is `#` and a flat is `b`, right after the letter: `f#4`, `bb3`,
`eb4`.

```aratu
melody = | a3 b c#4 d | e f# g# a |
```

That is the A major scale.

## A tune you know

"Twinkle, Twinkle, Little Star" is an old tune in the public domain.
Listen to its first phrase:

```aratu
tempo: 100
melody = | c4 c g g | a a g _ |
```

The `_` at the end holds the G for one more beat: a half note.

## Exercises

Finish the phrase: after the two bars come `f f e e | d d c _`.

```aratu exercise
tempo: 100
melody = | c4 c g g | a a g _ |
```

```aratu answer
tempo: 100
melody = | c4 c g g | a a g _ | f f e e | d d c _ |
```

Write the C major scale **going down**, from C5 to C4, in two bars.

```aratu exercise
melody = | c5 _ _ _ | c4 _ _ _ |
```

```aratu answer
melody = | c5 b4 a g | f e d c |
```
