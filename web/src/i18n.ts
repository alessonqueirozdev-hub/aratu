// The editor's words, in Portuguese and English.

export type Lang = "pt" | "en";

const words = {
  play: { pt: "Tocar", en: "Play" },
  stop: { pt: "Parar", en: "Stop" },
  examples: { pt: "Exemplos", en: "Examples" },
  messages: { pt: "Mensagens", en: "Messages" },
  allGood: { pt: "Tudo certo. Pode tocar.", en: "All good. Ready to play." },
  keepsPlaying: {
    pt: "Há erros: continua tocando a última versão que estava certa.",
    en: "There are errors: the last version that was right keeps playing.",
  },
  fixFirst: { pt: "Corrija os erros para tocar.", en: "Fix the errors to play." },
  bar: { pt: "Compasso", en: "Bar" },
  tempo: { pt: "Andamento", en: "Tempo" },
  meter: { pt: "Fórmula", en: "Meter" },
  line: { pt: "linha", en: "line" },
  error: { pt: "erro", en: "error" },
  warning: { pt: "aviso", en: "warning" },
  next: { pt: "A mudança entra no compasso", en: "The change comes in at bar" },
  hint: {
    pt: "Ctrl+Enter toca ou atualiza · Ctrl+. para · edite enquanto toca: a mudança entra na próxima barra",
    en: "Ctrl+Enter plays or updates · Ctrl+. stops · edit while it plays: the change comes in at the next bar line",
  },
  ecosystem: { pt: "Parte do ecossistema", en: "Part of the ecosystem" },
  source: { pt: "Código e manual", en: "Code and manual" },
  loading: { pt: "Carregando…", en: "Loading…" },
  tagline: { pt: "live coding que se lê como música", en: "live coding that reads like music" },
  "first-piece": { pt: "Primeira peça", en: "First piece" },
  chance: { pt: "Acaso", en: "Chance" },
  melody: { pt: "Melodia com dinâmica", en: "Melody with dynamics" },
  patterns: { pt: "Padrões ao vivo", en: "Live patterns" },
  form: { pt: "Seções e forma", en: "Sections and form" },
  waltz: { pt: "Valsa com anacruse", en: "Waltz with a pickup" },
} as const;

export type Word = keyof typeof words;

export function tr(word: Word, lang: Lang): string {
  return words[word][lang];
}

export function isWord(w: string): w is Word {
  return w in words;
}
