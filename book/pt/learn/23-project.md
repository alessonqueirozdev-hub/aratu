# Projeto: um maracatu

Agora você tem tudo para montar uma peça inteira. Vamos construir, parte
por parte, uma levada inspirada no maracatu de baque virado do Recife,
numa versão simplificada: com as alfaias, a caixa, o gonguê e o agbê, e
depois com forma, dinâmica e mixagem.

## 1. O pulso

O maracatu anda devagar e pesado. Comece pelo andamento e pela alfaia,
o tambor grave, marcando os tempos 1 e 3:

```aratu
tempo: 92
alfaia = O - O -
```

A maiúscula é o golpe acentuado, aberto e forte.

## 2. O desenho das alfaias

As alfaias fazem um desenho que se repete. Este ocupa um compasso, com
colcheias e semicolcheias:

```aratu
tempo: 92
alfaia = O -o o_oo o
```

Leia tempo a tempo: no 1, o golpe forte; no 2, uma pausa de colcheia e uma
colcheia; no 3, colcheia e duas semicolcheias; no 4, uma semínima.

## 3. A caixa

A caixa de guerra toca semicolcheias o tempo todo, com um acento no começo
de cada tempo. É uma linha de bateria comum, a caixa:

```aratu
tempo: 92
alfaia = O -o o_oo o
caixa  = Ssss Ssss Ssss Ssss
```

## 4. O gonguê

O gonguê é o sino de ferro que marca o tempo de todos. Ele alterna o som
aberto e o abafado:

```aratu
tempo: 92
alfaia = O -o o_oo o
caixa  = Ssss Ssss Ssss Ssss
gonguê = o -o -m o
```

## 5. Exercício: o agbê

O agbê é a cabaça coberta de contas. Junte-o à levada, em colcheias, com
um acento no começo de cada tempo.

```aratu exercise dynamics=1
tempo: 92
alfaia = O -o o_oo o
caixa  = Ssss Ssss Ssss Ssss
gonguê = o -o -m o
```

```aratu answer
tempo: 92
alfaia = O -o o_oo o
caixa  = Ssss Ssss Ssss Ssss
gonguê = o -o -m o
agbê   = Ss Ss Ss Ss
```

## 6. A forma: chamada e baque

No maracatu, o mestre chama e o grupo responde. Faça uma **chamada** só
com o gonguê, e depois o **baque**, com todos:

```aratu
tempo: 92
section chamada { gonguê = o -o -m o }
section baque {
  alfaia = O -o o_oo o
  caixa  = Ssss Ssss Ssss Ssss
  gonguê = o -o -m o
  agbê   = Ss Ss Ss Ss
}
form: chamada x2 baque x4
```

## 7. Dinâmica e mixagem

Para terminar, uma sala para o grupo tocar e um crescendo que vai
chegando:

```aratu
tempo: 92
section chamada { gonguê = o -o -m o }
section baque {
  alfaia = O -o o_oo o
  caixa  = Ssss Ssss Ssss Ssss
  gonguê = o -o -m o
  agbê   = Ss Ss Ss Ss
}
form: chamada x2 baque x4
at bar 3: all: cresc to ff over 4 bars
reverb.mix = 0.35
master.reverb.decay = 2.5 s
```

## 8. Exercício final

Monte a sua versão: a chamada uma vez só, o baque oito vezes, e o grupo
todo na sala, com `reverb.mix` em 0.35.

```aratu exercise sound=1 bars=9
tempo: 92
section chamada { gonguê = o -o -m o }
section baque {
  alfaia = O -o o_oo o
  caixa  = Ssss Ssss Ssss Ssss
  gonguê = o -o -m o
}
form: chamada x2 baque x4
```

```aratu answer
tempo: 92
section chamada { gonguê = o -o -m o }
section baque {
  alfaia = O -o o_oo o
  caixa  = Ssss Ssss Ssss Ssss
  gonguê = o -o -m o
}
form: chamada baque x8
reverb.mix = 0.35
```

## E agora?

- Abra a peça em **Tocar** (botão do exemplo) e continue mexendo: troque a
  levada da alfaia, ponha uma virada com `pattern` e `every`, escolha um
  som no painel **Sons**.
- Consulte o **Manual** quando quiser lembrar um símbolo.
- Toque com outras pessoas: um muda a alfaia, outro o gonguê, ao vivo.

> O maracatu de verdade se aprende com os mestres e as nações do Recife.
> Esta levada é só um ponto de partida para ouvir e escrever.
