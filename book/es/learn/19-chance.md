# El azar

Un loop que se repite siempre igual pronto suena mecánico. Un baterista
nunca toca dos compases exactamente iguales: una nota del hi-hat se pierde,
otra aparece. El azar le da esa vida al loop sin que lo toques: la
computadora decide, un poco al azar, qué tocar en cada vuelta. Es el *ad
lib* de la partitura.

## A veces

Un `?` después de un símbolo hace que suene solo a veces, la mitad de las
veces:

```aratu
tempo: 100
hh = cc cc c? cc
bd = b - b -
```

Un porcentaje dice con qué frecuencia:

```aratu
tempo: 100
hh = cc cc c?30% cc
sd = - s - s?80%
```

## Uno u otro

Las llaves eligen una entre varias opciones, separadas por `|`, en cada
vuelta del loop:

```aratu
tempo: 100
bajo = | {do2|sol1} _ do2 _ |
hh = {cc|co} cc cc cc
```

El bajo empieza a veces en do y a veces en sol; el hi-hat a veces se abre
en el primer tiempo.

## El mismo azar otra vez

Las opciones se sortean de nuevo en cada vuelta, pero a partir de una
semilla fija: la misma pieza improvisa siempre igual cuando se toca desde
el principio. `seed 7` elige otra improvisación; `seed random` hace cada
presentación distinta.

```aratu
seed 7
hh = c? c? c? c?
```

> Cambiar una línea no cambia la improvisación de las otras: cada línea
> tiene su propio sorteo.

Esta lección no tiene ejercicio para comprobar, porque el azar no tiene
respuesta correcta. Experimenta todo lo que quieras con los ejemplos.
