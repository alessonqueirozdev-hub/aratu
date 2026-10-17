# Proyecto: un maracatu

Ahora tienes todo para armar una pieza. Vamos a construir, parte por parte,
un ritmo inspirado en el maracatu de baque virado de Recife, en Brasil, en
una versión simplificada para la batería de Aratu.

## 1. El pulso

El maracatu va lento y pesado. Empieza por el tempo y el bombo, que hace el
papel de las alfaias graves (los tambores grandes):

```aratu
tempo: 92
drums {
  bd = b - b -
}
```

## 2. Las alfaias

Las alfaias hacen un dibujo que se repite. Este ocupa un compás, con
corcheas y semicorcheas:

```aratu
tempo: 92
drums {
  bd = b -b b_bb b
}
```

## 3. Las cajas

Las cajas tocan semicorcheas todo el tiempo, con acentos:

```aratu
tempo: 92
drums {
  bd = b -b b_bb b
  sd = Ssss Ssss Ssss Ssss
}
```

## 4. El gonguê

El gonguê es la campana de hierro que marca el tiempo de todos. El ride
(`r`) hace su papel aquí:

```aratu
tempo: 92
drums {
  bd = b -b b_bb b
  sd = Ssss Ssss Ssss Ssss
  rd = r -r -r r
}
```

## 5. Ejercicio final

Junta todo y agrega el agbê (el hi-hat cerrado en corcheas) y un bajo que
toque do2 y sol1, una blanca cada uno, en un compás.

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
bajo = | do2 _ sol1 _ |
```

## ¿Y ahora?

- Abre la pieza en **Tocar** (botón del ejemplo) y sigue cambiándola: cambia
  el bajo, agrega un redoble con `pattern` y `every`, haz un crescendo.
- Consulta el **Manual** cuando quieras recordar un símbolo.
- Toca con otras personas: una cambia la batería, otra el bajo, en vivo.

> El maracatu de verdad se aprende con los maestros y las naciones de
> Recife. Este ritmo es solo un punto de partida para escuchar y escribir.
