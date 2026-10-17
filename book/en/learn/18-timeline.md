# Changes in the right place

In the last lesson the music changed when you changed the text. Often,
though, the change is decided ahead of time: the fill in bar 8, the drums
dropping out in the chorus, the bass changing in the second part. This
lesson shows how to mark each change at the bar, and even the beat, where
it should happen.

## At a bar: `at bar`

A line that starts with `at bar 9:` happens at bar 9. Without it, the line
holds from the start of the piece.

```aratu
tempo: 110
pattern groove { hh = cc cc cc cc; bd = b - b -; sd = - s - s }
pattern fill   { sd = ss ss SS SS; bd = b - - - }
play groove
at bar 4: play fill
at bar 5: play groove
```

The groove plays in bars 1, 2 and 3; the fill in bar 4; the groove comes
back in bar 5.

## Just once: `once`

Going back to the groove after a fill is so common that it has a shortcut.
With `once` at the end, the pattern plays one time and whatever was playing
before comes back by itself:

```aratu
tempo: 110
pattern groove { hh = cc cc cc cc; bd = b - b -; sd = - s - s }
pattern fill   { sd = ss ss SS SS; bd = b - - - }
play groove
at bar 4: play fill once
```

Playing live, `now: play fill once` brings the fill in at the very next
beat, and the groove comes back at the bar where it would have been.

## On a beat of the bar: `beat`

A fill often starts in the middle of a bar. `at bar 4 beat 3:` comes in on
the third beat of bar 4:

```aratu
tempo: 110
pattern groove { hh = cc cc cc cc; bd = b - b -; sd = - s - s }
pattern fill   { sd = ss ss SS SS; bd = b - - - }
play groove
at bar 4 beat 3: play fill once
```

Listen: the first two beats of bar 4 still belong to the groove, and the
fill takes the last two. `beat 2.5` falls halfway between beats 2 and 3.

## For a stretch: `from bar … to …`

`from bar 5 to 6:` holds for those bars only. At bar 7 what held before
comes back, without you writing anything:

```aratu
tempo: 110
drums { hh = cc cc cc cc; bd = b - b -; sd = - s - s }
bass = | c2 _ g1 _ |
from bar 3 to 4: drums: out
```

The drums drop out in bars 3 and 4 and come back in bar 5. It works for
everything: dynamics, tempo, patterns, parts coming in and going out.

## A line that changes at a bar

A line's own music can change at a bar too. The new line starts from its
beginning, at the marked bar:

```aratu
tempo: 100
bass = | c2 _ g1 _ | a1 _ e1 _ |
hh = cc cc cc cc
at bar 5: bass = | f1 _ f2 _ | g1 _ g2 _ |
```

In bars 1 to 4 the bass plays the first line, twice; from bar 5 on, the
second. That is how to write a piece with different parts without needing
sections.

## Lengths in beats

Every change that takes time (`cresc`, `rit`, `accel`, tempo ramps)
accepts lengths in bars (`bars`), beats (`beats`), eighth notes (`eighths`)
or sixteenths (`sixteenths`):

```aratu
tempo: 90
melody = | c4 d e f | g _ _ _ |
at bar 2: melody: cresc to ff over 3 beats
```

## Exercises

Make the fill play once, at bar 3, and the groove come back by itself.

```aratu exercise bars=4
pattern groove { hh = cc cc cc cc }
pattern fill   { sd = ss ss SS SS }
play groove
```

```aratu answer
pattern groove { hh = cc cc cc cc }
pattern fill   { sd = ss ss SS SS }
play groove
at bar 3: play fill once
```

Now the fill starts on beat 3 of bar 2.

```aratu exercise bars=3
pattern groove { hh = cc cc cc cc }
pattern fill   { sd = ss ss SS SS }
play groove
at bar 2: play fill once
```

```aratu answer
pattern groove { hh = cc cc cc cc }
pattern fill   { sd = ss ss SS SS }
play groove
at bar 2 beat 3: play fill once
```

Take the drums out in bars 2 and 3 only, with a single line.

```aratu exercise bars=4
drums { hh = c c c c }
bass = | c2 _ _ _ |
```

```aratu answer
drums { hh = c c c c }
bass = | c2 _ _ _ |
from bar 2 to 3: drums: out
```

From bar 3 on, the bass plays `| f2 _ c2 _ |`.

```aratu exercise bars=4
bass = | c2 _ g1 _ |
```

```aratu answer
bass = | c2 _ g1 _ |
at bar 3: bass = | f2 _ c2 _ |
```
