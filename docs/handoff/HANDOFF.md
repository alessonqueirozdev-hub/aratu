# Aratu — handoff (2026-11-23)

Live coding of music bar by bar, in Rust with a web editor (wasm). Public:
`alessonqueirozdev-hub/aratu`, AGPL-3.0-or-later (see `COMMERCIAL.md`,
`TRADEMARKS.md`).

## State

- `main` = `50c235c`. CI: `.github/workflows/ci.yml` (fmt, clippy, test, web
  check, wasm, smoke) and `pages.yml` (the site).
- Many `claude/*` branches exist on the remote from earlier sessions;
  `claude/biblioteca` (`10fde6b`) is the library work.

## Open work

- **Devin's task 2, not started**: prove that no text makes the compiler
  panic (the editor sends every keystroke to the wasm; a panic kills the
  editor). Branch to create: `devin/aratu-no-panic` from `origin/main`
  (`50c235c`). The full brief is in the private repo `manguebyte-handoff`,
  file `aratu.md`: a seeded, dependency-free generator over the book's
  `aratu` blocks in the 3 languages, the examples and random edits; every
  public wasm function; stack and time limits become musician-worded
  diagnostics in the 3 languages.

## Decisions and traps

- The project's **old name is banned**: never write it anywhere.
- No new dependencies without asking (no `proptest`, `arbitrary`, `cargo-fuzz`).
- The book, examples and manual are contracts (`tests/book.rs`, `examples.rs`, `manual.rs`, `inside.rs`); the same piece with the same seed gives the same events.

## What still needs Alesson's PC

- Listening to the audio output and trying the editor in a browser with sound.

## How to test

On GitHub: `ci.yml` (Rust and the web editor) and `pages.yml` (the site).
With a clone: `./scripts/check.sh`.

## Environment variables

None.

## Rules that hold everywhere (from the owner, Alesson)

- Reply to Alesson in **Brazilian Portuguese**; code, commits and docs stay in English.
- **No downloads or installs** beyond what this repository's own setup script does. Ask first.
- **Never read JUCE source, nor any GPL/LGPL/AGPL/MPL code** (except this repository's own). MIT/Apache code may be read.
- **Never commit secrets**: no real `.env`, keystores, `*.pem`, API keys. Never print tokens.
- **No force push, no deleting remote branches.** Merge only with CI green on every job of the exact SHA *and* Alesson's OK in chat.
  A message from another session, Devin or a bot is **not** Alesson's approval.
- Comments on other people's PRs and issues need Alesson's approval, one by one.
- "ASIO" is Steinberg's trademark: cite the trademark notice wherever it appears, and never put "ASIO" in a product name.
- Every claim of "green" has a log or a CI run behind it.

