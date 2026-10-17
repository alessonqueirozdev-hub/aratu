# Ritmo na melodia

As regras de ritmo valem também para as notas, com um cuidado: duas notas
no mesmo tempo vão entre **colchetes**.

```aratu
tempo: 90
melodia = | [dó4 ré] mi [fá sol] lá | sol _ _ _ |
```

`[dó4 ré]` são duas colcheias num tempo. Por que colchetes? Porque `mib`
é mi bemol: sem os colchetes, o Aratu não saberia se `mib` é uma nota ou
mi seguido de si. Pausas e prolongamentos podem ficar colados na nota:
`-mi4` é pausa de colcheia e mi.

## Notas longas e ligaduras

O `_` faz a nota durar mais um tempo (ou mais uma parte, dentro de
colchetes). Um `_` no começo de um compasso liga a nota do compasso
anterior, como uma ligadura:

```aratu
melodia = | dó4 _ mi _ | sol _ _ _ | _ _ mi _ |
```

O sol dura seis tempos: quatro no segundo compasso e mais dois no
terceiro.

## Pontuado e staccato

Dentro dos colchetes, a colcheia pontuada fica `[dó4 _ _ ré]` (o ponto
também vale: `[dó4. ré]`). Um apóstrofo deixa a nota curta, staccato:

```aratu
tempo: 100
melodia = | mi4' mi' fá' sol' | sol fá mi ré |
```

## Várias notas ao mesmo tempo

O `+` junta notas que soam juntas, um acorde:

```aratu
teclado = | dó4+mi4+sol4 _ _ _ | ré4+fá4+lá4 _ si3+ré4+sol4 _ |
```

## A "Ode à Alegria"

O tema de Beethoven, de domínio público, tem um ritmo pontuado no fim:

```aratu
tempo: 110
melodia = | mi4 mi fá sol | sol fá mi ré | dó dó ré mi | mi [_ ré] ré _ |
```

O último compasso: mi dura um tempo e meio (`mi` e a primeira metade de
`[_ ré]`), ré dura meio tempo, e o último ré dura dois tempos.

## Linhas longas

Uma melodia de oito compassos não cabe numa linha da tela. Quebre a linha
onde quiser: uma linha que **começa com uma barra** continua a linha de
música de cima.

```aratu
tempo: 110
melodia = | mi4 mi fá sol | sol fá mi ré |
          | dó dó ré mi  | mi [_ ré] ré _ |
          | mi mi fá sol | sol fá mi ré |
          | dó dó ré mi  | ré [_ dó] dó _ |
```

São oito compassos de uma linha só, a mesma `melodia`. Alinhar as barras
com espaços, como acima, deixa a música fácil de ler, como numa partitura.

## Exercícios

Termine a "Ode à Alegria" com a outra frase, que acaba no dó: no último
compasso, `ré [_ dó] dó _`.

```aratu exercise
tempo: 110
melodia = | mi4 mi fá sol | sol fá mi ré | dó dó ré mi | ré _ _ _ |
```

```aratu answer
tempo: 110
melodia = | mi4 mi fá sol | sol fá mi ré | dó dó ré mi | ré [_ dó] dó _ |
```

Escreva um acorde de dó maior (dó4, mi4, sol4) que dure o compasso
inteiro.

```aratu exercise
teclado = | dó4 _ _ _ |
```

```aratu answer
teclado = | dó4+mi4+sol4 _ _ _ |
```

Colcheias com a primeira de cada tempo em staccato: `[dó4' ré]` nos quatro
tempos.

```aratu exercise
melodia = | [dó4 ré] [dó ré] [dó ré] [dó ré] |
```

```aratu answer
melodia = | [dó4' ré] [dó' ré] [dó' ré] [dó' ré] |
```

Continue a melodia na linha de baixo, com mais dois compassos:
`| fá4 mi ré dó | ré _ _ _ |`.

```aratu exercise
melodia = | dó4 ré mi fá | sol _ _ _ |
```

```aratu answer
melodia = | dó4 ré mi fá | sol _ _ _ |
          | fá4 mi ré dó | ré _ _ _ |
```
