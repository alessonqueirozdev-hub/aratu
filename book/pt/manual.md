# O manual do Aratu

O Aratu se escreve do jeito que um músico pensa: em compassos, tempos,
figuras, cifras e palavras da partitura. Quem lê uma cifra e conta um
compasso consegue ler uma peça em Aratu. Este manual descreve a linguagem
inteira; o curso **Aprender** ensina passo a passo. Todo exemplo aqui pode
ser ouvido e modificado. Para quem programa, [**Por dentro**](#/por-dentro)
dá nome técnico a cada coisa e mostra o que o computador faz com o texto.

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

**Acentos.** Um símbolo maiúsculo é acentuado: `cC cc` acentua o contratempo
do tempo 1.

**Barras.** `|` é a barra de compasso, como na partitura. Uma linha pode ter
vários compassos: `| cc cc | co cc |`. Uma linha sem barras é um compasso.

**Grupos.** Colchetes formam um grupo que ocupa uma parte, e aceitam espaços
dentro: `c[cc]` é uma colcheia e duas semicolcheias. O atalho `.` vale
`__`, então `c.c` é `c__c`.

**Ritmos euclidianos.** Um símbolo seguido de dois números entre parênteses
espalha aquele número de toques o mais por igual possível por aquele número
de passos: `b(3,8)` é `b- -b -- b-`, o tresillo, e `b(5,8)` é
`b- bb -b b-`, o cinquillo. Sozinho no compasso, ele se espalha pelo
compasso inteiro, em qualquer fórmula de compasso, como uma pausa sozinha;
ao lado de outros tempos, é um tempo, e `2[b(3,8)]` ocupa dois. Um terceiro
número o gira para depois, naquele número de passos, como `rotate`:
`b(3,8,2)`. As marcas valem para todos os toques (`S(2,4)`, `c(5,8)?`), e
ele funciona em bateria, toques de percussão, batidas e notas (`dó2(3,8)`).

```aratu
drums {
  bd = b(4,16)
  hh = c(5,8,1)
  sd = - s - s
}
```

## 3. Compassos compostos, aditivos e quiálteras

O tempo nem sempre é uma semínima. Estas são as fórmulas em que ele muda de
tamanho ou se agrupa de outro jeito, e as quiálteras que atravessam tempos.

**Compassos compostos.** Em 6/8 o tempo é uma semínima pontuada, então
`ccc ccc` é um compasso de colcheias contado em dois, como o músico conta.

```aratu
meter: 6/8
tempo: 70
hh = ccc ccc
bd = b__ b__
```

**Quiálteras que atravessam tempos.** Um número antes dos colchetes diz
quantos tempos o grupo ocupa: `2[dó4 ré mi]` são três notas em dois tempos
(a tercina de semínimas) e `3[c c c c]`, quatro em três.

**Compassos aditivos.** `meter: 7/8 = 2+2+3` diz como as colcheias se
agrupam. O compasso se escreve com uma palavra por grupo (`cc cc ccc`) ou
uma por colcheia (`c c c c c c c`), e o andamento conta colcheias.

```aratu
meter: 7/8 = 2+2+3
tempo: 240
hh = cc cc ccc
bd = b  b  b
```

**Mudança de fórmula.** `at bar 17: meter 7/8`.

## 4. Bateria

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
| `t` | tom agudo |
| `m` | tom médio |
| `f` | surdo |
| `k` | aro (baqueta atravessada na caixa) |
| `p` | palma |

```aratu
drums {
  hh = cc cc co cc
  bd = b - bb -
  sd = - s - S        // caixa acentuada no tempo 4
}
```

Toda linha dentro de `drums { }` é de bateria, assim como uma linha chamada
`hh`, `bd`, `sd`, `cr`, `rd`, `ht`, `mt`, `ft`, `toms`, `rim`, `kick`,
`snare`, `hat`, `ride` ou `crash` em qualquer lugar da peça, e também pelos
nomes em português e espanhol: `chimbal`, `bumbo`, `caixa`, `prato`,
`condução`, `ataque`, `aro`, `tons`, `palmas`, `charles`, `bombo`, `caja`,
`platillo`; o bloco também pode se chamar `bateria`. Uma linha chamada
`surdo` é o surdo do samba (veja a percussão abaixo); o surdo da bateria é
o `f`, em qualquer linha de bateria. Um apóstrofo deixa a batida curta: `c'` é um chimbal
seco. O chimbal fechado corta o aberto que ainda está soando.

**Rufo.** Um til depois do símbolo faz um rufo, como os tracinhos de trêmulo
na partitura: golpes de fusa (oito por tempo) durante toda a duração da
batida, com as mãos alternando. `s~ _ _ S` é um rufo de três tempos que
fecha num acento; com um regulador ele cresce até a próxima marca:

```aratu
drums {
  sd = p:s~< _ _ f:S
}
```

Vale para qualquer peça (`f~` no surdo, `x~` no prato) e para notas, como
trêmulo: `dó4~ _`. Uma crase antes do símbolo põe um toque leve do mesmo
tambor logo antes: `` `s `` é um flam e ``` ``s ``` um drag (veja o
capítulo 9).

## 5. Percussão brasileira

Uma linha com o nome de um instrumento lê os golpes dele, com as mesmas
regras de ritmo:

| Linha | Golpes |
|---|---|
| `pandeiro` | `p` polegar, `d` dedos, `t` tapa, `m` abafado |
| `zabumba` | `o` aberto, `m` abafado, `b` bacalhau |
| `triângulo` | `o` aberto, `f` fechado |
| `agogô` | `a` agudo, `g` grave |
| `surdo` | `o` aberto, `m` abafado |
| `tamborim`, `alfaia`, `gonguê` | `o` aberto, `m` abafado |
| `ganzá` | `s` |
| `cuíca` | `a` agudo, `g` grave |
| `cajón` | `b` grave, `t` tapa |

```aratu
tempo: 120
pandeiro = pd td pd td
zabumba  = | o - -o m | o - b - |
triângulo = fo fo fo fo
```

Maiúscula acentua e `(p)` é um golpe fantasma, como na bateria. Por
enquanto esses sons são sintetizados no navegador; as gravações virão
pelo motor do Manguebyte.

## 6. Melodias

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

**Linhas longas.** Uma linha que começa com `|` continua a linha de música
de cima, então uma melodia longa pode ocupar várias linhas:

```aratu
melodia = | dó4 ré mi fá | sol lá si dó5 |
          | si4 lá sol fá | mi ré dó _ |
```

Os compassos também podem começar na linha de baixo do `=`:

```aratu
melodia =
  | dó4 ré mi fá | sol lá si dó5 |
  | si4 lá sol fá | mi ré dó _ |
```

## 7. Cifras e batidas

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
`E/G#`, e também a escrita brasileira: `C7M`, `Bº`, `G7(b9)`. Valem ainda
`C6/9`, `E7alt` e a cifra com dó-ré-mi: `Lám`, `Sol7`, `Fá#m7`, `Dó/Mi`.
`N.C.` (ou `-`) é um compasso sem acorde.

**A batida.** Uma linha chamada `strum` usa as regras de ritmo com símbolos
de violão:

| Símbolo | Significado |
|---|---|
| `d` | batida para baixo |
| `u` | batida para cima |
| `m` | batida abafada |
| `g` | golpe no tampo (flamenco), sem as cordas |
| `t` | para baixo com golpe, tudo junto |
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

**O arpejador.** Uma linha chamada `arp` ao lado dos acordes toca as notas
deles uma de cada vez, em passos, enquanto cada acorde durar; os acordes
então só soam por ela, como por uma batida. Ela diz o caminho, os passos e,
se for mais de uma, as oitavas: `arp = up 16ths`, `down 8ths`,
`updown 16ths` (ou `up and down`), `random 16ths`, `up triplets`,
`up beats`, `up 16ths 2 octaves`. Os passos seguem o compasso (em 6/8 uma
colcheia é um terço de tempo), e cada acorde recomeça da sua nota mais
grave. Uma linha chamada `arp` com notas (`arp = | dó4 mi sol dó5 |`)
continua sendo uma melodia.

```aratu
piano {
  chords = | Am | F | C | G |
  arp    = updown 16ths 2 octaves
}
```

## 8. Harmonia pelo grau

**Acordes pelo grau.** Em algarismos romanos, relativos ao tom do começo
da peça (dó maior quando não há `key:`): maiúscula é maior, minúscula é
menor, e a qualidade vem depois. `bVII` empresta de fora do tom e `V7/V`
é a dominante da dominante. Com `:arp` o acorde soa arpejado, da nota mais
grave para a mais aguda; numa melodia, `dó4+mi4+sol4:arp`.

```aratu
key: C major
chords = | I | vi | ii7 | V7:arp |
```

| Escreva | Significa |
|---|---|
| `I` … `VII` | o grau, acorde maior |
| `i` … `vii` | o grau, acorde menor |
| `V7`, `ii7`, `IV7M`, `vii°` | depois do grau, o resto da cifra |
| `bVII`, `#iv°` | um acorde de fora do tom, um semitom abaixo ou acima do grau |
| `V7/V`, `V7/ii` | a dominante do grau escrito depois da barra |
| `I:arp` | o acorde arpejado |

**Melodias pelo grau.** Numa linha de melodia, um número é um grau do tom do
começo da peça: `1` é a tônica na oitava 4, `8` uma oitava acima; um
acidente depois do número sai da escala (`3b`, `4#`), e cada vírgula desce
uma oitava (`5,`, `1,,`). Os graus seguem a escala maior ou a menor natural.

```aratu
key: A minor
melodia = | 1 3 5 7b | 8 _ 5, _ |
```

Num tom menor os graus seguem a escala menor natural: em `key: A minor`
(ou `tom: Lá menor`), `i` é Am, `iv` é Dm, `VI` é F, e a dominante com
sensível se escreve `V7` (E7).

```aratu
key: A minor
chords = | i | iv | VI | V7 |
```

## 9. Dinâmica e articulação

As marcas de dinâmica são as da partitura: `ppp pp p mp mf f ff fff`, e
`sfz`, `fp`.

**Dentro de um compasso**, a marca vai na nota com dois-pontos, colada ou
com um espaço (`p:dó4` ou `p: dó4`), e não conta como tempo. Antes de colchetes, vale a partir da primeira nota
do grupo: `p:[dó4< sol4]`. Em `f:fá4`, o primeiro `f` é forte e o segundo é
a nota:

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

Uma linha de nível leva a parte inteira até a dinâmica que ela diz:
`melodia: cresc to ff over 4 bars` parte do que está escrito e chega a ff
no quarto compasso, mesmo que a linha esteja escrita em p. Os acentos
continuam acima das outras notas, e `fade out` leva a parte até o
silêncio.

Cada marca vale uma intensidade fixa: ppp 16, pp 33, p 49, mp 64, mf 80, f
96, ff 112, fff 127. O acento soma à marca em que a música está. Um
regulador sem marca depois anda um degrau (de `mf` para `f`, por exemplo)
até o fim da linha.

**Marcato e notas fantasma.** `s!` é marcato, mais forte que a maiúscula.
`(s)` é uma nota fantasma, quase sem som, como na partitura de bateria;
vale também para notas: `(dó4)`.

**Ornamentos.** Vão depois da nota, com dois-pontos. `dó5:tr` é um
trinado: a nota e a de cima na tonalidade, a cada oitavo de tempo,
começando e terminando na nota; `:tr#` e `:trb` sobem ou descem a de cima.
`dó5:mord` é um mordente: a nota, a de cima e a nota de novo, um oitavo de
tempo cada (menos numa nota curta); `dó5:mordi` usa a de baixo. Também
valem `:trill`, `:trinado`, `:trino`, `:mordent` e `:mordente`. A vizinha
segue a tonalidade de `key:` e as suas mudanças (dó maior quando não há
nenhuma).

**Notas de graça, flam e drag.** Uma crase antes de uma nota faz dela uma
nota de graça: não conta como tempo e soa um instante antes da seguinte
(`` `ré5 dó5 ``). Podem vir várias (`` `si4 `dó5 ré5 ``), coladas ou não, e
de um compasso para o seguinte. Num tambor, a crase põe um toque mais leve
do próprio tambor logo antes: `` `s `` é um flam, ``` ``s ``` um drag.

```aratu
key: G major
melodia = | sol4 si4 `dó5 ré5 _ | ré5:mord dó5 si4 lá4 | lá4:tr _ _ _ | sol4 _ _ _ |
sd = - `S - ``S
```

## 10. Andamento e tempo

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
partes esperam por ela. Com um número, dura quantas vezes se escrever:
`sol4^3` dura o triplo.

**Anacruse.** `pickup: beat 4` faz a peça começar no tempo 4 de um primeiro
compasso incompleto. A anacruse é o compasso 0, e o compasso 1 é o primeiro
completo, como os músicos contam. Uma linha cujo primeiro compasso tem o
tamanho da anacruse começa nela; as outras começam no compasso 1.

## 11. Expressão: rubato, swing e humanize

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

## 12. Repetições e saltos

Os sinais de repetição e de salto da partitura, e a repetição de um tempo.

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

### Repetir um tempo

Um `x` colado no colchete repete aquele tempo, cada vez no seu próprio
tempo: `| [mi4 ré dó ré]x4 |` é o grupo quatro vezes. O `x` vai colado ao
`]` (ou a um motivo); com espaço, `| - x8 |` continua sendo oito compassos
de pausa. `4x` também vale: `[mi4 ré dó ré]4x`. Chaves são para o acaso
(`{mi4|sol4}`); um grupo entre chaves sem `|` toca como grupo, com um aviso
para trocar por colchetes.

## 13. Padrões, seções e forma

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

## 14. Motivos, séries, listas e transformações

Ideias curtas com nome, usadas muitas vezes: motivos, séries dodecafônicas,
listas de melodias e as transformações que as mudam sem reescrevê-las.

### Motivos

`motif nome = …` dá nome a alguns tempos, sem barras de compasso, e o nome
se usa no lugar deles. O motivo pode ser definido antes ou depois de onde é
usado, pode usar outros motivos, e enquanto toca o editor acende as notas na
definição.

```aratu
key: C major
motif riff = [mi4 ré dó ré]
melodia = | riff riff+1 riff.inv riff.retro |
```

| Escreva | O motivo |
|---|---|
| `riff+2`, `riff-1` | dois graus acima, um abaixo, na tonalidade (`key:`; dó maior quando não há) |
| `riff+3st` | três semitons acima |
| `riff.inv` | invertido em torno da primeira nota |
| `riff.retro` | as notas de trás para frente, no mesmo ritmo |
| `riff.ri` | retrógrado da inversão; combina com `+n`: `riff.ri+2` |
| `riffx4` | quatro vezes |

Uma nota fora da escala, transposta por graus, guarda a distância da nota
da escala abaixo dela: dó# um grau acima vira ré#.

### Séries dodecafônicas

`row nome = …` guarda uma série: números de 0 a 11 (0 é dó) ou nomes de
nota, com ou sem oitava. A série dá as alturas e um ritmo com `o` dá os
lugares, depois de `from`:

```aratu
row serie = 0 11 3 4 8 7 9 5 6 1 2 10
melodia = | o o [o o] o | o _ [o o] o | O o o o | from serie.RI3 octave 4
```

Cada `o` toma a próxima nota, e a série recomeça quando acaba, como na
isorritmia (`O` é um `o` acentuado). As formas são `P`, `I`, `R` e `RI`,
cada uma de 0 a 11 semitons acima: `serie.P0` é a série escrita, `serie`
sozinho também. `octave N` põe todas as notas na oitava N. Depois de `from`
também vale um motivo (as suas notas, em ordem) ou um vetor escrito ali:
`from [0 4 7]`. Uma série de doze notas que repete alguma recebe um aviso,
e a aba **Séries**, ao lado das mensagens em Tocar, mostra a matriz 12×12 de
cada série, com os nomes das formas para clicar.

### Listas de melodias

`list nome { }` guarda melodias, uma por linha, com barras de compasso;
podem usar motivos e séries.

```aratu
list temas {
  | mi4 ré dó ré | mi mi mi _ |
  | ré4 ré ré _ | mi sol sol _ |
}
melodia = temas.1
```

`temas.2` escolhe a segunda (conta-se a partir de 1), `temas.all` toca
todas em ordem e `temas.random` sorteia uma inteira a cada volta, sempre a
mesma sequência para a mesma `seed`. Mudar o número com a música tocando
troca a melodia no próximo compasso. No modo palco, botões escolhem a
melodia de cada lista, e Alt+1 a Alt+9 escolhem a da linha onde está o
cursor.

Cada compasso de uma linha também pode ser uma melodia da lista:
`baixo = | temas.1 | temas.3 | temas.2 |`, misturada com compassos escritos
se quiser, e `temas.all` no lugar de um compasso põe todas, em ordem. Uma
lista que cita a si mesma para, com uma mensagem.

### Transformações

Verbos curtos mudam uma parte, uma linha ou um padrão sem reescrevê-lo:

| Verbo | Efeito |
|---|---|
| `hh.rotate 1` | desloca o padrão em uma parte |
| `hh.density 50%` | mantém cerca de metade das batidas |
| `drums.velocity 0.8` | toca mais suave |
| `baixo.transpose +12` | uma oitava acima |
| `melodia.invert` | inverte os intervalos |
| `melodia.retrograde` | toca de trás para frente |
| `melodia.ri` | retrógrado da inversão |
| `melodia.augment` | dobra as figuras |
| `melodia.diminish` | divide as figuras pela metade |
| `hh.fast 2` | dois compassos da linha em cada compasso |
| `baixo.slow 2` | cada compasso da linha em dois |
| `lead.echo 1 eighth x3` | cada som de novo uma colcheia depois, três vezes, cada vez mais baixo |

```aratu
melodia = | dó4 ré mi sol | lá sol mi ré |
melodia.transpose +7
```

Os nomes são os mesmos dos motivos e das séries: `.inv` ou `.invert`,
`.retro` ou `.retrograde`, `.ri`. `transpose` conta semitons e aceita
escrever `3st`, como nos motivos. `fast` e `slow` levam números inteiros e
se multiplicam: `augment` é `slow 2`, `diminish` é `fast 2`, e `fast 2` com
`slow 2` toca a linha como foi escrita. `echo` leva uma duração, como
`over`.

## 15. Tocar ao vivo

Enquanto a peça toca, você a muda; você não a reescreve.

- **Tudo se repete.** Cada linha toca sem parar, cada uma no seu tamanho.
- **Uma mudança é uma linha.** `hh = cc cc oc cc` troca o chimbal e mais
  nada; as outras partes continuam.
- **As mudanças caem no tempo.** Uma edição entra no compasso seguinte;
  `now:` a faz entrar no tempo seguinte: `now: play fill once` traz uma
  virada no próximo tempo, e o groove volta na barra em que estaria.
- **Prepare antes.** `at bar 17: play fill` programa uma mudança;
  `at bar 17 beat 3:` a faz entrar no terceiro tempo (`beat 2.5` cai entre o
  2 e o 3), e `from bar 17 to 24:` a faz valer só nesses compassos: no 25
  volta o que valia antes. Uma linha também muda num compasso:
  `at bar 9: melodia = | … |` toca a melodia nova a partir do 9, desde o
  começo dela. `play groove 3x` é o mesmo que `x3`.
- **Durações.** `over` aceita `bars`, `beats`, `eighths` e `sixteenths`
  em `cresc`, `fade`, `rit`, `accel`, rampas de andamento e parâmetros:
  `piano: cresc to ff over 6 beats`.
- **Sair e voltar.** `violão: out` silencia uma parte a partir do compasso
  seguinte e `violão: in` a traz de volta; `all: out` para tudo. Funcionam
  com `at bar` e `every` também.
- **Nada recomeça.** As partes mantêm o lugar no compasso quando outra muda,
  e enquanto o texto tiver erro, continua tocando a última versão certa.

**O palco.** Para tocar diante de uma plateia, o botão **Palco** em Tocar
(ou Ctrl+Shift+F) deixa só o código na tela, em tela cheia e com letra
grande para o projetor. Uma faixa no alto mostra sempre o compasso, os
tempos e o andamento, com uma linha que corre na duração de um compasso. O
contorno acende no primeiro tempo de cada compasso, os símbolos acendem
quando soam e a primeira mensagem aparece embaixo. Os botões da faixa tocam
e param, trazem as cenas, mudam o tamanho da letra (A− e A+), mostram o
mapa do ritmo embaixo e saem do palco; eles somem quando o mouse para e
ficam numa tela de toque. Esc também sai. O botão **2 colunas** divide o
código em duas colunas, como num jornal: a segunda começa de onde a
primeira termina, e continua assim enquanto você escreve. Cada coluna rola
sozinha: rolar uma não mexe a outra, e cada uma fica onde você a deixou até
a letra mudar de tamanho. Editar em qualquer uma muda o mesmo texto. No
celular o palco tem uma coluna só e um tamanho de letra próprio.
**Caber na tela** encolhe a letra conforme a música cresce, para tudo caber
sem rolar, até um tamanho ainda legível; daí em diante, as colunas rolam.
Desligado, a letra fica do tamanho escolhido com A− e A+, e você rola.

**Seguir a música e ver o som.** O botão **Seguir**, em Tocar e no palco,
leva a tela até o trecho que está tocando; ela espera enquanto você
digita, clica ou rola. O visor mostra a onda da música enquanto toca, e
um clique nela mostra as frequências. Uma linha com o nome de uma parte
ou de uma linha, um ponto e `onda` (`baixo.onda`) desenha embaixo dela a
onda dessa parte; com `rolo` (`drums.hh.rolo`), as notas do compasso
passando sob uma linha que anda, uma fileira para cada nota ou tambor. A
onda de uma parte é o seu som de verdade depois dos efeitos, lido do
osciloscópio do Manguebyte para aquela parte e parado numa passagem por
zero subindo, umas três voltas da nota mais grave; uma linha da bateria,
ou uma parte que o provisório toca, é desenhada a partir das notas que
toca. Os desenhos não mudam o som.
O mapa do ritmo, embaixo do editor, mostra a música no tempo: uma coluna
para cada compasso, na ordem em que toca, com a seção em cima, e uma
fileira para cada parte; uma linha laranja corre por ele e ele rola
junto. Uma peça que recomeça (uma forma, um loop) aparece uma volta só.

**Compartilhar e gravar.** O botão **Compartilhar**, em Tocar, copia um
link com a música dentro: quem abrir o link ouve e edita a mesma peça, e o
Ctrl+Z traz de volta o que estava no editor dela. A música vai na parte do
endereço depois do `#`, então não passa por nenhum servidor. O botão
**Gravar** grava tudo o que sai nas caixas de som, desde o momento em que é
apertado (e começa a tocar, se a música estiver parada) até ser apertado de
novo; aí o arquivo é baixado, em WAV quando o navegador consegue.

**Gestos de palco.** Uma palavra muda o arranjo inteiro, no próximo
compasso, no próximo tempo com `now:`, ou no compasso marcado com
`at bar`:

| Escreva | O que faz |
|---|---|
| `breakdown` (`quebra`) | a bateria, a percussão e o baixo saem |
| `build over 8 bars` (`subida em 8 compassos`) | um rufo de caixa em semínimas, colcheias, semicolcheias e fusas, crescendo, enquanto um filtro passa-altas sobe em todas as partes e volta ao fim; sem duração, 4 compassos |
| `drop` (`queda`) | tudo volta, com um prato no primeiro tempo |
| `solo baixo`, `solo off` | só uma parte toca; `solo off` (ou `all: in`) traz o resto |

`from bar 17 to 24: breakdown` vale só nesses compassos.

**Levadas prontas.** `levada house` (ou `groove`, `ritmo`) escreve as linhas
de uma bateria: house, techno, trap, dnb, breakbeat, hiphop, funk, rock,
reggaeton, funk carioca, bossa, samba, baião, maracatu. As levadas são em
4/4. Enquanto tocam, a palavra acende; uma linha escrita com o mesmo nome,
na mesma parte, toma o lugar da linha da levada.

**Atalhos.** Ctrl+Enter toca ou atualiza; Ctrl+. para; Ctrl+/ comenta as
linhas selecionadas (elas param de soar); Alt+M escreve ou apaga
`parte: out` para a parte da linha do cursor; Alt+S escreve ou apaga
`solo parte`.

**Cenas.** Oito lugares, em Tocar e no palco, guardam a música do editor e
a trazem de volta: clique numa cena vazia para guardar, numa guardada (ou
Ctrl+Shift+1 a 8) para trazer, e Shift+clique para guardar por cima. Com a
música tocando, a cena entra no próximo compasso, e o Ctrl+Z desfaz. As
cenas ficam neste navegador.

## 16. Acaso

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

## 17. Sons e efeitos

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

Os efeitos são os do Manguebyte, e entram na cadeia em qualquer ordem
(`baixo: synth.saw -> eq -> chorus -> delay`); o motor os roda sempre na
mesma ordem, do filtro ao delay:

| Efeito | Parâmetros |
|---|---|
| a parte | `volume` (0 a 1), `pan` (-1 esquerda a 1 direita), `reverb.mix` (quanto vai para o reverb da mixagem, 0 a 1) |
| `highpass`, `lowpass` | `cutoff` (20 a 20000 Hz), `resonance` (0 a 1) |
| `eq` | `low_gain`, `mid_gain`, `high_gain` (-24 a 24 dB), `low_freq`, `mid_freq`, `high_freq`, `mid_q` |
| `drive` | `amount` (0 a 36 dB), `output`, `mix`, `shape` (0 suave, 1 macio, 2 duro, 3 dobrado, 4 válvula) |
| `bitcrush` | `bits` (1 a 16), `rate` (Hz), `mix` |
| `compressor` | `threshold`, `ratio`, `attack`, `release`, `knee`, `makeup`, `mix` |
| `chorus`, `flanger`, `phaser` | `mix`, `rate`, `depth`, e mais: `voices`, `feedback`, `stages`… |
| `tremolo`, `autopan` | `rate`, `depth`, `shape` (0 senoide, 1 triângulo, 2 quadrada, 3 e 4 serra), `sync` (em tempos) |
| `delay` | `time` (em s ou ms), `feedback`, `mix`, `damping`, `pingpong`, `sync` (em tempos) |
| `tube` (`valvula`) | `drive` (0 a 24 dB), `output`, `mix`: o drive do Manguebyte com a sua curva de válvula (forma 4), que achata o alto da onda antes do baixo e acrescenta harmônicos pares; numa cadeia, ou a válvula ou o drive |

Ao lado de `eq`, `tube`, `drive`, `lowpass` e `highpass` numa cadeia, o
editor desenha a curva do efeito com os números da parte: a resposta do
equalizador e dos filtros, dos graves aos agudos, e o quanto o drive e a
válvula achatam o som. A curva do `eq`, clicada, abre um equalizador cujos
pontos dos graves, médios e agudos se arrastam; os números são reescritos
no código, ou escritos embaixo da cadeia quando ainda não estão.

**Os sons.** Cada som tem um código, escrito no começo da cadeia:
`baixo: bass.slap -> reverb`, `drums: drums.808`, `piano: piano.grand`. O
painel **Sons**, ao lado das mensagens em Tocar, lista todos por família
(baterias, teclados, teclados de percussão e sinos, cordas dedilhadas,
cordas de arco, baixos, sopros, vozes, sintetizadores), toca um compasso de cada um e põe o código
no texto com um clique. Um código que não existe é avisado, com o mais
parecido: *"seria `bass.slap`?"*. Sem código, a bateria, o violão e as
cifras escolhem sozinhos.

Alguns sons tocam com o acento: no `bass.slap` a nota normal é o polegar e
a maiúscula é o *pop*, a corda puxada (`| mi2 Mi3 mi2 Mi3 |`); no
`bass.acid` a maiúscula abre o filtro, como o accent da TB-303.

Os sons marcados como **gravados** (pianos, cravo, órgão de tubos, harpa,
marimba, vibrafone, xilofone, glockenspiel, kalimba, sinos tubulares, sax
tenor, gaita, flauta doce e a bateria acústica) são gravações da Versilian
Community Sample Library, em domínio público (CC0), baixadas na primeira vez
que uma peça as usa. Enquanto chegam, um som sintetizado parecido toca no
lugar. Vêm na taxa em que foram gravados, em estéreo (a gaita, gravada em
mono, em mono). Os pianos, o órgão de tubos, a harpa, a marimba, o
vibrafone, o xilofone e o glockenspiel, os sinos tubulares, o sax tenor, a
gaita e as peças da bateria acústica (menos o chimbal aberto e o aro) têm
gravações em duas ou três dinâmicas: quanto mais suave a nota, mais suave e
escura a gravação que ela toca, e quanto mais forte, mais brilhante (`f`,
`ff` e um acento num `mf` chegam à mais forte; na bateria, uma nota
fantasma `(s)` toca a mais suave). No órgão, as duas são registros, um
discreto e um forte; na gaita, as gravações fortes são tocadas com acento.
No piano de armário e na bateria, as notas gravadas mais de uma vez revezam
as tomadas.

A bateria gravada do Alesson (`drums.alesson`) foi gravada por Alesson
Queiroz e também está em domínio público (CC0). Bumbo, caixa e ataque são
gravações, com várias tomadas de cada um que se revezam; na caixa, a nota
fantasma `(s)` e as dinâmicas mais suaves tocam um toque leve, e os
acentos, um golpe mais forte. As outras peças (chimbais, condução, tons)
são sintetizadas.

**O motor do Manguebyte.** Todos os sons do painel tocam pelo motor do
Manguebyte, o motor de áudio do ecossistema, que roda no navegador: os
gravados num sampler; violões e baixos (`guitar.nylon`, `guitar.steel`,
`bass.finger`, `bass.slap`) em cordas modeladas; o piano elétrico e o sino
(`keys.epiano`, `keys.bell`) em FM; o órgão (`keys.organ`) com a sua caixa
rotativa, cuja velocidade é `organ.rotary` (0 parada, 1 lenta, 2 rápida); a
baixo 808 (`bass.808`), o acid (`bass.acid`) e os sintetizadores
(`synth.*`); as baterias eletrônicas (`drums.808`, `drums.909`,
`drums.606`, `drums.cr78`, `drums.trap`, `drums.house`, `drums.techno`,
`drums.dnb`, `drums.lofi`, e `drums.modeled`, cujas peças são modeladas
enquanto tocam) na sua bateria eletrônica; os códigos `poly.*`, os sons
prontos do seu sintetizador polifônico; a celesta e o steelpan
(`mallets.celesta`, `mallets.steelpan`) em lâminas modeladas; as cordas de
arco (`strings.*`, também em pizzicato), os sopros (`flute`, `clarinet`,
`sax`, `trumpet`, `trombone`) e as vozes (`choir.aah`, `choir.ooh`,
`voice.solo`) como instrumentos modelados. Cada parte tem os seus efeitos no motor, e a
mixagem passa por um reverb compartilhado, um compressor de cola e um
limitador. A percussão brasileira e o golpe ainda tocam num motor
provisório, até o Manguebyte tê-los; se algo der errado numa apresentação,
o interruptor do painel Sons desliga o Manguebyte e tudo passa a tocar no
provisório.

**Um som só toca depois de baixado.** Ao apertar **Tocar** ou **Ouvir**, o Aratu espera
os sons da peça chegarem, e um aviso embaixo da tela mostra quais estão
baixando. Um som trocado com a música tocando entra quando chega; até lá
continua o anterior. Se um download falhar, ou não responder em 60
segundos, o mesmo aviso diz qual som, por quê, e o que toca no lugar: a
cópia do motor provisório ou um som parecido.

Os parâmetros podem mudar por compasso como tudo mais: `at bar 16:
baixo.reverb.mix = 0.65`.

## 18. Números que se movem

**Parâmetros que se movem.** `baixo.lowpass.cutoff = 300 Hz -> 4000 Hz over
4 bars` leva o cutoff de 300 a 4000 Hz em quatro compassos e o deixa lá,
como um crescendo do som.

| Escreva | O que faz |
|---|---|
| `A -> B over 4 bars` | de A a B, uma vez, e fica em B |
| `A <-> B over 4 bars` | vai e volta, 4 compassos para cada lado, sem parar |
| `A -> B over 4 bars, back over 2 beats` | sobe e volta numa outra duração, uma vez |
| `A <-> B over 4 bars, back over 1 bar` | o mesmo, sem parar |
| `A -> B over 1 bar, loop` | sobe e recomeça de A, em rampa |

A duração vai em `bars`, `beats`, `eighths` ou `sixteenths`; num
compasso composto como 6/8, o tempo é a semínima pontuada, três colcheias. O
caminho segue a escala do parâmetro: frequências e tempos andam por razões
iguais, como o knob gira, e mix, volume e pan em linha reta. O Aratu marca um
ponto a cada dezesseis avos de tempo e o som desliza entre eles.

Sem tempo escrito, a linha começa a se mover onde foi escrita: no começo da
peça, ou no compasso seguinte se a música já toca. `at bar 9:` a faz
começar no compasso 9; `every 8 bars:` a faz terminar no fim de cada oito
compassos, como uma virada, e `every 8 bars, first:`, começar no primeiro.
Uma mudança posterior do mesmo parâmetro (`at bar 17: baixo.lowpass.cutoff =
800 Hz`) toma o lugar. Mudar os números com a música tocando não faz o
movimento recomeçar: ele segue de onde está, no novo caminho. Os dois
números ganham knob, e enquanto toca um segundo ponteiro mostra onde o
parâmetro está; uma versão nova do texto não o faz voltar ao valor
escrito.

## 19. Mixagem

Cada parte tem `volume` (0 a 1) e `pan` (-1, toda à esquerda, a 1, toda à
direita), como os faders e os botões de panorama de uma mesa de som:
`drums.volume = 0.5`, `violão.pan = -0.6`. O `reverb.mix` de cada parte diz
quanto dela vai para a sala compartilhada da mixagem.

Uma parte que só existe dentro de seções ou padrões se mixa do mesmo jeito,
daqui de fora: o número vale onde quer que ela toque, e também numa mudança
`at bar` ou num número que se move.

```aratu
section baque { alfaia = O -o o_oo o; caixa = Ssss Ssss Ssss Ssss }
form: baque x4
alfaia.volume = 1
at bar 3: alfaia.volume = 0.6
```

**Para a obra inteira.** Um número escrito sem parte vale para todas:
`reverb.mix = 0.5` põe todas as partes no reverb da mixagem. Uma cadeia
escrita para `todos` (ou `all`) põe os seus efeitos em todas as partes,
depois dos de cada uma. E o próprio barramento da mixagem, o master, tem
os seus números:

```aratu
todos: chorus -> delay
reverb.mix = 0.5
master.reverb.decay = 3 s
master.glue.on = 1
master.limiter.ceiling = -1
```

| Master | O que é |
|---|---|
| `master.reverb.decay`, `.size`, `.damping`, `.level` | o reverb compartilhado: duração (s), tamanho, abafamento, volume (dB) |
| `master.glue.on`, `.threshold`, `.ratio`, `.makeup` | o compressor de cola: 1 liga; limiar (dB), razão, ganho (dB) |
| `master.gain` | o volume antes do limitador, em dB |
| `master.limiter.ceiling` | o teto do limitador, de -12 a -0,3 dB |

**Ducking.** `pad: duck by kick` abre espaço para o bumbo: a cada batida
dele o volume do pad cai 70% e volta em um tempo, o pulsar que o sidechain
dá no estúdio. `pad: duck by kick 50% over 1 eighth` diz quanto e em quanto
tempo. Depois de `by` vem uma parte (`drums`), uma linha (`bd`), ou um
tambor pelo nome (`kick`, `snare`, `clap`, `hat`, ou `bumbo`, `caixa`,
`palmas`, `chimbal`) em qualquer linha que o toque; `all: duck by kick`
abaixa todas as outras partes. Ele move o volume da parte a partir do valor
escrito para ela, e funciona com `at bar`, `from bar … to …` e `every`.

```aratu
drums { bd = b b b b }
pad = | lá3+dó4+mi4 _ _ _ |
pad: duck by kick 60% over 3 sixteenths
```

## 20. Referência rápida

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
| `[mi4 ré]x4`, `[mi4 ré]4x` | um tempo repetido quatro vezes |
| `motif riff = …`, `riff+2`, `riff+3st` | motivo com nome; dois graus acima; três semitons |
| `riff.inv`, `riff.retro`, `riff.ri` | inversão, retrógrado, retrógrado da inversão |
| `row serie = 0 11 3 …`, `\| o o o o \| from serie.RI3` | série dodecafônica; ritmo que recebe as notas de uma forma |
| `list temas { }`, `temas.2`, `temas.all`, `temas.random` | lista de melodias; uma pelo número, todas, sorteio |
| `\| temas.1 \| temas.3 \|` | uma melodia da lista em cada compasso |
| `segno:`, `coda:`, `fine:` | sinais |
| `D.C.`, `D.S. al fine`, `D.S. al coda`, `to coda` | saltos |
| `section nome { }`, `form: intro verso x2 refrao` | seções e forma |
| `every 4 bars: play fill` | variação dentro do loop |
| `play fill once`, `add crash once` | um padrão uma vez, depois volta |
| `at bar 30:` | mudança programada |
| `now:` | mudança no próximo tempo |
| `at bar 30 beat 3:` | mudança num tempo do compasso |
| `from bar 9 to 16:` | só nesses compassos; depois volta o que valia |
| `at bar 9: melodia = …` | a linha muda a partir do compasso 9 |
| `over 6 beats`, `over 8 eighths` | durações em tempos, colcheias, semicolcheias |
| `violão: out`, `violão: in` | silencia uma parte, traz de volta |
| `->` | caminho do sinal |
| `parte.parâmetro = valor` | um parâmetro, desenhado como knob |
| `parte.lowpass.cutoff = 300 Hz -> 4000 Hz over 4 bars` | um parâmetro que se move, uma vez |
| `<->`, `back over 2 beats`, `loop` | vai e volta; outra duração na volta; recomeça |
| `2[dó ré mi]` | três notas em dois tempos |
| `meter: 7/8 = 2+2+3` | compasso aditivo |
| `s!`, `(s)`, `sol4^3` | marcato, nota fantasma, fermata três vezes mais longa |
| `dó5:tr`, `dó5:mord`, `` `ré5 dó5 ``, `` `s `` | trinado, mordente, nota de graça, flam |
| uma linha que começa com `\|` | continua a linha de música de cima |
| `\| I \| vi \| V7/V \|`, `Am:arp` | acordes pelo grau; acorde arpejado |
| `pandeiro = pd td pd td` | percussão brasileira |
| `breakdown`, `build over 8 bars`, `drop` | gestos de palco: sai a base, sobe, volta tudo |
| `solo baixo`, `solo off` | só uma parte toca, e o fim do solo |
| `levada house` | uma bateria pronta, que as linhas escritas mudam |
| `melodia = \| 1 3 5 8 \|`, `3b`, `5,` | melodia pelos graus do tom; acidente; uma oitava abaixo |
| `b(3,8)`, `b(3,8,2)` | ritmo euclidiano; girado dois passos para depois |
| `arp = up 16ths 2 octaves` | um arpejador nos acordes da parte |
| `hh.fast 2`, `baixo.slow 2`, `lead.echo 1 eighth x3` | mais rápido, mais lento, um eco |
| `pad: duck by kick 70% over 1 beat` | o pad abre espaço para o bumbo |
| `reverb.mix = 0.5` (sem parte) | vale para todas as partes |
| `todos: chorus -> delay` | efeitos em todas as partes |
| `master.reverb.decay = 3 s` | a mixagem: reverb, cola, limitador |

## 21. Em português, espanhol e inglês

Toda palavra-chave do Aratu pode ser escrita em inglês, português ou
espanhol, com ou sem acento, em maiúsculas ou minúsculas. As três versões
da mesma peça tocam exatamente igual.

| Inglês | Português | Espanhol |
|---|---|---|
| `meter:` | `compasso:`, `fórmula:` | `compás:` |
| `tempo:` | `andamento:`, `pulso:` | `pulso:` |
| `key:` | `tom:`, `tonalidade:` | `tonalidad:`, `tono:` |
| `pickup: beat 4` | `anacruse: tempo 4` | `anacrusa: tiempo 4` |
| `seed` | `semente` | `semilla` |
| `form:` | `forma:` | `forma:` |
| `at bar 9:` | `no compasso 9:` | `en el compás 9:` |
| `at bar 9 beat 3:` | `no compasso 9 tempo 3:` | `en el compás 9, tiempo 3:` |
| `from bar 9 to 16:` | `do compasso 9 ao 16:` | `desde el compás 9 hasta el 16:` |
| `every 4 bars, first:` | `a cada 4 compassos, primeiro:` | `cada 4 compases, primero:` |
| `now:` | `agora:`, `já:` | `ahora:`, `ya:` |
| `play`, `add`, `once` | `toque`, `adicione`, `uma vez` | `toca`, `añade`, `una vez` |
| `pattern`, `section` | `padrão`, `seção` | `patrón`, `sección` |
| `motif`, `row`, `list` | `motivo`, `série`, `lista` | `motivo`, `serie`, `lista` |
| `temas.all`, `temas.random` | `temas.todas`, `temas.acaso` | `temas.todos`, `temas.azar` |
| `from serie octave 3` | `com serie oitava 3` | `desde serie octava 3` |
| `over 4 bars` | `em 4 compassos` | `en 4 compases` |
| `beats`, `eighths`, `sixteenths` | `tempos`, `colcheias`, `semicolcheias` | `tiempos`, `corcheas`, `semicorcheas` |
| `cresc to ff` | `cresc até ff` | `cresc hasta ff` |
| `back over 2 beats`, `loop` | `volta em 2 tempos`, `repete` | `vuelta en 2 tiempos`, `repite` |
| `all:`, `out`, `in` | `todos:`, `fora`, `dentro` | `todos:`, `fuera`, `dentro` |
| `breakdown`, `build`, `drop` | `quebra`, `subida`, `queda` | `quiebre`, `subida`, `caída` |
| `groove`, `solo off` | `levada`, `solo desliga` | `ritmo`, `solo apagado` |
| `chords`, `strum` | `acordes`, `batida` | `acordes`, `rasgueo` |
| `.transpose`, `.invert` | `.transpor`, `.inverter` | `.transponer`, `.invertir` |
| `.fast`, `.slow`, `.echo` | `.rapido`, `.lento`, `.eco` | `.rapido`, `.lento`, `.eco` |
| `arp = up and down`, `down`, `triplets`, `octaves` | `arpejo = sobe e desce`, `desce`, `tercinas`, `oitavas` | `arpegio = sube y baja`, `baja`, `tresillos`, `octavas` |
| `duck by kick` | `abaixa com o bumbo` | `agacha con el bombo` |
| `drums` | `bateria` | `batería` |

Os nomes das dinâmicas (`mf`, `cresc`), do andamento (`rit`, `a tempo`) e
dos sinais (`D.S. al coda`) são italianos, como na partitura, e valem nas
três línguas. Os sons e os efeitos (`piano.grand`, `lowpass.cutoff`) têm
um nome só.
