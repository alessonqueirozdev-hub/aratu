// The example pieces, read from the repository's examples/ folder (a test in
// the language crate checks that each compiles without a single warning).

import chance from "../../examples/chance.aratu";
import drumAndBass from "../../examples/drum-and-bass.aratu";
import firstPiece from "../../examples/first-piece.aratu";
import form from "../../examples/form.aratu";
import gFunk from "../../examples/g-funk.aratu";
import manguebeat from "../../examples/manguebeat.aratu";
import melody from "../../examples/melody.aratu";
import motifs from "../../examples/motifs.aratu";
import patterns from "../../examples/patterns.aratu";
import setlist from "../../examples/setlist.aratu";
import sound from "../../examples/sound.aratu";
import sweeps from "../../examples/sweeps.aratu";
import trance from "../../examples/trance.aratu";
import twelveTone from "../../examples/twelve-tone.aratu";
import waltz from "../../examples/waltz.aratu";

export const examples = [
  { id: "first-piece", text: firstPiece },
  { id: "chance", text: chance },
  { id: "melody", text: melody },
  { id: "patterns", text: patterns },
  { id: "form", text: form },
  { id: "motifs", text: motifs },
  { id: "twelve-tone", text: twelveTone },
  { id: "setlist", text: setlist },
  { id: "waltz", text: waltz },
  { id: "sound", text: sound },
  { id: "sweeps", text: sweeps },
  { id: "trance", text: trance },
  { id: "manguebeat", text: manguebeat },
  { id: "drum-and-bass", text: drumAndBass },
  { id: "g-funk", text: gFunk },
] as const;
