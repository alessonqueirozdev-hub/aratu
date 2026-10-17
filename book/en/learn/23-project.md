# Project: a maracatu

Now you have everything to build a piece. We will build, part by part, a
groove inspired by the *maracatu de baque virado* of Recife, in Brazil, in
a simplified version for Aratu's drum kit.

## 1. The pulse

Maracatu moves slowly and heavily. Start with the tempo and the bass drum,
playing the part of the low *alfaias* (big rope drums):

```aratu
tempo: 92
drums {
  bd = b - b -
}
```

## 2. The alfaias

The alfaias play a figure that repeats. This one takes one bar, with
eighths and sixteenths:

```aratu
tempo: 92
drums {
  bd = b -b b_bb b
}
```

## 3. The snares

The snares play sixteenths all the time, with accents:

```aratu
tempo: 92
drums {
  bd = b -b b_bb b
  sd = Ssss Ssss Ssss Ssss
}
```

## 4. The gonguê

The *gonguê* is the iron bell that keeps time for everyone. The ride
(`r`) plays its part here:

```aratu
tempo: 92
drums {
  bd = b -b b_bb b
  sd = Ssss Ssss Ssss Ssss
  rd = r -r -r r
}
```

## 5. Final exercise

Put it all together and add the *agbê* (the closed hi-hat in eighths) and
a bass playing C2 and G1, a half note each, in one bar.

```aratu exercise
tempo: 92
drums {
  bd = b -b b_bb b
  sd = Ssss Ssss Ssss Ssss
  rd = r -r -r r
}
```

```aratu answer
tempo: 92
drums {
  bd = b -b b_bb b
  sd = Ssss Ssss Ssss Ssss
  rd = r -r -r r
  hh = cc cc cc cc
}
bass = | c2 _ g1 _ |
```

## What next?

- Open the piece in **Play** (the example's button) and keep changing it:
  change the bass, add a fill with `pattern` and `every`, make a crescendo.
- Look things up in the **Manual** whenever you need a symbol.
- Play with other people: one changes the drums, another the bass, live.

> Real maracatu is learnt from the masters and the *nações* of Recife. This
> groove is only a starting point for listening and writing.
