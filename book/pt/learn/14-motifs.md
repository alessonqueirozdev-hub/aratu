# Motivos, sequências e séries

Muita música é feita de poucas ideias usadas muitas vezes: um motivo que
volta, que sobe ou desce, que vira de cabeça para baixo. O Aratu deixa você
escrever a ideia uma vez e usá-la quantas vezes quiser.

## Repetir um tempo

Um `x` colado no colchete repete aquele tempo. Cada repetição ocupa o seu
próprio tempo:

```aratu
tempo: 100
melodia = | [mi4 ré dó ré]x4 |
```

É o mesmo que escrever `[mi4 ré dó ré]` quatro vezes. O `x` vai sempre
colado ao `]`; separado por espaço, `| - x8 |` continua querendo dizer oito
compassos de pausa. O número também pode vir antes: `[mi4 ré dó ré]4x`.

> Chaves `{ }` são para o acaso: `{mi4|sol4}` sorteia uma das notas a cada
> volta. Para agrupar, use colchetes. Se escrever `{lá2 mi3 lá mi}x4`, sem
> `|` dentro, o Aratu toca como um grupo e avisa para trocar por colchetes.

## Motivos

Um motivo é um trecho com nome. Defina com `motif` e use o nome no lugar
dos tempos:

```aratu
tempo: 100
motif riff = [mi4 ré dó ré]
melodia = | riff riff riff riff |
```

> O motivo pode estar escrito antes ou depois da linha que o usa. Enquanto
> toca, o editor acende as notas na definição do motivo.

## Sequências

Na teoria musical, uma **sequência** é um motivo repetido em outra altura.
`riff+1` é o motivo um grau acima, dentro da tonalidade (`key:`), e
`riff-1` um grau abaixo:

```aratu
tempo: 100
key: C major
motif riff = [mi4 ré dó ré]
melodia = | riff riff+1 riff+2 riff+3 |
```

Por graus, o motivo continua dentro da escala: de mi para fá é um grau,
como de dó para ré. Para contar em semitons, use `st`: `riff+3st` sobe três
semitons.

## Inversão e retrógrado

| Escreva | O motivo |
|---|---|
| `riff.inv` | invertido: cada intervalo de cabeça para baixo, a partir da primeira nota |
| `riff.retro` | retrógrado: as notas de trás para frente, no mesmo ritmo |
| `riff.ri` | retrógrado da inversão |

```aratu
tempo: 100
motif riff = [mi4 ré dó ré]
melodia = | riff riff.inv riff.retro riff.ri |
```

Também se combinam com a transposição: `riff.ri+2` é o retrógrado da
inversão dois graus acima.

## Ostinato

Um ostinato é um motivo que se repete por baixo enquanto o resto muda. No
Aratu toda linha já se repete sozinha, então basta um compasso:

```aratu
tempo: 100
motif ost = [lá2 mi3 lá3 mi3]
baixo = | ost ost ost ost |
piano {
  chords = | Am | F | C | G |
}
```

## Transformar uma linha inteira

As mesmas transformações valem para uma linha inteira, sem reescrevê-la.
Escreva o nome da linha, um ponto e o verbo:

```aratu
tempo: 100
melodia = | dó4 ré mi sol | lá sol mi ré |
melodia.transpose +7
```

| Escreva | O que faz |
|---|---|
| `melodia.transpose +7` | sobe sete semitons (`-12` desce uma oitava) |
| `melodia.inv`, `.retro`, `.ri` | inverte, de trás para frente, os dois |
| `melodia.augment` | dobra as figuras: tudo fica duas vezes mais lento |
| `melodia.diminish` | divide as figuras pela metade |
| `hh.rotate 1` | desloca o padrão em uma parte |
| `hh.density 50%` | toca só cerca de metade das batidas |
| `drums.velocity 0.8` | toca mais suave |

Em português: `transpor`, `aumentar`, `diminuir`, `girar`, `densidade`,
`intensidade`. Apague a linha da transformação e a melodia volta a ser
como foi escrita.

## Séries dodecafônicas

Na música dodecafônica, uma **série** ordena as doze notas da escala
cromática, cada uma uma vez. Escreva a série como um vetor de números de 0
a 11 (0 é dó, 1 é dó#, …) ou com nomes de nota:

```aratu
tempo: 80
row serie = 0 11 3 4 8 7 9 5 6 1 2 10
melodia = | o o o o | o o o o | o o o o | from serie.P0
```

A série dá as alturas e o ritmo dá os lugares: cada `o` recebe a próxima
nota da série. Se o ritmo tiver mais `o` do que a série tem notas, ela
recomeça. A série tem quatro formas, cada uma em doze transposições:

| Forma | O que é |
|---|---|
| `serie.P0` | a série original (P de *prime*) |
| `serie.I0` | a inversão, a partir da mesma primeira nota |
| `serie.R0` | o retrógrado, de trás para frente |
| `serie.RI0` | o retrógrado da inversão |
| `serie.P5`, `serie.RI3` | a forma transposta 5 ou 3 semitons acima |

`octave 3` no fim põe todas as notas na oitava 3, e `from [0 4 7]` usa um
vetor escrito ali mesmo. Uma série de doze notas que repete alguma recebe um
aviso. A aba **Séries**, ao lado das mensagens em Tocar, mostra a matriz
12×12 de cada série: clique no nome de uma forma para escrevê-lo no código.

## Listas de melodias

Para preparar uma apresentação, guarde as melodias numa lista, uma por
linha, e escolha pelo número:

```aratu
tempo: 100
list temas {
  | mi4 ré dó ré | mi mi mi _ |
  | ré4 ré ré _ | mi sol sol _ |
  | mi4 ré dó ré | mi mi mi mi |
}
melodia = temas.1
```

> Troque `temas.1` por `temas.2` com a música tocando: a troca entra no
> próximo compasso. `temas.all` toca todas, em ordem, e `temas.random`
> sorteia uma inteira a cada volta.

No modo palco aparecem botões para escolher a melodia de cada lista, sem
digitar nada diante do público, e Alt+1 a Alt+9 fazem o mesmo pelo teclado.

### Uma melodia por compasso

Cada compasso de uma linha também pode ser uma melodia da lista: escreva o
nome no lugar das notas daquele compasso.

```aratu
tempo: 100
list temas {
  | lá2 mi3 lá mi |
  | lá2 fá3 lá fá |
  | sol2 dó3 sol dó |
}
baixo = | temas.1 | temas.1 | temas.2 | temas.3 |
```

Dá para misturar com compassos escritos, `| dó4 _ _ _ | temas.2 |`, e
`temas.all` no lugar de um compasso põe todas, uma depois da outra.

## Exercícios

Repita o primeiro tempo nos quatro tempos do compasso, com `x`.

```aratu exercise
melodia = | [mi4 ré dó ré] - - - |
```

```aratu answer
melodia = | [mi4 ré dó ré]x4 |
```

Faça o motivo subir um grau a cada tempo, como uma sequência.

```aratu exercise
motif riff = [dó4 ré mi]
melodia = | riff riff riff riff |
```

```aratu answer
motif riff = [dó4 ré mi]
melodia = | riff riff+1 riff+2 riff+3 |
```

Transponha a melodia inteira uma quinta acima (sete semitons), com uma
linha só.

```aratu exercise
melodia = | dó4 ré mi sol |
```

```aratu answer
melodia = | dó4 ré mi sol |
melodia.transpose +7
```

Toque a série de trás para frente: a forma retrógrada.

```aratu exercise
row serie = 0 11 3 4 8 7 9 5 6 1 2 10
melodia = | o o o o | o o o o | o o o o | from serie.P0
```

```aratu answer
row serie = 0 11 3 4 8 7 9 5 6 1 2 10
melodia = | o o o o | o o o o | o o o o | from serie.R0
```

Monte o baixo com as melodias 1, 3 e 2 da lista, uma em cada compasso.

```aratu exercise bars=3
list temas {
  | lá2 mi3 lá mi |
  | lá2 fá3 lá fá |
  | sol2 dó3 sol dó |
}
baixo = | temas.1 | - | - |
```

```aratu answer
list temas {
  | lá2 mi3 lá mi |
  | lá2 fá3 lá fá |
  | sol2 dó3 sol dó |
}
baixo = | temas.1 | temas.3 | temas.2 |
```
