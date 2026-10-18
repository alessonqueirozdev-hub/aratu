# Sonidos y efectos

Hasta aquí cada parte sonaba con el sonido que elegía Aratu. Ahora eliges
tú: un piano, una guitarra de acero, un bajo slap, un sintetizador. Y pasas
el sonido por efectos, como un guitarrista pasa la guitarra por los
pedales.

## El sonido de una parte

Escribe el nombre de la parte, dos puntos y el código del sonido:

```aratu
tempo: 100
bajo: bass.slap
bajo = | mi2 Mi3 mi2 Mi3 | la2 La3 la2 La3 |
```

En `bass.slap`, la nota en mayúscula es el *pop*: la cuerda tirada, que
chasquea. Los códigos tienen dos partes, la familia y el instrumento:

| Familia | Algunos sonidos |
|---|---|
| teclados | `piano.grand`, `piano.upright`, `keys.epiano`, `keys.organ`, `keys.harpsichord` |
| percusión de láminas y campanas | `mallets.marimba`, `mallets.vibraphone`, `mallets.celesta`, `mallets.steelpan`, `bells.tubular` |
| cuerdas pulsadas | `guitar.nylon`, `guitar.steel`, `harp` |
| cuerdas frotadas | `strings.violin`, `strings.viola`, `strings.cello`, `strings.bass`, `strings.section`, `strings.violin_pizz` |
| bajos | `bass.finger`, `bass.slap`, `bass.808`, `bass.acid`, `bass.synth` |
| vientos | `flute`, `clarinet`, `sax`, `trumpet`, `trombone`, `sax.tenor`, `recorder.alto` |
| voces | `choir.aah`, `choir.ooh`, `voice.solo` |
| sintetizadores | `synth.saw`, `synth.pad`, `synth.supersaw`, `poly.trance_supersaw`, `poly.reese`, `poly.house_pluck`, `poly.string_machine` |
| baterías | `drums.kit`, `drums.808`, `drums.909`, `drums.606`, `drums.cr78`, `drums.trap`, `drums.house`, `drums.techno`, `drums.dnb`, `drums.lofi`, `drums.acoustic`, `drums.alesson` |

Abre el panel **Sonidos**, junto a los mensajes en Tocar, para escuchar
cada uno y poner el código en el texto con un clic. Todos suenan con el
motor de **Manguebyte**, el motor de audio del ecosistema de Aratu.

> Cambia `bass.slap` por `bass.808` y después por `guitar.steel`. Las notas
> son las mismas; el instrumento cambia.

Los códigos `poly.` son los sonidos listos del sintetizador polifónico de
Manguebyte: más de sesenta, de pads y leads a bajos, plucks, stabs y
efectos como `poly.riser`. Las baterías electrónicas imitan máquinas
famosas (`drums.808`, `drums.909`, `drums.606`, `drums.cr78`) y estilos
(`drums.trap`, `drums.house`, `drums.techno`, `drums.dnb`, `drums.lofi`);
las piezas tienen los mismos nombres en todas, así que cambiar el kit no
cambia el ritmo. Las cuerdas frotadas, los vientos y el coro se modelan
mientras suenan, sin nada que descargar.

> Cambia `drums: drums.808` por `drums.909` y después por `drums.lofi` en
> un ritmo tuyo. Después pon la melodía en un `flute` y en un
> `strings.violin`.

## La cadena de efectos

Después del sonido, unas flechas llevan la señal por los efectos, de
izquierda a derecha, como en un pedalboard:

```aratu
tempo: 100
bajo: synth.saw -> lowpass -> reverb
bajo = | do2 _ do2 mib2 | fa2 _ sol2 sib1 |
```

| Efecto | Qué hace |
|---|---|
| `lowpass` | apaga los agudos, como poner la mano delante de la boca |
| `highpass` | quita los graves, como una radio pequeña |
| `eq` | sube o baja graves, medios y agudos |
| `drive` | distorsiona, como un amplificador al máximo |
| `valvula` (o `tube`) | calienta y satura, como un amplificador de válvulas |
| `bitcrush` | sonido de videojuego antiguo |
| `compressor` | iguala el volumen, los fuertes y los suaves |
| `chorus` | duplica el sonido, que queda ancho y brillante |
| `flanger`, `phaser` | el sonido "volando", como un avión que pasa |
| `tremolo` | el volumen latiendo |
| `autopan` | el sonido yendo de un parlante al otro |
| `delay` | eco |
| `reverb` | el sonido en una sala, una iglesia, un salón |

## Los números se vuelven perillas

Cada efecto tiene sus números, escritos como `parte.efecto.número`. Y cada
número tiene una perilla redonda al lado: arrástrala hacia arriba o hacia
abajo y el sonido cambia al instante.

```aratu
tempo: 100
bajo: synth.saw -> lowpass -> delay
bajo = | do2 _ do2 mib2 | fa2 _ sol2 sib1 |
bajo.lowpass.cutoff = 800 Hz
bajo.lowpass.resonance = 0.3
bajo.delay.mix = 0.25
```

Los números de una parte también pueden ir en un bloque con su nombre, sin
repetir el nombre:

```aratu
tempo: 100
bajo: synth.saw -> lowpass -> delay
bajo = | do2 _ do2 mib2 | fa2 _ sol2 sib1 |
bajo {
  lowpass.cutoff = 800 Hz
  delay.time = 375 ms
  delay.feedback = 0.4
}
```

Los números más usados:

| Número | Qué es |
|---|---|
| `lowpass.cutoff`, `highpass.cutoff` | dónde corta el filtro, de 20 a 20000 Hz |
| `lowpass.resonance` | un pico en el corte, de 0 a 1 |
| `drive.amount` | cuánto distorsiona, de 0 a 36 dB |
| `chorus.mix`, `delay.mix` | cuánto del efecto se escucha, de 0 a 1 |
| `delay.time` | el tiempo del eco, en s o ms |
| `delay.feedback` | cuántas veces vuelve el eco, de 0 a casi 1 |
| `tremolo.rate` | cuántas veces por segundo late el volumen |

El manual tiene la lista completa. La perilla del cutoff se mueve en una
escala de frecuencia como la percibe el oído; mantén Shift para pasos más
finos.

## Ecualizador y válvula

Al lado de `eq`, `valvula`, `drive`, `lowpass` y `highpass` en una cadena,
el editor dibuja una curva pequeña que muestra lo que hace el efecto, y
cambia cuando giras las perillas. La del `eq` es la respuesta del
ecualizador: los graves a la izquierda, los agudos a la derecha, hacia
arriba lo que sube. Haz clic en ella y el ecualizador se abre grande:
arrastra los puntos de los graves, los medios y los agudos, y los números
se escriben en el código. La rueda del ratón en el punto de los medios
estrecha o ensancha la banda.

```aratu
tempo: 96
guitarra: guitar.steel -> eq -> valvula -> reverb
guitarra = | mi3 sol3 la3 sol3 | mi3 _ re3 _ |
guitarra.eq.low_gain = -6 dB
guitarra.eq.mid_gain = 4 dB
guitarra.eq.mid_freq = 900 Hz
guitarra.valvula.drive = 12 dB
```

La válvula es el drive de Manguebyte ajustado como una válvula: la curva
suave que redondea los picos, un poco de ganancia y el volumen devuelto
a su lugar. Sus números son `valvula.drive` (de 0 a 24 dB),
`valvula.output` y `valvula.mix`. La curva a su lado muestra cuánto se
aplana el sonido, y brilla más cuanto más fuerte se la toca. Como la
válvula y el drive son la misma etapa, una cadena usa una u otro.

## Un número que cambia en un compás

Como cualquier cambio, un número puede cambiar en un compás:

```aratu
tempo: 100
bajo: synth.saw -> lowpass
bajo = | do2 do3 do2 do3 |
bajo.lowpass.cutoff = 400 Hz
at bar 3: bajo.lowpass.cutoff = 3000 Hz
```

## Ejercicios

Estos ejercicios comprueban también el **sonido**: el instrumento, los
efectos y los números.

Haz que el bajo suene con el bajo slap, `bass.slap`.

```aratu exercise sound=1
bajo: synth.saw
bajo = | mi2 Mi3 mi2 Mi3 |
```

```aratu answer
bajo: bass.slap
bajo = | mi2 Mi3 mi2 Mi3 |
```

Pon un delay en la cadena del teclado, después del chorus.

```aratu exercise sound=1
teclado: keys.epiano -> chorus
teclado = | do4+mi4+sol4 _ _ _ |
```

```aratu answer
teclado: keys.epiano -> chorus -> delay
teclado = | do4+mi4+sol4 _ _ _ |
```

Apaga el sintetizador: el cutoff del lowpass en 600 Hz.

```aratu exercise sound=1
bajo: synth.saw -> lowpass
bajo = | do2 _ do3 _ |
bajo.lowpass.cutoff = 5000 Hz
```

```aratu answer
bajo: synth.saw -> lowpass
bajo = | do2 _ do3 _ |
bajo.lowpass.cutoff = 600 Hz
```

Pasa la guitarra por una válvula, con 12 dB de drive.

```aratu exercise sound=1
guitarra: guitar.steel -> reverb
guitarra = | mi3 sol3 la3 sol3 |
```

```aratu answer
guitarra: guitar.steel -> valvula -> reverb
guitarra = | mi3 sol3 la3 sol3 |
guitarra.valvula.drive = 12 dB
```
