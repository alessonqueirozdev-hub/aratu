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

## Stage gestures

In electronic music, a performance moves by gestures: the drums drop out
(the *breakdown*), the tension rises (the *build*) and everything comes
back at once (the *drop*). In Aratu, each gesture is a word:

| Write | What happens |
|---|---|
| `breakdown` | the drums, the percussion and the bass go out; the rest goes on |
| `build over 4 bars` | a snare roll that doubles its speed as it grows louder, while a filter takes the lows out of everything |
| `drop` | everything comes back, with a crash on the first beat |
| `solo bass` | only the bass plays; `solo off` brings the rest back |

With `now:` in front, the gesture comes in at the next beat; with
`at bar`, at the bar named:

```aratu
tempo: 124
drums { bd = b b b b; oh = -o -o -o -o; clap = - p - p }
bass = | [- c2]x4 | [- ab1]x4 |
pad = | c4+eb4+g4 _ _ _ | c4+eb4+ab4 _ _ _ |
at bar 5: breakdown
at bar 7: build over 2 bars
at bar 9: drop
```

Listen: four bars with everything, the pad alone, the build and the way
back on the crash. Playing live, write `now: drop` when the floor asks.

## Ready-made grooves

To start fast, one word writes a whole beat: `groove house`. The grooves
are: house, techno, trap, dnb, breakbeat, hiphop, funk, rock, reggaeton,
funk carioca, bossa, samba, baião and maracatu.

```aratu
tempo: 124
groove house
bass = | [- a1]x4 | [- f1]x4 |
```

While it plays, the word `groove` lights up. And any line you write with
the same name takes the place of the groove's line: `hh = c c c c`
changes only the hi-hat, and the rest of the groove goes on.

## Shortcuts for playing live

| Shortcut | What it does |
|---|---|
| Ctrl+Enter | plays, or updates what is playing |
| Ctrl+. | stops |
| Ctrl+/ | comments the selected lines out: they stop sounding; again, they come back |
| Alt+M | mutes the part of the line the cursor is on (writes `part: out`); again, it comes back |
| Alt+S | solos the part of the cursor's line (writes `solo part`); again, it ends |
| Ctrl+Shift+1 to 8 | brings back a kept scene |

The shortcuts write what they do in the text, so the change comes in at
the next bar line like any other, and stays if you share the piece.

## Faster, slower, and an echo

Three short verbs change a line while it plays, without writing it again.
Write the line's name, a dot and the verb:

```aratu
tempo: 120
drums {
  hh = | cc cc cc co | cc cc c- c- |
  bd = b - b -
  sd = - s - s
}
hh.fast 2
```

`hh.fast 2` plays two bars of the hi-hat in each bar: twice as fast.
`slow 2` stretches each bar over two. The numbers are whole numbers, so the
line always lands on a bar line; `fast 2` and `slow 2` together cancel out.
They work on a line or on a whole part (`drums.fast 2`). Delete the line and
the hi-hat goes back to its own speed.

`echo` plays every sound again a little later, and softer, like a voice
coming back from a wall:

```aratu
tempo: 100
lead: synth.pluck
lead = | e5 - g - | a - g - |
lead.echo 3 sixteenths x2
```

`echo 3 sixteenths` repeats each note three sixteenths later, and `x2` makes
two echoes, each softer than the one before. The length is written as after
`over`: `1 beat`, `1 eighth`, `3 sixteenths`. In Portuguese the verbs are
`rapido`, `lento` and `eco`; in Spanish `rapido`, `lento` and `eco` too.

## Scenes

Built a good moment of the piece? Keep it: in Play and on stage there are
eight **scenes**, numbered 1 to 8. Click an empty scene and the editor's
piece is kept in it; click a kept scene, or press Ctrl+Shift and the
number, and it comes back into the editor. While the music plays, the
change comes in at the next bar line, like any change, and Ctrl+Z undoes
it. Shift+click keeps the current piece over a scene.

So you prepare the intro, the groove, the breakdown and the drop as
scenes, and on stage go from one to the next with one key. The scenes are
kept in this browser.

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

Make a breakdown at bar 3 and the drop at bar 5.

```aratu exercise bars=6
drums { bd = b b b b }
bass = | c2 _ _ _ |
pad = | c4+eb4+g4 _ _ _ |
```

```aratu answer
drums { bd = b b b b }
bass = | c2 _ _ _ |
pad = | c4+eb4+g4 _ _ _ |
at bar 3: breakdown
at bar 5: drop
```

Start with the house groove and change the hi-hat to closed eighths.

```aratu exercise
drums { bd = b b b b }
```

```aratu answer
groove house
hh = cc cc cc cc
```

A build of two bars from bar 3.

```aratu exercise bars=5
drums { bd = b b b b }
```

```aratu answer
drums { bd = b b b b }
at bar 3: build over 2 bars
```

With one more line, play the hi-hat twice as fast.

```aratu exercise
hh = cc cc cc co
```

```aratu answer
hh = cc cc cc co
hh.fast 2
```

Give the melody an echo one beat after each note.

```aratu exercise
melody = | c5 - - - | g4 - - - |
```

```aratu answer
melody = | c5 - - - | g4 - - - |
melody.echo 1 beat
```
