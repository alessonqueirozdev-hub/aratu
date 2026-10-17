# Compases compuestos e irregulares

Hasta aquí el tiempo era siempre una negra, y un compás tenía dos, tres o
cuatro de ellos. En esta lección el tiempo cambia de tamaño: en 6/8 se
divide en tres, en 7/8 los tiempos tienen tamaños distintos, y un tresillo
puede atravesar dos tiempos.

## 6/8: dos tiempos, cada uno dividido en tres

En un compás de 6/8 hay seis corcheas, pero el músico no cuenta hasta seis:
cuenta **dos** tiempos, cada uno con tres corcheas, "UNO-dos-tres,
DOS-dos-tres". Aratu cuenta de la misma manera. Cada palabra es un tiempo,
una negra con puntillo, y `ccc` son sus tres corcheas:

```aratu
meter: 6/8
tempo: 66
hh = ccc ccc
bd = b__ b__
sd = - s
```

El bombo toca al principio de cada tiempo y sostiene las otras dos corcheas
(`b__`); la caja marca el segundo tiempo. Fíjate en que el tempo cuenta
tiempos, no corcheas: `tempo: 66` son 66 negras con puntillo por minuto.

> En español la indicación también se escribe con `compás:`:
> `compás: 6/8`.

El 12/8 es lo mismo, con cuatro tiempos: es el compás del blues lento y de
muchas baladas.

```aratu
meter: 12/8
tempo: 60
hh = ccc ccc ccc ccc
bd = b__ -   b_b -
sd = -   s   -   s
```

## 5/4: cinco tiempos

Un compás puede tener cinco tiempos, como en "Take Five", el tema de jazz
más famoso en este compás. La regla no cambia: cinco palabras.

```aratu
meter: 5/4
tempo: 160
hh = cc cc cc cc cc
bd = b  -  -  b  -
sd = -  -  s  -  s
```

## 7/8: tiempos de tamaños distintos

En los compases irregulares, comunes en la música de los Balcanes y en el
rock progresivo, las corcheas se agrupan en tiempos desiguales. Di cómo,
después de un `=`: `meter: 7/8 = 2+2+3` son dos tiempos de dos corcheas y
uno de tres. Cuenta "UNO-dos, UNO-dos, UNO-dos-tres".

Cada palabra es un grupo. Un grupo de dos corcheas se escribe `cc`, uno de
tres, `ccc`:

```aratu
meter: 7/8 = 2+2+3
tempo: 240
hh = cc cc ccc
bd = b_ b_ b__
sd = -  -  -s-
```

Aquí el tempo cuenta corcheas: `tempo: 240` son 240 corcheas por minuto.
También se puede escribir una palabra por corchea, siete palabras, si así
se lee mejor: `hh = c c c c c c c`.

> Cambia a `meter: 7/8 = 3+2+2` y ajusta las líneas: el mismo compás, con
> otro balanceo.

## Tresillos que atraviesan tiempos

Un tresillo de corcheas cabe en un tiempo (`ccc`). Pero existe el tresillo
de **negras**: tres notas iguales en el espacio de dos tiempos. Un número
antes de los corchetes dice cuántos tiempos ocupa el grupo:

```aratu
tempo: 80
melodia = | 2[do4 re mi] fa _ | sol _ _ _ |
hh = c c c c
```

Escucha el hi-hat: las tres notas de la melodía se reparten sobre los dos
primeros tiempos, sin coincidir con el segundo. `3[c c c c]` son cuatro
notas en tres tiempos.

## Ejercicios

Un compás de 6/8 con el hi-hat en las seis corcheas y el bombo al principio
de cada tiempo.

```aratu exercise
meter: 6/8
hh = - -
bd = - -
```

```aratu answer
meter: 6/8
hh = ccc ccc
bd = b__ b__
```

Escribe un compás de 7/8 agrupado en 2+2+3, con el hi-hat en todas las
corcheas y el bombo al principio de cada grupo.

```aratu exercise
meter: 7/8 = 2+2+3
hh = - - -
bd = - - -
```

```aratu answer
meter: 7/8 = 2+2+3
hh = cc cc ccc
bd = b_ b_ b__
```

Ahora 7/8 en 3+2+2: el grupo de tres va primero.

```aratu exercise
meter: 7/8 = 3+2+2
hh = - - -
bd = - - -
```

```aratu answer
meter: 7/8 = 3+2+2
hh = ccc cc cc
bd = b__ b_ b_
```

Un tresillo de negras en los dos primeros tiempos (do4, re4, mi4) y una
blanca de fa4.

```aratu exercise
melodia = | do4 re mi fa |
```

```aratu answer
melodia = | 2[do4 re mi] fa _ |
```
