# The Aratu manual

Aratu is written the way a musician thinks: in bars, beats, note values,
chords and score words. If you can read a lead sheet and count a bar, you
can read an Aratu piece. This manual describes the whole language; the
**Learn** course teaches it step by step. Every example here can be heard
and changed. For people who program, [**Inside**](#/por-dentro) gives each
thing its technical name and shows what the computer does with the text.

## 1. A first piece

```aratu
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
follows. There is no `main`, no braces to balance before the music plays,
and no word that exists only for the computer. `//` starts a comment.

## 2. Rhythm: beats and their parts

Three rules describe every rhythm in a bar.

**Rule 1. A space separates beats.** A line in 4/4 has four groups. If a bar
has the wrong number of beats, Aratu says so in those words: *"bar 2 of
`bass` has 5 beats, but the meter is 4/4"*.

**Rule 2. Symbols written together share their beat equally.**

| Written | In one beat |
|---|---|
| `c` | a quarter note |
| `cc` | two eighths |
| `cccc` | four sixteenths |
| `ccc` | an eighth-note triplet |
| `ccccc` | a quintuplet |

**Rule 3. `_` holds the sound before it for one more part; `-` is a rest.**

| Rhythm in one beat | Written |
|---|---|
| sixteenth, sixteenth, eighth | `ccc_` |
| sixteenth, eighth, sixteenth | `cc_c` |
| eighth, sixteenth, sixteenth | `c_cc` |
| dotted eighth, sixteenth | `c__c`, or the shorthand `c.c` |
| eighth rest, eighth | `-c` |

A `_` on its own is a whole beat held, so `c _` is a half note and `c _ _ _`
a whole note. A `_` at the start of a bar ties across the bar line.

**Rests** follow the same rules: `-` takes one part, as a note would.

| Rest | Written in 4/4 |
|---|---|
| a whole bar, in any meter | `\| - \|` |
| eight bars | `\| - x8 \|` |
| half | `- -` |
| quarter | `-` as a beat |
| eighth | `-c` or `c-` |
| sixteenth | `-ccc`, `c-cc` |
| dotted eighth, then a sixteenth | `-__c` or `-.c` |

**Accents.** An upper-case symbol is accented: `cC cc` accents the "and" of
beat one.

**Bar lines.** `|` is a bar line, as in a score. A line can hold several
bars: `| cc cc | co cc |`. A line with no bar line is one bar.

**Groups.** Square brackets make a group that shares one part, and spaces
are allowed inside them: `c[cc]` is an eighth and two sixteenths. The `.`
shorthand stands for `__`, so `c.c` is `c__c`.

## 3. Compound and additive meters, tuplets

The beat is not always a quarter note. These are the meters where it changes
size or groups another way, and the tuplets that cross beats.

**Compound meters.** In 6/8 the beat is a dotted quarter, so `ccc ccc` is one
bar of eighths, counted in two, as a musician counts it.

```aratu
meter: 6/8
tempo: 70
hh = ccc ccc
bd = b__ b__
```

**Tuplets across beats.** A number before the brackets says how many beats
the group takes: `2[c4 d e]` is three notes in two beats (quarter-note
triplets) and `3[c c c c]` four in three.

**Additive meters.** `meter: 7/8 = 2+2+3` says how the eighths group. A bar
is written one word per group (`cc cc ccc`) or one per eighth
(`c c c c c c c`), and the tempo counts eighths.

```aratu
meter: 7/8 = 2+2+3
tempo: 240
hh = cc cc ccc
bd = b  b  b
```

**Meter changes.** `at bar 17: meter 7/8`.

## 4. Drums

Each drum has a line, and each line follows the rhythm rules. The symbol
decides the sound, whatever the line is called:

| Symbol | Sound |
|---|---|
| `c` | closed hi-hat |
| `o` | open hi-hat |
| `b` | bass drum |
| `s` | snare |
| `r` | ride |
| `x` | crash |
| `t` | high tom |
| `m` | mid tom |
| `f` | floor tom |
| `k` | side stick (the stick across the snare's rim) |
| `p` | hand clap |

```aratu
drums {
  hh = cc cc co cc
  bd = b - bb -
  sd = - s - S        // an accented backbeat on four
}
```

Every line inside `drums { }` is a drum line, and so is a line called `hh`,
`bd`, `sd`, `cr`, `rd`, `ht`, `mt`, `ft`, `toms`, `rim`, `kick`, `snare`,
`hat`, `ride` or `crash` anywhere in the piece, and by their Portuguese and
Spanish names too: `chimbal`, `bumbo`, `caixa`, `prato`, `palmas`, `charles`,
`bombo`, `caja`, `platillo`; the block may also be called `bateria`. A line
called `surdo` is the samba surdo (see the percussion below); the kit's
floor tom is `f`. An apostrophe makes a hit short: `c'` is a tight hi-hat. A
closed hi-hat cuts an open one that is still ringing.

**Rolls.** A tilde after the symbol makes a roll, like the tremolo strokes in
a score: thirty-second strokes (eight a beat) through the whole length of
the hit, the hands alternating. `s~ _ _ S` is a three-beat roll closing on
an accent; under a hairpin it grows into the next mark:

```aratu
drums {
  sd = p:s~< _ _ f:S
}
```

It works on any drum (`f~` on the floor tom, `x~` on a cymbal) and on
notes, as a tremolo: `c4~ _`. A backtick before the symbol puts a light
stroke of the same drum just before: `` `s `` is a flam and ``` ``s ``` a
drag (see chapter 9).

## 5. Brazilian percussion

A line named after an instrument reads its strokes, with the same rhythm
rules:

| Line | Strokes |
|---|---|
| `pandeiro` | `p` thumb, `d` fingers, `t` slap, `m` muted |
| `zabumba` | `o` open, `m` muted, `b` bacalhau (the stick) |
| `triangle` (`triângulo`) | `o` open, `f` closed |
| `agogo` (`agogô`) | `a` high, `g` low |
| `surdo` | `o` open, `m` muted |
| `tamborim`, `alfaia`, `gongue` | `o` open, `m` muted |
| `shaker` (`ganzá`) | `s` |
| `cuica` (`cuíca`) | `a` high, `g` low |
| `cajon` (`cajón`) | `b` bass, `t` slap |

```aratu
tempo: 120
pandeiro = pd td pd td
zabumba  = | o - -o m | o - b - |
triangle = fo fo fo fo
```

Upper case accents and `(p)` is a ghost stroke, as on the drums. For now
these sounds are drawn in the browser; recordings will come with
Manguebyte's engine.

## 6. Melodies

Notes are written in lower case, with the octave: `c4` is middle C. Sharps
and flats follow the letter: `f#4`, `eb4`. Solfège works too: `dó4 ré4 mi4`
or `do4 re4 mi4`. Upper case accents a note: `C4`. An apostrophe makes it
short, staccato: `c4'`.

```aratu
melody = | c4 e4 g4 _ | a4 g4 e4 c4 |
```

The octave carries over until it changes, so `| c4 e g c5 |` is C4, E4, G4,
C5. A note written before any octave is taken as octave 4, and Aratu says
so.

Two or more notes in one beat go in square brackets: `[d e]` is two eighths,
`[b b]` two B eighths, `[eb g]` E-flat then G, `[c4 _ _ d]` a dotted eighth
and a sixteenth. Rests and holds can sit next to a note without brackets
(`-e4` is a rest, then an eighth), but two notes cannot: `eb` reads as
E-flat, so for two notes Aratu asks for the brackets.

Notes sounding together are joined with `+`: `c4+e4+g4` is a C major chord
on one beat.

**Long lines.** A line that starts with `|` goes on with the line of music
above it, so a long melody can take several lines:

```aratu
melody = | c4 d e f | g a b c5 |
         | b4 a g f | e d c _ |
```

## 7. Chords and strumming

A progression is written as a lead sheet, on a line called `chords`. The
rule of sharing works one level up: chords that share a bar divide it
equally.

```aratu
chords = | Am | F | C G | Am _ _ G |
```

C and G take half a bar each; Am lasts three beats and G one. The shares
must fall on the beats: three chords in a 4/4 bar would split it between
beats, so Aratu asks how long each lasts.

Chord symbols keep their usual spelling: `Am`, `F`, `G7`, `Cmaj7`, `Dm7b5`,
`E/G#`, and the spellings of a Brazilian *cifra* too: `C7M`, `Bº`, `G7(b9)`.
`C6/9`, `E7alt` and solfège roots work as well: `Lám`, `Sol7`, `Fa#m7`,
`Dó/Mi`. `N.C.` (or `-`) is a bar with no chord.

**Strumming.** A line called `strum` uses the rhythm rules with guitar
symbols:

| Symbol | Meaning |
|---|---|
| `d` | strum down |
| `u` | strum up |
| `m` | muted strum (a chuck) |
| `g` | a golpe, a tap on the guitar's top (flamenco), no strings |
| `t` | a down strum and a golpe together |
| `-` | the hand passes without striking |
| `_` | let it ring |
| upper case | accented strum |

```aratu
guitar {
  chords = | Am | F | C G | G |
  strum  = D du -u du
}
```

The strum repeats every bar and the chords change on their own. A down
strum sounds the strings from low to high a few milliseconds apart, an up
strum from high to low; accents strum harder and faster. Without a strum,
the chords play whole, like a keyboard.

## 8. Harmony by degree

**Chords by degree.** In Roman numerals, from the key at the start of the
piece (C major when there is no `key:`): upper case is major, lower case
minor, and the quality follows. `bVII` borrows from outside the key and
`V7/V` is the dominant of the dominant. With `:arp` the chord is rolled,
from its lowest note up; in a melody, `c4+e4+g4:arp`.

```aratu
key: C major
chords = | I | vi | ii7 | V7:arp |
```

| Write | Means |
|---|---|
| `I` … `VII` | the degree, a major chord |
| `i` … `vii` | the degree, a minor chord |
| `V7`, `ii7`, `IVmaj7`, `vii°` | after the degree, the rest of the chord symbol |
| `bVII`, `#iv°` | a chord from outside the key, a semitone below or above the degree |
| `V7/V`, `V7/ii` | the dominant of the degree after the slash |
| `I:arp` | the chord rolled |

In a minor key the degrees follow the natural minor scale: in
`key: A minor`, `i` is Am, `iv` is Dm, `VI` is F, and the dominant with
the leading tone is written `V7` (E7).

```aratu
key: A minor
chords = | i | iv | VI | V7 |
```

## 9. Dynamics and articulation

Dynamic marks are the score's: `ppp pp p mp mf f ff fff`, and `sfz`, `fp`.

**Inside a bar**, a mark sits on the note it starts with a colon, with or
without a space (`p:c4` or `p: c4`), and does not count as a beat. Before brackets, it starts
with the group's first note: `p:[c4< g4]`. In `f:f4` the first `f` is forte
and the second is the note:

```aratu
melody = | p:c4 e4 g4 c5 | f:g4 _ _ _ |
```

**Hairpins** are written as in a score: `<` crescendo, `>` diminuendo, from
the beat after them to the next mark.

```aratu
melody = | p:c4< d4 e4 f4 | f:g4> f4 e4 d4 | p:c4 _ _ _ |
```

**For a whole part**, one line changes its level:

```text
piano: mf
piano: cresc to ff over 4 bars
drums: dim to p over 2 bars
all: fade out over 8 bars
```

A level line takes the whole part to the dynamic it names:
`melody: cresc to ff over 4 bars` starts from what is written and reaches
ff in the fourth bar, even when the line is written in p. Accents stay
above the other notes, and `fade out` takes the part to silence.

Each mark stands for a fixed velocity: ppp 16, pp 33, p 49, mp 64, mf 80,
f 96, ff 112, fff 127. An accent adds to the current one. A hairpin with no
mark after it moves one step (from `mf` to `f`, say) by the end of the line.

**Marcato and ghost notes.** `s!` is marcato, stronger than upper case.
`(s)` is a ghost note, barely touched, as drum notation writes it; it works
on notes too: `(c4)`.

**Ornaments.** They go after the note, with a colon. `c5:tr` is a trill:
the note and the one above in the key, every eighth of a beat, starting
and ending on the note; `:tr#` and `:trb` raise or lower the one above.
`c5:mord` is a mordent: the note, the one above and the note again, an
eighth of a beat each (less on a short note); `c5:mordi` uses the one
below. `:trill`, `:trinado`, `:trino`, `:mordent` and `:mordente` work
too. The neighbour follows the key of `key:` and its changes (C major when
there is none).

**Grace notes, flams and drags.** A backtick before a note makes it a
grace note: it takes no beat and sounds an instant before the next one
(`` `d5 c5 ``). There may be several (`` `b4 `c5 d5 ``), glued or not, and
from one bar into the next. On a drum, the backtick puts a lighter stroke
of the same drum just before: `` `s `` is a flam, ``` ``s ``` a drag.

```aratu
key: G major
melody = | g4 b4 `c5 d5 _ | d5:mord c5 b4 a4 | a4:tr _ _ _ | g4 _ _ _ |
sd = - `S - ``S
```

## 10. Tempo and time

```aratu
tempo: 96
at bar 3: rit to 70 over 2 bars
at bar 5: a tempo
hh = c c c c
```

| Write | Means |
|---|---|
| `tempo: 96` | the first tempo |
| `at bar 30: tempo 130` | a new tempo at a bar |
| `rit to 80 over 2 bars` | ritardando (`rall` works too) |
| `accel to 140 over 4 bars` | accelerando |
| `a tempo` | back to the tempo before the rit |
| `tempo primo` | back to the first tempo |
| `tempo 120 -> 140 over 4 bars` | a ramp between two tempos |

**When a change happens.** A change with `at bar N:` happens at that bar. A
change written on its own happens where the piece starts or, while you play,
at the next bar line after you write it.

During a ritardando or accelerando the tempo changes evenly, beat by beat,
as a score reads, and every note lands at its exact time.

**Fermata.** `^` on a note holds it: `g4^` lasts twice its value, and every
part waits for it. With a number it lasts that many times: `g4^3`
lasts three times as long.

**Pickup.** `pickup: beat 4` starts the piece on the fourth beat of an
incomplete first bar. The pickup is bar 0, so bar 1 is the first full bar,
as musicians number them. A line whose first bar is as long as the pickup
starts in it; the others start at bar 1.

## 11. Feel: rubato, swing and humanize

**Rubato** means robbed time, and robbed time is given back.
`piano: rubato 10%` stretches and shrinks the beats inside each bar and
arrives at the bar line on time, so the piano breathes while the drums keep
the pulse. `all: rubato 10%` bends the whole piece together.

**Swing.** `swing 60%` delays every second eighth (50% is straight, 66% a
triplet feel).

```aratu
swing 62%
hh = cc cc cc cc
bd = b - b -
```

**Humanize.** `humanize` adds small random deviations of timing and
velocity, as a player would; `humanize 5 ms` sets how much.

## 12. Repeats and jumps

The repeat and jump signs of a score, and repeating a beat.

### Repeat signs

```text
chords = |: Am | F |1 C | G :|2 C | C |      // repeat, first and second endings
chords = |: Am | F :| x4                      // the passage four times
chords = | Am | % | % | F |                   // % repeats the bar before
chords = | Am | F | %% |                      // %% repeats the two before
```

`|:` and `:|` mark a passage played twice, unless `x` says how many times.
`|1` and `|2` start the first and second endings.

### Jumps

Markers go at the start of a bar, jumps at its end:

```aratu
melody = | c4 e g c5 | segno: d4 f a _ | e4 g b _ | fine: c4 _ _ _ | g4 _ _ _ D.S. al fine |
```

| Marker | Jump |
|---|---|
| `segno:` | `D.S.`, `D.S. al fine`, `D.S. al coda` |
| `fine:` | where `al fine` stops |
| `coda:` | where `to coda` goes |
| | `D.C.`, `D.C. al fine`: back to the start |

The piece is unrolled as a musician reads it. A jump without its marker is
an error stated plainly: *"there is a D.S. at bar 24 of `melody` but no
segno for it"*.

### Repeating a beat

An `x` glued to the bracket repeats that beat, each time in its own beat:
`| [e4 d c d]x4 |` is the group four times. The `x` is glued to the `]` (or
to a motif); with a space, `| - x8 |` is still eight bars of rest. `4x`
works too: `[e4 d c d]4x`. Braces are for chance (`{e4|g4}`); a group in
braces with no `|` plays as a group, with a hint to use brackets.

## 13. Patterns, sections and form

Material is prepared in advance and given a name:

```aratu
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
at bar 4: play fill
at bar 5: play groove
```

### Form

Sections are named parts of a piece, and a form line orders them:

```aratu
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
long as its longest line. The form plays the sections in order (`x2` plays
one twice) and then starts again. A block can sit on one line, with `;`
between its lines.

### Varying a loop

```aratu
pattern groove { hh = cc cc co cc; bd = b - b -; sd = - s - s }
pattern fill   { hh = cc cc cc cc; sd = ss ss SS SS }
pattern crash  { cr = x - - - }

play groove x3, fill
every 8 bars, first: add crash
```

`play` chooses the pattern that sounds; `play groove x3, fill` plays a
phrase of patterns in turn and loops it. `every 4 bars: play fill` plays
the fill in the fourth bar of every four, in place of the groove; `add`
plays a pattern on top of the one that is sounding.

A `play` stays until another one: `at bar 8: play fill` plays the fill
from bar 8 on. `once` plays it through one time and goes back:

```text
at bar 8: play fill once     // the fill in bar 8, the groove again in bar 9
now: play fill once          // a fill now, then the groove where it would be
now: add crash once          // a crash on top, once
```

The pattern that was playing keeps counting its bars meanwhile, so
`play groove x3, fill` does not lose its place in the phrase. With
`every`, a pattern already plays once each time round.

## 14. Motifs, rows, lists and transformations

Short named ideas used many times: motifs, twelve-tone rows, melody lists,
and the transformations that change them without rewriting them.

### Motifs

`motif name = …` names a few beats, without bar lines, and the name is used
in their place. A motif may be defined before or after where it is used, may
use other motifs, and while it plays the editor lights the notes in its
definition.

```aratu
key: C major
motif riff = [e4 d c d]
melody = | riff riff+1 riff.inv riff.retro |
```

| Write | The motif |
|---|---|
| `riff+2`, `riff-1` | two degrees up, one down, in the key (`key:`; C major when there is none) |
| `riff+3st` | three semitones up |
| `riff.inv` | inverted around its first note |
| `riff.retro` | its notes backwards, in the same rhythm |
| `riff.ri` | the retrograde inversion; it combines with `+n`: `riff.ri+2` |
| `riffx4` | four times |

A note outside the scale, moved by degrees, keeps its distance from the
scale note below it: C# one degree up is D#.

### Twelve-tone rows

`row name = …` keeps a row: numbers from 0 to 11 (0 is C) or note names,
with or without octaves. The row gives the pitches and a rhythm written with
`o` gives their places, before `from`:

```aratu
row serie = 0 11 3 4 8 7 9 5 6 1 2 10
melody = | o o [o o] o | o _ [o o] o | O o o o | from serie.RI3 octave 4
```

Each `o` takes the next note, and the row starts again when it runs out, as
in isorhythm (`O` is an accented `o`). The forms are `P`, `I`, `R` and
`RI`, each from 0 to 11 semitones up: `serie.P0` is the row as written, and
so is `serie` alone. `octave N` puts every note in octave N. After `from`
a motif works too (its notes, in order), or a vector written there:
`from [0 4 7]`. A twelve-note row that repeats a note gets a warning, and
the **Rows** tab, beside the messages in Play, shows each row's 12×12
matrix, with the forms' names to click.

### Melody lists

`list name { }` keeps melodies, one per line, with bar lines; they may use
motifs and rows.

```aratu
list temas {
  | e4 d c d | e e e _ |
  | d4 d d _ | e g g _ |
}
melody = temas.1
```

`temas.2` picks the second (counted from 1), `temas.all` plays them all in
order and `temas.random` draws a whole one each time round, always the same
order for the same `seed`. Changing the number while it plays changes the
melody at the next bar. On stage, buttons choose each list's melody, and
Alt+1 to Alt+9 choose it for the line where the cursor is.

Each bar of a line can be a melody from the list too:
`bass = | temas.1 | temas.3 | temas.2 |`, mixed with written bars if you
like, and `temas.all` in place of a bar puts them all, in order. A list that
names itself stops, with a message.

### Transformations

Short verbs change a part, a line or a pattern without rewriting it:

| Verb | Effect |
|---|---|
| `hh.rotate 1` | shifts the pattern by one part |
| `hh.density 50%` | keeps about half of the hits |
| `drums.velocity 0.8` | plays softer |
| `bass.transpose +12` | an octave up |
| `melody.invert` | turns the intervals upside down |
| `melody.retrograde` | plays it backwards |
| `melody.ri` | the inversion backwards |
| `melody.augment` | doubles the note values |
| `melody.diminish` | halves the note values |

The names are the ones motifs and rows take: `.inv` or `.invert`,
`.retro` or `.retrograde`, `.ri`. `transpose` counts semitones and may say
so with `3st`, as motifs do.

```aratu
melody = | c4 d e g | a g e d |
melody.transpose +7
```

## 15. Playing live

While a piece plays, you change it; you do not rewrite it.

- **Everything loops.** Each line plays over and over, each over its own
  length.
- **A change is one line.** `hh = cc cc oc cc` replaces the hi-hat and
  nothing else; the other parts keep playing.
- **Changes land on the beat.** An edit takes effect at the next bar line;
  `now:` makes it take effect at the next beat instead: `now: play fill
  once` brings a fill in on the next beat, and the groove comes back at the
  bar line where it would have been.
- **Prepare ahead.** `at bar 17: play fill` schedules a change;
  `at bar 17 beat 3:` brings it in on the third beat (`beat 2.5` falls
  between 2 and 3), and `from bar 17 to 24:` makes it hold for those bars
  only: at bar 25 what held before comes back. A line can change at a bar
  too: `at bar 9: melody = | … |` plays the new melody from bar 9, from its
  start. `play groove 3x` is the same as `x3`.
- **Lengths.** `over` takes `bars`, `beats`, `eighths` and `sixteenths` in
  `cresc`, `fade`, `rit`, `accel`, tempo ramps and parameters:
  `piano: cresc to ff over 6 beats`.
- **Out and in.** `guitar: out` silences a part from the next bar line and
  `guitar: in` brings it back; `all: out` stops everything. They work with
  `at bar` and `every` too.
- **Nothing restarts.** Parts keep their place in the bar when another part
  changes, and while the text has an error the last good version keeps
  playing.

**The stage.** To play in front of an audience, the **Stage** button in Play
(or Ctrl+Shift+F) leaves only the code on the screen, full screen and in
large type for a projector. The frame flashes on the first beat of each
bar, the symbols light up as they sound and the first message shows in the
corner. The buttons in the corner play and stop, change the type size (A−
and A+), show the rhythm map below and leave the stage; they fade when the
mouse rests. Esc leaves too. The **2 columns** button splits the code in two
columns, like a newspaper: the second goes on where the first ends, the two
scroll together, and editing in either changes the same text.

## 16. Chance

A loop that repeats exactly the same soon sounds mechanical. Chance gives it
life: the computer decides, a little at random, what to play each time
round. It is the *ad lib* of a score.

**Sometimes.** A `?` after a symbol plays it only some of the time, half of
the time by default; a percentage sets how often:

```aratu
hh = cc cc c?30% cc
sd = - s - s?80%
```

**One or the other.** Braces choose one of several, separated by `|`, each
time the loop comes round:

```aratu
bass = | {c2|g1} _ c2 _ |
hh   = {cc|co} cc cc cc
```

**The same chance again.** The choices are drawn again every time round, but
from a fixed seed, so the same piece improvises the same way each time it is
played from the start. `seed 7` picks another improvisation; `seed random`
makes every performance different. Changing one line does not change how
the others improvise.

## 17. Sound and effects

Each part has a sound and a chain of effects, written as signal flow:

```aratu
bass: synth.saw -> lowpass -> reverb
bass = | c2 _ c2 eb2 | f2 _ g2 bb1 |
bass {
  reverb.mix = 0.42
  lowpass.cutoff = 800 Hz
  lowpass.resonance = 0.3
}
```

Every number that sets a parameter is also a control: the editor draws a
knob beside it, shaped by the parameter (a cutoff knob reads in hertz on a
logarithmic scale, a mix knob from 0 to 1). Turning the knob rewrites the
number, so saving the text saves the performance. Hold Shift for finer
steps. A number written in another unit keeps it: `1.2 kHz` turns in
kilohertz, `40%` in percent.

The effects are Manguebyte's, and go in the chain in any order
(`bass: synth.saw -> eq -> chorus -> delay`); the engine always runs them
in the same order, from the filter to the delay:

| Effect | Parameters |
|---|---|
| the part | `volume` (0 to 1), `pan` (-1 left to 1 right), `reverb.mix` (how much goes to the mix's reverb, 0 to 1) |
| `highpass`, `lowpass` | `cutoff` (20 to 20000 Hz), `resonance` (0 to 1) |
| `eq` | `low_gain`, `mid_gain`, `high_gain` (-24 to 24 dB), `low_freq`, `mid_freq`, `high_freq`, `mid_q` |
| `drive` | `amount` (0 to 36 dB), `output`, `mix`, `shape` (0 smooth, 1 soft, 2 hard, 3 folded) |
| `bitcrush` | `bits` (1 to 16), `rate` (Hz), `mix` |
| `compressor` | `threshold`, `ratio`, `attack`, `release`, `knee`, `makeup`, `mix` |
| `chorus`, `flanger`, `phaser` | `mix`, `rate`, `depth`, and more: `voices`, `feedback`, `stages`… |
| `tremolo`, `autopan` | `rate`, `depth`, `shape` (0 sine, 1 triangle, 2 square, 3 and 4 saw), `sync` (in beats) |
| `delay` | `time` (in s or ms), `feedback`, `mix`, `damping`, `pingpong`, `sync` (in beats) |

**Sounds.** Each sound has a code, written at the head of the chain:
`bass: bass.slap -> reverb`, `drums: drums.808`, `piano: piano.grand`. The
**Sounds** panel, beside the messages in Play, lists them all by family
(drum kits, keyboards, mallets and bells, plucked strings, basses, winds,
synthesizers), plays a bar of each and puts the code in the text with a
click. A code that does not exist is reported, with the closest one: *"did
you mean `bass.slap`?"*. Without a code, drums, guitar and chords pick
their own.

Some sounds play the accent their own way: in `bass.slap` a plain note is
the thumb and an upper-case one is a *pop*, the string pulled
(`| e2 E3 e2 E3 |`); in `bass.acid` upper case opens the filter, like the
TB-303's accent.

The sounds marked **recorded** (pianos, harpsichord, pipe organ, harp,
marimba, vibraphone, xylophone, glockenspiel, kalimba, tubular bells, tenor
sax, harmonica, recorder and the acoustic kit) are recordings from the
Versilian Community Sample Library, in the public domain (CC0), fetched the
first time a piece uses them. While they arrive, a similar synthesized sound
plays instead.

Alesson's recorded kit (`drums.alesson`) was recorded by Alesson Queiroz
and is in the public domain too (CC0). Its kick, snare and crash are
recordings, with several takes of each that take turns; on the snare, a
ghost note `(s)` and the softest dynamics play a light stroke, and accents
a harder one. The other drums (hi-hats, ride, toms) are synthesized.

**Manguebyte's engine.** Every sound in the panel plays through
Manguebyte's engine, the ecosystem's audio engine, running in the browser:
the recorded ones in a sampler; guitars and basses (`guitar.nylon`,
`guitar.steel`, `bass.finger`, `bass.slap`) as modelled strings; the
electric piano and the bell (`keys.epiano`, `keys.bell`) in FM; the organ
(`keys.organ`) through its rotating speaker, whose speed is `organ.rotary`
(0 stopped, 1 slow, 2 fast); the 808 (`drums.808`), the 808 bass
(`bass.808`), the acid (`bass.acid`) and the synthesizers (`synth.*`). Each
part has its effects in the engine, and the mix goes through a shared
reverb, a glue compressor and a limiter. The Brazilian percussion and the
golpe still play in a stand-in until Manguebyte has them; if something goes
wrong in a performance, the switch in the Sounds panel turns Manguebyte off
and everything plays in the stand-in.

Parameters can change by bar like anything else: `at bar 16:
bass.reverb.mix = 0.65`.

## 18. Numbers that move

**Parameters that move.** `bass.lowpass.cutoff = 300 Hz -> 4000 Hz over 4
bars` takes the cutoff from 300 to 4000 Hz over four bars and leaves it
there, like a crescendo of the sound.

| Write | What it does |
|---|---|
| `A -> B over 4 bars` | from A to B, once, and stays at B |
| `A <-> B over 4 bars` | there and back, 4 bars each way, again and again |
| `A -> B over 4 bars, back over 2 beats` | up, then back over another length, once |
| `A <-> B over 4 bars, back over 1 bar` | the same, again and again |
| `A -> B over 1 bar, loop` | up, then from A again, as a ramp |

The length is in `bars`, `beats`, `eighths` or `sixteenths`; in a
compound meter such as 6/8, a beat is the dotted quarter, three eighths. The
way follows the parameter's scale: frequencies and times move by equal
ratios, as the knob turns, and mix, volume and pan in a straight line. Aratu
marks a point every sixteenth of a beat and the sound glides between them.

With no time written, the line starts moving where it is written: at the
start of the piece, or at the next bar if the music is playing. `at bar 9:`
makes it start at bar 9; `every 8 bars:` makes it end at the end of every
eight bars, like a fill, and `every 8 bars, first:` start at the first. A
later change of the same parameter (`at bar 17: bass.lowpass.cutoff =
800 Hz`) takes over. Changing the numbers while the music plays does not
start the motion over: it goes on from where it is, along the new way. Both
numbers get a knob, and while it plays a second needle shows where the
parameter is; a new version of the text does not send it back to the
written value.

## 19. Mixing

Each part has `volume` (0 to 1) and `pan` (-1, hard left, to 1, hard right),
like the faders and pan knobs of a mixing desk: `drums.volume = 0.5`,
`guitar.pan = -0.6`. Each part's `reverb.mix` says how much of it goes to the
mix's shared room.

A part that only exists inside sections or patterns is mixed the same way,
from outside them: the number holds wherever it plays, and so does a change
`at bar` or a number that moves.

```aratu
section groove { alfaia = O -o o_oo o; snare = Ssss Ssss Ssss Ssss }
form: groove x4
alfaia.volume = 1
at bar 3: alfaia.volume = 0.6
```

**For the whole piece.** A number written with no part is for all of
them: `reverb.mix = 0.5` puts every part in the mix's reverb. A chain
written for `all` puts its effects on every part, after each part's own.
And the mix's own bus, the master, has numbers of its own:

```aratu
all: chorus -> delay
reverb.mix = 0.5
master.reverb.decay = 3 s
master.glue.on = 1
master.limiter.ceiling = -1
```

| Master | What it is |
|---|---|
| `master.reverb.decay`, `.size`, `.damping`, `.level` | the shared reverb: length (s), size, damping, level (dB) |
| `master.glue.on`, `.threshold`, `.ratio`, `.makeup` | the glue compressor: 1 turns it on; threshold (dB), ratio, make-up (dB) |
| `master.gain` | the level into the limiter, in dB |
| `master.limiter.ceiling` | the limiter's ceiling, -12 to -0.3 dB |

## 20. Quick reference

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
| `p:`, `mf:`, `f:` | dynamic mark on a note |
| `<`, `>` | crescendo, diminuendo |
| `cresc to ff over 4 bars` | a dynamic change for a whole part |
| `rit`, `accel`, `a tempo`, `tempo primo` | tempo changes |
| `tempo 120 -> 140 over 4 bars` | a tempo ramp |
| `g4^` | fermata |
| `pickup: beat 4` | pickup |
| `rubato 10%`, `swing 60%`, `humanize` | feel |
| `pattern name { }`, `play name`, `add name` | named patterns, in turn or on top |
| `\|: :\|`, `:\| x4`, `\|1 \|2` | repeat, count, endings |
| `%`, `%%` | repeat the bar before, the two before |
| `[e4 d]x4`, `[e4 d]4x` | a beat repeated four times |
| `motif riff = …`, `riff+2`, `riff+3st` | a named motif; two degrees up; three semitones |
| `riff.inv`, `riff.retro`, `riff.ri` | inversion, retrograde, retrograde inversion |
| `row serie = 0 11 3 …`, `\| o o o o \| from serie.RI3` | a twelve-tone row; a rhythm taking a form's notes |
| `list temas { }`, `temas.2`, `temas.all`, `temas.random` | a melody list; one by number, all, at random |
| `\| temas.1 \| temas.3 \|` | a melody from the list in each bar |
| `segno:`, `coda:`, `fine:` | markers |
| `D.C.`, `D.S. al fine`, `D.S. al coda`, `to coda` | jumps |
| `section name { }`, `form: intro verse x2 chorus` | sections and the form |
| `every 4 bars: play fill` | a variation inside a loop |
| `play fill once`, `add crash once` | a pattern once, then back |
| `at bar 30:` | a scheduled change |
| `now:` | a change at the next beat |
| `at bar 30 beat 3:` | a change on a beat of the bar |
| `from bar 9 to 16:` | for those bars only; then what held comes back |
| `at bar 9: melody = …` | the line changes from bar 9 |
| `over 6 beats`, `over 8 eighths` | lengths in beats, eighths, sixteenths |
| `guitar: out`, `guitar: in` | silence a part, bring it back |
| `->` | signal flow |
| `part.param = value` | a parameter, drawn as a knob |
| `part.lowpass.cutoff = 300 Hz -> 4000 Hz over 4 bars` | a parameter that moves, once |
| `<->`, `back over 2 beats`, `loop` | there and back; another length back; again |
| `2[c d e]` | three notes in two beats |
| `meter: 7/8 = 2+2+3` | an additive meter |
| `s!`, `(s)`, `g4^3` | marcato, ghost note, a fermata three times as long |
| `c5:tr`, `c5:mord`, `` `d5 c5 ``, `` `s `` | trill, mordent, grace note, flam |
| a line that starts with `\|` | goes on with the line of music above |
| `\| I \| vi \| V7/V \|`, `Am:arp` | chords by degree; a rolled chord |
| `pandeiro = pd td pd td` | Brazilian percussion |
| `reverb.mix = 0.5` (no part) | for every part |
| `all: chorus -> delay` | effects on every part |
| `master.reverb.decay = 3 s` | the mix: reverb, glue, limiter |

## 21. In English, Portuguese and Spanish

Every Aratu keyword can be written in English, Portuguese or Spanish, with
or without accents, in upper or lower case. The three versions of a piece
play exactly the same.

| English | Portuguese | Spanish |
|---|---|---|
| `meter:` | `compasso:`, `fórmula:` | `compás:` |
| `tempo:` | `andamento:`, `pulso:` | `pulso:` |
| `key:` | `tom:`, `tonalidade:` | `tonalidad:`, `tono:` |
| `pickup: beat 4` | `anacruse: tempo 4` | `anacrusa: tiempo 4` |
| `seed` | `semente` | `semilla` |
| `form:` | `forma:` | `forma:` |
| `at bar 9:` | `no compasso 9:` | `en el compás 9:` |
| `at bar 9 beat 3:` | `no compasso 9 tempo 3:` | `en el compás 9, tiempo 3:` |
| `from bar 9 to 16:` | `do compasso 9 ao 16:` | `desde el compás 9 hasta el 16:` |
| `every 4 bars, first:` | `a cada 4 compassos, primeiro:` | `cada 4 compases, primero:` |
| `now:` | `agora:`, `já:` | `ahora:`, `ya:` |
| `play`, `add`, `once` | `toque`, `adicione`, `uma vez` | `toca`, `añade`, `una vez` |
| `pattern`, `section` | `padrão`, `seção` | `patrón`, `sección` |
| `motif`, `row`, `list` | `motivo`, `série`, `lista` | `motivo`, `serie`, `lista` |
| `temas.all`, `temas.random` | `temas.todas`, `temas.acaso` | `temas.todos`, `temas.azar` |
| `from serie octave 3` | `com serie oitava 3` | `desde serie octava 3` |
| `over 4 bars` | `em 4 compassos` | `en 4 compases` |
| `beats`, `eighths`, `sixteenths` | `tempos`, `colcheias`, `semicolcheias` | `tiempos`, `corcheas`, `semicorcheas` |
| `cresc to ff` | `cresc até ff` | `cresc hasta ff` |
| `back over 2 beats`, `loop` | `volta em 2 tempos`, `repete` | `vuelta en 2 tiempos`, `repite` |
| `all:`, `out`, `in` | `todos:`, `fora`, `dentro` | `todos:`, `fuera`, `dentro` |
| `chords`, `strum` | `acordes`, `batida` | `acordes`, `rasgueo` |
| `.transpose`, `.invert` | `.transpor`, `.inverter` | `.transponer`, `.invertir` |
| `drums` | `bateria` | `batería` |

The names of dynamics (`mf`, `cresc`), tempo changes (`rit`, `a tempo`) and
signs (`D.S. al coda`) are Italian, as in a score, and work in all three
languages. Sounds and effects (`piano.grand`, `lowpass.cutoff`) have one
name.
