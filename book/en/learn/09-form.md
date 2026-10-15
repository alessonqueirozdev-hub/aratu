# Repeats and jumps

A score's repeat signs are written in Aratu too.

## Repeat signs

`|:` and `:|` enclose a passage that is played twice:

```aratu
tempo: 110
melody = |: c4 e g e | d f a f :| c4 _ _ _ |
```

Listen: the first two bars twice, then the ending. To play it more times,
write how many after `:|`: `:| x4`.

## First and second endings

`|1` starts the first ending and `:|2` the second:

```aratu
chords = |: C | F |1 G | G :|2 C | C |
```

The first time: C, F, G, G. The second: C, F, C, C.

## Repeating the bar before

`%` repeats the bar before; `%%` repeats the two bars before:

```aratu
chords = | Am | % | F | % |
```

## D.C., D.S., fine and coda

`D.C.` goes back to the start and `D.S.` back to the sign `segno:`.
`al fine` stops at the bar marked `fine:`:

```aratu
tempo: 110
melody = | c4 e g c5 | segno: d4 f a _ | fine: c4 _ _ _ | g4 _ _ _ D.S. al fine |
```

Playing order: bars 1, 2, 3, 4, back to 2, 3, and the end.

> If a `D.S.` has no `segno:`, Aratu says exactly that, with the bar
> number.

## Exercises

Repeat the first two bars and end on the third.

```aratu exercise
melody = | c4 e g e | d f a f | c4 _ _ _ |
```

```aratu answer
melody = |: c4 e g e | d f a f :| c4 _ _ _ |
```

Use `%` so that Am lasts two bars and F too, in this order: Am, Am, F, F.

```aratu exercise
chords = | Am | F |
```

```aratu answer
chords = | Am | % | F | % |
```
