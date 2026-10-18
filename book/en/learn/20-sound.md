# Sounds and effects

So far each part played with the sound Aratu chose for it. Now you choose:
a piano, a steel-string guitar, a slap bass, a synthesizer. And you send
the sound through effects, the way a guitarist runs the guitar through
pedals.

## A part's sound

Write the part's name, a colon and the sound's code:

```aratu
tempo: 100
bass: bass.slap
bass = | e2 E3 e2 E3 | a2 A3 a2 A3 |
```

In `bass.slap`, an upper-case note is the *pop*: the string pulled so it
snaps. The codes have two parts, the family and the instrument:

| Family | Some sounds |
|---|---|
| keyboards | `piano.grand`, `piano.upright`, `keys.epiano`, `keys.organ`, `keys.harpsichord` |
| mallets and bells | `mallets.marimba`, `mallets.vibraphone`, `mallets.celesta`, `mallets.steelpan`, `bells.tubular` |
| plucked strings | `guitar.nylon`, `guitar.steel`, `harp` |
| bowed strings | `strings.violin`, `strings.viola`, `strings.cello`, `strings.bass`, `strings.section`, `strings.violin_pizz` |
| basses | `bass.finger`, `bass.slap`, `bass.808`, `bass.acid`, `bass.synth` |
| winds | `flute`, `clarinet`, `sax`, `trumpet`, `trombone`, `sax.tenor`, `recorder.alto` |
| voices | `choir.aah`, `choir.ooh`, `voice.solo` |
| synthesizers | `synth.saw`, `synth.pad`, `synth.supersaw`, `poly.trance_supersaw`, `poly.reese`, `poly.house_pluck`, `poly.string_machine` |
| drum kits | `drums.kit`, `drums.808`, `drums.909`, `drums.606`, `drums.cr78`, `drums.trap`, `drums.house`, `drums.techno`, `drums.dnb`, `drums.lofi`, `drums.acoustic`, `drums.alesson` |

Open the **Sounds** panel, beside the messages in Play, to hear each one
and put its code in the text with a click. They all play through
**Manguebyte**, the audio engine of Aratu's ecosystem.

> Change `bass.slap` to `bass.808` and then to `guitar.steel`. The notes
> stay the same; the instrument changes.

The `poly.` codes are the ready sounds of Manguebyte's polyphonic
synthesizer: more than sixty, from pads and leads to basses, plucks, stabs
and effects such as `poly.riser`. The electronic kits copy famous machines
(`drums.808`, `drums.909`, `drums.606`, `drums.cr78`) and styles
(`drums.trap`, `drums.house`, `drums.techno`, `drums.dnb`, `drums.lofi`);
the drums have the same names in all of them, so changing the kit does not
change the rhythm. Bowed strings, winds and the choir are modelled as they
play, with nothing to download.

> Change `drums: drums.808` to `drums.909` and then to `drums.lofi` in a
> rhythm of yours. Then put the melody on a `flute` and on a
> `strings.violin`.

## The effects chain

After the sound, arrows carry the signal through effects, from left to
right, like a pedalboard:

```aratu
tempo: 100
bass: synth.saw -> lowpass -> reverb
bass = | c2 _ c2 eb2 | f2 _ g2 bb1 |
```

| Effect | What it does |
|---|---|
| `lowpass` | muffles the highs, like putting a hand in front of your mouth |
| `highpass` | takes out the lows, like a small radio |
| `eq` | raises or lowers the bass, middle and treble |
| `drive` | distorts, like an amplifier turned all the way up |
| `tube` | warms and saturates, like a valve amplifier |
| `bitcrush` | the sound of an old video game |
| `compressor` | evens out the volume, the loud and the soft |
| `chorus` | doubles the sound, which turns wide and bright |
| `flanger`, `phaser` | the sound "flying", like a passing plane |
| `tremolo` | the volume pulsing |
| `autopan` | the sound moving from one speaker to the other |
| `delay` | echo |
| `reverb` | the sound in a room, a church, a hall |

## Numbers become knobs

Each effect has its numbers, written as `part.effect.number`. And each
number gets a round button, a knob, beside it: drag it up or down and the
sound changes at once.

```aratu
tempo: 100
bass: synth.saw -> lowpass -> delay
bass = | c2 _ c2 eb2 | f2 _ g2 bb1 |
bass.lowpass.cutoff = 800 Hz
bass.lowpass.resonance = 0.3
bass.delay.mix = 0.25
```

A part's numbers can also go in a block with its name, so you do not
repeat the name:

```aratu
tempo: 100
bass: synth.saw -> lowpass -> delay
bass = | c2 _ c2 eb2 | f2 _ g2 bb1 |
bass {
  lowpass.cutoff = 800 Hz
  delay.time = 375 ms
  delay.feedback = 0.4
}
```

The numbers used most:

| Number | What it is |
|---|---|
| `lowpass.cutoff`, `highpass.cutoff` | where the filter cuts, from 20 to 20000 Hz |
| `lowpass.resonance` | a peak at the cutoff, from 0 to 1 |
| `drive.amount` | how much it distorts, from 0 to 36 dB |
| `chorus.mix`, `delay.mix` | how much of the effect you hear, from 0 to 1 |
| `delay.time` | the time of the echo, in s or ms |
| `delay.feedback` | how many times the echo comes back, from 0 to almost 1 |
| `tremolo.rate` | how many times a second the volume pulses |

The manual has the whole list. The cutoff knob turns on a frequency scale,
the way the ear hears; hold Shift for finer steps.

## Equalizer and tube

Beside `eq`, `tube`, `drive`, `lowpass` and `highpass` in a chain, the
editor draws a small curve that shows what the effect does, and it moves
as you turn the knobs. The `eq`'s is the equalizer's response: the lows
on the left, the highs on the right, up what is boosted. Click it and the
equalizer opens large: drag the points of the lows, the mids and the
highs, and the numbers are written in the code. The mouse wheel on the
mids' point narrows or widens the band.

```aratu
tempo: 96
guitar: guitar.steel -> eq -> tube -> reverb
guitar = | e3 g3 a3 g3 | e3 _ d3 _ |
guitar.eq.low_gain = -6 dB
guitar.eq.mid_gain = 4 dB
guitar.eq.mid_freq = 900 Hz
guitar.tube.drive = 12 dB
```

The tube is Manguebyte's drive set up as a valve: the smooth curve that
rounds the peaks off, a little gain and the level brought back down. Its
numbers are `tube.drive` (0 to 24 dB), `tube.output` and `tube.mix`. The
curve beside it shows how much the sound is flattened, and glows the
harder it is driven. The tube and the drive are the same stage, so a
chain uses one or the other.

## A number that changes at a bar

Like any change, a number can change at a bar:

```aratu
tempo: 100
bass: synth.saw -> lowpass
bass = | c2 c3 c2 c3 |
bass.lowpass.cutoff = 400 Hz
at bar 3: bass.lowpass.cutoff = 3000 Hz
```

## Exercises

These exercises also check the **sound**: the instrument, the effects and
the numbers.

Make the bass play with the slap bass, `bass.slap`.

```aratu exercise sound=1
bass: synth.saw
bass = | e2 E3 e2 E3 |
```

```aratu answer
bass: bass.slap
bass = | e2 E3 e2 E3 |
```

Put a delay in the keys' chain, after the chorus.

```aratu exercise sound=1
keys: keys.epiano -> chorus
keys = | c4+e4+g4 _ _ _ |
```

```aratu answer
keys: keys.epiano -> chorus -> delay
keys = | c4+e4+g4 _ _ _ |
```

Muffle the synthesizer: the lowpass cutoff at 600 Hz.

```aratu exercise sound=1
bass: synth.saw -> lowpass
bass = | c2 _ c3 _ |
bass.lowpass.cutoff = 5000 Hz
```

```aratu answer
bass: synth.saw -> lowpass
bass = | c2 _ c3 _ |
bass.lowpass.cutoff = 600 Hz
```

Put the guitar through a tube, with 12 dB of drive.

```aratu exercise sound=1
guitar: guitar.steel -> reverb
guitar = | e3 g3 a3 g3 |
```

```aratu answer
guitar: guitar.steel -> tube -> reverb
guitar = | e3 g3 a3 g3 |
guitar.tube.drive = 12 dB
```
