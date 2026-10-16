// The editor's words, in Portuguese, Spanish and English.

export type Lang = "pt" | "es" | "en";

export const LANGS: Lang[] = ["pt", "es", "en"];

const words = {
  // Navigation.
  navPlay: { pt: "Tocar", es: "Tocar", en: "Play" },
  navLearn: { pt: "Aprender", es: "Aprender", en: "Learn" },
  navManual: { pt: "Manual", es: "Manual", en: "Manual" },
  tagline: {
    pt: "música escrita para tocar e mudar enquanto toca",
    es: "música escrita para tocar y cambiar mientras suena",
    en: "music written to play, and to change while it plays",
  },
  light: { pt: "Claro", es: "Claro", en: "Light" },
  dark: { pt: "Escuro", es: "Oscuro", en: "Dark" },
  theme: { pt: "Aparência", es: "Apariencia", en: "Appearance" },
  rhythmMap: { pt: "Mapa do ritmo", es: "Mapa del ritmo", en: "Rhythm map" },
  rhythmHelp: {
    pt: "Cada linha é uma parte, cada caixa um compasso, os números são os tempos e as listras finas as subdivisões. Arraste uma batida para movê-la, clique num espaço vazio para acrescentar, clique numa batida para tirar: o código muda junto.",
    es: "Cada fila es una parte, cada caja un compás, los números son los tiempos y las rayas finas las subdivisiones. Arrastra un golpe para moverlo, haz clic en un espacio vacío para agregar, haz clic en un golpe para quitarlo: el código cambia con él.",
    en: "Each row is a part, each box a bar, the numbers are the beats and the thin stripes the subdivisions. Drag a hit to move it, click an empty place to add one, click a hit to take it away: the code changes with it.",
  },
  grid: { pt: "Grade", es: "Cuadrícula", en: "Grid" },
  gridAuto: { pt: "auto", es: "auto", en: "auto" },
  eighths: { pt: "colcheias", es: "corcheas", en: "eighths" },
  sixteenths: { pt: "semicolcheias", es: "semicorcheas", en: "sixteenths" },
  triplets: { pt: "tercinas", es: "tresillos", en: "triplets" },

  // The playground.
  play: { pt: "Tocar", es: "Tocar", en: "Play" },
  stop: { pt: "Parar", es: "Detener", en: "Stop" },
  stage: { pt: "Palco", es: "Escenario", en: "Stage" },
  stageExit: { pt: "Sair", es: "Salir", en: "Exit" },
  stageSplit: { pt: "2 colunas", es: "2 columnas", en: "2 columns" },
  examples: { pt: "Exemplos", es: "Ejemplos", en: "Examples" },
  messages: { pt: "Mensagens", es: "Mensajes", en: "Messages" },
  allGood: { pt: "Tudo certo. Pode tocar.", es: "Todo bien. Puedes tocar.", en: "All good. Ready to play." },
  keepsPlaying: {
    pt: "Há erros: continua tocando a última versão que estava certa.",
    es: "Hay errores: sigue sonando la última versión que estaba bien.",
    en: "There are errors: the last version that was right keeps playing.",
  },
  fixFirst: { pt: "Corrija os erros para tocar.", es: "Corrige los errores para tocar.", en: "Fix the errors to play." },
  bar: { pt: "Compasso", es: "Compás", en: "Bar" },
  tempo: { pt: "Andamento", es: "Tempo", en: "Tempo" },
  meter: { pt: "Fórmula", es: "Compás", en: "Meter" },
  line: { pt: "linha", es: "línea", en: "line" },
  error: { pt: "erro", es: "error", en: "error" },
  warning: { pt: "aviso", es: "aviso", en: "warning" },
  next: { pt: "A mudança entra no compasso", es: "El cambio entra en el compás", en: "The change comes in at bar" },
  beat: { pt: "tempo", es: "tiempo", en: "beat" },
  hint: {
    pt: "Ctrl+Enter toca ou atualiza · Ctrl+. para · mude o texto enquanto toca: a mudança entra na próxima barra · Palco (Ctrl+Shift+F): só o código, em tela cheia; Esc sai",
    es: "Ctrl+Enter toca o actualiza · Ctrl+. detiene · cambia el texto mientras suena: el cambio entra en la próxima barra · Escenario (Ctrl+Shift+F): solo el código, en pantalla completa; Esc sale",
    en: "Ctrl+Enter plays or updates · Ctrl+. stops · change the text while it plays: the change comes in at the next bar line · Stage (Ctrl+Shift+F): only the code, full screen; Esc leaves",
  },
  ecosystem: { pt: "Parte do ecossistema", es: "Parte del ecosistema", en: "Part of the ecosystem" },
  source: { pt: "Código-fonte", es: "Código fuente", en: "Source code" },
  loading: { pt: "Carregando…", es: "Cargando…", en: "Loading…" },

  // The Sounds panel.
  sounds: { pt: "Sons", es: "Sonidos", en: "Sounds" },
  soundsHelp: {
    pt: "Cada som tem um código. Escreva-o depois do nome da parte, como em baixo: bass.slap, ou clique no código para colocá-lo onde está o cursor. ▶ toca um compasso dele.",
    es: "Cada sonido tiene un código. Escríbelo después del nombre de la parte, como en bajo: bass.slap, o haz clic en el código para ponerlo donde está el cursor. ▶ toca un compás.",
    en: "Each sound has a code. Write it after the part's name, as in bass: bass.slap, or click the code to put it where the cursor is. ▶ plays a bar of it.",
  },
  soundSearch: { pt: "Buscar um som", es: "Buscar un sonido", en: "Find a sound" },
  soundNone: { pt: "Nenhum som com esse nome.", es: "Ningún sonido con ese nombre.", en: "No sound by that name." },
  soundHear: { pt: "Ouvir", es: "Escuchar", en: "Listen" },
  soundUse: { pt: "Usar este som", es: "Usar este sonido", en: "Use this sound" },
  soundSampled: { pt: "gravado", es: "grabado", en: "recorded" },
  soundLoading: { pt: "baixando…", es: "descargando…", en: "loading…" },
  soundReady: { pt: "pronto", es: "listo", en: "ready" },
  mbSwitch: {
    pt: "Tocar amostras e baterias pelo motor do Manguebyte",
    es: "Tocar muestras y baterías con el motor de Manguebyte",
    en: "Play samples and drum kits with Manguebyte's engine",
  },
  mbHelp: {
    pt: "As vozes marcadas Manguebyte tocam pelo motor do Manguebyte; as outras, pelo motor provisório do navegador, até o Manguebyte tê-las. Desligado, tudo toca no provisório. A página recarrega ao trocar.",
    es: "Las voces marcadas Manguebyte suenan con el motor de Manguebyte; las demás, con el motor provisional del navegador, hasta que Manguebyte las tenga. Apagado, todo suena con el provisional. La página se recarga al cambiar.",
    en: "Voices marked Manguebyte play through Manguebyte's engine; the others through the browser's stand-in until Manguebyte has them. Off, everything plays in the stand-in. The page reloads when it changes.",
  },
  soundFailed: { pt: "não baixou", es: "no se descargó", en: "did not load" },

  // Twelve-tone rows and melody lists.
  rows: { pt: "Séries", es: "Series", en: "Rows" },
  rowsHelp: {
    pt: "A matriz de cada série da peça. Cada linha é uma forma P, lida da esquerda para a direita, ou R, da direita para a esquerda; cada coluna é uma forma I, de cima para baixo, ou RI, de baixo para cima. Clique num nome para escrevê-lo no código.",
    es: "La matriz de cada serie de la pieza. Cada fila es una forma P, leída de izquierda a derecha, o R, de derecha a izquierda; cada columna es una forma I, de arriba abajo, o RI, de abajo arriba. Haz clic en un nombre para escribirlo en el código.",
    en: "The matrix of each row in the piece. Each line is a P form, read left to right, or R, right to left; each column is an I form, top to bottom, or RI, bottom to top. Click a name to write it into the code.",
  },
  rowsNone: {
    pt: "Esta peça não tem nenhuma série. Escreva, por exemplo: row serie = 0 11 3 4 8 7 9 5 6 1 2 10",
    es: "Esta pieza no tiene ninguna serie. Escribe, por ejemplo: row serie = 0 11 3 4 8 7 9 5 6 1 2 10",
    en: "This piece has no row. Write, for example: row serie = 0 11 3 4 8 7 9 5 6 1 2 10",
  },
  rowsNumbers: { pt: "Ver números", es: "Ver números", en: "Show numbers" },
  rowsNames: { pt: "Ver notas", es: "Ver notas", en: "Show notes" },
  listAll: { pt: "todas", es: "todas", en: "all" },
  listRandom: { pt: "sorteio", es: "azar", en: "random" },
  listHint: {
    pt: "Alt+1 a Alt+9 escolhem a melodia da lista",
    es: "Alt+1 a Alt+9 eligen la melodía de la lista",
    en: "Alt+1 to Alt+9 choose the list's melody",
  },

  // Lessons and the manual.
  listen: { pt: "Ouvir", es: "Escuchar", en: "Listen" },
  listenAnswer: { pt: "Ouvir como deve soar", es: "Escuchar cómo debe sonar", en: "Hear how it should sound" },
  check: { pt: "Conferir", es: "Comprobar", en: "Check" },
  restart: { pt: "Recomeçar", es: "Empezar de nuevo", en: "Start over" },
  showAnswer: { pt: "Ver uma resposta", es: "Ver una respuesta", en: "See an answer" },
  useAnswer: { pt: "Usar esta resposta", es: "Usar esta respuesta", en: "Use this answer" },
  exercise: { pt: "Exercício", es: "Ejercicio", en: "Exercise" },
  example: { pt: "Experimente", es: "Prueba", en: "Try it" },
  right: {
    pt: "Muito bem! Soa exatamente como pedido.",
    es: "¡Muy bien! Suena exactamente como se pedía.",
    en: "Well done! It sounds exactly as asked.",
  },
  almost: { pt: "Quase. Ouça de novo e compare:", es: "Casi. Escucha otra vez y compara:", en: "Almost. Listen again and compare:" },
  andMore: { pt: "e mais", es: "y", en: "and" },
  moreProblems: { pt: "diferenças", es: "diferencias más", en: "more differences" },
  doesNotRead: {
    pt: "O Aratu ainda não consegue ler o seu código; veja a mensagem acima.",
    es: "Aratu todavía no puede leer tu código; mira el mensaje de arriba.",
    en: "Aratu cannot read your code yet; see the message above.",
  },
  previous: { pt: "Anterior", es: "Anterior", en: "Previous" },
  following: { pt: "Próxima", es: "Siguiente", en: "Next" },
  lessons: { pt: "Lições", es: "Lecciones", en: "Lessons" },
  chapters: { pt: "Capítulos", es: "Capítulos", en: "Chapters" },
  doneCount: { pt: "exercícios feitos", es: "ejercicios hechos", en: "exercises done" },
  openInPlay: { pt: "Abrir em Tocar", es: "Abrir en Tocar", en: "Open in Play" },

  // Example pieces.
  "first-piece": { pt: "Primeira peça", es: "Primera pieza", en: "First piece" },
  chance: { pt: "Acaso", es: "Azar", en: "Chance" },
  melody: { pt: "Melodia com dinâmica", es: "Melodía con dinámica", en: "Melody with dynamics" },
  patterns: { pt: "Padrões ao vivo", es: "Patrones en vivo", en: "Live patterns" },
  form: { pt: "Seções e forma", es: "Secciones y forma", en: "Sections and form" },
  waltz: { pt: "Valsa com anacruse", es: "Vals con anacrusa", en: "Waltz with a pickup" },
  sound: { pt: "Sons e knobs", es: "Sonidos y perillas", en: "Sounds and knobs" },
  motifs: { pt: "Motivos e sequências", es: "Motivos y secuencias", en: "Motifs and sequences" },
  "twelve-tone": { pt: "Série dodecafônica", es: "Serie dodecafónica", en: "Twelve-tone row" },
  setlist: { pt: "Lista de melodias no palco", es: "Lista de melodías en el escenario", en: "A melody list on stage" },
} as const;

export type Word = keyof typeof words;

export function tr(word: Word, lang: Lang): string {
  return words[word][lang];
}

export function isWord(w: string): w is Word {
  return w in words;
}
