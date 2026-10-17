# Andamento: rit, accel, fermata e anacruse

## Mudanças de andamento

O andamento pode mudar num compasso marcado com `at bar`:

```aratu
tempo: 100
at bar 3: rit to 60 over 2 bars
hh = c c c c
```

`rit to 60 over 2 bars` é um ritardando até 60, ao longo de dois
compassos (`rall` também vale). O contrário é `accel to 140 over 4 bars`.
`a tempo` volta ao andamento de antes do rit, e `tempo primo` volta ao
primeiro andamento da peça.

Durante um ritardando o andamento muda aos poucos, tempo a tempo, como se
lê numa partitura, e cada nota cai no momento exato.

## Fermata

`^` depois de uma nota é uma fermata: a nota dura o dobro, e todas as
partes esperam por ela.

```aratu
tempo: 90
melodia = | dó4 mi sol dó5^ | sol4 _ _ _ |
hh = c c c c
```

## Anacruse

Muitas melodias começam antes do primeiro tempo. `pickup: beat 4` diz que
a peça começa no tempo 4 de um compasso incompleto:

```aratu
tempo: 100
pickup: beat 4
melodia = | sol3 | dó4 _ mi _ | sol _ _ _ |
bd = | b - b - |
```

A anacruse é o compasso 0, e o compasso 1 é o primeiro completo, como os
músicos contam. Uma linha cujo primeiro compasso tem o tamanho da anacruse
começa nela; as outras começam no compasso 1.

## Exercícios

Ponha uma fermata na última nota do primeiro compasso.

```aratu exercise
melodia = | dó4 ré mi fá | sol _ _ _ |
```

```aratu answer
melodia = | dó4 ré mi fá^ | sol _ _ _ |
```

Faça esta melodia começar com a anacruse `sol3` no tempo 4.

```aratu exercise
melodia = | dó4 _ mi _ | sol _ _ _ |
```

```aratu answer
pickup: beat 4
melodia = | sol3 | dó4 _ mi _ | sol _ _ _ |
```
