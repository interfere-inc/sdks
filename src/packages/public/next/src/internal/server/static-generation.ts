import { workAsyncStorage } from "next/dist/server/app-render/work-async-storage.external.js";
import { workUnitAsyncStorage } from "next/dist/server/app-render/work-unit-async-storage.external.js";

export function isStaticGenerationRender() {
  try {
    const store = workAsyncStorage.getStore();

    if (!store) {
      return false;
    }

    if ("isStaticGeneration" in store) {
      return store.isStaticGeneration === true;
    }

    if (store.isBuildTimePrerendering) {
      return true;
    }

    const unit = workUnitAsyncStorage.getStore();

    return unit?.type !== "request" && unit?.type !== "prerender-runtime";
  } catch {
    return false;
  }
}
