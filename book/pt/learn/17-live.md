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

## No palco

O botão **Palco**, em Tocar, deixa só o código na tela, em tela cheia e com
letra grande, para tocar diante de uma plateia. O botão **2 colunas** divide
o código em duas colunas, como num jornal: a segunda continua de onde a
primeira termina, então cabe o dobro sem rolar a tela, e as duas rolam
juntas. Dá para editar em qualquer uma. As listas de melodias ganham botões
no canto, e Esc sai do palco.

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
