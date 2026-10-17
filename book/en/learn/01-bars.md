# The bar and the tempo

Every Aratu piece starts by saying two things: its **meter** and its
**tempo**.

```aratu
meter: 4/4
tempo: 90

hh = c c c c
bd = b - b -
```

`meter: 4/4` is the time signature: four beats in a bar. `tempo: 90` is the
tempo: ninety beats per minute, as on a metronome. See the round knob next
to the `90`: drag it up or down and the number changes. The music speeds up
or slows down from the next bar.

## The first rule: a space separates beats

Read the hi-hat line counting aloud: "one, two, three, four". Each group of
symbols between spaces is **one beat**. A bar of 4/4 has four groups.

When a bar has too many or too few beats, Aratu says so. This example has
one beat too many. Read the message under it and fix it:

```aratu broken
meter: 4/4
bd = b - b - b
```

## Several bars on one line

The bar line `|` separates bars, as in a score. This bass drum line has two
different bars, and the pair repeats:

```aratu
bd = | b - b - | b b b - |
sd = | - s - s | - s - s |
```

In the grid, each bar is a box, with its beats numbered above.

## Other meters

A waltz has three beats in a bar. Count "one, two, three" as you listen:

```aratu
meter: 3/4
tempo: 150

bd = b - -
hh = - c c
```

In 6/8 the beat is a dotted quarter and is counted in two, the way a
musician counts it. You will see this in the lesson on compound meters,
after you learn to divide the beat.

## Exercises

Write two bars of bass drum: in the first, it plays on beats 1 and 3; in
the second, on all four beats.

```aratu exercise
bd = | - - - - | - - - - |
```

```aratu answer
bd = | b - b - | b b b b |
```

Now a waltz in 3/4: bass drum on beat 1, snare on beats 2 and 3.

```aratu exercise
meter: 3/4
bd = - - -
sd = - - -
```

```aratu answer
meter: 3/4
bd = b - -
sd = - s s
```
