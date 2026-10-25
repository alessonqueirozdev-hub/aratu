// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
/*! Aratu, by Alesson Lucas Oliveira de Queiroz (https://aratu.live). Free software under the
 * GNU AGPL, version 3 or later, with the terms in NOTICE.md; source:
 * https://github.com/alessonqueirozdev-hub/aratu */

// Aratu as a library, for a page that holds it: `mount` puts the whole
// editor in an element, `mountPlayer` a piece that only plays. Both draw
// under their own `.aratu-root` (styles in aratu.css), keep nothing but
// what `storage` is given, and take everything with them on `destroy()`.
//
//   import { mount } from "./aratu.js";
//   const editor = mount(document.querySelector("#here"), {
//     lang: "pt",
//     assetBase: "/aratu/",
//     features: { daw: false },
//   });
//   editor.on("change", (code) => save(code));

export { mount, routeOf, ROUTES, VIEWS } from "./app";
export type { AratuEditor, EditorEvents, EditorOptions, Features, View } from "./app";
export { mountPlayer } from "./lite";
export type { AratuPlayer, PlayerEvents, PlayerOptions } from "./lite";
export type { AratuStorage } from "./shared";
export { decodePiece, encodePiece } from "./share";
export { LANGS, type Lang } from "./i18n";
