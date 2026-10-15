# O manual do Aratu

O Aratu se escreve do jeito que um músico pensa: em compassos, tempos,
figuras, cifras e palavras da partitura. Quem lê uma cifra e conta um
compasso consegue ler uma peça em Aratu. Este manual descreve a linguagem
inteira; o curso **Aprender** ensina passo a passo. Todo exemplo aqui pode
ser ouvido e modificado.

## 1. Uma primeira peça

```aratu
meter: 4/4
tempo: 96
key: A minor

drums {
  hh = cc cc co cc
  bd = b - b -
  sd = - s - s
}

baixo = | lá2 _ mi2 _ | fá2 _ dó3 _ |

violão {
  chords = | Am | F | C G | G |
  strum  = D du -u du
}
```

As primeiras linhas dão o quadro da peça: fórmula de compasso, andamento e
tom. Depois vem cada parte. Não há `main`, nem chaves para fechar antes de a
música tocar, nem palavras que só existem para o computador. `//` começa um
comentário.

## 2. Ritmo: os tempos e suas partes

Três regras descrevem todo ritmo dentro de um compasso.

**Regra 1. Um espaço separa os tempos.** Uma linha em 4/4 tem quatro grupos.
Se um compasso tiver o número errado de tempos, o Aratu diz isso com essas
palavras: *"o compasso 2 de `baixo` tem 5 tempos, mas a fórmula de compasso
é 4/4"*.

**Regra 2. Símbolos escritos juntos dividem o tempo em partes iguais.**

| Escrito | Num tempo |
|---|---|
| `c` | uma semínima |
| `cc` | duas colcheias |
| `cccc` | quatro semicolcheias |
| `ccc` | uma tercina de colcheias |
| `ccccc` | uma quintina |

**Regra 3. `_` faz o som anterior durar mais uma parte; `-` é uma pausa.**

| Ritmo num tempo | Escrito |
|---|---|
| semicolcheia, semicolcheia, colcheia | `ccc_` |
| semicolcheia, colcheia, semicolcheia | `cc_c` |
| colcheia, semicolcheia, semicolcheia | `c_cc` |
| colcheia pontuada, semicolcheia | `c__c`, ou o atalho `c.c` |
| pausa de colcheia, colcheia | `-c` |

Um `_` sozinho segura um tempo inteiro, então `c _` é uma mínima e
`c _ _ _` uma semibreve. Um `_` no começo de um compasso liga a nota do
compasso anterior.

**As pausas** seguem as mesmas regras: `-` ocupa uma parte, como uma nota
ocuparia.

| Pausa | Escrita em 4/4 |
|---|---|
| um compasso inteiro, em qualquer fórmula | `\| - \|` |
| oito compassos | `\| - x8 \|` |
| mínima | `- -` |
| semínima | `-` como um tempo |
| colcheia | `-c` ou `c-` |
| semicolcheia | `-ccc`, `c-cc` |
| colcheia pontuada, depois semicolcheia | `-__c` ou `-.c` |

**Compassos compostos.** Em 6/8 o tempo é uma semínima pontuada, então
`ccc ccc` é um compasso de colcheias contado em dois, como o músico conta.

```aratu
meter: 6/8
tempo: 70
hh = ccc ccc
bd = b__ b__
```

**Acentos.** Um símbolo maiúsculo é acentuado: `cC cc` acentua o contratempo
do tempo 1.

**Barras.** `|` é a barra de compasso, como na partitura. Uma linha pode ter
vários compassos: `| cc cc | co cc |`. Uma linha sem barras é um compasso.

**Grupos.** Colchetes formam um grupo que ocupa uma parte, e aceitam espaços
dentro: `c[cc]` é uma colcheia e duas semicolcheias. O atalho `.` vale
`__`, então `c.c` é `c__c`.

## 3. Bateria

Cada peça da bateria tem uma linha, e cada linha segue as regras de ritmo.
O símbolo decide o som, seja qual for o nome da linha:

| Símbolo | Som |
|---|---|
| `c` | chimbal fechado |
| `o` | chimbal aberto |
| `b` | bumbo |
| `s` | caixa |
| `r` | condução |
| `x` | ataque |

```aratu
drums {
  hh = cc cc co cc
  bd = b - bb -
  sd = - s - S        // caixa acentuada no tempo 4
}
```

Toda linha dentro de `drums { }` é de bateria, assim como uma linha chamada
`hh`, `bd`, `sd`, `cr`, `rd`, `kick`, `snare`, `hat`, `ride` ou `crash` em
qualquer lugar da peça. Um apóstrofo deixa a batida curta: `c'` é um chimbal
seco. O chimbal fechado corta o aberto que ainda está soando.

## 4. Melodias

As notas se escrevem em minúsculas, com a oitava: `dó4` (ou `c4`) é o dó
central. Sustenidos e bemóis vêm depois do nome: `fá#4`, `mib4`. Em letras:
`f#4`, `eb4`. Maiúscula acentua: `Dó4`. Um apóstrofo deixa a nota curta,
staccato: `dó4'`.

```aratu
melodia = | dó4 mi4 sol4 _ | lá4 sol4 mi4 dó4 |
```

A oitava continua até mudar, então `| dó4 mi sol dó5 |` é dó4, mi4, sol4,
dó5. Uma nota escrita antes de qualquer oitava vale oitava 4, e o Aratu
avisa.

Duas ou mais notas no mesmo tempo vão entre colchetes: `[ré mi]` são duas
colcheias, `[si si]` duas colcheias de si, `[mib sol]` mi bemol e depois sol,
`[dó4 _ _ ré]` uma colcheia pontuada e uma semicolcheia. Pausas e
prolongamentos podem ficar colados na nota sem colchetes (`-mi4` é pausa e
depois colcheia), mas duas notas não: `mib` é lido como mi bemol, então para
duas notas o Aratu pede os colchetes.

Notas que soam juntas se unem com `+`: `dó4+mi4+sol4` é um acorde de dó
maior num tempo.

## 5. Cifras e batidas

Uma sequência de acordes se escreve como uma cifra, numa linha chamada
`chords`. A regra da divisão vale um nível acima: acordes que dividem um
compasso dividem igualmente.

```aratu
chords = | Am | F | C G | Am _ _ G |
```

C e G ficam com meio compasso cada; Am dura três tempos e G um. As divisões
precisam cair nos tempos: três acordes num compasso 4/4 dividiriam o
compasso entre os tempos, então o Aratu pede que você diga quanto dura cada
um.

As cifras mantêm a escrita de sempre: `Am`, `F`, `G7`, `Cmaj7`, `Dm7b5`,
`E/G#`, e também a escrita brasileira: `C7M`, `Bº`, `G7(b9)`.

**A batida.** Uma linha chamada `strum` usa as regras de ritmo com símbolos
de violão:

| Símbolo | Significado |
|---|---|
| `d` | batida para baixo |
| `u` | batida para cima |
| `m` | batida abafada |
| `-` | a mão passa sem tocar |
| `_` | deixa soar |
| maiúscula | batida acentuada |

```aratu
violão {
  chords = | Am | F | C G | G |
  strum  = D du -u du
}
```

A batida se repete em cada compasso e os acordes trocam sozinhos. Na batida
para baixo as cordas soam da grave para a aguda, alguns milissegundos uma
depois da outra; na para cima, da aguda para a grave; os acentos batem mais
forte e mais rápido. Sem batida, os acordes tocam inteiros, como um teclado.

## 6. Dinâmica

As marcas de dinâmica são as da partitura: `ppp pp p mp mf f ff fff`, e
`sfz`, `fp`.

**Dentro de um compasso**, a marca vai colada na nota com dois-pontos, e
não conta como tempo:

```aratu
melodia = | p:dó4 mi4 sol4 dó5 | f:sol4 _ _ _ |
```

**Reguladores** se escrevem como na partitura: `<` crescendo, `>`
diminuendo, do tempo seguinte até a próxima marca.

```aratu
melodia = | p:dó4< ré4 mi4 fá4 | f:sol4> fá4 mi4 ré4 | p:dó4 _ _ _ |
```

**Para uma parte inteira**, uma linha muda o nível:

```text
piano: mf
piano: cresc to ff over 4 bars
drums: dim to p over 2 bars
all: fade out over 8 bars
```

O nível de uma parte multiplica as marcas escritas nas suas linhas: em `mf`
elas soam como escritas, em `ff` tudo soa mais forte na mesma proporção, e
`fade out` leva a parte até o silêncio.

Cada marca vale uma intensidade fixa: ppp 16, pp 33, p 49, mp 64, mf 80, f
96, ff 112, fff 127. O acento soma à marca em que a música está. Um
regulador sem marca depois anda um degrau (de `mf` para `f`, por exemplo)
até o fim da linha.

## 7. Andamento e tempo

```aratu
tempo: 96
at bar 3: rit to 70 over 2 bars
at bar 5: a tempo
hh = c c c c
```

| Escreva | Significa |
|---|---|
| `tempo: 96` | o primeiro andamento |
| `at bar 30: tempo 130` | um andamento novo num compasso |
| `rit to 80 over 2 bars` | ritardando (`rall` também vale) |
| `accel to 140 over 4 bars` | accelerando |
| `a tempo` | volta ao andamento de antes do rit |
| `tempo primo` | volta ao primeiro andamento |
| `tempo 120 -> 140 over 4 bars` | uma rampa entre dois andamentos |

**Quando uma mudança acontece.** Uma mudança com `at bar N:` acontece
naquele compasso. Uma mudança escrita sozinha acontece onde a peça começa
ou, enquanto você toca, no compasso seguinte ao momento em que é escrita.

Num ritardando ou accelerando o andamento muda aos poucos, tempo a tempo,
como se lê numa partitura, e cada nota cai no momento exato.

**Fermata.** `^` numa nota a sustenta: `sol4^` dura o dobro, e todas as
partes esperam por ela.

**Anacruse.** `pickup: beat 4` faz a peça começar no tempo 4 de um primeiro
compasso incompleto. A anacruse é o compasso 0, e o compasso 1 é o primeiro
completo, como os músicos contam. Uma linha cujo primeiro compasso tem o
tamanho da anacruse começa nela; as outras começam no compasso 1.

**Mudança de fórmula.** `at bar 17: meter 7/8`.

## 8. Expressão: rubato, swing e humanize

**Rubato** quer dizer tempo roubado, e o tempo roubado é devolvido.
`piano: rubato 10%` estica e encolhe os tempos dentro de cada compasso e
chega na barra no tempo certo, de modo que o piano respira enquanto a
bateria mantém o pulso. `all: rubato 10%` curva a peça inteira junto.

**Swing.** `swing 60%` atrasa toda segunda colcheia (50% é reto, 66% é a
sensação de tercina).

```aratu
swing 62%
hh = cc cc cc cc
bd = b - b -
```

**Humanize.** `humanize` acrescenta pequenos desvios aleatórios de tempo e
intensidade, como faria um músico; `humanize 5 ms` diz o quanto.

## 9. Padrões, seções, repetições e forma

O material é preparado antes e ganha um nome:

```aratu
pattern groove {
  hh = cc cc co cc
  bd = b - b -
  sd = - s - s
}

pattern fill {
  hh = cc cc cc cc
  bd = bb -b bb -b
  sd = ss ss SS SS
}

play groove
at bar 4: play fill
at bar 5: play groove
```

### Sinais de repetição

```text
chords = |: Am | F |1 C | G :|2 C | C |      // ritornelo, primeira e segunda casa
chords = |: Am | F :| x4                      // o trecho quatro vezes
chords = | Am | % | % | F |                   // % repete o compasso anterior
chords = | Am | F | %% |                      // %% repete os dois anteriores
```

`|:` e `:|` marcam um trecho tocado duas vezes, a menos que `x` diga
quantas. `|1` e `|2` começam a primeira e a segunda casa.

### Saltos

Os sinais vão no começo do compasso, os saltos no fim:

```aratu
melodia = | dó4 mi sol dó5 | segno: ré4 fá lá _ | mi4 sol si _ | fine: dó4 _ _ _ | sol4 _ _ _ D.S. al fine |
```

| Sinal | Salto |
|---|---|
| `segno:` | `D.S.`, `D.S. al fine`, `D.S. al coda` |
| `fine:` | onde `al fine` para |
| `coda:` | para onde `to coda` vai |
| | `D.C.`, `D.C. al fine`: volta ao começo |

A peça é desenrolada como um músico a lê. Um salto sem o seu sinal é um erro
dito com clareza: *"há um D.S. no compasso 24 de `melodia`, mas nenhum
segno para ele"*.

### Forma

Seções são partes da peça com nome, e uma linha de forma as ordena:

```aratu
section intro  { baixo = | lá2 _ _ _ | mi2 _ _ _ | }
section verso  {
  baixo = | lá2 _ mi2 _ | fá2 _ dó3 _ |
  drums { hh = cc cc co cc; bd = b - b - }
}
section refrao {
  violão { chords = | Am | F | C | G |; strum = D du -u du }
}
section final  { baixo = | lá2 _ _ _ | }

form: intro verso x2 refrao verso refrao x2 final
```

Uma seção guarda partes e linhas como a peça inteira, e dura o tanto da sua
linha mais longa. A forma toca as seções em ordem (`x2` toca uma duas vezes)
e então recomeça. Um bloco pode ficar numa linha só, com `;` entre as linhas.

### Variar um loop

```aratu
pattern groove { hh = cc cc co cc; bd = b - b -; sd = - s - s }
pattern fill   { hh = cc cc cc cc; sd = ss ss SS SS }
pattern crash  { cr = x - - - }

play groove x3, fill
every 8 bars, first: add crash
```

`play` escolhe o padrão que soa; `play groove x3, fill` toca uma frase de
padrões em sequência e a repete. `every 4 bars: play fill` toca a virada no
quarto compasso de cada quatro, no lugar do groove; `add` toca um padrão por
cima do que está soando.

Um `play` vale até outro: `at bar 8: play fill` toca a virada do
compasso 8 em diante. `once` (uma vez) toca o padrão uma vez e volta:

```text
at bar 8: play fill once     // a virada no compasso 8, o groove de volta no 9
now: play fill once          // uma virada já, depois o groove onde estaria
now: add crash once          // um prato por cima, uma vez
```

O padrão que estava tocando continua contando os compassos enquanto isso,
então `play groove x3, fill` não perde o lugar na frase. Com `every`, o
padrão já toca uma vez a cada volta.

## 10. Tocar ao vivo

Enquanto a peça toca, você a muda; você não a reescreve.

- **Tudo se repete.** Cada linha toca sem parar, cada uma no seu tamanho.
- **Uma mudança é uma linha.** `hh = cc cc oc cc` troca o chimbal e mais
  nada; as outras partes continuam.
- **As mudanças caem no tempo.** Uma edição entra no compasso seguinte;
  `now:` a faz entrar no tempo seguinte: `now: play fill once` traz uma
  virada no próximo tempo, e o groove volta na barra em que estaria.
- **Prepare antes.** `at bar 17: play fill` programa uma mudança.
- **Sair e voltar.** `violão: out` silencia uma parte a partir do compasso
  seguinte e `violão: in` a traz de volta; `all: out` para tudo. Funcionam
  com `at bar` e `every` também.
- **Nada recomeça.** As partes mantêm o lugar no compasso quando outra muda,
  e enquanto o texto tiver erro, continua tocando a última versão certa.

## 11. Transformações

Verbos curtos mudam uma parte, uma linha ou um padrão sem reescrevê-lo:

| Verbo | Efeito |
|---|---|
| `hh.rotate 1` | desloca o padrão em uma parte |
| `hh.density 50%` | mantém cerca de metade das batidas |
| `drums.velocity 0.8` | toca mais suave |
| `baixo.transpose +12` | uma oitava acima |
| `melodia.invert` | inverte os intervalos |
| `melodia.retrograde` | toca de trás para frente |
| `melodia.augment` | dobra as figuras |
| `melodia.diminish` | divide as figuras pela metade |

```aratu
melodia = | dó4 ré mi sol | lá sol mi ré |
melodia.transpose +7
```

## 12. Acaso

Um loop que se repete sempre igual logo soa mecânico. O acaso dá vida a ele:
o computador decide, um pouco ao acaso, o que tocar a cada volta. É o *ad
lib* da partitura.

**Às vezes.** Um `?` depois de um símbolo o faz tocar só às vezes, metade
das vezes por padrão; uma porcentagem diz com que frequência:

```aratu
hh = cc cc c?30% cc
sd = - s - s?80%
```

**Um ou outro.** Chaves escolhem uma entre várias opções, separadas por `|`,
a cada volta do loop:

```aratu
baixo = | {dó2|sol1} _ dó2 _ |
hh    = {cc|co} cc cc cc
```

**O mesmo acaso de novo.** As escolhas são sorteadas de novo a cada volta,
mas a partir de uma semente fixa, então a mesma peça improvisa do mesmo
jeito toda vez que é tocada do começo. `seed 7` escolhe outro improviso;
`seed random` faz cada apresentação diferente. Mudar uma linha não muda o
improviso das outras.

## 13. Sons e efeitos

Cada parte tem um som e uma cadeia de efeitos, escrita como o caminho do
sinal:

```aratu
baixo: synth.saw -> lowpass -> reverb
baixo = | dó2 _ dó2 mib2 | fá2 _ sol2 sib1 |
baixo {
  reverb.mix = 0.42
  lowpass.cutoff = 800 Hz
  lowpass.resonance = 0.3
}
```

Todo número que ajusta um parâmetro também é um controle: o editor desenha
um knob ao lado, com a forma do parâmetro (o de cutoff anda em hertz numa
escala logarítmica, o de mix de 0 a 1). Girar o knob reescreve o número,
então salvar o texto salva a apresentação. Segure Shift para passos mais
finos. Um número escrito em outra unidade a mantém: `1.2 kHz` gira em
quilohertz, `40%` em porcentagem.

| Parâmetro | Faixa |
|---|---|
| `volume` | 0 a 1 |
| `pan` | -1 (esquerda) a 1 (direita) |
| `attack`, `release` | 1 a 2000 ms, 5 a 5000 ms |
| `lowpass.cutoff`, `highpass.cutoff` | 20 a 20000 Hz |
| `lowpass.resonance`, `highpass.resonance` | 0 a 1 |
| `reverb.mix` | 0 a 1 |
| `delay.time` | 10 a 2000 ms |
| `delay.feedback`, `delay.mix` | 0 a 0,95, 0 a 1 |

Os sons são `synth.saw`, `synth.square`, `synth.sine`, `synth.triangle` e
`synth.pluck`; a bateria, as batidas e as cifras escolhem sozinhas uma
bateria, uma corda dedilhada e um teclado. São sons simples, embutidos no
navegador, até o motor do Manguebyte substituí-los.

Os parâmetros podem mudar por compasso como tudo mais: `at bar 16:
baixo.reverb.mix = 0.65`.

## 14. Referência rápida

| Escreva | Significa |
|---|---|
| `meter: 4/4`, `tempo: 96`, `key: A minor` | o quadro da peça |
| espaço | próximo tempo |
| `cc`, `cccc`, `ccc` | colcheias, semicolcheias, tercina num tempo |
| `_` | sustenta |
| `-` | pausa |
| `\| - \|`, `\| - x8 \|` | um compasso de pausa, oito compassos de pausa |
| `c.c` | colcheia pontuada e semicolcheia |
| maiúscula | acento |
| `dó4'` | staccato |
| `c?`, `c?30%` | às vezes (metade das vezes, ou com que frequência) |
| `{dó2\|sol1}` | um ou outro, escolhido a cada volta |
| `seed 7`, `seed random` | o mesmo improviso de novo, ou um novo a cada vez |
| `\|` | barra de compasso |
| `dó4 mi sol` | notas, com a oitava que continua |
| `fá#4`, `mib4`, `c4` | acidentes, letras |
| `[ré mi]` | notas agrupadas num tempo (necessário para duas ou mais) |
| `dó4+mi4+sol4` | notas juntas |
| `\| Am \| F \| C G \|` | cifras; compassos divididos igualmente |
| `d u m` | batida para baixo, para cima, abafada |
| `p:`, `mf:`, `f:` | dinâmica numa nota |
| `<`, `>` | crescendo, diminuendo |
| `cresc to ff over 4 bars` | mudança de dinâmica de uma parte inteira |
| `rit`, `accel`, `a tempo`, `tempo primo` | mudanças de andamento |
| `tempo 120 -> 140 over 4 bars` | rampa de andamento |
| `sol4^` | fermata |
| `pickup: beat 4` | anacruse |
| `rubato 10%`, `swing 60%`, `humanize` | expressão |
| `pattern nome { }`, `play nome`, `add nome` | padrões com nome, em sequência ou por cima |
| `\|: :\|`, `:\| x4`, `\|1 \|2` | ritornelo, vezes, casas |
| `%`, `%%` | repete o compasso anterior, os dois anteriores |
| `segno:`, `coda:`, `fine:` | sinais |
| `D.C.`, `D.S. al fine`, `D.S. al coda`, `to coda` | saltos |
| `section nome { }`, `form: intro verso x2 refrao` | seções e forma |
| `every 4 bars: play fill` | variação dentro do loop |
| `play fill once`, `add crash once` | um padrão uma vez, depois volta |
| `at bar 30:` | mudança programada |
| `now:` | mudança no próximo tempo |
| `violão: out`, `violão: in` | silencia uma parte, traz de volta |
| `->` | caminho do sinal |
| `parte.parâmetro = valor` | um parâmetro, desenhado como knob |
