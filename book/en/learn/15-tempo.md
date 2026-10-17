# Tempo: rit, accel, fermata and pickup

## Tempo changes

The tempo can change at a bar marked with `at bar`:

```aratu
tempo: 100
at bar 3: rit to 60 over 2 bars
hh = c c c c
```

`rit to 60 over 2 bars` is a ritardando to 60 over two bars (`rall` works
too). The opposite is `accel to 140 over 4 bars`. `a tempo` goes back to
the tempo before the rit, and `tempo primo` back to the piece's first
tempo.

During a ritardando the tempo changes evenly, beat by beat, as a score
reads, and every note lands at its exact moment.

A new tempo all at once, at one bar: `at bar 9: tempo 130`. And a ramp
between any two tempos, right from the start:

```aratu
tempo 90 -> 140 over 8 bars
hh = cc cc cc cc
bd = b - b -
```

The length can be in beats too: `rit to 60 over 6 beats`.

## Fermata

`^` after a note is a fermata: the note lasts twice as long, and every
part waits for it. With a number, it lasts that many times as long:
`g4^3` lasts three times as long.

```aratu
tempo: 90
melody = | c4 e g c5^ | g4 _ _ _ |
hh = c c c c
```

## Pickup

Many tunes start before the first beat. `pickup: beat 4` says the piece
starts on beat 4 of an incomplete bar:

```aratu
tempo: 100
pickup: beat 4
melody = | g3 | c4 _ e _ | g _ _ _ |
bd = | b - b - |
```

The pickup is bar 0, so bar 1 is the first full bar, as musicians number
them. A line whose first bar is as long as the pickup starts in it; the
others start at bar 1.

## Exercises

The exercises in this lesson also check **when** each note arrives.

Put a fermata on the last note of the first bar.

```aratu exercise timing=1
melody = | c4 d e f | g _ _ _ |
```

```aratu answer
melody = | c4 d e f^ | g _ _ _ |
```

Make this tune start with the pickup `g3` on beat 4.

```aratu exercise
melody = | c4 _ e _ | g _ _ _ |
```

```aratu answer
pickup: beat 4
melody = | g3 | c4 _ e _ | g _ _ _ |
```

Make a ritardando to 60, over two bars, starting at bar 2.

```aratu exercise timing=1 bars=4
tempo: 100
hh = c c c c
```

```aratu answer
tempo: 100
hh = c c c c
at bar 2: rit to 60 over 2 bars
```

Now a fermata three times as long on the final G.

```aratu exercise timing=1
melody = | c4 e g4^ - |
```

```aratu answer
melody = | c4 e g4^3 - |
```
