// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
//! Diagnostics in a musician's words, in Portuguese, Spanish and English.
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
    Es,
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
    /// How long a parameter takes to move: `over 4 bars`.
    Length,
    /// The word `bars` after `every 4`.
    BarsWord,
    /// `to 16` after `from bar 9`.
    RangeEnd,
    /// An arpeggiator's steps: `16ths`, `8ths`, `triplets`.
    ArpStep,
    /// What a part ducks under: `duck by kick`.
    DuckSource,
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
            (BarsWord, En) => "`bars` (every counts whole bars)",
            (BarsWord, Pt) => "`bars` (o every conta compassos inteiros)",
            (BarsWord, Es) => "`bars` (every cuenta compases enteros)",
            (RangeEnd, En) => "`to` and the last bar, such as `to 16`",
            (RangeEnd, Pt) => "`to` e o último compasso, como `to 16`",
            (RangeEnd, Es) => "`to` y el último compás, como `to 16`",
            (Length, En) => {
                "a length such as `over 4 bars`, `over 2 beats`, `over 8 eighths` or `over 16 sixteenths`"
            }
            (Length, Pt) => {
                "uma duração como `over 4 bars` (compassos), `over 2 beats` (tempos), `over 8 eighths` (colcheias) ou `over 16 sixteenths` (semicolcheias)"
            }
            (Number, Es) => "un número",
            (Meter, Es) => "una indicación de compás como 4/4",
            (Key, Es) => "una tonalidad como A minor",
            (Name, Es) => "un nombre",
            (BarNumber, Es) => "un número de compás",
            (Dynamic, Es) => "una dinámica (ppp pp p mp mf f ff fff)",
            (Colon, Es) => "dos puntos",
            (Bars, Es) => "una duración como `over 4 bars`",
            (Beat, Es) => "un tiempo como `beat 4`",
            (Value, Es) => "un valor",
            (Length, Es) => {
                "una duración como `over 4 bars` (compases), `over 2 beats` (tiempos), `over 8 eighths` (corcheas) o `over 16 sixteenths` (semicorcheas)"
            }
            (ArpStep, En) => {
                "the steps, such as `16ths`, `8ths`, `triplets` or `beats`, and maybe `2 octaves`"
            }
            (ArpStep, Pt) => {
                "os passos, como `16ths` (semicolcheias), `8ths` (colcheias), `triplets` (tercinas) ou `beats` (tempos), e talvez `2 octaves` (oitavas)"
            }
            (ArpStep, Es) => {
                "los pasos, como `16ths` (semicorcheas), `8ths` (corcheas), `triplets` (tresillos) o `beats` (tiempos), y quizá `2 octaves` (octavas)"
            }
            (DuckSource, En) => {
                "`by` and the part or line it makes room for, such as `duck by kick`"
            }
            (DuckSource, Pt) => {
                "`by` (com) e a parte ou linha para a qual ela abre espaço, como `duck by kick`"
            }
            (DuckSource, Es) => {
                "`by` (con) y la parte o línea a la que le hace sitio, como `duck by kick`"
            }
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
            (Change::Tempo, Lang::Es) => "tempo",
            (Change::Meter, Lang::Es) => "indicación de compás",
            (Change::Key, Lang::Es) => "tonalidad",
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
    /// `volume =` or `bass.reverb.mix =` with no number after `=`.
    MissingNumber {
        /// What is written before `=`.
        name: String,
        /// A number that fits, with its unit: `0.8`, `2000 Hz`.
        example: Option<String>,
        /// No part is named, so the number would hold for every part.
        every_part: bool,
    },
    /// `bass =` with no music after `=`.
    EmptyLine {
        name: String,
    },
    /// `motif riff =` or `row serie =` with nothing after `=`.
    EmptyDefinition {
        name: String,
        row: bool,
    },

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
    /// Two notes glued inside brackets: only a space is missing.
    GluedNotesInGroup {
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
    /// `c5:trem`: a word after a note that is no technique.
    UnknownTechnique {
        word: String,
        closest: Option<String>,
    },
    /// `s:tr`: a trill or a mordent on what has no note above or below.
    OrnamentNeedsNote {
        word: String,
    },
    /// `` `d5 `` with no note after it to lead into.
    GraceBeforeNothing,
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
    /// `hh: out`: out, in and levels are for parts, and `hh` is a line.
    LineNotPart {
        line: String,
        part: String,
    },
    UnknownSection {
        name: String,
    },
    UnknownParam {
        name: String,
        closest: Option<String>,
    },
    /// A motif that uses itself, directly or through others.
    MotifLoop {
        name: String,
    },
    /// A motif written with bar lines.
    MotifWithBars,
    /// `{a2 e3 a e}`: braces with a single option.
    BracesOneOption {
        inner: String,
    },
    /// `from serie` names no row, motif or vector.
    UnknownRow {
        name: String,
    },
    /// `serie.X3`: not a form of a row.
    BadRowForm {
        form: String,
    },
    /// A row's token is not a note.
    BadRowNote {
        text: String,
    },
    /// A twelve-note row repeats a pitch class.
    RowRepeats {
        note: String,
    },
    /// `temas.9` in a list of fewer melodies.
    ListIndex {
        name: String,
        index: usize,
        count: usize,
    },
    /// `from` with no `o` in the rhythm.
    NoPlaceholders,
    /// A chain names a voice or an effect that does not exist.
    UnknownSound {
        name: String,
        closest: Option<String>,
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
    /// A word before `:` that stands for a header: `andamento: 120`.
    UnknownHeader {
        word: String,
        suggestion: String,
    },
    /// `bass: 120`: nothing a part can be told.
    BadPartCommand {
        part: String,
    },
    /// A misspelt keyword: `ply groove`.
    UnknownWord {
        word: String,
        suggestion: String,
    },
    /// `riff.foo`: a form a motif does not have.
    UnknownMotifForm {
        name: String,
        form: String,
    },
    /// `temas.foo`: something a list cannot pick.
    UnknownListPick {
        name: String,
        pick: String,
    },
    /// `every 4 bars: melody = …`, or `at bar` before a line inside a
    /// pattern or section.
    LineChangeOnlyAtBar,
    /// `groove` with a name that is not one of the grooves.
    UnknownGroove {
        name: String,
        closest: Option<String>,
        names: String,
    },
    /// `at bar 9: groove house`: a groove writes lines, which do not
    /// change at a time this way.
    GrooveOnlyBare,
    /// A groove in a piece that is not in 4/4.
    GrooveMeter,
    /// `-> tube -> drive`: the valve is Manguebyte's drive stage.
    TubeAndDrive,
    /// A degree written as a note beyond what a melody reaches.
    DegreeRange {
        degree: i32,
    },
    /// A letter a percussion instrument does not play.
    UnknownStroke {
        ch: char,
        instrument: crate::ast::Perc,
    },
    /// `7/8 = 2+3`: groups that do not add up to the meter.
    BadMeterGroups {
        text: String,
        units: u32,
    },
    /// `from bar 16 to 9:`
    BadRange {
        from: i64,
        to: i64,
    },
    /// `from bar 9 to 16: tempo 140`: tempo, meter and key hold until the
    /// next change.
    FrameInRange,
    /// `at bar 9 beat 3: meter 3/4`
    FrameAtBeat,
    /// `at bar 9 beat 5:` in a bar of four beats.
    BeatOutOfBar {
        bar: i64,
        beat: f64,
        beats: Ratio,
    },
    /// `b(3)`, `b(3,x)`: a Euclidean rhythm written wrong.
    BadEuclidForm {
        text: String,
    },
    /// `b(9,8)`: more hits than steps, or steps that cannot be.
    BadEuclid {
        hits: i64,
        steps: i64,
    },
    /// `arp = up 16ths 9 octaves`
    ArpOctaves {
        count: i64,
    },
    /// An arpeggiator in a part with no chords line.
    ArpWithoutChords {
        part: String,
    },
    /// `duck by kick 150%`
    DuckAmount {
        amount: f64,
    },
    /// `hh.fast 1.5`, `bass.slow 0`
    BadSpeed {
        verb: String,
        times: f64,
    },
}

/// A share of a volume as a percentage: 1.5 is `150%`.
fn amount_text(amount: f64) -> String {
    format!("{}%", (amount * 10000.0).round() / 100.0)
}

fn beats_en(n: Ratio) -> String {
    if n == Ratio::ONE {
        "1 beat".into()
    } else {
        format!("{n} beats")
    }
}

fn beats_es(n: Ratio) -> String {
    if n == Ratio::ONE {
        "1 tiempo".into()
    } else {
        format!("{n} tiempos")
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
            (
                MissingNumber {
                    name,
                    example,
                    every_part,
                },
                En,
            ) => {
                let mut s = format!("a number is missing after `{name} =`");
                if let Some(e) = example {
                    s += &format!("; for example `{name} = {e}`");
                }
                if *every_part {
                    s += " (with no part name, it holds for every part)";
                }
                s
            }
            (
                MissingNumber {
                    name,
                    example,
                    every_part,
                },
                Pt,
            ) => {
                let mut s = format!("falta o número depois de `{name} =`");
                if let Some(e) = example {
                    s += &format!("; por exemplo `{name} = {e}`");
                }
                if *every_part {
                    s += " (sem nome de parte, vale para todas as partes)";
                }
                s
            }
            (EmptyLine { name }, En) => {
                format!("the line `{name}` is empty: write its music after the `=`")
            }
            (EmptyLine { name }, Pt) => {
                format!("a linha `{name}` está vazia: escreva a música depois do `=`")
            }
            (EmptyDefinition { name, row: false }, En) => {
                format!("the motif `{name}` is empty: write its notes after the `=`")
            }
            (EmptyDefinition { name, row: false }, Pt) => {
                format!("o motivo `{name}` está vazio: escreva as notas depois do `=`")
            }
            (EmptyDefinition { name, row: true }, En) => {
                format!("the row `{name}` is empty: write its notes after the `=`")
            }
            (EmptyDefinition { name, row: true }, Pt) => {
                format!("a série `{name}` está vazia: escreva as notas depois do `=`")
            }

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
            (GluedNotesInGroup { first, second }, En) => format!(
                "two notes are written together; put a space between them: `{first} {second}`"
            ),
            (GluedNotesInGroup { first, second }, Pt) => {
                format!("duas notas estão coladas; separe com um espaço: `{first} {second}`")
            }
            (GluedNotes { first, second }, Pt) => format!(
                "duas notas estão coladas num tempo; escreva entre colchetes: `[{first} {second}]`"
            ),
            (UnknownDrum { ch }, En) => format!(
                "`{ch}` is not a drum symbol: c closed hi-hat, o open hi-hat, b bass drum, s snare, r ride, x crash, t high tom, m mid tom, f floor tom, k side stick (rim), p hand clap"
            ),
            (UnknownDrum { ch }, Pt) => format!(
                "`{ch}` não é um símbolo de bateria: c chimbal fechado, o chimbal aberto, b bumbo, s caixa, r condução, x ataque, t tom agudo, m tom médio, f surdo, k aro, p palma"
            ),
            (UnknownStrum { ch }, En) => format!(
                "`{ch}` is not a strum symbol: d down, u up, m muted, g a tap on the top (golpe), t down and golpe together, - the hand passes, _ let it ring"
            ),
            (UnknownStrum { ch }, Pt) => format!(
                "`{ch}` não é um símbolo de batida: d para baixo, u para cima, m abafado, g golpe no tampo, t para baixo com golpe, - a mão passa, _ deixa soar"
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
            (UnknownTechnique { word, closest }, En) => match closest {
                Some(c) => format!("`:{word}` is not a technique I know; did you mean `:{c}`?"),
                None => format!(
                    "`:{word}` is not a technique I know: `:tr` trill, `:mord` and `:mordi` mordents, `:arp` a rolled chord"
                ),
            },
            (UnknownTechnique { word, closest }, Pt) => match closest {
                Some(c) => format!("não conheço a técnica `:{word}`; seria `:{c}`?"),
                None => format!(
                    "não conheço a técnica `:{word}`: `:tr` trinado, `:mord` e `:mordi` mordentes, `:arp` acorde arpejado"
                ),
            },
            (OrnamentNeedsNote { word }, En) => format!(
                "`:{word}` goes after a note: it plays the note's neighbour in the key, and a drum or a strum has none"
            ),
            (OrnamentNeedsNote { word }, Pt) => format!(
                "`:{word}` vai depois de uma nota: ele toca a vizinha dela na tonalidade, e um tambor ou uma batida não tem vizinha"
            ),
            (GraceBeforeNothing, En) => {
                "a grace note, written with a backtick (`), goes right before the note it leads into".into()
            }
            (GraceBeforeNothing, Pt) => {
                "a nota de graça, escrita com crase (`), vai logo antes da nota em que ela cai".into()
            }
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
            (LineNotPart { line, part }, En) => format!(
                "`{line}` is a line of the part `{part}`; out, in and levels are for a whole part: `{part}:`"
            ),
            (LineNotPart { line, part }, Pt) => format!(
                "`{line}` é uma linha da parte `{part}`; out, in e dinâmica valem para a parte inteira: `{part}:`"
            ),
            (UnknownSection { name }, En) => format!("there is no section called `{name}`"),
            (UnknownSection { name }, Pt) => format!("não existe seção chamada `{name}`"),
            (MotifLoop { name }, En) => format!("the motif `{name}` uses itself, so it would never end"),
            (MotifLoop { name }, Pt) => format!("o motivo `{name}` usa a si mesmo e nunca terminaria"),
            (BracesOneOption { inner }, En) => format!("braces choose between options split by `|`, as in `{{a|b}}`; to group notes in one beat, use brackets: `[{inner}]` (and `[{inner}]x4` to repeat it)"),
            (BracesOneOption { inner }, Pt) => format!("chaves escolhem entre opções separadas por `|`, como `{{a|b}}`; para agrupar notas num tempo, use colchetes: `[{inner}]` (e `[{inner}]x4` para repetir)"),
            (MotifWithBars, En) => "a motif is a few beats, without bar lines; for whole bars, use a list".into(),
            (MotifWithBars, Pt) => "um motivo é um trecho de tempos, sem barras de compasso; para compassos inteiros, use uma lista".into(),
            (UnknownRow { name }, En) => format!("there is no row, motif or list of notes called `{name}`"),
            (UnknownRow { name }, Pt) => format!("não existe série, motivo ou vetor de notas chamado `{name}`"),
            (BadRowForm { form }, En) => format!("`{form}` is not a form of a row: use P, I, R or RI with a number from 0 to 11, as in serie.RI3"),
            (BadRowForm { form }, Pt) => format!("`{form}` não é uma forma de série: use P, I, R ou RI com um número de 0 a 11, como em serie.RI3"),
            (BadRowNote { text }, En) => format!("`{text}` is not a note of a row: use 0 to 11 or a note name, such as c, eb4 or dó"),
            (BadRowNote { text }, Pt) => format!("`{text}` não é uma nota de série: use 0 a 11 ou um nome de nota, como dó, mib4 ou c"),
            (RowRepeats { note }, En) => format!("this twelve-note row repeats `{note}`; a twelve-tone row uses each of the twelve notes once"),
            (RowRepeats { note }, Pt) => format!("esta série de doze notas repete `{note}`; uma série dodecafônica usa cada uma das doze notas uma vez"),
            (ListIndex { name, index, count }, En) => format!("the list `{name}` has {count} melodies, so there is no number {index}"),
            (ListIndex { name, index, count }, Pt) => format!("a lista `{name}` tem {count} melodias, então não existe a de número {index}"),
            (NoPlaceholders, En) => "this line takes notes from a row but has no `o` to put them in".into(),
            (NoPlaceholders, Pt) => "esta linha pega notas de uma série, mas não tem nenhum `o` para colocá-las".into(),
            (UnknownSound { name, closest }, En) => match closest {
                Some(c) => format!(
                    "there is no voice or effect called `{name}`; did you mean `{c}`? The Sounds panel lists them all"
                ),
                None => format!(
                    "there is no voice or effect called `{name}`; the Sounds panel lists them all"
                ),
            },
            (UnknownSound { name, closest }, Pt) => match closest {
                Some(c) => format!(
                    "não existe som ou efeito chamado `{name}`; seria `{c}`? O painel Sons mostra todos"
                ),
                None => {
                    format!("não existe som ou efeito chamado `{name}`; o painel Sons mostra todos")
                }
            },
            (UnknownParam { name, closest }, En) => match closest {
                Some(c) => format!("`{name}` is not a parameter I know; did you mean `{c}`?"),
                None => format!("`{name}` is not a parameter I know yet, so it gets no knob"),
            },
            (UnknownParam { name, closest }, Pt) => match closest {
                Some(c) => format!("não conheço o parâmetro `{name}`; seria `{c}`?"),
                None => format!("ainda não conheço o parâmetro `{name}`, então ele fica sem knob"),
            },
            (UnknownHeader { word, suggestion }, En) => {
                format!("I do not know `{word}:`; did you mean `{suggestion}:`?")
            }
            (UnknownHeader { word, suggestion }, Pt) => {
                format!("não conheço `{word}:`; seria `{suggestion}:`?")
            }
            (UnknownHeader { word, suggestion }, Es) => {
                format!("no conozco `{word}:`; ¿sería `{suggestion}:`?")
            }
            (BadPartCommand { part }, En) => format!(
                "after `{part}:` comes a dynamic (`mf`), `out` or `in`, `cresc to ff over 4 bars`, `swing 60%`, `duck by kick`, or the sound and its effects (`synth.saw -> lowpass`)"
            ),
            (BadPartCommand { part }, Pt) => format!(
                "depois de `{part}:` vem uma dinâmica (`mf`), `out` ou `in`, `cresc to ff over 4 bars`, `swing 60%`, `duck by kick`, ou o som e os efeitos (`synth.saw -> lowpass`)"
            ),
            (BadPartCommand { part }, Es) => format!(
                "después de `{part}:` va una dinámica (`mf`), `out` o `in`, `cresc to ff over 4 bars`, `swing 60%`, `duck by kick`, o el sonido y sus efectos (`synth.saw -> lowpass`)"
            ),
            (UnknownWord { word, suggestion }, En) => {
                format!("I do not know `{word}`; did you mean `{suggestion}`?")
            }
            (UnknownWord { word, suggestion }, Pt) => {
                format!("não conheço `{word}`; seria `{suggestion}`?")
            }
            (UnknownWord { word, suggestion }, Es) => {
                format!("no conozco `{word}`; ¿sería `{suggestion}`?")
            }
            (UnknownMotifForm { name, form }, En) => format!(
                "`{name}` is a motif, but `.{form}` is not one of its forms: `.inv`, `.retro`, `.ri`, `+2` (degrees), `+3st` (semitones)"
            ),
            (UnknownMotifForm { name, form }, Pt) => format!(
                "`{name}` é um motivo, mas `.{form}` não é uma forma dele: `.inv`, `.retro`, `.ri`, `+2` (graus), `+3st` (semitons)"
            ),
            (UnknownMotifForm { name, form }, Es) => format!(
                "`{name}` es un motivo, pero `.{form}` no es una de sus formas: `.inv`, `.retro`, `.ri`, `+2` (grados), `+3st` (semitonos)"
            ),
            (UnknownStroke { ch, instrument }, En) => format!("`{ch}` is not a stroke {}", instrument.help(En)),
            (UnknownStroke { ch, instrument }, Pt) => format!("`{ch}` não é um golpe {}", instrument.help(Pt)),
            (UnknownStroke { ch, instrument }, Es) => format!("`{ch}` no es un golpe {}", instrument.help(Es)),
            (BadMeterGroups { text, units }, En) => format!("`{text}` should be groups that add up to {units}, such as `2+2+3` for 7/8"),
            (BadMeterGroups { text, units }, Pt) => format!("`{text}` deveria ser grupos que somam {units}, como `2+2+3` para 7/8"),
            (BadMeterGroups { text, units }, Es) => format!("`{text}` debería ser grupos que sumen {units}, como `2+2+3` para 7/8"),
            (BadRange { from, to }, En) => format!("the range ends before it starts: write `from bar {to} to {from}`"),
            (BadRange { from, to }, Pt) => format!("o trecho termina antes de começar: escreva `from bar {to} to {from}`"),
            (BadRange { from, to }, Es) => format!("el tramo termina antes de empezar: escribe `from bar {to} to {from}`"),
            (FrameInRange, En) => "a change of tempo, meter or key holds until the next one: write it with `at bar`, and another where it should go back".into(),
            (FrameInRange, Pt) => "uma mudança de andamento, fórmula ou tom vale até a próxima: escreva com `at bar`, e outra onde ela deve voltar".into(),
            (FrameInRange, Es) => "un cambio de tempo, compás o tonalidad vale hasta el siguiente: escríbelo con `at bar`, y otro donde deba volver".into(),
            (FrameAtBeat, En) => "meter and key change at a bar line: write `at bar 9:` without `beat`".into(),
            (FrameAtBeat, Pt) => "fórmula e tom mudam na barra de compasso: escreva `at bar 9:` sem `beat`".into(),
            (FrameAtBeat, Es) => "el compás y la tonalidad cambian en la barra: escribe `at bar 9:` sin `beat`".into(),
            (BeatOutOfBar { bar, beat, beats }, En) => format!("bar {bar} has {}, so there is no beat {beat}", beats_en(*beats)),
            (BeatOutOfBar { bar, beat, beats }, Pt) => format!("o compasso {bar} tem {}, então não existe o tempo {beat}", beats_pt(*beats)),
            (BeatOutOfBar { bar, beat, beats }, Es) => format!("el compás {bar} tiene {}, así que no existe el tiempo {beat}", beats_es(*beats)),
            (LineChangeOnlyAtBar, En) => "a line can change at a bar (`at bar 9: melody = …`) outside patterns and sections, but not with `every` or inside a pattern; to vary it every few bars, use a pattern with `every`".into(),
            (LineChangeOnlyAtBar, Pt) => "uma linha pode mudar num compasso (`at bar 9: melodia = …`) fora de padrões e seções, mas não com `every` nem dentro de um padrão; para variar de tantos em tantos compassos, use um padrão com `every`".into(),
            (UnknownGroove { name, closest, names }, En) => match closest {
                Some(c) => format!("there is no groove called `{name}`; did you mean `{c}`? The grooves are: {names}"),
                None => format!("there is no groove called `{name}`; the grooves are: {names}"),
            },
            (UnknownGroove { name, closest, names }, Pt) => match closest {
                Some(c) => format!("não existe a levada `{name}`; seria `{c}`? As levadas são: {names}"),
                None => format!("não existe a levada `{name}`; as levadas são: {names}"),
            },
            (UnknownGroove { name, closest, names }, Es) => match closest {
                Some(c) => format!("no existe el ritmo `{name}`; ¿sería `{c}`? Los ritmos son: {names}"),
                None => format!("no existe el ritmo `{name}`; los ritmos son: {names}"),
            },
            (TubeAndDrive, En) => "the tube is Manguebyte's drive stage set up as a valve, so a chain has one or the other; the drive's settings win here".into(),
            (TubeAndDrive, Pt) => "a válvula é o estágio de drive do Manguebyte ajustado como válvula, então a cadeia tem um ou o outro; aqui valem os ajustes do drive".into(),
            (TubeAndDrive, Es) => "la válvula es la etapa de drive de Manguebyte ajustada como válvula, así que la cadena tiene una u otro; aquí valen los ajustes del drive".into(),
            (GrooveOnlyBare, En) => "a groove writes lines, so it goes on a line of its own, without `at bar` or `every`; to change it at a bar, write the line that changes (`at bar 9: hh = …`) or use patterns".into(),
            (GrooveOnlyBare, Pt) => "uma levada escreve linhas, então vai numa linha só dela, sem `at bar` nem `every`; para mudar num compasso, escreva a linha que muda (`at bar 9: hh = …`) ou use padrões".into(),
            (GrooveOnlyBare, Es) => "un ritmo escribe líneas, así que va en una línea propia, sin `at bar` ni `every`; para cambiarlo en un compás, escribe la línea que cambia (`at bar 9: hh = …`) o usa patrones".into(),
            (DegreeRange { degree }, En) => format!("`{degree}` is not a degree a melody reaches: degrees go from 1, the key's first note, up to 21, three octaves higher; for notes below, write a comma after the degree (`5,`)"),
            (DegreeRange { degree }, Pt) => format!("`{degree}` não é um grau que uma melodia alcança: os graus vão do 1, a primeira nota do tom, até o 21, três oitavas acima; para notas mais graves, escreva uma vírgula depois do grau (`5,`)"),
            (DegreeRange { degree }, Es) => format!("`{degree}` no es un grado que una melodía alcance: los grados van del 1, la primera nota de la tonalidad, hasta el 21, tres octavas arriba; para notas más graves, escribe una coma después del grado (`5,`)"),
            (GrooveMeter, En) => "the grooves are in 4/4, and this piece is in another meter; write `meter: 4/4`, or write the drum lines yourself".into(),
            (GrooveMeter, Pt) => "as levadas são em 4/4, e esta peça está em outra fórmula; escreva `meter: 4/4`, ou escreva as linhas da bateria você mesmo".into(),
            (GrooveMeter, Es) => "los ritmos son en 4/4, y esta pieza está en otro compás; escribe `meter: 4/4`, o escribe tú las líneas de la batería".into(),
            (LineChangeOnlyAtBar, Es) => "una línea puede cambiar en un compás (`at bar 9: melodia = …`) fuera de patrones y secciones, pero no con `every` ni dentro de un patrón; para variarla cada tantos compases, usa un patrón con `every`".into(),
            (UnknownListPick { name, pick }, En) => format!(
                "the list `{name}` has no `.{pick}`: pick `.1`, `.2`…, `.all` or `.random`"
            ),
            (UnknownListPick { name, pick }, Pt) => format!(
                "a lista `{name}` não tem `.{pick}`: escolha `.1`, `.2`…, `.all` ou `.random`"
            ),
            (UnknownListPick { name, pick }, Es) => format!(
                "la lista `{name}` no tiene `.{pick}`: elige `.1`, `.2`…, `.all` o `.random`"
            ),
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
            (UnclosedBlock { name }, Es) => {
                format!("el bloque `{name}` se abre con {{ pero nunca se cierra con }}")
            }
            (StrayClose, Es) => "esta } no cierra nada".into(),
            (UnknownLine, Es) => "no entiendo esta línea".into(),
            (Expected { what, after }, Es) => {
                format!("falta {} después de `{after}`", what.words(Es))
            }
            (Unexpected { text }, Es) => format!("`{text}` no va aquí"),
            (BadMeter { text }, Es) => format!(
                "`{text}` no es una indicación de compás: escribe dos números como 4/4 o 6/8, el de abajo 1, 2, 4, 8, 16 o 32"
            ),
            (TempoOutOfRange { bpm }, Es) => {
                format!("un tempo de {bpm} está fuera de 10 a 400 pulsos por minuto")
            }
            (UnknownKey { text }, Es) => {
                format!("`{text}` no es una tonalidad; escríbela como `A minor` o `F# major`")
            }
            (DuplicateHeader { name }, Es) => {
                format!("`{name}:` aparece dos veces; vale la última")
            }
            (BadPickup { beat, beats }, Es) => {
                format!("un compás de {beats} tiempos no tiene un tiempo {beat} para empezar")
            }
            (BarZero, Es) => "los compases se cuentan desde 1".into(),
            (
                MissingNumber {
                    name,
                    example,
                    every_part,
                },
                Es,
            ) => {
                let mut s = format!("falta el número después de `{name} =`");
                if let Some(e) = example {
                    s += &format!("; por ejemplo `{name} = {e}`");
                }
                if *every_part {
                    s += " (sin nombre de parte, vale para todas las partes)";
                }
                s
            }
            (EmptyLine { name }, Es) => {
                format!("la línea `{name}` está vacía: escribe la música después del `=`")
            }
            (EmptyDefinition { name, row: false }, Es) => {
                format!("el motivo `{name}` está vacío: escribe las notas después del `=`")
            }
            (EmptyDefinition { name, row: true }, Es) => {
                format!("la serie `{name}` está vacía: escribe las notas después del `=`")
            }
            (UnclosedBracket { open }, Es) => format!("este {open} nunca se cierra"),
            (UnexpectedChar { ch }, Es) => format!("`{ch}` no significa nada aquí"),
            (UnknownNote { text }, Es) => format!(
                "`{text}` no es una nota: las notas son do re mi fa sol la si, o c d e f g a b, con la octava, como do4"
            ),
            (GluedNotesInGroup { first, second }, Es) => {
                format!("hay dos notas pegadas; sepáralas con un espacio: `{first} {second}`")
            }
            (GluedNotes { first, second }, Es) => format!(
                "hay dos notas pegadas en un tiempo; escríbelas entre corchetes: `[{first} {second}]`"
            ),
            (UnknownDrum { ch }, Es) => format!(
                "`{ch}` no es un símbolo de batería: c hi-hat cerrado, o hi-hat abierto, b bombo, s caja, r ride, x crash, t tom agudo, m tom medio, f tom de piso, k aro, p palmas"
            ),
            (UnknownStrum { ch }, Es) => format!(
                "`{ch}` no es un símbolo de rasgueo: d hacia abajo, u hacia arriba, m apagado, g golpe en la tapa, t hacia abajo con golpe, - la mano pasa, _ deja sonar"
            ),
            (UnknownChord { text }, Es) => format!("no conozco el acorde `{text}`"),
            (UnknownMark { word }, Es) => format!(
                "`{word}:` no es una dinámica (ppp pp p mp mf f ff fff sfz fp) ni una señal (segno: fine: coda:)"
            ),
            (ChanceRange { percent }, Es) => {
                format!("una probabilidad de {percent}% no está entre 0% y 100%")
            }
            (DynamicBeforeNothing, Es) => "la dinámica va justo antes de una nota".into(),
            (UnknownTechnique { word, closest }, Es) => match closest {
                Some(c) => format!("no conozco la técnica `:{word}`; ¿quisiste decir `:{c}`?"),
                None => format!(
                    "no conozco la técnica `:{word}`: `:tr` trino, `:mord` y `:mordi` mordentes, `:arp` acorde arpegiado"
                ),
            },
            (OrnamentNeedsNote { word }, Es) => format!(
                "`:{word}` va después de una nota: toca su vecina en la tonalidad, y un tambor o un rasgueo no tiene vecina"
            ),
            (GraceBeforeNothing, Es) => {
                "la nota de adorno, escrita con acento grave (`), va justo antes de la nota en la que cae".into()
            }
            (EmptyBrackets, Es) => "estos corchetes están vacíos".into(),
            (EmptyChoice, Es) => "una de las opciones entre { } está vacía".into(),
            (MisplacedMarker { marker }, Es) => {
                format!("`{marker}:` va al principio de un compás")
            }
            (MisplacedJump { jump }, Es) => {
                format!("`{jump}` va al final de un compás, justo antes de la barra")
            }
            (MisplacedRepeatCount, Es) => {
                "el número de veces, como `x4`, va justo después de :|".into()
            }
            (EmptyBar, Es) => "este compás está vacío; escribe silencios, como `- - - -`".into(),
            (BarRepeatNotAlone, Es) => {
                "`%` vale un compás entero y va solo entre las barras".into()
            }
            (NothingToRepeat, Es) => "no hay un compás antes de este `%` para repetir".into(),
            (HoldWithNothing, Es) => "este `_` no tiene una nota antes para sostener".into(),
            (NoOctave { note }, Es) => format!(
                "`{note}` no tiene octava y ninguna nota anterior le da una, así que vale {note}4"
            ),
            (
                BeatCount {
                    line,
                    bar,
                    found,
                    expected: _,
                    meter,
                },
                Es,
            ) => format!(
                "el compás {bar} de `{line}` tiene {}, pero la indicación de compás es {meter}",
                beats_es(*found)
            ),
            (
                PickupBeats {
                    line,
                    found,
                    expected,
                },
                Es,
            ) => format!(
                "la anacrusa de `{line}` tiene {}, pero el pickup deja lugar para {}",
                beats_es(*found),
                beats_es(*expected)
            ),
            (
                ChordsOffBeat {
                    line,
                    bar,
                    chords,
                    beats,
                },
                Es,
            ) => format!(
                "el compás {bar} de `{line}` reparte {} entre {chords} acordes, y eso no cae en los tiempos; usa _ para decir cuánto dura cada uno, como `Am _ F _`",
                beats_es(*beats)
            ),
            (
                JumpWithoutMarker {
                    line,
                    jump,
                    bar,
                    marker,
                },
                Es,
            ) => format!(
                "hay un {jump} en el compás {bar} de `{line}`, pero ningún {marker} para él"
            ),
            (RepeatsNeverEnd { line }, Es) => {
                format!("las repeticiones y saltos de `{line}` nunca terminan")
            }
            (StrumWithoutChords { part }, Es) => format!(
                "el rasgueo de `{part}` no tiene acordes para tocar; agrega una línea `chords = | Am | F |`"
            ),
            (UnknownPattern { name }, Es) => format!("no existe un patrón llamado `{name}`"),
            (UnknownTarget { name }, Es) => {
                format!("no existe una parte, línea o patrón llamado `{name}`")
            }
            (LineNotPart { line, part }, Es) => format!(
                "`{line}` es una línea de la parte `{part}`; out, in y los matices valen para la parte entera: `{part}:`"
            ),
            (UnknownSection { name }, Es) => format!("no existe una sección llamada `{name}`"),
            (MotifLoop { name }, Es) => format!("el motivo `{name}` se usa a sí mismo y nunca terminaría"),
            (BracesOneOption { inner }, Es) => format!("las llaves eligen entre opciones separadas por `|`, como `{{a|b}}`; para agrupar notas en un tiempo, usa corchetes: `[{inner}]` (y `[{inner}]x4` para repetirlo)"),
            (MotifWithBars, Es) => "un motivo es un trozo de tiempos, sin barras de compás; para compases enteros, usa una lista".into(),
            (UnknownRow { name }, Es) => format!("no existe una serie, un motivo o un vector de notas llamado `{name}`"),
            (BadRowForm { form }, Es) => format!("`{form}` no es una forma de serie: usa P, I, R o RI con un número de 0 a 11, como en serie.RI3"),
            (BadRowNote { text }, Es) => format!("`{text}` no es una nota de serie: usa 0 a 11 o un nombre de nota, como do, mib4 o c"),
            (RowRepeats { note }, Es) => format!("esta serie de doce notas repite `{note}`; una serie dodecafónica usa cada una de las doce notas una vez"),
            (ListIndex { name, index, count }, Es) => format!("la lista `{name}` tiene {count} melodías, así que no existe la número {index}"),
            (NoPlaceholders, Es) => "esta línea toma notas de una serie, pero no tiene ningún `o` donde ponerlas".into(),
            (UnknownSound { name, closest }, Es) => match closest {
                Some(c) => format!(
                    "no existe un sonido o efecto llamado `{name}`; ¿quisiste decir `{c}`? El panel Sonidos los muestra todos"
                ),
                None => format!(
                    "no existe un sonido o efecto llamado `{name}`; el panel Sonidos los muestra todos"
                ),
            },
            (UnknownParam { name, closest }, Es) => match closest {
                Some(c) => format!("no conozco el parámetro `{name}`; ¿sería `{c}`?"),
                None => format!("todavía no conozco el parámetro `{name}`, así que no tiene perilla"),
            },
            (BadUnit { unit, name }, Es) => format!("`{name}` no se mide en {unit}"),
            (
                ParamOutOfRange {
                    name,
                    value,
                    min,
                    max,
                },
                Es,
            ) => format!("{value} está fuera del rango de `{name}`, de {min} a {max}"),
            (HairpinWithoutTarget { line }, Es) => format!(
                "el regulador de `{line}` no tiene una dinámica después a la que llegar, así que avanza un escalón"
            ),
            (DuplicateLine { name }, Es) => format!("`{name}` aparece dos veces; vale la última"),
            (SameBarChange { bar, what }, Es) => format!(
                "dos cambios de {} en el compás {bar}; vale el último",
                what.words(Es)
            ),
            (SameBarChange { bar, what }, Pt) => {
                format!(
                    "duas mudanças de {} no compasso {bar}; vale a última",
                    what.words(Pt)
                )
            }
            (BadEuclidForm { text }, En) => format!(
                "`{text}`: write a Euclidean rhythm as the symbol and two numbers, the hits and the steps, such as `b(3,8)`, and maybe a third that turns it, `b(3,8,2)`"
            ),
            (BadEuclidForm { text }, Pt) => format!(
                "`{text}`: escreva um ritmo euclidiano como o símbolo e dois números, os toques e os passos, como `b(3,8)`, e talvez um terceiro que gira o ritmo, `b(3,8,2)`"
            ),
            (BadEuclidForm { text }, Es) => format!(
                "`{text}`: escribe un ritmo euclidiano como el símbolo y dos números, los golpes y los pasos, como `b(3,8)`, y quizá un tercero que lo gira, `b(3,8,2)`"
            ),
            (BadEuclid { steps, .. }, En) if *steps < 1 || *steps > 64 => {
                format!("a Euclidean rhythm takes from 1 to 64 steps, not {steps}")
            }
            (BadEuclid { steps, .. }, Pt) if *steps < 1 || *steps > 64 => {
                format!("um ritmo euclidiano tem de 1 a 64 passos, não {steps}")
            }
            (BadEuclid { steps, .. }, Es) if *steps < 1 || *steps > 64 => {
                format!("un ritmo euclidiano tiene de 1 a 64 pasos, no {steps}")
            }
            (BadEuclid { hits, steps }, En) => {
                format!("{hits} hits do not fit in {steps} steps: the first number is the hits, the second the steps")
            }
            (BadEuclid { hits, steps }, Pt) => {
                format!("{hits} toques não cabem em {steps} passos: o primeiro número são os toques, o segundo os passos")
            }
            (BadEuclid { hits, steps }, Es) => {
                format!("{hits} golpes no caben en {steps} pasos: el primer número son los golpes, el segundo los pasos")
            }
            (ArpOctaves { count }, En) => {
                format!("an arpeggiator goes over 1 to 4 octaves, not {count}")
            }
            (ArpOctaves { count }, Pt) => {
                format!("um arpejador percorre de 1 a 4 oitavas, não {count}")
            }
            (ArpOctaves { count }, Es) => {
                format!("un arpegiador recorre de 1 a 4 octavas, no {count}")
            }
            (ArpWithoutChords { part }, En) => format!(
                "the arpeggiator of `{part}` has no chords to play; add a line `chords = | Am | F |`"
            ),
            (ArpWithoutChords { part }, Pt) => format!(
                "o arpejador de `{part}` não tem acordes para tocar; acrescente uma linha `chords = | Am | F |`"
            ),
            (ArpWithoutChords { part }, Es) => format!(
                "el arpegiador de `{part}` no tiene acordes que tocar; añade una línea `chords = | Am | F |`"
            ),
            (DuckAmount { amount }, En) => format!(
                "a part ducks by 0% to 100% (or 0 to 1) of its volume, not {}",
                amount_text(*amount)
            ),
            (DuckAmount { amount }, Pt) => format!(
                "uma parte abaixa de 0% a 100% (ou de 0 a 1) do seu volume, não {}",
                amount_text(*amount)
            ),
            (DuckAmount { amount }, Es) => format!(
                "una parte baja de 0% a 100% (o de 0 a 1) de su volumen, no {}",
                amount_text(*amount)
            ),
            (BadSpeed { verb, times }, En) => format!(
                "`{verb}` takes a whole number from 1 to 16, so every bar lands on a bar line: `{verb} 2`, not `{verb} {times}`"
            ),
            (BadSpeed { verb, times }, Pt) => format!(
                "`{verb}` leva um número inteiro de 1 a 16, para todo compasso cair numa barra: `{verb} 2`, não `{verb} {times}`"
            ),
            (BadSpeed { verb, times }, Es) => format!(
                "`{verb}` lleva un número entero de 1 a 16, para que cada compás caiga en una barra: `{verb} 2`, no `{verb} {times}`"
            ),
        }
    }
}
