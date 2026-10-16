# The drums

Each drum has a short symbol. The symbol decides the sound, whatever the
line is called.

| Symbol | Sound |
|---|---|
| `c` | closed hi-hat |
| `o` | open hi-hat |
| `b` | bass drum |
| `s` | snare |
| `r` | ride cymbal |
| `x` | crash cymbal |
| `t` | high tom |
| `m` | mid tom |
| `f` | floor tom |
| `k` | side stick (the stick across the snare's rim) |

Each line also has a **name**, written before the `=`. The custom is to
use the short names studios and drum machines use:

| Line name | Short for | Drum |
|---|---|---|
| `hh` | hi-hat | hi-hat |
| `bd` | bass drum | bass drum |
| `sd` | snare drum | snare |
| `rd` | ride | ride cymbal |
| `cr` | crash | crash cymbal |
| `tom` | tom | toms |
| `rim` | rim | side stick |

The name is only a label, to find your way; the symbol decides the sound.
A line `hats = cc cc cc cc` inside `drums { }` sounds the same.

The lines can sit together in a `drums { }` block, which shows they all
belong to the kit:

```aratu
tempo: 100

drums {
  hh = cc cc cc co
  bd = b - bb -
  sd = - s - s
}
```

The hi-hat opens on the last eighth (`co`) and the bass drum doubles on
beat 3 (`bb`).

## Accents

An **upper-case** letter is played with an accent, louder. Listen to the
snare accented on beat 4:

```aratu
drums {
  hh = cc cc cc cc
  bd = b - b -
  sd = - s - S
}
```

And an apostrophe after a symbol makes it short: `c'` is a very tight
hi-hat.

## A cymbal to start

The crash marks the start of a phrase. Here it plays on beat 1 while the
ride keeps the eighths:

```aratu
drums {
  cr = x - - -
  rd = rr rr rr rr
  bd = b - b -
  sd = - s - s
}
```

> The `cr` (crash) line holds the crash, `x`, and the `rd` (ride) line
> holds the ride, `r`. Change an `r` to `c` and hear the ride turn into
> the hi-hat.

## Toms and the rim

The toms play the fills: `t` is the high tom, `m` the mid tom and `f`
the floor tom. Here the second bar ends going down through all three:

```aratu
tempo: 96

drums {
  hh  = | cc cc cc cc | cc cc -  -  |
  bd  = | b  -  bb -  | b  -  -  -  |
  sd  = | -  s  -  s  | -  s  -  -  |
  tom = | -  -  -  -  | -  -  tm ff |
}
```

The rim, `k`, is the stick laid across the snare: a dry, woody click with
no snare wires. It is the bossa nova beat:

```aratu
tempo: 130

drums {
  hh  = cc cc cc cc
  bd  = b -b b -b
  rim = | k- -k - k | - k -k - |
}
```

## Exercises

A rock beat with the snare **accented** on beat 4.

```aratu exercise dynamics=1
drums {
  bd = b - b -
  sd = - s - s
}
```

```aratu answer
drums {
  bd = b - b -
  sd = - s - S
}
```

Open the hi-hat on the last eighth of the bar.

```aratu exercise
hh = cc cc cc cc
```

```aratu answer
hh = cc cc cc co
```

End the bar with a fill: on beat 4, an eighth on the high tom and one
on the floor tom.

```aratu exercise
drums {
  sd  = s s s -
  tom = - - - -
}
```

```aratu answer
drums {
  sd  = s s s -
  tom = - - - tf
}
```

Now build a whole beat: hi-hat in eighths, bass drum on beats 1 and 3 and
on the second half of beat 3, snare on beats 2 and 4.

```aratu exercise
drums {
  hh = - - - -
  bd = - - - -
  sd = - - - -
}
```

```aratu answer
drums {
  hh = cc cc cc cc
  bd = b - bb -
  sd = - s - s
}
```
