# A mixagem da obra inteira

Com várias partes tocando juntas, falta o trabalho de quem senta na mesa de
som: deixar cada instrumento no volume certo, cada um no seu lugar entre as
caixas, todos na mesma sala. Isso é a **mixagem**.

## Volume: quem aparece mais

Cada parte tem um `volume`, de 0 (mudo) a 1. Uma bateria alta demais esconde
a melodia; abaixe-a:

```aratu
tempo: 100
drums { hh = cc cc cc cc; bd = b - b -; sd = - s - s }
melodia: keys.epiano
melodia = | mi4 sol lá sol | mi ré dó _ |
drums.volume = 0.5
melodia.volume = 0.9
```

> Arraste os dois knobs com a música tocando, como se fossem os faders de
> uma mesa de som.

## Pan: onde cada um fica

O `pan` põe a parte entre as caixas de som: -1 é toda à esquerda, 1 toda à
direita, 0 no meio. Abrir os instrumentos para os lados deixa cada um mais
claro:

```aratu
tempo: 100
violão: guitar.nylon
violão { chords = | Am | F | C | G |; strum = D du -u du }
teclado: keys.epiano
teclado = | lá4 _ dó5 _ | lá4 _ fá4 _ | sol4 _ mi4 _ | ré4 _ sol4 _ |
violão.pan = -0.6
teclado.pan = 0.6
```

Ouça com fones de ouvido: o violão à esquerda, o teclado à direita. O baixo
e o bumbo costumam ficar no meio.

## A mesma sala para todos

Na mixagem do Aratu há um reverb compartilhado, como a sala onde a banda
toca. O `reverb.mix` de cada parte diz quanto dela vai para essa sala.

Um número escrito **sem parte** vale para todas as partes. Para pôr a obra
inteira na sala, uma linha basta:

```aratu
tempo: 90
melodia: piano.grand
melodia = | mi4 sol lá sol | mi ré dó _ |
baixo: bass.finger
baixo = | dó2 _ sol1 _ | lá1 _ fá1 _ |
reverb.mix = 0.5
```

Uma parte pode ter o seu próprio valor, que vale no lugar do geral:
`baixo.reverb.mix = 0.1` deixa o baixo mais seco, perto de quem ouve.

## Efeitos para todos: `todos:`

Uma cadeia escrita para `todos` (ou `all`) põe os seus efeitos em todas as
partes, depois dos efeitos de cada uma:

```aratu
tempo: 90
melodia: keys.epiano -> tremolo
melodia = | mi4 sol lá sol | mi ré dó _ |
baixo: bass.finger
baixo = | dó2 _ sol1 _ | lá1 _ fá1 _ |
todos: chorus -> delay
delay.mix = 0.2
```

A melodia passa por tremolo, chorus e delay; o baixo, por chorus e delay. E
`delay.mix = 0.2`, sem parte, regula o delay de todas.

## O master

Tudo termina num último canal, o **master**: a sala compartilhada, um
compressor que "cola" as partes e um limitador que impede o som de
estourar. Os números dele começam com `master.`:

```aratu
tempo: 90
melodia: piano.grand
melodia = | mi4 sol lá sol | mi ré dó _ |
reverb.mix = 0.5
master.reverb.decay = 4 s
master.glue.on = 1
master.limiter.ceiling = -1
```

| Número | O que é |
|---|---|
| `master.reverb.decay` | quanto tempo a sala soa depois da nota, em segundos |
| `master.reverb.size` | o tamanho da sala, de 0 a 1 |
| `master.glue.on` | 1 liga o compressor de cola |
| `master.gain` | o volume de tudo, em dB |
| `master.limiter.ceiling` | o volume máximo, de -12 a -0,3 dB |

## Abrir espaço para o bumbo

Na música eletrônica quem manda é o bumbo, e as outras partes abrem espaço
para ele: a cada batida do bumbo o volume do pad cai na hora e volta antes
da próxima, e a música inteira parece respirar. No estúdio isso se chama
**ducking**, ou *sidechain*. No Aratu é uma linha:

```aratu
tempo: 124
drums { bd = b b b b; hh = -c -c -c -c }
pad: synth.pad
pad = | lá3+dó4+mi4 _ _ _ | fá3+lá3+dó4 _ _ _ |
pad: duck by kick
```

`pad: duck by kick` abaixa o pad 70% a cada bumbo e traz de volta em um
tempo. Diga quanto e em quanto tempo: `pad: duck by kick 50% over 1
eighth`. Depois de `by` vem a parte ou a linha para a qual abrir espaço
(`drums`, `bd`), ou um tambor pelo nome, `kick`, `snare` ou `clap`, em
qualquer linha que o toque.

> Apague a linha e escute: o pad fica parado, por cima do bumbo. Ponha de
> volta e a música pulsa outra vez.

Em português se escreve `pad: abaixa com o bumbo`, em espanhol
`pad: agacha con el bombo`, e a palavra do estúdio, `sidechain`, vale nas
três línguas.

## Exercícios

Abaixe a bateria para 0.4, para a melodia aparecer.

```aratu exercise sound=1
drums { hh = cc cc cc cc; bd = b - b - }
melodia = | mi4 sol lá sol |
```

```aratu answer
drums { hh = cc cc cc cc; bd = b - b - }
melodia = | mi4 sol lá sol |
drums.volume = 0.4
```

Ponha o violão à esquerda (-0.5) e o teclado à direita (0.5).

```aratu exercise sound=1
violão = | mi3 _ sol3 _ |
teclado = | dó5 _ mi5 _ |
```

```aratu answer
violão = | mi3 _ sol3 _ |
teclado = | dó5 _ mi5 _ |
violão.pan = -0.5
teclado.pan = 0.5
```

Ponha a obra inteira no reverb, com 0.6, numa linha só.

```aratu exercise sound=1
melodia = | mi4 sol lá sol |
baixo = | dó2 _ sol1 _ |
```

```aratu answer
melodia = | mi4 sol lá sol |
baixo = | dó2 _ sol1 _ |
reverb.mix = 0.6
```

Ponha chorus em todas as partes, com uma linha só.

```aratu exercise sound=1
melodia: keys.epiano
melodia = | mi4 sol lá sol |
baixo: bass.finger
baixo = | dó2 _ sol1 _ |
```

```aratu answer
melodia: keys.epiano
melodia = | mi4 sol lá sol |
baixo: bass.finger
baixo = | dó2 _ sol1 _ |
todos: chorus
```

Faça a sala do master soar por 5 segundos.

```aratu exercise sound=1
melodia = | mi4 sol lá sol |
reverb.mix = 0.5
```

```aratu answer
melodia = | mi4 sol lá sol |
reverb.mix = 0.5
master.reverb.decay = 5 s
```

Faça o pad abaixar pela metade (50%) a cada bumbo.

```aratu exercise sound=1
drums { bd = b b b b }
pad = | dó4+mi4+sol4 _ _ _ |
```

```aratu answer
drums { bd = b b b b }
pad = | dó4+mi4+sol4 _ _ _ |
pad: duck by kick 50%
```
