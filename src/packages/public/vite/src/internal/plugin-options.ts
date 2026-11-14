export interface InterfereViteOptions {
  /** Writes private build evidence outside public and build output directories. */
  buildReceipt?: string;
  /**
   * The endpoint the browser SDK posts telemetry to. Defaults to
   * Interfere's endpoint. Set it to a subdomain of your own site that
   * resolves to Interfere via a DNS record (e.g.
   * `"https://in.example.com"`) to survive ad-blockers, or to a
   * regional/local endpoint. Must be a full URL.
   */
  apiHost?: string;
  /**
   * Override the commit SHA used to derive the release slug. Defaults
   * to `git rev-parse HEAD`.
   * Set this when the build runs outside a git checkout (e.g. a
   * Docker layer-cached image build).
   */
  commitSha?: string;
  /** Separates source maps for distinct bundles built from the same commit. */
  buildVariant?: string;
  /**
   * Default service name for browser and server telemetry. Falls back to
   * `OTEL_SERVICE_NAME`, then the unscoped
   * app package name. Explicit runtime service names still win.
   */
  serviceName?: string;
  /**
   * Directory (relative to the project root) where the SSR/server build
   * writes its `.js.map` files, when that isn't `<build.outDir>/server`.
   * Set this for server hosts that emit outside Vite's `outDir` — e.g.
   * Nitro's `".output/server"`.
   */
  serverSourcemapsDir?: string;
  /**
   * Upload source maps for the release. A production build registers a release
   * when `INTERFERE_API_KEY` is set, and skips it with a warning when the key is
   * missing. `upload: false` registers the release without uploading source maps.
   * Interfere API failures, including a rejected secret key, never fail the
   * build: they are logged, local source maps are deleted and the build finishes.
   * Library builds (`build.lib`) register nothing and are left untouched unless
   * `upload: true` is set explicitly.
   *
   * `retain: true` builds once for every deployment environment: the build
   * registers nothing, emits debug-id-paired source maps next to its chunks and
   * leaves them in the output for `releaseBuildOutput()` (`@interfere/vite/release`)
   * to register and upload per environment at deploy time.
   */
  sourceMaps?: { upload?: boolean; retain?: boolean };
  /**
   * Turns Interfere off for this build: no release, no source-map upload, and
   * the browser and server SDKs send no events.
   */
  disable?: boolean;
}

const DOCS_URL = "https://interfere.com/docs/sdk/vite";

function buildError(title: string, details: readonly string[]) {
  return new Error([`Interfere: ${title}`, ...details].join("\n"));
}

export function rejectRemovedOptions(options: InterfereViteOptions) {
  const sourceMaps: unknown = options.sourceMaps;

  if (
    sourceMaps !== undefined &&
    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- JavaScript configs can still pass the removed boolean and string forms.
    (typeof sourceMaps !== "object" || sourceMaps === null)
  ) {
    throw buildError("Removed: interfere({ sourceMaps })", [
      '`sourceMaps: true`, `sourceMaps: false` and `sourceMaps: "required"` were removed.',
      "Use `sourceMaps: { upload: false }` to register the release without source maps, or `disable: true` to turn Interfere off for the build.",
      `See: ${DOCS_URL}`,
    ]);
  }

  if ("sourceMapReadinessTimeoutMs" in options) {
    throw buildError("Removed: interfere({ sourceMapReadinessTimeoutMs })", [
      "The build no longer waits for source-map processing. Remove the option.",
      `See: ${DOCS_URL}`,
    ]);
  }
}
