# Projeto: um maracatu

Agora você tem tudo para montar uma peça. Vamos construir, parte por
parte, uma levada inspirada no maracatu de baque virado do Recife, numa
versão simplificada para a bateria do Aratu.

## 1. O pulso

O maracatu anda devagar e pesado. Comece pelo andamento e pelo bumbo,
fazendo o papel das alfaias graves:

```aratu
tempo: 92
drums {
  bd = b - b -
}
```

## 2. As alfaias

As alfaias fazem um desenho que se repete. Este ocupa um compasso, com
colcheias e semicolcheias:

```aratu
tempo: 92
drums {
  bd = b -b b_bb b
}
```

## 3. As caixas

As caixas tocam semicolcheias o tempo todo, com acentos:

```aratu
tempo: 92
drums {
  bd = b -b b_bb b
  sd = Ssss Ssss Ssss Ssss
}
```

## 4. O gonguê

O gonguê é o sino de ferro que marca o tempo de todos. A condução (`r`)
faz o papel dele aqui:

```aratu
tempo: 92
drums {
  bd = b -b b_bb b
  sd = Ssss Ssss Ssss Ssss
  rd = r -r -r r
}
```

## 5. Exercício final

Junte tudo e acrescente o agbê (o chimbal fechado em colcheias) e um
baixo que toca dó2 e sol1, uma mínima cada, num compasso.

```aratu exercise
tempo: 92
drums {
  bd = b -b b_bb b
  sd = Ssss Ssss Ssss Ssss
  rd = r -r -r r
}
```

```aratu answer
tempo: 92
drums {
  bd = b -b b_bb b
  sd = Ssss Ssss Ssss Ssss
  rd = r -r -r r
  hh = cc cc cc cc
}
baixo = | dó2 _ sol1 _ |
```

## E agora?

- Abra a peça em **Tocar** (botão do exemplo) e continue mexendo: troque o
  baixo, ponha uma virada com `pattern` e `every`, faça um crescendo.
- Consulte o **Manual** quando quiser lembrar um símbolo.
- Toque com outras pessoas: um muda a bateria, outro o baixo, ao vivo.

> O maracatu de verdade se aprende com os mestres e as nações do Recife.
> Esta levada é só um ponto de partida para ouvir e escrever.
