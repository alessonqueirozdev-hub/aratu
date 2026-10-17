# Expresión: swing, rubato y humanize

Una computadora toca cada nota en el lugar exacto. Un músico, no: balancea
las corcheas, respira en una frase, nunca golpea dos veces igual. Tres
palabras le dan esa vida a lo que escribes, sin cambiar ninguna nota.

## Swing

En el jazz, el samba-jazz y el blues, las corcheas no son iguales: la
primera es más larga y la segunda llega un poco tarde. Es el **swing**.
Escribe cuánto:

```aratu
swing 62%
tempo: 110
hh = cc cc cc cc
bd = b - b -
sd = - s - s
```

El porcentaje dice dónde cae la segunda corchea dentro del tiempo. 50% es
recto, las dos iguales. 66% es la sensación de tresillo, `c_c`, el swing más
pesado. Entre uno y otro queda el balanceo que quieras.

> Arrastra la perilla del `62%` con la música sonando, de 50 a 70, y escucha
> cómo el hi-hat empieza recto y se va balanceando.

El swing también vale para una sola parte: `bajo: swing 60%`.

## Rubato

**Rubato** quiere decir tiempo robado. El músico estira un tiempo y encoge
otro, pero llega a la barra de compás en el lugar justo: el tiempo robado
se devuelve. Es la manera en que un pianista toca una melodía lenta.

```aratu
tempo: 70
melodia = | mi4 re do re | mi mi mi _ |
bd = b - - -
melodia: rubato 20%
```

Escucha: la melodía respira, pero el bombo sigue golpeando al principio de
cada compás, junto con ella. `all: rubato 10%` (o `todos:`) curva la pieza
entera a la vez, como un director de orquesta.

## Humanize

`humanize` agrega pequeñas desviaciones, de tiempo y de fuerza, como las de
un músico de verdad. Cada golpe sale un poco distinto:

```aratu
humanize
tempo: 100
hh = cc cc cc cc
bd = b - b -
```

`humanize 5 ms` dice el tamaño de la desviación. Las desviaciones se
sortean como en el azar: con la misma semilla, la misma pieza suena siempre
igual.

## Ejercicios

Estos ejercicios comprueban también **cuándo** llega cada nota, no solo
cuál es.

Pon un swing de 66% en este ritmo.

```aratu exercise timing=1
hh = cc cc cc cc
bd = b - b -
```

```aratu answer
swing 66%
hh = cc cc cc cc
bd = b - b -
```

El swing está demasiado pesado: déjalo en 58%.

```aratu exercise timing=1
swing 66%
hh = cc cc cc cc
```

```aratu answer
swing 58%
hh = cc cc cc cc
```

Haz que la melodía respire con un 20% de rubato, mientras el bombo sigue el
pulso.

```aratu exercise timing=1
melodia = | mi4 re do re |
bd = b - - -
```

```aratu answer
melodia = | mi4 re do re |
bd = b - - -
melodia: rubato 20%
```
