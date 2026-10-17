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
