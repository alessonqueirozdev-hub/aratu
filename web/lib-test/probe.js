// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Alesson Lucas Oliveira de Queiroz. Terms: LICENSE and NOTICE.md.
// Runs before anything else on the test page: counts the listeners on the
// window, the document and media queries, the AudioContexts made and
// closed, the worklets loaded, every fetch, and what the page's Content
// Security Policy refused, for test.js to compare before and after.
(() => {
  const violations = [];
  document.addEventListener("securitypolicyviolation", (e) =>
    violations.push(`${e.violatedDirective} ${e.blockedURI || e.sample || ""}`.trim()),
  );

  const where = (t) =>
    t === window ? "window" : t === document ? "document" : t instanceof MediaQueryList ? "media" : null;
  const ids = new WeakMap();
  let next = 1;
  const idOf = (f) => {
    if (!ids.has(f)) ids.set(f, next++);
    return ids.get(f);
  };
  const capture = (o) => (typeof o === "boolean" ? o : !!o?.capture);
  /** The listeners on the window, the document and media queries now. */
  const listeners = new Set();
  const add = EventTarget.prototype.addEventListener;
  const remove = EventTarget.prototype.removeEventListener;
  EventTarget.prototype.addEventListener = function (type, f, o) {
    const w = where(this);
    if (w && f) listeners.add(`${w} ${type} ${capture(o)} #${idOf(f)}`);
    return add.call(this, type, f, o);
  };
  EventTarget.prototype.removeEventListener = function (type, f, o) {
    const w = where(this);
    if (w && f) listeners.delete(`${w} ${type} ${capture(o)} #${idOf(f)}`);
    return remove.call(this, type, f, o);
  };

  const contexts = [];
  const Context = window.AudioContext;
  window.AudioContext = class extends Context {
    constructor(...args) {
      super(...args);
      contexts.push(this);
    }
  };
  const worklets = [];
  const addModule = AudioWorklet.prototype.addModule;
  AudioWorklet.prototype.addModule = function (url, o) {
    worklets.push(new URL(String(url), document.baseURI).href);
    return addModule.call(this, url, o);
  };
  const fetches = [];
  const fetch0 = window.fetch;
  window.fetch = function (input, init) {
    fetches.push(new URL(String(input instanceof Request ? input.url : input), document.baseURI).href);
    return fetch0.call(this, input, init);
  };

  const codemirror = (e) => e?.tagName === "STYLE" && e.textContent.includes("ͼ");

  window.__probe = {
    violations,
    listeners,
    contexts,
    worklets,
    fetches,
    /** What the page around the editor is, to see it unchanged. */
    page: () => ({
      href: location.href,
      history: history.length,
      title: document.title,
      html: [...document.documentElement.attributes].map((a) => `${a.name}=${a.value}`).join(" "),
      body: [...document.body.attributes].map((a) => `${a.name}=${a.value}`).join(" "),
      bodyChildren: document.body.children.length,
      // CodeMirror's one <style> for the document, shared by every editor
      // on it (its rules are only for its own generated class names), is
      // not the editor's to take away.
      headChildren: [...document.head.children].filter((e) => !codemirror(e)).length,
      sheets: [...document.styleSheets].filter((s) => !codemirror(s.ownerNode)).length,
    }),
  };

  // Which styles: none (a clean page), the host's reset and rules with
  // aratu.css, or all of it in cascade layers; and aratu.css after the
  // host's or before.
  const q = new URLSearchParams(location.search);
  const css = q.get("css") ?? "plain";
  const before = q.get("order") === "before";
  const link = (href) => {
    const l = document.createElement("link");
    l.rel = "stylesheet";
    l.href = href;
    document.head.append(l);
  };
  const aratu = css === "layered" ? "/static/aratu/v1/aratu.layered.css" : "/static/aratu/v1/aratu.css";
  if (css === "layered") link("/lib-test/layers.css");
  link("/lib-test/slots.css");
  if (before) link(aratu);
  if (css === "plain") ["/lib-test/preflight.css", "/lib-test/host.css"].forEach(link);
  if (css === "layered") ["/lib-test/preflight.layered.css", "/lib-test/host.layered.css"].forEach(link);
  if (!before) link(aratu);
})();
