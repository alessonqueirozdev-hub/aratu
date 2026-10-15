# Dinâmica: forte e piano

As marcas de dinâmica são as da partitura: `ppp pp p mp mf f ff fff`, e
também `sfz` e `fp`. Dentro de um compasso, a marca vai colada na nota
com dois-pontos, e vale dali em diante:

```aratu
tempo: 90
melodia = | p:dó4 mi sol dó5 | f:sol4 _ _ _ |
```

O primeiro compasso é piano; o segundo, forte. A marca não conta como
tempo.

## Crescendo e diminuendo

`<` depois de uma nota faz crescendo e `>` faz diminuendo, do tempo
seguinte até a próxima marca:

```aratu
tempo: 90
melodia = | p:dó4< ré mi fá | f:sol4> fá mi ré | p:dó4 _ _ _ |
```

Sem uma marca depois, o crescendo sobe um degrau (de mf para f, por
exemplo) até o fim da linha.

## Acentos

Uma letra maiúscula acentua: `Dó4`, ou `S` na caixa. O acento soma
intensidade à marca em que a música está.

## Mudar a parte inteira

Uma linha só muda o volume de uma parte inteira, em relação ao que está
escrito nela:

```aratu
melodia = | dó4 mi sol mi |
melodia: pp
```

Experimente `melodia: ff`, ou `melodia: cresc to ff over 4 bars` (um
crescendo de quatro compassos).

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
