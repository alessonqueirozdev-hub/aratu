# Silencios y silencios largos

El silencio también es música. En Aratu todo silencio se escribe con el
mismo signo, `-`, y la duración sale de las mismas reglas que los sonidos:
un `-` ocupa una parte del tiempo, como la ocuparía una nota.

| Silencio | Escrito en 4/4 |
|---|---|
| de negra | `-` solo en un tiempo |
| de corchea | `-c` (silencio, luego corchea) o `c-` |
| de semicorchea | `-ccc`, `c-cc` |
| de corchea con puntillo | `-__c` o `-.c` |
| de blanca | `- -` |

Escucha el silencio de corchea al principio de cada tiempo, el
"contratiempo":

```aratu
tempo: 100
hh = -c -c -c -c
bd = b b b b
```

Un `_` después de un silencio lo prolonga: `- _` también es un silencio de
blanca.

## El compás entero de silencio

Un `-` solo entre barras vale el compás entero, en cualquier indicación de
compás, como el silencio de redonda de la partitura. Y `- x4` son cuatro
compases de silencio: la `x` quiere decir "veces".

```aratu
hh = | c c c c |
bd = | - x2 | b - b - |
```

El hi-hat toca desde el principio; el bombo espera dos compases y entra en
el tercero. Como la línea del bombo tiene tres compases, se repite cada
tres.

> Los silencios largos sirven para que un instrumento entre después, como
> en una banda en la que la batería empieza sola.

## Ejercicios

Hi-hat solo en los contratiempos: silencio de corchea y corchea en cada
tiempo.

```aratu exercise
hh = cc cc cc cc
```

```aratu answer
hh = -c -c -c -c
```

El bombo espera **dos compases** de silencio y luego toca en los tiempos 1
y 3.

```aratu exercise
bd = | b - b - |
```

```aratu answer
bd = | - x2 | b - b - |
```
