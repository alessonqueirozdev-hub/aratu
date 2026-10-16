# El manual de Aratu

Aratu se escribe como piensa un músico: en compases, tiempos, figuras,
acordes y palabras de la partitura. Quien lee un cifrado y cuenta un compás
puede leer una pieza en Aratu. Este manual describe todo el lenguaje; el
curso **Aprender** lo enseña paso a paso. Cada ejemplo de aquí se puede
escuchar y cambiar.

## 1. Una primera pieza

```aratu
meter: 4/4
tempo: 96
key: A minor

drums {
  hh = cc cc co cc
  bd = b - b -
  sd = - s - s
}

bajo = | la2 _ mi2 _ | fa2 _ do3 _ |

guitarra {
  chords = | Am | F | C G | G |
  strum  = D du -u du
}
```

Las primeras líneas dan el marco de la pieza: indicación de compás, tempo y
tonalidad. Después viene cada parte. No hay `main`, ni llaves que cerrar
antes de que suene la música, ni palabras que solo existen para la
computadora. `//` empieza un comentario.

## 2. Ritmo: los tiempos y sus partes

Tres reglas describen todo ritmo dentro de un compás.

**Regla 1. Un espacio separa los tiempos.** Una línea en 4/4 tiene cuatro
grupos. Si un compás tiene el número equivocado de tiempos, Aratu lo dice
con esas palabras: *"el compás 2 de `bajo` tiene 5 tiempos, pero la
indicación de compás es 4/4"*.

**Regla 2. Los símbolos escritos juntos dividen el tiempo en partes
iguales.**

| Escrito | En un tiempo |
|---|---|
| `c` | una negra |
| `cc` | dos corcheas |
| `cccc` | cuatro semicorcheas |
| `ccc` | un tresillo de corcheas |
| `ccccc` | un quintillo |

**Regla 3. `_` hace que el sonido anterior dure una parte más; `-` es un
silencio.**

| Ritmo en un tiempo | Escrito |
|---|---|
| semicorchea, semicorchea, corchea | `ccc_` |
| semicorchea, corchea, semicorchea | `cc_c` |
| corchea, semicorchea, semicorchea | `c_cc` |
| corchea con puntillo, semicorchea | `c__c`, o el atajo `c.c` |
| silencio de corchea, corchea | `-c` |

Un `_` solo sostiene un tiempo entero, así que `c _` es una blanca y
`c _ _ _` una redonda. Un `_` al principio de un compás liga la nota del
compás anterior.

**Los silencios** siguen las mismas reglas: `-` ocupa una parte, como la
ocuparía una nota.

| Silencio | Escrito en 4/4 |
|---|---|
| un compás entero, en cualquier indicación | `\| - \|` |
| ocho compases | `\| - x8 \|` |
| de blanca | `- -` |
| de negra | `-` como un tiempo |
| de corchea | `-c` o `c-` |
| de semicorchea | `-ccc`, `c-cc` |
| de corchea con puntillo, luego semicorchea | `-__c` o `-.c` |

**Compases compuestos.** En 6/8 el tiempo es una negra con puntillo, así que
`ccc ccc` es un compás de corcheas contado en dos, como cuenta un músico.

```aratu
meter: 6/8
tempo: 70
hh = ccc ccc
bd = b__ b__
```

**Acentos.** Un símbolo en mayúscula se acentúa: `cC cc` acentúa el
contratiempo del tiempo 1.

**Barras.** `|` es la barra de compás, como en la partitura. Una línea puede
tener varios compases: `| cc cc | co cc |`. Una línea sin barras es un
compás.

**Grupos.** Los corchetes forman un grupo que ocupa una parte, y admiten
espacios dentro: `c[cc]` es una corchea y dos semicorcheas. El atajo `.`
vale `__`, así que `c.c` es `c__c`.

## 3. Batería

Cada pieza de la batería tiene una línea, y cada línea sigue las reglas del
ritmo. El símbolo decide el sonido, sea cual sea el nombre de la línea:

| Símbolo | Sonido |
|---|---|
| `c` | hi-hat cerrado |
| `o` | hi-hat abierto |
| `b` | bombo |
| `s` | caja |
| `r` | ride |
| `x` | crash |
| `t` | tom agudo |
| `m` | tom medio |
| `f` | tom de piso |
| `k` | aro (baqueta cruzada sobre la caja) |

```aratu
drums {
  hh = cc cc co cc
  bd = b - bb -
  sd = - s - S        // caja acentuada en el tiempo 4
}
```

Toda línea dentro de `drums { }` es de batería, igual que una línea llamada
`hh`, `bd`, `sd`, `cr`, `rd`, `ht`, `mt`, `ft`, `toms`, `rim`, `kick`,
`snare`, `hat`, `ride` o `crash` en cualquier lugar de la pieza. Un apóstrofo hace el golpe corto: `c'` es un
hi-hat seco. El hi-hat cerrado corta al abierto que todavía suena.

## 4. Melodías

Las notas se escriben en minúscula, con la octava: `do4` (o `c4`) es el do
central. Sostenidos y bemoles van después del nombre: `fa#4`, `mib4`. En
letras: `f#4`, `eb4`. La mayúscula acentúa: `Do4`. Un apóstrofo hace la
nota corta, staccato: `do4'`.

```aratu
melodia = | do4 mi4 sol4 _ | la4 sol4 mi4 do4 |
```

La octava sigue hasta que cambia, así que `| do4 mi sol do5 |` es do4, mi4,
sol4, do5. Una nota escrita antes de cualquier octava vale octava 4, y Aratu
lo avisa.

Dos o más notas en el mismo tiempo van entre corchetes: `[re mi]` son dos
corcheas, `[si si]` dos corcheas de si, `[mib sol]` mi bemol y luego sol,
`[do4 _ _ re]` una corchea con puntillo y una semicorchea. Los silencios y
prolongaciones pueden ir pegados a la nota sin corchetes (`-mi4` es un
silencio y luego una corchea), pero dos notas no: `mib` se lee como mi
bemol, así que para dos notas Aratu pide los corchetes.

Las notas que suenan juntas se unen con `+`: `do4+mi4+sol4` es un acorde de
do mayor en un tiempo.

## 5. Acordes y rasgueos

Una secuencia de acordes se escribe como un cifrado, en una línea llamada
`chords`. La regla de la división vale un nivel más arriba: los acordes que
comparten un compás lo dividen por igual.

```aratu
chords = | Am | F | C G | Am _ _ G |
```

C y G se quedan con medio compás cada uno; Am dura tres tiempos y G uno. Las
divisiones deben caer en los tiempos: tres acordes en un compás de 4/4 lo
dividirían entre los tiempos, así que Aratu pide que digas cuánto dura cada
uno.

Los acordes se escriben como siempre: `Am`, `F`, `G7`, `Cmaj7`, `Dm7b5`,
`E/G#`, y también con la escritura brasileña: `C7M`, `Bº`, `G7(b9)`.

**El rasgueo.** Una línea llamada `strum` usa las reglas del ritmo con
símbolos de guitarra:

| Símbolo | Significado |
|---|---|
| `d` | rasgueo hacia abajo |
| `u` | rasgueo hacia arriba |
| `m` | rasgueo apagado |
| `g` | golpe en la tapa (flamenco), sin las cuerdas |
| `t` | hacia abajo con golpe, todo junto |
| `-` | la mano pasa sin tocar |
| `_` | deja sonar |
| mayúscula | rasgueo acentuado |

```aratu
guitarra {
  chords = | Am | F | C G | G |
  strum  = D du -u du
}
```

El rasgueo se repite en cada compás y los acordes cambian solos. Hacia
abajo, las cuerdas suenan de la grave a la aguda, unos milisegundos una
después de otra; hacia arriba, de la aguda a la grave; los acentos golpean
más fuerte y más rápido. Sin rasgueo, los acordes suenan enteros, como un
teclado.

## 6. Matices

Las marcas de dinámica son las de la partitura: `ppp pp p mp mf f ff fff`, y
`sfz`, `fp`.

**Dentro de un compás**, la marca va en la nota con dos puntos, pegada o
con un espacio (`p:do4` o `p: do4`), y no cuenta como tiempo. Antes de corchetes, vale desde la primera nota
del grupo: `p:[do4< sol4]`. En `f:fa4`, la primera `f` es forte y lo demás
es la nota:

```aratu
melodia = | p:do4 mi4 sol4 do5 | f:sol4 _ _ _ |
```

**Los reguladores** se escriben como en la partitura: `<` crescendo, `>`
diminuendo, desde el tiempo siguiente hasta la próxima marca.

```aratu
melodia = | p:do4< re4 mi4 fa4 | f:sol4> fa4 mi4 re4 | p:do4 _ _ _ |
```

**Para una parte entera**, una línea cambia su nivel:

```text
piano: mf
piano: cresc to ff over 4 bars
drums: dim to p over 2 bars
all: fade out over 8 bars
```

Una línea de nivel lleva la parte entera hasta la dinámica que nombra:
`melodia: cresc to ff over 4 bars` sale de lo que está escrito y llega a ff
en el cuarto compás, aunque la línea esté escrita en p. Los acentos siguen
por encima de las otras notas, y `fade out` lleva la parte al silencio.

Cada marca vale una intensidad fija: ppp 16, pp 33, p 49, mp 64, mf 80, f
96, ff 112, fff 127. El acento suma a la marca en la que está la música. Un
regulador sin marca después avanza un escalón (de `mf` a `f`, por ejemplo)
hasta el final de la línea.

## 7. Tempo

```aratu
tempo: 96
at bar 3: rit to 70 over 2 bars
at bar 5: a tempo
hh = c c c c
```

| Escribe | Significa |
|---|---|
| `tempo: 96` | el primer tempo |
| `at bar 30: tempo 130` | un tempo nuevo en un compás |
| `rit to 80 over 2 bars` | ritardando (`rall` también vale) |
| `accel to 140 over 4 bars` | accelerando |
| `a tempo` | vuelve al tempo de antes del rit |
| `tempo primo` | vuelve al primer tempo |
| `tempo 120 -> 140 over 4 bars` | una rampa entre dos tempos |

**Cuándo ocurre un cambio.** Un cambio con `at bar N:` ocurre en ese compás.
Un cambio escrito solo ocurre donde empieza la pieza o, mientras tocas, en
el compás siguiente al momento en que lo escribes.

En un ritardando o accelerando el tempo cambia poco a poco, tiempo a tiempo,
como se lee en una partitura, y cada nota cae en su momento exacto.

**Calderón.** `^` en una nota la sostiene: `sol4^` dura el doble, y todas
las partes la esperan.

**Anacrusa.** `pickup: beat 4` hace que la pieza empiece en el tiempo 4 de
un primer compás incompleto. La anacrusa es el compás 0, y el compás 1 es el
primero completo, como cuentan los músicos. Una línea cuyo primer compás
mide lo mismo que la anacrusa empieza en ella; las demás empiezan en el
compás 1.

**Cambio de compás.** `at bar 17: meter 7/8`.

## 8. Expresión: rubato, swing y humanize

**Rubato** quiere decir tiempo robado, y el tiempo robado se devuelve.
`piano: rubato 10%` estira y encoge los tiempos dentro de cada compás y
llega a la barra a tiempo, de modo que el piano respira mientras la batería
mantiene el pulso. `all: rubato 10%` curva toda la pieza junta.

**Swing.** `swing 60%` retrasa cada segunda corchea (50% es recto, 66% es la
sensación de tresillo).

```aratu
swing 62%
hh = cc cc cc cc
bd = b - b -
```

**Humanize.** `humanize` agrega pequeñas desviaciones al azar de tiempo e
intensidad, como haría un músico; `humanize 5 ms` dice cuánto.

## 9. Patrones, secciones, repeticiones y forma

El material se prepara de antemano y recibe un nombre:

```aratu
pattern groove {
  hh = cc cc co cc
  bd = b - b -
  sd = - s - s
}

pattern fill {
  hh = cc cc cc cc
  bd = bb -b bb -b
  sd = ss ss SS SS
}

play groove
at bar 4: play fill
at bar 5: play groove
```

### Signos de repetición

```text
chords = |: Am | F |1 C | G :|2 C | C |      // repetición, primera y segunda casilla
chords = |: Am | F :| x4                      // el pasaje cuatro veces
chords = | Am | % | % | F |                   // % repite el compás anterior
chords = | Am | F | %% |                      // %% repite los dos anteriores
```

`|:` y `:|` marcan un pasaje que se toca dos veces, salvo que `x` diga
cuántas. `|1` y `|2` empiezan la primera y la segunda casilla.

### Saltos

Las señales van al principio del compás, los saltos al final:

```aratu
melodia = | do4 mi sol do5 | segno: re4 fa la _ | mi4 sol si _ | fine: do4 _ _ _ | sol4 _ _ _ D.S. al fine |
```

| Señal | Salto |
|---|---|
| `segno:` | `D.S.`, `D.S. al fine`, `D.S. al coda` |
| `fine:` | donde se detiene `al fine` |
| `coda:` | adonde va `to coda` |
| | `D.C.`, `D.C. al fine`: vuelve al principio |

La pieza se despliega como la lee un músico. Un salto sin su señal es un
error dicho con claridad: *"hay un D.S. en el compás 24 de `melodia`, pero
ningún segno para él"*.

### Forma

Las secciones son partes de la pieza con nombre, y una línea de forma las
ordena:

```aratu
section intro  { bajo = | la2 _ _ _ | mi2 _ _ _ | }
section verso  {
  bajo = | la2 _ mi2 _ | fa2 _ do3 _ |
  drums { hh = cc cc co cc; bd = b - b - }
}
section estribillo {
  guitarra { chords = | Am | F | C | G |; strum = D du -u du }
}
section final  { bajo = | la2 _ _ _ | }

form: intro verso x2 estribillo verso estribillo x2 final
```

Una sección guarda partes y líneas como la pieza entera, y dura lo que su
línea más larga. La forma toca las secciones en orden (`x2` toca una dos
veces) y luego vuelve a empezar. Un bloque puede ir en una sola línea, con
`;` entre las líneas.

### Variar un loop

```aratu
pattern groove { hh = cc cc co cc; bd = b - b -; sd = - s - s }
pattern fill   { hh = cc cc cc cc; sd = ss ss SS SS }
pattern crash  { cr = x - - - }

play groove x3, fill
every 8 bars, first: add crash
```

`play` elige el patrón que suena; `play groove x3, fill` toca una frase de
patrones en orden y la repite. `every 4 bars: play fill` toca el redoble en
el cuarto compás de cada cuatro, en lugar del groove; `add` toca un patrón
encima del que está sonando.

Un `play` vale hasta el siguiente: `at bar 8: play fill` toca el redoble
del compás 8 en adelante. `once` (una vez) toca el patrón una vez y vuelve:

```text
at bar 8: play fill once     // el redoble en el compás 8, el groove de nuevo en el 9
now: play fill once          // un redoble ya, después el groove donde estaría
now: add crash once          // un platillo encima, una vez
```

El patrón que estaba sonando sigue contando sus compases mientras tanto, así
que `play groove x3, fill` no pierde su lugar en la frase. Con `every`, el
patrón ya suena una vez en cada vuelta.

## 10. Tocar en vivo

Mientras la pieza suena, la cambias; no la reescribes.

- **Todo se repite.** Cada línea suena sin parar, cada una con su largo.
- **Un cambio es una línea.** `hh = cc cc oc cc` cambia el hi-hat y nada
  más; las otras partes siguen.
- **Los cambios caen en el tiempo.** Una edición entra en el compás
  siguiente; `now:` la hace entrar en el tiempo siguiente: `now: play fill
  once` trae un redoble en el tiempo siguiente, y el groove vuelve en la
  barra donde estaría.
- **Prepara de antemano.** `at bar 17: play fill` programa un cambio.
- **Salir y volver.** `guitarra: out` silencia una parte desde el compás
  siguiente y `guitarra: in` la trae de vuelta; `all: out` detiene todo.
  Funcionan con `at bar` y `every` también.
- **Nada vuelve a empezar.** Las partes mantienen su lugar en el compás
  cuando otra cambia, y mientras el texto tenga un error, sigue sonando la
  última versión que estaba bien.

**El escenario.** Para tocar frente al público, el botón **Escenario** en
Tocar (o Ctrl+Shift+F) deja solo el código en la pantalla, en pantalla
completa y con letra grande para el proyector. El borde destella en el
primer tiempo de cada compás, los símbolos se encienden cuando suenan y el
primer mensaje aparece en la esquina. Los botones de la esquina tocan y
detienen, cambian el tamaño de la letra (A− y A+), muestran el mapa del
ritmo abajo y salen del escenario; desaparecen cuando el ratón se queda
quieto. Esc también sale.

## 11. Transformaciones

Verbos cortos cambian una parte, una línea o un patrón sin reescribirlo:

| Verbo | Efecto |
|---|---|
| `hh.rotate 1` | desplaza el patrón una parte |
| `hh.density 50%` | conserva más o menos la mitad de los golpes |
| `drums.velocity 0.8` | toca más suave |
| `bajo.transpose +12` | una octava arriba |
| `melodia.invert` | invierte los intervalos |
| `melodia.retrograde` | la toca al revés |
| `melodia.augment` | duplica las figuras |
| `melodia.diminish` | divide las figuras a la mitad |

```aratu
melodia = | do4 re mi sol | la sol mi re |
melodia.transpose +7
```

## 12. Azar

Un loop que se repite siempre igual pronto suena mecánico. El azar le da
vida: la computadora decide, un poco al azar, qué tocar en cada vuelta. Es
el *ad lib* de la partitura.

**A veces.** Un `?` después de un símbolo hace que suene solo a veces, la
mitad de las veces si no se dice otra cosa; un porcentaje dice con qué
frecuencia:

```aratu
hh = cc cc c?30% cc
sd = - s - s?80%
```

**Uno u otro.** Las llaves eligen una entre varias opciones, separadas por
`|`, en cada vuelta del loop:

```aratu
bajo = | {do2|sol1} _ do2 _ |
hh   = {cc|co} cc cc cc
```

**El mismo azar otra vez.** Las opciones se sortean de nuevo en cada vuelta,
pero a partir de una semilla fija, así que la misma pieza improvisa igual
cada vez que se toca desde el principio. `seed 7` elige otra improvisación;
`seed random` hace cada presentación distinta. Cambiar una línea no cambia
la improvisación de las otras.

## 13. Sonidos y efectos

Cada parte tiene un sonido y una cadena de efectos, escrita como el camino
de la señal:

```aratu
bajo: synth.saw -> lowpass -> reverb
bajo = | do2 _ do2 mib2 | fa2 _ sol2 sib1 |
bajo {
  reverb.mix = 0.42
  lowpass.cutoff = 800 Hz
  lowpass.resonance = 0.3
}
```

Cada número que ajusta un parámetro también es un control: el editor dibuja
una perilla al lado, con la forma del parámetro (la de cutoff se mueve en
hercios en una escala logarítmica, la de mix de 0 a 1). Girar la perilla
reescribe el número, así que guardar el texto guarda la presentación.
Mantén Shift para pasos más finos. Un número escrito en otra unidad la
conserva: `1.2 kHz` gira en kilohercios, `40%` en porcentaje.

| Parámetro | Rango |
|---|---|
| `volume` | 0 a 1 |
| `pan` | -1 (izquierda) a 1 (derecha) |
| `attack`, `release` | 1 a 2000 ms, 5 a 5000 ms |
| `lowpass.cutoff`, `highpass.cutoff` | 20 a 20000 Hz |
| `lowpass.resonance`, `highpass.resonance` | 0 a 1 |
| `reverb.mix` | 0 a 1 |
| `delay.time` | 10 a 2000 ms |
| `delay.feedback`, `delay.mix` | 0 a 0,95, 0 a 1 |

Los sonidos son `synth.saw`, `synth.square`, `synth.sine`, `synth.triangle`
y `synth.pluck`; la batería, los rasgueos y los acordes eligen solos una
batería, una cuerda pulsada y un teclado. Son sonidos sencillos,
incorporados al navegador, hasta que el motor de Manguebyte los reemplace.

Los parámetros pueden cambiar por compás como todo lo demás: `at bar 16:
bajo.reverb.mix = 0.65`.

## 14. Referencia rápida

| Escribe | Significa |
|---|---|
| `meter: 4/4`, `tempo: 96`, `key: A minor` | el marco de la pieza |
| espacio | siguiente tiempo |
| `cc`, `cccc`, `ccc` | corcheas, semicorcheas, tresillo en un tiempo |
| `_` | sostiene |
| `-` | silencio |
| `\| - \|`, `\| - x8 \|` | un compás de silencio, ocho compases de silencio |
| `c.c` | corchea con puntillo y semicorchea |
| mayúscula | acento |
| `do4'` | staccato |
| `c?`, `c?30%` | a veces (la mitad de las veces, o con qué frecuencia) |
| `{do2\|sol1}` | uno u otro, elegido en cada vuelta |
| `seed 7`, `seed random` | la misma improvisación otra vez, o una nueva cada vez |
| `\|` | barra de compás |
| `do4 mi sol` | notas, con la octava que sigue |
| `fa#4`, `mib4`, `c4` | alteraciones, letras |
| `[re mi]` | notas agrupadas en un tiempo (necesario para dos o más) |
| `do4+mi4+sol4` | notas juntas |
| `\| Am \| F \| C G \|` | acordes; compases divididos por igual |
| `d u m` | rasgueo hacia abajo, hacia arriba, apagado |
| `p:`, `mf:`, `f:` | dinámica en una nota |
| `<`, `>` | crescendo, diminuendo |
| `cresc to ff over 4 bars` | cambio de dinámica de una parte entera |
| `rit`, `accel`, `a tempo`, `tempo primo` | cambios de tempo |
| `tempo 120 -> 140 over 4 bars` | rampa de tempo |
| `sol4^` | calderón |
| `pickup: beat 4` | anacrusa |
| `rubato 10%`, `swing 60%`, `humanize` | expresión |
| `pattern nombre { }`, `play nombre`, `add nombre` | patrones con nombre, en orden o encima |
| `\|: :\|`, `:\| x4`, `\|1 \|2` | repetición, veces, casillas |
| `%`, `%%` | repite el compás anterior, los dos anteriores |
| `segno:`, `coda:`, `fine:` | señales |
| `D.C.`, `D.S. al fine`, `D.S. al coda`, `to coda` | saltos |
| `section nombre { }`, `form: intro verso x2 estribillo` | secciones y forma |
| `every 4 bars: play fill` | variación dentro del loop |
| `play fill once`, `add crash once` | un patrón una vez, después vuelve |
| `at bar 30:` | cambio programado |
| `now:` | cambio en el tiempo siguiente |
| `guitarra: out`, `guitarra: in` | silencia una parte, la trae de vuelta |
| `->` | camino de la señal |
| `parte.parámetro = valor` | un parámetro, dibujado como perilla |
