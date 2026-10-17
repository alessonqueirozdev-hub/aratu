// A piece in a link: its text, compressed when the browser can, in the part
// of the address after `#`, so the link works on any static host and the
// text never reaches a server. `#/tocar?m=` and the code.

const toBase64Url = (bytes: Uint8Array): string => {
  let s = "";
  for (let i = 0; i < bytes.length; i += 0x8000) s += String.fromCharCode(...bytes.subarray(i, i + 0x8000));
  return btoa(s).replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/, "");
};

const fromBase64Url = (code: string): Uint8Array => {
  const b64 = code.replace(/-/g, "+").replace(/_/g, "/");
  const s = atob(b64 + "=".repeat((4 - (b64.length % 4)) % 4));
  return Uint8Array.from(s, (c) => c.charCodeAt(0));
};

/** Bytes through a (de)compression stream. */
const through = async (bytes: Uint8Array, stream: CompressionStream | DecompressionStream): Promise<Uint8Array> => {
  const pair = stream as unknown as ReadableWritablePair<Uint8Array, Uint8Array>;
  const source = new Blob([bytes.slice() as Uint8Array<ArrayBuffer>]).stream();
  return new Uint8Array(await new Response(source.pipeThrough(pair)).arrayBuffer());
};

/** The code for a piece: `z` and the deflated text, or `t` and the text. */
export async function encodePiece(text: string): Promise<string> {
  const bytes = new TextEncoder().encode(text);
  if (typeof CompressionStream === "function") {
    try {
      return "z" + toBase64Url(await through(bytes, new CompressionStream("deflate-raw")));
    } catch {
      // Fall through to the plain text.
    }
  }
  return "t" + toBase64Url(bytes);
}

/** The piece a code holds, or null when it cannot be read. */
export async function decodePiece(code: string): Promise<string | null> {
  try {
    const bytes = fromBase64Url(code.slice(1));
    if (code[0] === "t") return new TextDecoder().decode(bytes);
    if (code[0] === "z" && typeof DecompressionStream === "function") {
      return new TextDecoder().decode(await through(bytes, new DecompressionStream("deflate-raw")));
    }
  } catch {
    // A link cut short or mistyped.
  }
  return null;
}

/** The address that opens `code` in the editor, here. */
export function linkFor(code: string): string {
  return `${location.origin}${location.pathname}#/tocar?m=${code}`;
}

/** The piece's code in the address, if the address carries one. */
export function codeInAddress(): string | null {
  const query = location.hash.split("?")[1];
  return query ? new URLSearchParams(query).get("m") : null;
}
