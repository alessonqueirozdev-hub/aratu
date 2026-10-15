# Dynamics: loud and soft

Dynamic marks are the score's: `ppp pp p mp mf f ff fff`, and also `sfz`
and `fp`. Inside a bar, a mark sits on its note with a colon, and holds
from there on:

```aratu
tempo: 90
melody = | p:c4 e g c5 | f:g4 _ _ _ |
```

The first bar is piano; the second, forte. The mark does not count as a
beat.

## Crescendo and diminuendo

`<` after a note makes a crescendo and `>` a diminuendo, from the next
beat to the next mark:

```aratu
tempo: 90
melody = | p:c4< d e f | f:g4> f e d | p:c4 _ _ _ |
```

With no mark after it, a hairpin moves one step (from mf to f, say) by the
end of the line.

## Accents

An upper-case letter accents: `C4`, or `S` on the snare. An accent adds
to the dynamic the music is at.

## Changing a whole part

One line changes the level of a whole part, relative to what is written
in it:

```aratu
melody = | c4 e g e |
melody: pp
```

Try `melody: ff`, or `melody: cresc to ff over 4 bars` (a four-bar
crescendo).

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
