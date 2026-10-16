# A bateria

Cada peça da bateria tem um símbolo curto. O símbolo decide o som, seja
qual for o nome da linha.

| Símbolo | Som |
|---|---|
| `c` | chimbal fechado |
| `o` | chimbal aberto |
| `b` | bumbo |
| `s` | caixa |
| `r` | prato de condução |
| `x` | prato de ataque |
| `t` | tom agudo |
| `m` | tom médio |
| `f` | surdo |
| `k` | aro (baqueta atravessada na caixa) |

Cada linha também tem um **nome**, escrito antes do `=`. O costume é usar
as abreviações do inglês, as mesmas dos estúdios e das baterias
eletrônicas:

| Nome da linha | Vem de | Peça |
|---|---|---|
| `hh` | *hi-hat* | chimbal |
| `bd` | *bass drum* | bumbo |
| `sd` | *snare drum* | caixa |
| `rd` | *ride* | prato de condução |
| `cr` | *crash* | prato de ataque |
| `tom` | *tom* | tons e surdo |
| `rim` | *rim* | aro |

O nome é só um rótulo, para você se achar; quem decide o som é o símbolo.
Uma linha `chimbal = cc cc cc cc` dentro de `drums { }` soa igual.

As linhas podem ficar juntas num bloco `drums { }`. Assim fica claro que
são todas da bateria:

```aratu
tempo: 100

drums {
  hh = cc cc cc co
  bd = b - bb -
  sd = - s - s
}
```

O chimbal abre na última colcheia (`co`) e o bumbo dobra no tempo 3
(`bb`).

## Acento

Uma letra **maiúscula** é tocada com acento, mais forte. Ouça a caixa
acentuada no tempo 4:

```aratu
drums {
  hh = cc cc cc cc
  bd = b - b -
  sd = - s - S
}
```

E um apóstrofo depois do símbolo deixa o som curto: `c'` é um chimbal bem
seco.

## Um prato para começar

O prato de ataque marca o começo de uma frase. Aqui ele toca no tempo 1 e
a condução faz as colcheias:

```aratu
drums {
  cr = x - - -
  rd = rr rr rr rr
  bd = b - b -
  sd = - s - s
}
```

> A linha `cr` (*crash*) leva o prato de ataque, `x`, e a linha `rd`
> (*ride*) leva a condução, `r`. Troque um `r` por `c` e ouça a diferença
> entre a condução e o chimbal.

## Tons e aro

Os tons fazem as viradas: `t` é o tom agudo, `m` o tom médio e `f` o
surdo. Aqui o segundo compasso termina descendo pelos três:

```aratu
tempo: 96

drums {
  hh  = | cc cc cc cc | cc cc -  -  |
  bd  = | b  -  bb -  | b  -  -  -  |
  sd  = | -  s  -  s  | -  s  -  -  |
  tom = | -  -  -  -  | -  -  tm ff |
}
```

O aro, `k`, é a baqueta atravessada sobre a caixa: um som seco, de
madeira, sem a esteira. É a batida da bossa nova:

```aratu
tempo: 130

drums {
  hh  = cc cc cc cc
  bd  = b -b b -b
  rim = | k- -k - k | - k -k - |
}
```

## Exercícios

Uma batida de rock com a caixa **acentuada** no tempo 4.

```aratu exercise dynamics=1
drums {
  bd = b - b -
  sd = - s - s
}
```

```aratu answer
drums {
  bd = b - b -
  sd = - s - S
}
```

Abra o chimbal na última colcheia do compasso.

```aratu exercise
hh = cc cc cc cc
```

```aratu answer
hh = cc cc cc co
```

Termine o compasso com uma virada: no tempo 4, uma colcheia de tom agudo
e uma de surdo.

```aratu exercise
drums {
  sd  = s s s -
  tom = - - - -
}
```

```aratu answer
drums {
  sd  = s s s -
  tom = - - - tf
}
```

Agora monte uma batida completa: chimbal em colcheias, bumbo nos tempos 1
e 3 e na segunda metade do tempo 3, caixa nos tempos 2 e 4.

```aratu exercise
drums {
  hh = - - - -
  bd = - - - -
  sd = - - - -
}
```

```aratu answer
drums {
  hh = cc cc cc cc
  bd = b - bb -
  sd = - s - s
}
```
