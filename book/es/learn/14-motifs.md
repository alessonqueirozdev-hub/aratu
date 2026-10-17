# Motivos, secuencias y series

Mucha música está hecha de pocas ideas usadas muchas veces: un motivo que
vuelve, que sube o baja, que se pone de cabeza. Aratu te deja escribir la
idea una vez y usarla cuantas veces quieras.

## Repetir un tiempo

Una `x` pegada al corchete repite ese tiempo. Cada repetición ocupa su
propio tiempo:

```aratu
tempo: 100
melodia = | [mi4 re do re]x4 |
```

Es lo mismo que escribir `[mi4 re do re]` cuatro veces. La `x` va siempre
pegada al `]`; separada por un espacio, `| - x8 |` sigue queriendo decir
ocho compases de silencio. El número también puede ir antes:
`[mi4 re do re]4x`.

> Las llaves `{ }` son para el azar: `{mi4|sol4}` sortea una de las notas en
> cada vuelta. Para agrupar, usa corchetes. Si escribes `{la2 mi3 la mi}x4`,
> sin `|` dentro, Aratu lo toca como un grupo y avisa que cambies las llaves
> por corchetes.

## Motivos

Un motivo es un trozo con nombre. Defínelo con `motif` y usa el nombre en
lugar de los tiempos:

```aratu
tempo: 100
motif riff = [mi4 re do re]
melodia = | riff riff riff riff |
```

> El motivo puede estar escrito antes o después de la línea que lo usa.
> Mientras suena, el editor enciende las notas en la definición del motivo.

## Secuencias

En teoría musical, una **secuencia** es un motivo repetido en otra altura.
`riff+1` es el motivo un grado más arriba, dentro de la tonalidad (`key:`),
y `riff-1` un grado más abajo:

```aratu
tempo: 100
key: C major
motif riff = [mi4 re do re]
melodia = | riff riff+1 riff+2 riff+3 |
```

Por grados, el motivo sigue dentro de la escala: de mi a fa es un grado,
como de do a re. Para contar en semitonos, usa `st`: `riff+3st` sube tres
semitonos.

## Inversión y retrógrado

| Escribe | El motivo |
|---|---|
| `riff.inv` | invertido: cada intervalo de cabeza, a partir de la primera nota |
| `riff.retro` | retrógrado: las notas de atrás hacia adelante, en el mismo ritmo |
| `riff.ri` | retrógrado de la inversión |

```aratu
tempo: 100
motif riff = [mi4 re do re]
melodia = | riff riff.inv riff.retro riff.ri |
```

También se combinan con la transposición: `riff.ri+2` es el retrógrado de
la inversión dos grados más arriba.

## Ostinato

Un ostinato es un motivo que se repite abajo mientras el resto cambia. En
Aratu toda línea ya se repite sola, así que basta un compás:

```aratu
tempo: 100
motif ost = [la2 mi3 la3 mi3]
bajo = | ost ost ost ost |
piano {
  chords = | Am | F | C | G |
}
```

## Transformar una línea entera

Las mismas transformaciones valen para una línea entera, sin reescribirla.
Escribe el nombre de la línea, un punto y el verbo:

```aratu
tempo: 100
melodia = | do4 re mi sol | la sol mi re |
melodia.transpose +7
```

| Escribe | Qué hace |
|---|---|
| `melodia.transpose +7` | sube siete semitonos (`-12` baja una octava) |
| `melodia.inv`, `.retro`, `.ri` | invierte, de atrás hacia adelante, las dos cosas |
| `melodia.augment` | duplica las figuras: todo queda dos veces más lento |
| `melodia.diminish` | divide las figuras por la mitad |
| `hh.rotate 1` | desplaza el patrón una parte |
| `hh.density 50%` | toca solo cerca de la mitad de los golpes |
| `drums.velocity 0.8` | toca más suave |

En español: `transponer`, `aumentar`, `disminuir`, `rotar`, `densidad`,
`intensidad`. Borra la línea de la transformación y la melodía vuelve a ser
como fue escrita.

## Series dodecafónicas

En la música dodecafónica, una **serie** ordena las doce notas de la escala
cromática, cada una una vez. Escribe la serie como un vector de números de
0 a 11 (0 es do, 1 es do#, …) o con nombres de nota:

```aratu
tempo: 80
row serie = 0 11 3 4 8 7 9 5 6 1 2 10
melodia = | o o o o | o o o o | o o o o | from serie.P0
```

La serie da las alturas y el ritmo da los lugares: cada `o` recibe la
siguiente nota de la serie. Si el ritmo tiene más `o` que notas la serie,
esta vuelve a empezar. La serie tiene cuatro formas, cada una en doce
transposiciones:

| Forma | Qué es |
|---|---|
| `serie.P0` | la serie original (P de *prime*) |
| `serie.I0` | la inversión, desde la misma primera nota |
| `serie.R0` | el retrógrado, de atrás hacia adelante |
| `serie.RI0` | el retrógrado de la inversión |
| `serie.P5`, `serie.RI3` | la forma transpuesta 5 o 3 semitonos arriba |

`octave 3` al final pone todas las notas en la octava 3, y `from [0 4 7]`
usa un vector escrito ahí mismo. Una serie de doce notas que repite alguna
recibe un aviso. La pestaña **Series**, junto a los mensajes en Tocar,
muestra la matriz 12×12 de cada serie: haz clic en el nombre de una forma
para escribirlo en el código.

## Listas de melodías

Para preparar una presentación, guarda las melodías en una lista, una por
línea, y elige por el número:

```aratu
tempo: 100
list temas {
  | mi4 re do re | mi mi mi _ |
  | re4 re re _ | mi sol sol _ |
  | mi4 re do re | mi mi mi mi |
}
melodia = temas.1
```

> Cambia `temas.1` por `temas.2` mientras suena: el cambio entra en el
> compás siguiente. `temas.all` las toca todas, en orden, y `temas.random`
> sortea una entera en cada vuelta.

En el modo escenario aparecen botones para elegir la melodía de cada lista,
sin escribir nada frente al público, y Alt+1 a Alt+9 hacen lo mismo con el
teclado.

### Una melodía por compás

Cada compás de una línea también puede ser una melodía de la lista: escribe
el nombre en lugar de las notas de ese compás.

```aratu
tempo: 100
list temas {
  | la2 mi3 la mi |
  | la2 fa3 la fa |
  | sol2 do3 sol do |
}
bajo = | temas.1 | temas.1 | temas.2 | temas.3 |
```

Se puede mezclar con compases escritos, `| do4 _ _ _ | temas.2 |`, y
`temas.all` en lugar de un compás pone todas, una tras otra.

## Ejercicios

Repite el primer tiempo en los cuatro tiempos del compás, con `x`.

```aratu exercise
melodia = | [mi4 re do re] - - - |
```

```aratu answer
melodia = | [mi4 re do re]x4 |
```

Haz que el motivo suba un grado en cada tiempo, como una secuencia.

```aratu exercise
motif riff = [do4 re mi]
melodia = | riff riff riff riff |
```

```aratu answer
motif riff = [do4 re mi]
melodia = | riff riff+1 riff+2 riff+3 |
```

Transpón la melodía entera una quinta arriba (siete semitonos), con una
sola línea.

```aratu exercise
melodia = | do4 re mi sol |
```

```aratu answer
melodia = | do4 re mi sol |
melodia.transpose +7
```

Toca la serie de atrás hacia adelante: la forma retrógrada.

```aratu exercise
row serie = 0 11 3 4 8 7 9 5 6 1 2 10
melodia = | o o o o | o o o o | o o o o | from serie.P0
```

```aratu answer
row serie = 0 11 3 4 8 7 9 5 6 1 2 10
melodia = | o o o o | o o o o | o o o o | from serie.R0
```

Arma el bajo con las melodías 1, 3 y 2 de la lista, una en cada compás.

```aratu exercise bars=3
list temas {
  | la2 mi3 la mi |
  | la2 fa3 la fa |
  | sol2 do3 sol do |
}
bajo = | temas.1 | - | - |
```

```aratu answer
list temas {
  | la2 mi3 la mi |
  | la2 fa3 la fa |
  | sol2 do3 sol do |
}
bajo = | temas.1 | temas.3 | temas.2 |
```
