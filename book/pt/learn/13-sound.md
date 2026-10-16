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

> Estes são sons simples, embutidos no navegador. O motor do Manguebyte vai
> trazer instrumentos e efeitos de verdade, e os mesmos textos vão soar
> melhor sem mudar nada.
