# Alesson's recorded kit (CC0)

A sampled drum kit made from Alesson Queiroz's own recordings and dedicated to
the public domain under [CC0 1.0](https://creativecommons.org/publicdomain/zero/1.0/)
(see `LICENSE.txt`). WAV 48 kHz, 24-bit, mono, one hit per file, a General
MIDI map in `voice.sfz`, in the shape of Aratu's SFZ kits, played by
Manguebyte's sampler. Translated from the Portuguese README that came with
the kit (its name there: "Kit gravado do Alesson").

## Files

| File | What it is |
|---|---|
| `samples/` | 22 WAVs, one hit each |
| `voice.sfz` | the map: keys, velocity layers, round robins (with comments) |
| `report.json` | measurements of every sample (peak, length, layer, leakage of the extracted ones) and a summary of the decisions |
| `LICENSE.txt` | the CC0 1.0 dedication |

The kit also came with `demo_groove.wav`, four bars at 96 BPM played by
Manguebyte's own sampler with this `voice.sfz`; it is not copied here.

## Keys

| GM key | Piece | Samples | Layers and round robins | Where it came from |
|---|---|---|---|---|
| 36 Bass Drum 1 | kick | `kick_groove_rr1`–`rr6` | 1 layer, 6 round robins | extracted from `groove-completo` |
| 38 Acoustic Snare | main snare | `snare_v1_rr1`, `snare_v2_rr1`–`rr5`, `snare_v3_rr1`–`rr2` | v1 = vel. 1–40 (ghost note, 1); v2 = 41–100 (5 RR); v3 = 101–127 (2 RR) | `snare-drum-1` and `snare-drum-2` |
| 40 Electric Snare | **alternative snare** | `snare_groove_rr1`–`rr6` | 1 layer, 6 round robins | extracted from `groove-completo` |
| 49 Crash Cymbal 1 | crash | `crash_rr1`–`rr2` | 1 layer, 2 round robins | `ataque` |

**No sample** (the engine falls back to its synthesized sound): 42/44/46
closed, pedal and open hi-hat, and 57 (the second crash). They exist only
inside the groove and did not come out clean: see "What was left out".

**Velocity.** Gain follows (velocity/127)² (`amp_veltrack=100`), except in
the ghost layer, which has a fixed gain. On the snare (38), with Aratu's
dynamics: ppp (16) and pp (33) play the ghost note; p (49), mp (64), mf (80)
and f (96) play layer v2; ff (112), fff (127) and accents (mf + 32 = 112)
play v3. The layers' `volume` only joins one to the next: v2 −1.3 dB, to be
as loud as v3 between velocities 100 and 101; v1 +8.9 dB, to sit 3 dB under
the softest v2 note. The files keep the real dynamics.

**In Aratu** the voice is `drums.alesson`. The letters `b`, `s` and `x` play
keys 36, 38 and 49; `c` and `o` (the hi-hats) and the other drums stay
synthesized. The alternative snare (40) is reached only by MIDI note number.

## How it was made

### Sources

`snare-drum-1.mp3`, `snare-drum-2.mp3`, `ataque.mp3` and `groove-completo.mp3`
(MP3 192 kbit/s, 48 kHz) and `snare-drum-rufu.m4a` (AAC 128 kbit/s,
44.1 kHz), decoded to 48 kHz floating-point WAV.

### Cleaning (the same for all)

1. **Mono**: (L + R) / 2. Measured between the channels first: no delay or
   polarity inversion in any source. The groove and `snare-drum-1` are
   already mono (correlation 0.999 and 1.000); `snare-drum-2` (0.79) and
   `ataque` (0.54, uncorrelated above 500 Hz) sum without comb filtering:
   per octave band the sum loses at most 1.2 dB and 3.4 dB, with no notches.
2. **Causal high-pass** (Butterworth, no pre-echo), well below each piece,
   which also removes DC: snares 60 Hz (4th order; their lowest mode is
   ~180 Hz), kick 25 Hz (2nd), cymbal 250 Hz (4th). `ataque` was played
   together with a kick (a 45 Hz tone and a 50–300 Hz thump); the cymbal
   itself has nothing below ~400 Hz (lowest partials at 529 and 544 Hz),
   where the filter is flat within 0.1 dB.
3. **Start**: 0.5 ms before the attack (the first point at −24 dB from the
   hit's peak), with a 0.5 ms fade-in over those samples before the attack
   only. No pre-delay: the peak falls between 2 and 15 ms.
4. **End**: when the tail reaches the noise floor (or −60 dB); if the next
   hit comes first, 25 ms before it (the MP3's pre-echo starts before the
   hit). Cosine fade-out over the last quarter.
5. **Gain**: one per instrument, with the loudest hit at −1 dBFS (snare 38:
   −1.75 dB; snare 40: −0.78 dB; kick: +5.31 dB; cymbal: +9.19 dB). No
   hit-by-hit normalization, no compression, no EQ.
6. **24 bits** without dither: the sources' noise sits more than 70 dB above
   the LSB.

### Main snare (38)

- `snare-drum-1` reaches ~18.7 kHz and has hits of several strengths, but
  the ones at 0.382 s and 0.632 s are cut by the next hit at −28 and −30 dB
  and were left out. In went the ghost note at 0.144 s (peak −36 dBFS; its
  tail reaches the take's noise floor, −73 dBFS, before the next hit) and
  the two loud hits at 0.901 s and 2.488 s.
- `snare-drum-2`: six nearly identical hits (correlation 0.97), one per
  second, with nothing above 11.8 kHz. Five went in; the sixth is cut by the
  end of the file at −45 dB. The timbre is a little different (modes at
  182/201/224 Hz against 180/230/254 Hz in `snare-drum-1`, darker between 1
  and 4 kHz, rings longer); so it takes the middle layer instead of
  alternating with the hits of `snare-drum-1`.
- `snare-drum-rufu` gave no hits: it holds buzz rolls, strokes every
  25–50 ms, each cut by the next. The stretches at 36.9–38.4 s and
  50.6–51.6 s end in edit cuts, the short events at 38.7–45.4 s are stick
  clicks (ringing at ~830 Hz), not the snare, and 45.6–48 s is a ~1.1 kHz
  tone. So the ghost layer has a single hit.

### Cymbal (49)

The four hits of `ataque` are copies of the same recording (correlation 0.92
to 0.99 once aligned; the first differs between 0.1 and 1.5 s and starts
with a 40 ms edit cut, which was removed). Two were kept: the fourth, with
its whole tail to the end of the file (−65 dB), and the first, whose tail
meets the next hit at −45 dB; from 3.55 s on it continues with the same
recording, taken from the fourth hit (correlation 0.98 there), aligned to
the sample, through a 0.25 s crossfade. The tails last ~5 s.

### Extraction from the groove (36 and 40)

`groove-completo` is one 2.5 s bar (96 BPM) repeated: kick on sixteenths 1,
8, 10, 11 and 14 of the bar, snare on beats 2 and 4, open hi-hat on the "and"
of 1, closed hi-hat from the "and" of 2 to the "and" of 4, ghost notes
before the next bar and the cymbal on every beat 1. At 16.03 s there is a
cut and a repeat of the first bar; only the original part was used.

1. **Score**: onsets detected and labelled by their place in the bar and by
   the bands that rise (40–120 Hz kick; 150–350 Hz snare body; 6–16 kHz
   hi-hat; a long tail above 3 kHz cymbal).
2. **Separation**: score-informed deconvolutive NMF (KL divergence, STFT of
   1024 points and hop of 256), one spectro-temporal template per piece,
   activations only at the score's onsets. The templates must decay
   exponentially after the attack (otherwise the cymbal's template learns
   the snare that always falls 0.625 s after it) and stay within each
   piece's band. Then a (power) Wiener mask over the mix's STFT, with the
   mix's phase.
3. **Leakage**: the energy of the other pieces that passes through the mask
   within the sample's window, relative to the piece's own energy. Two
   numbers: the model's own estimate, and the **validation**: a synthetic
   replica of the groove with known parts (same score and levels, the
   isolated snare and cymbal takes with the groove's cymbal decay, separate
   kicks and hi-hats, an MP3 192 kbit/s round trip), separated by the same
   process. On the replica the model's estimate stayed within −8/+4 dB of
   the truth for the chosen kicks and snares, but was optimistic by 6 to
   25 dB where the pieces sound together in the same band (hi-hats, cymbal,
   kick with cymbal). So the decision used the validation: **down to
   −25 dB**.

| Chosen | Validated leakage | Model estimate |
|---|---|---|
| 6 kicks (5 on the sixteenth before beat 3, 1 just after 4) | −27.2 to −40.9 dB | −39 to −48 dB |
| 6 snares (beats 2 and 4) | −27.8 to −28.1 dB | −36 to −38 dB |

In both cases the MP3's own noise sits around −30 dB, as in every source at
192 kbit/s. No musical noise: the spectrograms show no isolated blinking
points; mask jumps of more than 20 dB between neighbouring frames appear in
up to 0.9 % of the cells in the snares and up to 6 % in the kicks.

### What was left out

| Piece of the groove | Validated leakage | Why |
|---|---|---|
| Closed hi-hat (42) | −0.7 to −11 dB; with 30 ms windows, −3 to −15 dB | the cymbal's tail and the kick's highs fill the same band |
| Closed hi-hat with the kick or snare | −5 to +19 dB | it sounds at the same time as them |
| Open hi-hat (46) | −8.5 to −11.9 dB | it sounds over the cymbal's tail, noise over noise |
| Pedal hi-hat (44) | — | it closes the open one on beat 2, under the snare |
| The groove's cymbal (57) | −0.2 to −4.0 dB | it plays under the open hi-hat, the snare and some ten other hits per bar |
| The groove's ghost notes | −30 to +8 dB | the few below −25 dB are cut by the next hit within 61–83 ms |
| Kicks with the cymbal (beat 1) | −13 to −18 dB | the cymbal's attack leaks |
| Snares at 6.872 s and 14.372 s | −24.0 and −25.8 dB | the first does not pass; the six chosen (−27.8 to −28.1 dB) are the cleanest of the twelve |

## Checks

- **Every sample**: first and last sample at zero, no steps at the end, no
  second attack (the detector finds a second snare at −16 dB 150 ms later
  and a hi-hat touch at −40 dB inside a kick), fade only after −35 dB or
  already at the noise floor, peak ≤ −1 dBFS, DC below −80 dB. All in
  `report.json`.
- **On Manguebyte's sampler**: `Instrument::open` loads `voice.sfz` with 22
  regions, 22 samples and no warning. 44 test notes played by the `Sampler`:
  each one sounded the right sample (correlation 1.000000), with the gain
  the opcodes predict (error below 0.05 dB) and the round robins in order.
- **Demo**: `demo_groove.wav` was played by the same `Sampler` (46 notes:
  kick, the three layers of snare 38 with ghosts and accents, snare 40 and
  the cymbal) and lowered by 3.67 dB so that its peak sits at −1 dBFS.

## Known limits

- The sources are lossy: MP3 at 192 kbit/s (codec noise around −30 dB in
  the extractions) and AAC in the roll. `snare-drum-2` has nothing above
  11.8 kHz, `ataque` reaches ~14–15 kHz and the groove 14–19 kHz, depending
  on the stretch.
- `snare_v3_rr1` has a shortened tail: a fade from −36 dB at 0.40 s (there
  are small noises in the take just after).
- The groove's kicks last ~0.3 s (the next kick comes 312 ms later); below
  ~−40 dB the tail is the kit's resonance, which cannot be separated.
- In the groove's snares, the tail below ~−50 dB carries a little of the
  kit's continuous low end (100–400 Hz).
- The cymbal is one strike, recorded once; the two samples differ only in
  the first second and a half.
- The ghost note is a single hit, with no round robin, at a fixed strength
  from 1 to 40.
- There are no hi-hats and no groove cymbal.
