# Cifrados y rasgueos de guitarra

Una secuencia de acordes se escribe como en un cifrado, en una línea
llamada `chords` (o `acordes`, o `cifra`). Cada compás entre barras tiene
su acorde:

```aratu
tempo: 90
chords = | Am | F | C | G |
```

Sola, la línea de acordes los toca enteros, como un teclado.

## Acordes que dividen el compás

Dos acordes en el mismo compás lo dividen por la mitad. Para otras
divisiones, usa `_` para decir cuánto dura cada acorde:

```aratu
chords = | C G | Am _ _ F | C |
```

En el segundo compás, Am dura tres tiempos y F uno.

Los acordes se escriben como siempre: `Am`, `G7`, `Cmaj7` (o `C7M`),
`Dm7`, `Bº`, `E/G#`, `G7(b9)`. También con do-re-mi, y un compás sin acorde
es `N.C.`:

```aratu
chords = | Lam | Fa | Do Sol | N.C. |
```

## El rasgueo de la guitarra

Una línea llamada `strum` dice cómo la mano golpea las cuerdas, con las
mismas reglas de ritmo:

| Símbolo | Rasgueo |
|---|---|
| `d` | hacia abajo |
| `u` | hacia arriba |
| `m` | apagado |
| `g` | golpe: los dedos golpean la tapa, sin tocar las cuerdas |
| `t` | hacia abajo con golpe, todo junto |
| `-` | la mano pasa sin tocar |
| mayúscula | acentuado |

La línea también puede llamarse `rasgueo` o `rasgueado`. Pon las dos líneas
en un bloque con el nombre del instrumento:

```aratu
tempo: 90
guitarra {
  chords = | Am | F | C | G |
  strum  = D du -u du
}
```

El rasgueo se repite en cada compás y los acordes cambian solos. En el
rasgueo hacia abajo las cuerdas suenan de la grave a la aguda; hacia
arriba, de la aguda a la grave, como en una guitarra de verdad.

> Cambia el rasgueo a `D -u D -u` y escucha la diferencia, con la música
> sonando.

## El golpe en la tapa

En el flamenco la mano también golpea la tapa de la guitarra: el *golpe*.
Se escribe `g`, y `G` es un golpe acentuado. En el golpe no suenan las
cuerdas; suena la madera:

```aratu
tempo: 100
guitarra {
  chords = | Am | G | F | E |
  strum  = dgdu dgdu dGdu dgdu
}
```

> Cada tiempo tiene cuatro semicorcheas: hacia abajo, golpe, hacia abajo,
> hacia arriba. El golpe del tercer tiempo va acentuado.

Muchas veces el golpe cae **junto** con el rasgueo hacia abajo: la mano
baja por las cuerdas y el dedo anular golpea la tapa en el mismo instante.
Se escribe `t`, de *todo junto*, y `T` es lo mismo con acento:

```aratu
tempo: 100
guitarra {
  chords = | Am | G | F | E |
  strum  = T-du g-du T-du g-du
}
```

> En la próxima lección los acordes se escriben por grado, en números
> romanos: `I`, `vi`, `IV`, `V7`.

## Ejercicios

Escribe la secuencia C, Am, F, G, un acorde por compás.

```aratu exercise
guitarra {
  chords = | C | C | C | C |
  strum  = d d d d
}
```

```aratu answer
guitarra {
  chords = | C | Am | F | G |
  strum  = d d d d
}
```

Ahora cambia el rasgueo a `D du D du`.

```aratu exercise
guitarra {
  chords = | Am | Dm | E7 | Am |
  strum  = d d d d
}
```

```aratu answer
guitarra {
  chords = | Am | Dm | E7 | Am |
  strum  = D du D du
}
```
