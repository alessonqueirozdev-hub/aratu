# Tocar en vivo

En Aratu, tocar en vivo es cambiar el texto mientras la música suena. Unas
pocas ideas lo hacen fácil.

- **Todo se repite.** Cada línea suena sin parar, cada una con su largo: un
  hi-hat de un compás, un bajo de dos y una secuencia de acordes de cuatro
  van juntos como en una banda.
- **El cambio entra en el compás siguiente.** Nada vuelve a empezar: las
  otras partes siguen en su lugar.
- **Con un error, nada se detiene.** Mientras el texto tenga un error,
  sigue sonando la última versión que estaba bien.

## Patrones

Un patrón es un grupo de líneas con nombre, preparado de antemano. `play`
elige qué patrón suena:

```aratu
tempo: 110
pattern groove {
  hh = cc cc cc cc
  bd = b - b -
  sd = - s - s
}
pattern fill {
  sd = ss ss SS SS
  bd = b - - -
}

play groove x3, fill
```

`play groove x3, fill` toca el groove tres veces, después el redoble, y
repite la frase de cuatro compases.

## Variar de vez en cuando

```aratu
tempo: 110
pattern groove { hh = cc cc cc cc; bd = b - b -; sd = - s - s }
pattern fill   { sd = ss ss SS SS }
pattern crash  { cr = x - - - }

play groove
every 4 bars: play fill
every 8 bars, first: add crash
```

`every 4 bars: play fill` cambia el groove por el redoble en el último
compás de cada cuatro. `add` toca un patrón **encima** del que está
sonando: aquí, un platillo en el primer tiempo de cada ocho compases.

## Programar un cambio

`at bar 9: play fill` marca un cambio para el compás 9. Una línea escrita
sin `at bar` entra al principio de la pieza o, si la escribes con la música
sonando, en el compás siguiente. Escrita con `now:` delante
(`now: play fill`), entra ya en el tiempo siguiente. Y con `once` al final
(`now: play fill once`), el redoble suena una vez y el groove vuelve solo,
en el lugar donde estaría.

## Sacar y traer una parte

`bateria: out` silencia una parte a partir del compás siguiente, y
`bateria: in` la trae de vuelta, como en una consola. `all: out` silencia
todo.

```aratu
tempo: 110
drums { hh = cc cc cc cc; bd = b - b -; sd = - s - s }
bajo = | do2 _ sol1 _ | la1 _ mi1 _ |
at bar 3: drums: out
at bar 5: drums: in
```

## En el escenario

El botón **Escenario**, en Tocar, deja solo el código en la pantalla, a
pantalla completa y con letra grande, para tocar frente al público. El botón
**2 columnas** divide el código en dos columnas, como en un periódico: la
segunda sigue donde termina la primera, así cabe el doble sin desplazar la
pantalla, y las dos se desplazan juntas. Se puede editar en cualquiera. Las
listas de melodías tienen botones en la esquina, y Esc sale del escenario.

## Ejercicios

Usa los patrones listos para tocar la frase: groove tres veces, después el
redoble.

```aratu exercise bars=4
pattern groove { hh = cc cc cc cc; bd = b - b -; sd = - s - s }
pattern fill   { sd = ss ss SS SS; bd = b - - - }
play groove
```

```aratu answer
pattern groove { hh = cc cc cc cc; bd = b - b -; sd = - s - s }
pattern fill   { sd = ss ss SS SS; bd = b - - - }
play groove x3, fill
```

Haz que la batería salga en el compás 2 y vuelva en el compás 3, mientras
el bajo sigue.

```aratu exercise bars=4
drums { hh = c c c c }
bajo = | do2 _ _ _ |
```

```aratu answer
drums { hh = c c c c }
bajo = | do2 _ _ _ |
at bar 2: drums: out
at bar 3: drums: in
```
