# Cifras e batidas de violão

Uma sequência de acordes se escreve como numa cifra, numa linha chamada
`chords`. Cada compasso entre barras tem o seu acorde:

```aratu
tempo: 90
chords = | Am | F | C | G |
```

Sozinha, a linha de cifras toca os acordes inteiros, como um teclado.

## Acordes que dividem o compasso

Dois acordes no mesmo compasso dividem o compasso ao meio. Para outras
divisões, use `_` para dizer quanto dura cada acorde:

```aratu
chords = | C G | Am _ _ F | C |
```

No segundo compasso, Am dura três tempos e F um.

As cifras são escritas do jeito de sempre, incluindo o jeito brasileiro:
`Am`, `G7`, `C7M`, `Dm7`, `Bº`, `E/G#`, `G7(b9)`.

## A batida do violão

Uma linha chamada `strum` diz como a mão bate nas cordas, com as mesmas
regras de ritmo:

| Símbolo | Batida |
|---|---|
| `d` | para baixo |
| `u` | para cima |
| `m` | abafada |
| `g` | golpe: os dedos batem no tampo, sem tocar as cordas |
| `-` | a mão passa sem tocar |
| maiúscula | acentuada |

Ponha as duas linhas num bloco com o nome do instrumento:

```aratu
tempo: 90
violão {
  chords = | Am | F | C | G |
  strum  = D du -u du
}
```

A batida se repete em cada compasso, e os acordes trocam sozinhos. Na
batida para baixo as cordas soam da grave para a aguda, na para cima da
aguda para a grave, como num violão de verdade.

> Mude a batida para `D -u D -u` e ouça a diferença, com a música tocando.

## O golpe no tampo

No flamenco a mão também bate no tampo do violão, o *golpe*. Ele se
escreve `g`, e `G` é um golpe acentuado. As cordas não soam no golpe;
soa a madeira:

```aratu
tempo: 100
violão {
  chords = | Am | G | F | E |
  strum  = dgdu dgdu dGdu dgdu
}
```

> Cada tempo tem quatro semicolcheias: para baixo, golpe, para baixo, para
> cima. O golpe do terceiro tempo é acentuado.

## Exercícios

Escreva a sequência C, Am, F, G, um acorde por compasso.

```aratu exercise
violão {
  chords = | C | C | C | C |
  strum  = d d d d
}
```

```aratu answer
violão {
  chords = | C | Am | F | G |
  strum  = d d d d
}
```

Agora mude a batida para `D du D du`.

```aratu exercise
violão {
  chords = | Am | Dm | E7 | Am |
  strum  = d d d d
}
```

```aratu answer
violão {
  chords = | Am | Dm | E7 | Am |
  strum  = D du D du
}
```
