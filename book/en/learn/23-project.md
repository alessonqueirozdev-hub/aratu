# Project: a maracatu

Now you have everything to build a whole piece. We will build, part by
part, a groove inspired by the *maracatu de baque virado* of Recife, in
Brazil, in a simplified version: with the alfaias, the snare, the gonguê
and the agbê, and then with form, dynamics and mixing.

## 1. The pulse

Maracatu moves slowly and heavily. Start with the tempo and the alfaia,
the big, low rope drum, marking beats 1 and 3:

```aratu
tempo: 92
alfaia = O - O -
```

The upper case is the accented stroke, open and strong.

## 2. The alfaias' figure

The alfaias play a figure that repeats. This one takes one bar, with
eighths and sixteenths:

```aratu
tempo: 92
alfaia = O -o o_oo o
```

Read it beat by beat: on 1, the strong stroke; on 2, an eighth rest and an
eighth; on 3, an eighth and two sixteenths; on 4, a quarter note.

## 3. The snare

The *caixa de guerra*, a marching snare, plays sixteenths all the time,
with an accent at the start of each beat. It is an ordinary drum line, the
snare:

```aratu
tempo: 92
alfaia = O -o o_oo o
snare  = Ssss Ssss Ssss Ssss
```

## 4. The gonguê

The gonguê is the iron bell that keeps time for everyone. It alternates the
open sound and the muted one:

```aratu
tempo: 92
alfaia = O -o o_oo o
snare  = Ssss Ssss Ssss Ssss
gongue = o -o -m o
```

## 5. Exercise: the agbê

The agbê is a gourd covered in a net of beads, shaken like a shaker. Add it
to the groove, in eighth notes, with an accent at the start of each beat.

```aratu exercise dynamics=1
tempo: 92
alfaia = O -o o_oo o
snare  = Ssss Ssss Ssss Ssss
gongue = o -o -m o
```

```aratu answer
tempo: 92
alfaia = O -o o_oo o
snare  = Ssss Ssss Ssss Ssss
gongue = o -o -m o
agbe   = Ss Ss Ss Ss
```

## 6. The form: call and groove

In maracatu, the master calls and the group answers. Make a **call** with
the gonguê alone, and then the **groove**, the *baque*, with everyone:

```aratu
tempo: 92
section call { gongue = o -o -m o }
section groove {
  alfaia = O -o o_oo o
  snare  = Ssss Ssss Ssss Ssss
  gongue = o -o -m o
  agbe   = Ss Ss Ss Ss
}
form: call x2 groove x4
```

## 7. Dynamics and mixing

To finish, a room for the group to play in and a crescendo that keeps
building:

```aratu
tempo: 92
section call { gongue = o -o -m o }
section groove {
  alfaia = O -o o_oo o
  snare  = Ssss Ssss Ssss Ssss
  gongue = o -o -m o
  agbe   = Ss Ss Ss Ss
}
form: call x2 groove x4
at bar 3: all: cresc to ff over 4 bars
reverb.mix = 0.35
master.reverb.decay = 2.5 s
```

## 8. Final exercise

Build your own version: the call only once, the groove eight times, and the
whole group in the room, with `reverb.mix` at 0.35.

```aratu exercise sound=1 bars=9
tempo: 92
section call { gongue = o -o -m o }
section groove {
  alfaia = O -o o_oo o
  snare  = Ssss Ssss Ssss Ssss
  gongue = o -o -m o
}
form: call x2 groove x4
```

```aratu answer
tempo: 92
section call { gongue = o -o -m o }
section groove {
  alfaia = O -o o_oo o
  snare  = Ssss Ssss Ssss Ssss
  gongue = o -o -m o
}
form: call groove x8
reverb.mix = 0.35
```

## What next?

- Open the piece in **Play** (the example's button) and keep changing it:
  change the alfaias' figure, add a fill with `pattern` and `every`, choose
  a sound in the **Sounds** panel.
- Look things up in the **Manual** whenever you need a symbol.
- Play with other people: one changes the alfaia, another the gonguê, live.

> Real maracatu is learnt from the masters and the *nações* of Recife. This
> groove is only a starting point for listening and writing.
