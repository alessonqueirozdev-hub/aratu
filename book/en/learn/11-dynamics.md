# Dynamics and articulation

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

## Ornaments: trill, mordent and grace note

Ornaments decorate a note with its neighbour in the key. They go after the
note, with a colon, like the chords' `:arp`:

| Ornament | Written | How it sounds |
|---|---|---|
| trill | `c5:tr` | the note and the one above, quickly in turn, for its whole length |
| mordent | `c5:mord` | the note, the one above and the note again, very fast, at its start |
| lower mordent | `c5:mordi` | the same, with the one below |
| grace note | `` `d5 c5 `` | a small note, right before the main one |

The neighbour is the key's: in G major, the trill on `a4` goes to `b4`, and
the one on `e5` to `f#5`. To change it, put `#` or `b` after the `tr`, like
the accidental over a score's *tr*: `e5:trb` trills with F natural.

```aratu
tempo: 80
key: G major
melody = | g4 b4 `c5 d5 _ | d5:mord c5 b4 a4 | a4:tr _ _ _ | g4 _ _ _ |
```

A grace note takes no beat: it sounds an instant before the main note. On
a drum, the backtick does the same with the drum itself: `` `s `` is a
**flam**, a light stroke just before, and ``` ``s ``` is a **drag**, two:

```aratu
tempo: 90
drums {
  hh = cc cc cc cc
  bd = b - b -
  sd = - `S - ``S
}
```

> Take the backticks off and hear the dry snare; with them, each hit is
> dragged into.

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

Decorate the melody: a **mordent** on the first e5 and a **trill** on the
last.

```aratu exercise
tempo: 80
melody = | e5 d5 c5 d5 | e5 _ _ _ |
```

```aratu answer
tempo: 80
melody = | e5:mord d5 c5 d5 | e5:tr _ _ _ |
```

A grace note `d5` before each c5.

```aratu exercise
melody = | c5 g4 c5 g4 |
```

```aratu answer
melody = | `d5 c5 g4 `d5 c5 g4 |
```

On the snare, a **flam** on beat 2 and a **drag** on beat 4.

```aratu exercise
sd = - s - s
```

```aratu answer
sd = - `s - ``s
```
