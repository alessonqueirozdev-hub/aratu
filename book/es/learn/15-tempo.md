# Tempo: rit, accel, calderón y anacrusa

## Cambios de tempo

El tempo puede cambiar en un compás marcado con `at bar`:

```aratu
tempo: 100
at bar 3: rit to 60 over 2 bars
hh = c c c c
```

`rit to 60 over 2 bars` es un ritardando hasta 60, a lo largo de dos
compases (`rall` también vale). Lo contrario es `accel to 140 over 4 bars`.
`a tempo` vuelve al tempo de antes del rit, y `tempo primo` vuelve al
primer tempo de la pieza.

Durante un ritardando el tempo cambia poco a poco, tiempo a tiempo, como se
lee en una partitura, y cada nota cae en el momento exacto.

Un tempo nuevo, de golpe, en un compás: `at bar 9: tempo 130`. Y una rampa
entre dos tempos cualesquiera, desde el principio:

```aratu
tempo 90 -> 140 over 8 bars
hh = cc cc cc cc
bd = b - b -
```

La duración también puede ir en tiempos: `rit to 60 over 6 beats`.

## Calderón

`^` después de una nota es un calderón: la nota dura el doble, y todas las
partes la esperan. Con un número, dura las veces que se escriba: `sol4^3`
dura el triple.

```aratu
tempo: 90
melodia = | do4 mi sol do5^ | sol4 _ _ _ |
hh = c c c c
```

## Anacrusa

Muchas melodías empiezan antes del primer tiempo. `pickup: beat 4` dice que
la pieza empieza en el tiempo 4 de un compás incompleto:

```aratu
tempo: 100
pickup: beat 4
melodia = | sol3 | do4 _ mi _ | sol _ _ _ |
bd = | b - b - |
```

La anacrusa es el compás 0, y el compás 1 es el primero completo, como
cuentan los músicos. Una línea cuyo primer compás mide lo mismo que la
anacrusa empieza en ella; las demás empiezan en el compás 1.

## Ejercicios

Los ejercicios de esta lección comprueban también **cuándo** llega cada
nota.

Pon un calderón en la última nota del primer compás.

```aratu exercise timing=1
melodia = | do4 re mi fa | sol _ _ _ |
```

```aratu answer
melodia = | do4 re mi fa^ | sol _ _ _ |
```

Haz que esta melodía empiece con la anacrusa `sol3` en el tiempo 4.

```aratu exercise
melodia = | do4 _ mi _ | sol _ _ _ |
```

```aratu answer
pickup: beat 4
melodia = | sol3 | do4 _ mi _ | sol _ _ _ |
```

Haz un ritardando hasta 60, a lo largo de dos compases, a partir del
compás 2.

```aratu exercise timing=1 bars=4
tempo: 100
hh = c c c c
```

```aratu answer
tempo: 100
hh = c c c c
at bar 2: rit to 60 over 2 bars
```

Ahora un calderón tres veces más largo en el sol del final.

```aratu exercise timing=1
melodia = | do4 mi sol4^ - |
```

```aratu answer
melodia = | do4 mi sol4^3 - |
```
