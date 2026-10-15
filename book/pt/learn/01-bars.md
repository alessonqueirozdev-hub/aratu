# O compasso e o andamento

Toda música do Aratu começa dizendo duas coisas: em que **compasso** ela
está e em que **andamento** ela anda.

```aratu
meter: 4/4
tempo: 90

hh = c c c c
bd = b - b -
```

`meter: 4/4` diz a fórmula de compasso: quatro tempos por compasso.
`tempo: 90` diz o andamento: noventa tempos por minuto, como num
metrônomo. Repare no botão redondo ao lado do `90`: arraste-o para cima ou
para baixo e o número muda. A música acelera ou desacelera no próximo
compasso.

## A primeira regra: um espaço separa os tempos

Leia a linha do chimbal contando em voz alta: "um, dois, três, quatro".
Cada grupo de símbolos separado por espaço é **um tempo**. Num compasso
4/4 há quatro grupos.

Se um compasso tiver tempos demais ou de menos, o Aratu avisa. Este
exemplo tem um tempo a mais. Leia a mensagem embaixo dele e corrija:

```aratu broken
meter: 4/4
bd = b - b - b
```

## Vários compassos numa linha

A barra `|` separa os compassos, como na partitura. Esta linha de bumbo
tem dois compassos diferentes, e o par se repete:

```aratu
bd = | b - b - | b b b - |
sd = | - s - s | - s - s |
```

Na grade, cada compasso aparece numa caixa, com os tempos numerados em
cima.

## Outras fórmulas de compasso

Numa valsa há três tempos por compasso. Conte "um, dois, três" enquanto
ouve:

```aratu
meter: 3/4
tempo: 150

bd = b - -
hh = - c c
```

Em 6/8 o tempo é uma semínima pontuada e se conta em dois, como um músico
conta. Você vai ver isso na próxima lição.

## Exercícios

Escreva dois compassos de bumbo: no primeiro, o bumbo toca nos tempos 1 e
3; no segundo, nos quatro tempos.

```aratu exercise
bd = | - - - - | - - - - |
```

```aratu answer
bd = | b - b - | b b b b |
```

Agora uma valsa em 3/4: bumbo no tempo 1, caixa nos tempos 2 e 3.

```aratu exercise
meter: 3/4
bd = - - -
sd = - - -
```

```aratu answer
meter: 3/4
bd = b - -
sd = - s s
```
