# Welcome to Aratu

Aratu is a way to write music on the computer keyboard and hear it at once.
You write the way a musician thinks: in bars, beats, notes and chord
symbols. Press **Listen**, and the music plays. Change a letter, and the
music changes with it.

This book teaches Aratu from the start, one idea at a time. You do not need
to know how to program. Knowing a little music helps, but music is learnt
here too.

## How this book works

- Each lesson explains one idea with **examples** you can hear and change
  as much as you like. If you make a mess, press **Start over**.
- Under each example there is a **grid**: the bars, the numbered beats and
  each symbol where it sounds, the way a teacher would draw it on the
  board. You can work on it: drag a symbol to another place, click an empty
  place to add one and a symbol to take it away, and the text changes with
  it.
- The **exercises** ask you to write something. **Hear how it should
  sound** plays the goal; **Check** compares what you wrote with the goal
  by its sound and says, bar by bar, where they differ.
- When something is written wrong, Aratu explains it in a musician's
  words, never as a "syntax error".

Your progress is kept in this browser, and the list of lessons shows a ✓
beside each lesson you finish.

## The first sound

Press **Listen**:

```aratu
hh = c c c c
```

This is one bar of hi-hat with four hits. `hh` is the line's name, and each
`c` is a hit of the closed hi-hat. The bar repeats until you press
**Stop**. Everything in Aratu repeats like this, as a loop.

> Change one of the `c`s to `o` and listen again: `o` is the open hi-hat.
> You can change the text while the music plays; the change comes in at
> the start of the next bar.

## More than one line

Each drum has its own line. Listen to the three together:

```aratu
hh = c c c c
bd = b - b -
sd = - s - s
```

`bd` is the bass drum and `sd` the snare drum. A `-` is a rest: on that
beat, that drum does not play. Bass drum on 1 and 3, snare on 2 and 4:
the most common beat in rock and pop.

## Your first exercise

Write a snare line that plays on all **four** beats. Hear the goal first,
then change the rests and press **Check**.

```aratu exercise
sd = - - - -
```

```aratu answer
sd = s s s s
```

In the next lesson you will see what a bar is to Aratu, and how to set the
tempo.

> Do you program, and want to know how Aratu works underneath? The
> [**Inside**](#/por-dentro) tab gives each thing its technical name: what a
> line is, a block, an `at bar`.
