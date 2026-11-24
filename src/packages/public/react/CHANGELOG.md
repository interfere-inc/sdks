# @interfere/react

## 13.1.4

No changes in this release.

## 13.1.3

### Patch Changes

- 7358d82: `@opentelemetry/api` is now a peer dependency (`^1.9.0`) instead of an exact dependency, so applications on any 1.x release of the API share one copy with the SDK instead of installing a second one.
- Updated dependencies [7358d82]
  - @interfere/types@11.2.2

## 13.1.2

### Patch Changes

- 72fabef: Node telemetry exporters negotiate HTTP/2 with compatible collectors and support HTTP/1.1, and browser fetch and XHR tracing sanitizes request URLs before they reach span processors.

- e8e3357: The React, Next.js, and Vite integrations now use OpenTelemetry 3 for tracing and log export, including in applications that already configure their own trace provider.
- 5d39a3c: React component frames from production builds keep their source URL, line, and column, so Interfere maps each component frame to your source code.

- Updated dependencies [64e0dab]
- Updated dependencies [5d39a3c]
- Updated dependencies [64e0dab]
- Updated dependencies [989e833]
  - @interfere/helpers@9.3.6
  - @interfere/types@11.2.1

### Network

- Headers:
  - 72fabef: Node collector requests identify the Interfere producer version in the `User-Agent` header.
- Retries:
  - 72fabef: Node exporters use Undici to retry transient collector failures within the configured export deadline, honoring `Retry-After`.
- Data sent and consent:
  - 5d39a3c: each React component frame in a captured exception includes the URL, line, and column of the component code, in the frames for Chrome, Firefox, and Safari.

## 13.1.1

### Patch Changes

- 4ac308b: A device fingerprint that finishes computing after the SDK is torn down, for example when the provider unmounts, is no longer written to `localStorage`.
- Updated dependencies [3ef4bdb]
  - @interfere/types@11.2.0
  - @interfere/javascript@0.6.0

## 13.1.0

### Minor Changes

- 9aaddd6: Server and browser telemetry from built bundles now report the `production` environment, or the Wrangler environment of a `CLOUDFLARE_ENV` build, without `INTERFERE_ENVIRONMENT`; set it only for other preview or staging deploys.
- 6e18c76: When release registration succeeds, `@interfere/vite` bakes its release token into the browser bundle for the browser SDK to use without exposing the build secret.

- 8390a99: Build and browser warnings now talk about services and release tokens: `withInterfere()` and `interfere()` no longer print the inferred service name mismatch warning, the Next.js build log links to the Services page, a Next.js build with a public key but no `instrumentation-client` file warns instead of failing, and the browser SDK's credential warnings point to building with `INTERFERE_API_KEY`.
- c0c40ed: Browser network errors such as `Failed to fetch` now record whether the browser was offline and whether the page was hidden or being left, so Interfere can tell a visitor's network from a failure in your app.

### Patch Changes

- 4d63981: Every public package is now published with npm trusted publishing and a provenance attestation.
- b8c5bad: The browser SDK no longer bundles zod, which makes the eager browser bundle about 25 KiB smaller gzipped, and Interfere now leaves `traceparent` to a Sentry client configured with `propagateTraceparent`.

- d1b1138: Breaking: consent is now one option and one setter, `init({ consent: "required" })` and `consent.set()`, which replace the boolean and object `consent` options and keep the SDK from storing anything on the device until consent is granted.

- 615b212: Backend hosts discovered from your workspace settings now receive trace headers only after a one-time CORS check shows they accept them, so their requests no longer fail on a header their `Access-Control-Allow-Headers` doesn't list.

- 050d678: The `@interfere/next` and `@interfere/react` READMEs are rewritten for people installing the SDK, and the React quick start now uses the Vite provider.
- 9ab763e: Bot detection now uses `isbot`, which makes the browser bundle about 20 KiB smaller gzipped and also skips uptime monitors such as Pingdom.
- 2876c24: Browser bundles now carry one copy of each OpenTelemetry package, which makes the telemetry chunk about 1.1 KiB smaller gzipped and stops most of a page's spans and log records from being lost before export.

- 2c82a56: Device fingerprinting no longer causes long tasks, and each browser's `fpHash` changes once after upgrading.

- 3de6985: Rage-click detection no longer keeps removed elements in memory after a route change.

- 934e614: Session replay options now take effect as documented: `blockClass`, `maskTextClass` and `ignoreClass` apply, an invalid selector is dropped with a warning, full snapshots are taken every 5 minutes and `recordCanvas` records WebP at two frames a second.

- d4da60c: Session replay holds less memory, no longer uploads unplayable gaps, records nothing for anonymous visitors when the replay policy is `never` and cancels uploads in progress when consent is revoked.

- b191ac7: Session replay now loads only rrweb's recorder, not its player, which makes the lazy replay chunk about 23 KiB smaller gzipped.
- 9aaddd6: Request spans no longer carry the browser's resource-timing events (`fetchStart` through `responseEnd`), which makes each request span sent when a page closes up to about 900 bytes smaller.
- d1b1138: Breaking: `tracing.mode` is removed, request spans now come from the OpenTelemetry fetch and XMLHttpRequest instrumentations in every setup, and Interfere leaves `traceparent` to Datadog, Sentry or an OpenTelemetry propagator that already writes it.

- bb1a76b: The device fingerprint is now computed once per device and kept in `localStorage`, so later page loads skip loading FingerprintJS and running its probes.

- a8d5250: Telemetry that is still being sent when a visitor leaves the page is now resent on the way out instead of being lost.

- Updated dependencies [4d63981]
- Updated dependencies [b8c5bad]
- Updated dependencies [9aaddd6]
- Updated dependencies [d1b1138]
- Updated dependencies [db2b560]
- Updated dependencies [ead7ecb]
- Updated dependencies [042188e]
- Updated dependencies [6e18c76]
- Updated dependencies [4eb17ec]
- Updated dependencies [ca4d372]
- Updated dependencies [ec31e5d]
- Updated dependencies [4eb17ec]
- Updated dependencies [e820a7f]
- Updated dependencies [f2e532d]
- Updated dependencies [5f28fcd]
- Updated dependencies [b8baffc]
- Updated dependencies [0be7ad5]
- Updated dependencies [c0c40ed]
- Updated dependencies [b77bfb3]
  - @interfere/constants@9.4.0
  - @interfere/helpers@9.3.5
  - @interfere/javascript@0.5.0
  - @interfere/types@11.1.0

### Network

- Headers:
  - 6e18c76: browser `/drain/v1/{signal}` exports and `/v3/replay/upload` send `x-interfere-release-token` when the build baked a token; `/v3/config` and `/v3/session` also receive it, with legacy public-key headers retained for older deployments.
  - b8c5bad: when Sentry is configured with `propagateTraceparent` for a request's URL, Interfere no longer writes its own `traceparent` on that request.
  - d1b1138: before consent, `baggage` carries `interfere.session.id` set to a page-scoped `page_<uuidv7>` id; previously no `baggage` was sent while analytics was denied.
  - 615b212: a backend host discovered from workspace settings receives no `traceparent`, `tracestate` or `baggage` until the CORS check passes, and afterwards only the headers that passed.
  - d1b1138: when `tracing.propagation` is unset, `traceparent` and `tracestate` are no longer written to URLs that Datadog RUM, Sentry with `propagateTraceparent` or another OpenTelemetry propagator writes them to; `baggage` is unchanged.
- Origins:
  - 615b212: the check is one credentialed `HEAD` request to the request's path without its query string, and a second without `baggage` if the first is rejected, at most once per host per browser session; it carries random trace ids and no application data.
- Retries:
  - d4da60c: replay chunks that depend on a keyframe dropped from a full retry queue are dropped with it instead of being retried.
  - d1b1138: leave-time telemetry that doesn't fit the keepalive budget is delivered on the next page load instead of being dropped.
  - a8d5250: an export still in flight when the page starts leaving is cancelled and sent once more through the leave path, so its spans can arrive twice if the first request had already reached Interfere.
- Data sent and consent:
  - c0c40ed: exception events for transport failures carry up to four more attributes: `error.type`, `interfere.page.visibility`, `interfere.page.leaving` and, when offline, `network.connection.type`; no URLs or identifiers are added.
  - d1b1138: before consent, spans and logs carry the page-scoped id, no session or identify request is sent, and nothing is stored in cookies, `localStorage` or `sessionStorage`; when consent is granted, one `session.start` log record links the page id to the session.
  - 2876c24: spans and log records that were dropped before export when two OpenTelemetry copies mixed are now sent.
  - 2c82a56: the session request's `fpHash` is computed from fewer browser signals, so its value changes once per browser.
  - 3de6985: a rage click whose last clicked element has already been removed and garbage-collected reports `ui.rage_click.selector` as `unknown`.
  - 934e614: replay chunks carry a full snapshot every 5 minutes instead of every minute; elements matching your `blockClass`, `maskTextClass` or `ignoreClass` are now blocked, masked or ignored; with `recordCanvas`, canvas frames are WebP at quality 0.6 and two frames a second.
  - d4da60c: revoking consent aborts replay uploads already in progress, anonymous visitors on `never` surfaces are not recorded before they are identified, and unplayable replay gaps are no longer uploaded.
  - d1b1138: request spans sent to the trace endpoint replace `interfere.request` log records, with the same URL redaction, identity and consent rules, and each `fetch` response is cloned once to detect when its body completes.
  - bb1a76b: on repeat visits `fpHash` is sent with the first session request instead of a later update, and the SDK stores one new `localStorage` key, `interfere:fp_hash`, under the same consent rules as `interfere:device_id`.
- CORS:
  - 6e18c76: custom proxies must allow `x-interfere-release-token` on browser telemetry, configuration, session and replay requests.
  - 615b212: a discovered host must list the headers it should receive in `Access-Control-Allow-Headers`; a host that answers `Access-Control-Allow-Origin: *` fails the credentialed check and needs `tracing.propagateTo`.

## 13.0.4

### Patch Changes

- 6107ecf: The published build is now reproducible: rebuilding the same source gives a byte-identical tarball, so the package can be verified against its provenance attestation.

## 13.0.3

### Patch Changes

- 65fa924: Session replay now honours the app's anonymous replay policy. With `on_error`, a visitor who has not been identified keeps about the last minute of replay in memory and uploads nothing until an error occurs: an uncaught error, an unhandled rejection, a captured exception or a failed request. The SDK then uploads that buffered minute, marked so ingest accepts it, and records the rest of the session normally. With `never`, anonymous sessions record nothing. Identified sessions and the default `always` policy record as before, and `identify()` switches an anonymous session to normal recording.
- e6bafd2: When the ingest endpoint rejects a page's telemetry because the app filters its deployment environment (for example `development` on a production key), the SDK now stops exporting traces, logs and metrics for the rest of that page load and logs one console warning that names the filtered environment. Previously every later batch was sent and rejected again.
- 2298a72: Telemetry exports are now gzip-compressed while the page is visible, cutting the bytes the SDK sends for traces, logs and metrics. Exports sent while the page is hiding or unloading stay uncompressed so they can still leave with the page. Exporter `forceFlush()` and `shutdown()` now wait for exports that are already in flight, so disposing the SDK or flushing on hide no longer returns before those requests finish.
- f967bca: Session replay dependencies are now bundled, so installing the SDK no longer changes an application's own `rrweb` version.
- 13e0639: The SDK no longer throws into your code and flushes more reliably when the page goes away:

  - Calling `console.error` with a value whose properties throw (a revoked proxy, a proxy with throwing traps, or an error with a throwing `message` or `name` getter) no longer throws from the console call. A failing internal subscriber can no longer break your console calls or skip other subscribers.
  - A failing page-lifecycle subscriber no longer reports an uncaught error to the page and no longer stops later subscribers.
  - The SDK's own `[Interfere:*]` diagnostics still print to the console but are no longer captured as your application's exceptions or logs.
  - The SDK now flushes on every `pagehide`, including when the page enters the back-forward cache, so telemetry is sent on browsers that skip `beforeunload`, such as mobile Safari. Exports started while the page unloads are split into requests small enough for keepalive delivery. The SDK never registers an `unload` listener.

- 85292bd: A render error is now recorded once when React retries the failed render and an outer boundary or framework reports the retried error again. Errors from the same throw site reported through different capture paths during the same synchronous update are treated as one event.
- 5ba5305: Failed and network-errored `fetch` and `XMLHttpRequest` requests now record a call-site stack that begins at the application code that made the request, instead of at Interfere's own request instrumentation, in Chromium, Firefox and Safari. Request span names also collapse content-hashed static asset filenames such as `/assets/button-B2ivX0_I.js` to `/assets/{asset}.js`, so each build's chunks no longer produce a separate span name.
- 343565f: Session replay now records why it stopped. Before replay pauses or halts, the last uploaded chunk ends with an `interfere.replay-stopped` custom event whose reason is `hidden`, `idle`, `unload`, `mutation-storm` or `recorder-faults`, so a replay viewer can explain a gap instead of showing it as missing.
- 13e0639: Page-load traces no longer include the SDK's own requests to Interfere as resource spans, and `tracing.ignoreUrls` now applies to page-load resource spans as well as to request spans. Pages that load the SDK without session replay no longer download replay configuration code.
- 13e0639: Type ErrorBoundary fallback and onError values as unknown to match React's support for throwing any JavaScript value. TypeScript code that reads `error.message` or `error.stack` directly in these callbacks no longer compiles: narrow the value first, for example with `error instanceof Error`. Runtime behavior is unchanged.
- 13e0639: Use neutral examples in SDK documentation and fixtures, and describe regression tests by their behavior. Remove internal issue references from public API documentation.

  Correct the SDK 13.0 migration notes: list breaking tracing changes together, consolidate repeated entries, and clarify that the canonical browser configuration is `tracing: { mode: "auto" }`, with `tracing: "auto"` still supported. Tracing ownership and header propagation are separate options; use `tracing.propagation: "external"` when another tool writes W3C headers.

- 13e0639: Include MIT licenses and package documentation, declare tree-shaking behavior, and document the supported Node.js and framework versions. Complete server adapter package metadata and link license badges to the included license.
- 13e0639: Remove workspace-only resolution conditions from stable and canary packages so consumers using custom conditions resolve shipped runtime and type files. Qualification tests now use the same sanitized package manifests while development source resolution stays unchanged.
- 13e0639: Errors thrown from another realm, such as an iframe, keep their original stack trace in Firefox. Previously the SDK replaced it with its own stack.
- 13e0639: Expand error and lifecycle coverage in Firefox and WebKit, verify Next.js instrumentation in the Edge runtime, and cover Windows source-map output paths.
- Updated dependencies [13e0639]
- Updated dependencies [13e0639]
- Updated dependencies [13e0639]
- Updated dependencies [13e0639]
- Updated dependencies [13e0639]
  - @interfere/types@11.0.2
  - @interfere/constants@9.3.3
  - @interfere/helpers@9.3.4
  - @interfere/javascript@0.4.1

## 13.0.2

No changes in this release.

## 13.0.1

### Patch Changes

- 30a48a6: A DOM `Event` thrown or passed as a rejection reason is now recorded as a non-error exception named after the event class, with the event type and a description of its target (tag, id and URL), instead of `Non-Error captured with keys: isTrusted`.
- 76a8c1e: Removed deprecated SDK APIs:

  - Removed `PrerenderSafeIdGenerator` from `@interfere/next/instrumentation`; use `register()` to install managed tracing, or keep your provider's generator in application mode.
  - Removed the `KernelInternalOptions` re-export from `@interfere/next/instrument-client`; use `KernelOptions` for `init()` options.
  - Removed `tracing.apiHosts`; use `tracing.propagateTo` with exact origins such as `https://api.example.com`.
  - Removed the Vite provider's `errorBoundary` prop; use `captureRenderErrors`.
  - Removed `uploadSourceMapsMetadata` and `UploadSourceMapsMetadata` from `@interfere/types`; use the source-map upload session and completion schemas. Existing legacy server uploads remain supported.

- 76a8c1e: Error boundaries no longer break when a component throws something that is not an `Error`, such as `throw "message"`, `throw null` or `throw { code }`. `<ErrorBoundary>` now renders its fallback and calls `onError` with the original thrown value. `<InterfereProvider>` passes the value through to your own boundary unchanged. Both record it as an exception. The `onCaughtError`, `onUncaughtError` and `onRecoverableError` callbacks from `reactErrorHandler()` now record non-`Error` values as well, where they used to drop them.
- 76a8c1e: Keep device and session tracking working on HTTP pages and when browser storage becomes unavailable. Give duplicated tabs independent window identifiers, and support Safari 16.4 and Chrome 111 without newer Promise or Array builtins.

  Log fingerprint initialization failures as diagnostic text so console instrumentation does not capture them as customer exceptions.

- 76a8c1e: Browser `init()` now accepts `ignoreErrors`, `denyUrls`, `errors.sampleRate` and `beforeSend`, so you can drop noisy exceptions, exclude errors thrown by third-party scripts, sample error volume, and scrub or drop an exception before it leaves the page. The options apply to every capture source, including errors raised before remote configuration loads. If a `beforeSend` hook throws, the original exception is still sent.
- 76a8c1e: Redact sensitive URL values in captured console logs, limit each body to 16 KiB, and cap capture at 100 records per minute. Console log capture now follows analytics consent and stops when that consent is denied or revoked.
- 76a8c1e: Uncaught errors are captured in more cases. Thrown strings and plain objects (for example `setTimeout(() => { throw "failed" })`) are now recorded as synthetic exceptions instead of being dropped, and Errors created in another same-origin realm such as an iframe are recorded with their original name, message and stack. Capture keeps working when application code replaces `window.onerror` after initialisation without chaining the previous handler, and a failure inside Interfere's capture no longer prevents your own `onerror` handler from running. The opaque "Script error." from cross-origin scripts loaded without CORS is still ignored.

  Repeated errors are rate limited: unhandled promise rejections, `console.error` errors and React root error callbacks now record at most five events per error name and message per minute, matching uncaught errors, so an error loop can no longer flood your project.

  Failed `<video>`, `<audio>` and `<source>` loads are now reported as resource errors alongside images, scripts and stylesheets.

  In Next.js production builds, client errors caught by your `global-error.tsx` or `error.tsx` are now captured. Next.js reports these through a reference to `console.error` that it saves while the page loads, so `@interfere/next/instrumentation-client` now installs its console hook before Next.js saves that reference.

- 76a8c1e: Session replay now stops cleanly when the page produces a mutation storm: a single update with more than 10,000 DOM mutations, or one that adds more than 100,000 nodes. Previously, recording such an update could block the main thread for hundreds of milliseconds. Now replay delivers what it has recorded so far and stops for the rest of the page view.
- 76a8c1e: Exception capture handles more edge cases:

  - Browser-extension noise is dropped only when the top frame of the stack is an extension (or Safari masked) URL. App errors that merely call through extension code are now reported.
  - `AggregateError.errors` (for example from `Promise.any`) appear in the exception chain after the aggregate, followed by its `cause`. Errors that reference themselves are listed once.
  - Exception messages, stacks, cause-chain entries and non-Error payloads are bounded in length and breadth, so a very large error (such as a multi-megabyte message) is still delivered, including with keepalive while the page unloads. The limits and helpers are exported from `@interfere/types/sdk/errors/truncate`.
  - Query tokens in the file URLs of fallback and component-stack frames are redacted with the same URL redaction policy as the rest of the event.

- 30a48a6: Exceptions captured before the provider settles consent, such as resource load failures during page startup, now carry `device.id` and the browser attributes consent allows. With tracing on, the SDK holds them until consent settles, for at most five seconds or until the next flush, and the device plugin now starts in the same tick as the consent decision.
- Updated dependencies [30a48a6]
- Updated dependencies [76a8c1e]
- Updated dependencies [76a8c1e]
- Updated dependencies [76a8c1e]
- Updated dependencies [76a8c1e]
- Updated dependencies [76a8c1e]
- Updated dependencies [76a8c1e]
  - @interfere/types@11.0.1
  - @interfere/helpers@9.3.3

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

- ae599ff: Keep trace baggage enabled by default with an explicit opt-out. Add external propagation ownership and exact-request trace correlation so applications can retain session attribution while another tracing SDK owns W3C headers.

  Applications relying on automatic detection of another SDK through private globals must set `tracing.propagation: "external"` before upgrading. Without that explicit setting, independently installed request wrappers can both write trace headers. OpenTelemetry global registration is still respected.

  ### Network
  - Headers: no new header names. Baggage remains enabled by default; `propagateTraceBaggage: false` suppresses Interfere baggage injection. `propagation: external` leaves traceparent and tracestate to the application. Existing caller headers are preserved.
  - Origins, methods and credentials: existing configured propagation targets, methods and credential modes are unchanged. No new destinations or CORS probes.
  - Retries: no application requests are replayed; SDK telemetry retry behavior is unchanged.
  - Data and consent: request correlation can emit the final outgoing trace ID with request-time session/device identity through existing telemetry delivery. Consent changes and disposal suppress delayed correlation. No additional identity collection is introduced.
  - CORS: APIs receiving baggage must allow the `baggage` header. Set `propagateTraceBaggage: false` when an API does not accept it.

- e8b9987: Browser trace headers reach a backend host discovered from workspace configuration on `GET` and `HEAD` from the first request, and on other methods only after one header-bearing `GET` or `HEAD` to that origin has received a response in this browser session (memo in `sessionStorage`), so an unmodified CORS policy never breaks a first `POST`. The page origin and explicit `tracing.propagateTo` origins always carry their headers. An origin whose header-bearing request fails at the network layer before it has ever answered one is marked header-less for the browser session with one diagnostic naming `Access-Control-Allow-Headers`; nothing is retried and every request is still observed. The README section "Which headers go where" lists the rules.
- 2e4a93a: Browser `tracing.headers` chooses which headers Interfere writes to allowed origins: every one by default, a list such as `["baggage"]` restricts the page origin, configured backend hosts and bare `propagateTo` entries, and `false` writes no `traceparent`, `tracestate` or `baggage` anywhere while requests are still observed. A `tracing.propagateTo` entry `{ origin, headers }` receives exactly the listed headers. These header options and explicit external propagation ownership control SDK header writes.
- 3dfaefa: Accept `company: { tier }` in `identity.set()` and send company tier changes with session identification.
- 8f92d43: Remove the surface-level tracing owner. Browser and server tracing modes are configured in application code and default to `auto`; `managed` and `application` remain explicit overrides. Remote configuration cannot change tracing ownership.
- c76563f: `identify()` now posts as soon as it is called, with the local session and device ids and whatever fingerprint is already known; it no longer waits for fingerprinting or a country lookup. A fingerprint that resolves later reaches the device through the next session sync, and is discarded if device consent was revoked or the identity was reset in the meantime. The browser no longer requests `cloudflare.com/cdn-cgi/trace` or parses the user agent for classification: country comes from the ingest edge and browser/OS classification from the server. With `tracing: false`, captured exceptions are exported again: each becomes one OTLP exception span posted directly to the trace drain, without constructing a tracer provider, exporter chain or instrumentation. `@interfere/types` removes `sdk/geo` (`cloudflareTraceSchema`, `CloudflareTrace`), whose only consumer was the removed geo lookup. It also removes `sdk/plugins/context/browser` (`browserContextSchema`, `deviceMetadataSchema` and their types), which lost its only consumer with the browser-side user-agent parsing. Exceptions exported with `tracing: false` go through the same URL redaction as the tracing path, and at most one export is in flight with up to 32 exceptions queued behind it.
- 3226d63: Keep unsampled server-side trace context. The browser SDK now extracts an unsampled `traceparent` (flags `00`) and its `tracestate` from the SSR meta tags as served instead of inventing a new sampled root; the initial page-view event carries it, sampled or not. Also, the Vite `traceMeta()` helper emits unsampled contexts too. The browser never decides sampling from the meta: its private tracer provider declares a `ParentBased(AlwaysOn)` sampler, and the Next server registers the same sampler unless `OTEL_TRACES_SAMPLER` is set. SSR trace meta without a fresh `interfere` tracestate stamp is now rejected, so a `traceparent` served from cached HTML can no longer link visitors together. The Next adapter owns cache-safe trace context, freshness stamps and ID generation internally. `withInterfere()` enables `clientTraceMetadata` whenever the browser SDK is configured; the browser accepts only fresh, stamped server context.
- f9f5849: Correlate browser telemetry by session, page view and user action instead of one ambient page trace. Every span, log record, exception and web-vital point the browser SDK emits carries `session.id` and `interfere.view.id`. Records produced synchronously inside a click or submit handler also carry `interfere.action.id`. A view starts at page load, on every URL change (history push, replace or back/forward; hash-only changes excluded) and after 30 minutes on one URL. Each view emits one `browser.navigation` page-view event, and each admitted click or submit emits one `interfere.user_action` event, both under the `@interfere/browser-events` scope. The initial page-view event carries the server render's trace context, sampled or not, as its link to the render.

  Nothing is parented to a page or navigation span any more. Requests, `span()` calls, long tasks and document-load spans start their own traces unless an explicit parent is active, so unrelated work on one page no longer shares a trace and an explicit `ROOT_CONTEXT` stays a root. Work after an `await` inside `span()` is a new root with the same session and view. The per-page `navigation` span and the per-click interaction span are gone: page views and user actions are log events that ship on the log cadence, so traces split into more, smaller trees with fewer spans in total.

- d44b603: Send the existing service identity on SDK config, session, replay, and release requests so workspace public keys can select the App when using workspace keys. Build config caching now separates services and namespaces, and retained Vite builds preserve the service name for later release publication. Existing App keys keep working.
- 87479c2: Browser requests preserve caller-provided trace headers and record the trace ID sent to the server. When Interfere owns propagation, allowed requests without trace headers receive a fresh context. Set `tracing.propagation: "external"` when another SDK owns W3C headers; baggage remains independently configurable.

### Patch Changes

- f584c2f: Preserve every shared JavaScript chunk alias and align its debug ID with the uploaded source map. Retain initiating callsites on XHR and transport failures, server error causes, and structured thrown values. Report partial client and server source-map coverage.

  Publish source maps on Next.js 15 after its final Webpack client compilation, preserving customer hooks and waiting for publication and cleanup before prerendering.

- 38bc8fc: Move the server telemetry runtime into `@interfere/server-core`, a new runtime dependency of `@interfere/next` and `@interfere/vite`. It owns registration, provider ownership, exception normalization and delivery, URL privacy, console bridging and bounded provider flushing, so concurrent server initialization emits each log once and captured exceptions export once. Next keeps its framework error hooks and Vercel drain integration; Vite keeps Undici request instrumentation and metrics.

  Vite redacts sensitive URL query values, applies the outbound propagation policy when it injects headers, and exposes `flush()` with the shared privacy options. React initialization and close have explicit lifecycle states, clean up after a failed startup, keep consent when the provider omits the consent prop, and dispose page-wide Web Vitals observers with their kernel.

- 68a9ee0: Remove the SDK 9 event envelope types, which no SDK 10 code path sends. `@interfere/types` no longer exports `sdk/envelope`, `sdk/plugins/payload/page-events`, `sdk/plugins/payload/rage-click`, `sdk/plugins/payload/replay`, `sdk/plugins/context/edge`, `sdk/plugins/context/node`, `errorEnvelopePayloadSchema` or `EVENT_TYPE_TO_PLUGIN`. `@interfere/react` drops its unused per-event consent lookup.
- 2938037: Use an exact-origin set for browser request propagation and share the server correlation baggage processor across Next and Vite, preserving Next's correlationSpanProcessor export.
- a1bbc63: Stamp capture-only telemetry exports with their send time so ingestion can measure delivery lateness when tracing is disabled.
- 6a1805f: Resume hidden-page telemetry exports when a replay upload releases keepalive capacity, so retained trace batches can leave before the page closes.
- 420d2c0: Document what remote configuration can change, how packaged plugins load, and which tracing settings remain controlled by application code.
- 9fc86e9: Allow a replacement telemetry exporter to drain retained data when its disposed predecessor is still waiting for the same scope lock.
- d9c142c: Disable FingerprintJS installation telemetry while retaining local device fingerprint generation.
- 4f88820: Report automatic tracing ownership detection once per browser session at info level instead of warning on every page load. Explicit application ownership stays silent, and managed-mode ownership conflicts still warn.
- 8ad8d61: Exclude internal browser test utilities from the published package while preserving all exported runtime files and declarations.
- eec3c43: Retained hidden-page exports now carry a stable carryover id, so the ingest gateway drops a replay of an export it already accepted. An export that left the page as a keepalive request is not replayed by the next page until its own request had time to settle, which stops the duplicate traces and logs a fast navigation could produce.
- 41463e2: Document release-age exclusions and lockfile verification for fresh SDK upgrades, including transitive Interfere packages.

  ### Network

  No runtime changes: headers, destinations, retries and transmitted data are unchanged.

- a80d8cd: Failed-request capture can now flag HTTP 200 responses that carry an application error. Opt in with `captureFailedRequests: { applicationErrors: true }`; it is off by default, never delays a successful request and never retains response bodies.

  Add `requestCancellation(initiator)` and `requestRecovered(reason)` to mark a request your app cancelled on purpose and a later retry that succeeded.

- ec97a9c: Ship CHANGELOG.md inside the published tarball of every package so release notes travel with the installed version.
- a49895d: Document workspace keys, App routing, service naming, migration, and auto-create controls.
- Updated dependencies [f584c2f]
- Updated dependencies [68a9ee0]
- Updated dependencies [2938037]
- Updated dependencies [420d2c0]
- Updated dependencies [3dfaefa]
- Updated dependencies [8f92d43]
- Updated dependencies [c76563f]
- Updated dependencies [3ff22ac]
- Updated dependencies [3226d63]
- Updated dependencies [ec97a9c]
- Updated dependencies [7c8441a]
- Updated dependencies [d44b603]
  - @interfere/types@11.0.0
  - @interfere/javascript@0.4.0
  - @interfere/constants@9.3.2
  - @interfere/helpers@9.3.2

## 13.0.0-rc.7

### Patch Changes

- d9c142c: Disable FingerprintJS installation telemetry while retaining local device fingerprint generation.

## 13.0.0-rc.6

### Minor Changes

- ae599ff: Keep trace baggage enabled by default with an explicit opt-out. Add external propagation ownership and exact-request trace correlation so applications can retain session attribution while another tracing SDK owns W3C headers.

  Applications relying on automatic detection of another SDK through private globals must set `tracing.propagation: "external"` before upgrading. Without that explicit setting, independently installed request wrappers can both write trace headers. OpenTelemetry global registration is still respected.

  ### Network
  - Headers: no new header names. Baggage remains enabled by default; `propagateTraceBaggage: false` suppresses Interfere baggage injection. `propagation: external` leaves traceparent and tracestate to the application. Existing caller headers are preserved.
  - Origins, methods and credentials: existing configured propagation targets, methods and credential modes are unchanged. No new destinations or CORS probes.
  - Retries: no application requests are replayed; SDK telemetry retry behavior is unchanged.
  - Data and consent: request correlation can emit the final outgoing trace ID with request-time session/device identity through existing telemetry delivery. Consent changes and disposal suppress delayed correlation. No additional identity collection is introduced.
  - Deployment and CORS: deploy backend identity indexing/read support before relying on baggage-free association. APIs receiving baggage must allow it in CORS. Local real-package browser fixtures and built-artifact probes passed; customer API preview acceptance remains required before customer adoption.

### Patch Changes

- 420d2c0: Document what remote configuration can change, how packaged plugins load, and which tracing settings remain controlled by application code.
- 9fc86e9: Allow a replacement telemetry exporter to drain retained data when its disposed predecessor is still waiting for the same scope lock.
- Updated dependencies [420d2c0]
  - @interfere/javascript@0.4.0-rc.2

## 13.0.0-rc.5

### Patch Changes

- a49895d: Document workspace keys, App routing, service naming, migration, and auto-create controls.

## 13.0.0-rc.4

No changes in this release.

## 13.0.0-rc.3

### Patch Changes

- 8ad8d61: Exclude internal browser test utilities from the published package while preserving all exported runtime files and declarations.
- 41463e2: Document release-age exclusions and lockfile verification for fresh SDK upgrades, including transitive Interfere packages.

  ### Network

  No runtime changes: headers, destinations, retries and transmitted data are unchanged.

- Updated dependencies [7c8441a]
  - @interfere/types@11.0.0-rc.2

## 13.0.0-rc.2

### Minor Changes

- e8b9987: Browser trace headers reach a backend host discovered from workspace configuration on `GET` and `HEAD` from the first request, and on other methods only after one header-bearing `GET` or `HEAD` to that origin has received a response in this browser session (memo in `sessionStorage`), so an unmodified CORS policy never breaks a first `POST`. The page origin and explicit `tracing.propagateTo` origins always carry their headers. An origin whose header-bearing request fails at the network layer before it has ever answered one is marked header-less for the browser session with one diagnostic naming `Access-Control-Allow-Headers`; nothing is retried and every request is still observed. The README section "Which headers go where" lists the rules.
- 2e4a93a: Browser `tracing.headers` chooses which headers Interfere writes to allowed origins: every one by default, a list such as `["baggage"]` restricts the page origin, configured backend hosts and bare `propagateTo` entries, and `false` writes no `traceparent`, `tracestate` or `baggage` anywhere while requests are still observed. A `tracing.propagateTo` entry `{ origin, headers }` receives exactly the listed headers. The vendor back-off and these options are the only ways to stop header writes.
- 3dfaefa: Accept `company: { tier }` in `identity.set()` and send company tier changes with session identification.
- 8f92d43: Remove the surface-level tracing owner. The SDKs no longer read a build-baked or remote `tracingOwnership`, and the warning about an option that differs from the surface setting is gone. The browser `tracing.mode` and the Next server `tracing` option keep `auto` as the default; the Vite server option keeps `managed`; `managed` and `application` in code are the only overrides.
- 87479c2: Every browser tracing mode writes `traceparent` and `tracestate` to allowed-origin requests that carry none, including `tracing: "application"` and `auto` after it backs off for a tracing vendor. A `traceparent` another tool wrote first is never replaced, the request span or observation records the trace id the request left with, and a request sent with no active span leaves with a fresh sampled root so the server span continues a trace Interfere also recorded. The tracing mode now decides only OpenTelemetry global ownership and whether requests are recorded as spans or observations; the auto-mode diagnostic says so. The one exception keeps single-writer wires: in `auto` and `application`, Datadog RUM whose `allowedTracingUrls` propagate `tracecontext` (the default propagator set) and Sentry with `propagateTraceparent` keep `traceparent` for themselves, since both append their own header after Interfere's, and Interfere merges baggage only.

### Patch Changes

- 2938037: Use an exact-origin set for browser request propagation and share the server correlation baggage processor across Next and Vite, preserving Next's correlationSpanProcessor export.
- 6a1805f: Resume hidden-page telemetry exports when a replay upload releases keepalive capacity, so retained trace batches can leave before the page closes.
- ec97a9c: Ship CHANGELOG.md inside the published tarball of every package so release notes travel with the installed version.
- Updated dependencies [2938037]
- Updated dependencies [3dfaefa]
- Updated dependencies [8f92d43]
- Updated dependencies [ec97a9c]
  - @interfere/types@11.0.0-rc.1
  - @interfere/javascript@0.4.0-rc.1
  - @interfere/constants@9.3.2-rc.0
  - @interfere/helpers@9.3.2-rc.0

## 13.0.0-rc.1

### Minor Changes

- d44b603: Send the existing service identity on SDK config, session, replay, and release requests so workspace public keys can select the App when gateway workspace-key routing is enabled. Build config caching now separates services and namespaces, and retained Vite builds preserve the service name for later release publication. Existing App keys keep working.

### Patch Changes

- a1bbc63: Stamp capture-only telemetry exports with their send time so ingestion can measure delivery lateness when tracing is disabled.
- 4f88820: Report automatic tracing ownership detection once per browser session at info level instead of warning on every page load. Explicit application ownership stays silent, and managed-mode ownership conflicts still warn.
- Updated dependencies [d44b603]
  - @interfere/javascript@0.4.0-rc.0

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

- 68a9ee0: Remove the SDK 9 event envelope types, which no SDK 10 code path sends. `@interfere/types` no longer exports `sdk/envelope`, `sdk/plugins/payload/page-events`, `sdk/plugins/payload/rage-click`, `sdk/plugins/payload/replay`, `sdk/plugins/context/edge`, `sdk/plugins/context/node`, `errorEnvelopePayloadSchema` or `EVENT_TYPE_TO_PLUGIN`. `@interfere/react` drops its unused per-event consent lookup.
- eec3c43: Retained hidden-page exports now carry a stable carryover id, so the ingest gateway drops a replay of an export it already accepted. An export that left the page as a keepalive request is not replayed by the next page until its own request had time to settle, which stops the duplicate traces and logs a fast navigation could produce.
- a80d8cd: Failed-request capture can now flag HTTP 200 responses that carry an application error. Opt in with `captureFailedRequests: { applicationErrors: true }`; it is off by default, never delays a successful request and never retains response bodies.

  Add `requestCancellation(initiator)` and `requestRecovered(reason)` to mark a request your app cancelled on purpose and a later retry that succeeded.

- Updated dependencies [68a9ee0]
- Updated dependencies [c76563f]
- Updated dependencies [3ff22ac]
- Updated dependencies [3226d63]
  - @interfere/types@11.0.0-rc.0

## 12.4.4

### Patch Changes

- bd9b8a3: Add browser exception-capture regression coverage for navigation, page scope, explicit OpenTelemetry context, and duplicate Error objects.
- Updated dependencies [3a4f991]
- Updated dependencies [4676d54]
- Updated dependencies [e1f7ee8]
  - @interfere/javascript@0.3.3
  - @interfere/types@10.6.3

## 12.4.3

### Patch Changes

- 461082f: Browser telemetry retries transient gateway failures and splits large trace and log batches before they exceed the ingestion limit. Hidden-page exports use bounded durable carryover so queued batches survive tab closure, with consent and identity cleanup.
- 3c6ba7c: Capture bounded call sites and Next.js action-header presence for failed browser fetches, plus allowlisted response error codes for failed fetch and XHR requests without consuming application responses.
- 6085507: Improve replay recovery across page navigation and tab closure while respecting replay consent. Preserve upload acknowledgments and distinguish recordings awaiting delivery from completed uploads.
- 2f2934a: Enable development telemetry when a public key is configured and remove undocumented environment overrides in favor of existing SDK options and automatic defaults.
- 4ecf0c8: Session replay sends the final recording segment while the page is still unloading, instead of waiting for a cross-tab lock the closing page is never granted. The retained copy still covers a failed or interrupted upload, which the next page on the site delivers.
- 2aac9f5: Prevent automatic CORS recovery from submitting customer mutations more than once. Only GET and HEAD requests may be retried without trace headers.
- c2428dd: Keep automatic tracing within the correct site boundaries on shared hosting platforms. Preserve discovered backend hosts in production builds so browser requests can connect to backend traces without manual host configuration.
- c1fa96c: Avoid adding automatic cross-origin trace headers to XMLHttpRequest calls, preventing tracing from breaking customer requests. Same-origin tracing, automatic fetch tracing, and explicit customer propagation settings are preserved.
- 8948010: Account switches now update the identified user, and a session created after inactivity retains the current user. Session expiry respects activity in other tabs and rotates on return from system sleep, including browsers whose monotonic clock pauses while the device sleeps.
- 4c794cb: Keep session replay recording through temporary network outages and retry failed uploads without changing their session or segment identity. Retain a bounded backlog while preserving fatal authentication and oversized-upload handling and existing privacy defaults.
- Updated dependencies [d5a2d36]
- Updated dependencies [461082f]
- Updated dependencies [2f2934a]
- Updated dependencies [c2428dd]
  - @interfere/types@10.6.2
  - @interfere/javascript@0.3.2
  - @interfere/helpers@9.3.1
  - @interfere/constants@9.3.1

## 12.4.2

### Patch Changes

- 11b1ec6: A cross-origin request that fails because the server's CORS preflight rejects `traceparent`, `tracestate` or `baggage` is now retried once without those headers instead of failing. Interfere then stops adding trace headers to that origin for the session and logs one `console.warn` naming the `Access-Control-Allow-Headers` fix.

## 12.4.1

### Patch Changes

- 8381980: Browser requests now carry trace context to the page's own site (same registrable domain, any subdomain or port) and to backend hosts Interfere reports in remote config, so browser-to-backend correlation works without `tracing.apiHosts`. `apiHosts` remains an additive override; third-party hosts never receive the headers.
- Updated dependencies [8381980]
  - @interfere/types@10.6.1

## 12.4.0

### Minor Changes

- b8a433b: Browser traces are now sent every 60 seconds instead of every 5, or sooner when 200 spans are waiting, so a page makes about two thirds fewer trace requests. Hiding or closing the page still sends every queued span at once. Spans still queued when a tab crashes or is killed without a hide event are lost. Logs are still sent every 5 seconds.

  The SDK no longer depends on zone.js, which removes about 40 KB of minified JavaScript and the overhead zone.js adds to every promise callback and timer. A click or submit span now ends when its synchronous event handler returns. Requests started after an `await` in the handler are children of the page span instead of the click span.

### Patch Changes

- 9dd1bf0: A build that baked SDK plugin settings from an older Interfere API, missing a newer plugin's key, now keeps those settings instead of discarding them: the browser no longer refetches `/v3/config` on every page load, and a plugin you disabled stays disabled. A missing key reads as enabled.
- 2dd3ff8: Propagate `device.id` in baggage with `session.id`, so server-side errors are attributed to the browser device and its user.
- 8ad3169: Re-send `identify` with the device id once device identity starts, so users identified before the device plugin loads are linked to the app they use.
- 48a8e64: While the browser SDK backs off after a network failure, it still sends exports that an `installIngestQueue` service worker controlling the page will queue, so offline captures reach the queue and replay after a background sync.
- 6795d30: Browser telemetry exports no longer crowd out the page's own beacons when the visitor switches tabs or leaves. Exports now use keepalive only while the page's keepalive requests stay under 32 KB in total, half the browser's 64 KiB limit, and a large batch sent while the page is hidden is split into requests of 16 KB or less. Previously one batched flush could take nearly the whole limit, so the browser rejected the next keepalive request, from Interfere or from another script on the page, before it was sent.
- af83af0: Keep browser telemetry flowing around the keepalive quota: an export the browser rejects for exceeding the keepalive quota is resent once without keepalive instead of pausing every signal for 5 s, a Retry-After wait no longer holds keepalive budget, and log batches exported while the page is hidden are split into keepalive-sized requests like traces.
- 9dd1bf0: Unmounting the Interfere provider immediately after it mounts (for example under React Strict Mode or a remounted layout) no longer leaves session replay or another lazily loaded plugin running against the disposed SDK instance.
- 76e3603: Keep `identity.set()` and `consent.set()` calls made before `init()` finishes, and apply them once the SDK is ready. Previously they were dropped, so a consent revocation from a banner that loaded early was lost.
- 698127f: React apps no longer spend extra main-thread time in `completeWork` and `commitHostUpdate` after the SDK initialises. The first initialisation performs one throwaway React mount in a detached document, taking about 0.1 ms.
- 4e6078e: Session replay now snapshots the page when a request fails while the session is paused. Since 12.3.1, an exception recorded in a hidden or idle tab triggers one full DOM snapshot, but failed fetch and XMLHttpRequest calls (`NetworkError`) and captured HTTP error responses (`HttpResponseError`) never triggered it. A background tab whose polling requests failed therefore still had no replay evidence for those errors, even though they belonged to the session. Every span that carries an exception now triggers the snapshot, with the same limit of one immediately and then at most one every five minutes per pause.
- Updated dependencies [9dd1bf0]
- Updated dependencies [c949553]
  - @interfere/types@10.6.0

## 12.3.4

### Patch Changes

- 2cc257c: Session replay uploads now send the device's wall-clock time with each upload in the `x-interfere-sent-at` header, so Interfere can line up replays from devices whose clocks run fast or slow with the errors and spans it already corrects. Previously, a replay from such a device could sit minutes, hours or days away from the errors recorded in the same session.

## 12.3.3

### Patch Changes

- 3fe50d6: Failed router prefetches are no longer reported as exceptions. A fetch carrying `Next-Router-Prefetch`, `Next-Router-Segment-Prefetch`, `Purpose: prefetch` or `Sec-Purpose: prefetch` keeps its span, marked `http.request.prefetch = true` with its status, but a network failure or error status on it no longer becomes a `NetworkError` or `HttpResponseError`. The router fetches again on navigation, so a failed prefetch never affects the visitor.

## 12.3.2

### Patch Changes

- 49d54e9: Browser telemetry exports now send the device's wall-clock time with each attempt in an `x-interfere-sent-at` header, so Interfere can correct spans, logs and metrics from visitors whose device clocks run fast or slow. Previously, such telemetry could show up hours or days away from when it really happened.
- 4734c16: `captureFailedRequests` no longer reports aborted or timed-out requests as network errors, and ignores network failures caused by leaving the page.
- 9823dce: Captured exceptions are exported immediately instead of waiting for the next five-second trace batch. Other spans keep batching.
- 2154056: Cut browser telemetry noise and request volume.

  - Spans are batched up to 200 at a time every five seconds instead of 10 at a time. While the page is hidden, a large batch is split into requests small enough to survive page unload.
  - Requests to common analytics, advertising, replay and monitoring vendors no longer produce fetch spans or resource errors. The list now matches each vendor host and its subdomains, so bare hosts such as `analytics.google.com` are covered, and adds Datadog RUM, LogRocket, FullStory, Amplitude, Segment, Hotjar, Mixpanel, Heap, Pendo, Microsoft Clarity, TikTok, Reddit, Pinterest, Spotify and LinkedIn pixels, Google Ads, Vercel Web Analytics (including its proxied paths) and proxied PostHog.
  - `tracing.ignoreUrls` now takes effect; it was previously ignored.
  - Long tasks shorter than 500 ms are no longer exported.
  - Click spans skip focusing a text field, the duplicate click a `<label>` forwards to its control, and repeat clicks on the same element within one second. Rage clicks are still recorded.
  - Routine session replay uploads no longer emit a `replay.chunk` span; only unload chunks and exception snapshots do.
  - An `AbortError` logged with `console.error` is no longer captured as an exception.

- c834d7f: The browser SDK now backs off when the ingest endpoint is unreachable, such as when an ad blocker blocks it, instead of retrying continuously.

## 12.3.1

### Patch Changes

- 6effe17: The session lifecycle is now the only source of replay pause and resume. A hidden tab, or five minutes without input, pauses the session; input or the page showing again resumes it. Rotation timing is unchanged.

  When an exception is recorded while the session is paused, the lifecycle emits `exceptionWhilePaused` (the first immediately, then at most once per five minutes per pause). Replay answers it with one full DOM snapshot. Before this, an error from a background or idle tab had no replay evidence even though it still belonged to the session. The snapshot chunk ends with an rrweb custom event, tag `interfere.paused-exception-snapshot`, whose payload `reason` is `hidden` or `idle`, and its `replay.chunk` span carries `replay.chunk.trigger=exception`.

- Updated dependencies [4d4457c]
  - @interfere/types@10.5.1
  - @interfere/javascript@0.3.1

## 12.3.0

### Patch Changes

- 07744b9: Name the real cause when browser telemetry export stops. A `RELEASE_NOT_FOUND` rejection reports that the ingest endpoint does not recognize the build's release; three consecutive `AUTH_PK_INVALID` rejections report that the page's public key was rejected, instead of both printing "rejected this build's release". A public-key rejection now pauses export for five minutes and then retries once, so a transient auth failure no longer disables telemetry for the rest of the page load.
- 6be5831: Session replay keeps recording when an upload is rejected for a reason that does not apply to the rest of the page. A segment conflict (409) now skips only that chunk; 408, 429 and other unexpected 4xx are retried like 5xx and halt only after three consecutive failures. Only 401, 403 and 413 still halt replay for the page.
- Updated dependencies [0c772cc]
  - @interfere/types@10.5.0

## 12.2.0

### Minor Changes

- c96a012: Name the span that carries a captured exception after its throw site. When no span is active at capture time the SDK used to open a zero-duration span called `interfere.exception.capture` (`interfere.captureError` on the server), so every trace showed the same opaque row. The fallback span is now named after the top in-app stack frame (`handleClick (app.js:42)`), falls back to the error's type when the stack has no usable frame, carries `interfere.capture.kind = "exception"`, and is marked `ERROR` for unhandled captures. When a span is active, the exception is still recorded as an OpenTelemetry `exception` event on it; the browser SDK now also sets that span's status to `ERROR` for unhandled captures, matching the server SDKs. Handled captures (`console.error`, error boundaries) never change a customer span's status.
- 4fff52d: Stamp a default `service.name` into Vite builds. `@interfere/vite/plugin` now derives one from the `serviceName` plugin option, then `VITE_INTERFERE_SERVICE_NAME`, then the project package name without its scope (`@acme/storefront` → `storefront`), and `@interfere/vite/init` bridges it onto the page. `@interfere/react` reads that stamped name when `init()` is called without `serviceName`, so a frontend that never set one no longer reports OpenTelemetry's `unknown_service`. An explicit `init({ serviceName })` still wins. `@interfere/types/sdk/service-name` adds the `SERVICE_NAME_GLOBAL` wire name.

### Patch Changes

- 68e9a31: Keep session replay recording when the browser rejects the page's own CSSOM calls. rrweb routes a `DOMException` thrown by `insertRule`, `deleteRule` or `replace` through the recorder fault handler, so pages whose CSS-in-JS emits vendor-only pseudo-elements (for example Chakra UI placeholder rules) hit the ten-fault limit within seconds and replay silently stopped for the rest of the page. Browser API rejections no longer count as recorder faults, and every halt now emits a `replay.halt` span with its reason.
- fe666ef: Export the session replay `replay.chunk` and `replay.halt` spans through the SDK's own tracer provider. They were created on the global OpenTelemetry tracer, which the SDK never registers, so neither span ever left the browser.
- 915503c: Deliver the session replay tail when the visitor leaves the page. Recording now pauses on `pagehide` and `visibilitychange` (the signals mobile Safari and back-forward-cached pages actually fire), resumes on `pageshow`, and flushes on `beforeunload` without stopping, so a cancelled navigation keeps recording. The first chunk is uploaded two seconds after recording starts instead of ten, so a short visit's full snapshot leaves while the page is alive; an unload chunk that exceeds the browser's 64 KiB keepalive budget is gzipped synchronously so it still fits, and a chunk that cannot fit is sent best-effort and recorded on the `replay.chunk` span with `replay.chunk.keepalive = false`.
- 67b57b8: Add `@interfere/helpers/retry` and `@interfere/helpers/retry-after`: one retry loop and one Retry-After parser (seconds or HTTP-date, optional cap). Session retry and the service-worker queue now share `isRetryableStatus`, so 408 and 425 responses are retried like 429 and 5xx.
- Updated dependencies [67b57b8]
- Updated dependencies [4fff52d]
- Updated dependencies [cb80e0a]
- Updated dependencies [47943fb]
  - @interfere/helpers@9.3.0
  - @interfere/types@10.4.0
  - @interfere/constants@9.3.0

## 12.1.4

### Patch Changes

- Updated dependencies [f9d411d]
  - @interfere/javascript@0.3.0

## 12.1.3

### Patch Changes

- 8a58c25: Keep the browser SDK disabled when no public key is configured, instead of sending requests that ingest rejects as unauthenticated, and warn once so the missing key is visible.

## 12.1.2

### Patch Changes

- dbc5398: Include MIT license files and UAParser Enterprise notices in published SDK packages.

## 12.1.1

### Patch Changes

- c4e0754: Retry telemetry exports after 429 and 503 responses within bounded deadlines, honor Retry-After delays, and allow pending retries to finish during supported flush lifecycles.
- Updated dependencies [c4e0754]
  - @interfere/helpers@9.2.1

## 12.1.0

### Patch Changes

- 820973a: Start session replay at DOMContentLoaded instead of waiting for the window load event, so slow-loading assets do not delay the first recorded snapshot.
- 4d17ef3: Add a `privacy` option that excludes private content from replay and interaction telemetry and redacts recorded URLs. URL credentials and sensitive query values are redacted by default; `privacy.urls` adjusts the query and fragment rules.
- 9a92e01: Keep overlapping telemetry exports within the browser's 64 KiB keepalive budget. Exports past the budget now use a plain fetch instead of being rejected by the browser and dropped, so captured errors are no longer lost when several exports run at once.
- da0e2e2: Fix page events, including click capture, staying off for the page when remote configuration arrives while a plugin is still loading.
- bbf8037: Improve browser tracing extensibility and clarify render-error capture configuration. Add independent source-map upload controls and ensure cleanup runs after upload failures.
- Updated dependencies [4d17ef3]
- Updated dependencies [a231cb5]
  - @interfere/types@10.3.0

## 12.0.5

### Patch Changes

- Updated dependencies [99f3752]
- Updated dependencies [8ffe6fb]
  - @interfere/types@10.2.2

## 12.0.4

### Patch Changes

- 2bea649: Improve browser-to-server session correlation.

## 12.0.3

### Patch Changes

- af4aa48: Export each SDK's default OpenTelemetry `service.name` from `@interfere/constants/service-names`. The SDKs read their defaults from it, so the reported service names are unchanged.
- Updated dependencies [af4aa48]
  - @interfere/constants@9.2.0

## 12.0.2

### Patch Changes

- 99a1f17: Support boolean consent shorthand.
- dcbcbd8: Improve reliability of queued telemetry delivery.
- b984525: Retry transient session registration failures twice while the page remains open, without waiting for another telemetry event. Cancel pending retries when the SDK is disposed or device consent changes, and keep late responses from affecting a newer session.
- Updated dependencies [99a1f17]
  - @interfere/types@10.2.1

## 12.0.1

### Patch Changes

- Updated dependencies [b52524b]
  - @interfere/javascript@0.2.1

## 12.0.0

### Major Changes

- 1f01b42: Send telemetry to `api.interfere.com` using new api routes.

### Minor Changes

- 27d080c: Fix trace stitching across SPA soft navigations.
- 01e49fd: Bug fixes and performance improvements.

### Patch Changes

- 78bf77a: Keep browser observations attached to the current page trace when the active context contains baggage without a span. Rotate expired page traces even when background callbacks retain an earlier page context. Normalize XHR span names and populate http.route using the same route templates and ID masking as fetch spans, even when failed-request capture is disabled.
- 63df2c9: Refresh dependency ranges. `@interfere/nest` now accepts NestJS 11 or 12 as a peer.
- a653b8d: Generate runtime configuration, session creation, replay upload, and telemetry ingestion methods alongside the release API. Publish these runtime operations to SDK and MCP documents without adding them to HTTP documentation.

  Use generated endpoint methods throughout the framework SDKs. Authenticate browser requests through the public-key header, preserve request timeouts and unload delivery, and report failed edge telemetry requests as failed exports.

- 65b3ebb: Use a source-map upload-session response containing the expiry and file paths for v3 uploads. Upload chunks through the generated client with a path and byte offset. Legacy signing schemas and v1 tokenized upload URLs remain available.

  Remove unused session request headers, build dependencies, and Vite release-result fields.

- a61b85d: updated react dependencies
- 4ca51e3: improve the reliability and accuracy of page-load traces
- 7d3b14e: Consolidate optional SDK and test settings while preserving configured defaults, disabled options, and omitted properties.
- 47bbcbf: Send browser telemetry producer versions through the `x-interfere-producer-version` header and remove the `_pv` query parameter.
- 78bf77a: Remove replay chunk transport from the public API client. The replay recorder now owns its internal upload transport and preserves authentication headers, compression, and failure handling. Public API discovery groups operations by product and excludes internal bookkeeping.
- 757a458: Reduce temporary iframe creation during replay startup while preserving initial snapshots and mutation recording.
- dc6f82b: Compile React components to avoid rebuilding unchanged provider context values.
- 4f9ef13: Widen React peer dependency range.
- 27d080c: Make the `analytics` consent category actually more encompassing.
- 0d1a8ef: Apply provider configuration before child consent effects so consent selected before SDK initialization is preserved when the kernel becomes available.
- 27d080c: Make the `analytics` consent category actually more encompassing.
- 27d080c: Report the real export result from the OTLP ingest exporters instead of assuming success before the request settles.
- 74c8f2b: Treat driven and headless browsers as bots so automation does not create anonymous users
- 27d080c: Discard the buffer tail when consent is revoked.
- 27d080c: Fix the `span()` helper and rage-click detection.
- aa79ebc: Register session devices before optional fingerprinting completes, so slow fingerprint probes do not delay device registration. Send fingerprint updates only while device consent remains active.
- 907a91a: Start a replay chunk at each snapshot checkout, including consecutive checkouts, so seeking can rebuild the recorded page.
- 27d080c: Reduce console.error volume during session replay on Firefox, and uncaught errors are also rate-limited per error.
- e772e67: Fix session replay recording after a tab or session resumes.
- 76e64d9: Honor initial consent before restoring device identity and starting tracing.
- 0a4bfd7: Remove raw exception messages and payloads from SDK diagnostics while preserving error types and export status.
- 2e3c112: Stop instrumenting the SDK's own OTLP exports.
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
- Updated dependencies [5b005aa]
- Updated dependencies [40501f3]
- Updated dependencies [2ec9c4f]
- Updated dependencies [b8b6902]
- Updated dependencies [0efde29]
- Updated dependencies [63df2c9]
- Updated dependencies [a653b8d]
- Updated dependencies [a653b8d]
- Updated dependencies [65b3ebb]
- Updated dependencies [47bbcbf]
- Updated dependencies [78bf77a]
- Updated dependencies [67ec54b]
- Updated dependencies [265a458]
- Updated dependencies [5ff7610]
- Updated dependencies [1f01b42]
- Updated dependencies [87fad80]
- Updated dependencies [88ce674]
- Updated dependencies [5b8542b]
- Updated dependencies [fac5408]
- Updated dependencies [27d080c]
- Updated dependencies [bbfda83]
- Updated dependencies [d25952d]
  - @interfere/types@10.2.0
  - @interfere/helpers@9.2.0
  - @interfere/constants@9.1.0
  - @interfere/javascript@0.2.0

## 11.2.1

### Patch Changes

- f1dedba: improved types
- Updated dependencies [f1dedba]
  - @interfere/constants@9.0.6
  - @interfere/helpers@9.1.2
  - @interfere/types@10.1.2

## 11.2.0

## 11.1.1

### Patch Changes

- c7f7d47: Add blank lines around multiline statements and before returns, from enforcing the internal padding-between-statements lint rule repo-wide. Whitespace only, no behavior or API difference.
- Updated dependencies [c7f7d47]
  - @interfere/constants@9.0.5
  - @interfere/helpers@9.1.1
  - @interfere/types@10.1.1

## 11.1.0

### Patch Changes

- 7b80d3f: Remove lint suppression comments that no longer suppress anything.
- 911ecd4: Update OpenTelemetry to 0.221: `BatchLogRecordProcessor` now takes a single options object, and the browser instrumentations drop `semconvStabilityOptIn` now that HTTP semantic conventions are stable.
- e347b65: Internal lint modernization: apply oxlint autofixes across previously quarantined rules (import specifier style, catch variable naming, DOM idiom updates, and related mechanical cleanups). No behavior changes intended.
- 7b22af5: Declare the `react-dom` peer range as `^19.0.0` instead of inheriting the workspace catalog.
- 9a8843a: Bump the shared Vitest catalog from 4.1.9 to 4.1.10
- Updated dependencies [1a40a00]
- Updated dependencies [7b80d3f]
- Updated dependencies [9162968]
- Updated dependencies [1949d88]
- Updated dependencies [42d939d]
- Updated dependencies [e347b65]
- Updated dependencies [18f2746]
- Updated dependencies [18f2746]
- Updated dependencies [ea785f4]
- Updated dependencies [9a8843a]
  - @interfere/types@10.1.0
  - @interfere/constants@9.0.4
  - @interfere/helpers@9.1.0

## 11.0.5

## 11.0.4

### Patch Changes

- 0a4a6f1: Trim internal BetterStack references from `_internalAdditional*` / `additional*` option doc comments. Comment-only cleanup; no API or runtime changes.
- 2ecaada: Fix remote-config and deploy-environment globals not baking into Next.js 16 Turbopack production builds. `withInterfere()` now returns a `(phase) => NextConfig` function and reads the build phase from that argument instead of the unreliable `process.env.NEXT_PHASE`. An async config function (one that resolves to a Promise) is now awaited and composed instead of throwing.

  Composition: `withInterfere()` should always be the outermost Next config wrapper.

- Updated dependencies [2ecaada]
- Updated dependencies [c6ecb1e]
- Updated dependencies [52e7a06]
- Updated dependencies [b562bb3]
  - @interfere/types@10.0.2

## 11.0.3

### Patch Changes

- Updated dependencies [d5d405c]
  - @interfere/helpers@9.0.3

## 11.0.2

### Patch Changes

- f797ab7: Session replay improvements.
- 32a1acf: bug fixes & performance improvements
- Updated dependencies [2f8aad9]
- Updated dependencies [42b109e]
- Updated dependencies [32a1acf]
  - @interfere/types@10.0.1
  - @interfere/constants@9.0.3

## 11.0.1

### Patch Changes

- 475f125: Move `consent` and `plugins` configuration from `init()` to `<InterfereProvider>`. Enhance PII handling.
- Updated dependencies [475f125]
  - @interfere/helpers@9.0.2

## 11.0.0

### Minor Changes

- 97eba37: Add `identity.clear()` to clear the linked identity and rotate to a fresh anonymous session on logout.
- b1e24c8: Improve session replay compression to further reduce bandwidth and at-rest storage. Fully backward compatible — existing recordings play back unchanged.
- 254bed9: Add `@interfere/react/sw-runtime` (`installIngestQueue`) and a `serviceWorker` init option so hosts that already run a service worker at scope `/` can fold ingest durability into it instead of registering the SDK's standalone worker (two workers can't both control `/`, and a `/api/interfere/`-scoped worker never sees page requests).

### Patch Changes

- 9883610: Republish to align packed devDependency pin with the current workspace version.
- a23cf2a: Fix browser events always resolving the `development` environment. The browser kernel now reads env vars via static `process.env.X` member expressions so bundlers can inline them (previously the whole-object lookup hit the empty browser process shim), and `withInterfere()` stamps the build-time deploy environment (`INTERFERE_ENVIRONMENT` ?? `VERCEL_ENV`) into the bundle as `__INTERFERE_ENVIRONMENT__`, which the kernel prefers over the env chain.
- 7ba7566: Add a WorkOS identify source and support a config-driven ingest URL for BYO-domain ingest.
- 6c47775: Resolve published workspace dependency ranges from package manifests.
- b19d109: Enhance exception capture in browser.
- f2281fc: Fix debugId round-trip from build plugins through the SDK to the collector for source map resolution.
- 4893278: Improve reliability for automatic browser error reporting.
- 6514e2a: `captureFailedRequests` is now on by default, capturing failed fetch/XHR requests (4xx/5xx) along with network transport-level failures. Pass `captureFailedRequests: false` to disable, or `{ statusCodes }` to tune the matched ranges.
- 2bc13b1: Fix: `identity.clear()` now rotates the `deviceId` and stamps it per span/log so post-logout events attribute to a fresh anonymous person instead of the previously-identified user.
- dc31127: Configurable ingest endpoint via `apiHost` — post telemetry to a subdomain of your own site (resolved to Interfere via DNS) or a regional endpoint. Ingest requests never carry cookies.

  **Breaking (`@interfere/next`):** proxy mode is removed. The `@interfere/next/route-handler` export is gone — delete your `app/api/interfere/[[...path]]/route.ts`. Telemetry now posts directly to Interfere; set `interfere.apiHost` for an ad-blocker-resistant subdomain setup.

- e2100a1: Capture the initial page URL as a browser navigation event during SDK startup.
- 80c1e40: Kernel improvements
- 5506f36: Stop hijacking the global OpenTelemetry `TracerProvider` / `ContextManager`.
- 5f7643a: Client-side error handling improvements.
- 0d53d41: Improve session replay upload reliability for large recordings.
- d5213b2: Stop self-tracing session replay chunk uploads by ignoring the replay upload URL in the OTel instrumentation.
- ed30c31: Improved resource load failures capture.
- 9a9e2a3: bug fixes
- 8b1c635: Resolve the deployment environment via a precedence chain — `init({ environment })` → `INTERFERE_ENVIRONMENT` → `VERCEL_ENV` → `NODE_ENV` → `development` — instead of reading `NODE_ENV` alone, and treat the environment as a free-form label. Adds an `environment` option to `init()`. Custom environments like `staging`/`canary`/`qa` now resolve correctly instead of collapsing to `unknown`.
- 8171e78: Instrument errors thrown in Next.js Server Actions, and enrich server-side error handling with route error context and captured request headers.
- d5213b2: Payload improvements.
- Updated dependencies [9883610]
- Updated dependencies [7ba7566]
- Updated dependencies [7c6bf32]
- Updated dependencies [dcdec1f]
- Updated dependencies [b19d109]
- Updated dependencies [f2281fc]
- Updated dependencies [4893278]
- Updated dependencies [6514e2a]
- Updated dependencies [ac95069]
- Updated dependencies [62cd678]
- Updated dependencies [e304910]
- Updated dependencies [dc31127]
- Updated dependencies [d265875]
- Updated dependencies [0edf4b9]
- Updated dependencies [5e12a55]
- Updated dependencies [ed30c31]
- Updated dependencies [8171e78]
- Updated dependencies [7ba7566]
  - @interfere/constants@9.0.2
  - @interfere/types@10.0.0

## 11.0.0-canary.8

### Patch Changes

- 9883610: Republish to align packed devDependency pin with the current workspace version.
- 2bc13b1: Fix: `identity.clear()` now rotates the `deviceId` and stamps it per span/log so post-logout events attribute to a fresh anonymous person instead of the previously-identified user.
- Updated dependencies [9883610]
  - @interfere/constants@9.0.2-canary.4
  - @interfere/types@10.0.0-canary.7

## 11.0.0-canary.7

### Patch Changes

- f2281fc: Fix debugId round-trip from build plugins through the SDK to the collector for source map resolution.
- dc31127: Configurable ingest endpoint via `apiHost` — post telemetry to a subdomain of your own site (resolved to Interfere via DNS) or a regional endpoint. Ingest requests never carry cookies.

  **Breaking (`@interfere/next`):** proxy mode is removed. The `@interfere/next/route-handler` export is gone — delete your `app/api/interfere/[[...path]]/route.ts`. Telemetry now posts directly to Interfere; set `interfere.apiHost` for an ad-blocker-resistant subdomain setup.

- d5213b2: Stop self-tracing session replay chunk uploads by ignoring the replay upload URL in the OTel instrumentation.
- d5213b2: Payload improvements.
- Updated dependencies [f2281fc]
- Updated dependencies [dc31127]
  - @interfere/types@10.0.0-canary.6

## 11.0.0-canary.6

### Patch Changes

- 6c47775: Resolve published workspace dependency ranges from package manifests.
- 9a9e2a3: bug fixes
- Updated dependencies [dcdec1f]
- Updated dependencies [e304910]
  - @interfere/types@10.0.0-canary.5

## 11.0.0-canary.5

### Patch Changes

- Updated dependencies [62cd678]
  - @interfere/types@9.0.3-canary.4

## 11.0.0-canary.4

### Minor Changes

- 254bed9: Add `@interfere/react/sw-runtime` (`installIngestQueue`) and a `serviceWorker` init option so hosts that already run a service worker at scope `/` can fold ingest durability into it instead of registering the SDK's standalone worker (two workers can't both control `/`, and a `/api/interfere/`-scoped worker never sees page requests).

### Patch Changes

- 80c1e40: Kernel improvements
- Updated dependencies [d265875]
  - @interfere/constants@9.0.2-canary.1
  - @interfere/types@9.0.3-canary.3

## 11.0.0-canary.3

### Minor Changes

- 97eba37: Add `identity.clear()` to clear the linked identity and rotate to a fresh anonymous session on logout.

### Patch Changes

- b19d109: Enhance exception capture in browser.
- 6514e2a: `captureFailedRequests` is now on by default, capturing failed fetch/XHR requests (4xx/5xx) along with network transport-level failures. Pass `captureFailedRequests: false` to disable, or `{ statusCodes }` to tune the matched ranges.
- e2100a1: Capture the initial page URL as a browser navigation event during SDK startup.
- 5f7643a: Client-side error handling improvements.
- ed30c31: Improved resource load failures capture.
- 8171e78: Instrument errors thrown in Next.js Server Actions, and enrich server-side error handling with route error context and captured request headers.
- Updated dependencies [b19d109]
- Updated dependencies [6514e2a]
- Updated dependencies [0edf4b9]
- Updated dependencies [ed30c31]
- Updated dependencies [8171e78]
  - @interfere/types@9.0.3-canary.2

## 11.0.0-canary.2

### Minor Changes

- b1e24c8: Improve session replay compression to further reduce bandwidth and at-rest storage. Fully backward compatible — existing recordings play back unchanged.

### Patch Changes

- a23cf2a: Fix browser events always resolving the `development` environment. The browser kernel now reads env vars via static `process.env.X` member expressions so bundlers can inline them (previously the whole-object lookup hit the empty browser process shim), and `withInterfere()` stamps the build-time deploy environment (`INTERFERE_ENVIRONMENT` ?? `VERCEL_ENV`) into the bundle as `__INTERFERE_ENVIRONMENT__`, which the kernel prefers over the env chain.
- 4893278: Improve reliability for automatic browser error reporting.
- 0d53d41: Improve session replay upload reliability for large recordings.
- 8b1c635: Resolve the deployment environment via a precedence chain — `init({ environment })` → `INTERFERE_ENVIRONMENT` → `VERCEL_ENV` → `NODE_ENV` → `development` — instead of reading `NODE_ENV` alone, and treat the environment as a free-form label. Adds an `environment` option to `init()`. Custom environments like `staging`/`canary`/`qa` now resolve correctly instead of collapsing to `unknown`.
- Updated dependencies [4893278]
  - @interfere/types@9.0.3-canary.1

## 10.0.1-canary.1

## 10.0.1-canary.0

### Patch Changes

- Updated dependencies [[`e62516e`](https://github.com/interfere-inc/interfere/commit/e62516eac5997c3de92160bae1052b68e2585520)]:
  - @interfere/constants@9.0.2-canary.0
  - @interfere/types@9.0.3-canary.0

## 10.0.0

### Patch Changes

- [#1772](https://github.com/interfere-inc/interfere/pull/1772) [`7fc320b`](https://github.com/interfere-inc/interfere/commit/7fc320b0512f4dcd222bd71b732a08ee9fad97a0) Thanks [@skve](https://github.com/skve)! - Added the ability to customize the path of the Interfere route handler (/api/interfere)

- Updated dependencies [[`7fc320b`](https://github.com/interfere-inc/interfere/commit/7fc320b0512f4dcd222bd71b732a08ee9fad97a0)]:
  - @interfere/constants@9.0.1

## 9.0.2

### Patch Changes

- [#1704](https://github.com/interfere-inc/interfere/pull/1704) [`3279b68`](https://github.com/interfere-inc/interfere/commit/3279b6880d32a0be71773c92e54cd6de79262614) Thanks [@skve](https://github.com/skve)! - Now confirms release completion explicitly and non-complete releases are excluded from finding

## 9.0.1

### Patch Changes

- [#1710](https://github.com/interfere-inc/interfere/pull/1710) [`bf784f0`](https://github.com/interfere-inc/interfere/commit/bf784f07e3b1f781814ae2330d15aa6e7b416adb) Thanks [@pjsny](https://github.com/pjsny)! - Relax peer dependency ranges to standard caret semver. `@interfere/next` now accepts `next ^16.0.0` and `react`/`react-dom ^19.0.0`, and `@interfere/react` now accepts `react`/`react-dom ^19.0.0`. Previously, narrow `>=` floors (e.g. `>=16.2.4`, `>=19.2.5`) caused npm `ERESOLVE` failures for users on slightly older 19.x or 16.x releases, even though the peers were marked optional.

## 9.0.0

### Major Changes

- normalization release

### Patch Changes

- Updated dependencies []:
  - @interfere/constants@10.0.0
  - @interfere/types@10.0.0

## 8.1.6

### Patch Changes

- [#1617](https://github.com/interfere-inc/interfere/pull/1617) [`026b2d5`](https://github.com/interfere-inc/interfere/commit/026b2d506a851aa481d63a4df4eb5512109cad0a) Thanks [@skve](https://github.com/skve)! - Overhaul browser error capture fidelity and add React 19-native integration points.

  **Noise reduction**

  - Drop structurally uninvestigatable events (no `fileName` on any frame — e.g. inline `<script>` `at <anonymous>:N:M`, `eval()`, CORS-masked `"Script error."`) at capture time instead of sending them to the pipeline.
  - Extend the browser-extension noise filter to cover Safari's `webkit-masked-url://` scheme and to walk the full `.cause` chain, not just the root exception.

  **Broader capture coverage**

  - Synthesize a stack frame from `window.onerror`'s `source`/`line`/`col` arguments when the Error object itself has a degenerate stack — recovers a usable frame for inline-script errors.
  - Coerce non-`Error` promise rejections (`Promise.reject("string")`, `Promise.reject({ obj })`) into synthetic Errors with `mechanism.synthetic = true` instead of silently dropping them.
  - Unwrap nested Errors inside plain-object rejections (`Promise.reject({ error: realErr })`, `Promise.reject({ cause: realErr })`) so the captured exception preserves the real stack and message instead of `"[object Object]"`. `cause` is preferred over other keys when multiple candidates exist.
  - Scan all arguments to `console.error` for `Error` instances, not just the first. React 18+ in production logs errors as `console.error("The above error occurred in ...", error, componentStack)`; previously we missed these entirely.
  - Raise `Error.stackTraceLimit` to 50 during plugin setup so deep React trees produce usable stacks, and restore the original value on cleanup.

  **Stack parser hardening**

  - Truncate each stack-trace line to 1 KiB before parsing, preventing catastrophic regex backtracking on pathological inputs.
  - Strip webpack's `(error: *)` frame wrappers so stack traces from webpack-devtool builds parse cleanly.
  - Honor `Error.framesToPop` — the documented Node convention that lets invariant/assertion wrappers instruct the capture layer to drop N leading frames. Non-numeric values are ignored for safety.

  **React 19 integration**

  - **New export `@interfere/react/react-error-handler`** with a `reactErrorHandler()` helper that returns `{ onCaughtError, onUncaughtError, onRecoverableError }` — wire it into `createRoot()` / `hydrateRoot()` for complete, zero-boundary capture of render-phase errors. Accepts optional user callbacks that run after capture:

    ```ts
    import { createRoot } from "react-dom/client";
    import { reactErrorHandler } from "@interfere/react/react-error-handler";

    createRoot(document.getElementById("root")!, reactErrorHandler()).render(
      <App />
    );
    ```

  - **`InterfereProvider` now auto-wraps children in a transparent capture boundary.** The boundary captures the error via `getDerivedStateFromError` and re-throws in `render()`, so the customer's existing `ErrorBoundary`, Next.js's `error.tsx`/`global-error.tsx`, or React's default unmount keep full control of the rendered UI. Opt out with `<InterfereProvider errorBoundary={false}>` if you've already wired `reactErrorHandler`.

  - `ErrorBoundary.componentDidCatch` now appends parsed `errorInfo.componentStack` frames to the captured exception (React 16–18 legacy and React 19 owner-stack formats both supported).

  **Structured `mechanism.type` taxonomy**

  BREAKING (internal): `mechanism.type` values on captured events now follow a structured `auto.<surface>.<library>.<method>` namespace so downstream classification can group on common prefixes instead of string-matching loose names. The pipeline treats this field as opaque passthrough JSONB and nothing currently branches on it.

  | Capture site                            | Before                 | After                                                 |
  | --------------------------------------- | ---------------------- | ----------------------------------------------------- |
  | Global `onerror`                        | `"onerror"`            | `"auto.browser.global_handlers.onerror"`              |
  | Global `unhandledrejection`             | `"unhandledrejection"` | `"auto.browser.global_handlers.onunhandledrejection"` |
  | `console.error` patch                   | `"console.error"`      | `"auto.browser.console.error"`                        |
  | Public `ErrorBoundary`                  | `"react"`              | `"auto.function.react.error_boundary"`                |
  | Provider `CaptureBoundary`              | `"react"`              | `"auto.function.react.capture_boundary"`              |
  | `reactErrorHandler.onCaughtError`       | `"react"`              | `"auto.function.react.caught_error"`                  |
  | `reactErrorHandler.onUncaughtError`     | `"react"`              | `"auto.function.react.uncaught_error"`                |
  | `reactErrorHandler.onRecoverableError`  | `"react"`              | `"auto.function.react.recoverable_error"`             |
  | `@interfere/next` server `captureError` | `"captureError"`       | `"auto.function.nextjs.capture_error"`                |
  | `@interfere/next` `onRequestError`      | `"onRequestError"`     | `"auto.function.nextjs.on_request_error"`             |

  The canonical values are exported as `MECHANISM_TYPE` from `@interfere/types/sdk/errors`.

- Updated dependencies [[`01e037e`](https://github.com/interfere-inc/interfere/commit/01e037e5dc3f6a013124dab44863d87a9be93029), [`026b2d5`](https://github.com/interfere-inc/interfere/commit/026b2d506a851aa481d63a4df4eb5512109cad0a)]:
  - @interfere/types@8.1.6

## 8.1.2

### Patch Changes

- [`669f5e4`](https://github.com/interfere-inc/interfere/commit/669f5e40c24cf47b4335b8e51df09efe40112d29) Thanks [@skve](https://github.com/skve)! - Added telemetry for per-sdk breakdowns

- Updated dependencies [[`669f5e4`](https://github.com/interfere-inc/interfere/commit/669f5e40c24cf47b4335b8e51df09efe40112d29)]:
  - @interfere/constants@8.1.2
  - @interfere/types@8.1.2

## 8.1.1

### Patch Changes

- [#1541](https://github.com/interfere-inc/interfere/pull/1541) [`2cb008f`](https://github.com/interfere-inc/interfere/commit/2cb008f4af34f6af83dad810af0fea99e93fbd01) Thanks [@pjsny](https://github.com/pjsny)! - Skip ingesting client errors when the root stack includes a browser extension script URL (`chrome-extension:`, `moz-extension:`, `safari-web-extension:`, `safari-extension:`, `ms-browser-extension:`), cutting noise from wallets, blockers, and similar injectors.

  Adds `shouldDropBrowserExtensionNoise()` in `@interfere/types` and applies it in the React errors plugin and `ErrorBoundary` before capture.

- Updated dependencies [[`2cb008f`](https://github.com/interfere-inc/interfere/commit/2cb008f4af34f6af83dad810af0fea99e93fbd01)]:
  - @interfere/types@8.1.1

## 8.1.0

### Minor Changes

- [`e3b34e8`](https://github.com/interfere-inc/interfere/commit/e3b34e8bb42717aa5058286cb5683c0d8b068497) Thanks [@skve](https://github.com/skve)! - Added automatic country code detection & arbitrary trait support

### Patch Changes

- Updated dependencies [[`e3b34e8`](https://github.com/interfere-inc/interfere/commit/e3b34e8bb42717aa5058286cb5683c0d8b068497)]:
  - @interfere/types@8.1.0
  - @interfere/constants@8.1.0

## 8.0.0

### Major Changes

- [`de35bcb`](https://github.com/interfere-inc/interfere/commit/de35bcbba7416339d95b4b98407e052046d6cce4) Thanks [@skve](https://github.com/skve)! - fixed race condition in session tracking

### Patch Changes

- Updated dependencies [[`de35bcb`](https://github.com/interfere-inc/interfere/commit/de35bcbba7416339d95b4b98407e052046d6cce4)]:
  - @interfere/constants@8.0.0
  - @interfere/types@8.0.0

## 7.0.0

### Major Changes

- [#1193](https://github.com/interfere-inc/interfere/pull/1193) [`61cdc18`](https://github.com/interfere-inc/interfere/commit/61cdc1846a92d4e8e23963e41c8253d22330e36c) Thanks [@skve](https://github.com/skve)! - Fixed an issue whereby the service worker was overzealous in proxying requests

## 6.0.0

### Major Changes

- [`4de11e1`](https://github.com/interfere-inc/interfere/commit/4de11e17a8a774139f47f06992b554370bae7150) Thanks [@skve](https://github.com/skve)! - Added remote config to plugin enablement

### Patch Changes

- Updated dependencies [[`4de11e1`](https://github.com/interfere-inc/interfere/commit/4de11e17a8a774139f47f06992b554370bae7150)]:
  - @interfere/constants@6.0.0
  - @interfere/types@6.0.0

## 5.0.1

### Patch Changes

- [`457d650`](https://github.com/interfere-inc/interfere/commit/457d650fc79360fc7b067893afb95aaa4a0f058a) Thanks [@skve](https://github.com/skve)! - Stop sending data if `NODE_ENV` is anything other than `production` and `NEXT_PUBLIC_INTERFERE_FORCE_ENABLE` is unset

- [`ae0d409`](https://github.com/interfere-inc/interfere/commit/ae0d4093813cd6672f9439501dafa1961da0dc25) Thanks [@skve](https://github.com/skve)! - Fixes an issue with session tracking not being correctly linked to end users

## 5.0.0

### Major Changes

- [`2ef1592`](https://github.com/interfere-inc/interfere/commit/2ef1592e1b45422faf73bfd731509d6458fcfa07) Thanks [@skve](https://github.com/skve)! - User & Session Tracking support

### Patch Changes

- Updated dependencies [[`2ef1592`](https://github.com/interfere-inc/interfere/commit/2ef1592e1b45422faf73bfd731509d6458fcfa07), [`d3614cd`](https://github.com/interfere-inc/interfere/commit/d3614cd7200702efb22dfa64a33b5109533347d1)]:
  - @interfere/constants@5.0.0
  - @interfere/types@5.0.0

## 4.0.0

### Major Changes

- fix bundle

### Patch Changes

- Updated dependencies []:
  - @interfere/constants@4.0.0
  - @interfere/types@4.0.0

## 3.0.0

### Major Changes

- [#1150](https://github.com/interfere-inc/interfere/pull/1150) [`7e882fc`](https://github.com/interfere-inc/interfere/commit/7e882fc8693b7c011c18e75aec0c352f7808062e) Thanks [@skve](https://github.com/skve)! - Updated durability during periods of service outages, performance and debugging improvements

### Patch Changes

- [`e77dde8`](https://github.com/interfere-inc/interfere/commit/e77dde8cd99f9d6ee93cba53f2be1960b1941d6f) Thanks [@skve](https://github.com/skve)! - Made session initialization much more durable

## 1.0.3

### Patch Changes

- [`8102e08`](https://github.com/interfere-inc/interfere/commit/8102e0858d1e97953a62f76f0774511955c05422) Thanks [@skve](https://github.com/skve)! - Fixed catalog dependency resolution

- Updated dependencies [[`8102e08`](https://github.com/interfere-inc/interfere/commit/8102e0858d1e97953a62f76f0774511955c05422)]:
  - @interfere/constants@1.0.3
  - @interfere/types@1.0.3

## 1.0.2

### Patch Changes

- fix types

- Updated dependencies []:
  - @interfere/constants@1.0.2
  - @interfere/types@1.0.2

## 1.0.1

### Patch Changes

- [`032905b`](https://github.com/interfere-inc/interfere/commit/032905b19f2d916d4709251cf632957f127ca917) Thanks [@skve](https://github.com/skve)! - Fixed package exports

- Updated dependencies [[`032905b`](https://github.com/interfere-inc/interfere/commit/032905b19f2d916d4709251cf632957f127ca917)]:
  - @interfere/constants@1.0.1
  - @interfere/types@1.0.1

## 1.0.0

### Major Changes

- [`17fe701`](https://github.com/interfere-inc/interfere/commit/17fe701227f2a55644eb77b254888a5a0657036d) Thanks [@skve](https://github.com/skve)! - feat: release v1

### Minor Changes

- [`949b3e2`](https://github.com/interfere-inc/interfere/commit/949b3e27002091477e4fdbd37e6c77d6de4ff780) Thanks [@skve](https://github.com/skve)! - Fixed dependency graph

- [#1096](https://github.com/interfere-inc/interfere/pull/1096) [`5e7582c`](https://github.com/interfere-inc/interfere/commit/5e7582cc38114b7af8847deb19dad4980a41182b) Thanks [@skve](https://github.com/skve)! - Added consent management features

### Patch Changes

- [`99b741d`](https://github.com/interfere-inc/interfere/commit/99b741d9cbc336d5194a4c36acd7c8afdad6d7ba) Thanks [@skve](https://github.com/skve)! - bump

- [#1116](https://github.com/interfere-inc/interfere/pull/1116) [`2070869`](https://github.com/interfere-inc/interfere/commit/207086911a6dc74b28798690208d01b24240be68) Thanks [@skve](https://github.com/skve)! - Fix infinite loop

- [`bc3cdb3`](https://github.com/interfere-inc/interfere/commit/bc3cdb362c595217c1ffe70e462c1e994a3043b1) Thanks [@skve](https://github.com/skve)! - bump

- [`6e791b6`](https://github.com/interfere-inc/interfere/commit/6e791b6e05151da43259bdb026b126949e2f6470) Thanks [@skve](https://github.com/skve)! - bump

- [`4506e8d`](https://github.com/interfere-inc/interfere/commit/4506e8d301a06600345dff51c7e141e783acee52) Thanks [@skve](https://github.com/skve)! - Updated readme

- [`2847cd7`](https://github.com/interfere-inc/interfere/commit/2847cd75cd9a067e6b265864f1ed01c01ffe8a22) Thanks [@skve](https://github.com/skve)! - Improvements, Native Next.js 16 support, and huge performance improvements.

- Updated dependencies [[`99b741d`](https://github.com/interfere-inc/interfere/commit/99b741d9cbc336d5194a4c36acd7c8afdad6d7ba), [`bc3cdb3`](https://github.com/interfere-inc/interfere/commit/bc3cdb362c595217c1ffe70e462c1e994a3043b1), [`6e791b6`](https://github.com/interfere-inc/interfere/commit/6e791b6e05151da43259bdb026b126949e2f6470), [`949b3e2`](https://github.com/interfere-inc/interfere/commit/949b3e27002091477e4fdbd37e6c77d6de4ff780), [`17fe701`](https://github.com/interfere-inc/interfere/commit/17fe701227f2a55644eb77b254888a5a0657036d), [`5e7582c`](https://github.com/interfere-inc/interfere/commit/5e7582cc38114b7af8847deb19dad4980a41182b), [`4506e8d`](https://github.com/interfere-inc/interfere/commit/4506e8d301a06600345dff51c7e141e783acee52), [`2847cd7`](https://github.com/interfere-inc/interfere/commit/2847cd75cd9a067e6b265864f1ed01c01ffe8a22)]:
  - @interfere/constants@1.0.0
  - @interfere/types@1.0.0

## 1.0.0-alpha.7

### Major Changes

- [`17fe701`](https://github.com/interfere-inc/interfere/commit/17fe701227f2a55644eb77b254888a5a0657036d) Thanks [@skve](https://github.com/skve)! - feat: release v1

### Patch Changes

- Updated dependencies [[`17fe701`](https://github.com/interfere-inc/interfere/commit/17fe701227f2a55644eb77b254888a5a0657036d)]:
  - @interfere/constants@1.0.0-alpha.5
  - @interfere/types@1.0.0-alpha.6

## 0.2.0-alpha.6

### Patch Changes

- [#1116](https://github.com/interfere-inc/interfere/pull/1116) [`2070869`](https://github.com/interfere-inc/interfere/commit/207086911a6dc74b28798690208d01b24240be68) Thanks [@skve](https://github.com/skve)! - Fix infinite loop

## 0.2.0-alpha.5

### Patch Changes

- bump

- Updated dependencies []:
  - @interfere/constants@0.2.0-alpha.4
  - @interfere/types@0.2.0-alpha.5

## 0.2.0-alpha.4

### Minor Changes

- Added consent management features

### Patch Changes

- Updated dependencies []:
  - @interfere/types@0.2.0-alpha.4

## 0.2.0-alpha.3

### Patch Changes

- bump

- Updated dependencies []:
  - @interfere/constants@0.2.0-alpha.3
  - @interfere/types@0.2.0-alpha.3

## 0.2.0-alpha.2

### Patch Changes

- bump

- Updated dependencies []:
  - @interfere/constants@0.2.0-alpha.2
  - @interfere/types@0.2.0-alpha.2

## 0.2.0-alpha.1

### Patch Changes

- Fix silent initialization issues

- Updated dependencies []:
  - @interfere/constants@0.2.0-alpha.1
  - @interfere/types@0.2.0-alpha.1

## 0.1.0-alpha.26

### Minor Changes

- [`949b3e2`](https://github.com/interfere-inc/interfere/commit/949b3e27002091477e4fdbd37e6c77d6de4ff780) Thanks [@skve](https://github.com/skve)! - Fixed dependency graph

### Patch Changes

- [`4506e8d`](https://github.com/interfere-inc/interfere/commit/4506e8d301a06600345dff51c7e141e783acee52) Thanks [@skve](https://github.com/skve)! - Updated readme

- Updated dependencies [[`949b3e2`](https://github.com/interfere-inc/interfere/commit/949b3e27002091477e4fdbd37e6c77d6de4ff780), [`4506e8d`](https://github.com/interfere-inc/interfere/commit/4506e8d301a06600345dff51c7e141e783acee52)]:
  - @interfere/constants@0.1.0-alpha.2
  - @interfere/types@0.1.0-alpha.3

## 0.1.0-alpha.1

### Patch Changes

- [`2847cd7`](https://github.com/interfere-inc/interfere/commit/2847cd75cd9a067e6b265864f1ed01c01ffe8a22) Thanks [@skve](https://github.com/skve)! - Improvements, Native Next.js 16 support, and huge performance improvements.

- Updated dependencies [[`2847cd7`](https://github.com/interfere-inc/interfere/commit/2847cd75cd9a067e6b265864f1ed01c01ffe8a22)]:
  - @interfere/constants@0.0.2-alpha.1
  - @interfere/types@0.1.0-alpha.2
