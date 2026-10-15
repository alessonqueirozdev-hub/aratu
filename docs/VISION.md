# Aratu: vision

*Draft, October 2026. Everything here is open to change; nothing is built.*

## The problem

Live coding is one of the most expressive ways to make music with a
computer, and one of the hardest to enter for a trained musician.

The leading languages place events in time with a unit musicians do not use.
In TidalCycles and Strudel the unit is the **cycle**: `"c e g"` divides one
cycle into three equal parts, and `"c [e g]"` gives `c` half the cycle and
`e` and `g` a quarter each. This is elegant for repeating patterns and
polyrhythm, but a musician thinks "quarter, quarter, half, in 4/4", not
"three events per cycle". Sonic Pi asks for `sleep` in beats between notes,
which puts arithmetic where music should be. Writing a melody with a dotted
rhythm, a tie across a bar line or a change of meter is hard in all of them.

Languages that do read like music, such as Alda, LilyPond and ABC notation,
are for writing scores, not for changing music while it plays.

## The landscape

| Language | In common with Aratu | What it lacks for this purpose |
|---|---|---|
| TidalCycles, Strudel | compact pattern notation; Strudel has sliders in the code | time is the cycle, not the measure; no lead-sheet chords, score dynamics or rubato |
| ixi lang (Thor Magnusson) | patterns that look like a score, very simple | no longer developed, tied to SuperCollider, no harmony or expression |
| Mercury (Timo Hoogland) | designed to be readable and beginner friendly | still oriented to programmers rather than music theory |
| FoxDot, Renardo | durations in beats | Python, with Python's ceremony |
| Sonic Pi | excellent for teaching | time by `sleep`, arithmetic where music should be |
| Glicol | written in Rust, runs in a browser | oriented to the audio graph, not to musical phrases |
| Alda, LilyPond, ABC | music notation as text | made to write scores, not to play live |

No language combines the measure and the beat as units, subdivision by
symbols sharing a beat, lead-sheet chords, strum patterns, score dynamics and
rubato, controls bound to the engine's parameters, and running inside a DAW
in step with its transport, in Portuguese and English.

**Where the users are.** Today's live coders are invested in Tidal and
Strudel and many prefer cycles and generative patterns, so Aratu is
unlikely to win them over quickly. The larger audience is musicians who do
not live code yet: band musicians, music teachers and students, producers who
live in a DAW, and Brazilian music education in particular, where there is
almost nothing in Portuguese.

**What decides adoption**, judging by the tools that succeeded: running in a
browser with nothing to install (the main reason for Strudel's reach); good
sounds on the first `play`; short examples and videos of performances; and
talking to the existing tools (MIDI and OSC out to Strudel and Tidal), so
they are partners rather than rivals. The design must also be tested on
people early: five musicians who have never programmed, three exercises,
and the language simplified wherever they get stuck. The number of symbols
(`_ - . : < > ^ + [ ] |`) is the first thing to test.

## Who it is for

- Musicians who read music or think in measures and note values, and want to
  improvise or perform with code.
- Music teachers and students: rhythm, harmony and form become visible and
  audible at the same moment.
- Live coders who want melody and harmony to be as easy as patterns.
- Portuguese-speaking musicians, from the start.

## Principles

1. **The measure is the unit of time.** A part is a sequence of measures, a
   measure is filled with note values, and the language checks that each
   measure adds up to its time signature.
2. **Patterns where they help.** Cyclic patterns stay available for grooves,
   drums and variation, and convert to and from measures, so a pattern can be
   written out and a written line can be turned into a pattern.
3. **Errors in the musician's words.** "Measure 2 of the piano has 5 beats,
   but the time signature is 4/4", never a parser error.
4. **Change while it plays, on the beat.** An edit takes effect at the next
   bar, or another boundary the player chooses, never in the middle of a
   note.
5. **Inside the DAW.** Aratu also runs as a plugin following the host's
   tempo, meter and position, sending MIDI to other tracks or playing its
   own sounds.
6. **Readable by people who did not write it.** A piece is a score others can
   follow.

## A sketch of the language

```
measure 4/4, tempo 96, key C major

melody: | C4 q   E4 q   G4 h        | A3 q.  B3 e  C4 h~ | C4 w |
chords: | C h        Am h          | F h         G h    | C w  |
bass:   | C2 q C2 q  A1 q A1 q     | F1 h        G1 h   | C2 w |
drums:  every bar  "kick . snare . kick kick snare ."

melody plays with piano, chords with pad, bass with pluck
```

- Note values: `w` `h` `q` `e` `s` (whole to sixteenth), dots, ties (`~`)
  across bar lines, tuplets (`3:2 [C D E] q`).
- Pitches: letters with octave (`C4`, `F#3`, `Bb2`) or solfège (`dó4`,
  `fá#3`), chord symbols (`Am`, `G7`, `Fmaj7`), scale degrees in a key.
- Meter and tempo change anywhere: `measure 7/8`, `tempo 120 -> 140 over 4
  bars`.
- Patterns in the cycle style inside quotes, placed per bar or per beat.
- Transformations a musician recognises: transpose, invert, retrograde,
  augment, diminish, humanise, swing.

## How it would run on Manguebyte

| Aratu needs | Manguebyte provides |
|---|---|
| Sample-accurate scheduling of notes and changes | the timestamped event stream and transport |
| MIDI in and out, MIDI files | `manguebyte-midi`, `manguebyte-devices` |
| Sounds | `manguebyte-dsp` synthesis, samplers, and instruments made with Parabolic |
| A graph changed while it plays | `manguebyte-graph` |
| Running inside a DAW in step with the host | the CLAP and VST 3 wrappers |
| Talking to other live-coding tools and visuals | `manguebyte-osc` |

The language itself (parser, checker, scheduler with a tempo map and a meter
map) is new work in this repository.

**Where the sound comes from.** Manguebyte is a library, not a separate
program: Aratu depends on its crates, and they are compiled into
Aratu's executable, or into its WebAssembly module in a browser. All
real-time synthesis (oscillators, the polyphonic synth, filters, envelopes,
effects, the audio graph) comes from there.

Parabolic is not embedded. Its licence (PolyForm Noncommercial plus a
commercial licence) would otherwise reach Aratu, which stays MIT OR
Apache-2.0. Parabolic's instruments reach Aratu in two ways instead:

- **On a desktop, as plugins.** Manguebyte hosts CLAP and VST 3 plugins
  (`manguebyte-host`), so Aratu plays an instrument made with Parabolic
  the way it plays any plugin, from any vendor.
- **Anywhere, including a browser, as data.** A browser cannot run native
  plugins, so Parabolic exports SFZ (WAV files and a mapping), and an open
  SFZ player in Manguebyte plays them.

| Layer | Licence | Role |
|---|---|---|
| Manguebyte | MIT OR Apache-2.0 | audio engine, synthesis, SFZ sampler, plugin hosting |
| Aratu | MIT OR Apache-2.0 | language, scheduling, editor; its sound is Manguebyte |
| Parabolic | PolyForm Noncommercial + commercial | authoring: guided capture, harmonic modelling, panel design, plugin export |

## The first version

In: measures, time signatures, note values, ties, tuplets, tempo, parts
playing built-in sounds or sending MIDI, cyclic patterns for drums, edits that
take effect at the next bar, errors in plain words, English and Portuguese
note names, a standalone editor, and a plugin that follows the DAW's
transport.

Out, for later: a visual score view, recording a performance to a MIDI file,
collaborative sessions, a browser version (WebAssembly), microtonality.

## Open questions

- The exact syntax: how close to music notation, how close to code.
- Whether the editor is our own (with Manguebyte's interface crates) or an
  extension for existing code editors, or both.
- How patterns and measures convert into each other when they disagree.
- How Portuguese and English keywords coexist in one piece.

## Relationship to the other projects

- **Manguebyte** is the foundation; anything Aratu needs from it is built
  there first.
- **Parabolic** instruments can be Aratu's sounds.
- Both tools will be presented on Manguebyte's website.
