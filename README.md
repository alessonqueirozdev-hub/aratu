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
the open-source audio framework for Rust, and runs in the browser; running
inside a DAW, in step with the host's transport, is planned.

> **Status: in development.** The language crate (`crates/aratu`) reads,
> checks and schedules every example in the book, and the web editor plays
> through Manguebyte's browser engine: every voice in the Sounds panel, each
> part's effects and the master bus (a shared reverb, a glue compressor and a
> true-peak limiter). The Brazilian percussion lines, the guitar's golpe and
> the clap (except on the 808) still play on a small Web Audio stand-in until Manguebyte has samples for
> them; the stand-in also takes over if Manguebyte cannot start, or when it
> is switched off in the Sounds panel. See [docs/VISION.md](docs/VISION.md)
> for where it is going.

## Try it

**<https://aratu.live/>**: the editor in the
browser, nothing to install, in Portuguese, Spanish and English. It has
three places:

- **Learn**: a course of twenty-four lessons, from the first sound to a
  whole piece: rhythm and meters, Brazilian percussion, melody and harmony,
  dynamics and form, tempo and feel, playing live, sound and mixing. Every
  example plays on the page and every exercise is checked by what it plays
  (and, where a lesson asks, by its timing or its sound), with what differs
  told bar by bar. Made for the classroom.
- **Manual**: the whole language in the same order, with every example
  playable.
- **Play**: a free editor with a rhythm map, a knob beside every number
  that sets the tempo or the sound, and a Sounds panel; change the text while it plays and the change comes in at
  the next bar line. A stage mode shows only the code, full screen, for
  playing in front of an audience.

## Running the editor

To run it from the source you need Rust (with `rustup target add wasm32-unknown-unknown`) and Node.

```
cd web
npm install
npm run dev
```

Then open <http://localhost:5173>. `Ctrl+Enter` plays or updates, `Ctrl+.`
stops, and any edit while it plays comes in at the next bar line.

## A taste

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

A space separates beats, symbols written together share their beat (`cc`
is two eighths), `_` holds and `-` rests. Chords are written as a lead
sheet, strums with `d` and `u`, dynamics and repeats with the score's own
signs, and note names in solfège (`dó4 ré4 mi4`) work as well as letters.
When a bar does not add up, Aratu says so in a musician's words, in
Portuguese, Spanish or English: *"o compasso 2 de `bass` tem 5 tempos, mas a
fórmula de compasso é 4/4"*.

## The Manguebyte ecosystem

Aratu is one of three projects that grow from the same mangue:

| Project | What it is | How it meets Aratu |
|---|---|---|
| [Manguebyte](https://github.com/alessonqueirozdev-hub/manguebyte) | the open-source audio framework for Rust (MIT OR Apache-2.0) | Aratu's sound engine: in the browser today, its instruments, effects, SFZ sampler and master bus; on the desktop, plugin hosting and the DAW wrappers (planned) |
| **Aratu** | this language and its editor | |
| Parabolic | a plugin and sampler generator for musicians | its instruments are to play in Aratu as plugins on a desktop or as SFZ in a browser (planned) |

Manguebyte's website (being built) will present all three together, each
linking to the others.

## Documents

- [book/](book): the course and the manual shown in the editor, in
  Portuguese, Spanish and English (`book/<lang>/learn/*.md` and
  `book/<lang>/manual.md`). The tests in
  [crates/aratu/tests/book.rs](crates/aratu/tests/book.rs) check that every
  example plays and every exercise can be solved
  (`every_block_of_the_book_works`), that each exercise sounds the same in
  every language (`every_exercise_sounds_the_same_in_every_language`), and
  that every feature of the language is taught in some lesson
  (`every_feature_is_taught_in_every_language`).
- [docs/DESIGN-NOTES.md](docs/DESIGN-NOTES.md): syntax discussion and recommendations
  (no program ceremony, one pattern line per bar, numbers as controls).
- [docs/VISION.md](docs/VISION.md): the problem with time in today's
  live-coding languages, the principles, the language sketch, how it would
  run on Manguebyte, and what the first version contains.

## Licence

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option, like Manguebyte.
