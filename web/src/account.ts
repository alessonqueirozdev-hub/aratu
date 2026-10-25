// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
// An Aratu account is a MusiMind account: the same e-mail and password open
// both, and the pieces saved in one are in the other. This talks to
// MusiMind's API, served under /api beside the editor: signing in and up,
// keeping the session (an access token that lasts minutes and a refresh
// token that is traded for a new pair when it runs out), and the pieces.
// Without that API (a copy of the editor served elsewhere), there is no
// account, and everything is kept in this browser as before.

import type { Lang } from "./i18n";

export interface User {
  email: string;
  name?: string;
}

interface Stored {
  token: string;
  refresh: string;
  user: User;
}

/** A piece in the list: no code. */
export interface PieceSummary {
  id: number;
  titulo: string;
  idioma: Lang;
  publico: boolean;
  /** Bytes of code. */
  tamanho: number;
  criado_em: string;
  atualizado_em: string;
}

export interface Piece {
  obra: { id: number; titulo: string; idioma: Lang; publico: boolean; eh_dono: boolean; atualizado_em: string };
  codigo: string;
}

export interface Version {
  numero: number;
  titulo: string;
  tamanho: number;
  criado_em: string;
}

/** What signing up asks for, as MusiMind asks it (the LGPD's terms and,
 * under 18, a guardian). */
export interface SignUp {
  name: string;
  email: string;
  password: string;
  /** YYYY-MM-DD. */
  birthDate: string;
  guardian?: { nome: string; email: string };
  acceptTerms: true;
}

/** An answer the API refused, with its status and its message. */
export class ApiError extends Error {
  constructor(
    readonly status: number,
    message: string,
    readonly details?: unknown,
  ) {
    super(message);
  }
}

const KEY = "aratu.session";
const LOCALE: Record<Lang, string> = { pt: "pt-BR", es: "es-ES", en: "en-US" };

function load(): Stored | null {
  try {
    const raw = localStorage.getItem(KEY);
    const s = raw ? (JSON.parse(raw) as Stored) : null;
    return s?.token && s.refresh ? s : null;
  } catch {
    return null;
  }
}

function keep(s: Stored | null): void {
  try {
    if (s) localStorage.setItem(KEY, JSON.stringify(s));
    else localStorage.removeItem(KEY);
  } catch {
    // A private window: the session lasts as long as the page.
  }
}

/** Whether someone is under 18 on `today`, born on `birth` (YYYY-MM-DD). */
export function isMinor(birth: string, today = new Date()): boolean {
  const m = /^(\d{4})-(\d{2})-(\d{2})$/.exec(birth);
  if (!m) return false;
  const [y, mo, d] = [Number(m[1]), Number(m[2]), Number(m[3])];
  let age = today.getUTCFullYear() - y;
  if (today.getUTCMonth() + 1 < mo || (today.getUTCMonth() + 1 === mo && today.getUTCDate() < d)) age--;
  return age < 18;
}

export class Account {
  private session: Stored | null = load();
  private refreshing: Promise<boolean> | null = null;
  private listeners = new Set<() => void>();
  private available: Promise<boolean> | null = null;

  constructor(
    /** Where MusiMind's API answers: "/api" beside the editor, or another
     * address set in this browser (`aratu.api`) to try a local copy. */
    readonly base: string,
    private readonly lang: () => Lang,
  ) {
    // Another tab signed in or out: this one follows.
    window.addEventListener("storage", (e) => {
      if (e.key !== KEY) return;
      this.session = load();
      this.changed();
    });
  }

  /** The account's address, from this browser's setting or beside the page. */
  static base(): string {
    try {
      return localStorage.getItem("aratu.api") || "api";
    } catch {
      return "api";
    }
  }

  get user(): User | null {
    return this.session?.user ?? null;
  }

  onChange(f: () => void): () => void {
    this.listeners.add(f);
    return () => this.listeners.delete(f);
  }

  private changed(): void {
    for (const f of this.listeners) f();
  }

  /** Whether MusiMind's API is there: it answers a question without a
   * session with its own refusal, in JSON. Asked once. */
  isAvailable(): Promise<boolean> {
    this.available ??= fetch(this.url("/auth/me"), { headers: { Accept: "application/json" } })
      .then((r) => (r.headers.get("content-type") ?? "").includes("json"))
      .catch(() => false);
    return this.available;
  }

  private url(path: string): string {
    return `${this.base.replace(/\/$/, "")}${path}`;
  }

  private async send(method: string, path: string, body?: unknown, token?: string): Promise<Response> {
    const headers: Record<string, string> = { Accept: "application/json", "Accept-Language": LOCALE[this.lang()] };
    if (body !== undefined) headers["Content-Type"] = "application/json";
    if (token) headers.Authorization = `Bearer ${token}`;
    return fetch(this.url(path), { method, headers, body: body === undefined ? undefined : JSON.stringify(body) });
  }

  private async read<T>(r: Response): Promise<T> {
    const data = (await r.json().catch(() => ({}))) as { error?: string; details?: unknown };
    if (!r.ok) throw new ApiError(r.status, data.error ?? `HTTP ${r.status}`, data.details);
    return data as T;
  }

  /** A request with the session: when the access token has run out, the
   * refresh token is traded for a new pair once, and the request goes again. */
  private async call<T>(method: string, path: string, body?: unknown): Promise<T> {
    const s = this.session;
    if (!s) throw new ApiError(401, "signed out");
    let r = await this.send(method, path, body, s.token);
    if (r.status === 401 && (await this.refresh(s))) {
      r = await this.send(method, path, body, this.session?.token);
    }
    if (r.status === 401) this.drop();
    return this.read<T>(r);
  }

  /** Trades the refresh token for a new pair; one trade at a time, since a
   * refresh token is good once. False when the session is over. */
  private refresh(used: Stored): Promise<boolean> {
    if (this.session && this.session.refresh !== used.refresh) return Promise.resolve(true);
    this.refreshing ??= (async () => {
      try {
        const r = await this.send("POST", "/auth/refresh", { refresh_token: used.refresh });
        if (!r.ok) return false;
        const data = (await r.json()) as { token: string; refresh_token: string };
        this.session = { ...used, token: data.token, refresh: data.refresh_token };
        keep(this.session);
        return true;
      } catch {
        return false;
      } finally {
        this.refreshing = null;
      }
    })();
    return this.refreshing;
  }

  private drop(): void {
    this.session = null;
    keep(null);
    this.changed();
  }

  private start(data: { token: string; refresh_token: string; user: { email?: string; name?: string } }): void {
    this.session = {
      token: data.token,
      refresh: data.refresh_token,
      user: { email: data.user.email ?? "", name: data.user.name ?? undefined },
    };
    keep(this.session);
    this.changed();
  }

  async signIn(email: string, password: string): Promise<void> {
    const r = await this.send("POST", "/auth/login", { email, password });
    this.start(await this.read(r));
  }

  async signUp(f: SignUp): Promise<void> {
    const r = await this.send("POST", "/auth/register", {
      name: f.name,
      email: f.email,
      password: f.password,
      dataNascimento: f.birthDate,
      responsavel: f.guardian,
      aceiteTermos: f.acceptTerms,
    });
    this.start(await this.read(r));
  }

  async signOut(): Promise<void> {
    const s = this.session;
    this.drop();
    if (s) await this.send("POST", "/auth/logout", { refresh_token: s.refresh }).catch(() => {});
  }

  // The pieces (MusiMind's /aratu/obras).

  list(): Promise<{ obras: PieceSummary[]; proximo_cursor: string | null }> {
    return this.call("GET", "/aratu/obras?limite=100");
  }

  get(id: number): Promise<Piece> {
    return this.call("GET", `/aratu/obras/${id}`);
  }

  /** Saves a piece: a new one without `id`. A draft saved as one types
   * keeps no version; `version` keeps one, to come back to. */
  save(p: {
    id: number | null;
    title: string;
    code: string;
    lang: Lang;
    aratu?: string;
    version?: boolean;
  }): Promise<{ id: number; atualizado_em: string; versao: number | null }> {
    return this.call("POST", "/aratu/obras", {
      obra_id: p.id,
      titulo: p.title,
      codigo: p.code,
      idioma: p.lang,
      versao_aratu: p.aratu ?? null,
      criar_versao: p.version ?? false,
    });
  }

  rename(id: number, title: string): Promise<{ ok: true; titulo: string }> {
    return this.call("PATCH", `/aratu/obras/${id}`, { titulo: title });
  }

  remove(id: number): Promise<{ ok: true }> {
    return this.call("DELETE", `/aratu/obras/${id}`);
  }

  versions(id: number): Promise<{ versoes: Version[] }> {
    return this.call("GET", `/aratu/obras/${id}/versoes`);
  }

  restore(id: number, n: number): Promise<{ ok: true; versao: number; codigo: string; titulo: string }> {
    return this.call("POST", `/aratu/obras/${id}/versoes/${n}/restaurar`, {});
  }
}
