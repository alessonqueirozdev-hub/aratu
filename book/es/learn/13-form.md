# Secciones y forma

Una canción tiene partes: introducción, estrofa, estribillo, final. Cada
parte vuelve más de una vez, y el orden en que aparecen es la **forma** de
la música. En Aratu escribes cada parte una sola vez, con nombre, y después
dices en qué orden suenan.

## Una sección

Una sección es un bloque con nombre, escrito con `section` (o `sección`).
Dentro van las partes y las líneas, como en una pieza entera:

```aratu
tempo: 100
section estrofa {
  bajo = | la2 _ mi2 _ | fa2 _ do3 _ |
  drums { hh = cc cc co cc; bd = b - b -; sd = - s - s }
}
form: estrofa
```

Un bloque puede ir en una sola línea, con `;` entre las líneas, como la
batería de arriba. La sección dura lo que dura su línea más larga: aquí,
dos compases.

## La forma

La línea `form:` (o `forma:`) dice el orden de las secciones. `x2` toca una
sección dos veces seguidas:

```aratu
tempo: 100
section intro      { bajo = | la2 _ _ _ | mi2 _ _ _ | }
section estrofa    {
  bajo = | la2 _ mi2 _ | fa2 _ do3 _ |
  drums { hh = cc cc co cc; bd = b - b -; sd = - s - s }
}
section estribillo {
  guitarra { chords = | Am | F | C | G |; strum = D du -u du }
  drums    { hh = cc cc cc cc; bd = b - b -; sd = - s - S }
}
section final      { bajo = | la2 _ _ _ | }

form: intro estrofa x2 estribillo estrofa estribillo final
```

Escucha el orden: introducción, estrofa dos veces, estribillo, estrofa,
estribillo, final. Al llegar al final, la forma vuelve a empezar, como todo
loop de Aratu.

> Los nombres de las secciones los eliges tú: `intro`, `a`, `b`, `puente`,
> `solo`. Escríbelos sin tilde y sin espacios.

## Secciones y repeticiones juntas

Dentro de una sección valen las barras de repetición, las casillas y el `%`
de la lección de repeticiones. Y la forma puede repetir un fragmento más
grande:

```aratu
tempo: 110
section a { melodia = | do4 mi sol mi | re fa la fa | }
section b { melodia = | mi4 sol si sol | do5 _ _ _ | }
form: a x2 b a
```

Es la forma AABA, la de muchas canciones del jazz y de la bossa nova.

## Ejercicios

Pon las secciones en orden: introducción, estrofa, estribillo.

```aratu exercise bars=3
section intro      { bajo = | la2 _ _ _ | }
section estrofa    { bajo = | do3 _ _ _ | }
section estribillo { bajo = | fa2 _ _ _ | }
form: intro
```

```aratu answer
section intro      { bajo = | la2 _ _ _ | }
section estrofa    { bajo = | do3 _ _ _ | }
section estribillo { bajo = | fa2 _ _ _ | }
form: intro estrofa estribillo
```

Ahora la forma AABA: la sección `a` dos veces, después `b`, después `a`.

```aratu exercise bars=4
section a { melodia = | do4 _ _ _ | }
section b { melodia = | sol4 _ _ _ | }
form: a b
```

```aratu answer
section a { melodia = | do4 _ _ _ | }
section b { melodia = | sol4 _ _ _ | }
form: a x2 b a
```

Escribe la sección `estribillo`, con el bajo tocando fa2 y sol2, una blanca
cada uno, para la forma que ya está lista.

```aratu exercise bars=2
section estrofa    { bajo = | la2 _ _ _ | }
section estribillo { bajo = | - | }
form: estrofa estribillo
```

```aratu answer
section estrofa    { bajo = | la2 _ _ _ | }
section estribillo { bajo = | fa2 _ sol2 _ | }
form: estrofa estribillo
```
