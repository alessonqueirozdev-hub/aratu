# Sons e efeitos

Até aqui cada parte tocava com o som que o Aratu escolhia. Agora você
escolhe: um piano, um violão de aço, um baixo slap, um sintetizador. E
passa o som por efeitos, como um guitarrista passa a guitarra pelos pedais.

## O som de uma parte

Escreva o nome da parte, dois-pontos e o código do som:

```aratu
tempo: 100
baixo: bass.slap
baixo = | mi2 Mi3 mi2 Mi3 | lá2 Lá3 lá2 Lá3 |
```

No `bass.slap`, a nota em maiúscula é o *pop*: a corda puxada, que
estala. Os códigos têm duas partes, a família e o instrumento:

| Família | Alguns sons |
|---|---|
| teclados | `piano.grand`, `piano.upright`, `keys.epiano`, `keys.organ`, `keys.harpsichord` |
| percussão de teclas e sinos | `mallets.marimba`, `mallets.vibraphone`, `mallets.kalimba`, `bells.tubular` |
| cordas dedilhadas | `guitar.nylon`, `guitar.steel`, `harp` |
| baixos | `bass.finger`, `bass.slap`, `bass.808`, `bass.acid`, `bass.synth` |
| sopros | `sax.tenor`, `recorder.alto` |
| sintetizadores | `synth.saw`, `synth.square`, `synth.pad`, `synth.supersaw`, `synth.lead` |
| baterias | `drums.kit`, `drums.808`, `drums.acoustic`, `drums.alesson` |

Abra o painel **Sons**, ao lado das mensagens em Tocar, para ouvir cada um
e pôr o código no texto com um clique. Todos tocam pelo motor do
**Manguebyte**, o motor de áudio do ecossistema do Aratu.

> Troque `bass.slap` por `bass.808` e depois por `guitar.steel`. As notas
> são as mesmas; o instrumento muda.

## A cadeia de efeitos

Depois do som, setas levam o sinal por efeitos, da esquerda para a direita,
como num pedalboard:

```aratu
tempo: 100
baixo: synth.saw -> lowpass -> reverb
baixo = | dó2 _ dó2 mib2 | fá2 _ sol2 sib1 |
```

| Efeito | O que faz |
|---|---|
| `lowpass` | abafa os agudos, como pôr a mão na frente da boca |
| `highpass` | tira os graves, como um rádio pequeno |
| `eq` | aumenta ou diminui graves, médios e agudos |
| `drive` | distorce, como um amplificador no talo |
| `valvula` (ou `tube`) | aquece e satura, como um amplificador de válvula |
| `bitcrush` | som de videogame antigo |
| `compressor` | iguala o volume, os fortes e os fracos |
| `chorus` | dobra o som, que fica largo e brilhante |
| `flanger`, `phaser` | o som "voando", como um avião passando |
| `tremolo` | o volume pulsando |
| `autopan` | o som indo de uma caixa de som para a outra |
| `delay` | eco |
| `reverb` | o som numa sala, numa igreja, num salão |

## Os números viram botões

Cada efeito tem os seus números, escritos como `parte.efeito.número`. E
cada número ganha um botão redondo, um knob, ao lado: arraste para cima ou
para baixo e o som muda na hora.

```aratu
tempo: 100
baixo: synth.saw -> lowpass -> delay
baixo = | dó2 _ dó2 mib2 | fá2 _ sol2 sib1 |
baixo.lowpass.cutoff = 800 Hz
baixo.lowpass.resonance = 0.3
baixo.delay.mix = 0.25
```

Os números de uma parte também podem ir num bloco com o nome dela, sem
repetir o nome:

```aratu
tempo: 100
baixo: synth.saw -> lowpass -> delay
baixo = | dó2 _ dó2 mib2 | fá2 _ sol2 sib1 |
baixo {
  lowpass.cutoff = 800 Hz
  delay.time = 375 ms
  delay.feedback = 0.4
}
```

Os números mais usados:

| Número | O que é |
|---|---|
| `lowpass.cutoff`, `highpass.cutoff` | onde o filtro corta, de 20 a 20000 Hz |
| `lowpass.resonance` | um pico no corte, de 0 a 1 |
| `drive.amount` | quanto distorce, de 0 a 36 dB |
| `chorus.mix`, `delay.mix` | quanto do efeito se ouve, de 0 a 1 |
| `delay.time` | o tempo do eco, em s ou ms |
| `delay.feedback` | quantas vezes o eco volta, de 0 a quase 1 |
| `tremolo.rate` | quantas vezes por segundo o volume pulsa |

O manual tem a lista completa. O knob do cutoff anda numa escala de
frequência como o ouvido percebe; segure Shift para passos mais finos.

## Equalizador e válvula

Ao lado de `eq`, `valvula`, `drive`, `lowpass` e `highpass` numa cadeia, o
editor desenha uma curva pequena que mostra o que o efeito faz, e ela
muda junto quando você gira os knobs. A do `eq` é a resposta do
equalizador: os graves à esquerda, os agudos à direita, para cima o que
aumenta. Clique nela e o equalizador abre grande: arraste os pontos dos
graves, dos médios e dos agudos, e os números são escritos no código. A
roda do mouse no ponto dos médios estreita ou alarga a faixa.

```aratu
tempo: 96
guitarra: guitar.steel -> eq -> valvula -> reverb
guitarra = | mi3 sol3 lá3 sol3 | mi3 _ ré3 _ |
guitarra.eq.low_gain = -6 dB
guitarra.eq.mid_gain = 4 dB
guitarra.eq.mid_freq = 900 Hz
guitarra.valvula.drive = 12 dB
```

A válvula é o drive do Manguebyte ajustado como uma válvula: a curva
suave que arredonda os picos, um pouco de ganho e o volume trazido de
volta. Os números dela são `valvula.drive` (de 0 a 24 dB), `valvula.output`
e `valvula.mix`. A curva ao lado dela mostra o quanto o som é achatado, e
brilha mais quanto mais forte ela é tocada. Como a válvula e o drive são o
mesmo estágio, uma cadeia usa um ou o outro.

## Um número que muda num compasso

Como qualquer mudança, um número pode mudar num compasso:

```aratu
tempo: 100
baixo: synth.saw -> lowpass
baixo = | dó2 dó3 dó2 dó3 |
baixo.lowpass.cutoff = 400 Hz
at bar 3: baixo.lowpass.cutoff = 3000 Hz
```

## Exercícios

Estes exercícios conferem também o **som**: o instrumento, os efeitos e os
números.

Faça o baixo tocar com o baixo slap, `bass.slap`.

```aratu exercise sound=1
baixo: synth.saw
baixo = | mi2 Mi3 mi2 Mi3 |
```

```aratu answer
baixo: bass.slap
baixo = | mi2 Mi3 mi2 Mi3 |
```

Ponha um delay na cadeia do teclado, depois do chorus.

```aratu exercise sound=1
teclado: keys.epiano -> chorus
teclado = | dó4+mi4+sol4 _ _ _ |
```

```aratu answer
teclado: keys.epiano -> chorus -> delay
teclado = | dó4+mi4+sol4 _ _ _ |
```

Abafe o sintetizador: o cutoff do lowpass em 600 Hz.

```aratu exercise sound=1
baixo: synth.saw -> lowpass
baixo = | dó2 _ dó3 _ |
baixo.lowpass.cutoff = 5000 Hz
```

```aratu answer
baixo: synth.saw -> lowpass
baixo = | dó2 _ dó3 _ |
baixo.lowpass.cutoff = 600 Hz
```

Passe a guitarra por uma válvula, com 12 dB de drive.

```aratu exercise sound=1
guitarra: guitar.steel -> reverb
guitarra = | mi3 sol3 lá3 sol3 |
```

```aratu answer
guitarra: guitar.steel -> valvula -> reverb
guitarra = | mi3 sol3 lá3 sol3 |
guitarra.valvula.drive = 12 dB
```
