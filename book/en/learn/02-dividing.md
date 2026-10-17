# Dividing the beat

So far every beat had one symbol, a quarter note. But music divides beats:
two eighths, four sixteenths, triplets. In Aratu that takes one rule.

## The second rule: symbols together share the beat

Symbols written together, with no space, divide the beat into equal parts.

| Written | In one beat |
|---|---|
| `c` | a quarter note |
| `cc` | two eighths |
| `cccc` | four sixteenths |
| `ccc` | an eighth-note triplet |

Listen to the beat dividing more and more, from 1 to 4:

```aratu
tempo: 80
hh = c cc ccc cccc
```

Look at the grid: beat 2 has two symbols, beat 3 three, beat 4 four, all
evenly spaced.

## The third rule: `_` holds, `-` rests

Inside a beat, `_` makes the sound before it last one more part, and `-` is
a part of silence. These two pieces write the most common rhythms:

| Rhythm in one beat | Written |
|---|---|
| sixteenth, sixteenth, eighth | `ccc_` |
| sixteenth, eighth, sixteenth | `cc_c` |
| eighth, sixteenth, sixteenth | `c_cc` |
| dotted eighth, sixteenth | `c__c` or `c.c` |
| eighth rest, eighth | `-c` |

The dot is a shorthand: `c.c` is the same as `c__c`, the dotted eighth.

```aratu
tempo: 80
hh = ccc_ cc_c c_cc c.c
```

A `_` alone in a beat holds the whole beat: `c _` is a half note and
`c _ _ _` a whole note.

## Across beats

A number before the brackets stretches the group over more beats:
`2[c c c]` is three even hits in two beats, quarter-note triplets.
The lesson on compound meters comes back to this, slowly.

```aratu
tempo: 80
hh = 2[c c c] c c
```

## Exercises

Make the hi-hat play eighths for the whole bar.

```aratu exercise
hh = c c c c
```

```aratu answer
hh = cc cc cc cc
```

Quarters on beats 1 and 3, sixteenths on beats 2 and 4.

```aratu exercise
hh = c c c c
```

```aratu answer
hh = c cccc c cccc
```

A dotted eighth and a sixteenth on every beat: the "galloping" rhythm.

```aratu exercise
hh = cc cc cc cc
```

```aratu answer
hh = c.c c.c c.c c.c
```

> Hear the goal before you write. Singing the rhythm ("taaa-ta, taaa-ta")
> helps you find how to write it.
