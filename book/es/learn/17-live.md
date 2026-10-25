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
repite la frase de cuatro compases. En español: `patrón` en lugar de
`pattern` y `toca` en lugar de `play`.

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

## Ya, en el tiempo siguiente

Una línea escrita con la música sonando entra en el compás siguiente. Con
`now:` (o `ahora:`) delante, entra ya en el tiempo siguiente:
`now: play fill once` trae el redoble en el momento, y el groove vuelve
solo, en el lugar donde estaría. La lección siguiente muestra cómo marcar
un cambio en un compás y un tiempo exactos.

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

## Gestos de escenario

En la música electrónica, una presentación avanza por gestos: la batería
desaparece (el *breakdown*), la tensión sube (el *build*) y todo vuelve de
golpe (el *drop*). En Aratu, cada gesto es una palabra:

| Escribe | Qué pasa |
|---|---|
| `breakdown` (o `quiebre`) | la batería, la percusión y el bajo salen; lo demás sigue |
| `build over 4 bars` (o `subida en 4 compases`) | un redoble de caja que dobla su velocidad y crece, mientras un filtro le quita los graves a todo |
| `drop` (o `caída`) | todo vuelve, con un platillo en el primer tiempo |
| `solo bajo` | solo suena el bajo; `solo off` trae lo demás de vuelta |

Con `now:` delante, el gesto entra en el tiempo siguiente; con `at bar`, en
el compás marcado:

```aratu
tempo: 124
drums { bd = b b b b; oh = -o -o -o -o; clap = - p - p }
bajo = | [- do2]x4 | [- lab1]x4 |
pad = | do4+mib4+sol4 _ _ _ | do4+mib4+lab4 _ _ _ |
at bar 5: breakdown
at bar 7: build over 2 bars
at bar 9: drop
```

Escucha: cuatro compases con todo, el pad solo, la subida y la vuelta con
el platillo. Tocando en vivo, escribe `now: drop` cuando la pista lo pida.

## Ritmos listos

Para empezar rápido, una palabra escribe una batería entera: `ritmo house`
(o `groove house`). Los ritmos son: house, techno, trap, dnb, breakbeat,
hiphop, funk, rock, reggaeton, funk carioca, bossa, samba, baião y
maracatu.

```aratu
tempo: 124
ritmo house
bajo = | [- la1]x4 | [- fa1]x4 |
```

Mientras suena, la palabra `ritmo` se ilumina. Y cualquier línea que
escribas con el mismo nombre toma el lugar de la línea del ritmo:
`hh = c c c c` cambia solo el hi-hat, y el resto del ritmo sigue.

## Atajos para tocar en vivo

| Atajo | Qué hace |
|---|---|
| Ctrl+Enter | toca, o actualiza lo que está sonando |
| Ctrl+. | para |
| Ctrl+/ | comenta las líneas seleccionadas: dejan de sonar; otra vez, vuelven |
| Alt+M | silencia la parte de la línea donde está el cursor (escribe `parte: out`); otra vez, vuelve |
| Alt+S | pone en solo la parte de la línea del cursor (escribe `solo parte`); otra vez, lo quita |
| Ctrl+Shift+1 a 8 | trae de vuelta una escena guardada |

Los atajos escriben en el texto lo que hacen, así que el cambio entra en el
compás siguiente, como cualquier otro, y queda guardado si compartes.

## Más rápido, más lento, y un eco

Tres verbos cortos cambian una línea mientras suena, sin volver a
escribirla. Escribe el nombre de la línea, un punto y el verbo:

```aratu
tempo: 120
drums {
  hh = | cc cc cc co | cc cc c- c- |
  bd = b - b -
  sd = - s - s
}
hh.fast 2
```

`hh.fast 2` toca dos compases del charles en cada compás: el doble de
rápido. `slow 2` estira cada compás en dos. Los números son enteros, para
que la línea siempre caiga en la barra de compás; `fast 2` y `slow 2`
juntos se anulan. Sirven para una línea o para una parte entera
(`drums.fast 2`). Borra la línea y el charles vuelve a su velocidad.

`echo` toca cada sonido otra vez un poco después, y más suave, como una voz
que vuelve de una pared:

```aratu
tempo: 100
lead: synth.pluck
lead = | mi5 - sol - | la - sol - |
lead.echo 3 sixteenths x2
```

`echo 3 sixteenths` repite cada nota tres semicorcheas después, y `x2` hace
dos ecos, cada uno más suave que el anterior. La duración se escribe como
después de `over`: `1 beat`, `1 eighth`, `3 sixteenths`. En español los
verbos son `rapido` (o `rápido`), `lento` y `eco`, y las duraciones
`1 tiempo`, `1 corchea`, `3 semicorcheas`.

## Escenas

¿Armaste un buen momento de la música? Guárdalo: en Tocar y en el escenario
hay ocho **escenas**, numeradas del 1 al 8. Haz clic en una escena vacía y
la música del editor queda guardada en ella; haz clic en una escena
guardada, o pulsa Ctrl+Shift y el número, y vuelve al editor. Con la música
sonando, el cambio entra en el compás siguiente, como cualquier cambio, y
Ctrl+Z lo deshace. Shift+clic guarda la música actual encima de una escena.

Así preparas la introducción, el groove, el breakdown y el drop como
escenas, y en el escenario pasas de una a otra con una tecla. Las escenas
quedan guardadas en este navegador.

## Ver el sonido: onda y rollo

Debajo de una parte, escribe su nombre, un punto y **onda**, y el editor
dibuja ahí la onda de su sonido mientras suena, como un osciloscopio. Con
**rollo** en lugar de onda, aparecen las notas del compás pasando bajo
una línea que avanza, una fila para cada nota o tambor, como el *piano
roll* de un estudio. Sirve para una parte (`bajo.onda`) o para una línea
de la batería (`drums.hh.rollo`).

```aratu
tempo: 120
drums { bd = b b b b; hh = cc cc cc cc }
drums.hh.rollo
bajo = | do2 do2 mib2 sol2 |
bajo.onda
```

Los dibujos no cambian el sonido: sirven para ver qué hace cada parte. La
onda es el sonido de verdad de la parte, después de sus efectos, como
Manguebyte lo pone en la mezcla: pon un `drive` o un `lowpass` en la
cadena y mira cómo cambia la onda. Se queda quieta en unas tres vueltas de
la nota más grave. Una línea de la batería (`drums.hh.onda`) se dibuja a
partir de los golpes, porque la batería suena como una sola parte. La onda
de toda la música está en el visor, al lado del compás; un clic en ella muestra las frecuencias, de
los graves a la izquierda a los agudos a la derecha, como en un
ecualizador.

## Seguir la música

Con el botón **Seguir**, en Tocar y en el escenario, la pantalla va sola
al trozo que está sonando: cuando la música pasa a una sección más abajo,
su código aparece. La pantalla espera mientras escribes, haces clic o te
desplazas, y solo avanza cuando lo que suena sale de la vista.

El **mapa del ritmo**, debajo del editor, muestra la música en el tiempo,
como el arreglo de una DAW: una columna para cada compás, en el orden en
que suena, con el nombre de la sección arriba, y una fila para cada
parte, solo con lo que toca en cada compás. Una línea naranja lo recorre
mientras la música suena, y el mapa se desplaza con ella.

## En el escenario

El botón **Escenario**, en Tocar, deja solo el código en la pantalla, a
pantalla completa y con letra grande, para tocar frente al público. El botón
**2 columnas** divide el código en dos columnas, como en un periódico: la
segunda sigue donde termina la primera, así cabe el doble sin desplazar la
pantalla, y cada una se desplaza sola. Se puede editar en cualquiera. Con
**Caber en pantalla**, la letra se achica a medida que la música crece, para
que la pieza entera quepa sin desplazar. Las listas de melodías tienen
botones en la esquina, y Esc sale del escenario.

## Tocar en una DAW

Una DAW (REAPER, Ableton Live, FL Studio, Logic) es el programa que graba,
mezcla y pone efectos a la música. Aratu toca dentro de ella con el plugin
**Aratu Bridge**, de Manguebyte: pon el plugin en una pista de instrumento
de la DAW, pulsa **DAW** en Tocar y después **Conectar**. Desde entonces el
sonido sale de la pista, y no del navegador, con los mismos instrumentos:
puedes grabar la pista, mezclarla con las otras y ponerle los efectos de la
DAW encima.

Quien manda en el tempo es la DAW. Su play empieza la música en su compás
siguiente, su stop la para, y su tempo se escribe en la línea `tempo:` de
la pieza. Con la DAW parada, **Tocar** sigue funcionando, y el sonido sale
en la DAW igual.

El plugin escucha en el puerto 47870; con más de un plugin en la DAW, cada
uno tiene su puerto, y se escribe en el panel. La primera vez, el navegador
puede preguntar si la página puede hablar con programas de este ordenador:
responde que sí. Por la DAW no llegan las ondas de las partes, así que
`bajo.onda` vuelve a dibujarse a partir de las notas.

> Pon el plugin en una pista, conecta y escribe `tempo: 90` en tu pieza.
> Dale play a la DAW a 120: la línea pasa a `tempo: 120` y la música entra
> en el compás siguiente de la DAW.

## Guardar en tu cuenta

Sin cuenta, la pieza queda guardada en este navegador. Con una cuenta de
MusiMind (el botón **Entrar**, arriba), queda guardada en la cuenta y se
abre en cualquier ordenador: pulsa **Guardar en mi cuenta**, y desde
entonces cada cambio se guarda solo, pocos segundos después. **Guardar
versión** guarda un retrato de la pieza para volver a él después, en
**Versiones**; en **Mis piezas** están todas, para abrir, renombrar o
borrar. Abrir un ejemplo o un enlace compartido no toca la pieza de la
cuenta: sigue allí como estaba.

## Compartir y grabar

¿Te gustó lo que hiciste? **Compartir** copia un enlace con tu música
adentro: mándalo a alguien, y quien lo abra oye y puede cambiar la misma
pieza. **Grabar** graba lo que está sonando, hasta que lo pulses de nuevo;
entonces el archivo de audio se descarga, para guardarlo o mostrarlo.

## Ejercicios

Escribe el bajo tocando do2 en los cuatro tiempos y muestra debajo la
onda del bajo.

```aratu exercise
drums { bd = b b b b }
```

```aratu answer
drums { bd = b b b b }
bajo = | do2 do2 do2 do2 |
bajo.onda
```

Usa los patrones listos para tocar la frase: groove tres veces, después el
redoble. (`x3` también puede escribirse `3x`.)

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

Haz un breakdown en el compás 3 y el drop en el compás 5.

```aratu exercise bars=6
drums { bd = b b b b }
bajo = | do2 _ _ _ |
pad = | do4+mib4+sol4 _ _ _ |
```

```aratu answer
drums { bd = b b b b }
bajo = | do2 _ _ _ |
pad = | do4+mib4+sol4 _ _ _ |
at bar 3: breakdown
at bar 5: drop
```

Empieza con el ritmo house y cambia el hi-hat por corcheas cerradas.

```aratu exercise
drums { bd = b b b b }
```

```aratu answer
ritmo house
hh = cc cc cc cc
```

Una subida de dos compases desde el compás 3.

```aratu exercise bars=5
drums { bd = b b b b }
```

```aratu answer
drums { bd = b b b b }
at bar 3: build over 2 bars
```

Con una línea más, toca el charles el doble de rápido.

```aratu exercise
hh = cc cc cc co
```

```aratu answer
hh = cc cc cc co
hh.fast 2
```

Dale a la melodía un eco un tiempo después de cada nota.

```aratu exercise
melodia = | do5 - - - | sol4 - - - |
```

```aratu answer
melodia = | do5 - - - | sol4 - - - |
melodia.echo 1 beat
```
