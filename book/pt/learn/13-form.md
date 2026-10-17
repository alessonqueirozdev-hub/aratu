# Seções e forma

Uma canção tem partes: introdução, verso, refrão, final. Cada parte volta
mais de uma vez, e a ordem delas é a **forma** da música. No Aratu você
escreve cada parte uma vez, com nome, e depois diz em que ordem elas tocam.

## Uma seção

Uma seção é um bloco com nome, escrito com `section` (ou `seção`). Dentro
dela vão as partes e as linhas, como numa peça inteira:

```aratu
tempo: 100
section verso {
  baixo = | lá2 _ mi2 _ | fá2 _ dó3 _ |
  drums { hh = cc cc co cc; bd = b - b -; sd = - s - s }
}
form: verso
```

Um bloco pode ficar numa linha só, com `;` entre as linhas, como a bateria
acima. A seção dura o tanto da sua linha mais longa: aqui, dois compassos.

## A forma

A linha `form:` (ou `forma:`) diz a ordem das seções. `x2` toca uma seção
duas vezes seguidas:

```aratu
tempo: 100
section intro  { baixo = | lá2 _ _ _ | mi2 _ _ _ | }
section verso  {
  baixo = | lá2 _ mi2 _ | fá2 _ dó3 _ |
  drums { hh = cc cc co cc; bd = b - b -; sd = - s - s }
}
section refrao {
  violão { chords = | Am | F | C | G |; strum = D du -u du }
  drums  { hh = cc cc cc cc; bd = b - b -; sd = - s - S }
}
section final  { baixo = | lá2 _ _ _ | }

form: intro verso x2 refrao verso refrao final
```

Ouça a ordem: introdução, verso duas vezes, refrão, verso, refrão, final. Ao
chegar ao fim, a forma recomeça, como todo loop do Aratu.

> Os nomes das seções são seus: `intro`, `a`, `b`, `ponte`, `solo`.
> Escreva sem acento e sem espaço.

## Seções e repetições juntas

Dentro de uma seção valem os ritornelos, as casas e o `%` da lição de
repetições. E a forma pode repetir um trecho maior:

```aratu
tempo: 110
section a { melodia = | dó4 mi sol mi | ré fá lá fá | }
section b { melodia = | mi4 sol si sol | dó5 _ _ _ | }
form: a x2 b a
```

É a forma AABA, a de muitas canções do jazz e da bossa nova.

## Exercícios

Ponha as seções na ordem: introdução, verso, refrão.

```aratu exercise bars=3
section intro  { baixo = | lá2 _ _ _ | }
section verso  { baixo = | dó3 _ _ _ | }
section refrao { baixo = | fá2 _ _ _ | }
form: intro
```

```aratu answer
section intro  { baixo = | lá2 _ _ _ | }
section verso  { baixo = | dó3 _ _ _ | }
section refrao { baixo = | fá2 _ _ _ | }
form: intro verso refrao
```

Agora a forma AABA: a seção `a` duas vezes, depois `b`, depois `a`.

```aratu exercise bars=4
section a { melodia = | dó4 _ _ _ | }
section b { melodia = | sol4 _ _ _ | }
form: a b
```

```aratu answer
section a { melodia = | dó4 _ _ _ | }
section b { melodia = | sol4 _ _ _ | }
form: a x2 b a
```

Escreva a seção `refrao`, com o baixo tocando fá2 e sol2, uma mínima cada,
para a forma que já está pronta.

```aratu exercise bars=2
section verso  { baixo = | lá2 _ _ _ | }
section refrao { baixo = | - | }
form: verso refrao
```

```aratu answer
section verso  { baixo = | lá2 _ _ _ | }
section refrao { baixo = | fá2 _ sol2 _ | }
form: verso refrao
```
