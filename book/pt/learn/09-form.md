# Repetições e saltos

Os sinais de repetição da partitura também se escrevem no Aratu.

## Ritornelo

`|:` e `:|` cercam um trecho que se toca duas vezes:

```aratu
tempo: 110
melodia = |: dó4 mi sol mi | ré fá lá fá :| dó4 _ _ _ |
```

Ouça: os dois primeiros compassos duas vezes, depois o final. Para tocar
mais vezes, escreva quantas depois do `:|`: `:| x4`.

## Casas de primeira e segunda vez

`|1` começa a primeira casa e `:|2` a segunda:

```aratu
chords = |: C | F |1 G | G :|2 C | C |
```

Na primeira vez: C, F, G, G. Na segunda: C, F, C, C.

## Repetir o compasso anterior

`%` repete o compasso anterior; `%%` repete os dois anteriores:

```aratu
chords = | Am | % | F | % |
```

## D.C., D.S., fine e coda

`D.C.` volta ao começo e `D.S.` volta ao sinal `segno:`. `al fine` para no
compasso marcado com `fine:`:

```aratu
tempo: 110
melodia = | dó4 mi sol dó5 | segno: ré4 fá lá _ | fine: dó4 _ _ _ | sol4 _ _ _ D.S. al fine |
```

Ordem de execução: compassos 1, 2, 3, 4, de volta ao 2, 3, e fim.

> Se um `D.S.` não tiver `segno:`, o Aratu diz exatamente isso, com o
> número do compasso.

## Exercícios

Repita os dois primeiros compassos e termine no terceiro.

```aratu exercise
melodia = | dó4 mi sol mi | ré fá lá fá | dó4 _ _ _ |
```

```aratu answer
melodia = |: dó4 mi sol mi | ré fá lá fá :| dó4 _ _ _ |
```

Use `%` para que o Am dure dois compassos e o F também, nesta ordem: Am,
Am, F, F.

```aratu exercise
chords = | Am | F |
```

```aratu answer
chords = | Am | % | F | % |
```
