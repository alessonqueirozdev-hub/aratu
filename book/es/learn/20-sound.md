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
| percusión de láminas y campanas | `mallets.marimba`, `mallets.vibraphone`, `mallets.kalimba`, `bells.tubular` |
| cuerdas pulsadas | `guitar.nylon`, `guitar.steel`, `harp` |
| bajos | `bass.finger`, `bass.slap`, `bass.808`, `bass.acid`, `bass.synth` |
| vientos | `sax.tenor`, `recorder.alto` |
| sintetizadores | `synth.saw`, `synth.square`, `synth.pad`, `synth.supersaw`, `synth.lead` |
| baterías | `drums.kit`, `drums.808`, `drums.acoustic`, `drums.alesson` |

Abre el panel **Sonidos**, junto a los mensajes en Tocar, para escuchar
cada uno y poner el código en el texto con un clic. Todos suenan con el
motor de **Manguebyte**, el motor de audio del ecosistema de Aratu.

> Cambia `bass.slap` por `bass.808` y después por `guitar.steel`. Las notas
> son las mismas; el instrumento cambia.

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
