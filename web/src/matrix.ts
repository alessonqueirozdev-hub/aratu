// The twelve-tone matrix of each row in the piece: the rows of the matrix
// are the forms P (read left to right) and R (right to left); the columns
// are I (top to bottom) and RI (bottom to top). A click on a form's name
// writes it into the piece.

import type { Program } from "./aratu";
import { type Lang, tr } from "./i18n";

const LETTERS = ["C", "C#", "D", "Eb", "E", "F", "F#", "G", "Ab", "A", "Bb", "B"];
const SOLFEGE_PT = ["dó", "dó#", "ré", "mib", "mi", "fá", "fá#", "sol", "láb", "lá", "sib", "si"];
const SOLFEGE_ES = ["do", "do#", "re", "mib", "mi", "fa", "fa#", "sol", "lab", "la", "sib", "si"];

const el = <K extends keyof HTMLElementTagNameMap>(tag: K, cls?: string, text?: string) => {
  const e = document.createElement(tag);
  if (cls) e.className = cls;
  if (text !== undefined) e.textContent = text;
  return e;
};

export class RowMatrix {
  /** Note names or numbers 0–11. */
  private numbers = false;

  constructor(
    private readonly box: HTMLElement,
    private readonly o: { lang: () => Lang; insert: (text: string) => void },
  ) {}

  render(program: Program | null): void {
    const lang = this.o.lang();
    this.box.innerHTML = "";
    const rows = program?.rows ?? [];
    const head = el("div", "matrix-head");
    head.append(el("p", "sound-help", tr("rowsHelp", lang)));
    const toggle = el("button", "matrix-toggle", this.numbers ? tr("rowsNames", lang) : tr("rowsNumbers", lang));
    toggle.onclick = () => {
      this.numbers = !this.numbers;
      this.render(program);
    };
    if (rows.length) head.append(toggle);
    this.box.append(head);
    if (rows.length === 0) {
      this.box.append(el("p", "sound-none", tr("rowsNone", lang)));
      return;
    }
    for (const row of rows) this.box.append(this.matrix(row.name, row.pitches));
  }

  private name(pc: number): string {
    if (this.numbers) return String(pc);
    const lang = this.o.lang();
    return (lang === "pt" ? SOLFEGE_PT : lang === "es" ? SOLFEGE_ES : LETTERS)[pc];
  }

  private matrix(name: string, pitches: number[]): HTMLElement {
    const n = pitches.length;
    const pcs = pitches.map((p) => ((p % 12) + 12) % 12);
    const first = pcs[0] ?? 0;
    const mod = (x: number) => ((x % 12) + 12) % 12;
    const wrap = el("div", "matrix");
    wrap.append(el("h3", "sound-family", name));
    const table = el("div", "matrix-grid");
    table.style.setProperty("--n", String(n));
    const form = (label: string) => {
      const b = el("button", "matrix-form", label);
      b.title = `${name}.${label}`;
      b.onclick = () => this.o.insert(`${name}.${label}`);
      return b;
    };
    // The I forms along the top, the RI forms along the bottom.
    const columns = pcs.map((p) => mod(p - first));
    table.append(el("span", "matrix-corner"));
    for (const c of columns) table.append(form(`I${c}`));
    table.append(el("span", "matrix-corner"));
    pcs.forEach((p, i) => {
      // Row i of the matrix is the prime form starting on the i-th note of
      // the inversion.
      const t = mod(first - p);
      table.append(form(`P${t}`));
      for (let j = 0; j < n; j++) {
        const cell = el("span", "matrix-cell", this.name(mod(pcs[j] + t)));
        if (i === 0 || j === 0) cell.classList.add("prime");
        table.append(cell);
      }
      table.append(form(`R${t}`));
    });
    table.append(el("span", "matrix-corner"));
    for (const c of columns) table.append(form(`RI${c}`));
    table.append(el("span", "matrix-corner"));
    wrap.append(table);
    return wrap;
  }
}
