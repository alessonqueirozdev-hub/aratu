# Inside Aratu

This part is for people who program, or who want to know how Aratu works
underneath. It gives technical names to the things the course and the manual
explain in musical words: what a line is, a block, an `at bar`, a motif, and
what the computer does with the text until it becomes sound.

None of this is needed to play. Every technical term comes with a plain
explanation beside it, so it can be read without knowing how to program.

## 1. What kind of language it is

Aratu is a **declarative** language: the text describes what sounds, not a
list of orders carried out one after another. It belongs with HTML, CSS or a
spreadsheet. You do not write "play this note, wait, play the next"; you
write what the music is, and Aratu works out when each sound happens.

Some consequences:

- **The order of lines hardly matters.** A motif may be used before it is
  defined, a pattern played before it is written. Aratu reads the whole text
  before playing (in programming, this is called *hoisting*).
- **There are no changing variables, loops or `if`s.** Repetition is already
  in time: every line goes back to its start by itself. Choice is `{c|g}`.
  Conditions are about time: `at bar 9:`, `every 4 bars:`.
- **The text is the single source of truth.** Each edit makes Aratu read the
  whole text again. What the player keeps from one version to the next is
  little: the bar where each line written live started sounding.

```aratu
tempo: 100
melody = | riff riff+1 riff+2 riff |
motif riff = [e4 d c d]
```

Here the motif is written after the line that uses it, and it works.

## 2. The lexicon: the words of the language

The lexicon is the set of words, or *tokens*, Aratu recognises. Each line is
a **statement**. Inside braces, `;` separates statements on one line, and
`//` starts a comment to the end of the line.

| Kind | Examples | What it is |
|---|---|---|
| name (identifier) | `melody`, `bass`, `riff`, `temas` | letters, digits and `_`, starting with a letter |
| path | `bass.reverb.mix`, `drums.hh`, `riff.inv` | names joined by dots |
| keyword | `tempo`, `meter`, `key`, `pattern`, `play`, `at`, `every`, `now`, `over`, `motif`, `row`, `list`, `from` | reserved words, with a fixed meaning |
| number | `120`, `0.42`, `-1`, `1.2` | whole numbers and decimals, with a point |
| unit | `Hz`, `kHz`, `ms`, `s`, `%` | right after a number |
| operator | `=`, `:`, `->`, `<->`, `.`, `,` | join the parts of a statement |
| music | `c4`, `f#`, `eb`, `do4`, `Am`, `c`, `o`, `_`, `-`, `[ ]`, `{ \| }`, `\|` | the symbols inside a line of music |

The music symbols make a small language inside the language: a music
**literal**, the way a string or a number is a literal in other languages.
A space separates beats, `|` separates bars, `[ ]` groups within a beat,
`{ | }` draws one, `_` holds, `-` rests, `+` joins notes in a chord, upper
case accents, `'` is staccato, `^` is a fermata, `?` is chance and `x4`
repeats.

## 3. The grammar: the kinds of statement

The grammar says how the words go together. Each statement has one of these
shapes (what is between `⟨ ⟩` is a place to fill):

| Shape | Example | What it is for |
|---|---|---|
| `⟨frame⟩: ⟨value⟩` | `tempo: 96`, `meter: 3/4`, `key: A minor` | the frame of the piece |
| `⟨name⟩ = ⟨music⟩` | `melody = \| c4 d e f \|` | a named line of music |
| `⟨name⟩ { … }` | `drums { hh = c c c c }` | a block of a part's lines |
| `⟨part⟩: ⟨sound⟩ -> ⟨effect⟩ …` | `bass: synth.saw -> lowpass` | a part's signal chain |
| `⟨path⟩ = ⟨value⟩` | `bass.reverb.mix = 0.42` | a parameter |
| `⟨path⟩ = ⟨value⟩ -> ⟨value⟩ over ⟨length⟩` | `bass.lowpass.cutoff = 300 Hz -> 4 kHz over 4 bars` | a parameter that moves |
| `⟨part⟩: ⟨order⟩` | `piano: mf`, `guitar: out`, `piano: cresc to ff over 4 bars` | an order to a part |
| `pattern ⟨name⟩ { … }`, `section ⟨name⟩ { … }` | `pattern fill { sd = ss ss SS SS }` | a named group of lines |
| `play …`, `add …`, `form: …` | `play groove x3, fill` | what plays, and in what order |
| `motif`, `row`, `list` | `motif riff = [e4 d]` | definitions other lines use |
| `⟨when⟩: ⟨statement⟩` | `at bar 9: play fill`, `at bar 9 beat 3: …`, `from bar 9 to 16: …`, `every 4 bars: …`, `now: …` | the same statement, at a moment or for a stretch |

The `⟨when⟩` goes in front of almost any statement. Without it, the
statement holds from the start of the piece or, if the music is playing,
from the next bar.

## 4. Naming things

This table is the heart of this part: for each thing in Aratu, the name it
would have in programming, and what it is like.

| In Aratu | In programming terms | Like |
|---|---|---|
| `melody = \| c4 d e f \|` | a **binding** of a name to a value; the value is a sequence of bars | a constant: it does not change while playing; each edit defines it again |
| a line | a **circular sequence**: when it ends, it starts again | a looping clip in a DAW |
| `drums { hh = …; bd = … }` | a **record** or *namespace*: a map from names to lines | an object (`drums.hh`) |
| `bass.reverb.mix = 0.42` | **property assignment** on a path through a **nested map** (part → effect → parameter) | `bass.reverb.mix = 0.42` in JavaScript, or a CSS rule |
| `bass: synth.saw -> lowpass -> reverb` | a **pipeline**: each one's output is the next one's input | function composition, `reverb(lowpass(saw))`, or the shell's `\|` |
| `synth.saw`, `piano.grand` | a **key in a catalogue** (an enum): a name that picks a ready-made sound | a value from a fixed list; not a method |
| `motif riff = [e4 d c d]` | a **macro**: the name is replaced by its content wherever it appears | a function without side effects, expanded in place (*inlining*) |
| `riff+2`, `riff+3st` | the motif with an **argument**: the transposition | `riff(2)` |
| `riff.inv`, `riff.retro`, `serie.P5` | a **pure transformation** written with a dot: it returns a new motif and leaves the original alone | a method or computed property without parentheses |
| `row serie = 0 11 3 4 …` | a **vector** (*array*) of whole numbers from 0 to 11 | `[0, 11, 3, 4, …]` |
| `\| o o o o \| from serie.P0` | a **zip** of the rhythm with the vector, starting it again when it runs out; each `o` is a **placeholder** to fill | `zip(rhythm, cycle(row))` |
| `list temas { … }` | an **array** of melodies, counted from 1 | `temas[1]` in JavaScript is `temas.2` in Aratu |
| `temas.all`, `temas.random` | concatenation; a random choice of one item | `temas.flat()`, `temas[random()]` |
| `{c\|g}` | a **random choice**, drawn each time round | `random.choice` |
| `c?30%` | an event with a 30% **probability** | `if (random() < 0.3)` |
| `seed 7` | the **seed** of the pseudo-random generator: the same seed gives the same result | `Random(7)` |
| `pattern groove { … }` | a **named block** of lines | a component, a scene, a template |
| `play groove x3, fill` | a **playlist**: a sequence with repeats, looping | a list of scenes |
| `\|: :\|`, `%`, `D.S. al coda` | the score's **jumps**, resolved before playing | a `goto` the compiler unrolls |
| `[e4 d]x4`, `riffx4` | **unrolled repetition**: it becomes four copies | *loop unrolling* |
| `p:`, `mf:` | a **state** that holds for the following notes of the line | the "current colour" in a drawing library |
| `300 Hz -> 4000 Hz over 4 bars` | **interpolation** of a value over time (on a logarithmic scale for frequencies) | a *tween*, keyframe automation |
| a knob | a **two-way binding**: turning it rewrites the number in the text | the text stays the source of truth |

### Is `at bar` a switch?

Not quite. A `switch` (or `case`) picks **one** path among several, each time
it runs, by a value. An `at bar` is a **rule indexed by time**: from bar 9
on, it holds, and it keeps holding until another rule changes the same
thing. It is closer to a **keyframe** in an animation, or an event marked on
a timeline.

```aratu
tempo: 110
bass = | c2 _ g1 _ |
at bar 5: bass.reverb.mix = 0.6
at bar 9: bass: out
```

To know the state at bar 12, Aratu gathers the rules of every bar up to
there: the reverb is at 0.6 (since bar 5) and the bass is out (since bar 9).
In programming, that is a **fold** of the changes, in time order.
`from bar 9 to 16:` is a rule with a start and an end: outside its stretch
it stays out of the fold, and what came before holds again.

`every`, on the other hand, is the nearest thing to an `if` on the bar
number:

```aratu
tempo: 110
pattern groove { hh = cc cc cc cc; bd = b - b -; sd = - s - s }
pattern fill { sd = ss ss SS SS }
play groove
every 4 bars: play fill
```

`every 4 bars: play fill` means, bar by bar, "if this is the last of every
four, play the fill instead": `if (bar % 4 == 0)`. And `every 4 bars, first:`
is `if (bar % 4 == 1)`.

## 5. From text to sound

When you edit, the text goes through these stages, each in a module of the
compiler (written in Rust and brought to the browser as WebAssembly):

1. **Lexing.** The text is cut into lines, blocks and words.
2. **Gathering.** Motifs, rows, lists and the key are collected first, so a
   line can use them from anywhere in the text.
3. **Parsing.** Each line becomes a statement in a **syntax tree** (*AST*),
   with the exact place each piece is written, for the messages and for
   lighting the notes.
4. **Model.** The statements become parts, lines, patterns, sections and
   orders, with the checks: whether each bar fits the meter, whether the
   names exist.
5. **Unrolling.** Repeats, jumps, `%` and `x4` become a plain sequence of
   bars.
6. **Dynamics and time.** Each note gets its strength, and positions become
   seconds through the tempo map. Positions are **exact fractions**: a
   triplet is 1/3 of a beat, with no rounding.
7. **Scheduling.** For the bars asked for, out come the **events**: each
   note, each hit, each parameter change, with its moment in seconds.

Scheduling is **deterministic**: the same text, the same seed and the same
starting bars always give the same events. That is why the player can ask
for one bar at a time, a little before it sounds, and an edit can come in at
the next bar without anything starting over.

The events go to the audio engine. Recorded sounds and drum kits play on
Manguebyte's engine (an AudioWorklet running WebAssembly); the others on the
stand-in engine built with the browser's Web Audio.

## 6. The data: what comes out of the compiler

The IDE talks to the compiler through a few functions the WebAssembly
exports (`aratu_compile`, `aratu_render`, `aratu_check`, `aratu_grid`,
`aratu_voices`), exchanging text as **JSON**. Compiling returns the
messages, the parts, the knobs, the lists, the rows and the lines written
live. Asking for bars returns the events. A note event looks like this:

```json
{"bar": 3, "beat": 1.5, "time": 4.75, "dur": 0.24, "part": "bass",
 "line": "bass", "vel": 80, "note": 45, "from": 120, "to": 123}
```

| Field | What it is |
|---|---|
| `bar`, `beat` | the bar and the beat within it, as written |
| `time`, `dur` | when it sounds and how long, in seconds from the start |
| `part`, `line` | the part and the line that play it |
| `vel` | its strength, from 0 to 127, as in MIDI |
| `note` | the note, as a MIDI number (60 is middle C) |
| `drum` | in place of `note`, the drum: `kick`, `snare`, `closed-hat`… |
| `acc` | 1 when the note was written in upper case |
| `param`, `value`, `glide` | in place of a note, a parameter change; `glide` is the seconds over which the sound slides to the value |
| `from`, `to` | where the symbol is written in the text, to light it when it sounds |

Messages are data too: each has its severity (error or warning), the stretch
of text and the text in the chosen language. An error does not stop the
music: while the text has an error, the last good version keeps playing,
like a *hot reload* that refuses a broken build.
