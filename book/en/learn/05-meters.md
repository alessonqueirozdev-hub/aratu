# Compound and irregular meters

So far the beat was always a quarter note, and a bar had two, three or
four of them. In this lesson the beat changes size: in 6/8 it divides in
three, in 7/8 the beats have different lengths, and a triplet can stretch
across two beats.

## 6/8: two beats, each divided in three

A bar of 6/8 has six eighth notes, but a musician does not count to six:
they count **two** beats, each with three eighths, "ONE-two-three,
TWO-two-three". Aratu counts the same way. Each word is a beat, a dotted
quarter, and `ccc` is its three eighths:

```aratu
meter: 6/8
tempo: 66
hh = ccc ccc
bd = b__ b__
sd = - s
```

The bass drum plays at the start of each beat and holds through the other
two eighths (`b__`); the snare marks the second beat. Notice that the tempo
counts beats, not eighths: `tempo: 66` is 66 dotted quarters a minute.

12/8 is the same, with four beats: it is the meter of the slow blues and of
many ballads.

```aratu
meter: 12/8
tempo: 60
hh = ccc ccc ccc ccc
bd = b__ -   b_b -
sd = -   s   -   s
```

## 5/4: five beats

A bar can have five beats, as in "Take Five", the best-known jazz tune in
this meter. The rule does not change: five words.

```aratu
meter: 5/4
tempo: 160
hh = cc cc cc cc cc
bd = b  -  -  b  -
sd = -  -  s  -  s
```

## 7/8: beats of different lengths

In irregular meters, common in Balkan music and in progressive rock, the
eighths are grouped into unequal beats. Say how after an `=`:
`meter: 7/8 = 2+2+3` is two beats of two eighths and one of three. Count
"ONE-two, ONE-two, ONE-two-three".

Each word is a group. A group of two eighths is written `cc`, a group of
three `ccc`:

```aratu
meter: 7/8 = 2+2+3
tempo: 240
hh = cc cc ccc
bd = b_ b_ b__
sd = -  -  -s-
```

Here the tempo counts eighths: `tempo: 240` is 240 eighths a minute. You
can also write one word per eighth, seven words, if that is easier to read:
`hh = c c c c c c c`.

> Change it to `meter: 7/8 = 3+2+2` and adjust the lines: the same bar,
> with a different swing to it.

## Triplets across beats

An eighth-note triplet fits in one beat (`ccc`). But there is also the
**quarter-note** triplet: three even notes in the space of two beats. A
number before the brackets says how many beats the group takes:

```aratu
tempo: 80
melody = | 2[c4 d e] f _ | g _ _ _ |
hh = c c c c
```

Listen to the hi-hat: the three notes of the melody spread over the first
two beats, without lining up with the second one. `3[c c c c]` is four
notes in three beats.

## Exercises

A bar of 6/8 with the hi-hat on all six eighths and the bass drum at the
start of each beat.

```aratu exercise
meter: 6/8
hh = - -
bd = - -
```

```aratu answer
meter: 6/8
hh = ccc ccc
bd = b__ b__
```

Write a bar of 7/8 grouped 2+2+3, with the hi-hat on every eighth and the
bass drum at the start of each group.

```aratu exercise
meter: 7/8 = 2+2+3
hh = - - -
bd = - - -
```

```aratu answer
meter: 7/8 = 2+2+3
hh = cc cc ccc
bd = b_ b_ b__
```

Now 7/8 as 3+2+2: the group of three comes first.

```aratu exercise
meter: 7/8 = 3+2+2
hh = - - -
bd = - - -
```

```aratu answer
meter: 7/8 = 3+2+2
hh = ccc cc cc
bd = b__ b_ b_
```

A quarter-note triplet on the first two beats (C4, D4, E4) and a half note
on F4.

```aratu exercise
melody = | c4 d e f |
```

```aratu answer
melody = | 2[c4 d e] f _ |
```
