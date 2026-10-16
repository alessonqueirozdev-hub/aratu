# Sonidos, efectos y perillas

Cada parte tiene un sonido y una cadena de efectos, escrita como el camino
de la señal, de izquierda a derecha, como en un pedalboard:

```aratu
tempo: 100
bajo: synth.saw -> lowpass -> reverb
bajo = | do2 _ do2 mib2 | fa2 _ sol2 sib1 |
```

El sonido es el código al principio de la cadena. Hay decenas: abre el
panel **Sonidos**, junto a los mensajes en Tocar, para escuchar cada uno y
poner el código en el texto con un clic. Cambia `synth.saw` por
`bass.slap`, `bass.808` o `piano.grand` y escucha:

```aratu
tempo: 100
bajo: bass.slap
bajo = | mi2 Mi3 mi2 Mi3 | la2 La3 la2 La3 |
```

> En `bass.slap`, la nota en mayúscula es el *pop*: la cuerda tirada, que
> chasquea.

## Los números se vuelven perillas

Los parámetros se ajustan con números, y cada número tiene una perilla
redonda al lado. Arrastra la perilla hacia arriba o hacia abajo: el número
en el texto cambia y el sonido cambia con él, al instante.

```aratu
tempo: 100
bajo: synth.saw -> lowpass -> reverb
bajo = | do2 _ do2 mib2 | fa2 _ sol2 sib1 |
bajo {
  lowpass.cutoff = 800 Hz
  lowpass.resonance = 0.3
  reverb.mix = 0.2
}
```

La perilla de `lowpass.cutoff` se mueve en una escala logarítmica en
hercios, como percibe el oído; la de `reverb.mix` va de 0 a 1. Mantén
Shift para pasos más finos.

| Parámetro | Rango |
|---|---|
| `volume` | 0 a 1 |
| `pan` | -1 (izquierda) a 1 (derecha) |
| `lowpass.cutoff`, `highpass.cutoff` | 20 a 20000 Hz |
| `lowpass.resonance` | 0 a 1 |
| `reverb.mix` | 0 a 1 |
| `delay.time` | 10 a 2000 ms |
| `delay.feedback`, `delay.mix` | 0 a 0,95, 0 a 1 |

Un parámetro también puede cambiar en un compás: `at bar 9: bajo.reverb.mix = 0.6`.

## Parámetros que se mueven

Un parámetro puede ir de un valor a otro poco a poco, como en un crescendo.
`->` va una vez y se queda; `over` dice en cuánto tiempo:

```aratu
tempo: 100
bajo: synth.saw -> lowpass
bajo = | do2 do3 do2 do3 | mib2 mib3 mib2 mib3 |
bajo.lowpass.cutoff = 300 Hz -> 4000 Hz over 4 bars
```

Con `<->` en lugar de la flecha, va y vuelve sin parar: sube en cuatro
compases, baja en cuatro, sube otra vez.

```aratu
tempo: 100
bajo: synth.saw -> lowpass
bajo = | do2 do3 do2 do3 | mib2 mib3 mib2 mib3 |
bajo.lowpass.cutoff = 300 Hz <-> 4000 Hz over 4 bars
```

| Escribe | Qué hace |
|---|---|
| `A -> B over 4 bars` | va de A a B en 4 compases y se queda en B |
| `A <-> B over 4 bars` | va y vuelve, 4 compases para cada lado, sin parar |
| `A -> B over 4 bars, back over 2 bars` | sube en 4, vuelve en 2, una vez |
| `A <-> B over 4 bars, back over 1 bar` | sube en 4, vuelve en 1, sin parar |
| `A -> B over 1 bar, loop` | sube y empieza otra vez desde A, como una rampa |

La duración puede ser en compases (`bars`), tiempos (`beats`), corcheas
(`eighths`) o semicorcheas (`sixteenths`): `over 2 beats`,
`over 3 eighths`.

El cutoff anda en la misma escala que la perilla, la del oído: a mitad de
camino entre 300 y 4000 Hz está en unos 1100 Hz, no en 2150. Los dos
números tienen perilla, y mientras suena una segunda aguja muestra dónde
está el parámetro ahora.

> Escrita con la música sonando, la línea empieza a moverse en el compás
> siguiente, y girar una de las perillas cambia el camino sin hacerlo
> empezar de nuevo. Con `at bar 9:` delante, empieza en el compás 9; con
> `every 8 bars:`, termina al final de cada ocho compases, como un
> redoble; con `every 8 bars, first:`, empieza en el primero de cada ocho.

> Los sonidos grabados y la batería analógica ya suenan con el motor de
> Manguebyte. Los efectos también pasarán a él, y los mismos textos sonarán
> mejor sin cambiar nada.
