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
| `t` | tom agudo |
| `m` | tom medio |
| `f` | tom de piso |
| `k` | aro (baqueta cruzada sobre la caja) |

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
  rd = rr rr rr rr
  bd = b - b -
  sd = - s - s
}
```

> Como el sonido sale del símbolo, las letras de platillo van en la línea
> `cr` y el ride en la línea `rd`, pero cualquier nombre serviría.

## Toms y aro

Los toms hacen los redobles: `t` es el tom agudo, `m` el tom medio y `f`
el tom de piso. Aquí el segundo compás termina bajando por los tres:

```aratu
tempo: 96

drums {
  hh  = | cc cc cc cc | cc cc -  -  |
  bd  = | b  -  bb -  | b  -  -  -  |
  sd  = | -  s  -  s  | -  s  -  -  |
  tom = | -  -  -  -  | -  -  tm ff |
}
```

El aro, `k`, es la baqueta cruzada sobre la caja: un sonido seco, de
madera, sin la bordonera. Es el golpe de la bossa nova:

```aratu
tempo: 130

drums {
  hh  = cc cc cc cc
  bd  = b -b b -b
  rim = | k- -k - k | - k -k - |
}
```

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

Termina el compás con un redoble: en el tiempo 4, una corchea de tom
agudo y una de tom de piso.

```aratu exercise
drums {
  sd  = s s s -
  tom = - - - -
}
```

```aratu answer
drums {
  sd  = s s s -
  tom = - - - tf
}
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
