# Dynamics: loud and soft

Dynamic marks are the score's: `ppp pp p mp mf f ff fff`, and also `sfz`
and `fp`. Inside a bar, a mark sits on its note with a colon, and holds
from there on:

```aratu
tempo: 90
melody = | p:c4 e g c5 | f:g4 _ _ _ |
```

The first bar is piano; the second, forte. The mark does not count as a
beat, and it may have a space after the colon: `p: c4` works too. In
`f: f4` the first `f` is forte and the second is the note F.

## Crescendo and diminuendo

`<` after a note makes a crescendo and `>` a diminuendo, from the next
beat to the next mark:

```aratu
tempo: 90
melody = | p:c4< d e f | f:g4> f e d | p:c4 _ _ _ |
```

With no mark after it, a hairpin moves one step (from mf to f, say) by the
end of the line.

## Articulation: accent, marcato, staccato and ghost note

Articulation says **how** each note is attacked. There are four signs,
from the strongest to the weakest:

| Sign | Written | How it sounds |
|---|---|---|
| marcato | `S!`, `c4!` | the strongest, very marked |
| accent | `S`, `C4` | louder than its neighbours |
| staccato | `s'`, `c4'` | short, detached |
| ghost note | `(s)`, `(c4)` | almost silent, felt more than heard |

An accent adds to the dynamic the music is at, and a marcato adds even
more. Ghost notes are the spice of funk and samba on the snare: very light
strokes between the strong hits.

```aratu
tempo: 96
drums {
  hh = cc cc cc cc
  bd = b - b b
  sd = (s)(s) S! (s)(s) S!
}
```

> Take the parentheses off the ghost notes and hear how heavy the snare
> gets. The parentheses push those strokes into the background.

## Changing a whole part

One line takes a whole part to a dynamic, whatever is written in it:

```aratu
melody = | c4 e g e |
melody: pp
```

Try `melody: ff`, or `melody: cresc to ff over 4 bars`: a four-bar
crescendo that starts from what is written and reaches ff.

## Exercises

The first bar **piano**, the second **forte**.

```aratu exercise dynamics=1
melody = | c4 d e f | g f e d |
```

```aratu answer
melody = | p:c4 d e f | f:g4 f e d |
```

Start at `p` and make a crescendo to `f` in the next bar.

```aratu exercise dynamics=1
melody = | p:c4 d e f | f:g4 _ _ _ |
```

```aratu answer
melody = | p:c4< d e f | f:g4 _ _ _ |
```

On the snare, beats 2 and 4 marcato, with ghost notes on the eighths in
between: `(s)` on the second half of beats 1 and 3.

```aratu exercise dynamics=1
sd = s- s s- s
```

```aratu answer
sd = -(s) s! -(s) s!
```
