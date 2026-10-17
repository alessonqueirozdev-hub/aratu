# El ritmo en la melodía

Las reglas del ritmo también valen para las notas, con un cuidado: dos
notas en el mismo tiempo van entre **corchetes**.

```aratu
tempo: 90
melodia = | [do4 re] mi [fa sol] la | sol _ _ _ |
```

`[do4 re]` son dos corcheas en un tiempo. ¿Por qué corchetes? Porque `mib`
es mi bemol: sin corchetes, Aratu no sabría si `mib` es una nota o un mi
seguido de un si. Los silencios y prolongaciones pueden ir pegados a la
nota: `-mi4` es silencio de corchea y mi.

## Notas largas y ligaduras

El `_` hace que la nota dure un tiempo más (o una parte más, dentro de
corchetes). Un `_` al principio de un compás liga la nota del compás
anterior, como una ligadura:

```aratu
melodia = | do4 _ mi _ | sol _ _ _ | _ _ mi _ |
```

El sol dura seis tiempos: cuatro en el segundo compás y dos más en el
tercero.

## Puntillo y staccato

Dentro de los corchetes, la corchea con puntillo queda `[do4 _ _ re]` (el
punto también vale: `[do4. re]`). Un apóstrofo hace la nota corta,
staccato:

```aratu
tempo: 100
melodia = | mi4' mi' fa' sol' | sol fa mi re |
```

## Varias notas a la vez

El `+` une notas que suenan juntas, un acorde:

```aratu
teclado = | do4+mi4+sol4 _ _ _ | re4+fa4+la4 _ si3+re4+sol4 _ |
```

## La "Oda a la alegría"

El tema de Beethoven, de dominio público, tiene un ritmo con puntillo al
final:

```aratu
tempo: 110
melodia = | mi4 mi fa sol | sol fa mi re | do do re mi | mi [_ re] re _ |
```

El último compás: mi dura un tiempo y medio (`mi` y la primera mitad de
`[_ re]`), re dura medio tiempo, y el último re dura dos tiempos.

## Ejercicios

Termina la "Oda a la alegría" con la otra frase, que acaba en do: en el
último compás, `re [_ do] do _`.

```aratu exercise
tempo: 110
melodia = | mi4 mi fa sol | sol fa mi re | do do re mi | re _ _ _ |
```

```aratu answer
tempo: 110
melodia = | mi4 mi fa sol | sol fa mi re | do do re mi | re [_ do] do _ |
```

Escribe un acorde de do mayor (do4, mi4, sol4) que dure el compás entero.

```aratu exercise
teclado = | do4 _ _ _ |
```

```aratu answer
teclado = | do4+mi4+sol4 _ _ _ |
```

Corcheas con la primera de cada tiempo en staccato: `[do4' re]` en los
cuatro tiempos.

```aratu exercise
melodia = | [do4 re] [do re] [do re] [do re] |
```

```aratu answer
melodia = | [do4' re] [do' re] [do' re] [do' re] |
```
