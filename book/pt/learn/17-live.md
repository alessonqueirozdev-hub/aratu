# Tocar ao vivo

No Aratu, tocar ao vivo é mudar o texto enquanto a música toca. Algumas
ideias deixam isso fácil.

- **Tudo se repete.** Cada linha toca sem parar, cada uma no seu tamanho:
  um chimbal de um compasso, um baixo de dois e uma sequência de acordes de
  quatro andam juntos como numa banda.
- **A mudança entra no compasso seguinte.** Nada recomeça: as outras partes
  continuam no lugar.
- **Com erro, nada para.** Enquanto o texto estiver com erro, continua
  tocando a última versão certa.

## Padrões

Um padrão é um grupo de linhas com nome, preparado antes. `play` escolhe
qual padrão soa:

```aratu
tempo: 110
pattern groove {
  hh = cc cc cc cc
  bd = b - b -
  sd = - s - s
}
pattern fill {
  sd = ss ss SS SS
  bd = b - - -
}

play groove x3, fill
```

`play groove x3, fill` toca o groove três vezes, depois a virada, e repete
a frase de quatro compassos. Em português: `padrão` no lugar de `pattern`
e `toque` no lugar de `play`.

## Variar de tempos em tempos

```aratu
tempo: 110
pattern groove { hh = cc cc cc cc; bd = b - b -; sd = - s - s }
pattern fill   { sd = ss ss SS SS }
pattern crash  { cr = x - - - }

play groove
every 4 bars: play fill
every 8 bars, first: add crash
```

`every 4 bars: play fill` troca o groove pela virada no último compasso de
cada quatro. `add` toca um padrão **por cima** do que está soando: aqui, um
prato no primeiro tempo de cada oito compassos.

## Já, no próximo tempo

Uma linha escrita com a música tocando entra no próximo compasso. Com
`now:` (ou `agora:`) na frente, ela entra já no próximo tempo:
`now: play fill once` traz a virada na hora, e o groove volta sozinho, no
lugar em que estaria. A lição seguinte mostra como marcar uma mudança num
compasso e num tempo exatos.

## Tirar e trazer uma parte

`bateria: out` silencia uma parte a partir do próximo compasso, e
`bateria: in` a traz de volta, como num mixer. `all: out` silencia tudo.

```aratu
tempo: 110
drums { hh = cc cc cc cc; bd = b - b -; sd = - s - s }
baixo = | dó2 _ sol1 _ | lá1 _ mi1 _ |
at bar 3: drums: out
at bar 5: drums: in
```

## Mais rápido, mais lento, e um eco

Três verbos curtos mudam uma linha enquanto ela toca, sem escrevê-la de
novo. Escreva o nome da linha, um ponto e o verbo:

```aratu
tempo: 120
drums {
  hh = | cc cc cc co | cc cc c- c- |
  bd = b - b -
  sd = - s - s
}
hh.fast 2
```

`hh.fast 2` toca dois compassos do chimbal em cada compasso: duas vezes mais
rápido. `slow 2` estica cada compasso por dois. Os números são inteiros,
para a linha sempre cair na barra de compasso; `fast 2` e `slow 2` juntos se
anulam. Valem para uma linha ou para uma parte inteira (`drums.fast 2`).
Apague a linha e o chimbal volta à velocidade dele.

`echo` toca cada som de novo um pouco depois, e mais baixo, como uma voz que
volta de uma parede:

```aratu
tempo: 100
lead: synth.pluck
lead = | mi5 - sol - | lá - sol - |
lead.echo 3 sixteenths x2
```

`echo 3 sixteenths` repete cada nota três semicolcheias depois, e `x2` faz
dois ecos, cada um mais baixo que o anterior. A duração se escreve como
depois de `over`: `1 beat`, `1 eighth`, `3 sixteenths`. Em português os
verbos são `rapido` (ou `rápido`), `lento` e `eco`, e as durações
`1 tempo`, `1 colcheia`, `3 semicolcheias`.

## No palco

O botão **Palco**, em Tocar, deixa só o código na tela, em tela cheia e com
letra grande, para tocar diante de uma plateia. O botão **2 colunas** divide
o código em duas colunas, como num jornal: a segunda continua de onde a
primeira termina, então cabe o dobro sem rolar a tela, e as duas rolam
juntas. Dá para editar em qualquer uma. Com **Caber na tela**, a letra
encolhe conforme a música cresce, para a peça inteira caber sem rolar. As
listas de melodias ganham botões no canto, e Esc sai do palco.

## Compartilhar e gravar

Gostou do que fez? **Compartilhar** copia um link com a sua música dentro:
mande para alguém, e quem abrir ouve e pode mexer na mesma peça. **Gravar**
grava o que está tocando, até você apertar de novo; aí o arquivo de áudio é
baixado, para guardar ou mostrar a quem quiser.

## Exercícios

Use os padrões prontos para tocar a frase: groove três vezes, depois a
virada. (`x3` também pode ser escrito `3x`.)

```aratu exercise bars=4
pattern groove { hh = cc cc cc cc; bd = b - b -; sd = - s - s }
pattern fill   { sd = ss ss SS SS; bd = b - - - }
play groove
```

```aratu answer
pattern groove { hh = cc cc cc cc; bd = b - b -; sd = - s - s }
pattern fill   { sd = ss ss SS SS; bd = b - - - }
play groove x3, fill
```

Faça a bateria sair no compasso 2 e voltar no compasso 3, enquanto o baixo
continua.

```aratu exercise bars=4
drums { hh = c c c c }
baixo = | dó2 _ _ _ |
```

```aratu answer
drums { hh = c c c c }
baixo = | dó2 _ _ _ |
at bar 2: drums: out
at bar 3: drums: in
```

Com mais uma linha, toque o chimbal duas vezes mais rápido.

```aratu exercise
hh = cc cc cc co
```

```aratu answer
hh = cc cc cc co
hh.fast 2
```

Dê à melodia um eco um tempo depois de cada nota.

```aratu exercise
melodia = | dó5 - - - | sol4 - - - |
```

```aratu answer
melodia = | dó5 - - - | sol4 - - - |
melodia.echo 1 beat
```
