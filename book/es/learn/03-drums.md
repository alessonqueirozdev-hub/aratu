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

Cada línea tiene también un **nombre**, escrito antes del `=`. Lo
habitual es usar las abreviaturas del inglés, las mismas de los estudios y
de las cajas de ritmo:

| Nombre de la línea | Viene de | Pieza |
|---|---|---|
| `hh` | *hi-hat* | hi-hat |
| `bd` | *bass drum* | bombo |
| `sd` | *snare drum* | caja |
| `rd` | *ride* | platillo ride |
| `cr` | *crash* | platillo crash |
| `tom` | *tom* | toms |
| `rim` | *rim* | aro |

El nombre es solo una etiqueta, para orientarte; el sonido lo decide el
símbolo. Una línea `hihat = cc cc cc cc` dentro de `drums { }` suena
igual.

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

> La línea `cr` (*crash*) lleva el platillo crash, `x`, y la línea `rd`
> (*ride*) lleva el ride, `r`. Cambia una `r` por `c` y escucha la
> diferencia entre el ride y el hi-hat.

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

## Nombres en español y palmas

Las líneas también pueden llamarse en español: `charles`, `bombo`,
`caja`, `platillo`, `aro`, `palmas`; y el bloque puede llamarse
`batería`. La `p` son las palmas:

```aratu
tempo: 100

batería {
  charles = cc cc cc cc
  bombo   = b - b -
  palmas  = - p - p
}
```

## Percusión brasileña

Cada instrumento de percusión tiene sus golpes. Una línea con el nombre
del instrumento lee esos golpes:

| Línea | Golpes |
|---|---|
| `pandeiro` (`pandero`) | `p` pulgar, `d` dedos, `t` palmada, `m` apagado |
| `zabumba` | `o` abierto, `m` apagado, `b` bacalhau (varilla) |
| `triángulo` | `o` abierto, `f` cerrado |
| `agogó` | `a` agudo, `g` grave |
| `surdo` | `o` abierto, `m` apagado |
| `tamborim`, `alfaia`, `gonguê` | `o` abierto, `m` apagado |
| `ganzá` | `s` |
| `cuíca` | `a` agudo, `g` grave |
| `cajón` | `b` grave, `t` golpe agudo |

Un baião, con zabumba, triángulo y agogó:

```aratu
tempo: 110
zabumba   = | o - -o m | o - b - |
triángulo = fo fo fo fo
agogó     = a g ag -
```

## Redoble

Una tilde, `~`, después del símbolo hace un **redoble**: el golpe se vuelve
golpes muy rápidos, ocho por tiempo, durante toda su duración. Con `_` el
redoble dura más tiempos, y con `<` crece hasta la próxima marca:

```aratu
tempo: 90

drums {
  sd = | p:s~< _ _ f:S | - s - s |
  bd = | -     - - b   | b - b - |
  cr = | -     - - x   | - - - - |
}
```

> El redoble crece durante tres tiempos y estalla en el acento con el
> platillo. Prueba `f~` en el tom de piso o `x~` en el platillo.

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
