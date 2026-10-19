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

## Gestos de palco

Na música eletrônica, uma apresentação se move por gestos: a bateria some
(o *breakdown*), a tensão sobe (o *build*) e tudo volta de uma vez (o *drop*).
No Aratu, cada gesto é uma palavra:

| Escreva | O que acontece |
|---|---|
| `breakdown` (ou `quebra`) | a bateria, a percussão e o baixo saem; o resto continua |
| `build over 4 bars` (ou `subida em 4 compassos`) | um rufo de caixa que dobra de velocidade e cresce, enquanto um filtro tira os graves de tudo |
| `drop` (ou `queda`) | tudo volta, com um prato no primeiro tempo |
| `solo baixo` | só o baixo toca; `solo off` traz o resto de volta |

Com `now:` na frente, o gesto entra no próximo tempo; com `at bar`, no
compasso marcado:

```aratu
tempo: 124
drums { bd = b b b b; oh = -o -o -o -o; clap = - p - p }
baixo = | [- dó2]x4 | [- láb1]x4 |
pad = | dó4+mib4+sol4 _ _ _ | dó4+mib4+láb4 _ _ _ |
at bar 5: breakdown
at bar 7: build over 2 bars
at bar 9: drop
```

Ouça: quatro compassos com tudo, o pad sozinho, a subida e a volta com o
prato. Tocando ao vivo, escreva `now: drop` na hora em que a pista pedir.

## Levadas prontas

Para começar rápido, uma palavra escreve uma bateria inteira:
`levada house` (ou `groove house`). As levadas são: house, techno, trap,
dnb, breakbeat, hiphop, funk, rock, reggaeton, funk carioca, bossa, samba,
baião e maracatu.

```aratu
tempo: 124
levada house
baixo = | [- lá1]x4 | [- fá1]x4 |
```

Enquanto toca, a palavra `levada` acende. E qualquer linha que você
escrever com o mesmo nome toma o lugar da linha da levada: `hh = c c c c`
troca só o chimbal, e o resto da levada continua.

## Atalhos para tocar ao vivo

| Atalho | O que faz |
|---|---|
| Ctrl+Enter | toca, ou atualiza o que está tocando |
| Ctrl+. | para |
| Ctrl+/ | comenta as linhas selecionadas: elas param de soar; de novo, voltam |
| Alt+M | silencia a parte da linha onde está o cursor (escreve `parte: out`); de novo, ela volta |
| Alt+S | põe em solo a parte da linha do cursor (escreve `solo parte`); de novo, tira |
| Ctrl+Shift+1 a 8 | traz de volta uma cena guardada |

Os atalhos escrevem no texto o que fazem, então a mudança entra no próximo
compasso, como qualquer outra, e fica guardada se você compartilhar.

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

## Cenas

Montou um momento bom da música? Guarde: em Tocar e no palco há oito
**cenas**, numeradas de 1 a 8. Clique numa cena vazia e a música do editor
fica guardada nela; clique numa cena guardada, ou aperte Ctrl+Shift e o
número, e ela volta para o editor. Com a música tocando, a troca entra no
próximo compasso, como qualquer mudança, e o Ctrl+Z desfaz. Shift+clique
guarda a música atual por cima de uma cena.

Assim você prepara a introdução, o groove, o breakdown e o drop como cenas,
e no palco passa de uma para outra com uma tecla. As cenas ficam guardadas
neste navegador.

## Ver o som: onda e rolo

Embaixo de uma parte, escreva o nome dela, um ponto e **onda**, e o
editor desenha ali a onda do som dela enquanto toca, como um
osciloscópio. Com **rolo** no lugar de onda, aparecem as notas do
compasso passando sob uma linha que anda, uma fileira para cada nota ou
tambor, como o *piano roll* de um estúdio. Vale para uma parte
(`baixo.onda`) ou para uma linha da bateria (`drums.hh.rolo`).

```aratu
tempo: 120
drums { bd = b b b b; hh = cc cc cc cc }
drums.hh.rolo
baixo = | dó2 dó2 mib2 sol2 |
baixo.onda
```

Os desenhos não mudam o som: servem para ver o que cada parte faz. A onda
é o som de verdade da parte, depois dos efeitos, como o Manguebyte o põe
na mixagem: ponha um `drive` ou um `lowpass` na cadeia e veja a onda
mudar. Ela fica parada em umas três voltas da nota mais grave. Uma linha
da bateria (`drums.hh.onda`) é desenhada a partir dos golpes, porque a
bateria soa como uma parte só. A onda da música inteira fica no visor, ao
lado do compasso; um clique nela mostra as frequências, do grave à
esquerda ao agudo à direita, como num equalizador.

## Seguir a música

Com o botão **Seguir**, em Tocar e no palco, a tela vai sozinha até o
trecho que está tocando: quando a música passa para uma seção mais
abaixo, o código dela aparece. A tela espera enquanto você digita, clica
ou rola, e só anda quando o que toca sai de vista.

O **mapa do ritmo**, embaixo do editor, mostra a música no tempo, como o
arranjo de uma DAW: uma coluna para cada compasso, na ordem em que toca,
com o nome da seção em cima, e uma linha para cada parte, só com o que
ela toca em cada compasso. Uma linha laranja corre por ele enquanto a
música toca, e o mapa rola junto.

## No palco

O botão **Palco**, em Tocar, deixa só o código na tela, em tela cheia e com
letra grande, para tocar diante de uma plateia. O botão **2 colunas** divide
o código em duas colunas, como num jornal: a segunda continua de onde a
primeira termina, então cabe o dobro sem rolar a tela, e cada uma rola
sozinha. Dá para editar em qualquer uma. Com **Caber na tela**, a letra
encolhe conforme a música cresce, para a peça inteira caber sem rolar. As
listas de melodias ganham botões no canto, e Esc sai do palco.

## Tocar numa DAW

Uma DAW (REAPER, Ableton Live, FL Studio, Logic) é o programa que grava,
mixa e põe efeitos na música. O Aratu toca dentro dela com o plugin
**Aratu Bridge**, do Manguebyte: ponha o plugin numa trilha de instrumento
da DAW, aperte **DAW** em Tocar e depois **Conectar**. Daí em diante o som
sai da trilha, e não do navegador, com os mesmos instrumentos: dá para
gravar a trilha, mixar com as outras e pôr os efeitos da DAW por cima.

Quem manda no tempo é a DAW. O play dela começa a música no próximo
compasso dela, o stop para, e o andamento dela é escrito na linha `tempo:`
da peça. Com a DAW parada, **Tocar** continua funcionando, e o som sai na
DAW do mesmo jeito.

O plugin escuta na porta 47870; com mais de um plugin na DAW, cada um tem a
sua porta, e ela se escreve no painel. Na primeira vez, o navegador pode
perguntar se a página pode falar com programas deste computador: responda
que sim. Pela DAW não chegam as ondas das partes, então `baixo.onda` volta
a ser desenhada pelas notas.

> Ponha o plugin numa trilha, conecte, e escreva `tempo: 90` na sua peça.
> Dê play na DAW a 120: a linha vira `tempo: 120` e a música entra no
> compasso seguinte da DAW.

## Compartilhar e gravar

Gostou do que fez? **Compartilhar** copia um link com a sua música dentro:
mande para alguém, e quem abrir ouve e pode mexer na mesma peça. **Gravar**
grava o que está tocando, até você apertar de novo; aí o arquivo de áudio é
baixado, para guardar ou mostrar a quem quiser.

## Exercícios

Escreva o baixo tocando dó2 nos quatro tempos e mostre embaixo dele a
onda do baixo.

```aratu exercise
drums { bd = b b b b }
```

```aratu answer
drums { bd = b b b b }
baixo = | dó2 dó2 dó2 dó2 |
baixo.onda
```

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

Faça um breakdown no compasso 3 e o drop no compasso 5.

```aratu exercise bars=6
drums { bd = b b b b }
baixo = | dó2 _ _ _ |
pad = | dó4+mib4+sol4 _ _ _ |
```

```aratu answer
drums { bd = b b b b }
baixo = | dó2 _ _ _ |
pad = | dó4+mib4+sol4 _ _ _ |
at bar 3: breakdown
at bar 5: drop
```

Comece com a levada house e troque o chimbal por colcheias fechadas.

```aratu exercise
drums { bd = b b b b }
```

```aratu answer
levada house
hh = cc cc cc cc
```

Uma subida de dois compassos a partir do compasso 3.

```aratu exercise bars=5
drums { bd = b b b b }
```

```aratu answer
drums { bd = b b b b }
at bar 3: build over 2 bars
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
