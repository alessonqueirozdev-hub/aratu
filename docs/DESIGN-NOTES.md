# Design notes on the syntax

*October 2026. A record of a design discussion and the recommendations that
came out of it. Nothing here is final.*

## Where the discussion started

A first sketch wrapped the piece in program structure:

```
fn void main (time_signature, tempo, key_signature) {
  const time_signature: 4/4
  let mut tempo: 120
  let mut key_signature: C

  fn Drum_kit (ostinato) {
    ostinato.measure(32) {
      hh.sub::8 = c c c o c c c o      // c = closed, o = open
      bd.div::4 = b b b b
      sd.div::4 = ~ ~ s ~
    }
  }
}
```

with tempo changes by measure (`In measure(30) -> tempo = 130`), an anacrusis
declared at the start (`Meter.anacrusis(4/4::4)`), and effects opened with
`Init Bass.reverb ->`, after which the editor draws a knob next to an editable
number.

## Principles agreed on

1. **Change the music, do not rewrite it.** A local change needs a local edit:
   changing the hi-hat is one line, and it does not restart anything else.
2. **Composition and performance are two uses of one language.** Blocks and
   sections organise a piece; short statements change it while it plays.
3. **Named patterns.** Material is prepared ahead (`pattern groove`,
   `pattern fill`) and chosen, combined and transformed live (`play fill`).
4. **Changes land on musical time.** `at bar 30:` schedules a change; an edit
   made while playing takes effect at the next bar line unless asked
   otherwise.
5. **Transformations with a musical meaning.** `rotate`, `density`,
   `velocity`, `transpose`, each predictable.
6. **Every parameter is reachable from code and from a control**, both bound to
   the same value in the engine, kept in step both ways, and changed without
   clicks (smoothing in the engine).
7. **Judge the language by the effort of changing a piece live**, not only by
   the code needed to write it: write a 64-bar performance twice, as a
   composition and as the shortest live version, and simplify the language if
   the live version needs much rewriting.

## Recommendations

**1. No program ceremony.** `fn`, `void`, `main`, `const`, `let mut` and `::`
are words for programmers. A musician reading `fn void main()` learns nothing
about the music, and the phrase is not valid in Rust or C. Aratu is
written *in* Rust; it does not have to look like Rust. The file is the piece,
and its first lines set the frame:

```
meter: 4/4
tempo: 120
key: C major
pickup: beat 4
```

Everything can change at a bar line, so there is no need to mark what is
mutable.

**2. One pattern line fills one bar.** The number of steps sets the
subdivision: eight steps in 4/4 are eighths, four are quarters, sixteen are
sixteenths. This keeps the compactness of TidalCycles and Strudel
mini-notation while fixing its timing problem, because the unit is now the
measure a musician counts, not an abstract cycle.

```
drums {
  hh = c c c o c c c o      // 8 steps: eighths
  bd = b - - b b - - -
  sd = - - s - - - s -
}
```

A line can span bars with a bar line (`| ... | ... |`), and the subdivision
can be stated where the count alone is not enough (`hh/16 = ...`).

**2b. Rhythms inside a beat (refines 2).** Musicians count beats and
subdivide each one ("1 e & a"). Three rules cover the common rhythms:

1. A space separates beats: a line in 4/4 has four groups, and the checker
   says so when it does not ("this bar has 5 beats, but the meter is 4/4").
2. Letters written together divide their beat into equal parts: `c` is a
   quarter, `cc` two eighths, `cccc` four sixteenths, `ccc` an eighth-note
   triplet with no special syntax.
3. `_` holds the previous note for one more part; `-` is a rest.

| Rhythm in one beat | Written | Why |
|---|---|---|
| four sixteenths | `cccc` | four equal parts |
| sixteenth, sixteenth, eighth | `ccc_` | four parts, the last note lasts two |
| sixteenth, eighth, sixteenth | `cc_c` | the middle note lasts two parts |
| dotted eighth, sixteenth | `c__c`, or `c.c` as a shorthand | the first note lasts three parts of four |
| eighth-note hi-hat for a bar | `cc cc co cc` | four beats of two eighths |

In 6/8 the beat is a dotted quarter, so `ccc ccc` is a bar as a musician
counts it. `|` stays the bar line, as in a score, rather than a separator
inside a beat. This replaces "the number of steps sets the subdivision" from
recommendation 2: the beat, not the bar, is the unit that is divided.

The question was raised by a first proposal that used `|` inside a beat
(`cc|c` for sixteenth, sixteenth, eighth); the rules above give each of its
cases a single reading.

**2c. Accents.** An upper-case step is accented: `cC cc` accents the "and"
of the first beat. Pitched lines are written in lower case, as LilyPond does
(`c4 e4 g4`), so that `C4` can mean an accented C4. Chord symbols keep their
usual spelling (`Am`, `F`, `G7`).

**2d. Chords as a lead sheet.** A chord progression is written the way a
lead sheet (a *cifra*) writes it, and the beat rule applies one level up:
what shares a bar divides it equally.

```
chords = | Am | F | C G | G |      // C and G take half a bar each
chords = | Am _ _ F |              // Am for three beats, F for one
```

This replaces an earlier sketch, `chord(Am(1/2), F(1/2), C, G).measure(1)`,
where unmarked chords last the default length and marked ones a fraction of
a bar: the bar lines say the same with no parameters.

**2e. Strumming.** A strum pattern uses the beat rules with its own symbols:
`d` down, `u` up, `m` muted (a chuck), `g` a golpe on the top (added on 15
October 2026 for flamenco: the wood sounds, not the strings), `t` a down strum and a golpe together
(*tudo junto*, *todo junto*, *together* in the three languages), `-` the hand
passes without striking, `_` hold, and upper case for an accent.

```
guitar {
  chords = | Am | F | C G | G |
  strum  = D du -u du             // the common pop strum
}
```

The pattern repeats over every chord, so a live change of strum is one line
and the chords change on their own at the right bar. Under the hood a down
strum sounds the chord's strings low to high a few milliseconds apart and an
up strum high to low, the spread set by tempo and accent; this needs a table
of chord shapes on the fretboard (Am = x02210 and so on). The sound can come
from guitar samples made with Parabolic or from Manguebyte's plucked-string
model. An earlier sketch applied the strum with an arrow
(`guitar.chord(Am, F, C, G).measure(1) -> d d u d`); the arrow is kept for
signal flow (recommendation 5), so the strum is an assignment.

**2f. Dynamics and expression.** Score words, written two ways: inside a
bar for composing, as one short line for changing a piece live.

- Marks: `ppp pp p mp mf f ff fff`, plus `sfz` and `fp`. Inside a bar a mark
  is attached to the beat it starts with a colon, so it is not counted as a
  beat: `| p:c4 e4 g4 c5 |`. A space after the colon is allowed (`p: c4`),
  since every other colon in Aratu is followed by one.
- Hairpins as in a score: `<` crescendo and `>` diminuendo, from the beat
  they follow to the next mark: `| p:c4< d4 e4 f4 | f:g4 |`.
- Live: `piano: cresc to ff over 4 bars`, `drums: dim to p over 2 bars`,
  `all: fade out over 8 bars`.
- Each mark is a fixed velocity (ppp 16, pp 33, p 49, mp 64, mf 80, f 96,
  ff 112, fff 127, the convention notation programs use); an upper-case
  accent adds to it.

**2g. Tempo changes.** `rit to 90 over 2 bars` (`rall` too),
`accel to 140 over 4 bars`, `a tempo` back to the tempo before the change,
`tempo primo` back to the first tempo, a fermata as `^` on a note (`g4^`,
twice its length by default) or `fermata 3 beats`. For the engine, a tempo
ramp is integrated (the position of a beat under a linear ramp has a closed
form), so events stay on their exact sample during a rit.

**2h. Rubato is robbed time that is given back.** `piano: rubato 10%`
stretches and shrinks the beats inside the phrase and arrives at the bar
line on time, so one part breathes while the others keep the pulse;
`all: rubato` bends the whole piece. Separate from it: `humanize` (small
random deviations of timing and velocity) and `swing 60%`.

**2i. Repeats.** Two kinds, which work differently.

*Repeats in the form* use the score's signs, in the text form ABC notation
has long used: `|:` and `:|` around a passage (played twice, or `:| x4`),
`|1` and `|2` for first and second endings, `%` for a measure repeat (the
shape of the sign itself) and `%%` for two bars, and the jumps: markers
`segno:`, `coda:` and `fine:` at the start of a bar, jumps `D.C.`, `D.S.`,
`D.C. al fine`, `D.S. al coda` and `to coda` at the end of one. Sections
repeat in a form line: `form: intro verse x2 chorus verse chorus x2 outro`.
The engine unrolls all of this into one timeline, as a musician reads a
score with its jumps, and the editor shows the unrolled form with its
brackets. A jump with no marker is reported in a musician's words ("there is
a D.S. at bar 24 but no segno before it").

*Repeats while playing* need no sign, because patterns loop; what is needed
is a way to say when to vary: `every 4 bars: drums fill` (the fourth bar of
every four), `every 8 bars, first: crash`, `play groove x3, fill` (a
four-bar phrase that loops).

**3. Simple words for time.** `pickup: beat 4` instead of
`Meter.anacrusis(4/4::4)`; `at bar 30: tempo 130`; a ramp as
`tempo 120 -> 140 over 4 bars`.

**4. Numbers are the controls.** Instead of a separate `Init Bass.reverb ->`
statement, any number assigned to a parameter is drawn by the editor as a
control beside it, shaped by the parameter itself: range, unit, and scale
(a cutoff knob is logarithmic and reads in hertz, a mix knob reads 0 to 1).

```
bass {
  pattern = C2 - C2 G1
  reverb.mix = 0.42        // the editor draws a knob here
  lowpass.cutoff = 800 Hz  // and a logarithmic one here
}
```

Turning the knob rewrites the number in the text, so the code stays the
single source of truth: saving the file saves the performance. Inline controls
exist elsewhere (Strudel's sliders, Bret Victor's work on live programming);
what is new here is that the control comes from the engine's own parameter
description, through Manguebyte's parameters and widgets.

**5. The arrow is signal flow.** Musicians know signal chains from
pedalboards and mixers, so `->` reads as "into":

```
bass: synth.saw -> lowpass -> reverb
```

Writing `bass.reverb` on its own adds the effect with its defaults and shows
its panel.

**6. Transformations as short verbs.** `hh.rotate 1`, `hh.density 50%`,
`drums.velocity 0.8`, `bass.transpose +12`, usable on a part, a pattern or a
whole section, each with an exact musical definition in the documentation.

## Notes on the first interface mock-up

The layout is right: code on the left, the panel the code produces on the
right (instruments, effects with knobs, patterns, quick transformations) and
the timeline at the bottom. Details to fix in the next version: the line
numbers repeat (two 23s, two 33s); `main` opens a brace that never closes;
the status bar names the language "Rust", which tells a user they are writing
Rust. The knobs on the panel and in the code would be one and the same
control.

## The name

**Decided (15 October 2026): Aratu.** Candidates tied to manguebeat and
maracatu, checked free as crate names on crates.io and unused by any
live-coding project on GitHub (15 October 2026):

| Name | What it is | Why it fits |
|---|---|---|
| **Baque** | the beat of maracatu (*baque virado*, *baque solto*) | it means "the beat": a rhythm language called Baque, short and easy in any language |
| **Gonguê** | maracatu's iron bell, which keeps time for everyone | that is the language's job; the accent becomes `gongue` in code and URLs |
| **Alfaia** | maracatu's bass drum | strong and recognisable |
| **Aratu** (chosen) | a mangrove crab | echoes Manguebyte's crab and the mangue itself |
| **Frevo** | Recife's rhythm | Recife in general rather than manguebeat |

A trademark search (INPI) is needed before a name is final.

## Still to represent

In order of how often musicians need them:

1. **Articulations**: staccato, legato (slurs), tenuto, marcato; ghost notes
   in parentheses as in drum notation (`(s)`); flams, drags and rolls;
   glissando, slides and bends; vibrato and trills; grace notes; arpeggiated
   (rolled) chords.
2. **Harmony by function**: scale degrees and Roman numerals (`I vi IV V`);
   modulation (`at bar 17: key D major`); voicings (close, open, drop 2) and
   tensions (`G7b9`); arpeggio patterns assigned like a strum
   (`arp = up 8ths`).
3. **Meter**: additive meters (7/8 as 2+2+3, essential for frevo and much
   music of the Northeast of Brazil); tuplets that span beats (a half-note
   triplet), which the beat rules do not cover yet; polyrhythm (3 against 2)
   and polymeter (parts in different meters); metric modulation.
4. **Brazilian feel**: `swing 60%` is jazz swing. The pushed sixteenths of
   samba, maracatu and frevo are different; `feel samba`, `feel maracatu`,
   `feel frevo` would be a cultural strength no live-coding language has.
5. **More voices and instrument techniques**: two voices in one part, the
   piano's sustain pedal, breath marks and caesura, 8va.
6. **Chance**, which live coders expect: "maybe" (`s?`) and "one of"
   (`{a|b}`), a musician's *ad lib*.

**Open:** how staccato is written: a mark after the note (`c4'`, close to a
score) or a word (`stacc c4 e4`, easier to remember).

## Proposal for what is still missing

One principle: **what is used often gets one character; what is rare gets a
word.** Each kind of line (drums, strum, pedal) keeps its own symbols, as the
strum already does with `d u m`.

**Articulations, frequent ones, one character after the note:**

| Score | Aratu | Why |
|---|---|---|
| staccato | `c4'` | the apostrophe recalls the dot |
| marcato | `c4!` | stronger than an upper-case accent |
| tenuto | `c4=` | the score's dash |
| slur (legato) | `c4~d4~e4` | the tilde joins the notes |
| ghost note (drums) | `(s)` | exactly as in drum notation |
| grace note, flam, drag | `` `d4 c4 ``, `` `s ``, ``` ``s ``` | the backtick is the small note before |
| roll, tremolo | `s*` | |
| fermata | `g4^` | already decided |

**Rare techniques, a word after a colon** (the colon already attaches marks):
`c4:trill`, `g4:vib`, `c4:gliss g4`, `e4:bend+2`, `Am:arp` (rolled chord).

**Harmony by function.** Roman numerals in a progression,
`chords = | I | vi | IV | V |`, relative to `key:`, with secondary dominants
(`V7/V`) and borrowed chords (`bVII`). Arpeggios assigned like a strum, with
chord-tone numbers (1 root, 3 third, 5 fifth, 7 seventh, 8 octave) and the
beat rules: `arp = 13 58 13 58`. Modulation: `at bar 17: key D major`.
Voicing: `piano.voicing = drop2`.

**Meter.** Additive meters `meter: 7/8 = 2+2+3`, written `cc cc ccc`.
Tuplets across beats: a number before brackets gives the beats they span,
`2[c d e]` (a half-note triplet). Polyrhythm follows from the same rule:
`bell = 4[c c c]` is three notes over four beats against drums in four.
Polymeter: `bell { meter: 3/4 }` inside a 4/4 piece. Metric modulation:
`at bar 9: tempo q = q.`.

**Brazilian feel.** `feel samba`, `feel maracatu`, `feel frevo`,
`feel baiao`, beside `feel swing 60%`; each feel is a map of micro-timing
offsets by position in the beat, and users can define their own.

**Voices and techniques.** `piano.right` and `piano.left`, as a pianist
thinks; `voice 1`, `voice 2` for other instruments. Pedal with the score's
signs: `pedal = | P _ _ * |`. Breath `,`, caesura `//`, octave shift as a
prefix: `8va: c4 e g`.

**Chance.** `s?` sometimes (half the time), `s?30%`, `{a|b}` one of two.

**Decided (15 October 2026):** staccato is `c4'`.

**Decided (15 October 2026):** see "Provisional decisions" below for the
choices made while implementing.

**Decided (15 October 2026):** chance uses the symbols (`?`, `{a|b}`); the
manual explains it in its own chapter. Background:
since chance is used constantly when playing live and `?` reads as
"maybe?") or musicians' words (`maybe c`, `one of c2 g1`). Chance means the
computer decides, a little at random, what to play each time round, so a
loop varies by itself the way a drummer never plays a bar quite the same:
`hh = cc cc c? cc` plays the third-beat eighth only some of the time,
`c?30%` three times in ten, and `bass = | {c2|g1} _ c2 _ |` starts each
repeat on either C or G.


## Provisional decisions (15 October 2026)

Implementing the language needed answers the manual left open. The user
approved these as provisional; each can be reopened, and the manual marks
them *Provisional*.

1. **Two notes in one beat need brackets.** `[d e]`, never `de`, because
   `eb` reads as E-flat. Rests and holds may sit next to a note (`-e4`,
   `c4_`). The checker answers `de` with "put them in brackets: `[d e]`".
2. **Everything loops over its own length.** A one-bar hi-hat, a two-bar bass
   and a four-bar progression loop independently, as a band's parts line up.
   With a pickup, the pickup is bar 0 and plays once; bar 1 is the first full
   bar. A line whose first bar is as long as the pickup starts in it; the
   other lines start at bar 1.
3. **A change written without `at bar` happens where the piece starts, or,
   while playing live, at the next bar line after it is written.** (First
   proposed as "only with `at bar` in a file"; changed because the editor
   evaluates the whole text on every edit, so a bare line has to mean "from
   when it appeared". The editor remembers when each such line first
   appeared.)
4. **A tempo ramp is linear in score beats**, as notation programs do it, so
   the position of every beat has a closed form (a logarithm) and notes land
   on their exact sample.
5. **`play` chooses, `add` layers.** `every 4 bars: play fill` replaces the
   current pattern in the fourth bar of every four; `every 8 bars, first: add
   crash` plays a pattern on top. This replaces `every 4 bars: drums fill` and
   `every 8 bars, first: crash`, which named a part and a sound where every
   other statement names a pattern.
6. **Chords sharing a bar must fall on the beats.** `| C G |` in 4/4 is two
   beats each; three chords in 4/4 are an error that asks for holds,
   `| Am _ F G |`.
7. **The drum symbol decides the sound**, whatever the line is called: c
   closed hi-hat, o open hi-hat, b bass drum, s snare, r ride, x crash, t high
   tom, m mid tom, f floor tom, k side stick (added on 15 October 2026, at the
   user's request: the kit was missing its toms and rim). Lines
   inside `drums { }` and lines with drum names (`hh`, `bd`, `sd`, `cr`, `rd`,
   `kick`, `snare`, `hat`, `ride`, `crash` and a few more) are drum lines.
8. **A fermata is `^`, twice the value, and every part waits for it.** How to
   give it another length (`fermata 3 beats`) stays open.
9. **`seed 7` and `seed: 7` both work.**

Smaller choices made along the way:

- **Voices by code** (15 October 2026): `bass: bass.slap -> reverb`. The
  catalogue lives in `crates/aratu/src/voices.rs`; the checker warns about an
  unknown code and names the closest one, and the editor's Sounds panel lists
  them all. Sampled voices are VCSL recordings (CC0) prepared by
  `web/scripts/samples.mjs`, with an SFZ file each for Manguebyte's sampler.
  The browser sounds stand in until Manguebyte's engine plays the same codes;
  the user prefers every voice and effect to come from Manguebyte.
- **The contract with Manguebyte's web engine** (agreed on 16 October 2026,
  Manguebyte aa9a07c, `crates/manguebyte-web/README.md`, ABI version 2): one
  32-byte record per event; byte 9 holds flags, bit 0 the accent (Aratu's
  `acc`). Drum keys follow General MIDI: b 36, s 38, k 37, c 42, o 46, x 49,
  r 51, t 50, m 47, f 43, and the flamenco golpe 88, the first key past every
  GM and GM2 percussion key. The kit's `voice.sfz` uses the same keys.
- `~` after a symbol is a roll (a tremolo on a note): strokes of an eighth of
  a beat through the hit's length, the hands alternating, growing into the
  next mark under a hairpin (15 October 2026, at the user's request).
- `.` inside a beat stands for `__`, so `c.c` is `c__c` and `[c4. d]` is
  `[c4 _ _ d]`.
- Inside brackets, glued symbols and spaces are the same level: `[-c4 d4]`
  has three parts. Brackets nest: `c[cc]`.
- A note before any octave is taken as octave 4, with a warning.
- A live level (`piano: ff`, `cresc to ff over 4 bars`, `fade out`) takes
  the part from its written dynamics to the mark it names, accents staying
  above. (Until 15 October 2026 it scaled the written marks against mf, so a
  part written in p and taken to ff ended near mp; the user heard no
  crescendo.)
- A hairpin with no later mark moves one step (mf to f) by the end of the
  line, with a warning.
- `section` holds parts and lines like the top of the piece and lasts as long
  as its longest line; the form loops. A block may sit on one line with `;`
  between statements: `pattern crash { cr = x - - - }`.
- Brazilian *cifra* spellings are read: `C7M`, `Bº`, `G7(b9)`, `Dm7(9)`.
- Keywords are English for now. **Open:** Portuguese keywords (`compasso:`,
  `andamento:`) beside the English ones.

## Long rests and silencing a part

**Decided (15 October 2026), chosen by the user from three proposals each:**

- `| - |`, a rest alone between bar lines, is a whole bar of rest in any
  meter, like the score's whole rest; `| - x8 |` is eight bars of rest, the
  multi-measure rest. The `x` already means "times" in `:| x4`, `verse x2`
  and `play groove x3`. (Also proposed: `| -8 |`, which reads as "minus
  eight", and a word, `| rest 8 |`.)
- `guitar: out` silences a part from the next bar line and `guitar: in`
  brings it back, as on a mixer; `all: out` silences everything. Both work
  with `at bar` and `every`. (Also proposed: the score's `tacet` and
  `tutti`, and no new words at all.)

## Playing a pattern once

**Decided (15 October 2026), chosen by the user from three proposals:**

- `play fill once` plays the pattern (or the phrase, `play fill x2, break
  once`) through one time, then the `play` before it goes on where it would
  have been: it kept counting its bars meanwhile, so a four-bar phrase does
  not lose its place. `add crash once` puts a pattern on top for its own
  length. It works with `now:` and `at bar`; with `every` a pattern already
  plays once each time round. Without `once`, `play` stays until another
  `play`, with or without `now:`. (Also proposed: leaving `play` as it was,
  so a single fill needs `now: play fill, groove x3` or deleting the line,
  which moves the phrase; and making every `now:` line a one-time gesture,
  which leaves no way to change the groove now and keep it.)
- The word is English like the other keywords; `uma vez` comes with the
  Portuguese keywords.

## What building it showed (15 October 2026)

Notes from implementing the language crate and the web editor, for
decisions still to make.

- **Bare changes need memory in the editor.** Because the editor compiles
  the whole text on every edit, a line such as `play fill` or
  `piano: cresc to ff over 4 bars` cannot mean "now" on its own: it would
  start again on every keystroke. The editor remembers, for each such line,
  the bar it first sounded from, and the scheduler takes those bars as
  input. Editing the line makes it a new line that starts at the next bar.
  The lines already written when playing starts keep bar 1, so the first
  edit does not restart them.
- **`now:` splits a bar between two versions of the piece.** The editor
  schedules whole bars a little ahead. When a new version brings a `now:`
  line it has not seen, the editor finds the next whole beat it has already
  scheduled, takes back what the old version scheduled from that beat on
  (the sounds and the parameter changes), and renders the rest of that bar
  from the new version, from that beat (`Options::from_beat`). For the
  scheduler the line is still anchored to a bar: the bar the beat falls in,
  so a `now: play fill` written during the second beat of bar 5 counts its
  loop from bar 5, and the fill's first beats, which were never heard, are
  left out. Parameters set at that bar line are set at the beat instead.
  Other new lines in the same edit, without `now:`, still wait for the next
  bar line; everything else edited with it comes in at the beat. If the next
  beat is the next bar line, the swap happens there, as for any edit.
- **A knob changes the sound at once and the text with it**, so the knob
  does not wait for the bar line: it sets the parameter on the engine and
  rewrites the number, and the next version of the piece agrees with it. A
  number set at a bar, and the tempo, only change the text.
- **Drum and line names.** With the symbol deciding the drum's sound, the
  line name (`hh`, `bd`) only names the line, so two lines can play the
  same drum. The list of names read as drum lines outside `drums { }` is in
  `crates/aratu/src/parse.rs`.
- **Chance never shifts with edits elsewhere.** Each line draws from its own
  stream, seeded by the piece's seed, the line's name, the bar and the
  symbol, so editing the bass does not change how the hi-hat improvises.
- **Open:** keywords in Portuguese; the full drum table; a fermata of a
  given length; tuplets across beats and additive meters (proposed above);
  how rubato chooses its phrase (today it bends inside each bar).

## Proposed: synthesizers, and a drawing for each effect (15 October 2026)

**Open, asked for by the user; the syntax is to be confirmed with them before
it is built.** In their words: the drums are there, but how do you make
synthesizers in Aratu? And besides the knobs, an equalizer should appear when
you call `equalizer`, and each effect called should be drawn in its own way.

**1. An electronic-music kit, and synthesizers the musician defines.** A
starting point that follows the rules already decided (numbers become knobs,
`->` is the signal flow):

```
synth lead {
  osc    = saw x3 detune 12 cents      // three detuned saws (unison)
  filter = lowpass 1200 Hz, res 0.3, env 0.6
  amp    = adsr 5 ms, 200 ms, 0.7, 300 ms
  lfo    = sine 5 Hz -> pitch 10 cents
}
lead  = | c4 e g c5 |
bass: acid                              // presets: acid (303-like), sub, reese, pad, pluck, stab, supersaw
drums: kit 808                          // kits: 808, 909, and our own
lead -> delay 3/16 -> reverb
```

The effects Manguebyte's engine has or is finishing: filters (low-pass,
high-pass, band-pass, a Moog-style ladder), delay, chorus, flanger, phaser,
distortion and saturation, compressor, gate, expander, limiter, reverb,
convolution, and sidechain ducking (the kick pushing the pad down, the pumping
of electronic music).

**2. A drawing for each effect, besides the knobs.** Writing `equalizer` (or
`eq`) draws the equalizer's curve with bands that can be dragged, and dragging
rewrites the numbers in the text, as the knobs do. Each effect has its own
drawing: a filter its response curve, an ADSR envelope its shape, an LFO its
wave, a compressor its transfer curve with the gain reduction live, a delay
its repeats in time, a reverb its decay, and level and spectrum meters where
they help.

From Manguebyte's side, the web engine will publish, with the parameters in
JSON, a description of each effect with a drawing kind (eq and its bands,
filter, adsr, lfo, compressor, delay, reverb) and what is needed to draw the
right curve (for example the frequency response computed by the engine
itself, so the drawing matches the sound). This comes with the sound-design
step, right after the web engine; Manguebyte will say when the format is
ready. Until then the editor can draw the curves from its own formulas and
switch later.

## The rhythm map (15 October 2026)

At the user's request the rhythm map is quantized and editable. Each bar is
drawn as a grid of equal steps, as fine as its rhythm needs (eighths,
sixteenths, triplets, up to 48 steps a beat), with thin stripes for the
steps and stronger ones for the beats; a choice of grid (auto, eighths,
sixteenths, triplets) refines it. Dragging a symbol moves it, clicking an
empty place adds one, clicking a symbol removes it, and the bar is written
back with the beat rules: each beat in as few parts as it needs, two notes
in a beat in brackets, a drum hit followed by a rest written as the hit
alone (a drum does not sustain). Notes moved get explicit octaves, so the
octave carried from the note before cannot change their pitch.

Not editable from the map: bars with a choice `{a|b}`, chord shares, bars
copied by `%` and bars written out from `- x8`. **Open:** dragging a note
up and down to change its pitch; moving a note over a bar line (today it is
cut at the bar's end); keeping the octave of the bar after an edited one when
it was carried over.

## Repetition, motifs, transformations, rows and lists (16 October 2026)

Agreed with the user and built as described below, with two changes:
`temas.next` became `temas.all` (the melodies in order) and `temas.random`
(a whole melody drawn each time round from the seed); on stage, buttons and
Alt+1…9 change a list reference in the text, and the side panel's Rows tab
shows each row's 12×12 matrix. Code in `crates/aratu/src/motifs.rs`, tests in
`crates/aratu/tests/motifs.rs`, lesson `book/*/learn/14-motifs.md`.

1. **Repeat a beat inside a bar**: `| [mi3 ré3 dó3 ré3]x4 |` is the group
   four times, each taking its own beat. `x` glued to `]` (or `}`, or to a
   motif name), never spaced: spaced `| - x8 |` already means eight bars of
   rest, and in drum lines `x` is the crash. Overflowing the bar gives the
   usual beat-count error.
2. **Motifs**: `motif riff = [mi3 ré3 dó3 ré3]` (beats, no bar lines; for
   several bars use a list). Used as a beat word: `| riff riff+1 riff+2
   riff+3 |`. `+n`/`-n` moves it n degrees in the key (`key:`, C major
   when none), a note outside the scale keeping its offset; `+nst` moves
   semitones. `riffx4` repeats it. A motif may use another, with a depth
   limit and an error on a loop. Spans point at the definition, so the
   editor lights the motif while it plays. Definitions may come anywhere:
   a pre-pass over the layout collects motifs, rows, lists and the first
   key before lines are parsed.
3. **Transformations on a motif**: `riff.inv` (chromatic inversion around
   its first note), `riff.retro` (its pitches in reverse order, same
   rhythm, as serial practice treats rows), `riff.ri` (retrograde
   inversion); they combine with transposition: `riff.ri+2`.
4. **Twelve-tone rows as vectors**: `row serie = 0 11 3 4 8 7 9 5 6 1 2 10`
   (pitch classes, or note names with or without octaves). Forms `P n`,
   `I n`, `R n`, `RI n`, relative: `P0` is the row as written, `Pn` n
   semitones up. A row gives pitches; a rhythm gives their places:
   `melodia = | o o [o o] o | o _ [o o] o | from serie.P0 octave 4`, each
   `o` taking the next pitch (cycling, as in isorhythm). `from [0 11 3 …]`
   takes an inline vector. A twelve-note row that repeats a pitch class
   gets a warning naming it. Later: the 12×12 matrix in the editor.
5. **Melody lists**: `list temas { … }`, one melody (with bar lines) per
   line; `melodia = temas.2` picks one by its number, counted from 1;
   `temas.all` plays them in order. Changing the index while playing
   comes in at the next bar, like any edit. Later: `temas.next` and
   `temas.random` with `every`, and keys 1–9 choosing the index in stage
   mode.

Implementation notes: a new module (context with motifs, rows, lists and
key; degree shift with the major or natural-minor scale; row forms;
placeholder `o` filled after parsing) threaded into `parse_music`, whose
current signature stays as a wrapper. New messages in pt/es/en: unknown
row, list index out of range, motif loop, motif with bar lines, row
repeating a pitch class. Tests, a manual section and a lesson in the three
languages.

## Manguebyte's engine plays samples and drum kits (16 October 2026)

At the user's request, part by part: Manguebyte's web engine (wave 5b,
commit 29000fd, built by `web/scripts/manguebyte.mjs` into
`web/public/manguebyte/`) plays the sampled voices (sampler parts loading
our `voice.sfz`), the acoustic kit (a sampler kit, whose missing keys such
as the golpe's 88 play Manguebyte's synthesized drums) and the analogue kit
`drums.kit` (its synthesized kit). Every other voice stays in the Web Audio
stand-in until Manguebyte has it. Each such part gets its own engine (one
AudioWorkletNode), whose output goes into the part's Web Audio chain, so
high-pass, low-pass, delay, reverb sends, volume and pan still apply while
Manguebyte's own effects (wave 5c) are not in. Events go as note on (with
the accent flag) and note off; a mid-bar swap sends Manguebyte's cancel,
and stop sends cancel and all notes off. Levels were measured in the
browser against the stand-in's. A switch in the Sounds panel turns
Manguebyte off (kept in the browser), and a browser that cannot run it
falls back to the stand-in by itself.

## Parameters that move (16 October 2026)

The user asked for effects to change gradually, the way a crescendo does,
over bars, beats or smaller lengths, going up and coming back. The writing
follows the tempo ramp already in the language (`tempo 120 -> 140 over 4
bars`):

- `bass.lowpass.cutoff = 300 Hz -> 4000 Hz over 4 bars` goes once and stays.
- `<->` in place of the arrow goes there and back, again and again.
- `back over 2 beats` gives the way back another length; `loop` starts
  again at the end (with no way back, a ramp).
- Lengths are `bars`, `beats`, `eighths` and `sixteenths`, counted in the
  meter where the motion starts; in 6/8 a beat is three eighths.

The motion follows the parameter's scale, the one its knob turns on:
frequencies and times by equal ratios, mixes and levels in a straight line.

A moving parameter is a command even without a time, so it is anchored
like any live line: from the start of the piece, or from the bar where it
was written while playing. Its live key is only the path it moves
(`bass.lowpass.cutoff = ->`), so turning one of its knobs, which rewrites a
number, changes the way without starting the motion over. `at bar 9:`
starts it there; `every 8 bars:` places it so it ends at the end of each
stretch, like a fill, and `every 8 bars, first:` starts it at the first bar
of each. A later change of the same parameter at a bar takes over.

The scheduler marks a point every sixteenth of a beat and one at each turn,
with the seconds since the point before (`glide`); the web engine moves the
sound towards each point over that time (`setTargetAtTime` with a third of
it as the time constant), which needs no earlier event to start from and
survives a mid-bar swap. A program lists its moving parameters, and while it
plays a new version of the text leaves them where they are instead of
setting them back to the written value. Both numbers get knobs; a second
needle on them shows where the parameter is now.

## Syntax review (16 October 2026)

A pass over the whole lexicon and grammar, compiling small pieces to see
what Aratu accepts. Three kinds of findings.

**Accepted but wrong, or told badly (fix first):**

- `at bar 3: melodia = | … |` is read as a second definition from bar 1
  (only the "appears twice" warning). Either a line may change at a bar, or
  this must be an error.
- Drum lines and parts by their Portuguese and Spanish names are not drums:
  `caixa = - s - s` is a note error, and worse, `bumbo = b - b -` and
  `chimbal = c c c c` play B4 and C4 with only an octave warning. `bateria`
  is not a drum part.
- `cp`, `clap` and `perc` are drum line names, but there are no symbols for
  them: `cp = c - c -` plays the closed hat.
- Misleading messages: an unknown header (`andamento: 120`, `tempp: 120`)
  says a name is missing after `->`; `riff.invert` says `riff` is not a
  note; `every 2 beats` mentions `over 4 bars`; typos of keywords (`ply`,
  `patern`) and parameters get no "did you mean", which voices already have.

**The same idea written differently in different places:**

- Inversion and retrograde: `.inv .retro .ri` on motifs, `.invert
  .retrograde` on lines, `I R RI` on rows; each rejects the others.
- `[…]4x` is accepted, `play groove 3x` and `form: a 3x` are not.
- Parameter motions take bars, beats, eighths and sixteenths; `cresc`,
  `fade`, `rit`, `accel` and tempo ramps take bars only.
- Times are only `at bar N`: no beat (`at bar 9 beat 3`), no ranges
  (`from bar 9 to 16`), no `every 2 beats`.
- Notes take solfège, chord symbols do not (`Dó`, `Rém`, `Sol7`); `N.C.`,
  `C6/9` and `alt` chords are unknown (a rest `-` stands for N.C.).
- Keywords are English only, while notes, keys and messages take
  Portuguese and Spanish. Open: accept `compasso:`, `andamento:`, `tom:` and
  the like as synonyms, or keep English and only explain better.

**Not yet writable (most already proposed above, in "Proposal for what is
still missing"):** continuing a long line on the next line; tuplets across
beats (`2[c d e]`); marcato `!` (tried by the user), tenuto, slurs (the
proposed `~` is now the roll); ghost notes `(s)`, flams, clap, cowbell and
pedal hat; Brazilian percussion lines with their own strokes (pandeiro,
zabumba, triangle, alfaia, gonguê, agogô, surdo, tamborim); a fermata's
length; additive meters and polymeter; Roman numerals and arpeggio lines;
`feel samba`, `feel maracatu`, `feel frevo`; techniques by word
(`:trill`, `:gliss`, `:bend`, `Am:arp`); hands, pedal and 8va; a choice
between fragments longer than a beat.

**To call the syntax finished:** a formal grammar (EBNF) as the single
reference, tests that cover each rule, and the editor's highlighting taken
from the same word lists.

**Done the same day, the first two groups of the review:** Portuguese and
Spanish drum names and `bateria`; `p` for a hand clap (played by the
stand-in until Manguebyte has one); "did you mean" for headers said in
Portuguese or Spanish or misspelt, keywords, parameters, motif forms and
list picks, and a part told nothing it understands; `at bar 9: melody = …`
changes the line from bar 9 from its own start, and `from bar 9 to 16:`
for those bars only; one set of names for inversion and retrograde on
motifs, rows and lines (`inv`/`invert`/`I`, `retro`/`retrograde`/`R`,
`ri`/`RI`), with `ri` and `transpose 3st` on lines; `3x` in `play` and
`form`; beats, eighths, sixteenths and fractions of bars for every `over`;
solfège chord roots, `N.C.`, `6/9` and `alt`; `at bar 9 beat 3:` for any
command but meter and key (the bar is laid out in stretches cut at such
beats, and a sudden tempo change is a step at its beat), and
`from bar 9 to 16:` for any command but tempo, meter and key, after which
what held before comes back (a parameter goes back to an earlier value
written at a bar, or to the value its part is set up with). Keywords stay
English; the open question of Portuguese synonyms is still open.

## Keywords in three languages, and the third group of the review (16 October 2026)

The user asked for English, Portuguese and Spanish all to be accepted.
`crates/aratu/src/words.rs` holds, for each keyword, the Portuguese and
Spanish words that stand for it, folded (lower case, no accents); the
cursor's `keyword` and every place that compares words ask it. A word may
stand for different keywords in different places (`compasso:` the meter,
`no compasso 9` the bar, `tempo 3` after a bar number the beat). Short
words that are also notes (`do`, `de`, `a`) are only read where nothing
else could be meant; a row's `from` is `from`, `desde`, `com` or `con`,
since `de` and `do` are notes. The editor lights the longer words only.
A test plays one piece written in the three languages and compares every
event. A drum part answers to its name in any language.

Then the third group, the things that could not be written:

- A line starting with a bar line goes on with the line of music above
  (not inside a list, where each line is a melody); the two bar lines
  around the line break are one, and a comment between is left out.
- A beat may be longer than one: `2[c d e]` (a tuplet across two beats)
  and additive meters, `meter: 7/8 = 2+2+3`, where a bar is written one
  word per group or one per unit and the tempo counts units.
- `s!` marcato, `(s)` ghost notes, `g4^3` a fermata three times as long.
- Roman numerals in chord lines (`I vi IV V7`, `vii°`, `bVII`, `V7/V`),
  from the key at the start of the piece; `Am:arp` rolls a chord, as does
  `c4+e4+g4:arp`.
- Brazilian percussion lines named after the instrument, each with its
  strokes (pandeiro, zabumba, triângulo, agogô, surdo, tamborim, alfaia,
  gonguê, ganzá, cuíca, and the cajón); `surdo` is now the samba surdo,
  and the kit's floor tom stays `f`. The stand-in synthesizes them;
  VCSL has agogô, cajón, claps, triangles and shakers, and the user
  offered to record the rest for Manguebyte.

Still to do from the review: `feel samba`, `feel maracatu`, `feel frevo`
(they need measured micro-timing, not guesses), techniques by word
(`:trill`, `:gliss`, `:bend`, which need the engine), hands and pedal, and
a choice between fragments longer than a beat.

## Manguebyte plays almost everything

Manguebyte's web engine (commit 4bf2b50) grew from a sampler and an
analogue kit to ten instruments and a fourteen-stage effect chain per
part, with a master bus (shared reverb, glue compressor, true-peak
limiter). Aratu now runs one engine with a numbered part per Aratu part,
and maps every voice in the Sounds panel to one of those instruments:
guitars and fingered basses to the plucked string, the electric piano
and the bell to FM, the organ to the drawbar organ with its rotating
speaker, the 808 to the analogue kit and the kick bass, the acid bass to
the acid voice, every `synth.*` to the subtractive synth with its knobs.
Levels were measured voice by voice and calibrated with a gain per voice,
so that a part at volume 1 sits near the others. Only the Brazilian
percussion and the golpe stay in the stand-in, until there are samples.

The parameter table now holds Manguebyte's ids, ranges, units and
defaults, so a knob means the same in both. Volume becomes the part's
gain in dB, `reverb.mix` the send to the shared reverb. A part's chain
lists effects in any order; the engine runs them in its fixed order.

For the whole piece: a number with no part was already for every part;
a chain written for `all` (`todos`) now adds its effects to every part
after its own, and `master.*` reaches the master bus. Playing and the
previews wait (up to four seconds) for Manguebyte's parts to be ready,
so the first notes are never the stand-in's.

## The course grows with the language

The language had grown faster than the course: additive meters, tuplets,
Brazilian percussion, Roman numerals, articulation, motions, the timeline
words, global effects and the master were in the manual as rows and
paragraphs, and some (swing, rubato, sections and form, transformations)
in no lesson at all. A student meets the language through the course, so
the course was rebuilt around it, in 24 lessons that follow a music
curriculum: rhythm (bars, dividing, drums, rests, compound and additive
meters, Brazilian percussion), notes (melodies, rhythm in melodies and long
lines, chords, harmony by degree), expression (dynamics and articulation,
repeats, sections and form, motifs and transformations, tempo, swing and
rubato), playing live (patterns, the timeline, chance) and sound (voices
and effects, numbers that move, mixing), ending in a maracatu project
written with real percussion lines, sections, dynamics and a mix.

Exercises can now be checked for when the notes come (`timing=1`: tempo,
swing, rubato, fermatas) and for how they sound (`sound=1`: voices, effects,
numbers, numbers set while playing or moving), so the lessons on sound and
expression have exercises too. While doing this, the checker was found to
index a table of six drum names, so a missing tom, clap or percussion
stroke panicked; every sound now has a name.

The manual was reordered to follow the course, one chapter per subject
instead of a grab-bag chapter 9, and a test (`every_feature_is_taught_in_
every_language`) lists the features of the language and fails when one of
them appears in no lesson's code, in any of the three languages. A new
feature gets a line in that list, and a lesson.

## A drum kit recorded for Aratu (16 October 2026)

Alesson Queiroz recorded a snare, a crash and a whole groove; Manguebyte's
session cut them into 22 hits (round robins, and three velocity layers on
the snare), mapped them to General MIDI keys in an SFZ file and dedicated
them to the public domain (CC0 1.0). Aratu keeps the folder as it came in
`web/public/samples/drums.alesson/`, with its README translated into
English, as the voice `drums.alesson`: a sampler kit on Manguebyte, whose
keys without a sample (hi-hats, ride, toms) play Manguebyte's synthesized
drums. Its level matches the acoustic kit's at the same gain. The stand-in
plays one take each of kick, snare and crash and the analogue kit for the
rest. `web/scripts/samples.mjs` now starts over only the VCSL folders it
makes, keeps this one and credits it in `SOURCES.md`.

## Ornaments, grace notes, flams and drags (16 October 2026)

Proposed in this file's "Proposal for what is still missing" and approved
by the user: a technique is a word after a colon, like `Am:arp`, and the
small note before is a backtick.

- `c5:tr` (also `:trill`, `:trinado`, `:trino`) trills with the note above
  in the key, every eighth of a beat as a roll does, an odd number of
  strokes so it starts and ends on the note; `:tr#` and `:trb` raise or
  lower the note above, as the accidental over a score's tr.
- `c5:mord` (`:mordent`, `:mordente`) plays the note, the one above and the
  note held, an eighth of a beat each, or a third of a short note;
  `c5:mordi` the one below. The neighbour follows `key:` and its changes.
- A backtick before a note makes a grace note: no beat of its own, played
  60 ms before the note it leads into, softer. Several may come in a row,
  glued or not, and over a bar line. On a drum line the backticks count
  strokes of the same drum, 30 ms apart and softer: `` `s `` a flam,
  ``` ``s ``` a drag. Events carry `grace` (its place before its note) so
  exercises tell a flam from a single stroke.
- In motifs, grace notes move with transposition, degrees and inversion,
  and keep their distance to their note when the motif goes backwards.

Still to come: glissando, slide, bend and vibrato (`:gliss`, `:slide`,
`:bend+2`, `:vib`), which move a note's pitch while it sounds and need
Manguebyte's engine and its event record to carry that. The roll `~` stays
strokes of an eighth of a beat; the user's recording of a buzz roll
(strokes every 25 to 50 ms, with edit cuts) could not be cut into hits, and
a clean recording would let `~` play it as a loop.

## A sound plays only once it has been downloaded

Playing used to wait up to four seconds for Manguebyte's parts and then
start anyway, on the stand-in's synthesized voices; a sampled kit played
synthesized drums while its samples came; a voice changed while playing
was heard on a stand-in until it arrived. That let a piece be heard with
sounds that were not its own. Now:

- A part sounds only with a voice that has arrived. Playing (the editor,
  the book's examples, the Sounds panel) waits, with no time limit of its
  own, until every part has its voice, and a notice at the bottom of every
  view says which voices are downloading.
- A voice changed while the music plays is made in a Manguebyte part of
  its own; the voice before it goes on sounding until the new one is
  ready, then hands over. A new part waits in silence.
- The stand-in is only a fallback for a download that failed (an HTTP
  error, a network failure, or no answer in 60 seconds) or for a browser
  where Manguebyte cannot run. The notice then says which sound, why, and
  what plays instead: the stand-in's own copy of the samples, or a
  synthesized voice like it. Manguebyte switched off in the Sounds panel
  is a choice, not a failure, and is not reported.

## Stage gestures and ready-made grooves

Pattern languages make live coding quick with functions over patterns.
Aratu takes another road, the performer's: the words are the gestures of a
set. `breakdown` takes the drums, the percussion and the bass out; `build
over 8 bars` plays a snare roll that doubles its speed as it grows louder
while a highpass rises on every part; `drop` brings everything back on a
crash; `solo bass` leaves one part. They are commands like `out` and `in`,
so `now:`, `at bar` and `from bar … to …` work with them, and they are
taught in the live lesson with exercises the checker hears.

`groove house` (`levada`, `ritmo`) writes the lines of a ready-made beat,
read from the groove's own Aratu text, so a performer starts from a style
and changes it line by line: a line written with the same name, in the same
part, takes the groove's line's place, without the warning a repeated line
gets. The groove's sounds light the word that wrote them. The grooves are
the common shapes of each style in 4/4, Brazilian ones included (samba,
baião, maracatu, bossa, funk carioca).

In the editor, Ctrl+/ comments lines out, Alt+M writes or removes
`part: out` for the part of the cursor's line, and Alt+S writes or removes
`solo part`. The shortcuts write the command in the text instead of acting
on the sound directly, so the change lands at the next bar line like any
edit, and a shared link keeps it.

A build's highpass reaches parts whose chain names no highpass: the browser
engine switches Manguebyte's highpass on for a part the first time a
highpass value arrives for it, starting from the value the part is set up
with.
## Quick tools for playing live (17 October 2026)

Set beside Strudel and TidalCycles, Aratu won on notation and teaching and
lost on the small generators that make a pattern language fun to play:
Euclidean rhythms, an arpeggiator, sidechain, `fast` and `slow`, an echo.
Each now has a form that follows the rules already decided, so a musician
reads it as a score word and a live coder finds what they expect.

- **Euclidean rhythms, `b(3,8)`.** A symbol and, in parentheses, the hits
  and the steps, Bjorklund's spreading (Toussaint's Euclidean rhythms):
  `(3,8)` is the tresillo, `(5,8)` the cinquillo, `(7,12)` a West African
  bell. The beat rules decide its length, with no new rule: alone in its
  bar it fills the bar in any meter, as `| - |` does; beside other beats it
  is one beat divided in its steps; `2[b(3,8)]` spans two beats. A third
  number turns it later by that many steps, the way `rotate` turns a line,
  so the language has one direction of turning; Tidal's `(3,8,2)` turns the
  other way, starting two steps into the pattern, and gives another
  rotation of the same necklace. Marks after the symbol count for every hit
  (`S(2,4)`, `c(5,8)?`), and it works on drums, percussion strokes, strums
  and notes (`c2(3,8)`). Spaces inside the parentheses are allowed;
  `b(3,8)x2` repeats it like a bracket. The rhythm map does not redraw it,
  since its hits are written once.
- **Chance on a hit written once and played many times** (a Euclidean
  rhythm, but also `[c?]x4` and a motif used twice in a bar) was drawn once
  for all of them, so they all played or none did. Each time the same hit
  comes round in a bar now draws anew; the first draws exactly as before, so
  no piece written until now changes what it improvises.
- **The arpeggiator, `arp = up 16ths`.** A line like the strum, beside a
  `chords` line, so the chords change on their own and one word changes how
  they are played: `up`, `down`, `updown` (or `up and down`), `random`; the
  steps `8ths`, `16ths`, `32nds`, `triplets`, `beats` (sixteenths when not
  said), and `2 octaves`. The steps follow the meter of the bar they play
  in (a sixteenth in 6/8 is a sixth of a beat), so the line has no beats of
  its own to count. As under a strum, the chords then sound only through
  it. Each chord starts again from its lowest note, and the notes are the
  chord's from the third octave, where the keyboard chord sits; `.transpose`
  moves them. A line called `arp` is an arpeggiator only when its music
  starts with a way, so the melodies already called `arp` in the examples
  stay melodies. In Portuguese `arpejo = sobe e desce 8ths`, in Spanish
  `arpegio = sube y baja corcheas`. Still proposed: chord-tone numbers with
  the beat rules, `arp = 1535 1535`, for patterns an arpeggiator's ways do
  not give (an Alberti bass).
- **Ducking, `pad: duck by kick 70% over 1 beat`.** A part command, like
  `out` and `cresc`, so it takes `at bar`, `from bar … to …`, `every` and
  `now:`. The scheduler turns each hit of the source into volume points on
  the part: down by the amount at the hit, then back over the length along
  `1 - a(1 - t)²` (quick at first, landing softly), a point every sixteenth
  of a beat with `glide`, as moving parameters do, and the written volume
  again at the end; a new hit before the way back ends starts again from
  the bottom. The volume it comes back to is the part's as written (`volume
  =`, or set at a bar). The source is a part, a line, or a drum by its name
  in any language (`kick`, `bumbo`, `bombo`; `snare`, `clap`, `hat`) from
  whatever line plays it, since a producer means the kick, not the line
  called `bd`; `all: duck by kick` ducks every other part. The defaults,
  70% and a beat, are the pumping of house music. `sidechain` is the
  studio's word in all three languages; `abaixa com o bumbo` and `agacha
  con el bombo` read as Portuguese and Spanish. Manguebyte lists sidechain
  ducking among its effects; when it is in, the same line can drive it and
  the points can go.
- **`hh.fast 2`, `bass.slow 2`.** Verbs like `rotate`, on a line, a part or
  a pattern. Whole numbers only, so every bar of the line lands on a bar
  line; they multiply, `augment` being `slow 2` and `diminish` `fast 2`, so
  `fast 2` and `slow 2` cancel. The scheduler fits any speed: each bar plays
  the stretch of the line's bars it covers.
- **`lead.echo 1 eighth x3`.** Each sound again that much later, each echo
  at 60% of the one before, over bar lines; the length is written as after
  `over`, and `x3` counts the echoes as `play groove x3` counts the times.
  Tidal's `off` (a shifted copy, often transposed) is this with one echo;
  a transposed echo is left for later.

## Melodies by degree, and scenes

A number in a melody line is a degree of the piece's first key, as many
musicians sing a tune to learn it ("one, three, five"): `1` is the tonic
in octave 4 and `8` an octave higher, the accidental comes after the number
(`3b`, `4#`, where a note's `b3` would be the note B), and a comma goes an
octave down (`5,`, as in ABC notation; the apostrophe is staccato already).
The same tune moves to another key by changing `key:` alone, which is what
a singer's transposition needs.

Scenes are eight places in the editor that keep the whole text of the piece
and bring it back with a click or Ctrl+Shift+1 to 8. A scene is the text,
not a sound: brought back while playing, it lands at the next bar line like
any edit, and Ctrl+Z undoes it, so nothing new had to be taught to the
player. They are kept in the browser; sections and patterns remain the way
to write an arrangement into the piece itself.

## The page as a score

The editor's look comes from engraved music, so it reads as a place for
music rather than a generic web tool. The scenes are boxed like a score's
rehearsal marks. The bar count, the tempo and the stacked meter
(6 over 8) sit in a dark display in the transport, set in Gloock, a face
cut like an engraver's. A line crosses the display in the time of one bar,
so a change can be timed to the bar line without counting. The colours
stayed. The text and the code are set in Atkinson Hyperlegible, which the
Braille Institute drew so that letters easily confused (I, l and 1, O and
0) stay apart. That matters in a classroom and in code. The faces come from
Google Fonts and are not kept in the repository. Without them the page
falls back to the system's serif, sans and mono.

The stage is a full-screen sheet of code under a rail that always shows the
bar, the beats and the tempo, and the bar's line along its foot. The
controls on the rail fade while the mouse rests, and stay on a touch screen,
which has no resting mouse. A problem in the piece shows at the foot,
never over the line being written. On a phone the stage has one column and
its own letter size, and the button that leaves it stays in reach.

On a phone the tabs go to the foot of the screen, under the thumb. Long
lines wrap. The course's contents fold into a bar that names the page you
are on.
