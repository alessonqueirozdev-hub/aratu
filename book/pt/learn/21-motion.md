# Números que se movem

Um DJ gira o botão do filtro devagar e a pista inteira sente a música
abrindo. Um guitarrista pisa no pedal e o som vai e volta. No Aratu um
número também pode andar sozinho, de um valor a outro, enquanto a música
toca.

## De um valor a outro: `->`

Escreva o valor de partida, uma seta, o valor de chegada e em quanto tempo,
com `over` (ou `em`):

```aratu
tempo: 100
baixo: synth.saw -> lowpass
baixo = | dó2 dó3 dó2 dó3 | mib2 mib3 mib2 mib3 |
baixo.lowpass.cutoff = 300 Hz -> 4000 Hz over 4 bars
```

O filtro abre ao longo de quatro compassos e fica aberto. Os dois números
ganham knob, e enquanto toca um segundo ponteiro mostra onde o número está
agora.

O cutoff anda na escala do ouvido: no meio do caminho entre 300 e 4000 Hz
ele está em cerca de 1100 Hz, não em 2150, e por isso a subida soa igual do
começo ao fim.

## Vai e volta: `<->`

Com `<->` no lugar da seta, o número vai e volta sem parar: abre em quatro
compassos, fecha em quatro, abre de novo.

```aratu
tempo: 100
baixo: synth.saw -> lowpass
baixo = | dó2 dó3 dó2 dó3 | mib2 mib3 mib2 mib3 |
baixo.lowpass.cutoff = 300 Hz <-> 4000 Hz over 4 bars
```

## Outra duração na volta, e recomeçar

| Escreva | O que faz |
|---|---|
| `A -> B over 4 bars` | vai de A a B em 4 compassos e fica em B |
| `A <-> B over 4 bars` | vai e volta, 4 compassos para cada lado, sem parar |
| `A -> B over 4 bars, back over 2 bars` | sobe em 4, volta em 2, uma vez |
| `A <-> B over 4 bars, back over 1 bar` | sobe em 4, volta em 1, sem parar |
| `A -> B over 1 bar, loop` | sobe e recomeça de A, como uma rampa |

Em português: `em 4 compassos, volta em 2 compassos`, e `repete` no lugar
de `loop`.

Um `loop` curto faz efeitos rítmicos. Aqui o volume do pad sobe a cada
tempo e recomeça, como um compressor "respirando":

```aratu
tempo: 120
pad: synth.pad
pad = | dó4+mib4+sol4 _ _ _ |
drums { bd = b b b b }
pad.volume = 0.1 -> 0.8 over 1 beat, loop
```

A duração pode ser em compassos (`bars`), tempos (`beats`), colcheias
(`eighths`) ou semicolcheias (`sixteenths`).

## Quando começa

- Escrita sozinha, a linha começa no começo da peça, ou no compasso
  seguinte, se você a escrever com a música tocando. Girar um dos knobs
  muda o caminho sem fazê-lo recomeçar.
- Com `at bar 9:` na frente, começa no compasso 9.
- Com `every 8 bars:`, termina no fim de cada oito compassos, como uma
  virada que prepara a volta.
- Com `every 8 bars, first:`, começa no primeiro de cada oito.

```aratu
tempo: 120
baixo: synth.saw -> lowpass
baixo = | dó2 dó3 dó2 dó3 |
drums { hh = cc cc cc cc; bd = b b b b }
baixo.lowpass.cutoff = 400 Hz
every 4 bars: baixo.lowpass.cutoff = 400 Hz -> 6000 Hz over 2 bars
```

## Exercícios

Faça o filtro abrir de 300 Hz a 4000 Hz em 4 compassos.

```aratu exercise sound=1 bars=4
baixo: synth.saw -> lowpass
baixo = | dó2 dó3 dó2 dó3 |
baixo.lowpass.cutoff = 300 Hz
```

```aratu answer
baixo: synth.saw -> lowpass
baixo = | dó2 dó3 dó2 dó3 |
baixo.lowpass.cutoff = 300 Hz -> 4000 Hz over 4 bars
```

Agora o filtro vai e volta sem parar, dois compassos para cada lado.

```aratu exercise sound=1 bars=8
baixo: synth.saw -> lowpass
baixo = | dó2 dó3 dó2 dó3 |
baixo.lowpass.cutoff = 300 Hz -> 4000 Hz over 2 bars
```

```aratu answer
baixo: synth.saw -> lowpass
baixo = | dó2 dó3 dó2 dó3 |
baixo.lowpass.cutoff = 300 Hz <-> 4000 Hz over 2 bars
```

No compasso 3, o reverb do teclado sobe para 0.6, de uma vez.

```aratu exercise sound=1 bars=4
teclado: keys.epiano -> reverb
teclado = | dó4+mi4+sol4 _ _ _ |
```

```aratu answer
teclado: keys.epiano -> reverb
teclado = | dó4+mi4+sol4 _ _ _ |
at bar 3: teclado.reverb.mix = 0.6
```
