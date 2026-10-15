# Aratu

<p align="center">
  <img src="docs/images/aratu-mascot.png" width="320"
       alt="Aratu, an orange mangrove crab wearing headphones, coding music on a laptop">
</p>

**A live-coding language that reads like music.** Part of the
[Manguebyte](https://github.com/alessonqueirozdev-hub/manguebyte) ecosystem.

*Aratu is a crab of the mangrove, the mangue of Recife that gave
manguebeat, and Manguebyte, their names.*

Aratu is a live-coding language and environment for people who know
music: measures, time signatures, note values and tempo come first, so that
what you type is laid out in time the way a musician already thinks about
it. It is built on [Manguebyte](https://github.com/alessonqueirozdev-hub/manguebyte),
the open-source audio framework for Rust, and runs on its own or inside a
DAW, in step with the host's transport.

> **Status: idea.** Nothing here is built yet. This repository holds the
> vision so that it is not lost while Manguebyte reaches and then passes
> JUCE. See [docs/VISION.md](docs/VISION.md).

## A taste

The syntax below is a sketch, not a decision:

```
measure 4/4, tempo 96

piano: | C4 q   E4 q   G4 h   | A3 q.  B3 e  C4 h |
bass:  | C2 w                 | A1 w              |
drums: every bar  "kick . snare . kick kick snare ."
```

Notes have values (`w` whole, `h` half, `q` quarter, `e` eighth, dots and
tuplets), measures are checked, and patterns in the cycle style of
TidalCycles and Strudel are available where they fit, such as the drum line.
Note names in Portuguese (`dó ré mi`) work as well as letters.

## The Manguebyte ecosystem

Aratu is one of three projects that grow from the same mangue:

| Project | What it is | How it meets Aratu |
|---|---|---|
| [Manguebyte](https://github.com/alessonqueirozdev-hub/manguebyte) | the open-source audio framework for Rust (MIT OR Apache-2.0) | Aratu's sound engine: synthesis, effects, the SFZ sampler, plugin hosting and the DAW wrappers |
| **Aratu** | this language and its editor | |
| Parabolic | a plugin and sampler generator for musicians | its instruments play in Aratu as plugins on a desktop or as SFZ in a browser |

Manguebyte's website (being built) will present all three together, each
linking to the others.

## Documents

- [docs/MANUAL.md](docs/MANUAL.md): the manual, a draft written from the design so far:
  rhythm, drums, melodies, chords and strumming, dynamics, tempo, feel,
  patterns, live playing, transformations, sound, and a quick reference.
- [docs/DESIGN-NOTES.md](docs/DESIGN-NOTES.md): syntax discussion and recommendations
  (no program ceremony, one pattern line per bar, numbers as controls).
- [docs/VISION.md](docs/VISION.md): the problem with time in today's
  live-coding languages, the principles, the language sketch, how it would
  run on Manguebyte, and what the first version contains.

## Licence

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option, like Manguebyte.
