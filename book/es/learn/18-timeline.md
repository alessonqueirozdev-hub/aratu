# Cambios en el lugar justo

En la lección anterior la música cambiaba cuando tocabas el texto. Muchas
veces, sin embargo, el cambio ya está decidido de antemano: el redoble en
el compás 8, la batería que sale en el estribillo, el bajo que cambia en la
segunda parte. Esta lección muestra cómo marcar cada cambio en el compás, e
incluso en el tiempo, en que debe ocurrir.

## En el compás: `at bar`

Una línea que empieza con `at bar 9:` (o `en el compás 9:`) ocurre en el
compás 9. Sin eso, vale desde el principio de la pieza.

```aratu
tempo: 110
pattern groove { hh = cc cc cc cc; bd = b - b -; sd = - s - s }
pattern fill   { sd = ss ss SS SS; bd = b - - - }
play groove
at bar 4: play fill
at bar 5: play groove
```

El groove suena en los compases 1, 2 y 3; el redoble en el 4; el groove
vuelve en el 5.

## Una sola vez: `once`

Volver al groove después del redoble es tan común que tiene un atajo. Con
`once` al final, el patrón suena una vez y lo que estaba antes vuelve solo:

```aratu
tempo: 110
pattern groove { hh = cc cc cc cc; bd = b - b -; sd = - s - s }
pattern fill   { sd = ss ss SS SS; bd = b - - - }
play groove
at bar 4: play fill once
```

Tocando en vivo, `now: play fill once` (o `ahora:`) trae el redoble ya en
el tiempo siguiente, y el groove vuelve en el compás en que estaría.

## En un tiempo del compás: `beat`

Un redoble muchas veces empieza en medio del compás. `at bar 4 beat 3:`
(o `en el compás 4 tiempo 3:`) entra en el tercer tiempo del compás 4:

```aratu
tempo: 110
pattern groove { hh = cc cc cc cc; bd = b - b -; sd = - s - s }
pattern fill   { sd = ss ss SS SS; bd = b - - - }
play groove
at bar 4 beat 3: play fill once
```

Escucha: los dos primeros tiempos del compás 4 todavía son del groove, y el
redoble ocupa los dos últimos. `beat 2.5` cae a mitad de camino entre el
tiempo 2 y el 3.

## Por un fragmento: `from bar … to …`

`from bar 5 to 6:` (o `del compás 5 al 6:`) vale solo en esos compases. En
el compás 7 vuelve lo que valía antes, sin que escribas nada:

```aratu
tempo: 110
drums { hh = cc cc cc cc; bd = b - b -; sd = - s - s }
bajo = | do2 _ sol1 _ |
from bar 3 to 4: drums: out
```

La batería sale en los compases 3 y 4 y vuelve en el 5. Sirve para todo:
dinámica, tempo, patrones, partes que entran y salen.

## Una línea que cambia en un compás

También la música misma de una línea puede cambiar en un compás. La línea
nueva empieza desde su principio, en el compás marcado:

```aratu
tempo: 100
bajo = | do2 _ sol1 _ | la1 _ mi1 _ |
hh = cc cc cc cc
at bar 5: bajo = | fa1 _ fa2 _ | sol1 _ sol2 _ |
```

En los compases 1 a 4 el bajo toca la primera línea, dos veces; del 5 en
adelante, la segunda. Así se escribe una canción con partes distintas sin
necesidad de secciones.

## Duraciones en tiempos

Todo cambio que lleva tiempo (`cresc`, `rit`, `accel`, rampas de tempo)
acepta duraciones en compases (`bars`), tiempos (`beats`), corcheas
(`eighths`) o semicorcheas (`sixteenths`):

```aratu
tempo: 90
melodia = | do4 re mi fa | sol _ _ _ |
at bar 2: melodia: cresc to ff over 3 beats
```

En español: `en 3 tiempos`, `en 8 corcheas`.

## Ejercicios

Haz que el redoble suene una vez, en el compás 3, y que el groove vuelva
solo.

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

Ahora el redoble empieza en el tiempo 3 del compás 2.

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

Saca la batería solo en los compases 2 y 3, con una sola línea.

```aratu exercise bars=4
drums { hh = c c c c }
bajo = | do2 _ _ _ |
```

```aratu answer
drums { hh = c c c c }
bajo = | do2 _ _ _ |
from bar 2 to 3: drums: out
```

A partir del compás 3, el bajo pasa a tocar `| fa2 _ do2 _ |`.

```aratu exercise bars=4
bajo = | do2 _ sol1 _ |
```

```aratu answer
bajo = | do2 _ sol1 _ |
at bar 3: bajo = | fa2 _ do2 _ |
```
