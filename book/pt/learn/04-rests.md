# Silêncios e pausas longas

O silêncio também é música. No Aratu toda pausa se escreve com o mesmo
sinal, `-`, e a duração vem das mesmas regras dos sons: um `-` ocupa uma
parte do tempo, como uma nota ocuparia.

| Pausa | Escrita em 4/4 |
|---|---|
| semínima | `-` sozinho num tempo |
| colcheia | `-c` (pausa, depois colcheia) ou `c-` |
| semicolcheia | `-ccc`, `c-cc` |
| colcheia pontuada | `-__c` ou `-.c` |
| mínima | `- -` |

Ouça a pausa de colcheia no começo de cada tempo, o "contratempo":

```aratu
tempo: 100
hh = -c -c -c -c
bd = b b b b
```

Um `_` depois de uma pausa prolonga o silêncio: `- _` também é uma pausa de
mínima.

## O compasso inteiro de pausa

Um `-` sozinho entre as barras vale o compasso inteiro, em qualquer
fórmula, como a pausa de semibreve da partitura. E `- x4` são quatro
compassos de pausa: o `x` quer dizer "vezes".

```aratu
hh = | c c c c |
bd = | - x2 | b - b - |
```

O chimbal toca desde o começo; o bumbo espera dois compassos e entra no
terceiro. Como a linha do bumbo tem três compassos, ela se repete a cada
três.

> Pausas longas servem para fazer um instrumento entrar depois, como numa
> banda em que a bateria começa sozinha.

## Exercícios

Chimbal só nos contratempos: pausa de colcheia e colcheia em cada tempo.

```aratu exercise
hh = cc cc cc cc
```

```aratu answer
hh = -c -c -c -c
```

O bumbo espera **dois compassos** de silêncio e então toca nos tempos 1 e
3.

```aratu exercise
bd = | b - b - |
```

```aratu answer
bd = | - x2 | b - b - |
```
