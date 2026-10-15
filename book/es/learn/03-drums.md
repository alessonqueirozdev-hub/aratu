# La batería

Cada pieza de la batería tiene un símbolo corto. El símbolo decide el
sonido, sea cual sea el nombre de la línea.

| Símbolo | Sonido |
|---|---|
| `c` | hi-hat cerrado |
| `o` | hi-hat abierto |
| `b` | bombo |
| `s` | caja |
| `r` | platillo ride |
| `x` | platillo crash |

Las líneas pueden ir juntas en un bloque `drums { }`. Así queda claro que
todas son de la batería:

```aratu
tempo: 100

drums {
  hh = cc cc cc co
  bd = b - bb -
  sd = - s - s
}
```

El hi-hat se abre en la última corchea (`co`) y el bombo dobla en el tiempo
3 (`bb`).

## Acento

Una letra **mayúscula** se toca con acento, más fuerte. Escucha la caja
acentuada en el tiempo 4:

```aratu
drums {
  hh = cc cc cc cc
  bd = b - b -
  sd = - s - S
}
```

Y un apóstrofo después del símbolo hace el sonido corto: `c'` es un hi-hat
muy seco.

## Un platillo para empezar

El crash marca el comienzo de una frase. Aquí toca en el tiempo 1 y el ride
lleva las corcheas:

```aratu
drums {
  cr = x - - -
  rd = cc cc cc cc
  bd = b - b -
  sd = - s - s
}
```

> Como el sonido sale del símbolo, las letras de platillo van en la línea
> `cr` y el ride en la línea `rd`, pero cualquier nombre serviría.

## Ejercicios

Un ritmo de rock con la caja **acentuada** en el tiempo 4.

```aratu exercise dynamics=1
drums {
  bd = b - b -
  sd = - s - s
}
```

```aratu answer
drums {
  bd = b - b -
  sd = - s - S
}
```

Abre el hi-hat en la última corchea del compás.

```aratu exercise
hh = cc cc cc cc
```

```aratu answer
hh = cc cc cc co
```

Ahora arma un ritmo completo: hi-hat en corcheas, bombo en los tiempos 1 y
3 y en la segunda mitad del tiempo 3, caja en los tiempos 2 y 4.

```aratu exercise
drums {
  hh = - - - -
  bd = - - - -
  sd = - - - -
}
```

```aratu answer
drums {
  hh = cc cc cc cc
  bd = b - bb -
  sd = - s - s
}
```
