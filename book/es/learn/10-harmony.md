# Armonía por grados

En la lección anterior los acordes tenían nombre: Am, F, C, G. Quien
estudia armonía muchas veces piensa de otra manera: no en "la menor", sino
en "el sexto grado de la tonalidad". Escribir por grados muestra la función
de cada acorde, y la misma secuencia sirve para cualquier tonalidad.

## Los grados de la tonalidad

Los grados se escriben en números romanos, de I a VII, a partir de la
tonalidad de la pieza, que va en `key:` (o `tonalidad:`). En do mayor, I
es do, IV es fa, V es sol:

```aratu
key: C major
tempo: 90
chords = | I | IV | V | I |
```

**Mayúscula es acorde mayor, minúscula es menor.** En do mayor, el segundo,
el tercero y el sexto grados son menores: `ii` es Dm, `iii` es Em, `vi` es
Am. La secuencia más tocada del pop es esta:

```aratu
key: C major
tempo: 90
chords = | I | vi | IV | V |
```

> Escribe `key: G major` en lugar de `key: C major` y escucha: la misma
> secuencia, ahora en sol. Así se cambia la tonalidad de una canción para
> que quepa en la voz de quien canta.

La tonalidad también puede escribirse en español, con los nombres do-re-mi:
`tonalidad: Sol mayor`, `tonalidad: La menor`.

## Séptimas y otros acordes

Después del grado va el resto del cifrado, como en un cifrado común: `V7`
es el acorde de séptima de la dominante, `ii7` el segundo grado con
séptima, `IV7M` el cuarto grado con séptima mayor. `vii°` es el acorde
disminuido del séptimo grado.

```aratu
key: C major
tempo: 80
chords = | ii7 | V7 | I7M | vii° |
```

El ii, V, I es la cadencia más importante del jazz y de la bossa nova.

## Tonalidades menores

En una tonalidad menor los grados cambian: en la menor, `i` es Am, `iv` es
Dm, y `V7` es E7, la dominante con la sensible:

```aratu
key: A minor
tempo: 90
chords = | i | iv | V7 | i |
```

## Acordes de fuera de la tonalidad

Un bemol o un sostenido antes del grado toma prestado un acorde de fuera
de la tonalidad: `bVII` en do mayor es Bb, el acorde que da ese sabor a
rock. Y una barra hace una dominante secundaria: `V7/V` es la dominante de
la dominante, en do mayor D7, que lleva a G.

```aratu
key: C major
tempo: 90
chords = | I | bVII | IV | I | I | V7/V | V7 | I |
```

## Arpegiar un acorde

Con `:arp` después del acorde, las notas suenan una después de otra, de
abajo hacia arriba, como un arpa o una guitarra punteada despacio. Vale
para cualquier acorde, con nombre o por grado:

```aratu
key: C major
tempo: 70
chords = | I:arp | vi:arp | IV:arp | V7:arp |
```

## El arpegiador

`:arp` arpegia un acorde una vez. Un **arpegiador** recorre las notas del
acorde una a una, en pasos, mientras dura el acorde, como el arpegiador de
un sintetizador, o los dedos de quien toca el arpa. Escribe una línea
llamada `arp` junto a los acordes, con el camino que hace y el largo de sus
pasos:

```aratu
key: A minor
tempo: 110
piano {
  chords = | i | VI | III | VII |
  arp    = up 16ths
}
```

Cada semicorchea toca la nota siguiente del acorde que está sonando, de
abajo hacia arriba, y cada acorde vuelve a empezar desde su nota más grave.
El acorde en sí no suena en bloque: lo toca el arpegiador, como el rasgueo
toca los acordes de la guitarra. Cambia los acordes y toca los nuevos;
cambia una palabra del `arp` y los mismos acordes suenan de otra manera.

| Escribe | Qué toca |
|---|---|
| `arp = up 16ths` | desde la nota más grave hacia arriba, en semicorcheas |
| `arp = down 8ths` | desde la nota más aguda hacia abajo, en corcheas |
| `arp = updown 16ths` | sube y vuelve bajando (también `up and down`) |
| `arp = random 16ths` | una de las notas del acorde en cada paso, al azar |
| `arp = up triplets` | tres pasos por tiempo; `beats`, uno por tiempo |
| `arp = up 16ths 2 octaves` | el acorde repartido en dos octavas |

```aratu
tempo: 120
piano {
  chords = | Am | F | C | G |
  arp    = updown 8ths 2 octaves
}
```

> En español la línea también se escribe `arpegio = sube 16ths`, `baja`,
> `sube y baja`, con `corcheas`, `semicorcheas`, `tresillos` y
> `2 octavas`.

## Melodías por grados

Los grados también escriben melodías. Un número en lugar de la nota es un
grado de la tonalidad: `1` es la primera nota de la tonalidad, `3` la
tercera, `5` la quinta, y el `8` es el `1` una octava arriba. Es como
muchos músicos cantan una melodía para aprenderla: "uno, tres, cinco,
tres".

```aratu
key: C major
tempo: 100
melodia = | 1 3 5 3 | 4 6 8 _ | 7 5 2 4 | 3 _ 1 _ |
```

> Cambia la tonalidad a `key: E minor` sin tocar nada más: la misma
> melodía, ahora en mi menor. En tonalidad menor, los grados siguen la
> escala menor, así que la tercera ya es menor.

Una alteración después del número sale de la escala: `4#` es la cuarta
aumentada, `7b` la séptima menor, la nota del blues. Y una coma baja una
octava: `5,` es la quinta debajo del `1`, `1,,` dos octavas abajo, para el
bajo.

```aratu
key: A minor
tempo: 96
melodia = | 1 _ [3 4] 5 | 7b _ 5 _ | 4 3 2 5, | 1 _ _ _ |
bajo    = | 1,, _ _ _ | 4,, _ _ _ | 5,, _ _ _ | 1,, _ _ _ |
```

Los grados hacen todo lo que hace una nota: corchetes (`[1 2]`), acordes
(`1+3+5`), `_`, staccato, calderón, `x4` y grupos irregulares.

## Con el rasgueo de la guitarra

Los grados funcionan en una línea de acordes de cualquier instrumento, con
rasgueo y todo:

```aratu
key: D major
tempo: 96
guitarra {
  chords = | I | V | vi | IV |
  strum  = D du -u du
}
```

## Ejercicios

Escribe I, IV, V, I en do mayor, un acorde por compás.

```aratu exercise
key: C major
chords = | I | I | I | I |
```

```aratu answer
key: C major
chords = | I | IV | V | I |
```

La misma secuencia por grados, pero ahora en sol mayor. Cambia solo la
tonalidad.

```aratu exercise
key: C major
chords = | I | vi | IV | V |
```

```aratu answer
key: G major
chords = | I | vi | IV | V |
```

La cadencia ii, V, I en do mayor: `ii7`, `V7` e `I7M`, y el último dura dos
compases.

```aratu exercise
key: C major
chords = | I | I | I | I |
```

```aratu answer
key: C major
chords = | ii7 | V7 | I7M | % |
```

En la menor: i, iv, V7, i, con el último acorde arpegiado.

```aratu exercise
key: A minor
chords = | i | i | i | i |
```

```aratu answer
key: A minor
chords = | i | iv | V7 | i:arp |
```

Haz que el arpegiador baje en vez de subir, todavía en corcheas.

```aratu exercise
chords = | Am | F | C | G |
arp    = up 8ths
```

```aratu answer
chords = | Am | F | C | G |
arp    = down 8ths
```

Escribe por grados, en do mayor: 1, 2, 3 y 5, una negra cada uno.

```aratu exercise
key: C major
melodia = | 1 _ _ _ |
```

```aratu answer
key: C major
melodia = | 1 2 3 5 |
```
