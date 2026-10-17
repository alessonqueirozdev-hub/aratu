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
