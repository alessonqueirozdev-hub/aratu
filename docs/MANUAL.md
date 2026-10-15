# The Aratu manual

*Draft, October 2026. Aratu is not built yet: this manual describes the
language as it has been designed so far, so that the design can be read,
tried on paper and argued about. Points still open are marked **Open**.*

Aratu is written the way a musician thinks: in measures, beats, note
values, chords and score words. If you can read a lead sheet and count a bar,
you can read a Aratu piece.

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
12. [Chance](#12-chance)
13. [Sound and effects](#13-sound-and-effects)
14. [Quick reference](#14-quick-reference)

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
has the wrong number of beats, Aratu tells you in those words: *"this bar
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

**Rests** use the same rules: `-` takes one part, as a note would.

| Rest | Written in 4/4 |
|---|---|
| a whole bar, in any meter | `\| - \|` |
| eight bars | `\| - x8 \|` |
| half | `- -` |
| quarter | `-` as a beat |
| eighth | `-c` or `c-` |
| sixteenth | `-ccc`, `c-cc` |
| dotted eighth, then a sixteenth | `-__c`, or `-.c` |

A `-` alone between bar lines is a whole bar of rest, like the score's whole
rest, and `| - x8 |` counts as eight bars of rest (the `x` means "times", as
in `:| x4`). A `_` after a rest lengthens the silence.

**Compound meters.** In 6/8 the beat is a dotted quarter, so `ccc ccc` is one
bar of eighths, counted in two, as a musician counts it.

**Accents.** An upper-case symbol is accented: `cC cc` accents the "and" of
beat one.

**Bar lines.** `|` is a bar line, as in a score. A line can hold several bars:
`| cc cc | co cc |`. A line with no bar line is one bar.

**Groups.** Square brackets make a group that shares one part, and spaces
are allowed inside them: `c[cc]` is an eighth and two sixteenths, the same
as `cc_c` read the other way round. The `.` shorthand stands for `__`, so
`c.c` is `c__c`.

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

The symbol decides the sound, whatever the line is called: `x` is a crash in
any drum line. Every line inside `drums { }` is a drum line, and so is a line
called `hh`, `bd`, `sd`, `cr`, `rd`, `kick`, `snare`, `hat`, `ride` or
`crash` anywhere in the piece.

*Provisional:* the symbol table above. **Open:** more drums and symbols, and
how a user maps them to their own samples.

## 4. Melodies

Notes are written in lower case, with the octave: `c4` is middle C.
An apostrophe after a note makes it short, staccato: `c4'` (on drums too:
`c'` is a short hi-hat). Sharps
and flats follow the letter: `f#4`, `eb4`. Solfège works too: `dó4 ré4 mi4`.
Upper case accents a note: `C4`.

```
melody = | c4 e4 g4 _ | a4 g4 e4 c4 |
```

The octave carries over until it changes, so `| c4 e g c5 |` is C4, E4, G4,
C5. A note written before any octave is taken as octave 4, and Aratu says so.

Several notes in one beat follow rule 2 inside square brackets: `[d e]` is
two eighths, `[b b]` two B eighths, `[eb g]` E-flat then G, `[c4 _ _ d]` a
dotted eighth and a sixteenth. Rests and holds can sit next to a note
without brackets (`-e4` is a rest, then an eighth), but two notes cannot:
`eb` would read as E-flat, so for two notes Aratu asks for the brackets.

Notes sounding together are joined with `+`: `c4+e4+g4` is a C major chord
on one beat.

*Provisional (15 October 2026):* brackets are required for two or more
notes in one beat.

## 5. Chords and strumming

A progression is written as a lead sheet (a *cifra*). The rule of sharing
works one level up: chords that share a bar divide it equally.

```
chords = | Am | F | C G | G |      // C and G take half a bar each
chords = | Am _ _ F |              // Am for three beats, F for one
```

The shares must fall on the beats. In 4/4, `| C G |` gives two beats to
each chord, but three chords would split the bar between beats, so Aratu asks
how long each lasts: `| Am _ F G |`.

Chord symbols keep their usual spelling: `Am`, `F`, `G7`, `Cmaj7`, `Dm7b5`,
`E/G#`, and the spellings of a Brazilian *cifra* too: `C7M`, `Bº`, `G7(b9)`.

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

The strum repeats over every chord (a pattern loops over its own length, so
a one-bar strum repeats every bar), and the chords change on their own at
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

A live mark sets the level of the whole part around the marks written in its
lines: at `mf` they sound as written, at `ff` everything is louder by the
same proportion, and `fade out` takes the part to silence.

Each mark stands for a fixed velocity: ppp 16, pp 33, p 49, mp 64, mf 80,
f 96, ff 112, fff 127. An accent adds to the current one. A hairpin with no
mark after it moves one step (from `mf` to `f`, say) by the end of the line.

## 7. Tempo and time

```
tempo: 96                         // at the start
at bar 30: tempo 130              // a change at a bar
rit to 80 over 2 bars             // ritardando (rall works too)
accel to 140 over 4 bars          // accelerando
a tempo                           // back to the tempo before the rit
tempo primo                       // back to the first tempo
tempo 120 -> 140 over 4 bars      // a ramp between two tempos
```

**When a change happens.** A change with `at bar N:` happens at that bar. A
change written on its own happens where the piece starts or, while you play,
at the next bar line after you write it. So in a composition, say where:
`at bar 28: rit to 80 over 2 bars`.

**Fermata.** `^` on a note holds it: `g4^` lasts twice its value, and every
part waits for it. **Open:** how to write a fermata of a given length
(`fermata 3 beats`).

**Pickup (anacrusis).** `pickup: beat 4` starts the piece on the fourth beat
of an incomplete first bar. The pickup is bar 0, so bar 1 is the first full
bar, as musicians number them. A line whose first bar is as long as the
pickup starts in the pickup; the others start at bar 1.

**Meter changes.** `at bar 17: meter 7/8`.

During a ritardando or accelerando every note still lands on its exact
sample: the tempo changes evenly beat by beat, as the score reads, and the
engine integrates that curve instead of stepping it.

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
section intro  { bass = | a2 _ _ _ | e2 _ _ _ | }
section verse  {
  bass = | a2 _ e2 _ | f2 _ c3 _ |
  drums { hh = cc cc co cc; bd = b - b - }
}
section chorus {
  guitar { chords = | Am | F | C | G |; strum = D du -u du }
}
section outro  { bass = | a2 _ _ _ | }

form: intro verse x2 chorus verse chorus x2 outro
```

A section holds parts and lines the way the whole piece does, and lasts as
long as its longest line. The form plays the sections in order (`x2` plays one
twice) and then starts again. A block can sit on one line, with `;` between
its lines.

### Varying a loop while playing

Patterns loop by themselves, so playing live needs a way to say when to vary:

```
pattern groove { hh = cc cc co cc; bd = b - b -; sd = - s - s }
pattern fill   { hh = cc cc cc cc; sd = ss ss SS SS }
pattern crash  { cr = x - - - }

play groove x3, fill              // a four-bar phrase, looped
every 8 bars, first: add crash    // the first bar of every eight
```

`play` chooses the pattern that sounds; `play groove x3, fill` plays a
phrase of patterns in turn and loops it. `every 4 bars: play fill` plays the
fill in the fourth bar of every four, in place of the groove; `add` plays a
pattern on top of the one that is sounding, so `every 8 bars, first: add
crash` puts a crash on the first beat of every eight bars.

*Provisional:* the syntax of `section` and the word `add`.

## 10. Playing live

While a piece plays, you change it, you do not rewrite it.

- **Everything loops.** Each line plays over and over, each over its own
  length: a one-bar hi-hat, a two-bar bass and a four-bar progression line up
  as a band's parts do.
- **A change is one line.** `hh = cc cc oc cc` replaces the hi-hat and
  nothing else; the other parts keep playing.
- **Changes land on the beat.** An edit takes effect at the next bar line.
  `now:` makes it take effect at the next beat instead.
- **Prepare ahead.** `at bar 17: play fill` schedules a change.
- **Out and in.** `guitar: out` silences a part from the next bar line and
  `guitar: in` brings it back, as on a mixer; `all: out` stops everything.
  They work with `at bar 17:` and `every 8 bars, first:` too.
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

## 12. Chance

A loop that repeats exactly the same every bar soon sounds mechanical. A
drummer never plays two bars quite alike: a hi-hat note drops out, a ghost
note appears, the bass changes its first note now and then. Chance gives a
loop that life without you touching it: the computer decides, a little at
random, what to play each time round. It is the *ad lib* of a score.

**Sometimes.** A `?` after a symbol plays it only some of the time, half of
the time by default:

```
hh = cc cc c? cc
```

The eighth on beat three comes and goes, so every bar is slightly different.
A percentage sets how often:

```
hh = cc cc c?30% cc         // three times in ten
sd = - s - s?80%            // the last backbeat almost always
```

**One or the other.** Curly brackets choose one of several, separated by
`|`, each time the loop comes round:

```
bass = | {c2|g1} _ c2 _ |   // each repeat starts on either C or G
hh   = {cc|co} cc cc cc     // the first beat is closed, or closed then open
```

**The same chance again.** Each `?` and each choice is drawn again every time
the loop passes, but from a fixed seed, so the same piece improvises the same
way each time it is played from the start: useful for recording or
rendering. `seed 7` picks another improvisation; `seed random` makes every
performance different.

## 13. Sound and effects

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

## 14. Quick reference

| Write | Means |
|---|---|
| `meter: 4/4`, `tempo: 96`, `key: A minor` | the frame of the piece |
| space | next beat |
| `cc`, `cccc`, `ccc` | eighths, sixteenths, a triplet in one beat |
| `_` | hold |
| `-` | rest |
| `\| - \|`, `\| - x8 \|` | a whole bar of rest, eight bars of rest |
| `c.c` | dotted eighth and sixteenth |
| upper case | accent |
| `c4'` | staccato |
| `c?`, `c?30%` | sometimes (half the time, or how often) |
| `{c2\|g1}` | one or the other, chosen each time round |
| `seed 7`, `seed random` | the same improvisation again, or a new one each time |
| `\|` | bar line |
| `c4 e g` | notes, octave carried over |
| `f#4`, `eb4`, `dó4` | accidentals, solfège |
| `[d e]` | notes grouped in one beat (needed for two or more notes) |
| `c4+e4+g4` | notes together |
| `\| Am \| F \| C G \|` | chords, shared bars divide equally |
| `d u m` | strum down, up, muted |
| `p:`, `mf:`, `f:` | dynamic mark on a beat |
| `<`, `>` | crescendo, diminuendo |
| `cresc to ff over 4 bars` | a live dynamic change |
| `rit`, `accel`, `a tempo`, `tempo primo` | tempo changes |
| `tempo 120 -> 140 over 4 bars` | a tempo ramp |
| `g4^` | fermata |
| `pickup: beat 4` | anacrusis |
| `rubato 10%`, `swing 60%`, `humanize` | feel |
| `pattern name { }`, `play name`, `add name` | named patterns, played in turn or on top |
| `\|: :\|`, `:\| x4`, `\|1 \|2` | repeat, count, endings |
| `%`, `%%` | repeat the previous bar, two bars |
| `segno:`, `coda:`, `fine:` | markers |
| `D.C.`, `D.S. al fine`, `D.S. al coda`, `to coda` | jumps |
| `section name { }`, `form: intro verse x2 chorus` | sections and the form |
| `every 4 bars: play fill` | a variation inside a loop |
| `at bar 30:` | a scheduled change |
| `now:` | a change at the next beat |
| `guitar: out`, `guitar: in` | silence a part, bring it back |
| `->` | signal flow |
| `part.param = value` | a parameter, drawn as a knob |
