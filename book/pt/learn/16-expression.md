# Expressão: swing, rubato e humanize

Um computador toca cada nota no lugar exato. Um músico, não: ele balança as
colcheias, respira numa frase, nunca bate duas vezes igual. Três palavras
dão essa vida ao que você escreve, sem mudar nenhuma nota.

## Swing

No jazz, no samba-jazz e no blues, as colcheias não são iguais: a primeira
é mais longa e a segunda chega um pouco atrasada. É o **swing**. Escreva
quanto:

```aratu
swing 62%
tempo: 110
hh = cc cc cc cc
bd = b - b -
sd = - s - s
```

A porcentagem diz onde cai a segunda colcheia dentro do tempo. 50% é
reto, as duas iguais. 66% é a sensação de tercina, `c_c`, o swing mais
pesado. Entre um e outro fica o balanço que você quiser.

> Arraste o knob do `62%` com a música tocando, de 50 a 70, e ouça o
> chimbal começar reto e ir balançando.

O swing também vale para uma parte só: `baixo: swing 60%`.

## Rubato

**Rubato** quer dizer tempo roubado. O músico estica um tempo e encolhe
outro, mas chega na barra de compasso no lugar certo: o tempo roubado é
devolvido. É o jeito de um pianista tocar uma melodia lenta.

```aratu
tempo: 70
melodia = | mi4 ré dó ré | mi mi mi _ |
bd = b - - -
melodia: rubato 20%
```

Ouça: a melodia respira, mas o bumbo continua batendo no começo de cada
compasso, junto com ela. `all: rubato 10%` curva a peça inteira junto,
como um maestro.

## Humanize

`humanize` acrescenta pequenos desvios, de tempo e de força, como os de um
músico de verdade. Cada batida sai um pouco diferente:

```aratu
humanize
tempo: 100
hh = cc cc cc cc
bd = b - b -
```

`humanize 5 ms` diz o tamanho do desvio. Os desvios são sorteados como no
acaso: com a mesma semente, a mesma peça toca sempre do mesmo jeito.

## Exercícios

Estes exercícios conferem também **quando** cada nota chega, não só qual
nota é.

Ponha swing de 66% nesta levada.

```aratu exercise timing=1
hh = cc cc cc cc
bd = b - b -
```

```aratu answer
swing 66%
hh = cc cc cc cc
bd = b - b -
```

O swing está pesado demais: deixe em 58%.

```aratu exercise timing=1
swing 66%
hh = cc cc cc cc
```

```aratu answer
swing 58%
hh = cc cc cc cc
```

Faça a melodia respirar com 20% de rubato, enquanto o bumbo segue o pulso.

```aratu exercise timing=1
melodia = | mi4 ré dó ré |
bd = b - - -
```

```aratu answer
melodia = | mi4 ré dó ré |
bd = b - - -
melodia: rubato 20%
```
