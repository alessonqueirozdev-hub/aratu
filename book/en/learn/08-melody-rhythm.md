# Rhythm in a melody

The rhythm rules work for notes too, with one care: two notes in the same
beat go in **brackets**.

```aratu
tempo: 90
melody = | [c4 d] e [f g] a | g _ _ _ |
```

`[c4 d]` is two eighths in one beat. Why brackets? Because `eb` is E-flat:
without brackets, Aratu could not tell `eb` from an E followed by a B.
Rests and holds can sit next to a note: `-e4` is an eighth rest, then an E.

## Long notes and ties

`_` makes a note last one more beat (or one more part, inside brackets). A
`_` at the start of a bar ties over from the bar before:

```aratu
melody = | c4 _ e _ | g _ _ _ | _ _ e _ |
```

The G lasts six beats: four in the second bar and two more in the third.

## Dotted notes and staccato

Inside brackets, a dotted eighth is `[c4 _ _ d]` (the dot works too:
`[c4. d]`). An apostrophe makes a note short, staccato:

```aratu
tempo: 100
melody = | e4' e' f' g' | g f e d |
```

## Several notes at once

`+` joins notes that sound together, a chord:

```aratu
keys = | c4+e4+g4 _ _ _ | d4+f4+a4 _ b3+d4+g4 _ |
```

## The "Ode to Joy"

Beethoven's theme, in the public domain, has a dotted rhythm at the end:

```aratu
tempo: 110
melody = | e4 e f g | g f e d | c c d e | e [_ d] d _ |
```

The last bar: E lasts a beat and a half (`e` and the first half of
`[_ d]`), D lasts half a beat, and the last D lasts two beats.

## Exercises

End the "Ode to Joy" with the other phrase, which lands on C: in the last
bar, `d [_ c] c _`.

```aratu exercise
tempo: 110
melody = | e4 e f g | g f e d | c c d e | d _ _ _ |
```

```aratu answer
tempo: 110
melody = | e4 e f g | g f e d | c c d e | d [_ c] c _ |
```

Write a C major chord (C4, E4, G4) that lasts the whole bar.

```aratu exercise
keys = | c4 _ _ _ |
```

```aratu answer
keys = | c4+e4+g4 _ _ _ |
```

Eighths with the first of each beat staccato: `[c4' d]` on all four beats.

```aratu exercise
melody = | [c4 d] [c d] [c d] [c d] |
```

```aratu answer
melody = | [c4' d] [c' d] [c' d] [c' d] |
```
