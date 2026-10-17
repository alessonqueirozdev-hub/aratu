# Compassos compostos e irregulares

Até aqui o tempo era sempre uma semínima, e um compasso tinha dois, três ou
quatro deles. Nesta lição o tempo muda de tamanho: em 6/8 ele se divide em
três, em 7/8 os tempos têm tamanhos diferentes, e uma tercina pode
atravessar dois tempos.

## 6/8: dois tempos, cada um dividido em três

Num compasso 6/8 há seis colcheias, mas o músico não conta até seis: conta
**dois** tempos, cada um com três colcheias, "UM-dois-três, DOIS-dois-três".
O Aratu conta do mesmo jeito. Cada palavra é um tempo, uma semínima
pontuada, e `ccc` são as suas três colcheias:

```aratu
meter: 6/8
tempo: 66
hh = ccc ccc
bd = b__ b__
sd = - s
```

O bumbo toca no começo de cada tempo e segura as outras duas colcheias
(`b__`); a caixa marca o segundo tempo. Repare que o andamento conta tempos,
não colcheias: `tempo: 66` são 66 semínimas pontuadas por minuto.

> Em português e em espanhol a fórmula também se escreve com
> `compasso:`: `compasso: 6/8`.

O 12/8 é o mesmo, com quatro tempos: é o compasso do blues lento e de
muita balada.

```aratu
meter: 12/8
tempo: 60
hh = ccc ccc ccc ccc
bd = b__ -   b_b -
sd = -   s   -   s
```

## 5/4: cinco tempos

Um compasso pode ter cinco tempos, como em "Take Five", o tema de jazz
mais famoso nessa fórmula. A regra não muda: cinco palavras.

```aratu
meter: 5/4
tempo: 160
hh = cc cc cc cc cc
bd = b  -  -  b  -
sd = -  -  s  -  s
```

## 7/8: tempos de tamanhos diferentes

Nos compassos irregulares, comuns na música dos Bálcãs e no rock
progressivo, as colcheias se agrupam em tempos desiguais. Diga como, depois de
um `=`: `meter: 7/8 = 2+2+3` são dois tempos de duas colcheias e um de três.
Conte "UM-dois, UM-dois, UM-dois-três".

Cada palavra é um grupo. Um grupo de duas colcheias se escreve `cc`, um de
três, `ccc`:

```aratu
meter: 7/8 = 2+2+3
tempo: 240
hh = cc cc ccc
bd = b_ b_ b__
sd = -  -  -s-
```

Aqui o andamento conta colcheias: `tempo: 240` são 240 colcheias por minuto.
Também dá para escrever uma palavra por colcheia, sete palavras, se for
mais fácil de ler: `hh = c c c c c c c`.

> Troque para `meter: 7/8 = 3+2+2` e ajuste as linhas: o mesmo compasso,
> com outro balanço.

## Tercinas que atravessam tempos

Uma tercina de colcheias cabe num tempo (`ccc`). Mas existe a tercina de
**semínimas**: três notas iguais no espaço de dois tempos. Um número antes
dos colchetes diz quantos tempos o grupo ocupa:

```aratu
tempo: 80
melodia = | 2[dó4 ré mi] fá _ | sol _ _ _ |
hh = c c c c
```

Ouça o chimbal: as três notas da melodia se espalham por cima dos dois
primeiros tempos, sem coincidir com o segundo. `3[c c c c]` são quatro notas
em três tempos.

## Exercícios

Um compasso 6/8 com o chimbal nas seis colcheias e o bumbo no começo de
cada tempo.

```aratu exercise
meter: 6/8
hh = - -
bd = - -
```

```aratu answer
meter: 6/8
hh = ccc ccc
bd = b__ b__
```

Escreva um compasso 7/8 agrupado em 2+2+3, com o chimbal em todas as
colcheias e o bumbo no começo de cada grupo.

```aratu exercise
meter: 7/8 = 2+2+3
hh = - - -
bd = - - -
```

```aratu answer
meter: 7/8 = 2+2+3
hh = cc cc ccc
bd = b_ b_ b__
```

Agora 7/8 em 3+2+2: o grupo de três vem primeiro.

```aratu exercise
meter: 7/8 = 3+2+2
hh = - - -
bd = - - -
```

```aratu answer
meter: 7/8 = 3+2+2
hh = ccc cc cc
bd = b__ b_ b_
```

Uma tercina de semínimas nos dois primeiros tempos (dó4, ré4, mi4) e uma
mínima de fá4.

```aratu exercise
melodia = | dó4 ré mi fá |
```

```aratu answer
melodia = | 2[dó4 ré mi] fá _ |
```
