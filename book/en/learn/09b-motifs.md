# Motifs, sequences and rows

Much music is made of a few ideas used many times: a motif that comes
back, rises or falls, turns upside down. Aratu lets you write the idea once
and use it as often as you like.

## Repeating a beat

An `x` glued to the bracket repeats that beat. Each repeat takes its own
beat:

```aratu
tempo: 100
melody = | [e4 d c d]x4 |
```

It is the same as writing `[e4 d c d]` four times. The `x` is always glued
to the `]`; with a space, `| - x8 |` still means eight bars of rest.

## Motifs

A motif is a passage with a name. Define it with `motif` and use the name in
place of the beats:

```aratu
tempo: 100
motif riff = [e4 d c d]
melody = | riff riff riff riff |
```

> The motif may be written before or after the line that uses it. While it
> plays, the editor lights the notes in the motif's definition.

## Sequences

In music theory, a **sequence** is a motif repeated at another pitch.
`riff+1` is the motif one degree up, in the key (`key:`), and `riff-1` one
degree down:

```aratu
tempo: 100
key: C major
motif riff = [e4 d c d]
melody = | riff riff+1 riff+2 riff+3 |
```

By degrees, the motif stays in the scale: E to F is one degree, like C to
D. To count in semitones, use `st`: `riff+3st` goes up three semitones.

## Inversion and retrograde

| Write | The motif |
|---|---|
| `riff.inv` | inverted: each interval upside down, from the first note |
| `riff.retro` | retrograde: the notes backwards, in the same rhythm |
| `riff.ri` | the retrograde of the inversion |

```aratu
tempo: 100
motif riff = [e4 d c d]
melody = | riff riff.inv riff.retro riff.ri |
```

They combine with transposition too: `riff.ri+2` is the retrograde
inversion two degrees up.

## Ostinato

An ostinato is a motif repeated underneath while the rest changes. In Aratu
every line repeats on its own already, so one bar is enough:

```aratu
tempo: 100
motif ost = [a2 e3 a3 e3]
bass = | ost ost ost ost |
piano {
  chords = | Am | F | C | G |
}
```

## Twelve-tone rows

In twelve-tone music a **row** orders the twelve notes of the chromatic
scale, each once. Write the row as a vector of numbers from 0 to 11 (0 is
C, 1 is C#, …) or with note names:

```aratu
tempo: 80
row serie = 0 11 3 4 8 7 9 5 6 1 2 10
melody = | o o o o | o o o o | o o o o | from serie.P0
```

The row gives the pitches and the rhythm gives their places: each `o` takes
the next note of the row. When the rhythm has more `o`s than the row has
notes, the row starts again. A row has four forms, each in twelve
transpositions:

| Form | What it is |
|---|---|
| `serie.P0` | the row as written (P for *prime*) |
| `serie.I0` | the inversion, from the same first note |
| `serie.R0` | the retrograde, backwards |
| `serie.RI0` | the retrograde of the inversion |
| `serie.P5`, `serie.RI3` | the form transposed 5 or 3 semitones up |

`octave 3` at the end puts every note in octave 3, and `from [0 4 7]` uses
a vector written right there. A twelve-note row that repeats a note gets a
warning. The **Rows** tab, beside the messages in Play, shows each row's
12×12 matrix: click a form's name to write it into the code.

## Melody lists

To prepare a performance, keep the melodies in a list, one per line, and
pick one by its number:

```aratu
tempo: 100
list temas {
  | e4 d c d | e e e _ |
  | d4 d d _ | e g g _ |
  | e4 d c d | e e e e |
}
melody = temas.1
```

> Change `temas.1` to `temas.2` while it plays: the change comes in at the
> next bar. `temas.all` plays them all, in order, and `temas.random` draws a
> whole one each time round.

On stage, buttons appear to choose each list's melody without typing in
front of the audience, and Alt+1 to Alt+9 do the same from the keyboard.

## Exercises

Repeat the first beat in all four beats of the bar, with `x`.

```aratu exercise
melody = | [e4 d c d] - - - |
```

```aratu answer
melody = | [e4 d c d]x4 |
```

Make the motif rise one degree on each beat, as a sequence.

```aratu exercise
motif riff = [c4 d e]
melody = | riff riff riff riff |
```

```aratu answer
motif riff = [c4 d e]
melody = | riff riff+1 riff+2 riff+3 |
```

Play the row backwards: its retrograde form.

```aratu exercise
row serie = 0 11 3 4 8 7 9 5 6 1 2 10
melody = | o o o o | o o o o | o o o o | from serie.P0
```

```aratu answer
row serie = 0 11 3 4 8 7 9 5 6 1 2 10
melody = | o o o o | o o o o | o o o o | from serie.R0
```
