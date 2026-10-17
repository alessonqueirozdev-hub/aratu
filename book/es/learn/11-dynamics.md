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
