# El compás y el tempo

Toda pieza de Aratu empieza diciendo dos cosas: en qué **compás** está y a
qué **tempo** va.

```aratu
meter: 4/4
tempo: 90

hh = c c c c
bd = b - b -
```

`meter: 4/4` es la indicación de compás: cuatro tiempos por compás.
`tempo: 90` es el tempo: noventa pulsos por minuto, como en un metrónomo.
Mira la perilla redonda junto al `90`: arrástrala hacia arriba o hacia
abajo y el número cambia. La música acelera o se frena en el compás
siguiente.

## La primera regla: un espacio separa los tiempos

Lee la línea del hi-hat contando en voz alta: "uno, dos, tres, cuatro".
Cada grupo de símbolos separado por un espacio es **un tiempo**. En un
compás de 4/4 hay cuatro grupos.

Si un compás tiene tiempos de más o de menos, Aratu avisa. Este ejemplo
tiene un tiempo de más. Lee el mensaje debajo y corrígelo:

```aratu broken
meter: 4/4
bd = b - b - b
```

## Varios compases en una línea

La barra `|` separa los compases, como en la partitura. Esta línea de
bombo tiene dos compases distintos, y el par se repite:

```aratu
bd = | b - b - | b b b - |
sd = | - s - s | - s - s |
```

En la cuadrícula, cada compás aparece en una caja, con los tiempos
numerados arriba.

## Otras indicaciones de compás

En un vals hay tres tiempos por compás. Cuenta "uno, dos, tres" mientras
escuchas:

```aratu
meter: 3/4
tempo: 150

bd = b - -
hh = - c c
```

En 6/8 el tiempo es una negra con puntillo y se cuenta en dos, como cuenta
un músico. Lo verás en la próxima lección.

## Ejercicios

Escribe dos compases de bombo: en el primero, el bombo toca en los tiempos
1 y 3; en el segundo, en los cuatro tiempos.

```aratu exercise
bd = | - - - - | - - - - |
```

```aratu answer
bd = | b - b - | b b b b |
```

Ahora un vals en 3/4: bombo en el tiempo 1, caja en los tiempos 2 y 3.

```aratu exercise
meter: 3/4
bd = - - -
sd = - - -
```

```aratu answer
meter: 3/4
bd = b - -
sd = - s s
```
