# Dinâmica e articulação

As marcas de dinâmica são as da partitura: `ppp pp p mp mf f ff fff`, e
também `sfz` e `fp`. Dentro de um compasso, a marca vai colada na nota
com dois-pontos, e vale dali em diante:

```aratu
tempo: 90
melodia = | p:dó4 mi sol dó5 | f:sol4 _ _ _ |
```

O primeiro compasso é piano; o segundo, forte. A marca não conta como
tempo, e pode ter um espaço depois dos dois-pontos: `p: dó4` também vale.
Em `f: fá4`, o primeiro `f` é forte e o segundo é a nota fá.

## Crescendo e diminuendo

`<` depois de uma nota faz crescendo e `>` faz diminuendo, do tempo
seguinte até a próxima marca:

```aratu
tempo: 90
melodia = | p:dó4< ré mi fá | f:sol4> fá mi ré | p:dó4 _ _ _ |
```

Sem uma marca depois, o crescendo sobe um degrau (de mf para f, por
exemplo) até o fim da linha.

## Articulação: acento, marcato, staccato e nota fantasma

A articulação diz **como** cada nota é atacada. São quatro sinais, do mais
forte ao mais fraco:

| Sinal | Escrito | Como soa |
|---|---|---|
| marcato | `S!`, `dó4!` | o mais forte, bem marcado |
| acento | `S`, `Dó4` | mais forte que as vizinhas |
| staccato | `s'`, `dó4'` | curto, destacado |
| nota fantasma | `(s)`, `(dó4)` | quase sem som, só sentida |

O acento soma intensidade à marca em que a música está, e o marcato soma
ainda mais. As notas fantasma são o tempero do funk e do samba na caixa:
toques bem leves entre as batidas fortes.

```aratu
tempo: 96
drums {
  hh = cc cc cc cc
  bd = b - b b
  sd = (s)(s) S! (s)(s) S!
}
```

> Tire os parênteses das notas fantasma e ouça como a caixa fica
> pesada. Os parênteses fazem os toques sumirem para o fundo.

## Ornamentos: trinado, mordente e nota de graça

Os ornamentos enfeitam uma nota com a vizinha dela na tonalidade. Vão
depois da nota, com dois-pontos, como o `:arp` dos acordes:

| Ornamento | Escrito | Como soa |
|---|---|---|
| trinado | `dó5:tr` | a nota e a de cima, alternando rápido, durante toda a duração |
| mordente | `dó5:mord` | a nota, a de cima e a nota de novo, bem rápido, no começo |
| mordente inferior | `dó5:mordi` | o mesmo, com a de baixo |
| nota de graça | `` `ré5 dó5 `` | uma nota pequena, colada antes da principal |

A vizinha é a da tonalidade: em sol maior, o trinado de `lá4` vai a `si4`,
e o de `mi5` a `fá#5`. Para mudar, ponha `#` ou `b` depois do `tr`, como o
acidente em cima do *tr* na partitura: `mi5:trb` trina com fá natural.

```aratu
tempo: 80
key: G major
melodia = | sol4 si4 `dó5 ré5 _ | ré5:mord dó5 si4 lá4 | lá4:tr _ _ _ | sol4 _ _ _ |
```

A nota de graça não conta como tempo: ela soa um instante antes da
principal. Num tambor, a crase faz o mesmo com o próprio tambor: `` `s `` é
um **flam**, um toque leve logo antes, e ``` ``s ``` é um **drag**, dois:

```aratu
tempo: 90
drums {
  hh = cc cc cc cc
  bd = b - b -
  sd = - `S - ``S
}
```

> Tire as crases e ouça a caixa seca; com elas, cada batida ganha um
> arrasto antes.

## Mudar a parte inteira

Uma linha só leva uma parte inteira a uma dinâmica, seja qual for a que
está escrita nela:

```aratu
melodia = | dó4 mi sol mi |
melodia: pp
```

Experimente `melodia: ff`, ou `melodia: cresc to ff over 4 bars`: um
crescendo de quatro compassos, que parte do que está escrito e chega a ff.

## Exercícios

O primeiro compasso **piano**, o segundo **forte**.

```aratu exercise dynamics=1
melodia = | dó4 ré mi fá | sol fá mi ré |
```

```aratu answer
melodia = | p:dó4 ré mi fá | f:sol4 fá mi ré |
```

Comece em `p` e faça um crescendo até `f` no compasso seguinte.

```aratu exercise dynamics=1
melodia = | p:dó4 ré mi fá | f:sol4 _ _ _ |
```

```aratu answer
melodia = | p:dó4< ré mi fá | f:sol4 _ _ _ |
```

Na caixa, os tempos 2 e 4 em marcato e notas fantasma nas colcheias
entre eles: `(s)` na segunda metade dos tempos 1 e 3.

```aratu exercise dynamics=1
sd = s- s s- s
```

```aratu answer
sd = -(s) s! -(s) s!
```

Enfeite a melodia: um **mordente** no primeiro mi5 e um **trinado** no
último.

```aratu exercise
tempo: 80
melodia = | mi5 ré5 dó5 ré5 | mi5 _ _ _ |
```

```aratu answer
tempo: 80
melodia = | mi5:mord ré5 dó5 ré5 | mi5:tr _ _ _ |
```

Uma nota de graça `ré5` antes de cada dó5.

```aratu exercise
melodia = | dó5 sol4 dó5 sol4 |
```

```aratu answer
melodia = | `ré5 dó5 sol4 `ré5 dó5 sol4 |
```

Na caixa, um **flam** no tempo 2 e um **drag** no tempo 4.

```aratu exercise
sd = - s - s
```

```aratu answer
sd = - `s - ``s
```
