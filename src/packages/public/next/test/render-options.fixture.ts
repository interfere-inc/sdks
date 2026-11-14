import type { WorkStoreContext } from "next/dist/server/async-storage/work-store.js";
import { createRequestStoreForAPI } from "next/dist/server/async-storage/request-store.js";
import { NextRequest } from "next/server";

export const RENDER_OPTIONS = {
  assetPrefix: "",
  isBuildTimePrerendering: false,
  isDraftMode: false,
  cacheLifeProfiles: { default: { stale: 300, revalidate: 900, expire: 3600 } },
  staticPageGenerationTimeout: 60,
  cacheComponents: false,
  validationLevel: "warning",
  experimental: {
    authInterrupts: false,
    useCacheTimeout: 50_000,
    durableUseCacheEntries: false,
  },
  waitUntil: undefined,
  onAfterTaskError: undefined,
} satisfies Omit<WorkStoreContext["renderOpts"], "onClose">;

export const createRequestStore = () =>
  createRequestStoreForAPI(
    new NextRequest("https://example.com/account"),
    { pathname: "/account" },
    { tags: [], expirationsByCacheKind: new Map() },
    undefined,
    undefined,
    undefined,
  );
