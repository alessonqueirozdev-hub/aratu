# Dividindo o tempo

Até aqui cada tempo tinha um símbolo só, uma semínima. Mas a música divide
os tempos: duas colcheias, quatro semicolcheias, tercinas. No Aratu isso
cabe numa regra.

## A segunda regra: símbolos juntos dividem o tempo

Símbolos escritos juntos, sem espaço, dividem o tempo em partes iguais.

| Escrito | Num tempo |
|---|---|
| `c` | uma semínima |
| `cc` | duas colcheias |
| `cccc` | quatro semicolcheias |
| `ccc` | uma tercina de colcheias |

Ouça o tempo se dividindo cada vez mais, do 1 ao 4:

```aratu
tempo: 80
hh = c cc ccc cccc
```

Olhe a grade: no tempo 2 há dois símbolos, no 3 há três, no 4 há quatro,
todos com o mesmo espaço entre si.

## A terceira regra: `_` segura, `-` silencia

Dentro de um tempo, `_` faz o som anterior durar mais uma parte, e `-` é
uma parte de silêncio. Com essas duas peças se escrevem os ritmos mais
comuns:

| Ritmo num tempo | Escrito |
|---|---|
| semicolcheia, semicolcheia, colcheia | `ccc_` |
| semicolcheia, colcheia, semicolcheia | `cc_c` |
| colcheia, semicolcheia, semicolcheia | `c_cc` |
| colcheia pontuada, semicolcheia | `c__c` ou `c.c` |
| pausa de colcheia, colcheia | `-c` |

O ponto é um atalho: `c.c` é o mesmo que `c__c`, a colcheia pontuada.

```aratu
tempo: 80
hh = ccc_ cc_c c_cc c.c
```

Um `_` sozinho num tempo segura o tempo inteiro: `c _` é uma mínima e
`c _ _ _` uma semibreve.

## Mais de um tempo

Um número antes dos colchetes estica o grupo por mais tempos:
`2[c c c]` são três batidas iguais em dois tempos, a tercina de semínimas.

```aratu
tempo: 80
hh = 2[c c c] c c
```

## Exercícios

Faça o chimbal tocar colcheias no compasso inteiro.

```aratu exercise
hh = c c c c
```

```aratu answer
hh = cc cc cc cc
```

Semínimas nos tempos 1 e 3, semicolcheias nos tempos 2 e 4.

```aratu exercise
hh = c c c c
```

```aratu answer
hh = c cccc c cccc
```

Colcheia pontuada e semicolcheia em todos os tempos: o ritmo "galopado".

```aratu exercise
hh = cc cc cc cc
```

```aratu answer
hh = c.c c.c c.c c.c
```

> Ouça o objetivo antes de escrever. Tentar repetir o ritmo com a voz
> ("tááá-ta, tááá-ta") ajuda a achar a escrita.
