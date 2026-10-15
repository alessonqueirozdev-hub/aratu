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
  rd = cc cc cc cc
  bd = b - b -
  sd = - s - s
}
```

> Since the symbol decides the sound, the cymbal letters sit in a line
> called `cr` and the ride in `rd`, but any names would do.

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
