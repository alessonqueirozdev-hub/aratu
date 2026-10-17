# Sons, efeitos e knobs

Cada parte tem um som e uma cadeia de efeitos, escrita como o caminho do
sinal, da esquerda para a direita, como num pedalboard:

```aratu
tempo: 100
baixo: synth.saw -> lowpass -> reverb
baixo = | dó2 _ dó2 mib2 | fá2 _ sol2 sib1 |
```

O som é o código no começo da cadeia. Há dezenas: abra o painel **Sons**,
ao lado das mensagens em Tocar, para ouvir cada um e pôr o código no texto
com um clique. Troque `synth.saw` por `bass.slap`, `bass.808` ou
`piano.grand` e ouça:

```aratu
tempo: 100
baixo: bass.slap
baixo = | mi2 Mi3 mi2 Mi3 | lá2 Lá3 lá2 Lá3 |
```

> No `bass.slap`, a nota em maiúscula é o *pop*: a corda puxada, que estala.

## Os números viram botões

Os parâmetros se ajustam com números, e cada número ganha um botão
redondo, um knob, ao lado. Arraste o knob para cima ou para baixo: o
número no texto muda e o som muda junto, na hora.

```aratu
tempo: 100
baixo: synth.saw -> lowpass -> reverb
baixo = | dó2 _ dó2 mib2 | fá2 _ sol2 sib1 |
baixo {
  lowpass.cutoff = 800 Hz
  lowpass.resonance = 0.3
  reverb.mix = 0.2
}
```

O knob do `lowpass.cutoff` anda numa escala logarítmica em hertz, como o
ouvido percebe; o do `reverb.mix` vai de 0 a 1. Segure Shift para passos
mais finos.

| Parâmetro | Faixa |
|---|---|
| `volume` | 0 a 1 |
| `pan` | -1 (esquerda) a 1 (direita) |
| `lowpass.cutoff`, `highpass.cutoff` | 20 a 20000 Hz |
| `lowpass.resonance` | 0 a 1 |
| `reverb.mix` | 0 a 1 |
| `delay.time` | 10 a 2000 ms |
| `delay.feedback`, `delay.mix` | 0 a 0,95, 0 a 1 |

Um parâmetro também pode mudar num compasso: `at bar 9: baixo.reverb.mix = 0.6`.

## Parâmetros que se movem

Um parâmetro pode ir de um valor a outro aos poucos, como num crescendo.
`->` vai uma vez e fica lá; `over` diz em quanto tempo:

```aratu
tempo: 100
baixo: synth.saw -> lowpass
baixo = | dó2 dó3 dó2 dó3 | mib2 mib3 mib2 mib3 |
baixo.lowpass.cutoff = 300 Hz -> 4000 Hz over 4 bars
```

Com `<->` no lugar da seta, ele vai e volta sem parar: sobe em quatro
compassos, desce em quatro, sobe de novo.

```aratu
tempo: 100
baixo: synth.saw -> lowpass
baixo = | dó2 dó3 dó2 dó3 | mib2 mib3 mib2 mib3 |
baixo.lowpass.cutoff = 300 Hz <-> 4000 Hz over 4 bars
```

| Escreva | O que faz |
|---|---|
| `A -> B over 4 bars` | vai de A a B em 4 compassos e fica em B |
| `A <-> B over 4 bars` | vai e volta, 4 compassos para cada lado, sem parar |
| `A -> B over 4 bars, back over 2 bars` | sobe em 4, volta em 2, uma vez |
| `A <-> B over 4 bars, back over 1 bar` | sobe em 4, volta em 1, sem parar |
| `A -> B over 1 bar, loop` | sobe e recomeça de A, como uma rampa |

A duração pode ser em compassos (`bars`), tempos (`beats`), colcheias
(`eighths`) ou semicolcheias (`sixteenths`): `over 2 beats`,
`over 3 eighths`.

O cutoff anda na mesma escala do knob, a do ouvido: no meio do caminho entre
300 e 4000 Hz ele está em cerca de 1100 Hz, não em 2150. Os dois números
ganham knob, e enquanto toca um segundo ponteiro mostra onde o parâmetro
está agora.

> Escrita com a música tocando, a linha começa a se mover no compasso
> seguinte, e girar um dos knobs muda o caminho sem fazê-lo recomeçar. Com
> `at bar 9:` na frente, começa no compasso 9; com `every 8 bars:`,
> termina no fim de cada oito compassos, como uma virada; com
> `every 8 bars, first:`, começa no primeiro de cada oito.

> Todos esses sons, e os efeitos, tocam pelo motor do Manguebyte, o motor
> de áudio do ecossistema. Um número escrito sem parte vale para a obra
> inteira: `reverb.mix = 0.5` põe tudo no reverb, e `todos: chorus` põe
> chorus em todas as partes.
