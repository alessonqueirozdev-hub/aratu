# Sonidos, efectos y perillas

Cada parte tiene un sonido y una cadena de efectos, escrita como el camino
de la señal, de izquierda a derecha, como en un pedalboard:

```aratu
tempo: 100
bajo: synth.saw -> lowpass -> reverb
bajo = | do2 _ do2 mib2 | fa2 _ sol2 sib1 |
```

Los sonidos de ahora son `synth.saw`, `synth.square`, `synth.sine`,
`synth.triangle` y `synth.pluck`. La batería, la guitarra y los acordes
eligen su sonido solos.

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

> Estos son sonidos sencillos, incorporados al navegador. El motor de
> Manguebyte traerá instrumentos y efectos de verdad, y los mismos textos
> sonarán mejor sin cambiar nada.
