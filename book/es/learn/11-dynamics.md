# Matices y articulación

Las marcas de dinámica son las de la partitura: `ppp pp p mp mf f ff fff`,
y también `sfz` y `fp`. Dentro de un compás, la marca va pegada a la nota
con dos puntos, y vale de ahí en adelante:

```aratu
tempo: 90
melodia = | p:do4 mi sol do5 | f:sol4 _ _ _ |
```

El primer compás es piano; el segundo, forte. La marca no cuenta como
tiempo, y puede llevar un espacio después de los dos puntos: `p: do4`
también vale. En `f: fa4`, la primera `f` es forte y `fa4` es la nota.

## Crescendo y diminuendo

`<` después de una nota hace un crescendo y `>` un diminuendo, desde el
tiempo siguiente hasta la próxima marca:

```aratu
tempo: 90
melodia = | p:do4< re mi fa | f:sol4> fa mi re | p:do4 _ _ _ |
```

Sin una marca después, el regulador sube un escalón (de mf a f, por
ejemplo) hasta el final de la línea.

## Articulación: acento, marcato, staccato y nota fantasma

La articulación dice **cómo** se ataca cada nota. Son cuatro signos, del
más fuerte al más débil:

| Signo | Escrito | Cómo suena |
|---|---|---|
| marcato | `S!`, `do4!` | el más fuerte, bien marcado |
| acento | `S`, `Do4` | más fuerte que las vecinas |
| staccato | `s'`, `do4'` | corto, separado |
| nota fantasma | `(s)`, `(do4)` | casi sin sonido, apenas se siente |

El acento suma intensidad a la marca en la que está la música, y el
marcato suma todavía más. Las notas fantasma son el condimento del funk y
del samba en la caja: toques muy suaves entre los golpes fuertes.

```aratu
tempo: 96
drums {
  hh = cc cc cc cc
  bd = b - b b
  sd = (s)(s) S! (s)(s) S!
}
```

> Quita los paréntesis de las notas fantasma y escucha cómo la caja se
> vuelve pesada. Los paréntesis hacen que los toques se vayan al fondo.

## Ornamentos: trino, mordente y nota de adorno

Los ornamentos adornan una nota con su vecina en la tonalidad. Van después
de la nota, con dos puntos, como el `:arp` de los acordes:

| Ornamento | Escrito | Cómo suena |
|---|---|---|
| trino | `do5:tr` | la nota y la de arriba, alternando rápido, durante toda la duración |
| mordente | `do5:mord` | la nota, la de arriba y la nota otra vez, muy rápido, al principio |
| mordente inferior | `do5:mordi` | lo mismo, con la de abajo |
| nota de adorno | `` `re5 do5 `` | una nota pequeña, pegada antes de la principal |

La vecina es la de la tonalidad: en sol mayor, el trino de `la4` va a
`si4`, y el de `mi5` a `fa#5`. Para cambiarla, pon `#` o `b` después del
`tr`, como la alteración sobre el *tr* en la partitura: `mi5:trb` trina con
fa natural.

```aratu
tempo: 80
key: G major
melodia = | sol4 si4 `do5 re5 _ | re5:mord do5 si4 la4 | la4:tr _ _ _ | sol4 _ _ _ |
```

La nota de adorno no cuenta como tiempo: suena un instante antes de la
principal. En un tambor, el acento grave hace lo mismo con el propio
tambor: `` `s `` es un **flam**, un toque suave justo antes, y ``` ``s ```
es un **drag**, dos:

```aratu
tempo: 90
drums {
  hh = cc cc cc cc
  bd = b - b -
  sd = - `S - ``S
}
```

> Quita los acentos graves y escucha la caja seca; con ellos, cada golpe
> lleva un arrastre antes.

## Cambiar la parte entera

Una sola línea lleva una parte entera a una dinámica, sea cual sea la que
está escrita en ella:

```aratu
melodia = | do4 mi sol mi |
melodia: pp
```

Prueba `melodia: ff`, o `melodia: cresc to ff over 4 bars`: un crescendo
de cuatro compases, que sale de lo que está escrito y llega a ff.

## Ejercicios

El primer compás **piano**, el segundo **forte**.

```aratu exercise dynamics=1
melodia = | do4 re mi fa | sol fa mi re |
```

```aratu answer
melodia = | p:do4 re mi fa | f:sol4 fa mi re |
```

Empieza en `p` y haz un crescendo hasta `f` en el compás siguiente.

```aratu exercise dynamics=1
melodia = | p:do4 re mi fa | f:sol4 _ _ _ |
```

```aratu answer
melodia = | p:do4< re mi fa | f:sol4 _ _ _ |
```

En la caja, los tiempos 2 y 4 en marcato y notas fantasma en las corcheas
entre ellos: `(s)` en la segunda mitad de los tiempos 1 y 3.

```aratu exercise dynamics=1
sd = s- s s- s
```

```aratu answer
sd = -(s) s! -(s) s!
```

Adorna la melodía: un **mordente** en el primer mi5 y un **trino** en el
último.

```aratu exercise
tempo: 80
melodia = | mi5 re5 do5 re5 | mi5 _ _ _ |
```

```aratu answer
tempo: 80
melodia = | mi5:mord re5 do5 re5 | mi5:tr _ _ _ |
```

Una nota de adorno `re5` antes de cada do5.

```aratu exercise
melodia = | do5 sol4 do5 sol4 |
```

```aratu answer
melodia = | `re5 do5 sol4 `re5 do5 sol4 |
```

En la caja, un **flam** en el tiempo 2 y un **drag** en el tiempo 4.

```aratu exercise
sd = - s - s
```

```aratu answer
sd = - `s - ``s
```
