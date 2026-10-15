//! Diagnostics in a musician's words, in Portuguese and English.
//!
//! A diagnostic carries what went wrong as data ([`Message`]); the words are
//! produced on demand in the reader's language, so the editor can switch
//! language without compiling again and tests can match on the data.

use crate::ratio::Ratio;
use crate::span::Span;

/// The language messages are written in.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Lang {
    #[default]
    Pt,
    En,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Severity {
    /// The piece cannot be played as written.
    Error,
    /// The piece plays, but probably not as meant.
    Warning,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Diagnostic {
    pub severity: Severity,
    pub span: Span,
    pub message: Message,
}

impl Diagnostic {
    pub fn error(span: Span, message: Message) -> Diagnostic {
        Diagnostic {
            severity: Severity::Error,
            span,
            message,
        }
    }

    pub fn warning(span: Span, message: Message) -> Diagnostic {
        Diagnostic {
            severity: Severity::Warning,
            span,
            message,
        }
    }

    pub fn text(&self, lang: Lang) -> String {
        self.message.text(lang)
    }

    pub fn is_error(&self) -> bool {
        self.severity == Severity::Error
    }
}

/// What a statement expected to find, named for the message.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Thing {
    Number,
    Meter,
    Key,
    Name,
    BarNumber,
    Dynamic,
    Colon,
    Bars,
    Beat,
    Value,
}

impl Thing {
    fn words(self, lang: Lang) -> &'static str {
        use Lang::*;
        use Thing::*;
        match (self, lang) {
            (Number, En) => "a number",
            (Number, Pt) => "um número",
            (Meter, En) => "a meter such as 4/4",
            (Meter, Pt) => "uma fórmula de compasso como 4/4",
            (Key, En) => "a key such as A minor",
            (Key, Pt) => "um tom como A minor",
            (Name, En) => "a name",
            (Name, Pt) => "um nome",
            (BarNumber, En) => "a bar number",
            (BarNumber, Pt) => "um número de compasso",
            (Dynamic, En) => "a dynamic mark (ppp pp p mp mf f ff fff)",
            (Dynamic, Pt) => "uma dinâmica (ppp pp p mp mf f ff fff)",
            (Colon, En) => "a colon",
            (Colon, Pt) => "dois-pontos",
            (Bars, En) => "a length such as `over 4 bars`",
            (Bars, Pt) => "uma duração como `over 4 bars`",
            (Beat, En) => "a beat such as `beat 4`",
            (Beat, Pt) => "um tempo como `beat 4`",
            (Value, En) => "a value",
            (Value, Pt) => "um valor",
        }
    }
}

/// A kind of change in the frame of the piece.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Change {
    Tempo,
    Meter,
    Key,
}

impl Change {
    fn words(self, lang: Lang) -> &'static str {
        match (self, lang) {
            (Change::Tempo, Lang::En) => "tempo",
            (Change::Tempo, Lang::Pt) => "andamento",
            (Change::Meter, Lang::En) => "meter",
            (Change::Meter, Lang::Pt) => "fórmula de compasso",
            (Change::Key, Lang::En) => "key",
            (Change::Key, Lang::Pt) => "tom",
        }
    }
}

/// Everything Aratu can say about a piece.
#[derive(Clone, Debug, PartialEq)]
pub enum Message {
    // Layout of blocks.
    UnclosedBlock {
        name: String,
    },
    StrayClose,

    // Statements.
    UnknownLine,
    Expected {
        what: Thing,
        after: String,
    },
    Unexpected {
        text: String,
    },
    BadMeter {
        text: String,
    },
    TempoOutOfRange {
        bpm: f64,
    },
    UnknownKey {
        text: String,
    },
    DuplicateHeader {
        name: String,
    },
    BadPickup {
        beat: i64,
        beats: i64,
    },
    BarZero,

    // Music lines.
    UnclosedBracket {
        open: char,
    },
    UnexpectedChar {
        ch: char,
    },
    UnknownNote {
        text: String,
    },
    GluedNotes {
        first: String,
        second: String,
    },
    UnknownDrum {
        ch: char,
    },
    UnknownStrum {
        ch: char,
    },
    UnknownChord {
        text: String,
    },
    UnknownMark {
        word: String,
    },
    ChanceRange {
        percent: f64,
    },
    DynamicBeforeNothing,
    EmptyBrackets,
    EmptyChoice,
    MisplacedMarker {
        marker: String,
    },
    MisplacedJump {
        jump: String,
    },
    MisplacedRepeatCount,
    EmptyBar,
    BarRepeatNotAlone,
    NothingToRepeat,
    HoldWithNothing,
    NoOctave {
        note: String,
    },

    // Checks on the whole piece.
    BeatCount {
        line: String,
        bar: usize,
        found: Ratio,
        expected: Ratio,
        meter: String,
    },
    PickupBeats {
        line: String,
        found: Ratio,
        expected: Ratio,
    },
    ChordsOffBeat {
        line: String,
        bar: usize,
        chords: usize,
        beats: Ratio,
    },
    JumpWithoutMarker {
        line: String,
        jump: String,
        bar: usize,
        marker: String,
    },
    RepeatsNeverEnd {
        line: String,
    },
    StrumWithoutChords {
        part: String,
    },
    UnknownPattern {
        name: String,
    },
    UnknownTarget {
        name: String,
    },
    UnknownSection {
        name: String,
    },
    UnknownParam {
        name: String,
    },
    ParamOutOfRange {
        name: String,
        value: f64,
        min: f64,
        max: f64,
    },
    HairpinWithoutTarget {
        line: String,
    },
    DuplicateLine {
        name: String,
    },
    SameBarChange {
        bar: i64,
        what: Change,
    },
    BadUnit {
        unit: String,
        name: String,
    },
}

fn beats_en(n: Ratio) -> String {
    if n == Ratio::ONE {
        "1 beat".into()
    } else {
        format!("{n} beats")
    }
}

fn beats_pt(n: Ratio) -> String {
    if n == Ratio::ONE {
        "1 tempo".into()
    } else {
        format!("{n} tempos")
    }
}

impl Message {
    /// The message in words, in the given language.
    pub fn text(&self, lang: Lang) -> String {
        use Lang::*;
        use Message::*;
        match (self, lang) {
            (UnclosedBlock { name }, En) => {
                format!("the block `{name}` opens with {{ but is never closed with }}")
            }
            (UnclosedBlock { name }, Pt) => {
                format!("o bloco `{name}` abre com {{ mas nunca fecha com }}")
            }
            (StrayClose, En) => "this } closes nothing".into(),
            (StrayClose, Pt) => "este } não fecha nada".into(),

            (UnknownLine, En) => "I don't understand this line".into(),
            (UnknownLine, Pt) => "não entendi esta linha".into(),
            (Expected { what, after }, En) => {
                format!("{} is missing after `{after}`", what.words(En))
            }
            (Expected { what, after }, Pt) => {
                format!("falta {} depois de `{after}`", what.words(Pt))
            }
            (Unexpected { text }, En) => format!("`{text}` does not belong here"),
            (Unexpected { text }, Pt) => format!("`{text}` não cabe aqui"),
            (BadMeter { text }, En) => format!(
                "`{text}` is not a meter: write two numbers like 4/4 or 6/8, the lower one 1, 2, 4, 8, 16 or 32"
            ),
            (BadMeter { text }, Pt) => format!(
                "`{text}` não é uma fórmula de compasso: escreva dois números como 4/4 ou 6/8, o de baixo 1, 2, 4, 8, 16 ou 32"
            ),
            (TempoOutOfRange { bpm }, En) => {
                format!("a tempo of {bpm} is outside 10 to 400 beats per minute")
            }
            (TempoOutOfRange { bpm }, Pt) => {
                format!("o andamento {bpm} está fora de 10 a 400 batidas por minuto")
            }
            (UnknownKey { text }, En) => {
                format!("`{text}` is not a key; write it like `A minor` or `F# major`")
            }
            (UnknownKey { text }, Pt) => {
                format!("`{text}` não é um tom; escreva como `A minor` ou `F# major`")
            }
            (DuplicateHeader { name }, En) => {
                format!("`{name}:` is set twice; the last one counts")
            }
            (DuplicateHeader { name }, Pt) => {
                format!("`{name}:` aparece duas vezes; vale a última")
            }
            (BadPickup { beat, beats }, En) => {
                format!("a bar of {beats} beats has no beat {beat} to start on")
            }
            (BadPickup { beat, beats }, Pt) => {
                format!("um compasso de {beats} tempos não tem tempo {beat} para começar")
            }
            (BarZero, En) => "bars are counted from 1".into(),
            (BarZero, Pt) => "os compassos contam a partir de 1".into(),

            (UnclosedBracket { open }, En) => format!("this {open} is never closed"),
            (UnclosedBracket { open }, Pt) => format!("este {open} nunca fecha"),
            (UnexpectedChar { ch }, En) => format!("`{ch}` has no meaning here"),
            (UnexpectedChar { ch }, Pt) => format!("`{ch}` não tem significado aqui"),
            (UnknownNote { text }, En) => format!(
                "`{text}` is not a note: notes are c d e f g a b, or dó ré mi fá sol lá si, with an octave such as c4"
            ),
            (UnknownNote { text }, Pt) => format!(
                "`{text}` não é uma nota: as notas são dó ré mi fá sol lá si, ou c d e f g a b, com a oitava, como dó4"
            ),
            (GluedNotes { first, second }, En) => format!(
                "two notes are written together in one beat; put them in brackets: `[{first} {second}]`"
            ),
            (GluedNotes { first, second }, Pt) => format!(
                "duas notas estão coladas num tempo; escreva entre colchetes: `[{first} {second}]`"
            ),
            (UnknownDrum { ch }, En) => format!(
                "`{ch}` is not a drum symbol: c closed hi-hat, o open hi-hat, b bass drum, s snare, r ride, x crash"
            ),
            (UnknownDrum { ch }, Pt) => format!(
                "`{ch}` não é um símbolo de bateria: c chimbal fechado, o chimbal aberto, b bumbo, s caixa, r condução, x ataque"
            ),
            (UnknownStrum { ch }, En) => format!(
                "`{ch}` is not a strum symbol: d down, u up, m muted, - the hand passes, _ let it ring"
            ),
            (UnknownStrum { ch }, Pt) => format!(
                "`{ch}` não é um símbolo de batida: d para baixo, u para cima, m abafado, - a mão passa, _ deixa soar"
            ),
            (UnknownChord { text }, En) => format!("`{text}` is not a chord symbol I know"),
            (UnknownChord { text }, Pt) => format!("não conheço a cifra `{text}`"),
            (UnknownMark { word }, En) => format!(
                "`{word}:` is neither a dynamic (ppp pp p mp mf f ff fff sfz fp) nor a marker (segno: fine: coda:)"
            ),
            (UnknownMark { word }, Pt) => format!(
                "`{word}:` não é uma dinâmica (ppp pp p mp mf f ff fff sfz fp) nem um sinal (segno: fine: coda:)"
            ),
            (ChanceRange { percent }, En) => {
                format!("a chance of {percent}% is not between 0% and 100%")
            }
            (ChanceRange { percent }, Pt) => {
                format!("uma chance de {percent}% não fica entre 0% e 100%")
            }
            (DynamicBeforeNothing, En) => "a dynamic mark goes right before a note".into(),
            (DynamicBeforeNothing, Pt) => "a marca de dinâmica vai logo antes de uma nota".into(),
            (EmptyBrackets, En) => "these brackets are empty".into(),
            (EmptyBrackets, Pt) => "estes colchetes estão vazios".into(),
            (EmptyChoice, En) => "one of the choices between { } is empty".into(),
            (EmptyChoice, Pt) => "uma das escolhas entre { } está vazia".into(),
            (MisplacedMarker { marker }, En) => {
                format!("`{marker}:` goes at the start of a bar")
            }
            (MisplacedMarker { marker }, Pt) => {
                format!("`{marker}:` vai no começo de um compasso")
            }
            (MisplacedJump { jump }, En) => {
                format!("`{jump}` goes at the end of a bar, just before the bar line")
            }
            (MisplacedJump { jump }, Pt) => {
                format!("`{jump}` vai no fim de um compasso, logo antes da barra")
            }
            (MisplacedRepeatCount, En) => "a repeat count such as `x4` goes right after :|".into(),
            (MisplacedRepeatCount, Pt) => {
                "o número de vezes, como `x4`, vai logo depois de :|".into()
            }
            (EmptyBar, En) => "this bar is empty; write rests in it, like `- - - -`".into(),
            (EmptyBar, Pt) => "este compasso está vazio; escreva pausas, como `- - - -`".into(),
            (BarRepeatNotAlone, En) => {
                "`%` stands for a whole bar and goes alone between bar lines".into()
            }
            (BarRepeatNotAlone, Pt) => {
                "`%` vale um compasso inteiro e fica sozinho entre as barras".into()
            }
            (NothingToRepeat, En) => "there is no bar before this `%` to repeat".into(),
            (NothingToRepeat, Pt) => "não há compasso antes deste `%` para repetir".into(),
            (HoldWithNothing, En) => "this `_` has no note before it to hold".into(),
            (HoldWithNothing, Pt) => "este `_` não tem nota antes para sustentar".into(),
            (NoOctave { note }, En) => format!(
                "`{note}` has no octave and no note before it gives one, so it is taken as {note}4"
            ),
            (NoOctave { note }, Pt) => {
                format!("`{note}` está sem oitava e nenhuma nota antes dá uma, então vale {note}4")
            }

            (
                BeatCount {
                    line,
                    bar,
                    found,
                    expected: _,
                    meter,
                },
                En,
            ) => format!(
                "bar {bar} of `{line}` has {}, but the meter is {meter}",
                beats_en(*found)
            ),
            (
                BeatCount {
                    line,
                    bar,
                    found,
                    expected: _,
                    meter,
                },
                Pt,
            ) => format!(
                "o compasso {bar} de `{line}` tem {}, mas a fórmula de compasso é {meter}",
                beats_pt(*found)
            ),
            (
                PickupBeats {
                    line,
                    found,
                    expected,
                },
                En,
            ) => format!(
                "the pickup of `{line}` has {}, but the pickup leaves room for {}",
                beats_en(*found),
                beats_en(*expected)
            ),
            (
                PickupBeats {
                    line,
                    found,
                    expected,
                },
                Pt,
            ) => format!(
                "a anacruse de `{line}` tem {}, mas o pickup deixa lugar para {}",
                beats_pt(*found),
                beats_pt(*expected)
            ),
            (
                ChordsOffBeat {
                    line,
                    bar,
                    chords,
                    beats,
                },
                En,
            ) => format!(
                "bar {bar} of `{line}` shares {} among {chords} chords, which does not fall on the beats; hold with _ to say how long each lasts, like `Am _ F _`",
                beats_en(*beats)
            ),
            (
                ChordsOffBeat {
                    line,
                    bar,
                    chords,
                    beats,
                },
                Pt,
            ) => format!(
                "o compasso {bar} de `{line}` divide {} entre {chords} acordes, o que não cai nos tempos; use _ para dizer quanto dura cada um, como `Am _ F _`",
                beats_pt(*beats)
            ),
            (
                JumpWithoutMarker {
                    line,
                    jump,
                    bar,
                    marker,
                },
                En,
            ) => format!("there is a {jump} at bar {bar} of `{line}` but no {marker} for it"),
            (
                JumpWithoutMarker {
                    line,
                    jump,
                    bar,
                    marker,
                },
                Pt,
            ) => {
                format!("há um {jump} no compasso {bar} de `{line}`, mas nenhum {marker} para ele")
            }
            (RepeatsNeverEnd { line }, En) => {
                format!("the repeats and jumps of `{line}` never come to an end")
            }
            (RepeatsNeverEnd { line }, Pt) => {
                format!("as repetições e saltos de `{line}` não chegam ao fim")
            }
            (StrumWithoutChords { part }, En) => format!(
                "the strum of `{part}` has no chords to play; add a line `chords = | Am | F |`"
            ),
            (StrumWithoutChords { part }, Pt) => format!(
                "a batida de `{part}` não tem acordes para tocar; acrescente uma linha `chords = | Am | F |`"
            ),
            (UnknownPattern { name }, En) => format!("there is no pattern called `{name}`"),
            (UnknownPattern { name }, Pt) => format!("não existe padrão chamado `{name}`"),
            (UnknownTarget { name }, En) => {
                format!("there is no part, line or pattern called `{name}`")
            }
            (UnknownTarget { name }, Pt) => {
                format!("não existe parte, linha ou padrão chamado `{name}`")
            }
            (UnknownSection { name }, En) => format!("there is no section called `{name}`"),
            (UnknownSection { name }, Pt) => format!("não existe seção chamada `{name}`"),
            (UnknownParam { name }, En) => {
                format!("`{name}` is not a parameter I know yet, so it gets no knob")
            }
            (UnknownParam { name }, Pt) => {
                format!("ainda não conheço o parâmetro `{name}`, então ele fica sem knob")
            }
            (BadUnit { unit, name }, En) => format!("`{name}` is not measured in {unit}"),
            (BadUnit { unit, name }, Pt) => format!("`{name}` não se mede em {unit}"),
            (
                ParamOutOfRange {
                    name,
                    value,
                    min,
                    max,
                },
                En,
            ) => {
                format!("{value} is outside the range of `{name}`, {min} to {max}")
            }
            (
                ParamOutOfRange {
                    name,
                    value,
                    min,
                    max,
                },
                Pt,
            ) => {
                format!("{value} está fora da faixa de `{name}`, de {min} a {max}")
            }
            (HairpinWithoutTarget { line }, En) => format!(
                "the hairpin in `{line}` has no dynamic mark after it to arrive at, so it moves one step"
            ),
            (HairpinWithoutTarget { line }, Pt) => format!(
                "o crescendo ou diminuendo de `{line}` não tem dinâmica depois para chegar, então anda um degrau"
            ),
            (DuplicateLine { name }, En) => {
                format!("`{name}` is written twice; the last one counts")
            }
            (DuplicateLine { name }, Pt) => format!("`{name}` aparece duas vezes; vale a última"),
            (SameBarChange { bar, what }, En) => {
                format!(
                    "two {} changes at bar {bar}; the last one counts",
                    what.words(En)
                )
            }
            (SameBarChange { bar, what }, Pt) => {
                format!(
                    "duas mudanças de {} no compasso {bar}; vale a última",
                    what.words(Pt)
                )
            }
        }
    }
}
