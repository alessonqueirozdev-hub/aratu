# Proyecto: un maracatu

Ahora tienes todo para armar una pieza entera. Vamos a construir, parte por
parte, un ritmo inspirado en el maracatu de baque virado de Recife, en
Brasil, en una versión simplificada: con las alfaias, la caja, el gonguê y
el agbê, y después con forma, dinámica y mezcla.

## 1. El pulso

El maracatu va lento y pesado. Empieza por el tempo y por la alfaia, el
tambor grave, marcando los tiempos 1 y 3:

```aratu
tempo: 92
alfaia = O - O -
```

La mayúscula es el golpe acentuado, abierto y fuerte.

## 2. El dibujo de las alfaias

Las alfaias hacen un dibujo que se repite. Este ocupa un compás, con
corcheas y semicorcheas:

```aratu
tempo: 92
alfaia = O -o o_oo o
```

Léelo tiempo a tiempo: en el 1, el golpe fuerte; en el 2, un silencio de
corchea y una corchea; en el 3, una corchea y dos semicorcheas; en el 4,
una negra.

## 3. La caja

La caja de guerra toca semicorcheas todo el tiempo, con un acento al
principio de cada tiempo. Es una línea de batería común, la caja:

```aratu
tempo: 92
alfaia = O -o o_oo o
caja   = Ssss Ssss Ssss Ssss
```

## 4. El gonguê

El gonguê es la campana de hierro que marca el tiempo de todos. Alterna el
sonido abierto y el apagado:

```aratu
tempo: 92
alfaia = O -o o_oo o
caja   = Ssss Ssss Ssss Ssss
gonguê = o -o -m o
```

## 5. Ejercicio: el agbê

El agbê es la calabaza cubierta de cuentas. Súmalo al ritmo, en corcheas,
con un acento al principio de cada tiempo.

```aratu exercise dynamics=1
tempo: 92
alfaia = O -o o_oo o
caja   = Ssss Ssss Ssss Ssss
gonguê = o -o -m o
```

```aratu answer
tempo: 92
alfaia = O -o o_oo o
caja   = Ssss Ssss Ssss Ssss
gonguê = o -o -m o
agbê   = Ss Ss Ss Ss
```

## 6. La forma: llamada y baque

En el maracatu, el maestro llama y el grupo responde. Haz una **llamada**
solo con el gonguê, y después el **baque**, con todos:

```aratu
tempo: 92
section llamada { gonguê = o -o -m o }
section baque {
  alfaia = O -o o_oo o
  caja   = Ssss Ssss Ssss Ssss
  gonguê = o -o -m o
  agbê   = Ss Ss Ss Ss
}
form: llamada x2 baque x4
```

## 7. Dinámica y mezcla

Para terminar, una sala para que toque el grupo y un crescendo que va
llegando:

```aratu
tempo: 92
section llamada { gonguê = o -o -m o }
section baque {
  alfaia = O -o o_oo o
  caja   = Ssss Ssss Ssss Ssss
  gonguê = o -o -m o
  agbê   = Ss Ss Ss Ss
}
form: llamada x2 baque x4
at bar 3: all: cresc to ff over 4 bars
reverb.mix = 0.35
master.reverb.decay = 2.5 s
alfaia.volume = 1
```

Las alfaias solo existen dentro del `baque`, pero se mezclan desde aquí
afuera, como cualquier parte: `alfaia.volume = 1` las pone al frente donde
sea que toquen.

## 8. Ejercicio final

Arma tu versión: la llamada una sola vez, el baque ocho veces, todo el
grupo en la sala, con `reverb.mix` en 0.35, y las alfaias al frente, con
`volume` en 1.

```aratu exercise sound=1 bars=9
tempo: 92
section llamada { gonguê = o -o -m o }
section baque {
  alfaia = O -o o_oo o
  caja   = Ssss Ssss Ssss Ssss
  gonguê = o -o -m o
}
form: llamada x2 baque x4
```

```aratu answer
tempo: 92
section llamada { gonguê = o -o -m o }
section baque {
  alfaia = O -o o_oo o
  caja   = Ssss Ssss Ssss Ssss
  gonguê = o -o -m o
}
form: llamada baque x8
reverb.mix = 0.35
alfaia.volume = 1
```

## ¿Y ahora?

- Abre la pieza en **Tocar** (botón del ejemplo) y sigue cambiándola:
  cambia el dibujo de la alfaia, agrega un redoble con `pattern` y `every`,
  elige un sonido en el panel **Sonidos**.
- Consulta el **Manual** cuando quieras recordar un símbolo.
- Toca con otras personas: una cambia la alfaia, otra el gonguê, en vivo.

> El maracatu de verdad se aprende con los maestros y las naciones de
> Recife. Este ritmo es solo un punto de partida para escuchar y escribir.
