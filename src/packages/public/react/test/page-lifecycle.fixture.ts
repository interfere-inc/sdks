import {
  onPageHidden,
  onPageShown,
  resetPageLifecycleForTests,
} from "../src/internal/page-lifecycle/page-lifecycle.js";

const calls: { name: string; state: DocumentVisibilityState }[] = [];
const lifecycle: { type: string; persisted: boolean }[] = [];

window.addEventListener("pageshow", (event) => {
  lifecycle.push({ type: "pageshow", persisted: event.persisted });
});

window.addEventListener("pagehide", (event) => {
  lifecycle.push({ type: "pagehide", persisted: event.persisted });
});

const fixture = {
  calls,
  lifecycle,
  onPageHidden,
  onPageShown,
  reset: resetPageLifecycleForTests,
  record(name: string) {
    calls.push({ name, state: document.visibilityState });
  },
};

declare global {
  var lifecycleFixture: typeof fixture;
}

globalThis.lifecycleFixture = fixture;
