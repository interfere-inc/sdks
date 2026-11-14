# @interfere/vite

## 13.0.3

### Patch Changes

- 13e0639: Capture the `x-forwarded-for` client IP header with server request errors by default, including Next.js Node and Edge request errors and Vite request middleware failures. A client IP address is personal data in many jurisdictions; if your privacy notice or data processing terms do not cover it, set `privacy: { clientIp: false }` in server `register()` to omit this header from captured exception events. Authorization and cookie headers remain excluded.
- 13e0639: Use neutral examples in SDK documentation and fixtures, and describe regression tests by their behavior. Remove internal issue references from public API documentation.

  Correct the SDK 13.0 migration notes: list breaking tracing changes together, consolidate repeated entries, and clarify that the canonical browser configuration is `tracing: { mode: "auto" }`, with `tracing: "auto"` still supported. Tracing ownership and header propagation are separate options; use `tracing.propagation: "external"` when another tool writes W3C headers.

- 13e0639: Include MIT licenses and package documentation, declare tree-shaking behavior, and document the supported Node.js and framework versions. Complete server adapter package metadata and link license badges to the included license.
- 13e0639: Remove workspace-only resolution conditions from stable and canary packages so consumers using custom conditions resolve shipped runtime and type files. Qualification tests now use the same sanitized package manifests while development source resolution stays unchanged.
- 13e0639: Expand error and lifecycle coverage in Firefox and WebKit, verify Next.js instrumentation in the Edge runtime, and cover Windows source-map output paths.
- 13e0639: Source maps that Interfere turns on are never left in your build output, even when one of them is malformed or the upload is skipped.

  - `vite build --watch` uploads and removes source maps on every rebuild, not only the first.
  - Library builds (`build.lib`) are left untouched: no debug-ID runtime, no forced source maps and no release. Pass `interfere({ sourceMaps: { upload: true } })` to upload a library's source maps.
  - The release commit is read from the git checkout at the Vite `root`, so builds started from another directory get the right release.
  - A client build followed by a separate `vite build --ssr` uploads and removes the server's source maps instead of waiting for them.
  - With Vite environments (for example TanStack Start), client source maps are published under the URLs the browser loads (`assets/…`), not the build folder (`client/assets/…`).
  - Debug IDs are only added to chunks that emit a source map, so generated chunks without a map, such as a framework's virtual manifest, no longer carry a debug ID that points at nothing. Client chunks now keep their `//# debugId=` comment after minification.

- 13e0639: TanStack Start request tracing is now accurate for streamed responses and safer on every runtime.

  - `interfereRequestMiddleware` keeps the server request span open until a streamed response body finishes. A stream that fails partway through now marks the span as errored and records the exception, instead of reporting a successful request.
  - When another OpenTelemetry provider (for example Sentry) is registered and Interfere runs in managed mode, an error thrown through `interfereRequestMiddleware` is now delivered to Interfere, not only to the other provider.
  - `captureError`, `withRequestSpan` and `traceMeta` no longer throw on runtimes without a `process` global (such as Cloudflare Workers without Node.js compatibility, or Deno), and read the public key the build inlined there.
  - A third-party propagator that throws no longer breaks the request: the request still runs and `traceMeta()` returns no tags.
  - New `interfereFunctionMiddleware` for TanStack Start captures errors thrown by `createServerFn` handlers, which Start serializes to the client so the request middleware never sees them. Add it to `createStart({ functionMiddleware })`. Thrown redirects and `notFound()` are not reported.
  - The TanStack Start quick start now calls `register()` from a `src/server.ts` server entry instead of the isomorphic router file. The old setup bundled the Node SDK into the browser bundle; following the new setup shrinks the client entry chunk from about 900 KB to about 400 KB.

- Updated dependencies [65fa924]
- Updated dependencies [e6bafd2]
- Updated dependencies [2298a72]
- Updated dependencies [f967bca]
- Updated dependencies [13e0639]
- Updated dependencies [85292bd]
- Updated dependencies [5ba5305]
- Updated dependencies [343565f]
- Updated dependencies [13e0639]
- Updated dependencies [13e0639]
- Updated dependencies [13e0639]
- Updated dependencies [13e0639]
- Updated dependencies [13e0639]
- Updated dependencies [13e0639]
- Updated dependencies [13e0639]
- Updated dependencies [13e0639]
- Updated dependencies [13e0639]
  - @interfere/react@13.0.3
  - @interfere/types@11.0.2
  - @interfere/server-core@0.1.3
  - @interfere/build@0.5.2
  - @interfere/constants@9.3.3
  - @interfere/helpers@9.3.4
  - @interfere/javascript@0.4.1

## 13.0.2

### Patch Changes

- @interfere/react@13.0.2

## 13.0.1

### Patch Changes

- 76a8c1e: Continue builds with a warning when a commit SHA is unavailable, skipping release publication. Detect commit SHAs from GitLab, Netlify, Render, CircleCI, Bitbucket, Azure Pipelines, Buildkite, Railway, AWS Amplify, and Heroku environments while preserving explicit source ID overrides.

  Remove generated Next.js source maps when release publication is skipped unless source-map purging is explicitly disabled.

- 76a8c1e: Removed deprecated SDK APIs:

  - Removed `PrerenderSafeIdGenerator` from `@interfere/next/instrumentation`; use `register()` to install managed tracing, or keep your provider's generator in application mode.
  - Removed the `KernelInternalOptions` re-export from `@interfere/next/instrument-client`; use `KernelOptions` for `init()` options.
  - Removed `tracing.apiHosts`; use `tracing.propagateTo` with exact origins such as `https://api.example.com`.
  - Removed the Vite provider's `errorBoundary` prop; use `captureRenderErrors`.
  - Removed `uploadSourceMapsMetadata` and `UploadSourceMapsMetadata` from `@interfere/types`; use the source-map upload session and completion schemas. Existing legacy server uploads remain supported.

- 76a8c1e: Server `register()` now prints one warning when it adopts a tracer provider another tool registered first, such as Sentry or dd-trace with its OpenTelemetry bridge. Server errors still reach Interfere, but request traces arrive only if that provider exports them over OTLP; pass `register({ tracing: "application" })` to confirm the setup and silence the message. When dd-trace traces the process without its OpenTelemetry bridge, `register()` also warns once that Datadog and Interfere record separate traces and that dd-trace owns outbound trace headers.
- 76a8c1e: Browser `init()` now accepts `ignoreErrors`, `denyUrls`, `errors.sampleRate` and `beforeSend`, so you can drop noisy exceptions, exclude errors thrown by third-party scripts, sample error volume, and scrub or drop an exception before it leaves the page. The options apply to every capture source, including errors raised before remote configuration loads. If a `beforeSend` hook throws, the original exception is still sent.
- 76a8c1e: Server error handling now follows the host's lifecycle instead of overriding it.

  - After an uncaught exception, the SDK captures the error, flushes telemetry, and exits with code 1 only when no other `uncaughtException` listener exists, which is when Node itself would have exited. Under `next start`, which keeps its own listener, the server keeps serving instead of being shut down by the SDK in production.
  - `@interfere/vite/server` now captures uncaught exceptions and unhandled rejections in the server process. The `captureUncaughtException` and `captureUnhandledRejection` options turn this off.
  - Telemetry is flushed on `SIGTERM`, `SIGINT` and when the event loop drains, for at most 2 seconds. If your app handles the signal itself, shutdown stays yours. Otherwise the SDK flushes and then lets the signal terminate the process as it would have.
  - `register()` from `@interfere/next` does nothing during `next build`: it installs no process handlers and makes no network calls.
  - Registering again, for example after a dev-server module reload, replaces the SDK's process listeners rather than adding more.

- Updated dependencies [30a48a6]
- Updated dependencies [76a8c1e]
- Updated dependencies [76a8c1e]
- Updated dependencies [76a8c1e]
- Updated dependencies [76a8c1e]
- Updated dependencies [76a8c1e]
- Updated dependencies [76a8c1e]
- Updated dependencies [76a8c1e]
- Updated dependencies [76a8c1e]
- Updated dependencies [76a8c1e]
- Updated dependencies [76a8c1e]
- Updated dependencies [76a8c1e]
- Updated dependencies [76a8c1e]
- Updated dependencies [76a8c1e]
- Updated dependencies [30a48a6]
  - @interfere/types@11.0.1
  - @interfere/react@13.0.1
  - @interfere/helpers@9.3.3
  - @interfere/server-core@0.1.2
  - @interfere/build@0.5.1

## 13.0.0

### Major Changes

- 3ff22ac: Make browser tracing ownership explicit. Replace `tracing: true` with the canonical options object, such as `tracing: { mode: "auto" }`. The string shorthand `tracing: "auto"` is also accepted, as are `"managed"`, `"application"` and `false`. The default is `auto`; application code controls the mode. `auto` respects an existing OpenTelemetry propagator, `managed` records request spans, and `application` records request observations without claiming OpenTelemetry globals. Header propagation is independent: set `tracing.propagation: "external"` when another tool owns W3C trace headers. Existing caller headers are preserved, and baggage has its own opt-out. With `tracing: false`, exceptions still export through the capture-only transport.

- 8735286: SDK 13.0 separates browser recording, request correlation and OpenTelemetry tracing, and retires the public server OTel kit. Server `register()` adopts an existing provider or manages tracing when none exists; use `register({ tracing: "application" })` to require application ownership. In the browser, use the canonical options object `tracing: { mode: "auto" }`; the shorthand `tracing: "auto"` remains supported. Explicit `managed` and `application` modes control OpenTelemetry ownership and request recording. Use `tracing.propagation: "external"` independently when another tool owns outgoing W3C headers.

  Upgrade all `@interfere/*` packages together. The table lists every v12 option or export that changes.

  | v12                                                                                                                        | v13                                                                                                                                                                                                                                                                                                                                                  |
  | -------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
  | `buildInterfereOtelKit()` and the `InterfereOtelKit` type from `@interfere/next/instrumentation`                           | Removed. Call `register()` when Interfere owns tracing, or `register({ tracing: "application" })` and point your provider at Interfere with `OTEL_EXPORTER_OTLP_ENDPOINT`, `OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf` and a `Bearer` header.                                                                                                        |
  | `kit.idGenerator` and the `PrerenderSafeIdGenerator` export                                                                | `kit.idGenerator` is removed with the kit; `PrerenderSafeIdGenerator` is deprecated and removed in 14.0. Managed `register()` installs the prerender-safe generator itself. Application mode keeps your provider's generator.                                                                                                                        |
  | `kit.traceContextPropagator`, `kit.propagators` and the documented "trace context before `propagators`" ordering           | Not needed. `register()` fixes the propagator order inside the Next adapter: cache-safe W3C trace context, freshness stamp, baggage. In application mode your propagator decides what the browser receives.                                                                                                                                          |
  | `kit.spanProcessors`, `kit.logRecordProcessors`, `kit.resourceAttributes`, `kit.instrumentations`                          | Not needed. Managed mode constructs them; application mode receives your spans over OTLP and needs none of them.                                                                                                                                                                                                                                     |
  | `kit.enableConsoleBridge()` and `kit.fetchRemoteConfig()`                                                                  | Both run inside `register()` in either mode. The console bridge emits into the logger provider you registered, or into a private Interfere provider when you registered none.                                                                                                                                                                        |
  | `register()` detecting another tracer provider by sniffing the OpenTelemetry registry or `propagation.fields()`            | Server `register()` defaults to `auto`: adopt an existing provider, or manage tracing when none exists. Set `tracing: "managed"` or `tracing: "application"` in code to override ownership.                                                                                                                                                          |
  | `<TraceMeta />` from `@interfere/next/server`                                                                              | Removed. `withInterfere()` enables Next's `clientTraceMetadata`, which carries the `tracestate` freshness stamp that the 13.x browser SDK requires; a bare `traceparent` meta tag no longer links anything. Delete the component from your layout. `TraceMeta` on `@interfere/vite/server` is unchanged.                                             |
  | `errorBoundary` prop on `InterfereProvider` (`@interfere/react`, `@interfere/next`)                                        | Removed. Use `captureRenderErrors={false}` to stop the provider capturing render errors; the default captures and rethrows to your boundary as before.                                                                                                                                                                                               |
  | `errorBoundary` prop on `InterfereProvider` (`@interfere/vite`)                                                            | Deprecated; removed in 14.0. The Vite provider now takes `captureRenderErrors`, which wins when both are set.                                                                                                                                                                                                                                        |
  | `tracing.apiHosts` (browser `init()`)                                                                                      | Deprecated; removed in 14.0. Use `tracing.propagateTo` with exact origins (`https://api.example.com`). Each `apiHosts` hostname maps to `https://<host>` with one warning.                                                                                                                                                                           |
  | `tracing.propagateContextUrls` (browser `init()`)                                                                          | Removed. Use `tracing.propagateTo`. The server-side `propagateContextUrls` option of `@interfere/vite/server` is unchanged.                                                                                                                                                                                                                          |
  | Same-site trace propagation (any subdomain, scheme or port of the page's registrable domain) and automatic sibling origins | Removed. The browser injects `traceparent`, `tracestate` and `baggage` only to the page origin, `tracing.propagateTo` entries and the workspace's configured backend hosts as exact `https://` origins. Sibling subdomains need their own entry.                                                                                                     |
  | Retrying a failed cross-origin request without trace headers                                                               | Removed. One application call produces one request. A suspected CORS failure logs once per origin and stamps `interfere.propagation.suspect_cors` on the request observation; no header is stripped and nothing is retried.                                                                                                                          |
  | Browser `init()` without a tracing mode                                                                                    | The default is `tracing: { mode: "auto" }`. The shorthand `tracing: "auto"` and both forms of `managed` and `application` remain supported. Mode controls OpenTelemetry ownership and request recording; use `tracing.propagation: "external"` for application-owned W3C headers. `tracing: false` skips tracing while retaining exception delivery. |

  Behaviour that changes without a code change:

  - The browser writes `traceparent` and `tracestate` only to requests that carry none, so a header another tool or your own code set first is never replaced, and it merges `interfere.session.id` (and the consented `interfere.device.id`) into any existing `baggage` header on every allowed request in every tracing mode. Each request span or `interfere.request` observation records the trace id the request left with as `interfere.request.trace_id`, so the browser record joins the server span whichever tool wrote the header.
  - The browser keeps an unsampled server-side trace context (`traceparent` flags `00`) and its `tracestate`; it rejects SSR trace meta whose `interfere` tracestate stamp is missing or older than two minutes, so cached HTML cannot link visitors together.
  - `withInterfere()` enables `clientTraceMetadata` whenever the browser SDK is configured. Managed server tracing stamps the metadata; application-owned tracing supplies its own metadata, and the browser rejects an unstamped `traceparent`.
  - Managed server `register()` registers a `ParentBased(AlwaysOn)` sampler unless `OTEL_TRACES_SAMPLER` is set; Interfere never re-decides a remote sampling decision.
  - Disposing the SDK restores only the `fetch`, `XMLHttpRequest`, `history`, `console` and `onerror` wrappers that are still Interfere's own, and releases only the OpenTelemetry globals it registered. Another SDK's wrappers survive `close()`.

  Mixed deployments of 12.x browsers with 13.x servers, or the reverse, keep working: storage keys, session and device identity and the ingest protocol are unchanged, and the 12.x line receives compatibility and security fixes for twelve months after 13.0 is published.

### Minor Changes

- 3dfaefa: Accept `company: { tier }` in `identity.set()` and send company tier changes with session identification.
- 8f92d43: Remove the surface-level tracing owner. Browser and server tracing modes are configured in application code and default to `auto`; `managed` and `application` remain explicit overrides. Remote configuration cannot change tracing ownership.
- c76563f: `identify()` now posts as soon as it is called, with the local session and device ids and whatever fingerprint is already known; it no longer waits for fingerprinting or a country lookup. A fingerprint that resolves later reaches the device through the next session sync, and is discarded if device consent was revoked or the identity was reset in the meantime. The browser no longer requests `cloudflare.com/cdn-cgi/trace` or parses the user agent for classification: country comes from the ingest edge and browser/OS classification from the server. With `tracing: false`, captured exceptions are exported again: each becomes one OTLP exception span posted directly to the trace drain, without constructing a tracer provider, exporter chain or instrumentation. `@interfere/types` removes `sdk/geo` (`cloudflareTraceSchema`, `CloudflareTrace`), whose only consumer was the removed geo lookup. It also removes `sdk/plugins/context/browser` (`browserContextSchema`, `deviceMetadataSchema` and their types), which lost its only consumer with the browser-side user-agent parsing. Exceptions exported with `tracing: false` go through the same URL redaction as the tracing path, and at most one export is in flight with up to 32 exceptions queued behind it.
- 3226d63: Keep unsampled server-side trace context. The browser SDK now extracts an unsampled `traceparent` (flags `00`) and its `tracestate` from the SSR meta tags as served instead of inventing a new sampled root; the initial page-view event carries it, sampled or not. Also, the Vite `traceMeta()` helper emits unsampled contexts too. The browser never decides sampling from the meta: its private tracer provider declares a `ParentBased(AlwaysOn)` sampler, and the Next server registers the same sampler unless `OTEL_TRACES_SAMPLER` is set. SSR trace meta without a fresh `interfere` tracestate stamp is now rejected, so a `traceparent` served from cached HTML can no longer link visitors together. The Next adapter owns cache-safe trace context, freshness stamps and ID generation internally. `withInterfere()` enables `clientTraceMetadata` whenever the browser SDK is configured; the browser accepts only fresh, stamped server context.
- f9f5849: Correlate browser telemetry by session, page view and user action instead of one ambient page trace. Every span, log record, exception and web-vital point the browser SDK emits carries `session.id` and `interfere.view.id`. Records produced synchronously inside a click or submit handler also carry `interfere.action.id`. A view starts at page load, on every URL change (history push, replace or back/forward; hash-only changes excluded) and after 30 minutes on one URL. Each view emits one `browser.navigation` page-view event, and each admitted click or submit emits one `interfere.user_action` event, both under the `@interfere/browser-events` scope. The initial page-view event carries the server render's trace context, sampled or not, as its link to the render.

  Nothing is parented to a page or navigation span any more. Requests, `span()` calls, long tasks and document-load spans start their own traces unless an explicit parent is active, so unrelated work on one page no longer shares a trace and an explicit `ROOT_CONTEXT` stays a root. Work after an `await` inside `span()` is a new root with the same session and view. The per-page `navigation` span and the per-click interaction span are gone: page views and user actions are log events that ship on the log cadence, so traces split into more, smaller trees with fewer spans in total.

- 4193144: Deliver application-mode server errors through Interfere's provider as exception spans linked to the host span, even when the host has no exporter to Interfere. Add `errors: "host"` to keep the recording host span as the only copy; the default deduplicates host events when both providers export to Interfere.
- 3c5fbd0: Allow server `register({ consoleBridge: { level: "warn" } })` to bridge only warnings and errors. Support debug, info, warn, and error thresholds without patching lower-level console methods; boolean options keep their existing behavior.
- 7856701: Default server registration to automatic tracing ownership. Adopt an existing OpenTelemetry tracer provider with one debug diagnostic, or manage tracing when none is registered. Keep explicit managed and application overrides in code.
- d44b603: Send the existing service identity on SDK config, session, replay, and release requests so workspace public keys can select the App when using workspace keys. Build config caching now separates services and namespaces, and retained Vite builds preserve the service name for later release publication. Existing App keys keep working.

### Patch Changes

- f584c2f: Preserve every shared JavaScript chunk alias and align its debug ID with the uploaded source map. Retain initiating callsites on XHR and transport failures, server error causes, and structured thrown values. Report partial client and server source-map coverage.

  Publish source maps on Next.js 15 after its final Webpack client compilation, preserving customer hooks and waiting for publication and cleanup before prerendering.

  Separate source maps for Vercel deployments built from the same Git commit. Keep build-time release identities in browser and server bundles during promotion, while retaining the original commit for source ownership.

- 38bc8fc: Move the server telemetry runtime into `@interfere/server-core`, a new runtime dependency of `@interfere/next` and `@interfere/vite`. It owns registration, provider ownership, exception normalization and delivery, URL privacy, console bridging and bounded provider flushing, so concurrent server initialization emits each log once and captured exceptions export once. Next keeps its framework error hooks and Vercel drain integration; Vite keeps Undici request instrumentation and metrics.

  Vite redacts sensitive URL query values, applies the outbound propagation policy when it injects headers, and exposes `flush()` with the shared privacy options. React initialization and close have explicit lifecycle states, clean up after a failed startup, keep consent when the provider omits the consent prop, and dispose page-wide Web Vitals observers with their kernel.

- 38bc8fc: Accept Vite `^8.2.4` and TanStack Start `^1.168.0` as peers instead of the exact versions the SDK was built with, so projects on a newer Vite 8 or TanStack Start 1 release install without `--legacy-peer-deps`.
- e8b9987: Browser trace headers reach a backend host discovered from workspace configuration on `GET` and `HEAD` from the first request, and on other methods only after one header-bearing `GET` or `HEAD` to that origin has received a response in this browser session (memo in `sessionStorage`), so an unmodified CORS policy never breaks a first `POST`. The page origin and explicit `tracing.propagateTo` origins always carry their headers. An origin whose header-bearing request fails at the network layer before it has ever answered one is marked header-less for the browser session with one diagnostic naming `Access-Control-Allow-Headers`; nothing is retried and every request is still observed. The README section "Which headers go where" lists the rules.
- 2938037: Use an exact-origin set for browser request propagation and share the server correlation baggage processor across Next and Vite, preserving Next's correlationSpanProcessor export.
- 420d2c0: Document what remote configuration can change, how packaged plugins load, and which tracing settings remain controlled by application code.
- 41463e2: Document release-age exclusions and lockfile verification for fresh SDK upgrades, including transitive Interfere packages.

  ### Network

  No runtime changes: headers, destinations, retries and transmitted data are unchanged.

- ec97a9c: Ship CHANGELOG.md inside the published tarball of every package so release notes travel with the installed version.
- 575cc65: Warn once per build when the inferred service name differs from the resolved App slug, with guidance to preserve the existing service name during upgrades.
- a49895d: Document workspace keys, App routing, service naming, migration, and auto-create controls. Clarify public-key build diagnostics.
- Updated dependencies [f584c2f]
- Updated dependencies [38bc8fc]
- Updated dependencies [ae599ff]
- Updated dependencies [e8b9987]
- Updated dependencies [68a9ee0]
- Updated dependencies [2938037]
- Updated dependencies [a1bbc63]
- Updated dependencies [6a1805f]
- Updated dependencies [2e4a93a]
- Updated dependencies [420d2c0]
- Updated dependencies [9fc86e9]
- Updated dependencies [d9c142c]
- Updated dependencies [4f88820]
- Updated dependencies [8ad8d61]
- Updated dependencies [eec3c43]
- Updated dependencies [3dfaefa]
- Updated dependencies [41463e2]
- Updated dependencies [a80d8cd]
- Updated dependencies [8f92d43]
- Updated dependencies [c76563f]
- Updated dependencies [8735286]
- Updated dependencies [3ff22ac]
- Updated dependencies [3226d63]
- Updated dependencies [f9f5849]
- Updated dependencies [35cbb18]
- Updated dependencies [ec97a9c]
- Updated dependencies [7c8441a]
- Updated dependencies [a49895d]
- Updated dependencies [d44b603]
- Updated dependencies [87479c2]
  - @interfere/react@13.0.0
  - @interfere/types@11.0.0
  - @interfere/javascript@0.4.0
  - @interfere/build@0.5.0
  - @interfere/server-core@0.1.1
  - @interfere/constants@9.3.2
  - @interfere/helpers@9.3.2

## 13.0.0-rc.7

### Patch Changes

- Updated dependencies [d9c142c]
  - @interfere/react@13.0.0-rc.7

## 13.0.0-rc.6

### Patch Changes

- 420d2c0: Document what remote configuration can change, how packaged plugins load, and which tracing settings remain controlled by application code.
- Updated dependencies [ae599ff]
- Updated dependencies [420d2c0]
- Updated dependencies [9fc86e9]
- Updated dependencies [420d2c0]
- Updated dependencies [35cbb18]
  - @interfere/react@13.0.0-rc.6
  - @interfere/javascript@0.4.0-rc.2
  - @interfere/build@0.5.0-rc.4

## 13.0.0-rc.5

### Patch Changes

- a49895d: Document workspace keys, App routing, service naming, migration, and auto-create controls. Clarify public-key build diagnostics.
- Updated dependencies [a49895d]
  - @interfere/react@13.0.0-rc.5

## 13.0.0-rc.4

### Minor Changes

- 4193144: Deliver application-mode server errors through Interfere's provider as exception spans linked to the host span, even when the host has no exporter to Interfere. Add `errors: "host"` to keep the recording host span as the only copy; the default deduplicates host events when both providers export to Interfere.

### Patch Changes

- @interfere/react@13.0.0-rc.4

## 13.0.0-rc.3

### Patch Changes

- 41463e2: Document release-age exclusions and lockfile verification for fresh SDK upgrades, including transitive Interfere packages.

  ### Network

  No runtime changes: headers, destinations, retries and transmitted data are unchanged.

- Updated dependencies [8ad8d61]
- Updated dependencies [41463e2]
- Updated dependencies [7c8441a]
  - @interfere/react@13.0.0-rc.3
  - @interfere/types@11.0.0-rc.2
  - @interfere/build@0.5.0-rc.3

## 13.0.0-rc.2

### Minor Changes

- 3dfaefa: Accept `company: { tier }` in `identity.set()` and send company tier changes with session identification.
- 8f92d43: Remove the surface-level tracing owner. The SDKs no longer read a build-baked or remote `tracingOwnership`, and the warning about an option that differs from the surface setting is gone. The browser `tracing.mode` and the Next server `tracing` option keep `auto` as the default; the Vite server option keeps `managed`; `managed` and `application` in code are the only overrides.
- 7856701: Default server registration to automatic tracing ownership. Adopt an existing OpenTelemetry tracer provider with one debug diagnostic, or manage tracing when none is registered. Keep explicit managed and application overrides in code.

### Patch Changes

- e8b9987: Browser trace headers reach a backend host discovered from workspace configuration on `GET` and `HEAD` from the first request, and on other methods only after one header-bearing `GET` or `HEAD` to that origin has received a response in this browser session (memo in `sessionStorage`), so an unmodified CORS policy never breaks a first `POST`. The page origin and explicit `tracing.propagateTo` origins always carry their headers. An origin whose header-bearing request fails at the network layer before it has ever answered one is marked header-less for the browser session with one diagnostic naming `Access-Control-Allow-Headers`; nothing is retried and every request is still observed. The README section "Which headers go where" lists the rules.
- 2938037: Use an exact-origin set for browser request propagation and share the server correlation baggage processor across Next and Vite, preserving Next's correlationSpanProcessor export.
- ec97a9c: Ship CHANGELOG.md inside the published tarball of every package so release notes travel with the installed version.
- 575cc65: Warn once per build when the inferred service name differs from the resolved App slug, with guidance to preserve the existing service name during upgrades.
- Updated dependencies [e8b9987]
- Updated dependencies [2938037]
- Updated dependencies [6a1805f]
- Updated dependencies [2e4a93a]
- Updated dependencies [3dfaefa]
- Updated dependencies [8f92d43]
- Updated dependencies [ec97a9c]
- Updated dependencies [87479c2]
  - @interfere/react@13.0.0-rc.2
  - @interfere/types@11.0.0-rc.1
  - @interfere/javascript@0.4.0-rc.1
  - @interfere/build@0.5.0-rc.2
  - @interfere/constants@9.3.2-rc.0
  - @interfere/helpers@9.3.2-rc.0

## 13.0.0-rc.1

### Minor Changes

- 3c5fbd0: Allow server `register({ consoleBridge: { level: "warn" } })` to bridge only warnings and errors. Support debug, info, warn, and error thresholds without patching lower-level console methods; boolean options keep their existing behavior.
- d44b603: Send the existing service identity on SDK config, session, replay, and release requests so workspace public keys can select the App when gateway workspace-key routing is enabled. Build config caching now separates services and namespaces, and retained Vite builds preserve the service name for later release publication. Existing App keys keep working.

### Patch Changes

- Updated dependencies [a1bbc63]
- Updated dependencies [4f88820]
- Updated dependencies [d44b603]
  - @interfere/react@13.0.0-rc.1
  - @interfere/javascript@0.4.0-rc.0
  - @interfere/build@0.5.0-rc.1

## 13.0.0-rc.0

### Major Changes

- 8735286: SDK 13.0 separates browser recording, request correlation and OpenTelemetry tracing, and retires the public server OTel kit in favour of two supported setups: Interfere-managed tracing (`register()` / `init()` with no options) and application-owned tracing (`register({ tracing: "application" })` / `init({ tracing: "application" })`, with your provider exporting to Interfere over standard OTLP). In the browser the default is `tracing: "auto"`: Interfere writes trace headers unless another tracing tool is on the page, and `"managed"` and `"application"` remain explicit overrides. Customer code no longer composes Interfere samplers, ID generators or propagators, and no option is order-sensitive.

  Upgrade all `@interfere/*` packages together. The table lists every v12 option or export that changes.

  | v12                                                                                                                        | v13                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
  | -------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
  | `buildInterfereOtelKit()` and the `InterfereOtelKit` type from `@interfere/next/instrumentation`                           | Removed. Call `register()` when Interfere owns tracing, or `register({ tracing: "application" })` and point your provider at Interfere with `OTEL_EXPORTER_OTLP_ENDPOINT`, `OTEL_EXPORTER_OTLP_PROTOCOL=http/protobuf` and a `Bearer` header.                                                                                                                                                                                                                                                                                                                                                                                                             |
  | `kit.idGenerator` and the `PrerenderSafeIdGenerator` export                                                                | `kit.idGenerator` is removed with the kit; `PrerenderSafeIdGenerator` is deprecated and removed in 14.0. Managed `register()` installs the prerender-safe generator itself. Application mode keeps your provider's generator.                                                                                                                                                                                                                                                                                                                                                                                                                             |
  | `kit.traceContextPropagator`, `kit.propagators` and the documented "trace context before `propagators`" ordering           | Not needed. `register()` fixes the propagator order inside the Next adapter: cache-safe W3C trace context, freshness stamp, baggage. In application mode your propagator decides what the browser receives.                                                                                                                                                                                                                                                                                                                                                                                                                                               |
  | `kit.spanProcessors`, `kit.logRecordProcessors`, `kit.resourceAttributes`, `kit.instrumentations`                          | Not needed. Managed mode constructs them; application mode receives your spans over OTLP and needs none of them.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
  | `kit.enableConsoleBridge()` and `kit.fetchRemoteConfig()`                                                                  | Both run inside `register()` in either mode. The console bridge emits into the logger provider you registered, or into a private Interfere provider when you registered none.                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
  | `register()` detecting another tracer provider by sniffing the OpenTelemetry registry or `propagation.fields()`            | `register({ tracing })` with explicit precedence: the option, then the surface's tracing owner baked at build, then `managed`. Managed `register()` under an existing provider logs one warning naming `tracing: "application"`, keeps capturing through an isolated provider and writes no other global.                                                                                                                                                                                                                                                                                                                                                 |
  | `<TraceMeta />` from `@interfere/next/server`                                                                              | Removed. `withInterfere()` enables Next's `clientTraceMetadata`, which carries the `tracestate` freshness stamp that the 13.x browser SDK requires; a bare `traceparent` meta tag no longer links anything. Delete the component from your layout. `TraceMeta` on `@interfere/vite/server` is unchanged.                                                                                                                                                                                                                                                                                                                                                  |
  | `errorBoundary` prop on `InterfereProvider` (`@interfere/react`, `@interfere/next`)                                        | Removed. Use `captureRenderErrors={false}` to stop the provider capturing render errors; the default captures and rethrows to your boundary as before.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
  | `errorBoundary` prop on `InterfereProvider` (`@interfere/vite`)                                                            | Deprecated; removed in 14.0. The Vite provider now takes `captureRenderErrors`, which wins when both are set.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
  | `tracing.apiHosts` (browser `init()`)                                                                                      | Deprecated; removed in 14.0. Use `tracing.propagateTo` with exact origins (`https://api.example.com`). Each `apiHosts` hostname maps to `https://<host>` with one warning.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                |
  | `tracing.propagateContextUrls` (browser `init()`)                                                                          | Removed. Use `tracing.propagateTo`. The server-side `propagateContextUrls` option of `@interfere/vite/server` is unchanged.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
  | Same-site trace propagation (any subdomain, scheme or port of the page's registrable domain) and automatic sibling origins | Removed. The browser injects `traceparent`, `tracestate` and `baggage` only to the page origin, `tracing.propagateTo` entries and the workspace's configured backend hosts as exact `https://` origins. Sibling subdomains need their own entry.                                                                                                                                                                                                                                                                                                                                                                                                          |
  | Retrying a failed cross-origin request without trace headers                                                               | Removed. One application call produces one request. A suspected CORS failure logs once per origin and stamps `interfere.propagation.suspect_cors` on the request observation; no header is stripped and nothing is retried.                                                                                                                                                                                                                                                                                                                                                                                                                               |
  | Browser `init()` without a tracing mode                                                                                    | `tracing` is `"auto"` by default: Interfere owns trace headers unless Datadog RUM, Sentry tracing or another OpenTelemetry propagator is already on the page, in which case it logs which tool it found and behaves as `"application"`. `"managed"` always writes trace headers to allowed origins; `"application"` records errors, replay, navigation, interactions, vitals and request observations without writing an OpenTelemetry global, creating a request span or writing `traceparent`. Both spellings, `tracing: "application"` and `tracing: { mode: "application" }`, are accepted. `tracing: false` still skips the tracing module entirely. |

  Behaviour that changes without a code change:

  - The browser writes `traceparent` and `tracestate` only to requests that carry none, so a header another tool or your own code set first is never replaced, and it merges `interfere.session.id` (and the consented `interfere.device.id`) into any existing `baggage` header on every allowed request in every tracing mode. Each request span or `interfere.request` observation records the trace id the request left with as `interfere.request.trace_id`, so the browser record joins the server span whichever tool wrote the header.
  - The browser keeps an unsampled server-side trace context (`traceparent` flags `00`) and its `tracestate`; it rejects SSR trace meta whose `interfere` tracestate stamp is missing or older than two minutes, so cached HTML cannot link visitors together.
  - `withInterfere()` enables `clientTraceMetadata` only when the surface's tracing owner is Interfere, so application-owned apps never serve an Interfere-unstamped `traceparent`.
  - Managed server `register()` registers a `ParentBased(AlwaysOn)` sampler unless `OTEL_TRACES_SAMPLER` is set; Interfere never re-decides a remote sampling decision.
  - Disposing the SDK restores only the `fetch`, `XMLHttpRequest`, `history`, `console` and `onerror` wrappers that are still Interfere's own, and releases only the OpenTelemetry globals it registered. Another SDK's wrappers survive `close()`.

  Mixed deployments of 12.x browsers with 13.x servers, or the reverse, keep working: storage keys, session and device identity and the ingest protocol are unchanged, and the 12.x line receives compatibility and security fixes for twelve months after 13.0 is published.

### Minor Changes

- c76563f: `identify()` now posts as soon as it is called, with the local session and device ids and whatever fingerprint is already known; it no longer waits for fingerprinting or a country lookup. A fingerprint that resolves later reaches the device through the next session sync, and is discarded if device consent was revoked or the identity was reset in the meantime. The browser no longer requests `cloudflare.com/cdn-cgi/trace` or parses the user agent for classification: country comes from the ingest edge and browser/OS classification from the server. With `tracing: false`, captured exceptions are exported again: each becomes one OTLP exception span posted directly to the trace drain, without constructing a tracer provider, exporter chain or instrumentation. `@interfere/types` removes `sdk/geo` (`cloudflareTraceSchema`, `CloudflareTrace`), whose only consumer was the removed geo lookup. It also removes `sdk/plugins/context/browser` (`browserContextSchema`, `deviceMetadataSchema` and their types), which lost its only consumer with the browser-side user-agent parsing. Exceptions exported with `tracing: false` go through the same URL redaction as the tracing path, and at most one export is in flight with up to 32 exceptions queued behind it.
- 3ff22ac: Make browser tracing ownership explicit. `init({ tracing })` now takes `false` or an options object with `mode: "managed" | "application"`; the boolean `true` form is gone. Managed mode (the default) registers Interfere's context manager and W3C + baggage propagator only when the OpenTelemetry registration call succeeds, records what it registered and releases exactly that on `close()`; when another propagator already owns trace headers it warns once, injects nothing and observes requests as events. Application mode never writes an OpenTelemetry API global, injects no header and creates no request span: requests are recorded as `interfere.request` log events that carry the application's active span context, and exceptions still record on the host's active span. The default mode comes from the workspace surface setting (`tracingOwnership`, baked by the Next and Vite builds or read from the remote configuration); an explicit `mode` that disagrees with it logs one warning. Every patch Interfere installs (`fetch`, `XMLHttpRequest`, `history`, `console`, `window.onerror`) is restored only while it is still the installed function and goes inert beneath a wrapper another SDK installed later. With `tracing: false` the kernel no longer loads the OTLP exporter chain at all; exceptions on that path are still not exported, as in 12.x, until the capture-only transport lands. Propagated baggage uses the namespaced keys `interfere.session.id` and `interfere.device.id`.
- 3226d63: Keep unsampled server-side trace context. The browser SDK now extracts an unsampled `traceparent` (flags `00`) and its `tracestate` from the SSR meta tags as served instead of inventing a new sampled root; the initial page-view event carries it, sampled or not. Also, the Vite `traceMeta()` helper emits unsampled contexts too. The browser never decides sampling from the meta: its private tracer provider declares a `ParentBased(AlwaysOn)` sampler, and the Next server registers the same sampler unless `OTEL_TRACES_SAMPLER` is set. SSR trace meta without a fresh `interfere` tracestate stamp is now rejected, so a `traceparent` served from cached HTML can no longer link visitors together. The Next adapter owns the cache-safe trace-context, freshness-stamp and ID-generator pieces internally: `register()` fixes their order, the kit's `propagators` is a complete ordered list to spread as-is, and `traceContextPropagator`, `idGenerator` and `PrerenderSafeIdGenerator` are deprecated. `withInterfere()` enables `clientTraceMetadata` only when the surface's tracing owner is Interfere (`tracingOwnership` in the remote configuration, default `managed`).
- f9f5849: Correlate browser telemetry by session, page view and user action instead of one ambient page trace. Every span, log record, exception and web-vital point the browser SDK emits carries `session.id` and `interfere.view.id`. Records produced synchronously inside a click or submit handler also carry `interfere.action.id`. A view starts at page load, on every URL change (history push, replace or back/forward; hash-only changes excluded) and after 30 minutes on one URL. Each view emits one `browser.navigation` page-view event, and each admitted click or submit emits one `interfere.user_action` event, both under the `@interfere/browser-events` scope. The initial page-view event carries the server render's trace context, sampled or not, as its link to the render.

  Nothing is parented to a page or navigation span any more. Requests, `span()` calls, long tasks and document-load spans start their own traces unless an explicit parent is active, so unrelated work on one page no longer shares a trace and an explicit `ROOT_CONTEXT` stays a root. Work after an `await` inside `span()` is a new root with the same session and view. The per-page `navigation` span and the per-click interaction span are gone: page views and user actions are log events that ship on the log cadence, so traces split into more, smaller trees with fewer spans in total.

### Patch Changes

- Updated dependencies [68a9ee0]
- Updated dependencies [eec3c43]
- Updated dependencies [a80d8cd]
- Updated dependencies [c76563f]
- Updated dependencies [8735286]
- Updated dependencies [3ff22ac]
- Updated dependencies [3226d63]
- Updated dependencies [f9f5849]
  - @interfere/types@11.0.0-rc.0
  - @interfere/react@13.0.0-rc.0
  - @interfere/build@0.4.6-rc.0

## 12.4.4

### Patch Changes

- Updated dependencies [3a4f991]
- Updated dependencies [bd9b8a3]
- Updated dependencies [4676d54]
- Updated dependencies [e1f7ee8]
  - @interfere/javascript@0.3.3
  - @interfere/react@12.4.4
  - @interfere/build@0.4.5
  - @interfere/types@10.6.3

## 12.4.3

### Patch Changes

- d5a2d36: Read each app's anonymous session replay policy from its remote configuration and bake it into Next.js and Vite builds, so the browser SDK can record visitors who are not signed in only after an error.
- 6347222: Check surface bindings before compiling production telemetry. Incomplete surfaces leave the SDK inactive with one setup link, while temporary configuration outages keep capture enabled.
- 2f2934a: Enable development telemetry when a public key is configured and remove undocumented environment overrides in favor of existing SDK options and automatic defaults.
- c2428dd: Keep automatic tracing within the correct site boundaries on shared hosting platforms. Preserve discovered backend hosts in production builds so browser requests can connect to backend traces without manual host configuration.
- Updated dependencies [d5a2d36]
- Updated dependencies [461082f]
- Updated dependencies [3c6ba7c]
- Updated dependencies [6347222]
- Updated dependencies [6085507]
- Updated dependencies [2f2934a]
- Updated dependencies [4ecf0c8]
- Updated dependencies [2aac9f5]
- Updated dependencies [c2428dd]
- Updated dependencies [c1fa96c]
- Updated dependencies [8948010]
- Updated dependencies [4c794cb]
- Updated dependencies [152d420]
  - @interfere/types@10.6.2
  - @interfere/javascript@0.3.2
  - @interfere/react@12.4.3
  - @interfere/helpers@9.3.1
  - @interfere/build@0.4.4
  - @interfere/constants@9.3.1

## 12.4.2

### Patch Changes

- Updated dependencies [11b1ec6]
  - @interfere/react@12.4.2

## 12.4.1

### Patch Changes

- Updated dependencies [8381980]
  - @interfere/react@12.4.1
  - @interfere/types@10.6.1
  - @interfere/build@0.4.3

## 12.4.0

### Patch Changes

- 9dd1bf0: A build that baked SDK plugin settings from an older Interfere API, missing a newer plugin's key, now keeps those settings instead of discarding them: the browser no longer refetches `/v3/config` on every page load, and a plugin you disabled stays disabled. A missing key reads as enabled.
- 698127f: React apps no longer spend extra main-thread time in `completeWork` and `commitHostUpdate` after the SDK initialises. The first initialisation performs one throwaway React mount in a detached document, taking about 0.1 ms.
- Updated dependencies [9dd1bf0]
- Updated dependencies [2dd3ff8]
- Updated dependencies [8ad3169]
- Updated dependencies [48a8e64]
- Updated dependencies [6795d30]
- Updated dependencies [af83af0]
- Updated dependencies [9dd1bf0]
- Updated dependencies [76e3603]
- Updated dependencies [698127f]
- Updated dependencies [c949553]
- Updated dependencies [4e6078e]
- Updated dependencies [b8a433b]
  - @interfere/types@10.6.0
  - @interfere/react@12.4.0
  - @interfere/build@0.4.2

## 12.3.4

### Patch Changes

- Updated dependencies [2cc257c]
  - @interfere/react@12.3.4

## 12.3.3

### Patch Changes

- Updated dependencies [3fe50d6]
  - @interfere/react@12.3.3

## 12.3.2

### Patch Changes

- Updated dependencies [49d54e9]
- Updated dependencies [4734c16]
- Updated dependencies [9823dce]
- Updated dependencies [2154056]
- Updated dependencies [c834d7f]
  - @interfere/react@12.3.2

## 12.3.1

### Patch Changes

- 4d4457c: The 2 s cap on direct OTLP trace and log exports now follows the surface's hosting platform from the Interfere configuration, not the `VERCEL` environment variable. The configuration reports `platform: "vercel"` for a Next.js surface linked to a Vercel project, and `"cloudflare"` for a surface linked to a Cloudflare Worker. The build injects it the same way it injects the drain modes.

  - The cap applies when the injected platform is `vercel`. A surface with no linked platform exports with the standard timeouts, even when `VERCEL` is set.
  - When the build could not load the configuration, the SDK falls back to `VERCEL` for this decision. That fallback is removed in the next major.
  - `VERCEL_ENV` and `VERCEL_TARGET_ENV` still serve only as fallbacks for the environment label.
  - `@interfere/types`: adds `@interfere/types/integrations/platforms/hosting` (`hostingPlatformSchema`, `HostingPlatform`, `PLATFORM_GLOBAL`). `resolveServerTracing` takes the injected `platform`.
  - `@interfere/javascript`: `SdkConfigResponse` and the release config types gain the optional `platform` field.

- Updated dependencies [6effe17]
- Updated dependencies [4d4457c]
  - @interfere/react@12.3.1
  - @interfere/types@10.5.1
  - @interfere/build@0.4.1
  - @interfere/javascript@0.3.1

## 12.3.0

### Minor Changes

- 31c892f: Add `interfere({ sourceMaps: { retain: true } })` and `releaseBuildOutput()` from `@interfere/vite/release`, so one production build can serve every deployment environment: the build registers nothing and keeps debug-id-paired source maps in its output, and a deploy-time step registers the release for each environment, uploads the maps and deletes them.
- 7582c3e: Share application service-name inference between Next.js and Vite. Both build integrations use the explicit plugin option, then `OTEL_SERVICE_NAME`, then the application's unscoped package name. Vite retains `VITE_INTERFERE_SERVICE_NAME` as a fallback before package inference. Browser and server telemetry consume the inferred default, including Next.js edge telemetry and Vite server registration, while explicit runtime names retain precedence.
- 0c772cc: Server trace and log sourcing are now automatic. The mode comes only from the surface configuration injected at build time, which reports `drain` when a trace or log drain exists for the surface.

  - Removed from `@interfere/next`: the `serverTracing` option of `register()` and `buildInterfereOtelKit()`.
  - Removed environment variables: `INTERFERE_SERVER_TRACING` and `INTERFERE_LOG_SOURCING`. Vite no longer reads `VITE_INTERFERE_SERVER_TRACING` or `VITE_INTERFERE_LOG_SOURCING` from `.env`.
  - When the surface has no drain, or the build cannot load its configuration, the SDK exports directly (`full` / `sdk`) as before.
  - On Vercel (`VERCEL` set), direct OTLP trace and log exports are capped at 2 s, per attempt and per flush. A flush can no longer hold a function invocation open for up to 65 s.
  - `@interfere/types` changes:
    - `resolveServerTracingMode` and `readServerTracingMode` are replaced by `resolveServerTracing` and `readServerTracing`, which return `{ mode, vercel }`.
    - `resolveLogSourcingMode` takes the injected value.
    - `SERVER_TRACING_ENV_VAR` and `LOG_SOURCING_ENV_VAR` are removed.

  Migration: delete the `serverTracing` option and the two environment variables. The mode is automatic.

### Patch Changes

- 457c645: `releaseBuildOutput()` returns `status: "skipped"` instead of `"failed"` when the release already holds source maps from another build of the same bundle (`409 RELEASE_UPLOAD_CONFLICT`), so re-releasing a cache-restored build no longer fails the deploy.
- Updated dependencies [07744b9]
- Updated dependencies [7582c3e]
- Updated dependencies [6be5831]
- Updated dependencies [0c772cc]
  - @interfere/react@12.3.0
  - @interfere/build@0.4.0
  - @interfere/types@10.5.0

## 12.2.0

### Minor Changes

- c96a012: Name the span that carries a captured exception after its throw site. When no span is active at capture time the SDK used to open a zero-duration span called `interfere.exception.capture` (`interfere.captureError` on the server), so every trace showed the same opaque row. The fallback span is now named after the top in-app stack frame (`handleClick (app.js:42)`), falls back to the error's type when the stack has no usable frame, carries `interfere.capture.kind = "exception"`, and is marked `ERROR` for unhandled captures. When a span is active, the exception is still recorded as an OpenTelemetry `exception` event on it; the browser SDK now also sets that span's status to `ERROR` for unhandled captures, matching the server SDKs. Handled captures (`console.error`, error boundaries) never change a customer span's status.
- 4fff52d: Stamp a default `service.name` into Vite builds. `@interfere/vite/plugin` now derives one from the `serviceName` plugin option, then `VITE_INTERFERE_SERVICE_NAME`, then the project package name without its scope (`@acme/storefront` → `storefront`), and `@interfere/vite/init` bridges it onto the page. `@interfere/react` reads that stamped name when `init()` is called without `serviceName`, so a frontend that never set one no longer reports OpenTelemetry's `unknown_service`. An explicit `init({ serviceName })` still wins. `@interfere/types/sdk/service-name` adds the `SERVICE_NAME_GLOBAL` wire name.
- 47943fb: Send the public key on release uploads so workspace secret keys can publish releases.

### Patch Changes

- Updated dependencies [c96a012]
- Updated dependencies [68e9a31]
- Updated dependencies [fe666ef]
- Updated dependencies [915503c]
- Updated dependencies [67b57b8]
- Updated dependencies [4fff52d]
- Updated dependencies [cb80e0a]
- Updated dependencies [47943fb]
  - @interfere/react@12.2.0
  - @interfere/helpers@9.3.0
  - @interfere/types@10.4.0
  - @interfere/constants@9.3.0
  - @interfere/build@0.3.3

## 12.1.4

### Patch Changes

- Updated dependencies [f9d411d]
  - @interfere/javascript@0.3.0
  - @interfere/build@0.3.2
  - @interfere/react@12.1.4

## 12.1.3

### Patch Changes

- 5680798: Link release warnings to the app's `/apps` settings page in the dashboard instead of the retired `/surfaces` path.
- 8a58c25: Keep the browser SDK disabled when no public key is configured, instead of sending requests that ingest rejects as unauthenticated, and warn once so the missing key is visible.
- Updated dependencies [8a58c25]
  - @interfere/react@12.1.3

## 12.1.2

### Patch Changes

- dbc5398: Include MIT license files and UAParser Enterprise notices in published SDK packages.
- Updated dependencies [dbc5398]
  - @interfere/react@12.1.2

## 12.1.1

### Patch Changes

- Updated dependencies [c4e0754]
  - @interfere/helpers@9.2.1
  - @interfere/react@12.1.1

## 12.1.0

### Minor Changes

- a231cb5: Builds no longer fail when Interfere is unavailable or `INTERFERE_API_KEY` is missing. Adds a `disable` option. Vite removes the boolean and `"required"` forms of `sourceMaps`, and `sourceMapReadinessTimeoutMs`; use `sourceMaps: { upload: false }` or `disable: true`.

### Patch Changes

- f34f8f2: Retry timed-out source-map status requests within the existing publication deadline while preserving readiness validation and caller cancellation.
- Updated dependencies [820973a]
- Updated dependencies [4d17ef3]
- Updated dependencies [9a92e01]
- Updated dependencies [da0e2e2]
- Updated dependencies [a231cb5]
- Updated dependencies [bbf8037]
- Updated dependencies [8a20a12]
  - @interfere/react@12.1.0
  - @interfere/types@10.3.0
  - @interfere/build@0.3.1

## 12.0.5

### Patch Changes

- 15efdb8: Compress source-map uploads, use larger streaming chunks, and make concurrency configurable.
- Updated dependencies [99f3752]
- Updated dependencies [8ffe6fb]
- Updated dependencies [15efdb8]
  - @interfere/build@0.3.0
  - @interfere/types@10.2.2
  - @interfere/react@12.0.5

## 12.0.4

### Patch Changes

- Updated dependencies [2bea649]
  - @interfere/react@12.0.4

## 12.0.3

### Patch Changes

- af4aa48: Export each SDK's default OpenTelemetry `service.name` from `@interfere/constants/service-names`. The SDKs read their defaults from it, so the reported service names are unchanged.
- Updated dependencies [af4aa48]
  - @interfere/constants@9.2.0
  - @interfere/react@12.0.3

## 12.0.2

### Patch Changes

- 99a1f17: Support boolean consent shorthand.
- Updated dependencies [99a1f17]
- Updated dependencies [dcbcbd8]
- Updated dependencies [b984525]
  - @interfere/react@12.0.2
  - @interfere/types@10.2.1
  - @interfere/build@0.2.2

## 12.0.1

### Patch Changes

- Updated dependencies [b52524b]
  - @interfere/javascript@0.2.1
  - @interfere/build@0.2.1
  - @interfere/react@12.0.1

## 12.0.0

### Major Changes

- 1f01b42: Send telemetry to `api.interfere.com` using new api routes.

### Minor Changes

- 21309b2: Upload source maps separately for each deploy environment, so preview and production builds of the same commit no longer conflict. Server-side events now report the build's deploy environment, which matches browser events and the uploaded source maps. Server events that previously fell back to `NODE_ENV` can now carry a different environment label.
- 67ec54b: Require source maps for release readiness.

  Align Next.js and Vite build configuration for plugins, ingest URL, server tracing, and log sourcing. Both build integrations register releases and verify upload acceptance, and only remove local maps after the server accepts the complete upload.

- 87fad80: Share deployment metadata and source-map file utilities across Next and Vite. Both integrations call the generated API client endpoints directly. Source maps are removed only after durable acceptance of the complete upload.
- 27d080c: Emit the full W3C trace-context meta set from SSR.

### Patch Changes

- 2ec9c4f: Share bounded, ordered asynchronous mapping between the build plugins. Limit Vite source-map discovery and rewriting to 16 concurrent files while preserving duplicate-map selection.

  Retain sequential retry and chunk-upload behavior under performance linting.

- b8b6902: retry throttled and transient source-map uploads instead of failing the release
- 63df2c9: Refresh dependency ranges. `@interfere/nest` now accepts NestJS 11 or 12 as a peer.
- 21f4335: Give separately bundled build variants distinct source-map release identities while preserving the original commit SHA.
- d3c3e02: Upload source maps through the API gateway in resumable chunks.
- a653b8d: Add the generated JavaScript API client with ESM, CommonJS, and TypeScript declarations. Next and Vite call its endpoints directly for build configuration, release creation, source-map uploads, and telemetry. Remove the shared build client and its combined release-preparation method. Preserve ordered uploads, expiration checks, and cleanup only after complete server acceptance.

  Build requests use Fern's retry policy for HTTP 408, 429, and 5xx responses. Network failures and HTTP 425 are no longer retried.

  Retire the unused CLI and Speakeasy SDK packages, their generation configuration, and compatibility wrappers. Remove legacy host remapping and Collector error aliases. Update framework setup instructions to use the generated release endpoints.

- a653b8d: Generate runtime configuration, session creation, replay upload, and telemetry ingestion methods alongside the release API. Publish these runtime operations to SDK and MCP documents without adding them to HTTP documentation.

  Use generated endpoint methods throughout the framework SDKs. Authenticate browser requests through the public-key header, preserve request timeouts and unload delivery, and report failed edge telemetry requests as failed exports.

- 65b3ebb: Use a source-map upload-session response containing the expiry and file paths for v3 uploads. Upload chunks through the generated client with a path and byte offset. Legacy signing schemas and v1 tokenized upload URLs remain available.

  Remove unused session request headers, build dependencies, and Vite release-result fields.

- 7d3b14e: Consolidate optional SDK and test settings while preserving configured defaults, disabled options, and omitted properties.
- 4f9ef13: Widen React peer dependency range.
- ffaa95b: Fix build output that referenced deleted source maps after upload.
- 5b8542b: Preserve inferred capture attribute types and verify optional telemetry fields with native exporter tests.

  Preserve deep-merge behavior for nested values and use schema-owned field types in telemetry contracts.

  Preserve inferred release-slug types and validate baked configuration before using it.

  Treat inherited object-property names as unknown release error codes and hostnames rather than matching them as configured entries.

  Use the generated request contracts for handwritten source-map upload helpers while preserving finalize-response validation.

  Validate test application manifests before creating temporary directories so malformed manifests do not leave unused directories behind.

  Validate source maps before patching JavaScript debug IDs in Vite and Next builds, preserving the original artifacts when a map is malformed.

  Preserve special own keys such as `__proto__` as data when omitting undefined properties, without changing the result prototype.

  Validate Vite source maps before injecting debug IDs into JavaScript chunks, so malformed maps leave the chunk unchanged.

- fac5408: Strengthen event validation and type handling, correct promise handling, and update test runner integration.
- d25952d: Fix source map uploads for some server-rendered builds, and clarify build errors caused by network restrictions.
- Updated dependencies [5b005aa]
- Updated dependencies [40501f3]
- Updated dependencies [2ec9c4f]
- Updated dependencies [b8b6902]
- Updated dependencies [78bf77a]
- Updated dependencies [0efde29]
- Updated dependencies [3a309ab]
- Updated dependencies [63df2c9]
- Updated dependencies [21309b2]
- Updated dependencies [a653b8d]
- Updated dependencies [a653b8d]
- Updated dependencies [65b3ebb]
- Updated dependencies [a61b85d]
- Updated dependencies [4ca51e3]
- Updated dependencies [7d3b14e]
- Updated dependencies [47bbcbf]
- Updated dependencies [78bf77a]
- Updated dependencies [757a458]
- Updated dependencies [dc6f82b]
- Updated dependencies [4f9ef13]
- Updated dependencies [27d080c]
- Updated dependencies [0d1a8ef]
- Updated dependencies [27d080c]
- Updated dependencies [27d080c]
- Updated dependencies [74c8f2b]
- Updated dependencies [27d080c]
- Updated dependencies [27d080c]
- Updated dependencies [27d080c]
- Updated dependencies [01e49fd]
- Updated dependencies [67ec54b]
- Updated dependencies [aa79ebc]
- Updated dependencies [39c53c1]
- Updated dependencies [907a91a]
- Updated dependencies [27d080c]
- Updated dependencies [e772e67]
- Updated dependencies [265a458]
- Updated dependencies [76e64d9]
- Updated dependencies [5ff7610]
- Updated dependencies [0a4bfd7]
- Updated dependencies [1f01b42]
- Updated dependencies [87fad80]
- Updated dependencies [2e3c112]
- Updated dependencies [88ce674]
- Updated dependencies [ffaa95b]
- Updated dependencies [5b8542b]
- Updated dependencies [fac5408]
- Updated dependencies [27d080c]
- Updated dependencies [bbfda83]
- Updated dependencies [d25952d]
  - @interfere/types@10.2.0
  - @interfere/build@0.2.0
  - @interfere/helpers@9.2.0
  - @interfere/react@12.0.0
  - @interfere/constants@9.1.0
  - @interfere/javascript@0.2.0

## 11.2.1

### Patch Changes

- f1dedba: Cut the console→OTel bridge's contribution to request CPU.
- f1dedba: improved types
- Updated dependencies [f1dedba]
- Updated dependencies [345b3d8]
  - @interfere/constants@9.0.6
  - @interfere/helpers@9.1.2
  - @interfere/react@11.2.1
  - @interfere/types@10.1.2
  - @interfere/sdk@11.2.1

## 11.2.0

### Minor Changes

- Server runtime release-slug resolution no longer falls back to spawning `git rev-parse HEAD`. At runtime the slug comes from the build-baked global or the `INTERFERE_SOURCE_ID`-family env keys only; the synchronous git subprocess is now injected exclusively at build-time call sites (`withInterfere`, the Vite plugin, and the release pipeline), keeping `node:child_process` out of the server instrumentation import graph entirely.

### Patch Changes

- c7f7d47: Add blank lines around multiline statements and before returns, from enforcing the internal padding-between-statements lint rule repo-wide. Whitespace only, no behavior or API difference.
- Updated dependencies [c7f7d47]
  - @interfere/constants@9.0.5
  - @interfere/helpers@9.1.1
  - @interfere/react@11.2.0
  - @interfere/types@10.1.1
  - @interfere/sdk@11.2.0

## 11.1.1

### Patch Changes

- c7f7d47: Add blank lines around multiline statements and before returns, from enforcing the internal padding-between-statements lint rule repo-wide. Whitespace only, no behavior or API difference.
- 29c2e0b: Improve source-map uploads.
- Updated dependencies [c7f7d47]
  - @interfere/constants@9.0.5
  - @interfere/helpers@9.1.1
  - @interfere/react@11.1.1
  - @interfere/types@10.1.1
  - @interfere/sdk@11.1.1

## 11.1.0

### Patch Changes

- 7b80d3f: Remove lint suppression comments that no longer suppress anything.
- 911ecd4: Update OpenTelemetry to 0.221: `BatchLogRecordProcessor` now takes a single options object, and the browser instrumentations drop `semconvStabilityOptIn` now that HTTP semantic conventions are stable.
- e347b65: Internal lint modernization: apply oxlint autofixes across previously quarantined rules (import specifier style, catch variable naming, DOM idiom updates, and related mechanical cleanups). No behavior changes intended.
- 7b22af5: Declare the `react-dom` peer range as `^19.0.0` instead of inheriting the workspace catalog.
- 18f2746: Improved release slug / SHA deduction.
- 18f2746: Bug fixes & performance improvements
- 9a8843a: Bump the shared Vitest catalog from 4.1.9 to 4.1.10
- Updated dependencies [73ac9f4]
- Updated dependencies [1a40a00]
- Updated dependencies [7b80d3f]
- Updated dependencies [9162968]
- Updated dependencies [1949d88]
- Updated dependencies [42d939d]
- Updated dependencies [911ecd4]
- Updated dependencies [e347b65]
- Updated dependencies [7b22af5]
- Updated dependencies [18f2746]
- Updated dependencies [f410225]
- Updated dependencies [ddb16ce]
- Updated dependencies [18f2746]
- Updated dependencies [ea785f4]
- Updated dependencies [9a8843a]
  - @interfere/sdk@11.1.0
  - @interfere/types@10.1.0
  - @interfere/constants@9.0.4
  - @interfere/react@11.1.0
  - @interfere/helpers@9.1.0

## 11.0.5

### Patch Changes

- @interfere/sdk@11.0.5
- @interfere/react@11.0.5

## 11.0.4

### Patch Changes

- 2ecaada: Fix remote-config and deploy-environment globals not baking into Next.js 16 Turbopack production builds. `withInterfere()` now returns a `(phase) => NextConfig` function and reads the build phase from that argument instead of the unreliable `process.env.NEXT_PHASE`. An async config function (one that resolves to a Promise) is now awaited and composed instead of throwing.

  Composition: `withInterfere()` should always be the outermost Next config wrapper.

- 52e7a06: Standardize credential terminology to "public key" and "secret key" in build diagnostics, CLI help, plugin warnings, and READMEs. Env var names (`INTERFERE_API_KEY`, `INTERFERE_PUBLIC_KEY`), CLI flags (`--api-key`), and diagnostic codes are unchanged.
- e06fea2: Update the missing-public-key diagnostic to point at the current Vite SDK docs URL (`/docs/sdk/vite` instead of the retired `/docs/guides/vite`).
- Updated dependencies [0a4a6f1]
- Updated dependencies [2ecaada]
- Updated dependencies [c6ecb1e]
- Updated dependencies [52e7a06]
- Updated dependencies [b562bb3]
  - @interfere/react@11.0.4
  - @interfere/types@10.0.2
  - @interfere/sdk@11.0.4

## 11.0.3

### Patch Changes

- Updated dependencies [d5d405c]
  - @interfere/helpers@9.0.3
  - @interfere/react@11.0.3
  - @interfere/sdk@11.0.3

## 11.0.2

### Patch Changes

- 2f8aad9: Add `logSourcing: "sdk" | "drain"` alongside `serverTracing`, resolved from `/v2/config` at build time and readable via `readLogSourcingMode()`.

  In `drain` mode the SDK skips its OTLP log exporter and console bridge (Next + Vite); the trace exporter is likewise skipped in `serverTracing: "drain"`. With both set, the SDK ships no telemetry and Vercel's platform drains carry it end to end.

- 32a1acf: bug fixes & performance improvements
- Updated dependencies [2f8aad9]
- Updated dependencies [f797ab7]
- Updated dependencies [42b109e]
- Updated dependencies [2151370]
- Updated dependencies [82f4c4f]
- Updated dependencies [bd5b333]
- Updated dependencies [32a1acf]
  - @interfere/types@10.0.1
  - @interfere/react@11.0.2
  - @interfere/constants@9.0.3
  - @interfere/sdk@11.0.2

## 11.0.1

### Patch Changes

- 475f125: Move `consent` and `plugins` configuration from `init()` to `<InterfereProvider>`. Enhance PII handling.
- Updated dependencies [475f125]
  - @interfere/react@11.0.1
  - @interfere/helpers@9.0.2
  - @interfere/sdk@11.0.1

## 11.0.0

### Minor Changes

- 97eba37: Add `identity.clear()` to clear the linked identity and rotate to a fresh anonymous session on logout.

### Patch Changes

- 9883610: Republish to align packed devDependency pin with the current workspace version.
- 6c47775: Resolve published workspace dependency ranges from package manifests.
- 1501b2f: SSR instrumentation improvements
- f2281fc: Fix debugId round-trip from build plugins through the SDK to the collector for source map resolution.
- 2bc13b1: Fix: `identity.clear()` now rotates the `deviceId` and stamps it per span/log so post-logout events attribute to a fresh anonymous person instead of the previously-identified user.
- dc31127: Configurable ingest endpoint via `apiHost` — post telemetry to a subdomain of your own site (resolved to Interfere via DNS) or a regional endpoint. Ingest requests never carry cookies.

  **Breaking (`@interfere/next`):** proxy mode is removed. The `@interfere/next/route-handler` export is gone — delete your `app/api/interfere/[[...path]]/route.ts`. Telemetry now posts directly to Interfere; set `interfere.apiHost` for an ad-blocker-resistant subdomain setup.

- dea34cf: Source map generation optimizations
- 9a9e2a3: bug fixes
- 8b1c635: Declare OpenAPI security schemes on the public collector contract. The SDK now exposes first-class auth: `publishableKey` (`?pk=…`) as a constructor option for ingestion operations, and per-operation `{ apiKey }` security on release management methods (`releases.*`, `sourceMaps.*`, `telemetry.report`) — replacing the custom `httpClient` interceptor pattern. The next/vite plugins pass the typed security argument internally.
- Updated dependencies [9883610]
- Updated dependencies [a23cf2a]
- Updated dependencies [7ba7566]
- Updated dependencies [7c6bf32]
- Updated dependencies [6c47775]
- Updated dependencies [97eba37]
- Updated dependencies [dcdec1f]
- Updated dependencies [b19d109]
- Updated dependencies [f2281fc]
- Updated dependencies [4893278]
- Updated dependencies [6514e2a]
- Updated dependencies [cfaf24f]
- Updated dependencies [ac95069]
- Updated dependencies [62cd678]
- Updated dependencies [e304910]
- Updated dependencies [2bc13b1]
- Updated dependencies [dc31127]
- Updated dependencies [e2100a1]
- Updated dependencies [d265875]
- Updated dependencies [0edf4b9]
- Updated dependencies [80c1e40]
- Updated dependencies [5506f36]
- Updated dependencies [5f7643a]
- Updated dependencies [0d53d41]
- Updated dependencies [b1e24c8]
- Updated dependencies [d5213b2]
- Updated dependencies [5e12a55]
- Updated dependencies [ed30c31]
- Updated dependencies [9a9e2a3]
- Updated dependencies [8b1c635]
- Updated dependencies [8b1c635]
- Updated dependencies [8171e78]
- Updated dependencies [7ba7566]
- Updated dependencies [d5213b2]
- Updated dependencies [254bed9]
- Updated dependencies [3ae0b71]
  - @interfere/constants@9.0.2
  - @interfere/react@11.0.0
  - @interfere/types@10.0.0
  - @interfere/sdk@11.0.0

## 11.0.0-canary.8

### Patch Changes

- 9883610: Republish to align packed devDependency pin with the current workspace version.
- 2bc13b1: Fix: `identity.clear()` now rotates the `deviceId` and stamps it per span/log so post-logout events attribute to a fresh anonymous person instead of the previously-identified user.
- Updated dependencies [9883610]
- Updated dependencies [2bc13b1]
  - @interfere/constants@9.0.2-canary.4
  - @interfere/react@11.0.0-canary.8
  - @interfere/types@10.0.0-canary.7
  - @interfere/sdk@11.0.0-canary.8

## 11.0.0-canary.7

### Patch Changes

- 1501b2f: SSR instrumentation improvements
- f2281fc: Fix debugId round-trip from build plugins through the SDK to the collector for source map resolution.
- dc31127: Configurable ingest endpoint via `apiHost` — post telemetry to a subdomain of your own site (resolved to Interfere via DNS) or a regional endpoint. Ingest requests never carry cookies.

  **Breaking (`@interfere/next`):** proxy mode is removed. The `@interfere/next/route-handler` export is gone — delete your `app/api/interfere/[[...path]]/route.ts`. Telemetry now posts directly to Interfere; set `interfere.apiHost` for an ad-blocker-resistant subdomain setup.

- Updated dependencies [f2281fc]
- Updated dependencies [cfaf24f]
- Updated dependencies [dc31127]
- Updated dependencies [d5213b2]
- Updated dependencies [d5213b2]
  - @interfere/react@11.0.0-canary.7
  - @interfere/types@10.0.0-canary.6
  - @interfere/sdk@11.0.0-canary.7

## 11.0.0-canary.6

### Patch Changes

- 6c47775: Resolve published workspace dependency ranges from package manifests.
- 9a9e2a3: bug fixes
- Updated dependencies [6c47775]
- Updated dependencies [dcdec1f]
- Updated dependencies [e304910]
- Updated dependencies [9a9e2a3]
  - @interfere/react@11.0.0-canary.6
  - @interfere/types@10.0.0-canary.5
  - @interfere/sdk@11.0.0-canary.6

## 11.0.0-canary.5

### Patch Changes

- Updated dependencies [62cd678]
  - @interfere/types@9.0.3-canary.4
  - @interfere/sdk@11.0.0-canary.5
  - @interfere/react@11.0.0-canary.5

## 11.0.0-canary.4

### Patch Changes

- Updated dependencies [d265875]
  - @interfere/constants@9.0.2-canary.1
  - @interfere/helpers@9.0.1-canary.0
  - @interfere/types@9.0.3-canary.3
  - @interfere/react@11.0.0-canary.4
  - @interfere/sdk@11.0.0-canary.4
- Updated dependencies [80c1e40]
- Updated dependencies [254bed9]
  - @interfere/react@11.0.0-canary.4
  - @interfere/sdk@11.0.0-canary.4

## 11.0.0-canary.3

### Minor Changes

- 97eba37: Add `identity.clear()` to clear the linked identity and rotate to a fresh anonymous session on logout.

### Patch Changes

- dea34cf: Source map generation optimizations
- Updated dependencies [97eba37]
- Updated dependencies [b19d109]
- Updated dependencies [6514e2a]
- Updated dependencies [e2100a1]
- Updated dependencies [0edf4b9]
- Updated dependencies [5f7643a]
- Updated dependencies [ed30c31]
- Updated dependencies [8171e78]
  - @interfere/react@11.0.0-canary.3
  - @interfere/types@9.0.3-canary.2
  - @interfere/sdk@11.0.0-canary.3

## 11.0.0-canary.2

### Patch Changes

- 8b1c635: Declare OpenAPI security schemes on the public collector contract. The SDK now exposes first-class auth: `publishableKey` (`?pk=…`) as a constructor option for ingestion operations, and per-operation `{ apiKey }` security on release management methods (`releases.*`, `sourceMaps.*`, `telemetry.report`) — replacing the custom `httpClient` interceptor pattern. The next/vite plugins pass the typed security argument internally.
- Updated dependencies [a23cf2a]
- Updated dependencies [4893278]
- Updated dependencies [0d53d41]
- Updated dependencies [b1e24c8]
- Updated dependencies [8b1c635]
- Updated dependencies [8b1c635]
  - @interfere/react@11.0.0-canary.2
  - @interfere/types@9.0.3-canary.1
  - @interfere/sdk@11.0.0-canary.2

## 10.0.1-canary.1

### Patch Changes

- Updated dependencies [[`8d043a1`](https://github.com/interfere-inc/interfere/commit/8d043a131d13dc976325a3af21e71513970555f4)]:
  - @interfere/sdk@10.0.1-canary.1
  - @interfere/react@10.0.1-canary.1

## 10.0.1-canary.0

### Patch Changes

- Updated dependencies [[`e62516e`](https://github.com/interfere-inc/interfere/commit/e62516eac5997c3de92160bae1052b68e2585520)]:
  - @interfere/constants@9.0.2-canary.0
  - @interfere/types@9.0.3-canary.0
  - @interfere/react@10.0.1-canary.0
  - @interfere/sdk@10.0.1-canary.0

## Unreleased

### Major changes — Vite and TanStack Start support

The Vite SDK gains a server runtime, automatic source-map upload, an auto-detecting plugin mode, and a proxy handler that mirrors the `@interfere/next` proxy contract for TanStack Start / Nitro apps.

#### New exports

- **`@interfere/vite/provider`** — `<InterfereProvider>` that wraps `useSyncExternalStore` + the React-package provider. Customers drop the boilerplate they used to copy from the sandbox apps.

- **`@interfere/vite/server`** — `register()`, `captureError`, `TraceMeta`. Server-side Node OTel bootstrap mirroring `@interfere/next/instrumentation` so TanStack Start / Nitro customers get full server-side error capture, console→OTel bridging, and trace stitching.

- **`@interfere/vite/handler`** — Web-Standards `Request → Response` proxy handler. Customers mount it at `routes/api/interfere/$.ts` (TanStack Start) or any framework with a catch-all route.

#### Plugin upgrades

- New **`mode: "auto" | "direct" | "proxy"`** option on `interfere()`. Default is `"auto"`: detects `tanstack-start` / `nitro` plugins and switches to proxy mode automatically; otherwise stays in direct mode for vanilla Vite SPAs.

- **Source-map upload at `vite build`**. Requires `INTERFERE_API_KEY`. Walks the dist tree, injects debug IDs, signs and PUTs to R2, completes the manifest, preflights the release, and cleans up `.js.map` files locally. Walks both the client and server sub-bundles in TanStack Start + Nitro under one release.

- **`VITE_INTERFERE_FORCE_ENABLE`** plumbing — stamps `__INTERFERE_FORCE_ENABLE__` so the kernel honours it without `process.env`.

#### Internal

- Build-time pipeline (discover, release, sign, PUT, complete, preflight, cleanup) is implemented inside the Vite SDK. It does not depend on `@interfere/next` or any shared SDK build pipeline.

- Wire-format change: `ManifestBundler` enum gains `"rollup"` for Vite/Rolldown source-map manifests. Existing `webpack`, `turbopack`, and `tsc` producers are unaffected.

## 0.1.2

### Patch Changes

- Updated dependencies []:
  - @interfere/react@10.0.0
  - @interfere/types@10.0.0

## 0.1.1

### Patch Changes

- [`669f5e4`](https://github.com/interfere-inc/interfere/commit/669f5e40c24cf47b4335b8e51df09efe40112d29) Thanks [@skve](https://github.com/skve)! - Added telemetry for per-sdk breakdowns

- Updated dependencies [[`669f5e4`](https://github.com/interfere-inc/interfere/commit/669f5e40c24cf47b4335b8e51df09efe40112d29)]:
  - @interfere/react@8.1.2
  - @interfere/types@8.1.2

## 0.1.0

### Minor Changes

- [#1485](https://github.com/interfere-inc/interfere/pull/1485) [`fdeba54`](https://github.com/interfere-inc/interfere/commit/fdeba54ce98e2fdf8211b497011042d1364f06ed) Thanks [@pjsny](https://github.com/pjsny)! - Initial npm release of the Vite plugin and `@interfere/vite/init` on the `alpha` dist-tag. Install with `@interfere/vite@alpha`.

### Patch Changes

- Updated dependencies [[`e3b34e8`](https://github.com/interfere-inc/interfere/commit/e3b34e8bb42717aa5058286cb5683c0d8b068497)]:
  - @interfere/types@8.1.0
  - @interfere/react@8.1.0
