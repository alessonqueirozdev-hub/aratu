# Brazilian percussion

The drum kit has one symbol per piece. Brazilian percussion is different:
each instrument has a line of its own, named after the instrument, with its
own **strokes**: the thumb on the pandeiro, the bacalhau on the zabumba,
the triangle open or closed. The rhythm follows the same rules as always.

## A line named after the instrument

Write the instrument's name before the `=`, and use the letters of its
strokes:

```aratu
tempo: 100
triangle = fo fo fo fo
```

`f` is the closed triangle (the hand holds the metal) and `o` is the open
one (the hand lets go and it rings). That "closed, open" in eighth notes is
the heart of the baião.

| Line | Strokes |
|---|---|
| `pandeiro` | `p` thumb, `d` fingertips, `t` slap, `m` muted |
| `zabumba` | `o` open, `m` muted, `b` bacalhau (the thin stick underneath) |
| `triangle` | `o` open, `f` closed |
| `agogo` | `a` high bell, `g` low bell |
| `surdo` | `o` open, `m` muted |
| `tamborim` | `o` open, `m` muted |
| `alfaia` | `o` open, `m` muted |
| `gongue` | `o` open, `m` muted |
| `shaker` | `s` |
| `cuica` | `a` high, `g` low |
| `cajon` | `b` bass, `t` slap |

The names keep their Brazilian spelling because they are the instruments'
own names. The **pandeiro** is the Brazilian frame drum with jingles; the
**zabumba** is the wide bass drum of baião and forró, struck with a mallet
on top and a thin stick, the *bacalhau*, underneath; the **agogô** is a
pair of bells; the **surdo** is the big bass drum of samba; the
**tamborim** is a small, high frame drum played with a stick; the
**alfaia** and the **gonguê** (a large iron bell) come from maracatu; and
the **cuíca** is the friction drum that "talks" in samba.

A name may be written with or without its accent (`agogô` or `agogo`), and
in Portuguese or Spanish too (`triângulo`, `ganzá`, `cajón`). `agbe`,
`caxixi` and `chocalho` play like the shaker.

> If you write a letter the instrument does not have, `x` on the pandeiro
> for example, Aratu tells you which strokes it has.

## The baião

The baião brings three instruments together: the zabumba, the triangle and
the agogô (or the accordion, which comes later). The zabumba plays the
open stroke on beat 1, the muted one at the end of beat 2, and the
bacalhau answers underneath:

```aratu
tempo: 110
zabumba  = | o - -o m | o - b - |
triangle = fo fo fo fo
agogo    = a g ag -
```

> Change the tempo to 140 and the baião turns into forró pé de serra.

## The samba

In samba, each instrument has its role. The surdo marks beat 2, the
tamborim plays the figure, and the pandeiro and the shaker fill in the
sixteenths:

```aratu
tempo: 96
meter: 2/4
surdo    = m O
tamborim = o-oo -oo-
pandeiro = pdtd pdtd
shaker   = Ssss Ssss
```

Samba is counted in two. An upper-case letter is an accent, as on the
drums: `O` on the surdo is the strong stroke of beat 2, and `S` on the
shaker marks the start of each beat.

## With the drums

Percussion lines play together with the drums and the other parts. In
Brazil the drum kit's floor tom is also called *surdo*, but in Aratu a
`surdo` line is always the samba surdo; the floor tom is still `f`, on a
drum line:

```aratu
tempo: 100
drums {
  bd = b - b -
  sd = - s - s
}
agogo = ag ag a- g-
cajon = b t bb t
```

## Strokes spread evenly

Many rhythms, in Brazil and all over the world, are a few strokes spread as
evenly as they can be over the steps of a bar. Aratu writes them just so:
the stroke, then in parentheses how many strokes and over how many steps.

```aratu
tempo: 100
zabumba  = o(3,8)
triangle = fo fo fo fo
```

`o(3,8)` is three open strokes over the eight eighths of the bar, the same
as `o- -o -- o-`: the **tresillo**, the figure of the baião's zabumba,
heard under a great deal of the music of the Americas. Rhythms written this
way are called **Euclidean**: the way the strokes are spread is the same as
the Greek mathematician Euclid's way of finding the greatest common
divisor of two numbers.

Alone in its bar, the rhythm spreads over the whole bar, in any meter, the
way a rest alone fills its bar; next to other beats it takes one beat, divided
into that many steps.

| Write | The same as | What it is |
|---|---|---|
| `o(3,8)` | `o- -o -- o-` | the tresillo |
| `o(5,8)` | `o- oo -o o-` | the cinquillo, of Cuban music |
| `b(4,16)` | `b b b b` | the bass drum on every beat |
| `a(7,12)` in 12/8 | `a-a a-a -aa -a-` | a bell of West African music |

A third number **turns** the rhythm: `o(3,8,2)` is the tresillo two steps
later, as `.rotate 2` would turn it. Listen to the bell in 12/8, with the
surdo marking the four beats:

```aratu
meter: 12/8
tempo: 70
agogo = a(7,12)
surdo = O m O m
```

> Change `a(7,12)` to `a(7,12,1)` or to `a(5,12)` while it plays: another
> bell of the same family.

## Exercises

Write the baião triangle: closed and open in eighth notes, on all four
beats.

```aratu exercise
triangle = - - - -
```

```aratu answer
triangle = fo fo fo fo
```

The samba surdo: muted on beat 1 and open, accented, on beat 2.

```aratu exercise
meter: 2/4
surdo = o o
```

```aratu answer
meter: 2/4
surdo = m O
```

The pandeiro in sixteenths: thumb, fingers, slap, fingers, on each beat.

```aratu exercise
meter: 2/4
pandeiro = pppp pppp
```

```aratu answer
meter: 2/4
pandeiro = pdtd pdtd
```

Finish the baião: the zabumba with the open stroke on beat 1 and the muted
one on the second half of beat 2, and the bacalhau on beat 3.

```aratu exercise
zabumba = | - - - - |
```

```aratu answer
zabumba = | o -m b - |
```

The tresillo in one word: three open zabumba strokes spread over the eight
eighths of the bar.

```aratu exercise
zabumba = - - - -
```

```aratu answer
zabumba = o(3,8)
```

Now turn the tresillo, so its three strokes come two steps later.

```aratu exercise
zabumba = o(3,8)
```

```aratu answer
zabumba = o(3,8,2)
```
