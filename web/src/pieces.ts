// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
// The account in the editor: the button at the top that signs in or up with a
// MusiMind account, and, once in, the pieces kept there. The piece in the
// editor either belongs to the account, and is saved as it is typed (a
// draft, no version) and as a version when asked, or is only in this
// browser until "Save to my account". Opening an example, a shared link or a
// piece from the book takes the editor away from the account's piece, so
// nothing is saved over it.

import { Account, ApiError, type PieceSummary, type Version, isMinor } from "./account";
import { type Lang, type Word, tr } from "./i18n";

export interface PiecesOptions {
  button: HTMLButtonElement;
  account: Account;
  lang: () => Lang;
  /** The text in the editor, and putting another there (as an edit). */
  text: () => string;
  open: (text: string) => void;
  /** Texts that are not anyone's piece (the examples), so opening another
   * piece over them needs no asking. */
  spare: (text: string) => boolean;
}

interface Current {
  id: number | null;
  title: string;
}

const KEY = "aratu.piece";
const SAVE_AFTER = 1200;

function load(): Current {
  try {
    const c = JSON.parse(localStorage.getItem(KEY) ?? "null") as Current | null;
    if (c && (c.id === null || Number.isInteger(c.id))) return c;
  } catch {
    // Nothing kept, or a private window.
  }
  return { id: null, title: "" };
}

function keep(c: Current): void {
  try {
    localStorage.setItem(KEY, JSON.stringify(c));
  } catch {
    // A private window: the open piece is forgotten on reload.
  }
}

function el<K extends keyof HTMLElementTagNameMap>(tag: K, cls?: string, text?: string): HTMLElementTagNameMap[K] {
  const e = document.createElement(tag);
  if (cls) e.className = cls;
  if (text !== undefined) e.textContent = text;
  return e;
}

/** A title from the piece: its first comment, or "Untitled". */
export function titleOf(code: string, untitled: string): string {
  const line = code
    .split("\n")
    .map((l) => /^\s*\/\/\s*(.+)$/.exec(l)?.[1]?.trim() ?? "")
    .find((l) => l.length > 0);
  const t = (line ?? untitled).replace(/[.:,;]+$/, "");
  return t.length > 120 ? `${t.slice(0, 117).trimEnd()}…` : t;
}

export class Pieces {
  private current = load();
  private panel = el("div", "acct-panel");
  private tab: "in" | "up" = "in";
  private list: PieceSummary[] | null = null;
  private versions: Version[] | null = null;
  private status: { word: Word; detail?: string } | null = null;
  private timer = 0;
  private saving = false;
  private dirty = false;
  private error = "";

  constructor(private readonly o: PiecesOptions) {
    this.panel.hidden = true;
    this.panel.setAttribute("role", "dialog");
    document.body.append(this.panel);
    o.button.hidden = true;
    o.button.addEventListener("click", () => this.toggle());
    document.addEventListener("pointerdown", (e) => {
      const t = e.target as Node;
      if (!this.panel.hidden && !this.panel.contains(t) && !o.button.contains(t)) this.panel.hidden = true;
    });
    document.addEventListener("keydown", (e) => {
      if (e.key === "Escape" && !this.panel.hidden) this.panel.hidden = true;
    });
    window.addEventListener("resize", () => !this.panel.hidden && this.place());
    o.account.onChange(() => {
      this.list = null;
      this.versions = null;
      this.status = null;
      if (!o.account.user) this.current = { id: null, title: "" };
      keep(this.current);
      this.render();
    });
    void o.account.isAvailable().then((yes) => {
      o.button.hidden = !yes;
      this.render();
    });
    this.render();
  }

  private t(word: Word): string {
    return tr(word, this.o.lang());
  }

  /** The language changed: the words follow. */
  relabel(): void {
    this.render();
  }

  // The piece in the editor.

  /** The editor's text changed: an account's piece is saved shortly after. */
  edited(): void {
    if (this.current.id === null || !this.o.account.user) return;
    this.dirty = true;
    this.status = { word: "acctUnsaved" };
    this.renderStatus();
    window.clearTimeout(this.timer);
    this.timer = window.setTimeout(() => void this.save(false), SAVE_AFTER);
  }

  /** Something else is put in the editor (an example, a shared link, a
   * piece from the book): it is not the account's piece any more. */
  detach(): void {
    window.clearTimeout(this.timer);
    if (this.dirty && this.current.id !== null) void this.save(false);
    this.current = { id: null, title: "" };
    this.versions = null;
    this.status = null;
    keep(this.current);
    this.render();
  }

  private async save(version: boolean): Promise<void> {
    const account = this.o.account;
    if (!account.user || this.saving) {
      if (this.saving) this.timer = window.setTimeout(() => void this.save(version), SAVE_AFTER);
      return;
    }
    this.saving = true;
    this.dirty = false;
    this.status = { word: "acctSaving" };
    this.renderStatus();
    const code = this.o.text();
    try {
      const title = this.current.title || titleOf(code, this.t("acctUntitled"));
      const r = await account.save({ id: this.current.id, title, code, lang: this.o.lang(), version });
      this.current = { id: r.id, title };
      keep(this.current);
      this.status = { word: version && r.versao ? "acctVersionSaved" : "acctSaved", detail: r.versao ? String(r.versao) : undefined };
      if (version) this.versions = null;
      if (this.panel.hidden) this.list = null;
      else void this.loadList();
    } catch (e) {
      this.dirty = true;
      this.status = { word: "acctNotSaved", detail: this.why(e) };
    } finally {
      this.saving = false;
    }
    this.render();
  }

  /** What went wrong, in words. */
  private why(e: unknown): string {
    if (!(e instanceof ApiError)) return this.t("acctOffline");
    const word: Partial<Record<number, Word>> = {
      401: "acctExpired",
      409: "acctLimit",
      413: "acctTooLarge",
      422: "acctRefused",
    };
    const w = word[e.status];
    return w ? this.t(w) : e.message;
  }

  // The panel.

  private toggle(): void {
    this.panel.hidden = !this.panel.hidden;
    if (this.panel.hidden) return;
    this.render();
    this.place();
    if (this.o.account.user && this.list === null) void this.loadList();
  }

  private place(): void {
    const r = this.o.button.getBoundingClientRect();
    this.panel.style.top = `${r.bottom + window.scrollY + 8}px`;
    const left = Math.min(window.innerWidth - this.panel.offsetWidth - 8, r.right - this.panel.offsetWidth);
    this.panel.style.left = `${Math.max(8, left)}px`;
  }

  private async loadList(): Promise<void> {
    try {
      this.list = (await this.o.account.list()).obras;
      this.error = "";
    } catch (e) {
      this.error = this.why(e);
    }
    this.render();
  }

  private render(): void {
    const user = this.o.account.user;
    const b = this.o.button;
    b.textContent = user ? (user.name || user.email).split(/[\s@]/)[0] : this.t("acctSignIn");
    b.classList.toggle("on", !!user);
    b.title = user ? `${user.name ?? ""} ${user.email}`.trim() : this.t("acctSignInHint");
    this.panel.replaceChildren(...(user ? this.signedIn() : this.signedOut()));
    if (!this.panel.hidden) this.place();
  }

  private renderStatus(): void {
    const s = this.panel.querySelector(".acct-status");
    if (s) s.textContent = this.statusText();
  }

  private statusText(): string {
    if (!this.status) return "";
    const text = this.t(this.status.word);
    return this.status.detail ? text.replace("{n}", this.status.detail).replace("{why}", this.status.detail) : text;
  }

  private signedOut(): HTMLElement[] {
    const tabs = el("div", "seg acct-tabs");
    for (const [id, word] of [
      ["in", "acctSignIn"],
      ["up", "acctSignUp"],
    ] as const) {
      const t = el("button", this.tab === id ? "on" : "", this.t(word));
      t.type = "button";
      t.onclick = () => {
        this.tab = id;
        this.error = "";
        this.render();
      };
      tabs.append(t);
    }
    const about = el("p", "acct-about", this.t("acctAbout"));
    return [el("h3", "", this.t("acctTitle")), about, tabs, this.tab === "in" ? this.signInForm() : this.signUpForm()];
  }

  private field(label: Word, input: HTMLInputElement): HTMLLabelElement {
    const l = el("label", "acct-field");
    l.append(el("span", "", this.t(label)), input);
    return l;
  }

  private input(type: string, name: string, autocomplete: string): HTMLInputElement {
    const i = el("input");
    i.type = type;
    i.name = name;
    i.autocomplete = autocomplete as AutoFill;
    i.required = true;
    return i;
  }

  private errorLine(): HTMLElement {
    const p = el("p", "acct-error", this.error);
    p.setAttribute("role", "alert");
    p.hidden = !this.error;
    return p;
  }

  private signInForm(): HTMLFormElement {
    const f = el("form", "acct-form");
    const email = this.input("email", "email", "email");
    const password = this.input("password", "password", "current-password");
    const go = el("button", "btn acct-go", this.t("acctSignIn"));
    go.type = "submit";
    const forgot = el("a", "acct-link", this.t("acctForgot"));
    forgot.href = "https://musimind.com.br/forgot-password";
    forgot.target = "_blank";
    forgot.rel = "noopener";
    f.append(this.field("acctEmail", email), this.field("acctPassword", password), this.errorLine(), go, forgot);
    f.onsubmit = async (e) => {
      e.preventDefault();
      go.disabled = true;
      try {
        await this.o.account.signIn(email.value.trim(), password.value);
        this.error = "";
        void this.loadList();
      } catch (err) {
        this.error = err instanceof ApiError ? err.message : this.t("acctOffline");
        this.render();
      }
    };
    return f;
  }

  private signUpForm(): HTMLFormElement {
    const f = el("form", "acct-form");
    const name = this.input("text", "name", "name");
    const email = this.input("email", "email", "email");
    const password = this.input("password", "new-password", "new-password");
    password.minLength = 8;
    const birth = this.input("date", "birth", "bday");
    birth.max = new Date().toISOString().slice(0, 10);
    const guardian = el("fieldset", "acct-guardian");
    guardian.append(el("legend", "", this.t("acctGuardian")));
    const gName = this.input("text", "guardian-name", "off");
    const gEmail = this.input("email", "guardian-email", "off");
    guardian.append(this.field("acctGuardianName", gName), this.field("acctGuardianEmail", gEmail));
    const minor = () => isMinor(birth.value);
    const showGuardian = () => {
      guardian.hidden = !minor();
      gName.required = gEmail.required = minor();
    };
    birth.addEventListener("change", showGuardian);
    showGuardian();
    const terms = el("label", "acct-terms");
    const accept = this.input("checkbox", "terms", "off");
    const words = el("span");
    const link = (word: Word, href: string) => {
      const a = el("a", "", this.t(word));
      a.href = href;
      a.target = "_blank";
      a.rel = "noopener";
      return a;
    };
    const [before, middle, after] = this.t("acctTerms").split(/\{terms\}|\{privacy\}/);
    words.append(before ?? "", link("acctTermsLink", "https://musimind.com.br/termos"), middle ?? "", link("acctPrivacyLink", "https://musimind.com.br/privacidade"), after ?? "");
    terms.append(accept, words);
    const go = el("button", "btn acct-go", this.t("acctCreate"));
    go.type = "submit";
    f.append(
      this.field("acctName", name),
      this.field("acctEmail", email),
      this.field("acctNewPassword", password),
      this.field("acctBirth", birth),
      guardian,
      terms,
      this.errorLine(),
      go,
    );
    f.onsubmit = async (e) => {
      e.preventDefault();
      go.disabled = true;
      try {
        await this.o.account.signUp({
          name: name.value.trim(),
          email: email.value.trim(),
          password: password.value,
          birthDate: birth.value,
          guardian: minor() ? { nome: gName.value.trim(), email: gEmail.value.trim() } : undefined,
          acceptTerms: true,
        });
        this.error = "";
        void this.loadList();
      } catch (err) {
        this.error = err instanceof ApiError ? this.details(err) : this.t("acctOffline");
        this.render();
      }
    };
    return f;
  }

  /** The first reason the API gave for each field, or its message. */
  private details(e: ApiError): string {
    const d = e.details as { fieldErrors?: Record<string, string[]>; formErrors?: string[] } | undefined;
    const all = [...(d?.formErrors ?? []), ...Object.values(d?.fieldErrors ?? {}).map((errs) => errs[0] ?? "")];
    return all.filter(Boolean).join(" ") || e.message;
  }

  private signedIn(): HTMLElement[] {
    const user = this.o.account.user!;
    const head = el("div", "acct-head");
    const who = el("div", "acct-who");
    who.append(el("strong", "", user.name || user.email), el("span", "", user.email));
    const out = el("button", "btn acct-out", this.t("acctSignOut"));
    out.type = "button";
    out.onclick = () => void this.o.account.signOut();
    head.append(who, out);

    // The piece in the editor.
    const here = el("section", "acct-here");
    here.append(el("h4", "", this.t("acctThisPiece")));
    if (this.current.id === null) {
      here.append(el("p", "acct-note", this.t("acctNotInAccount")));
      const keepIt = el("button", "btn acct-go", this.t("acctSaveToAccount"));
      keepIt.type = "button";
      keepIt.onclick = () => void this.save(true);
      here.append(keepIt);
    } else {
      const title = el("input", "acct-title");
      title.value = this.current.title;
      title.maxLength = 120;
      title.setAttribute("aria-label", this.t("acctTitleLabel"));
      title.onchange = async () => {
        const t = title.value.trim();
        if (!t || this.current.id === null) return void (title.value = this.current.title);
        try {
          await this.o.account.rename(this.current.id, t);
          this.current = { ...this.current, title: t };
          keep(this.current);
          this.list = null;
          void this.loadList();
        } catch (e) {
          this.status = { word: "acctNotSaved", detail: this.why(e) };
          this.renderStatus();
        }
      };
      const row = el("div", "acct-row");
      const version = el("button", "btn", this.t("acctSaveVersion"));
      version.type = "button";
      version.onclick = () => {
        window.clearTimeout(this.timer);
        void this.save(true);
      };
      const fresh = el("button", "btn", this.t("acctNew"));
      fresh.type = "button";
      fresh.onclick = () => this.detach();
      const history = el("button", "btn", this.t("acctVersions"));
      history.type = "button";
      history.onclick = () => void this.showVersions();
      row.append(version, history, fresh);
      here.append(title, row);
      if (this.versions) here.append(this.versionList());
    }
    const status = el("p", "acct-status", this.statusText());
    status.setAttribute("role", "status");
    here.append(status);

    // The account's pieces.
    const mine = el("section", "acct-mine");
    mine.append(el("h4", "", this.t("acctMine")));
    if (this.error) mine.append(this.errorLine());
    if (this.list === null && !this.error) mine.append(el("p", "acct-note", this.t("acctLoading")));
    else if (this.list?.length === 0) mine.append(el("p", "acct-note", this.t("acctNone")));
    const ul = el("ul", "acct-list");
    for (const p of this.list ?? []) ul.append(this.row(p));
    mine.append(ul);
    return [head, here, mine];
  }

  private row(p: PieceSummary): HTMLLIElement {
    const li = el("li", p.id === this.current.id ? "on" : "");
    const open = el("button", "acct-open");
    open.type = "button";
    const when = new Date(p.atualizado_em).toLocaleDateString(this.o.lang(), { day: "numeric", month: "short", year: "numeric" });
    open.append(el("span", "acct-name", p.titulo), el("span", "acct-when", when));
    open.onclick = () => void this.openPiece(p.id);
    const del = el("button", "acct-del", "×");
    del.type = "button";
    del.title = this.t("acctDelete");
    del.setAttribute("aria-label", `${this.t("acctDelete")}: ${p.titulo}`);
    del.onclick = async () => {
      if (!window.confirm(this.t("acctDeleteAsk").replace("{title}", p.titulo))) return;
      try {
        await this.o.account.remove(p.id);
        if (p.id === this.current.id) this.detach();
        void this.loadList();
      } catch (e) {
        this.error = this.why(e);
        this.render();
      }
    };
    li.append(open, del);
    return li;
  }

  private async openPiece(id: number): Promise<void> {
    if (id === this.current.id) return;
    const text = this.o.text();
    if (this.current.id === null && text.trim() && !this.o.spare(text) && !window.confirm(this.t("acctReplaceAsk"))) return;
    try {
      const p = await this.o.account.get(id);
      this.detach();
      this.o.open(p.codigo);
      this.current = { id: p.obra.id, title: p.obra.titulo };
      keep(this.current);
      this.dirty = false;
      this.status = { word: "acctOpened" };
      this.panel.hidden = true;
    } catch (e) {
      this.error = this.why(e);
    }
    this.render();
  }

  private async showVersions(): Promise<void> {
    if (this.current.id === null) return;
    if (this.versions) {
      this.versions = null;
      return this.render();
    }
    try {
      this.versions = (await this.o.account.versions(this.current.id)).versoes;
    } catch (e) {
      this.status = { word: "acctNotSaved", detail: this.why(e) };
    }
    this.render();
  }

  private versionList(): HTMLElement {
    const ul = el("ul", "acct-list acct-versions");
    if (this.versions?.length === 0) ul.append(el("li", "acct-note", this.t("acctNoVersions")));
    for (const v of this.versions ?? []) {
      const li = el("li");
      const when = new Date(v.criado_em).toLocaleString(this.o.lang(), { dateStyle: "short", timeStyle: "short" });
      const back = el("button", "acct-open");
      back.type = "button";
      back.append(el("span", "acct-name", `${this.t("acctVersion")} ${v.numero}`), el("span", "acct-when", when));
      back.title = this.t("acctRestore");
      back.onclick = async () => {
        const id = this.current.id;
        if (id === null || !window.confirm(this.t("acctRestoreAsk").replace("{n}", String(v.numero)))) return;
        // What the editor has and the account not yet goes first, so the
        // restore keeps it as a version.
        window.clearTimeout(this.timer);
        if (this.dirty) await this.save(false);
        try {
          const r = await this.o.account.restore(id, v.numero);
          window.clearTimeout(this.timer);
          this.o.open(r.codigo);
          // Putting it in the editor saved nothing yet: the restore did.
          window.clearTimeout(this.timer);
          this.dirty = false;
          this.current = { id, title: r.titulo };
          keep(this.current);
          this.versions = null;
          this.status = { word: "acctRestored", detail: String(v.numero) };
        } catch (e) {
          this.status = { word: "acctNotSaved", detail: this.why(e) };
        }
        this.render();
      };
      li.append(back);
      ul.append(li);
    }
    return ul;
  }
}
