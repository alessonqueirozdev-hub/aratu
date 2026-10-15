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
about the music, and the phrase is not valid in Rust or C. MusiMangue is
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
`d` down, `u` up, `m` muted (a chuck), `-` the hand passes without striking,
`_` hold, and upper case for an accent.

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
  beat: `| p:c4 e4 g4 c5 |`.
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
