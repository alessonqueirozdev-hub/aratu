# O acaso

Um loop que se repete sempre igual logo soa mecânico. Um baterista nunca
toca dois compassos exatamente iguais: uma nota do chimbal some, outra
aparece. O acaso dá essa vida ao loop sem você mexer: o computador decide,
um pouco ao acaso, o que tocar a cada volta. É o *ad lib* da partitura.

## Às vezes

Um `?` depois de um símbolo faz ele tocar só às vezes, metade das vezes:

```aratu
tempo: 100
hh = cc cc c? cc
bd = b - b -
```

Uma porcentagem diz com que frequência:

```aratu
tempo: 100
hh = cc cc c?30% cc
sd = - s - s?80%
```

## Um ou outro

Chaves escolhem uma entre várias opções, separadas por `|`, a cada volta
do loop:

```aratu
tempo: 100
baixo = | {dó2|sol1} _ dó2 _ |
hh = {cc|co} cc cc cc
```

O baixo começa ora em dó, ora em sol; o chimbal às vezes abre no primeiro
tempo.

## O mesmo acaso de novo

As escolhas são sorteadas de novo a cada volta, mas a partir de uma
semente fixa: a mesma peça improvisa sempre do mesmo jeito quando é tocada
do começo. `seed 7` escolhe outro improviso; `seed random` faz cada
apresentação diferente.

```aratu
seed 7
hh = c? c? c? c?
```

> Mudar uma linha não muda o improviso das outras: cada linha tem o seu
> próprio sorteio.

Esta lição não tem exercício com conferência, porque o acaso não tem
resposta certa. Experimente à vontade nos exemplos acima.
