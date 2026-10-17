# Dividir el tiempo

Hasta ahora cada tiempo tenía un solo símbolo, una negra. Pero la música
divide los tiempos: dos corcheas, cuatro semicorcheas, tresillos. En Aratu
eso cabe en una regla.

## La segunda regla: los símbolos juntos dividen el tiempo

Los símbolos escritos juntos, sin espacio, dividen el tiempo en partes
iguales.

| Escrito | En un tiempo |
|---|---|
| `c` | una negra |
| `cc` | dos corcheas |
| `cccc` | cuatro semicorcheas |
| `ccc` | un tresillo de corcheas |

Escucha el tiempo dividiéndose cada vez más, del 1 al 4:

```aratu
tempo: 80
hh = c cc ccc cccc
```

Mira la cuadrícula: en el tiempo 2 hay dos símbolos, en el 3 hay tres, en
el 4 hay cuatro, todos a la misma distancia entre sí.

## La tercera regla: `_` sostiene, `-` calla

Dentro de un tiempo, `_` hace que el sonido anterior dure una parte más, y
`-` es una parte de silencio. Con estas dos piezas se escriben los ritmos
más comunes:

| Ritmo en un tiempo | Escrito |
|---|---|
| semicorchea, semicorchea, corchea | `ccc_` |
| semicorchea, corchea, semicorchea | `cc_c` |
| corchea, semicorchea, semicorchea | `c_cc` |
| corchea con puntillo, semicorchea | `c__c` o `c.c` |
| silencio de corchea, corchea | `-c` |

El punto es un atajo: `c.c` es lo mismo que `c__c`, la corchea con
puntillo.

```aratu
tempo: 80
hh = ccc_ cc_c c_cc c.c
```

Un `_` solo en un tiempo sostiene el tiempo entero: `c _` es una blanca y
`c _ _ _` una redonda.

## Más de un tiempo

Un número antes de los corchetes estira el grupo por más tiempos:
`2[c c c]` son tres golpes iguales en dos tiempos, el tresillo de negras.
La lección sobre compases compuestos vuelve a esto con calma.

```aratu
tempo: 80
hh = 2[c c c] c c
```

## Ejercicios

Haz que el hi-hat toque corcheas en todo el compás.

```aratu exercise
hh = c c c c
```

```aratu answer
hh = cc cc cc cc
```

Negras en los tiempos 1 y 3, semicorcheas en los tiempos 2 y 4.

```aratu exercise
hh = c c c c
```

```aratu answer
hh = c cccc c cccc
```

Corchea con puntillo y semicorchea en todos los tiempos: el ritmo
"galopado".

```aratu exercise
hh = cc cc cc cc
```

```aratu answer
hh = c.c c.c c.c c.c
```

> Escucha el objetivo antes de escribir. Repetir el ritmo con la voz
> ("taaa-ta, taaa-ta") ayuda a encontrar la escritura.
