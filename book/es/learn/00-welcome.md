# Bienvenido a Aratu

Aratu es una manera de escribir música con el teclado de la computadora y
escucharla al instante. Escribes como piensa un músico: en compases,
tiempos, notas y cifrados. Presionas **Escuchar** y la música suena.
Cambias una letra y la música cambia con ella.

Este libro enseña Aratu desde el principio, una idea a la vez. No hace
falta saber programar. Ayuda saber un poco de música, pero aquí también se
aprende música.

## Cómo funciona este libro

- Cada lección explica una idea con **ejemplos** que puedes escuchar y
  cambiar cuanto quieras. Si lo desordenas, presiona **Empezar de nuevo**.
- Debajo de cada ejemplo aparece una **cuadrícula**: los compases, los
  tiempos numerados y cada símbolo en el lugar donde suena, como lo
  dibujaría un profesor en el pizarrón. Se puede tocar: arrastra un símbolo
  a otro lugar, haz clic en un espacio vacío para agregar y en un símbolo
  para quitarlo, y el texto cambia con él.
- Los **ejercicios** te piden escribir algo. **Escuchar cómo debe sonar**
  toca el objetivo; **Comprobar** compara lo que escribiste con el objetivo
  por el sonido y dice, compás por compás, dónde está la diferencia.
- Si algo está mal escrito, Aratu lo explica con palabras de músico, en
  lugar de "error de sintaxis".

Tu progreso se guarda en este navegador, y la lista de lecciones muestra un
✓ en cada lección terminada.

## El primer sonido

Presiona **Escuchar**:

```aratu
hh = c c c c
```

Esto es un compás de hi-hat con cuatro golpes. `hh` es el nombre de la
línea (de *hi-hat*), y cada `c` es un golpe del hi-hat cerrado. El compás
se repite sin parar hasta que presionas **Detener**. En Aratu todo se
repite así, como un loop.

> Cambia una de las `c` por `o` y escucha de nuevo: `o` es el hi-hat
> abierto. Puedes cambiar el texto mientras la música suena; el cambio
> entra al principio del compás siguiente.

## Más de una línea

Cada instrumento de la batería tiene su línea. Escucha las tres juntas:

```aratu
hh = c c c c
bd = b - b -
sd = - s - s
```

`bd` es el bombo (*bass drum*) y `sd` es la caja (*snare drum*). El `-` es
un silencio: en ese tiempo, ese instrumento no toca. Bombo en el 1 y el 3,
caja en el 2 y el 4: es el ritmo más común del rock y del pop.

## Tu primer ejercicio

Escribe una línea de caja que toque en los **cuatro** tiempos. Escucha
primero el objetivo, cambia los silencios y presiona **Comprobar**.

```aratu exercise
sd = - - - -
```

```aratu answer
sd = s s s s
```

En la próxima lección verás qué es un compás para Aratu y cómo elegir el
tempo.

> ¿Programas y quieres saber cómo funciona Aratu por debajo? La pestaña
> [**Por dentro**](#/por-dentro) da nombre técnico a cada cosa: qué es una
> línea, un bloque, un `at bar`.
