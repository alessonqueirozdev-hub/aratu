# Harmonia pelo grau

Na lição anterior os acordes tinham nome: Am, F, C, G. Quem estuda
harmonia muitas vezes pensa de outro jeito: não em "lá menor", mas em "o
sexto grau do tom". Escrever pelo grau mostra a função de cada acorde, e a
mesma sequência serve para qualquer tom.

## Os graus do tom

Os graus se escrevem em algarismos romanos, de I a VII, a partir do tom da
peça, que vem em `key:` (ou `tom:`). Em dó maior, I é dó, IV é fá, V é
sol:

```aratu
key: C major
tempo: 90
chords = | I | IV | V | I |
```

**Maiúscula é acorde maior, minúscula é menor.** Em dó maior, o segundo, o
terceiro e o sexto graus são menores: `ii` é Dm, `iii` é Em, `vi` é Am. A
sequência mais tocada do pop é esta:

```aratu
key: C major
tempo: 90
chords = | I | vi | IV | V |
```

> Escreva `key: G major` no lugar de `key: C major` e ouça: a mesma
> sequência, agora em sol. É assim que se muda o tom de uma música para
> caber na voz de quem canta.

O tom também pode ser escrito em português: `tom: Sol maior`,
`tom: Lá menor`.

## Sétimas e outros acordes

Depois do grau vem o resto da cifra, como numa cifra comum: `V7` é o
acorde de sétima da dominante, `ii7` o segundo grau com sétima, `IV7M` o
quarto grau com sétima maior. `vii°` é o acorde diminuto do sétimo grau.

```aratu
key: C major
tempo: 80
chords = | ii7 | V7 | I7M | vii° |
```

O ii, V, I é a cadência mais importante do jazz e da bossa nova.

## Tons menores

Num tom menor os graus mudam: em lá menor, `i` é Am, `iv` é Dm, e `V7` é
E7, a dominante com a sensível:

```aratu
key: A minor
tempo: 90
chords = | i | iv | V7 | i |
```

## Acordes de fora do tom

Um bemol ou sustenido antes do grau pega um acorde emprestado de fora do
tom: `bVII` em dó maior é Bb, o acorde que dá aquele sabor de rock. E uma
barra faz uma dominante secundária: `V7/V` é a dominante da dominante, em
dó maior D7, que leva ao G.

```aratu
key: C major
tempo: 90
chords = | I | bVII | IV | I | I | V7/V | V7 | I |
```

## Arpejar um acorde

Com `:arp` depois do acorde, as notas soam uma depois da outra, de baixo
para cima, como uma harpa ou um violão dedilhado devagar. Vale para
qualquer acorde, com nome ou pelo grau:

```aratu
key: C major
tempo: 70
chords = | I:arp | vi:arp | IV:arp | V7:arp |
```

## O arpejador

`:arp` arpeja um acorde uma vez. Um **arpejador** percorre as notas do
acorde uma de cada vez, em passos, enquanto o acorde durar, como o
arpejador de um sintetizador, ou os dedos de quem toca harpa. Escreva uma
linha chamada `arp` ao lado dos acordes, com o caminho que ela faz e o
tamanho dos passos:

```aratu
key: A minor
tempo: 110
piano {
  chords = | i | VI | III | VII |
  arp    = up 16ths
}
```

Cada semicolcheia toca a próxima nota do acorde que está soando, de baixo
para cima, e cada acorde recomeça da sua nota mais grave. O acorde em si
não soa em bloco: quem o toca é o arpejador, como a batida toca os acordes
do violão. Troque os acordes e ele toca os novos; troque uma palavra do
`arp` e os mesmos acordes soam de outro jeito.

| Escreva | O que toca |
|---|---|
| `arp = up 16ths` | da nota mais grave para cima, em semicolcheias |
| `arp = down 8ths` | da nota mais aguda para baixo, em colcheias |
| `arp = updown 16ths` | sobe e volta descendo (também `up and down`) |
| `arp = random 16ths` | uma das notas do acorde a cada passo, por acaso |
| `arp = up triplets` | três passos por tempo; `beats`, um por tempo |
| `arp = up 16ths 2 octaves` | o acorde espalhado por duas oitavas |

```aratu
tempo: 120
piano {
  chords = | Am | F | C | G |
  arp    = updown 8ths 2 octaves
}
```

> Em português a linha também se escreve `arpejo = sobe 16ths`, `desce`,
> `sobe e desce`, com `colcheias`, `semicolcheias`, `tercinas` e
> `2 oitavas`.

## Melodias pelo grau

Os graus também escrevem melodias. Um número no lugar da nota é um grau
do tom: `1` é a primeira nota do tom, `3` a terça, `5` a quinta, e o `8`
é o `1` uma oitava acima. É como muitos músicos cantam uma melodia para
aprender: "um, três, cinco, três".

```aratu
key: C major
tempo: 100
melodia = | 1 3 5 3 | 4 6 8 _ | 7 5 2 4 | 3 _ 1 _ |
```

> Troque o tom para `key: E minor` sem mexer em mais nada: a mesma
> melodia, agora em mi menor. Em tom menor, os graus seguem a escala
> menor, então a terça já é menor.

Um acidente depois do número sai da escala: `4#` é a quarta aumentada,
`7b` a sétima menor, a nota do blues. E uma vírgula desce uma oitava:
`5,` é a quinta abaixo do `1`, `1,,` duas oitavas abaixo, para o baixo.

```aratu
key: A minor
tempo: 96
melodia = | 1 _ [3 4] 5 | 7b _ 5 _ | 4 3 2 5, | 1 _ _ _ |
baixo   = | 1,, _ _ _ | 4,, _ _ _ | 5,, _ _ _ | 1,, _ _ _ |
```

Os graus fazem tudo o que uma nota faz: colchetes (`[1 2]`), acordes
(`1+3+5`), `_`, staccato, fermata, `x4` e quiálteras.

## Com a batida do violão

Os graus funcionam numa linha de cifras de qualquer instrumento, com batida
e tudo:

```aratu
key: D major
tempo: 96
violão {
  chords = | I | V | vi | IV |
  strum  = D du -u du
}
```

## Exercícios

Escreva I, IV, V, I em dó maior, um acorde por compasso.

```aratu exercise
key: C major
chords = | I | I | I | I |
```

```aratu answer
key: C major
chords = | I | IV | V | I |
```

A mesma sequência pelo grau, mas agora em sol maior. Mude só o tom.

```aratu exercise
key: C major
chords = | I | vi | IV | V |
```

```aratu answer
key: G major
chords = | I | vi | IV | V |
```

A cadência ii, V, I em dó maior: `ii7`, `V7` e `I7M`, e o último dura dois
compassos.

```aratu exercise
key: C major
chords = | I | I | I | I |
```

```aratu answer
key: C major
chords = | ii7 | V7 | I7M | % |
```

Em lá menor: i, iv, V7, i, com o último acorde arpejado.

```aratu exercise
key: A minor
chords = | i | i | i | i |
```

```aratu answer
key: A minor
chords = | i | iv | V7 | i:arp |
```

Faça o arpejador descer em vez de subir, ainda em colcheias.

```aratu exercise
chords = | Am | F | C | G |
arp    = up 8ths
```

```aratu answer
chords = | Am | F | C | G |
arp    = down 8ths
```

Escreva pelo grau, em dó maior: 1, 2, 3 e 5, uma semínima cada.

```aratu exercise
key: C major
melodia = | 1 _ _ _ |
```

```aratu answer
key: C major
melodia = | 1 2 3 5 |
```
