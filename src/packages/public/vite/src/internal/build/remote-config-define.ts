import type { createBuildConfigResolver } from "@interfere/build/config";

type BakedConfig = Awaited<
  ReturnType<ReturnType<typeof createBuildConfigResolver>>
>;

export function remoteConfigDefine(baked: BakedConfig | null) {
  return {
    "import.meta.env.VITE_INTERFERE_INGEST_URL": baked?.ingestUrl
      ? JSON.stringify(baked.ingestUrl)
      : undefined,
    "import.meta.env.VITE_INTERFERE_SERVER_TRACING": JSON.stringify(
      baked?.serverTracing ?? null,
    ),
    "import.meta.env.VITE_INTERFERE_LOG_SOURCING": JSON.stringify(
      baked?.logSourcing ?? null,
    ),
    "import.meta.env.VITE_INTERFERE_PLATFORM":
      baked?.platform === undefined
        ? "undefined"
        : JSON.stringify(baked.platform),
    "import.meta.env.VITE_INTERFERE_BACKEND_HOSTS": baked?.backendHosts
      ? JSON.stringify(JSON.stringify(baked.backendHosts))
      : undefined,
    "import.meta.env.VITE_INTERFERE_ANONYMOUS_REPLAY": JSON.stringify(
      baked?.anonymousReplayPolicy ?? null,
    ),
    "import.meta.env.VITE_INTERFERE_PLUGINS": baked?.plugins
      ? JSON.stringify(JSON.stringify(baked.plugins))
      : undefined,
  };
}
