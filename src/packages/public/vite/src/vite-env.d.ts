/// <reference types="vite/client" />

/** Vite environment variables consumed or injected by the Interfere SDK. */
interface ImportMetaEnv {
  readonly VITE_INTERFERE_SERVER_TRACING?: string;
  readonly VITE_INTERFERE_LOG_SOURCING?: string;
  /** Build-injected hosting platform from the surface configuration; `null` when the surface has none. */
  readonly VITE_INTERFERE_PLATFORM?: string | null;
  /** Build-injected ingest endpoint, retained during SSR document rendering. */
  readonly VITE_INTERFERE_API_HOST?: string | null;
  /** Build-injected flag that disables both browser and server SDKs. */
  readonly VITE_INTERFERE_DISABLED?: string;
  /** Build-injected deployment environment. */
  readonly VITE_INTERFERE_ENVIRONMENT?: string | null;
  /** Build-injected proxy ingest endpoint, avoiding a runtime configuration fetch. */
  readonly VITE_INTERFERE_INGEST_URL?: string | null;
  /** Build-injected plugin configuration encoded as JSON. */
  readonly VITE_INTERFERE_BACKEND_HOSTS?: string | null;
  readonly VITE_INTERFERE_PLUGINS?: string | null;
  /** Build-injected anonymous session replay policy from the surface configuration. */
  readonly VITE_INTERFERE_ANONYMOUS_REPLAY?: string | null;
  /** Public key required for telemetry ingestion. */
  readonly VITE_INTERFERE_PUBLIC_KEY?: string;
  /** Build-injected release slug derived from the commit SHA. */
  readonly VITE_INTERFERE_RELEASE_SLUG?: string | null;
  readonly VITE_INTERFERE_RELEASE_TOKEN?: string | null;
  /** Build-injected default `service.name`: the plugin option, `OTEL_SERVICE_NAME`, or the package name without its scope. */
  readonly VITE_INTERFERE_SERVICE_NAME?: string | null;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}

declare module "cloudflare:workers" {
  export function waitUntil(promise: Promise<unknown>): void;
}
