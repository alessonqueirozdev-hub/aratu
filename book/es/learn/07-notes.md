# Notas y melodías

Hasta aquí solo había batería. Ahora, notas. Una nota se escribe con el
nombre y la octava: `do4` es el do central del piano. Los nombres pueden ir
en solfeo o en letras, como en los cifrados:

| Solfeo | do | re | mi | fa | sol | la | si |
|---|---|---|---|---|---|---|---|
| Letra | c | d | e | f | g | a | b |

Escucha la escala de do mayor subiendo:

```aratu
tempo: 100
melodia = | do4 re4 mi4 fa4 | sol4 la4 si4 do5 |
```

Una línea que no es de batería suena con un sintetizador. Ponle el nombre
que quieras: `melodia`, `flauta`, `voz`.

## La octava sigue hasta que cambia

No hace falta repetir la octava en cada nota: vale hasta que aparece otra.
La escala de arriba se puede escribir así:

```aratu
melodia = | do4 re mi fa | sol la si do5 |
```

## Sostenidos y bemoles

El sostenido es `#` y el bemol es `b`, justo después del nombre: `fa#4`,
`sib3`, `eb4` (mi bemol).

```aratu
melodia = | la3 si do#4 re | mi fa# sol# la |
```

Es la escala de la mayor.

## Una melodía conocida

"Estrellita, ¿dónde estás?" es una melodía antigua, de dominio público.
Escucha la primera frase:

```aratu
tempo: 100
melodia = | do4 do sol sol | la la sol _ |
```

El `_` del final sostiene el sol un tiempo más: una blanca.

## Ejercicios

Completa la frase: después de los dos compases vienen `fa fa mi mi | re re do _`.

```aratu exercise
tempo: 100
melodia = | do4 do sol sol | la la sol _ |
```

```aratu answer
tempo: 100
melodia = | do4 do sol sol | la la sol _ | fa fa mi mi | re re do _ |
```

Escribe la escala de do mayor **bajando**, del do5 al do4, en dos
compases.

```aratu exercise
melodia = | do5 _ _ _ | do4 _ _ _ |
```

```aratu answer
melodia = | do5 si4 la sol | fa mi re do |
```
