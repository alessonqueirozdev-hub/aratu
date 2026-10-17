# Percussão brasileira

A bateria tem um símbolo por peça. A percussão brasileira é diferente: cada
instrumento tem uma linha só dele, com o nome do instrumento, e os seus
próprios **golpes**: o polegar no pandeiro, o bacalhau na zabumba, o
triângulo aberto ou fechado. O ritmo segue as mesmas regras de sempre.

## Uma linha com o nome do instrumento

Escreva o nome do instrumento antes do `=`, e use as letras dos golpes dele:

```aratu
tempo: 100
triângulo = fo fo fo fo
```

`f` é o triângulo fechado (a mão segura o ferro) e `o` é o aberto (a mão
solta e ele soa). Esse "fechado, aberto" em colcheias é o coração do baião.

| Linha | Golpes |
|---|---|
| `pandeiro` | `p` polegar, `d` ponta dos dedos, `t` tapa, `m` abafado |
| `zabumba` | `o` aberto, `m` abafado, `b` bacalhau (a vareta por baixo) |
| `triângulo` | `o` aberto, `f` fechado |
| `agogô` | `a` campana aguda, `g` campana grave |
| `surdo` | `o` aberto, `m` abafado |
| `tamborim` | `o` aberto, `m` abafado |
| `alfaia` | `o` aberto, `m` abafado |
| `gonguê` | `o` aberto, `m` abafado |
| `ganzá` | `s` |
| `cuíca` | `a` aguda, `g` grave |
| `cajón` | `b` grave, `t` tapa |

O nome pode ir com ou sem acento (`triangulo`, `agogo`) e também em
espanhol. `agbê`, `caxixi` e `chocalho` tocam como o ganzá.

> Se você escrever uma letra que o instrumento não tem, por exemplo `x` no
> pandeiro, o Aratu diz quais golpes ele tem.

## O baião

O baião junta três instrumentos: a zabumba, o triângulo e o agogô (ou a
sanfona, que fica para depois). A zabumba toca o aberto no tempo 1, o
abafado no fim do tempo 2, e o bacalhau responde por baixo:

```aratu
tempo: 110
zabumba   = | o - -o m | o - b - |
triângulo = fo fo fo fo
agogô     = a g ag -
```

> Mude o andamento para 140 e o baião vira forró pé de serra.

## O samba

No samba, cada instrumento tem um papel. O surdo marca o tempo 2, o
tamborim faz o desenho, o pandeiro e o ganzá enchem as semicolcheias:

```aratu
tempo: 96
meter: 2/4
surdo    = m O
tamborim = o-oo -oo-
pandeiro = pdtd pdtd
ganzá    = Ssss Ssss
```

O samba se conta em dois. A letra maiúscula é um acento, como na bateria:
`O` no surdo é o golpe forte do tempo 2, e `S` no ganzá marca o começo de
cada tempo.

## Com a bateria

As linhas de percussão tocam junto com a bateria e com as outras partes.
Um `surdo` é o surdo do samba; o surdo da bateria continua sendo o `f`,
numa linha de bateria:

```aratu
tempo: 100
drums {
  bd = b - b -
  sd = - s - s
}
agogô  = ag ag a- g-
cajón  = b t bb t
```

## Exercícios

Escreva o triângulo do baião: fechado e aberto em colcheias, nos quatro
tempos.

```aratu exercise
triângulo = - - - -
```

```aratu answer
triângulo = fo fo fo fo
```

O surdo do samba: abafado no tempo 1 e aberto, acentuado, no tempo 2.

```aratu exercise
meter: 2/4
surdo = o o
```

```aratu answer
meter: 2/4
surdo = m O
```

O pandeiro em semicolcheias: polegar, dedos, tapa, dedos, em cada tempo.

```aratu exercise
meter: 2/4
pandeiro = pppp pppp
```

```aratu answer
meter: 2/4
pandeiro = pdtd pdtd
```

Complete o baião: a zabumba com o aberto no tempo 1 e o abafado na segunda
metade do tempo 2, e o bacalhau no tempo 3.

```aratu exercise
zabumba = | - - - - |
```

```aratu answer
zabumba = | o -m b - |
```
