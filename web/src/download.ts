// Downloads of sounds: how long one may take before it counts as failed,
// and what went wrong, in a form the editor can say in each language.

/** How long a sound's download may take before it counts as failed. */
export const DOWNLOAD_LIMIT = 60_000;

export class TimeoutError extends Error {
  constructor(readonly ms: number) {
    super(`no answer after ${Math.round(ms / 1000)} s`);
  }
}

/** `p`, or a TimeoutError once `ms` have gone by without it. */
export function limited<T>(p: Promise<T>, ms = DOWNLOAD_LIMIT): Promise<T> {
  return new Promise((resolve, reject) => {
    const timer = window.setTimeout(() => reject(new TimeoutError(ms)), ms);
    p.then(
      (v) => {
        window.clearTimeout(timer);
        resolve(v);
      },
      (e: unknown) => {
        window.clearTimeout(timer);
        reject(e);
      },
    );
  });
}

/** What went wrong with a download: `timeout:60`, `status:404`, `network`,
 * or `text:` and the error's own words. */
export function detailOf(error: unknown): string {
  if (error instanceof TimeoutError) return `timeout:${Math.round(error.ms / 1000)}`;
  const text = error instanceof Error ? error.message : String(error);
  const status = /\b([45]\d\d)\b/.exec(text);
  if (status) return `status:${status[1]}`;
  if (/failed to fetch|networkerror|network error|load failed/i.test(text)) return "network";
  return `text:${text.slice(0, 160)}`;
}
