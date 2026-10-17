# Números que se mueven

Un DJ gira despacio la perilla del filtro y toda la pista siente que la
música se abre. Un guitarrista pisa el pedal y el sonido va y vuelve. En
Aratu un número también puede andar solo, de un valor a otro, mientras la
música suena.

## De un valor a otro: `->`

Escribe el valor de partida, una flecha, el valor de llegada y en cuánto
tiempo, con `over` (o `en`):

```aratu
tempo: 100
bajo: synth.saw -> lowpass
bajo = | do2 do3 do2 do3 | mib2 mib3 mib2 mib3 |
bajo.lowpass.cutoff = 300 Hz -> 4000 Hz over 4 bars
```

El filtro se abre a lo largo de cuatro compases y se queda abierto. Los dos
números tienen perilla, y mientras suena una segunda aguja muestra dónde
está el número ahora.

El cutoff anda en la escala del oído: a mitad de camino entre 300 y
4000 Hz está en unos 1100 Hz, no en 2150, y por eso la subida suena pareja
del principio al final.

## Va y vuelve: `<->`

Con `<->` en lugar de la flecha, el número va y vuelve sin parar: se abre en
cuatro compases, se cierra en cuatro, se abre otra vez.

```aratu
tempo: 100
bajo: synth.saw -> lowpass
bajo = | do2 do3 do2 do3 | mib2 mib3 mib2 mib3 |
bajo.lowpass.cutoff = 300 Hz <-> 4000 Hz over 4 bars
```

## Otra duración a la vuelta, y empezar de nuevo

| Escribe | Qué hace |
|---|---|
| `A -> B over 4 bars` | va de A a B en 4 compases y se queda en B |
| `A <-> B over 4 bars` | va y vuelve, 4 compases para cada lado, sin parar |
| `A -> B over 4 bars, back over 2 bars` | sube en 4, vuelve en 2, una vez |
| `A <-> B over 4 bars, back over 1 bar` | sube en 4, vuelve en 1, sin parar |
| `A -> B over 1 bar, loop` | sube y empieza otra vez desde A, como una rampa |

En español: `en 4 compases, vuelta en 2 compases`, y `repite` en lugar de
`loop`.

Un `loop` corto hace efectos rítmicos. Aquí el volumen del pad sube en cada
tiempo y vuelve a empezar, como un compresor "respirando":

```aratu
tempo: 120
pad: synth.pad
pad = | do4+mib4+sol4 _ _ _ |
drums { bd = b b b b }
pad.volume = 0.1 -> 0.8 over 1 beat, loop
```

La duración puede ser en compases (`bars`), tiempos (`beats`), corcheas
(`eighths`) o semicorcheas (`sixteenths`).

## Cuándo empieza

- Escrita sola, la línea empieza al principio de la pieza, o en el compás
  siguiente, si la escribes con la música sonando. Girar una de las
  perillas cambia el camino sin hacerlo empezar de nuevo.
- Con `at bar 9:` delante, empieza en el compás 9.
- Con `every 8 bars:`, termina al final de cada ocho compases, como un
  redoble que prepara la vuelta.
- Con `every 8 bars, first:`, empieza en el primero de cada ocho.

```aratu
tempo: 120
bajo: synth.saw -> lowpass
bajo = | do2 do3 do2 do3 |
drums { hh = cc cc cc cc; bd = b b b b }
bajo.lowpass.cutoff = 400 Hz
every 4 bars: bajo.lowpass.cutoff = 400 Hz -> 6000 Hz over 2 bars
```

## Ejercicios

Haz que el filtro se abra de 300 Hz a 4000 Hz en 4 compases.

```aratu exercise sound=1 bars=4
bajo: synth.saw -> lowpass
bajo = | do2 do3 do2 do3 |
bajo.lowpass.cutoff = 300 Hz
```

```aratu answer
bajo: synth.saw -> lowpass
bajo = | do2 do3 do2 do3 |
bajo.lowpass.cutoff = 300 Hz -> 4000 Hz over 4 bars
```

Ahora el filtro va y vuelve sin parar, dos compases para cada lado.

```aratu exercise sound=1 bars=8
bajo: synth.saw -> lowpass
bajo = | do2 do3 do2 do3 |
bajo.lowpass.cutoff = 300 Hz -> 4000 Hz over 2 bars
```

```aratu answer
bajo: synth.saw -> lowpass
bajo = | do2 do3 do2 do3 |
bajo.lowpass.cutoff = 300 Hz <-> 4000 Hz over 2 bars
```

En el compás 3, la reverb del teclado sube a 0.6, de golpe.

```aratu exercise sound=1 bars=4
teclado: keys.epiano -> reverb
teclado = | do4+mi4+sol4 _ _ _ |
```

```aratu answer
teclado: keys.epiano -> reverb
teclado = | do4+mi4+sol4 _ _ _ |
at bar 3: teclado.reverb.mix = 0.6
```
