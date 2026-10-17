# Sections and form

A song has parts: intro, verse, chorus, ending. Each part comes back more
than once, and their order is the **form** of the piece. In Aratu you write
each part once, with a name, and then say in what order they play.

## A section

A section is a named block, written with `section`. Inside it go the parts
and the lines, as in a whole piece:

```aratu
tempo: 100
section verse {
  bass = | a2 _ e2 _ | f2 _ c3 _ |
  drums { hh = cc cc co cc; bd = b - b -; sd = - s - s }
}
form: verse
```

A block can sit on a single line, with `;` between its lines, like the
drums above. A section lasts as long as its longest line: here, two bars.

## The form

The `form:` line gives the order of the sections. `x2` plays a section
twice in a row:

```aratu
tempo: 100
section intro  { bass = | a2 _ _ _ | e2 _ _ _ | }
section verse  {
  bass = | a2 _ e2 _ | f2 _ c3 _ |
  drums { hh = cc cc co cc; bd = b - b -; sd = - s - s }
}
section chorus {
  guitar { chords = | Am | F | C | G |; strum = D du -u du }
  drums  { hh = cc cc cc cc; bd = b - b -; sd = - s - S }
}
section outro  { bass = | a2 _ _ _ | }

form: intro verse x2 chorus verse chorus outro
```

Listen to the order: intro, verse twice, chorus, verse, chorus, outro. When
it reaches the end, the form starts again, like every loop in Aratu.

> The section names are yours: `intro`, `a`, `b`, `bridge`, `solo`. Write
> them as one word, with no spaces.

## Sections and repeats together

Inside a section, the repeat signs, endings and `%` from the lesson on
repeats all work. And the form can repeat a longer stretch:

```aratu
tempo: 110
section a { melody = | c4 e g e | d f a f | }
section b { melody = | e4 g b g | c5 _ _ _ | }
form: a x2 b a
```

That is the AABA form, the form of many jazz standards and bossa nova
songs.

## Exercises

Put the sections in order: intro, verse, chorus.

```aratu exercise bars=3
section intro  { bass = | a2 _ _ _ | }
section verse  { bass = | c3 _ _ _ | }
section chorus { bass = | f2 _ _ _ | }
form: intro
```

```aratu answer
section intro  { bass = | a2 _ _ _ | }
section verse  { bass = | c3 _ _ _ | }
section chorus { bass = | f2 _ _ _ | }
form: intro verse chorus
```

Now the AABA form: section `a` twice, then `b`, then `a`.

```aratu exercise bars=4
section a { melody = | c4 _ _ _ | }
section b { melody = | g4 _ _ _ | }
form: a b
```

```aratu answer
section a { melody = | c4 _ _ _ | }
section b { melody = | g4 _ _ _ | }
form: a x2 b a
```

Write the `chorus` section, with the bass playing F2 and G2, a half note
each, for the form that is already there.

```aratu exercise bars=2
section verse  { bass = | a2 _ _ _ | }
section chorus { bass = | - | }
form: verse chorus
```

```aratu answer
section verse  { bass = | a2 _ _ _ | }
section chorus { bass = | f2 _ g2 _ | }
form: verse chorus
```
