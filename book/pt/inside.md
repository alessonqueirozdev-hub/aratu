# Por dentro do Aratu

Esta parte é para quem programa, ou para quem quer saber como o Aratu
funciona por baixo. Ela dá nome técnico às coisas que o curso e o manual
explicam com palavras de música: o que é uma linha, um bloco, um `at bar`, um
motivo, e o que o computador faz com o texto até ele virar som.

Não é preciso nada disso para tocar. Cada termo técnico vem com uma
explicação simples ao lado, então dá para ler sem saber programar.

## 1. Que tipo de linguagem é

O Aratu é uma linguagem **declarativa**: o texto descreve o que soa, e não
uma lista de ordens a cumprir uma depois da outra. É a mesma família do HTML,
do CSS ou de uma planilha. Você não escreve "toque esta nota, espere, toque a
próxima"; escreve como a música é, e o Aratu calcula quando cada som
acontece.

Algumas consequências:

- **A ordem das linhas quase não importa.** Um motivo pode ser usado antes de
  ser definido, um padrão pode ser tocado antes de ser escrito. O Aratu lê o
  texto todo antes de tocar (em programação, isso se chama *hoisting*).
- **Não há variáveis que mudam, laços nem `if`.** A repetição já está no
  tempo: toda linha volta ao começo sozinha. A escolha é `{dó|sol}`. As
  condições são de tempo: `at bar 9:`, `every 4 bars:`.
- **O texto é a única fonte da verdade.** Cada edição faz o Aratu ler o texto
  inteiro de novo. O que o tocador guarda entre uma versão e outra é pouco:
  em que compasso cada linha escrita ao vivo começou a soar.

```aratu
tempo: 100
melodia = | riff riff+1 riff+2 riff |
motif riff = [mi4 ré dó ré]
```

Aqui o motivo está escrito depois da linha que o usa, e funciona.

## 2. O léxico: as palavras da linguagem

O léxico é o conjunto de "palavras" (em inglês, *tokens*) que o Aratu
reconhece. Cada linha é uma **declaração** (*statement*). Dentro de chaves,
`;` separa declarações na mesma linha, e `//` começa um comentário até o fim
da linha.

| Tipo | Exemplos | O que é |
|---|---|---|
| nome (identificador) | `melodia`, `bass`, `riff`, `temas` | letras, números e `_`, começando por letra |
| caminho | `bass.reverb.mix`, `drums.hh`, `riff.inv` | nomes ligados por pontos |
| palavra-chave | `tempo`, `meter`, `key`, `pattern`, `play`, `at`, `every`, `now`, `over`, `motif`, `row`, `list`, `from` | palavras reservadas, com sentido fixo |
| número | `120`, `0.42`, `-1`, `1.2` | inteiros e decimais, com ponto |
| unidade | `Hz`, `kHz`, `ms`, `s`, `%` | logo depois de um número |
| operador | `=`, `:`, `->`, `<->`, `.`, `,` | ligam as partes de uma declaração |
| música | `dó4`, `c4`, `fá#`, `mib`, `Am`, `c`, `o`, `_`, `-`, `[ ]`, `{ \| }`, `\|` | os símbolos dentro de uma linha de música |

Os símbolos de música formam uma pequena linguagem dentro da linguagem: um
**literal** de música, como uma string ou um número são literais em outras
linguagens. O espaço separa tempos, `|` separa compassos, `[ ]` agrupa num
tempo, `{ | }` sorteia, `_` sustenta, `-` é pausa, `+` junta notas num
acorde, maiúscula é acento, `'` é staccato, `^` é fermata, `?` é acaso e
`x4` repete.

## 3. A gramática: os tipos de declaração

A gramática diz como as palavras se juntam. Cada declaração tem uma destas
formas (o que está entre `⟨ ⟩` é um lugar a preencher):

| Forma | Exemplo | Para que serve |
|---|---|---|
| `⟨quadro⟩: ⟨valor⟩` | `tempo: 96`, `meter: 3/4`, `key: A minor` | o quadro da peça |
| `⟨nome⟩ = ⟨música⟩` | `melodia = \| dó4 ré mi fá \|` | uma linha de música com nome |
| `⟨nome⟩ { … }` | `drums { hh = c c c c }` | um bloco de linhas de uma parte |
| `⟨parte⟩: ⟨som⟩ -> ⟨efeito⟩ …` | `bass: synth.saw -> lowpass` | a cadeia de sinal de uma parte |
| `⟨caminho⟩ = ⟨valor⟩` | `bass.reverb.mix = 0.42` | um parâmetro |
| `⟨caminho⟩ = ⟨valor⟩ -> ⟨valor⟩ over ⟨duração⟩` | `bass.lowpass.cutoff = 300 Hz -> 4 kHz over 4 bars` | um parâmetro que se move |
| `⟨parte⟩: ⟨ordem⟩` | `piano: mf`, `guitar: out`, `piano: cresc to ff over 4 bars` | uma ordem para uma parte |
| `pattern ⟨nome⟩ { … }`, `section ⟨nome⟩ { … }` | `pattern fill { sd = ss ss SS SS }` | um grupo de linhas com nome |
| `play …`, `add …`, `form: …` | `play groove x3, fill` | o que toca, e em que ordem |
| `motif`, `row`, `list` | `motif riff = [mi4 ré]` | definições usadas por outras linhas |
| `⟨quando⟩: ⟨declaração⟩` | `at bar 9: play fill`, `every 4 bars: …`, `now: …` | a mesma declaração, num momento |

O `⟨quando⟩` vem na frente de quase qualquer declaração. Sem ele, ela vale do
começo da peça, ou, se a música já está tocando, do próximo compasso.

## 4. Dando nome às coisas

Esta tabela é o coração desta parte: para cada coisa do Aratu, o nome que ela
teria em programação, e com o que se parece.

| No Aratu | Em termos de programação | Parecido com |
|---|---|---|
| `melodia = \| dó4 ré mi fá \|` | uma **ligação** (*binding*) de um nome a um valor; o valor é uma sequência de compassos | uma constante: não muda enquanto toca; cada edição a define de novo |
| uma linha | uma **sequência circular**: ao acabar, volta ao começo | um clipe em loop numa DAW |
| `drums { hh = …; bd = … }` | um **registro** ou *namespace*: um mapa de nomes para linhas | um objeto (`drums.hh`) |
| `bass.reverb.mix = 0.42` | **atribuição de propriedade** num caminho de um **mapa aninhado** (parte → efeito → parâmetro) | `bass.reverb.mix = 0.42` em JavaScript, ou uma regra de CSS |
| `bass: synth.saw -> lowpass -> reverb` | um **pipeline**: a saída de cada um é a entrada do próximo | composição de funções, `reverb(lowpass(saw))`, ou o `\|` do terminal |
| `synth.saw`, `piano.grand` | uma **chave de um catálogo** (um enum): um nome que escolhe um som pronto | um valor de uma lista fixa; não é um método |
| `motif riff = [mi4 ré dó ré]` | uma **macro**: o nome é trocado pelo conteúdo onde aparece | uma função sem efeitos colaterais, expandida no lugar (*inlining*) |
| `riff+2`, `riff+3st` | o motivo com um **argumento**: a transposição | `riff(2)` |
| `riff.inv`, `riff.retro`, `serie.P5` | uma **transformação pura** escrita com ponto: devolve um motivo novo e não muda o original | um método ou propriedade calculada sem parênteses |
| `row serie = 0 11 3 4 …` | um **vetor** (*array*) de inteiros de 0 a 11 | `[0, 11, 3, 4, …]` |
| `\| o o o o \| from serie.P0` | um **zip** do ritmo com o vetor, que recomeça quando acaba; cada `o` é um **marcador** a preencher | `zip(ritmo, cycle(serie))` |
| `list temas { … }` | um **array** de melodias, contado a partir de 1 | `temas[1]` em JavaScript é `temas.2` no Aratu |
| `temas.all`, `temas.random` | concatenação; escolha aleatória de um item | `temas.flat()`, `temas[random()]` |
| `{dó\|sol}` | **escolha aleatória**, sorteada a cada volta | `random.choice` |
| `c?30%` | um evento com **probabilidade** de 30% | `if (random() < 0.3)` |
| `seed 7` | a **semente** do gerador pseudoaleatório: a mesma semente dá o mesmo resultado | `Random(7)` |
| `pattern groove { … }` | um **bloco nomeado** de linhas | um componente, uma cena, um template |
| `play groove x3, fill` | uma **playlist**: uma sequência com repetições, em loop | uma lista de cenas |
| `\|: :\|`, `%`, `D.S. al coda` | **saltos** da partitura, resolvidos antes de tocar | um `goto` que o compilador desenrola |
| `[mi4 ré]x4`, `riffx4` | **repetição desenrolada**: vira quatro cópias | *loop unrolling* |
| `p:`, `mf:` | um **estado** que vale para as notas seguintes da linha | a "cor atual" numa biblioteca de desenho |
| `300 Hz -> 4000 Hz over 4 bars` | **interpolação** de um valor no tempo (em escala logarítmica para frequências) | *tween*, automação por keyframes |
| um knob | **ligação nos dois sentidos**: girar reescreve o número do texto | *two-way binding*; o texto continua sendo a fonte da verdade |

### `at bar` é um switch?

Não exatamente. Um `switch` (ou `case`) escolhe **um** caminho entre vários,
cada vez que é executado, conforme um valor. Um `at bar` é uma **regra
indexada pelo tempo**: a partir do compasso 9, aquilo vale, e continua
valendo até outra regra mudar a mesma coisa. Se parece mais com um
**keyframe** numa animação, ou com um evento marcado numa linha do tempo.

```aratu
tempo: 110
bass = | dó2 _ sol1 _ |
at bar 5: bass.reverb.mix = 0.6
at bar 9: bass: out
```

Para saber o estado no compasso 12, o Aratu junta as regras de todos os
compassos até ali: o reverb está em 0.6 (desde o 5) e o baixo está fora
(desde o 9). Em programação, isso é uma **dobra** (*fold*) das mudanças, em
ordem de tempo.

Já o `every` é o mais perto de um `if` com o número do compasso:

```aratu
tempo: 110
pattern groove { hh = cc cc cc cc; bd = b - b -; sd = - s - s }
pattern fill { sd = ss ss SS SS }
play groove
every 4 bars: play fill
```

`every 4 bars: play fill` quer dizer, compasso a compasso, "se este é o
último de cada quatro, toque a virada no lugar": `if (compasso % 4 == 0)`. E
`every 4 bars, first:` é `if (compasso % 4 == 1)`.

## 5. Do texto ao som

Quando você edita, o texto passa por estas etapas, cada uma num módulo do
compilador (escrito em Rust e levado ao navegador como WebAssembly):

1. **Léxico.** O texto é cortado em linhas, blocos e palavras.
2. **Pré-leitura.** Motivos, séries, listas e o tom são recolhidos primeiro,
   para que uma linha possa usá-los de qualquer lugar do texto.
3. **Análise sintática** (*parse*). Cada linha vira uma declaração numa
   **árvore de sintaxe** (*AST*), com o lugar exato onde cada pedaço está
   escrito, para as mensagens e para acender as notas.
4. **Modelo.** As declarações viram partes, linhas, padrões, seções e
   ordens, com as conferências: se cada compasso cabe na fórmula, se os
   nomes existem.
5. **Desenrolar.** Ritornelos, saltos, `%` e `x4` viram uma sequência
   simples de compassos.
6. **Dinâmica e tempo.** Cada nota recebe a sua força, e as posições viram
   segundos pelo mapa de andamento. As posições são **frações exatas**: uma
   tercina é 1/3 de tempo, sem arredondar.
7. **Agendamento** (*scheduling*). Para os compassos pedidos, saem os
   **eventos**: cada nota, cada batida, cada mudança de parâmetro, com o
   instante em segundos.

O agendamento é **determinístico**: o mesmo texto, a mesma semente e os
mesmos compassos de partida dão sempre os mesmos eventos. Por isso o tocador
pode pedir um compasso de cada vez, um pouco antes de ele soar, e uma
edição pode entrar no compasso seguinte sem que nada recomece.

Os eventos vão para o motor de áudio. Os sons gravados e as baterias tocam
pelo motor do Manguebyte (um AudioWorklet que roda WebAssembly); os outros,
pelo motor provisório feito com o Web Audio do navegador.

## 6. Os dados: o que sai do compilador

A IDE conversa com o compilador por poucas funções exportadas pelo
WebAssembly (`aratu_compile`, `aratu_render`, `aratu_check`, `aratu_grid`,
`aratu_voices`), trocando texto em **JSON**. Compilar devolve as mensagens,
as partes, os knobs, as listas, as séries e as linhas escritas ao vivo.
Pedir compassos devolve os eventos. Um evento de nota se parece com isto:

```json
{"bar": 3, "beat": 1.5, "time": 4.75, "dur": 0.24, "part": "bass",
 "line": "bass", "vel": 80, "note": 45, "from": 120, "to": 123}
```

| Campo | O que é |
|---|---|
| `bar`, `beat` | o compasso e o tempo dentro dele, como escrito |
| `time`, `dur` | quando soa e quanto dura, em segundos desde o começo |
| `part`, `line` | a parte e a linha que o tocam |
| `vel` | a força, de 0 a 127, como no MIDI |
| `note` | a nota, como número MIDI (60 é o dó central) |
| `drum` | no lugar de `note`, a peça da bateria: `kick`, `snare`, `closed-hat`… |
| `acc` | 1 quando a nota foi escrita em maiúscula |
| `param`, `value`, `glide` | no lugar da nota, uma mudança de parâmetro; `glide` são os segundos em que o som desliza até o valor |
| `from`, `to` | onde o símbolo está escrito no texto, para acendê-lo quando soa |

As mensagens também são dados: cada uma tem a gravidade (erro ou aviso), o
trecho do texto e o texto na língua escolhida. Um erro não para a música:
enquanto o texto tiver erro, continua tocando a última versão certa, como um
*hot reload* que recusa uma versão quebrada.
