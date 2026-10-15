# The MusiMangue manual

*Draft, October 2026. MusiMangue is not built yet: this manual describes the
language as it has been designed so far, so that the design can be read,
tried on paper and argued about. Points still open are marked **Open**.*

MusiMangue is written the way a musician thinks: in measures, beats, note
values, chords and score words. If you can read a lead sheet and count a bar,
you can read a MusiMangue piece.

## Contents

1. [A first piece](#1-a-first-piece)
2. [Rhythm: beats and their parts](#2-rhythm-beats-and-their-parts)
3. [Drums](#3-drums)
4. [Melodies](#4-melodies)
5. [Chords and strumming](#5-chords-and-strumming)
6. [Dynamics](#6-dynamics)
7. [Tempo and time](#7-tempo-and-time)
8. [Feel: rubato, swing and humanize](#8-feel-rubato-swing-and-humanize)
9. [Patterns, sections, repeats and form](#9-patterns-sections-repeats-and-form)
10. [Playing live](#10-playing-live)
11. [Transformations](#11-transformations)
12. [Sound and effects](#12-sound-and-effects)
13. [Quick reference](#13-quick-reference)

---

## 1. A first piece

```
meter: 4/4
tempo: 96
key: A minor

drums {
  hh = cc cc co cc
  bd = b - b -
  sd = - s - s
}

bass = | a2 _ e2 _ | f2 _ c3 _ |

guitar {
  chords = | Am | F | C G | G |
  strum  = D du -u du
}
```

The first lines set the frame of the piece: meter, tempo and key. Each part
follows. There is no `main`, no braces you must balance before the music
plays, and no word that exists only for the computer.

## 2. Rhythm: beats and their parts

Three rules describe every rhythm in a bar.

**Rule 1. A space separates beats.** A line in 4/4 has four groups. If a bar
has the wrong number of beats, MusiMangue tells you in those words: *"this bar
has 5 beats, but the meter is 4/4"*.

**Rule 2. Symbols written together share their beat equally.**

| Written | In one beat |
|---|---|
| `c` | a quarter note |
| `cc` | two eighths |
| `cccc` | four sixteenths |
| `ccc` | an eighth-note triplet |
| `ccccc` | a quintuplet |

**Rule 3. `_` holds the note before it for one more part; `-` is a rest.**

| Rhythm in one beat | Written |
|---|---|
| sixteenth, sixteenth, eighth | `ccc_` |
| sixteenth, eighth, sixteenth | `cc_c` |
| eighth, sixteenth, sixteenth | `c_cc` |
| dotted eighth, sixteenth | `c__c`, or the shorthand `c.c` |
| rest, then an eighth | `-c` |

A `_` on its own is a whole beat held, so `c _` is a half note and `c _ _ _`
a whole note. A `_` at the start of a bar ties across the bar line.

**Compound meters.** In 6/8 the beat is a dotted quarter, so `ccc ccc` is one
bar of eighths, counted in two, as a musician counts it.

**Accents.** An upper-case symbol is accented: `cC cc` accents the "and" of
beat one.

**Bar lines.** `|` is a bar line, as in a score. A line can hold several bars:
`| cc cc | co cc |`.

## 3. Drums

Each drum has a line, and each line follows the rhythm rules. The symbols are
short and chosen per drum: `c` closed hi-hat, `o` open hi-hat, `b` bass
drum, `s` snare, `r` ride, `x` crash.

```
drums {
  hh = cc cc co cc
  bd = b - bb -
  sd = - s - S        // accented backbeat on four
}
```

**Open:** the full table of drum names and symbols, and how a user maps them
to their own samples.

## 4. Melodies

Notes are written in lower case, with the octave: `c4` is middle C. Sharps
and flats follow the letter: `f#4`, `eb4`. Solfège works too: `dó4 ré4 mi4`.
Upper case accents a note: `C4`.

```
melody = | c4 e4 g4 _ | a4 g4 e4 c4 |
```

The octave carries over until it changes, so `| c4 e g c5 |` is C4, E4, G4,
C5.

Several notes in one beat follow rule 2: written together, they share the
beat. Square brackets group them explicitly when letters alone would be
ambiguous: `[d e]` is two eighths, `[b b]` two B eighths, `[eb g]` E-flat
then G.

Notes sounding together are joined with `+`: `c4+e4+g4` is a C major chord
on one beat.

**Open:** whether gluing note names without brackets (`de` for two eighths)
is allowed, given flats written with `b`.

## 5. Chords and strumming

A progression is written as a lead sheet (a *cifra*). The rule of sharing
works one level up: chords that share a bar divide it equally.

```
chords = | Am | F | C G | G |      // C and G take half a bar each
chords = | Am _ _ F |              // Am for three beats, F for one
```

Chord symbols keep their usual spelling: `Am`, `F`, `G7`, `Cmaj7`, `Dm7b5`,
`E/G#`.

**Strumming.** A strum pattern uses the rhythm rules with guitar symbols:

| Symbol | Meaning |
|---|---|
| `d` | strum down |
| `u` | strum up |
| `m` | muted strum (a chuck) |
| `-` | the hand passes without striking |
| `_` | let it ring |
| upper case | accented strum |

```
guitar {
  chords = | Am | F | C G | G |
  strum  = D du -u du             // down, down-up, -up, down-up
}
```

The strum repeats over every chord, and the chords change on their own at
the right bar. A down strum sounds the strings from low to high a few
milliseconds apart, an up strum from high to low; accents strum harder and
faster.

**Open:** the table of chord shapes on the fretboard, and how a user chooses a
voicing (open chord, barre, position).

## 6. Dynamics

Dynamic marks are the score's: `ppp pp p mp mf f ff fff`, and `sfz`, `fp`.

**Inside a bar**, a mark attaches to the beat it starts with a colon, so it
does not count as a beat:

```
melody = | p:c4 e4 g4 c5 | f:g4 _ _ _ |
```

**Hairpins** are written as in a score: `<` crescendo, `>` diminuendo, from the
beat after them to the next mark.

```
melody = | p:c4< d4 e4 f4 | f:g4> f4 e4 d4 | p:c4 _ _ _ |
```

**Live**, one line changes a part:

```
piano: mf
piano: cresc to ff over 4 bars
drums: dim to p over 2 bars
all: fade out over 8 bars
```

Each mark stands for a fixed velocity: ppp 16, pp 33, p 49, mp 64, mf 80,
f 96, ff 112, fff 127. An accent adds to the current one.

## 7. Tempo and time

```
tempo: 96                         // at the start
at bar 30: tempo 130              // a change at a bar
rit to 80 over 2 bars             // ritardando (rall works too)
accel to 140 over 4 bars          // accelerando
a tempo                           // back to the tempo before the rit
tempo primo                       // back to the first tempo
```

**Fermata.** `^` on a note holds it: `g4^` lasts twice its value by default;
`fermata 3 beats` sets the length.

**Pickup (anacrusis).** `pickup: beat 4` starts the piece on the fourth beat
of an incomplete first bar.

**Meter changes.** `at bar 17: meter 7/8`.

During a ritardando or accelerando every note still lands on its exact
sample: the engine integrates the tempo curve instead of stepping it.

## 8. Feel: rubato, swing and humanize

**Rubato** means robbed time, and robbed time is given back.
`piano: rubato 10%` stretches and shrinks the beats inside each phrase and
arrives at the bar line on time, so the piano breathes while the drums keep
the pulse. `all: rubato 10%` bends the whole piece together.

**Swing.** `swing 60%` delays every second eighth (50% is straight, 66% a
triplet feel).

**Humanize.** `humanize` adds small random deviations of timing and velocity,
as a player would; `humanize 5 ms` sets how much.

## 9. Patterns, sections, repeats and form

Material is prepared in advance and given a name:

```
pattern groove {
  hh = cc cc co cc
  bd = b - b -
  sd = - s - s
}

pattern fill {
  hh = cc cc cc cc
  bd = bb -b bb -b
  sd = ss ss SS SS
}

play groove
```

Changes are scheduled by bar:

```
at bar 8: play fill
at bar 9: play groove
at bar 30: tempo 130
```

### Repeat signs

The signs of a score, written in text:

```
chords = |: Am | F |1 C | G :|2 C | C |      // repeat, first and second endings
chords = |: Am | F :| x4                      // play the passage four times
chords = | Am | % | % | F |                   // % repeats the previous bar
chords = | Am | F | %% |                      // %% repeats the previous two
```

`|:` and `:|` mark a repeated passage, played twice unless `x` says how many
times. `|1` and `|2` start the first and second endings.

### Jumps

Markers go at the start of a bar, jumps at its end:

```
melody = | c4 e g c5 | segno: d4 f a _ | e4 g b _ | fine: c4 _ _ _ | g4 _ _ _ D.S. al fine |
```

| Marker | Jump |
|---|---|
| `segno:` | `D.S.`, `D.S. al fine`, `D.S. al coda` |
| `fine:` | where `al fine` stops |
| `coda:` | where `to coda` goes |
| | `D.C.`, `D.C. al fine`: back to the start |

The piece is unrolled as a musician reads it, and the editor shows the result
on the timeline. A jump without its marker is an error stated plainly:
*"there is a D.S. at bar 24 but no segno before it"*.

### Form

Sections are named parts of a piece, and a form line orders them:

```
section verse  { ... }
section chorus { ... }

form: intro verse x2 chorus verse chorus x2 outro
```

### Varying a loop while playing

Patterns loop by themselves, so playing live needs a way to say when to vary:

```
every 4 bars: drums fill          // the fourth bar of every four
every 8 bars, first: crash        // the first bar of every eight
play groove x3, fill              // a four-bar phrase, looped
```

**Open:** the exact syntax of `section`.

## 10. Playing live

While a piece plays, you change it, you do not rewrite it.

- **A change is one line.** `hh = cc cc oc cc` replaces the hi-hat and
  nothing else; the other parts keep playing.
- **Changes land on the beat.** An edit takes effect at the next bar line.
  `now:` makes it take effect at the next beat instead.
- **Prepare ahead.** `at bar 17: play fill` schedules a change.
- **Nothing restarts.** Parts keep their place in the bar when another part
  changes.

## 11. Transformations

Short verbs change a part or a pattern without rewriting it:

| Verb | Effect |
|---|---|
| `hh.rotate 1` | shifts the pattern by one part |
| `hh.density 50%` | keeps half of the hits |
| `drums.velocity 0.8` | plays softer |
| `bass.transpose +12` | an octave up |
| `melody.invert` | turns the intervals upside down |
| `melody.retrograde` | plays it backwards |
| `melody.augment` | doubles the note values |
| `melody.diminish` | halves the note values |

## 12. Sound and effects

Each part has a sound and a chain of effects, written as signal flow:

```
bass: synth.saw -> lowpass -> reverb
```

Parameters are set with numbers, and **every number is also a control**: the
editor draws a knob beside it, shaped by the parameter (a cutoff knob reads in
hertz on a logarithmic scale, a mix knob from 0 to 1). Turning the knob
rewrites the number in your code, so saving the file saves the performance.

```
bass {
  reverb.mix = 0.42
  lowpass.cutoff = 800 Hz
  lowpass.resonance = 0.3
}
```

Parameters can change by bar like anything else:

```
at bar 16: bass.reverb.mix = 0.65
```

Instruments made with Parabolic can be used as sounds.

## 13. Quick reference

| Write | Means |
|---|---|
| `meter: 4/4`, `tempo: 96`, `key: A minor` | the frame of the piece |
| space | next beat |
| `cc`, `cccc`, `ccc` | eighths, sixteenths, a triplet in one beat |
| `_` | hold |
| `-` | rest |
| `c.c` | dotted eighth and sixteenth |
| upper case | accent |
| `\|` | bar line |
| `c4 e g` | notes, octave carried over |
| `f#4`, `eb4`, `dó4` | accidentals, solfège |
| `[d e]` | notes grouped in one beat |
| `c4+e4+g4` | notes together |
| `\| Am \| F \| C G \|` | chords, shared bars divide equally |
| `d u m` | strum down, up, muted |
| `p:`, `mf:`, `f:` | dynamic mark on a beat |
| `<`, `>` | crescendo, diminuendo |
| `cresc to ff over 4 bars` | a live dynamic change |
| `rit`, `accel`, `a tempo`, `tempo primo` | tempo changes |
| `g4^`, `fermata 3 beats` | fermata |
| `pickup: beat 4` | anacrusis |
| `rubato 10%`, `swing 60%`, `humanize` | feel |
| `pattern name { }`, `play name` | named patterns |
| `\|: :\|`, `:\| x4`, `\|1 \|2` | repeat, count, endings |
| `%`, `%%` | repeat the previous bar, two bars |
| `segno:`, `coda:`, `fine:` | markers |
| `D.C.`, `D.S. al fine`, `D.S. al coda`, `to coda` | jumps |
| `form: intro verse x2 chorus` | the form |
| `every 4 bars: drums fill` | a variation inside a loop |
| `at bar 30:` | a scheduled change |
| `now:` | a change at the next beat |
| `->` | signal flow |
| `part.param = value` | a parameter, drawn as a knob |
