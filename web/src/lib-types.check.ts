// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
// The types written for the library (lib/aratu.d.ts) are those of the code:
// `npm run check` fails here when one changes without the other. Nothing
// imports this file.

import type * as Declared from "../lib/aratu";
import type * as Real from "./lib";

/** True when each type can stand for the other. */
type Same<A, B> = [A] extends [B] ? ([B] extends [A] ? true : false) : false;
type Assert<T extends true> = T;

export type Checks = [
  Assert<Same<typeof Declared.mount, typeof Real.mount>>,
  Assert<Same<typeof Declared.mountPlayer, typeof Real.mountPlayer>>,
  Assert<Same<typeof Declared.routeOf, typeof Real.routeOf>>,
  Assert<Same<typeof Declared.ROUTES, typeof Real.ROUTES>>,
  Assert<Same<typeof Declared.VIEWS, typeof Real.VIEWS>>,
  Assert<Same<typeof Declared.LANGS, typeof Real.LANGS>>,
  Assert<Same<typeof Declared.encodePiece, typeof Real.encodePiece>>,
  Assert<Same<typeof Declared.decodePiece, typeof Real.decodePiece>>,
  Assert<Same<Declared.EditorOptions, Real.EditorOptions>>,
  Assert<Same<Declared.PlayerOptions, Real.PlayerOptions>>,
  Assert<Same<Declared.AratuEditor, Real.AratuEditor>>,
  Assert<Same<Declared.AratuPlayer, Real.AratuPlayer>>,
  Assert<Same<Declared.EditorEvents, Real.EditorEvents>>,
  Assert<Same<Declared.Features, Real.Features>>,
  Assert<Same<Declared.AratuStorage, Real.AratuStorage>>,
  Assert<Same<Declared.View, Real.View>>,
  Assert<Same<Declared.Lang, Real.Lang>>,
];
