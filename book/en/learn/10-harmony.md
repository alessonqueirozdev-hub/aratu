# Harmony by degree

In the last lesson chords had names: Am, F, C, G. Harmony students often
think another way: not "A minor" but "the sixth degree of the key".
Writing by degree shows what each chord does, and the same progression
works in any key.

## The degrees of the key

Degrees are written in Roman numerals, from I to VII, counted from the key
of the piece, which goes in `key:`. In C major, I is C, IV is F, V is G:

```aratu
key: C major
tempo: 90
chords = | I | IV | V | I |
```

**Upper case is a major chord, lower case a minor one.** In C major the
second, third and sixth degrees are minor: `ii` is Dm, `iii` is Em, `vi`
is Am. The most played progression in pop is this one:

```aratu
key: C major
tempo: 90
chords = | I | vi | IV | V |
```

> Write `key: G major` in place of `key: C major` and listen: the same
> progression, now in G. That is how a song is moved to another key to fit
> the singer's voice.

## Sevenths and other chords

After the degree comes the rest of the chord symbol, as on any lead sheet:
`V7` is the dominant seventh chord, `ii7` the second degree with a
seventh, `Imaj7` the first degree with a major seventh. `viidim` (or
`vii°`) is the diminished chord on the seventh degree.

```aratu
key: C major
tempo: 80
chords = | ii7 | V7 | Imaj7 | viidim |
```

The ii–V–I is the most important cadence in jazz and bossa nova.

## Minor keys

In a minor key the degrees change: in A minor, `i` is Am, `iv` is Dm, and
`V7` is E7, the dominant with the leading note:

```aratu
key: A minor
tempo: 90
chords = | i | iv | V7 | i |
```

## Chords from outside the key

A flat or sharp before the degree borrows a chord from outside the key:
`bVII` in C major is Bb, the chord that gives that rock flavour. And a
slash makes a secondary dominant: `V7/V` is the dominant of the dominant,
D7 in C major, which leads to G.

```aratu
key: C major
tempo: 90
chords = | I | bVII | IV | I | I | V7/V | V7 | I |
```

## Rolling a chord

With `:arp` after the chord, its notes sound one after another, from the
bottom up, like a harp or a guitar picked slowly. It works on any chord,
by name or by degree:

```aratu
key: C major
tempo: 70
chords = | I:arp | vi:arp | IV:arp | V7:arp |
```

## With the guitar strum

Degrees work on the chords line of any instrument, strum and all:

```aratu
key: D major
tempo: 96
guitar {
  chords = | I | V | vi | IV |
  strum  = D du -u du
}
```

## Exercises

Write I, IV, V, I in C major, one chord per bar.

```aratu exercise
key: C major
chords = | I | I | I | I |
```

```aratu answer
key: C major
chords = | I | IV | V | I |
```

The same progression by degree, but now in G major. Change only the key.

```aratu exercise
key: C major
chords = | I | vi | IV | V |
```

```aratu answer
key: G major
chords = | I | vi | IV | V |
```

The ii–V–I cadence in C major: `ii7`, `V7` and `Imaj7`, with the last one
lasting two bars.

```aratu exercise
key: C major
chords = | I | I | I | I |
```

```aratu answer
key: C major
chords = | ii7 | V7 | Imaj7 | % |
```

In A minor: i, iv, V7, i, with the last chord rolled.

```aratu exercise
key: A minor
chords = | i | i | i | i |
```

```aratu answer
key: A minor
chords = | i | iv | V7 | i:arp |
```
