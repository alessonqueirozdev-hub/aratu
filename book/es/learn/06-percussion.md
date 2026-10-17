# Percusión brasileña

La batería tiene un símbolo por pieza. La percusión brasileña es distinta:
cada instrumento tiene una línea propia, con el nombre del instrumento, y
sus propios **golpes**: el pulgar en el pandeiro, el bacalhau en la
zabumba, el triángulo abierto o cerrado. El ritmo sigue las mismas reglas
de siempre.

## Una línea con el nombre del instrumento

Escribe el nombre del instrumento antes del `=`, y usa las letras de sus
golpes:

```aratu
tempo: 100
triángulo = fo fo fo fo
```

`f` es el triángulo cerrado (la mano sujeta el hierro) y `o` es el abierto
(la mano lo suelta y suena). Ese "cerrado, abierto" en corcheas es el
corazón del baião.

| Línea | Golpes |
|---|---|
| `pandeiro` | `p` pulgar, `d` punta de los dedos, `t` palmada, `m` apagado |
| `zabumba` | `o` abierto, `m` apagado, `b` bacalhau (la varilla por debajo) |
| `triángulo` | `o` abierto, `f` cerrado |
| `agogó` | `a` campana aguda, `g` campana grave |
| `surdo` | `o` abierto, `m` apagado |
| `tamborim` | `o` abierto, `m` apagado |
| `alfaia` | `o` abierto, `m` apagado |
| `gonguê` | `o` abierto, `m` apagado |
| `ganzá` | `s` |
| `cuíca` | `a` aguda, `g` grave |
| `cajón` | `b` grave, `t` golpe agudo |

El nombre puede ir con o sin tilde (`triangulo`, `agogo`) y también en
portugués: `agogô`. `pandero` vale lo mismo que `pandeiro`. `agbê`,
`caxixi` y `chocalho` suenan como el ganzá.

> Si escribes una letra que el instrumento no tiene, por ejemplo `x` en el
> pandeiro, Aratu te dice qué golpes tiene.

## El baião

El baião junta tres instrumentos: la zabumba, el triángulo y el agogó (o el
acordeón, que queda para después). La zabumba toca el abierto en el tiempo
1, el apagado al final del tiempo 2, y el bacalhau responde por debajo:

```aratu
tempo: 110
zabumba   = | o - -o m | o - b - |
triángulo = fo fo fo fo
agogó     = a g ag -
```

> Cambia el tempo a 140 y el baião se vuelve forró pé de serra.

## El samba

En el samba, cada instrumento tiene su papel. El surdo marca el tiempo 2,
el tamborim hace el dibujo, el pandeiro y el ganzá llenan las
semicorcheas:

```aratu
tempo: 96
meter: 2/4
surdo    = m O
tamborim = o-oo -oo-
pandeiro = pdtd pdtd
ganzá    = Ssss Ssss
```

El samba se cuenta en dos. La letra mayúscula es un acento, como en la
batería: `O` en el surdo es el golpe fuerte del tiempo 2, y `S` en el ganzá
marca el principio de cada tiempo.

## Con la batería

Las líneas de percusión tocan junto con la batería y con las otras partes.
Un `surdo` es el surdo del samba; el tom de piso de la batería sigue siendo
la `f`, en una línea de batería:

```aratu
tempo: 100
drums {
  bd = b - b -
  sd = - s - s
}
agogó  = ag ag a- g-
cajón  = b t bb t
```

## Ejercicios

Escribe el triángulo del baião: cerrado y abierto en corcheas, en los
cuatro tiempos.

```aratu exercise
triángulo = - - - -
```

```aratu answer
triángulo = fo fo fo fo
```

El surdo del samba: apagado en el tiempo 1 y abierto, acentuado, en el
tiempo 2.

```aratu exercise
meter: 2/4
surdo = o o
```

```aratu answer
meter: 2/4
surdo = m O
```

El pandeiro en semicorcheas: pulgar, dedos, palmada, dedos, en cada
tiempo.

```aratu exercise
meter: 2/4
pandeiro = pppp pppp
```

```aratu answer
meter: 2/4
pandeiro = pdtd pdtd
```

Completa el baião: la zabumba con el abierto en el tiempo 1 y el apagado en
la segunda mitad del tiempo 2, y el bacalhau en el tiempo 3.

```aratu exercise
zabumba = | - - - - |
```

```aratu answer
zabumba = | o -m b - |
```
