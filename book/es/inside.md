# Por dentro de Aratu

Esta parte es para quien programa, o para quien quiere saber cómo funciona
Aratu por debajo. Da nombre técnico a las cosas que el curso y el manual
explican con palabras de música: qué es una línea, un bloque, un `at bar`,
un motivo, y qué hace la computadora con el texto hasta que se vuelve
sonido.

Nada de esto hace falta para tocar. Cada término técnico viene con una
explicación sencilla al lado, así que se puede leer sin saber programar.

## 1. Qué tipo de lenguaje es

Aratu es un lenguaje **declarativo**: el texto describe lo que suena, no una
lista de órdenes que se cumplen una tras otra. Es la misma familia que HTML,
CSS o una hoja de cálculo. No escribes "toca esta nota, espera, toca la
siguiente"; escribes cómo es la música, y Aratu calcula cuándo ocurre cada
sonido.

Algunas consecuencias:

- **El orden de las líneas casi no importa.** Un motivo se puede usar antes
  de definirlo, un patrón se puede tocar antes de escribirlo. Aratu lee todo
  el texto antes de tocar (en programación, eso se llama *hoisting*).
- **No hay variables que cambian, bucles ni `if`.** La repetición ya está en
  el tiempo: toda línea vuelve al principio sola. La elección es
  `{do|sol}`. Las condiciones son de tiempo: `at bar 9:`, `every 4 bars:`.
- **El texto es la única fuente de verdad.** Cada edición hace que Aratu lea
  todo el texto de nuevo. Lo que el reproductor guarda entre una versión y
  otra es poco: en qué compás empezó a sonar cada línea escrita en vivo.

```aratu
tempo: 100
melodia = | riff riff+1 riff+2 riff |
motif riff = [mi4 re do re]
```

Aquí el motivo está escrito después de la línea que lo usa, y funciona.

## 2. El léxico: las palabras del lenguaje

El léxico es el conjunto de "palabras" (en inglés, *tokens*) que Aratu
reconoce. Cada línea es una **sentencia** (*statement*). Dentro de llaves,
`;` separa sentencias en la misma línea, y `//` empieza un comentario hasta
el final de la línea.

| Tipo | Ejemplos | Qué es |
|---|---|---|
| nombre (identificador) | `melodia`, `bajo`, `riff`, `temas` | letras, números y `_`, empezando por una letra |
| ruta | `bajo.reverb.mix`, `drums.hh`, `riff.inv` | nombres unidos por puntos |
| palabra clave | `tempo`, `meter`, `key`, `pattern`, `play`, `at`, `every`, `now`, `over`, `motif`, `row`, `list`, `from` | palabras reservadas, con sentido fijo |
| número | `120`, `0.42`, `-1`, `1.2` | enteros y decimales, con punto |
| unidad | `Hz`, `kHz`, `ms`, `s`, `%` | justo después de un número |
| operador | `=`, `:`, `->`, `<->`, `.`, `,` | unen las partes de una sentencia |
| música | `do4`, `c4`, `fa#`, `mib`, `Am`, `c`, `o`, `_`, `-`, `[ ]`, `{ \| }`, `\|` | los símbolos dentro de una línea de música |

Los símbolos de música forman un pequeño lenguaje dentro del lenguaje: un
**literal** de música, como una cadena o un número son literales en otros
lenguajes. El espacio separa tiempos, `|` separa compases, `[ ]` agrupa en un
tiempo, `{ | }` sortea, `_` sostiene, `-` es silencio, `+` junta notas en un
acorde, la mayúscula es acento, `'` es staccato, `^` es calderón, `?` es azar
y `x4` repite.

## 3. La gramática: los tipos de sentencia

La gramática dice cómo se juntan las palabras. Cada sentencia tiene una de
estas formas (lo que va entre `⟨ ⟩` es un lugar para completar):

| Forma | Ejemplo | Para qué sirve |
|---|---|---|
| `⟨marco⟩: ⟨valor⟩` | `tempo: 96`, `meter: 3/4`, `key: A minor` | el marco de la pieza |
| `⟨nombre⟩ = ⟨música⟩` | `melodia = \| do4 re mi fa \|` | una línea de música con nombre |
| `⟨nombre⟩ { … }` | `drums { hh = c c c c }` | un bloque de líneas de una parte |
| `⟨parte⟩: ⟨sonido⟩ -> ⟨efecto⟩ …` | `bajo: synth.saw -> lowpass` | la cadena de señal de una parte |
| `⟨ruta⟩ = ⟨valor⟩` | `bajo.reverb.mix = 0.42` | un parámetro |
| `⟨ruta⟩ = ⟨valor⟩ -> ⟨valor⟩ over ⟨duración⟩` | `bajo.lowpass.cutoff = 300 Hz -> 4 kHz over 4 bars` | un parámetro que se mueve |
| `⟨parte⟩: ⟨orden⟩` | `piano: mf`, `guitar: out`, `piano: cresc to ff over 4 bars` | una orden para una parte |
| `pattern ⟨nombre⟩ { … }`, `section ⟨nombre⟩ { … }` | `pattern fill { sd = ss ss SS SS }` | un grupo de líneas con nombre |
| `play …`, `add …`, `form: …` | `play groove x3, fill` | qué suena, y en qué orden |
| `motif`, `row`, `list` | `motif riff = [mi4 re]` | definiciones que usan otras líneas |
| `⟨cuándo⟩: ⟨sentencia⟩` | `at bar 9: play fill`, `at bar 9 beat 3: …`, `from bar 9 to 16: …`, `every 4 bars: …`, `now: …` | la misma sentencia, en un momento o en un tramo |

El `⟨cuándo⟩` va delante de casi cualquier sentencia. Sin él, vale desde el
principio de la pieza o, si la música ya suena, desde el compás siguiente.

## 4. Dar nombre a las cosas

Esta tabla es el corazón de esta parte: para cada cosa de Aratu, el nombre
que tendría en programación, y a qué se parece.

| En Aratu | En términos de programación | Parecido a |
|---|---|---|
| `melodia = \| do4 re mi fa \|` | un **enlace** (*binding*) de un nombre a un valor; el valor es una secuencia de compases | una constante: no cambia mientras suena; cada edición la define de nuevo |
| una línea | una **secuencia circular**: al terminar, vuelve al principio | un clip en loop en una DAW |
| `drums { hh = …; bd = … }` | un **registro** o *namespace*: un mapa de nombres a líneas | un objeto (`drums.hh`) |
| `bajo.reverb.mix = 0.42` | **asignación de propiedad** en una ruta de un **mapa anidado** (parte → efecto → parámetro) | `bajo.reverb.mix = 0.42` en JavaScript, o una regla de CSS |
| `bajo: synth.saw -> lowpass -> reverb` | un **pipeline**: la salida de cada uno es la entrada del siguiente | composición de funciones, `reverb(lowpass(saw))`, o el `\|` de la terminal |
| `synth.saw`, `piano.grand` | una **clave de un catálogo** (un enum): un nombre que elige un sonido listo | un valor de una lista fija; no es un método |
| `motif riff = [mi4 re do re]` | una **macro**: el nombre se cambia por su contenido donde aparece | una función sin efectos secundarios, expandida en su lugar (*inlining*) |
| `riff+2`, `riff+3st` | el motivo con un **argumento**: la transposición | `riff(2)` |
| `riff.inv`, `riff.retro`, `serie.P5` | una **transformación pura** escrita con punto: devuelve un motivo nuevo y no cambia el original | un método o propiedad calculada sin paréntesis |
| `row serie = 0 11 3 4 …` | un **vector** (*array*) de enteros de 0 a 11 | `[0, 11, 3, 4, …]` |
| `\| o o o o \| from serie.P0` | un **zip** del ritmo con el vector, que empieza de nuevo al terminar; cada `o` es un **marcador** para completar | `zip(ritmo, cycle(serie))` |
| `list temas { … }` | un **array** de melodías, contado desde 1 | `temas[1]` en JavaScript es `temas.2` en Aratu |
| `temas.all`, `temas.random` | concatenación; elección al azar de un elemento | `temas.flat()`, `temas[random()]` |
| `{do\|sol}` | **elección al azar**, sorteada en cada vuelta | `random.choice` |
| `c?30%` | un evento con **probabilidad** del 30% | `if (random() < 0.3)` |
| `seed 7` | la **semilla** del generador pseudoaleatorio: la misma semilla da el mismo resultado | `Random(7)` |
| `pattern groove { … }` | un **bloque con nombre** de líneas | un componente, una escena, una plantilla |
| `play groove x3, fill` | una **playlist**: una secuencia con repeticiones, en loop | una lista de escenas |
| `\|: :\|`, `%`, `D.S. al coda` | **saltos** de la partitura, resueltos antes de tocar | un `goto` que el compilador desenrolla |
| `[mi4 re]x4`, `riffx4` | **repetición desenrollada**: se vuelve cuatro copias | *loop unrolling* |
| `p:`, `mf:` | un **estado** que vale para las notas siguientes de la línea | el "color actual" en una biblioteca de dibujo |
| `300 Hz -> 4000 Hz over 4 bars` | **interpolación** de un valor en el tiempo (en escala logarítmica para frecuencias) | *tween*, automatización por keyframes |
| una perilla | **enlace en los dos sentidos**: girarla reescribe el número del texto | *two-way binding*; el texto sigue siendo la fuente de verdad |

### ¿`at bar` es un switch?

No exactamente. Un `switch` (o `case`) elige **un** camino entre varios, cada
vez que se ejecuta, según un valor. Un `at bar` es una **regla indexada por
el tiempo**: a partir del compás 9, eso vale, y sigue valiendo hasta que otra
regla cambie lo mismo. Se parece más a un **keyframe** en una animación, o a
un evento marcado en una línea de tiempo.

```aratu
tempo: 110
bajo = | do2 _ sol1 _ |
at bar 5: bajo.reverb.mix = 0.6
at bar 9: bajo: out
```

Para saber el estado en el compás 12, Aratu junta las reglas de todos los
compases hasta ahí: el reverb está en 0.6 (desde el 5) y el bajo está fuera
(desde el 9). En programación, eso es un **pliegue** (*fold*) de los cambios,
en orden de tiempo. `from bar 9 to 16:` es una regla con principio y fin:
fuera del tramo no entra en el pliegue, y vuelve a valer lo que venía antes.

El `every`, en cambio, es lo más parecido a un `if` con el número del compás:

```aratu
tempo: 110
pattern groove { hh = cc cc cc cc; bd = b - b -; sd = - s - s }
pattern fill { sd = ss ss SS SS }
play groove
every 4 bars: play fill
```

`every 4 bars: play fill` quiere decir, compás a compás, "si este es el
último de cada cuatro, toca el redoble en su lugar": `if (compas % 4 == 0)`.
Y `every 4 bars, first:` es `if (compas % 4 == 1)`.

## 5. Del texto al sonido

Cuando editas, el texto pasa por estas etapas, cada una en un módulo del
compilador (escrito en Rust y llevado al navegador como WebAssembly):

1. **Léxico.** El texto se corta en líneas, bloques y palabras.
2. **Lectura previa.** Motivos, series, listas y la tonalidad se recogen
   primero, para que una línea pueda usarlos desde cualquier lugar del
   texto.
3. **Análisis sintáctico** (*parse*). Cada línea se vuelve una sentencia en
   un **árbol de sintaxis** (*AST*), con el lugar exacto donde está escrito
   cada trozo, para los mensajes y para encender las notas.
4. **Modelo.** Las sentencias se vuelven partes, líneas, patrones,
   secciones y órdenes, con las comprobaciones: si cada compás cabe en el
   compás indicado, si los nombres existen.
5. **Desenrollar.** Repeticiones, saltos, `%` y `x4` se vuelven una
   secuencia simple de compases.
6. **Matices y tiempo.** Cada nota recibe su fuerza, y las posiciones se
   vuelven segundos por el mapa de tempo. Las posiciones son **fracciones
   exactas**: un tresillo es 1/3 de tiempo, sin redondear.
7. **Programación** (*scheduling*). Para los compases pedidos, salen los
   **eventos**: cada nota, cada golpe, cada cambio de parámetro, con el
   instante en segundos.

La programación es **determinista**: el mismo texto, la misma semilla y los
mismos compases de partida dan siempre los mismos eventos. Por eso el
reproductor puede pedir un compás cada vez, un poco antes de que suene, y una
edición puede entrar en el compás siguiente sin que nada empiece de nuevo.

Los eventos van al motor de audio. Los sonidos grabados y las baterías
suenan con el motor de Manguebyte (un AudioWorklet que ejecuta WebAssembly);
los demás, con el motor provisional hecho con el Web Audio del navegador.

## 6. Los datos: lo que sale del compilador

El IDE habla con el compilador por unas pocas funciones exportadas por el
WebAssembly (`aratu_compile`, `aratu_render`, `aratu_check`, `aratu_grid`,
`aratu_voices`), intercambiando texto en **JSON**. Compilar devuelve los
mensajes, las partes, las perillas, las listas, las series y las líneas
escritas en vivo. Pedir compases devuelve los eventos. Un evento de nota se
parece a esto:

```json
{"bar": 3, "beat": 1.5, "time": 4.75, "dur": 0.24, "part": "bajo",
 "line": "bajo", "vel": 80, "note": 45, "from": 120, "to": 123}
```

| Campo | Qué es |
|---|---|
| `bar`, `beat` | el compás y el tiempo dentro de él, como está escrito |
| `time`, `dur` | cuándo suena y cuánto dura, en segundos desde el principio |
| `part`, `line` | la parte y la línea que lo tocan |
| `vel` | la fuerza, de 0 a 127, como en MIDI |
| `note` | la nota, como número MIDI (60 es el do central) |
| `drum` | en lugar de `note`, la pieza de la batería: `kick`, `snare`, `closed-hat`… |
| `acc` | 1 cuando la nota se escribió en mayúscula |
| `param`, `value`, `glide` | en lugar de la nota, un cambio de parámetro; `glide` son los segundos en que el sonido se desliza hasta el valor |
| `from`, `to` | dónde está escrito el símbolo en el texto, para encenderlo cuando suena |

Los mensajes también son datos: cada uno tiene la gravedad (error o aviso),
el trozo del texto y el texto en el idioma elegido. Un error no detiene la
música: mientras el texto tenga un error, sigue sonando la última versión
correcta, como un *hot reload* que rechaza una versión rota.
