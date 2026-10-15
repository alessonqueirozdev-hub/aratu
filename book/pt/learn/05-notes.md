# Notas e melodias

Até aqui só havia bateria. Agora, notas. Uma nota se escreve com o nome e a
oitava: `dó4` é o dó central do piano. Os nomes podem ser em solfejo ou em
letras, como nas cifras:

| Solfejo | dó | ré | mi | fá | sol | lá | si |
|---|---|---|---|---|---|---|---|
| Letra | c | d | e | f | g | a | b |

Ouça a escala de dó maior subindo:

```aratu
tempo: 100
melodia = | dó4 ré4 mi4 fá4 | sol4 lá4 si4 dó5 |
```

Uma linha que não é de bateria toca com um som de sintetizador. Use o nome
que quiser para ela: `melodia`, `flauta`, `voz`.

## A oitava continua até mudar

Não é preciso repetir a oitava em cada nota: ela vale até aparecer outra.
A escala acima pode ser escrita assim:

```aratu
melodia = | dó4 ré mi fá | sol lá si dó5 |
```

## Sustenidos e bemóis

O sustenido é `#` e o bemol é `b`, logo depois do nome: `fá#4`, `sib3`,
`eb4` (mi bemol).

```aratu
melodia = | lá3 si dó#4 ré | mi fá# sol# lá |
```

Essa é a escala de lá maior.

## Uma melodia conhecida

"Brilha, brilha, estrelinha" é uma melodia antiga, de domínio público.
Ouça a primeira frase:

```aratu
tempo: 100
melodia = | dó4 dó sol sol | lá lá sol _ |
```

O `_` no fim segura o sol por mais um tempo: uma mínima.

## Exercícios

Complete a frase: depois dos dois compassos, vêm `fá fá mi mi | ré ré dó _`.

```aratu exercise
tempo: 100
melodia = | dó4 dó sol sol | lá lá sol _ |
```

```aratu answer
tempo: 100
melodia = | dó4 dó sol sol | lá lá sol _ | fá fá mi mi | ré ré dó _ |
```

Escreva a escala de dó maior **descendo**, do dó5 ao dó4, em dois
compassos.

```aratu exercise
melodia = | dó5 _ _ _ | dó4 _ _ _ |
```

```aratu answer
melodia = | dó5 si4 lá sol | fá mi ré dó |
```
