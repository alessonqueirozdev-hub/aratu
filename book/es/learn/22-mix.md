# La mezcla de la obra entera

Con varias partes sonando juntas, falta el trabajo de quien se sienta en la
consola: dejar cada instrumento en el volumen justo, cada uno en su lugar
entre los parlantes, todos en la misma sala. Eso es la **mezcla**.

## Volumen: quién se escucha más

Cada parte tiene un `volume`, de 0 (mudo) a 1. Una batería demasiado fuerte
tapa la melodía; bájala:

```aratu
tempo: 100
drums { hh = cc cc cc cc; bd = b - b -; sd = - s - s }
melodia: keys.epiano
melodia = | mi4 sol la sol | mi re do _ |
drums.volume = 0.5
melodia.volume = 0.9
```

> Arrastra las dos perillas con la música sonando, como si fueran los
> faders de una consola.

## Pan: dónde queda cada uno

El `pan` pone la parte entre los parlantes: -1 es todo a la izquierda, 1
todo a la derecha, 0 en el centro. Abrir los instrumentos hacia los lados
hace que cada uno se oiga más claro:

```aratu
tempo: 100
guitarra: guitar.nylon
guitarra { chords = | Am | F | C | G |; strum = D du -u du }
teclado: keys.epiano
teclado = | la4 _ do5 _ | la4 _ fa4 _ | sol4 _ mi4 _ | re4 _ sol4 _ |
guitarra.pan = -0.6
teclado.pan = 0.6
```

Escucha con auriculares: la guitarra a la izquierda, el teclado a la
derecha. El bajo y el bombo suelen quedarse en el centro.

## La misma sala para todos

En la mezcla de Aratu hay una reverb compartida, como la sala donde toca la
banda. El `reverb.mix` de cada parte dice cuánto de ella va a esa sala.

Un número escrito **sin parte** vale para todas las partes. Para poner la
obra entera en la sala, basta una línea:

```aratu
tempo: 90
melodia: piano.grand
melodia = | mi4 sol la sol | mi re do _ |
bajo: bass.finger
bajo = | do2 _ sol1 _ | la1 _ fa1 _ |
reverb.mix = 0.5
```

Una parte puede tener su propio valor, que vale en lugar del general:
`bajo.reverb.mix = 0.1` deja el bajo más seco, cerca de quien escucha.

## Efectos para todos: `todos:`

Una cadena escrita para `todos` (o `all`) pone sus efectos en todas las
partes, después de los efectos de cada una:

```aratu
tempo: 90
melodia: keys.epiano -> tremolo
melodia = | mi4 sol la sol | mi re do _ |
bajo: bass.finger
bajo = | do2 _ sol1 _ | la1 _ fa1 _ |
todos: chorus -> delay
delay.mix = 0.2
```

La melodía pasa por tremolo, chorus y delay; el bajo, por chorus y delay. Y
`delay.mix = 0.2`, sin parte, ajusta el delay de todas.

## El master

Todo termina en un último canal, el **master**: la sala compartida, un
compresor que "pega" las partes y un limitador que impide que el sonido
sature. Sus números empiezan con `master.`:

```aratu
tempo: 90
melodia: piano.grand
melodia = | mi4 sol la sol | mi re do _ |
reverb.mix = 0.5
master.reverb.decay = 4 s
master.glue.on = 1
master.limiter.ceiling = -1
```

| Número | Qué es |
|---|---|
| `master.reverb.decay` | cuánto tiempo suena la sala después de la nota, en segundos |
| `master.reverb.size` | el tamaño de la sala, de 0 a 1 |
| `master.glue.on` | 1 enciende el compresor que pega las partes |
| `master.gain` | el volumen de todo, en dB |
| `master.limiter.ceiling` | el volumen máximo, de -12 a -0,3 dB |

## Ejercicios

Baja la batería a 0.4, para que se escuche la melodía.

```aratu exercise sound=1
drums { hh = cc cc cc cc; bd = b - b - }
melodia = | mi4 sol la sol |
```

```aratu answer
drums { hh = cc cc cc cc; bd = b - b - }
melodia = | mi4 sol la sol |
drums.volume = 0.4
```

Pon la guitarra a la izquierda (-0.5) y el teclado a la derecha (0.5).

```aratu exercise sound=1
guitarra = | mi3 _ sol3 _ |
teclado = | do5 _ mi5 _ |
```

```aratu answer
guitarra = | mi3 _ sol3 _ |
teclado = | do5 _ mi5 _ |
guitarra.pan = -0.5
teclado.pan = 0.5
```

Pon la obra entera en la reverb, con 0.6, en una sola línea.

```aratu exercise sound=1
melodia = | mi4 sol la sol |
bajo = | do2 _ sol1 _ |
```

```aratu answer
melodia = | mi4 sol la sol |
bajo = | do2 _ sol1 _ |
reverb.mix = 0.6
```

Pon chorus en todas las partes, con una sola línea.

```aratu exercise sound=1
melodia: keys.epiano
melodia = | mi4 sol la sol |
bajo: bass.finger
bajo = | do2 _ sol1 _ |
```

```aratu answer
melodia: keys.epiano
melodia = | mi4 sol la sol |
bajo: bass.finger
bajo = | do2 _ sol1 _ |
todos: chorus
```

Haz que la sala del master suene durante 5 segundos.

```aratu exercise sound=1
melodia = | mi4 sol la sol |
reverb.mix = 0.5
```

```aratu answer
melodia = | mi4 sol la sol |
reverb.mix = 0.5
master.reverb.decay = 5 s
```
