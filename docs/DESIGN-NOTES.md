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
