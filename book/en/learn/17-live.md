# Playing live

In Aratu, playing live means changing the text while the music plays. A few
ideas make it easy.

- **Everything loops.** Each line plays over and over, each over its own
  length: a one-bar hi-hat, a two-bar bass and a four-bar progression line
  up as a band's parts do.
- **A change comes in at the next bar.** Nothing restarts: the other parts
  keep their place.
- **An error stops nothing.** While the text has an error, the last
  version that was right keeps playing.

## Patterns

A pattern is a named group of lines, prepared ahead. `play` chooses which
pattern sounds:

```aratu
tempo: 110
pattern groove {
  hh = cc cc cc cc
  bd = b - b -
  sd = - s - s
}
pattern fill {
  sd = ss ss SS SS
  bd = b - - -
}

play groove x3, fill
```

`play groove x3, fill` plays the groove three times, then the fill, and
loops the four-bar phrase.

## Varying from time to time

```aratu
tempo: 110
pattern groove { hh = cc cc cc cc; bd = b - b -; sd = - s - s }
pattern fill   { sd = ss ss SS SS }
pattern crash  { cr = x - - - }

play groove
every 4 bars: play fill
every 8 bars, first: add crash
```

`every 4 bars: play fill` swaps the groove for the fill in the last bar of
every four. `add` plays a pattern **on top** of the one sounding: here, a
crash on the first beat of every eight bars.

## Right now, at the next beat

A line written while the music plays comes in at the next bar. With
`now:` in front, it comes in at the very next beat:
`now: play fill once` brings the fill in at once, and the groove comes
back by itself, where it would have been. The next lesson shows how to
mark a change at an exact bar and beat.

## Taking a part out and back in

`drums: out` silences a part from the next bar line, and `drums: in`
brings it back, as on a mixer. `all: out` silences everything.

```aratu
tempo: 110
drums { hh = cc cc cc cc; bd = b - b -; sd = - s - s }
bass = | c2 _ g1 _ | a1 _ e1 _ |
at bar 3: drums: out
at bar 5: drums: in
```

## On stage

The **Stage** button, in Play, leaves only the code on the screen, full
screen and in large letters, to play in front of an audience. The
**2 columns** button splits the code in two columns, like a newspaper: the
second goes on where the first ends, so twice as much fits without
scrolling, and the two scroll together. You can edit in either. With
**Fit the screen**, the letters get smaller as the piece grows, so the whole
piece shows without scrolling. Melody lists get buttons in the corner, and
Esc leaves the stage.

## Sharing and recording

Like what you made? **Share** copies a link with your piece inside: send it
to someone, and whoever opens it hears it and can change the same piece.
**Record** records what is playing until you press it again; then the audio
file downloads, to keep or to show anyone.

## Exercises

Use the ready-made patterns to play the phrase: the groove three times,
then the fill. (`x3` can also be written `3x`.)

```aratu exercise bars=4
pattern groove { hh = cc cc cc cc; bd = b - b -; sd = - s - s }
pattern fill   { sd = ss ss SS SS; bd = b - - - }
play groove
```

```aratu answer
pattern groove { hh = cc cc cc cc; bd = b - b -; sd = - s - s }
pattern fill   { sd = ss ss SS SS; bd = b - - - }
play groove x3, fill
```

Take the drums out at bar 2 and back in at bar 3, while the bass keeps
playing.

```aratu exercise bars=4
drums { hh = c c c c }
bass = | c2 _ _ _ |
```

```aratu answer
drums { hh = c c c c }
bass = | c2 _ _ _ |
at bar 2: drums: out
at bar 3: drums: in
```
