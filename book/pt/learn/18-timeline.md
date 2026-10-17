# Mudanças no lugar certo

Na lição anterior a música mudava quando você mexia no texto. Muitas vezes,
porém, a mudança já está decidida antes: a virada no compasso 8, a bateria
que sai no refrão, o baixo que muda na segunda parte. Esta lição mostra como
marcar cada mudança no compasso, e até no tempo, em que ela deve acontecer.

## No compasso: `at bar`

Uma linha que começa com `at bar 9:` (ou `no compasso 9:`) acontece no
compasso 9. Sem isso, ela vale desde o começo da peça.

```aratu
tempo: 110
pattern groove { hh = cc cc cc cc; bd = b - b -; sd = - s - s }
pattern fill   { sd = ss ss SS SS; bd = b - - - }
play groove
at bar 4: play fill
at bar 5: play groove
```

O groove toca nos compassos 1, 2 e 3; a virada no 4; o groove volta no 5.

## Uma vez só: `once`

Voltar ao groove depois da virada é tão comum que tem um atalho. Com `once`
no fim, o padrão toca uma vez e o que estava antes volta sozinho:

```aratu
tempo: 110
pattern groove { hh = cc cc cc cc; bd = b - b -; sd = - s - s }
pattern fill   { sd = ss ss SS SS; bd = b - - - }
play groove
at bar 4: play fill once
```

Tocando ao vivo, `now: play fill once` (ou `agora:`) traz a virada já no
próximo tempo, e o groove volta no compasso em que estaria.

## Num tempo do compasso: `beat`

Uma virada muitas vezes começa no meio do compasso. `at bar 4 beat 3:`
(ou `no compasso 4 tempo 3:`) entra no terceiro tempo do compasso 4:

```aratu
tempo: 110
pattern groove { hh = cc cc cc cc; bd = b - b -; sd = - s - s }
pattern fill   { sd = ss ss SS SS; bd = b - - - }
play groove
at bar 4 beat 3: play fill once
```

Ouça: os dois primeiros tempos do compasso 4 ainda são do groove, e a
virada toma os dois últimos. `beat 2.5` cai na metade do caminho entre o
tempo 2 e o 3.

## Por um trecho: `from bar … to …`

`from bar 5 to 6:` (ou `do compasso 5 ao 6:`) vale só nesses compassos. No
compasso 7 volta o que valia antes, sem você escrever nada:

```aratu
tempo: 110
drums { hh = cc cc cc cc; bd = b - b -; sd = - s - s }
baixo = | dó2 _ sol1 _ |
from bar 3 to 4: drums: out
```

A bateria sai nos compassos 3 e 4 e volta no 5. Serve para tudo:
dinâmica, andamento, padrões, partes que entram e saem.

## Uma linha que muda num compasso

Também a própria música de uma linha pode mudar num compasso. A linha nova
começa do começo dela, no compasso marcado:

```aratu
tempo: 100
baixo = | dó2 _ sol1 _ | lá1 _ mi1 _ |
hh = cc cc cc cc
at bar 5: baixo = | fá1 _ fá2 _ | sol1 _ sol2 _ |
```

Nos compassos 1 a 4 o baixo toca a primeira linha, duas vezes; do 5 em
diante, a segunda. É assim que se escreve uma música com partes diferentes
sem precisar de seções.

## Durações em tempos

Toda mudança que leva um tempo (`cresc`, `rit`, `accel`, rampas de
andamento) aceita durações em compassos (`bars`), tempos (`beats`),
colcheias (`eighths`) ou semicolcheias (`sixteenths`):

```aratu
tempo: 90
melodia = | dó4 ré mi fá | sol _ _ _ |
at bar 2: melodia: cresc to ff over 3 beats
```

Em português: `em 3 tempos`, `em 8 colcheias`.

## Exercícios

Faça a virada tocar uma vez, no compasso 3, e o groove voltar sozinho.

```aratu exercise bars=4
pattern groove { hh = cc cc cc cc }
pattern fill   { sd = ss ss SS SS }
play groove
```

```aratu answer
pattern groove { hh = cc cc cc cc }
pattern fill   { sd = ss ss SS SS }
play groove
at bar 3: play fill once
```

Agora a virada começa no tempo 3 do compasso 2.

```aratu exercise bars=3
pattern groove { hh = cc cc cc cc }
pattern fill   { sd = ss ss SS SS }
play groove
at bar 2: play fill once
```

```aratu answer
pattern groove { hh = cc cc cc cc }
pattern fill   { sd = ss ss SS SS }
play groove
at bar 2 beat 3: play fill once
```

Tire a bateria só nos compassos 2 e 3, com uma linha só.

```aratu exercise bars=4
drums { hh = c c c c }
baixo = | dó2 _ _ _ |
```

```aratu answer
drums { hh = c c c c }
baixo = | dó2 _ _ _ |
from bar 2 to 3: drums: out
```

A partir do compasso 3, o baixo passa a tocar `| fá2 _ dó2 _ |`.

```aratu exercise bars=4
baixo = | dó2 _ sol1 _ |
```

```aratu answer
baixo = | dó2 _ sol1 _ |
at bar 3: baixo = | fá2 _ dó2 _ |
```
