# Repeticiones y saltos

Los signos de repetición de la partitura también se escriben en Aratu.

## Barras de repetición

`|:` y `:|` encierran un pasaje que se toca dos veces:

```aratu
tempo: 110
melodia = |: do4 mi sol mi | re fa la fa :| do4 _ _ _ |
```

Escucha: los dos primeros compases dos veces, después el final. Para
tocarlo más veces, escribe cuántas después de `:|`: `:| x4`.

## Casillas de primera y segunda vez

`|1` empieza la primera casilla y `:|2` la segunda:

```aratu
chords = |: C | F |1 G | G :|2 C | C |
```

La primera vez: C, F, G, G. La segunda: C, F, C, C.

## Repetir el compás anterior

`%` repite el compás anterior; `%%` repite los dos anteriores:

```aratu
chords = | Am | % | F | % |
```

## D.C., D.S., fine y coda

`D.C.` vuelve al principio y `D.S.` vuelve al signo `segno:`. `al fine` se
detiene en el compás marcado con `fine:`:

```aratu
tempo: 110
melodia = | do4 mi sol do5 | segno: re4 fa la _ | fine: do4 _ _ _ | sol4 _ _ _ D.S. al fine |
```

Orden de ejecución: compases 1, 2, 3, 4, de vuelta al 2, 3, y fin.

> Si un `D.S.` no tiene `segno:`, Aratu lo dice exactamente así, con el
> número de compás.

## Ejercicios

Repite los dos primeros compases y termina en el tercero.

```aratu exercise
melodia = | do4 mi sol mi | re fa la fa | do4 _ _ _ |
```

```aratu answer
melodia = |: do4 mi sol mi | re fa la fa :| do4 _ _ _ |
```

Usa `%` para que Am dure dos compases y F también, en este orden: Am, Am,
F, F.

```aratu exercise
chords = | Am | F |
```

```aratu answer
chords = | Am | % | F | % |
```
