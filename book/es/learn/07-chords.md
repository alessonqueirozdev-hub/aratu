# Cifrados y rasgueos de guitarra

Una secuencia de acordes se escribe como en un cifrado, en una línea
llamada `chords`. Cada compás entre barras tiene su acorde:

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
`Dm7`, `Bº`, `E/G#`, `G7(b9)`.

## El rasgueo de la guitarra

Una línea llamada `strum` dice cómo la mano golpea las cuerdas, con las
mismas reglas de ritmo:

| Símbolo | Rasgueo |
|---|---|
| `d` | hacia abajo |
| `u` | hacia arriba |
| `m` | apagado |
| `-` | la mano pasa sin tocar |
| mayúscula | acentuado |

Pon las dos líneas en un bloque con el nombre del instrumento:

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
