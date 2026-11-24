# @interfere/next

## 13.1.5

### Patch Changes

- 0c9298c: The SDKs now carry their own copy of the OpenTelemetry SDK and depend only on `@opentelemetry/api` (`>=1.9.0 <2`), so an app on OpenTelemetry 2.x with `@opentelemetry/api` 1.9 installs them without peer dependency conflicts or a second API copy.
- Updated dependencies [0c9298c]
  - @interfere/react@13.1.5
  - @interfere/server-core@0.2.4
  - @interfere/types@11.2.3
  - @interfere/build@0.6.4

## 13.1.4

### Patch Changes

- @interfere/react@13.1.4

## 13.1.3

### Patch Changes

- 7358d82: `@opentelemetry/api` is now a peer dependency (`^1.9.0`) instead of an exact dependency, so applications on any 1.x release of the API share one copy with the SDK instead of installing a second one.
- Updated dependencies [7358d82]
  - @interfere/react@13.1.3
  - @interfere/server-core@0.2.3
  - @interfere/types@11.2.2
  - @interfere/build@0.6.3

## 13.1.2

### Patch Changes

- 72fabef: Node telemetry exporters negotiate HTTP/2 with compatible collectors and support HTTP/1.1, and browser fetch and XHR tracing sanitizes request URLs before they reach span processors.

- 989e833: `next dev` with an `INTERFERE_API_KEY` that has the `release:write` scope and no `INTERFERE_PUBLIC_KEY` now mints a release token for the service's shared local-development release and bakes it into the dev bundle, so browser telemetry is sent in local development without a public key, and the dev server warns when it cannot mint a token.
- e8e3357: The React, Next.js, and Vite integrations now use OpenTelemetry 3 for tracing and log export, including in applications that already configure their own trace provider.
- Updated dependencies [c7b2dbd]
- Updated dependencies [64e0dab]
- Updated dependencies [72fabef]
- Updated dependencies [e8e3357]
- Updated dependencies [5d39a3c]
- Updated dependencies [64e0dab]
- Updated dependencies [989e833]
  - @interfere/server-core@0.2.2
  - @interfere/helpers@9.3.6
  - @interfere/react@13.1.2
  - @interfere/types@11.2.1
  - @interfere/build@0.6.2

### Network

- Headers:
  - 72fabef: Node collector requests identify the Interfere producer version in the `User-Agent` header.
- Retries:
  - 72fabef: Node exporters use Undici to retry transient collector failures within the configured export deadline, honoring `Retry-After`.

## 13.1.1

### Patch Changes

- 4ac308b: A device fingerprint that finishes computing after the SDK is torn down, for example when the provider unmounts, is no longer written to `localStorage`.
- f070137: A build with one malformed source map no longer leaves other source maps, or the `sourceMappingURL` comments that point to them, in the build output.
- 2279673: On Cloudflare Workers, a server flush no longer waits on a flush started by another request, so it no longer fails with "This promise was created by a request that has since ended".
- 7a535a6: On Cloudflare Workers, `register()` no longer waits on startup begun by another request, so a request that ends during cold start can no longer fail later requests with "This promise was created by a request that has since ended" or leave the SDK uninitialised.
- Updated dependencies [4ac308b]
- Updated dependencies [3ef4bdb]
- Updated dependencies [2279673]
- Updated dependencies [7a535a6]
  - @interfere/react@13.1.1
  - @interfere/types@11.2.0
  - @interfere/javascript@0.6.0
  - @interfere/server-core@0.2.1
  - @interfere/build@0.6.1

## 13.1.0

### Minor Changes

- b70adcc: On the Edge runtime the SDK no longer traces its own telemetry requests, which used to send one span after another about each export until the server stopped.

- ead7ecb: Source-map uploads now send each map's SHA-256 first and skip uploading maps your Company already stores.

- 4eb17ec: When your workspace accepts release tokens, Next.js builds no longer need a public key: with `INTERFERE_API_KEY` set, even to a workspace secret alone, `withInterfere()` registers the release before compiling and bakes its token into the bundle for the browser SDK to send on every ingest request, and a build with no repository or deploy target linked in Interfere keeps its instrumentation instead of being disabled.

- 4eb17ec: When your workspace accepts release tokens, SDK settings no longer need a public key: a build with `INTERFERE_API_KEY` and no public key bakes its settings (server tracing, log sourcing, plugins, ingest URL, backend hosts, anonymous replay) by fetching `/v3/config` with the release token it registered, the server fetches its remote plugin config with the token baked into the server bundle, and `createBuildConfigResolver` now takes a credential, `{ publicKey }` or `{ releaseToken }`, in place of the public key string.

- 4eb17ec: Breaking: `resolveReleaseRequest` now returns the release request itself and never skips a release, so builds register releases and upload source maps without a repository or deploy target linked in Interfere.

- 5f28fcd: Server runtimes without `INTERFERE_API_KEY` now send telemetry with the release token the build baked into the server bundle, so the Next.js Node and Edge runtimes and the Vite Node server pick `INTERFERE_API_KEY`, then the baked release token, then the public key.

- b8baffc: Server runtimes now send telemetry with `INTERFERE_API_KEY` when it is set, and send the public key in the `x-interfere-public-key` header instead of a URL query parameter.

- 85d4511: Builds register releases with their service identity in the request body using only `INTERFERE_API_KEY`.

- 8390a99: Build and browser warnings now talk about services and release tokens: `withInterfere()` and `interfere()` no longer print the inferred service name mismatch warning, the Next.js build log links to the Services page, a Next.js build with a public key but no `instrumentation-client` file warns instead of failing, and the browser SDK's credential warnings point to building with `INTERFERE_API_KEY`.
- 0be7ad5: A build with a workspace API key now publishes source maps for every service: `getReleasesConfig` answers with a `null` legacy `surface` field for a service that has none, and the build plugins read the config without it.

### Patch Changes

- 4d63981: Every public package is now published with npm trusted publishing and a provenance attestation.
- b8c5bad: The browser SDK no longer bundles zod, which makes the eager browser bundle about 25 KiB smaller gzipped, and Interfere now leaves `traceparent` to a Sentry client configured with `propagateTraceparent`.

- d1b1138: Breaking: consent is now one option and one setter, `init({ consent: "required" })` and `consent.set()`, which replace the boolean and object `consent` options and keep the SDK from storing anything on the device until consent is granted.

- 615b212: Backend hosts discovered from your workspace settings now receive trace headers only after a one-time CORS check shows they accept them, so their requests no longer fail on a header their `Access-Control-Allow-Headers` doesn't list.

- 050d678: The `@interfere/next` and `@interfere/react` READMEs are rewritten for people installing the SDK, and the React quick start now uses the Vite provider.
- 9ab763e: Bot detection now uses `isbot`, which makes the browser bundle about 20 KiB smaller gzipped and also skips uptime monitors such as Pingdom.
- 069bd77: `withInterfereProxy` and Next.js span recording no longer throw into your app or replace your error when Interfere can't read it.
- af313ca: `next build --turbopack` on Next.js 15.4 and later now uploads source maps.

- 2876c24: Browser bundles now carry one copy of each OpenTelemetry package, which makes the telemetry chunk about 1.1 KiB smaller gzipped and stops most of a page's spans and log records from being lost before export.

- 4365430: Release requests made at build time now send the SDK name and version in the `x-interfere-producer-version` header.

- 9aaddd6: Request spans no longer carry the browser's resource-timing events (`fetchStart` through `responseEnd`), which makes each request span sent when a page closes up to about 900 bytes smaller.
- d1b1138: Breaking: `tracing.mode` is removed, request spans now come from the OpenTelemetry fetch and XMLHttpRequest instrumentations in every setup, and Interfere leaves `traceparent` to Datadog, Sentry or an OpenTelemetry propagator that already writes it.

- c37dcbe: The READMEs now call the `release:write` permission **Source maps**, matching Settings > Developer.
- b77bfb3: Server spans now read session and device ids only from `interfere.*` baggage, so browsers still on SDK 12.x or earlier must upgrade to 13.x to keep server spans linked to their sessions.

- a8d5250: Telemetry that is still being sent when a visitor leaves the page is now resent on the way out instead of being lost.

- Updated dependencies [4d63981]
- Updated dependencies [b8c5bad]
- Updated dependencies [9aaddd6]
- Updated dependencies [d1b1138]
- Updated dependencies [615b212]
- Updated dependencies [db2b560]
- Updated dependencies [ead7ecb]
- Updated dependencies [050d678]
- Updated dependencies [9ab763e]
- Updated dependencies [4eb17ec]
- Updated dependencies [2876c24]
- Updated dependencies [2c82a56]
- Updated dependencies [3de6985]
- Updated dependencies [042188e]
- Updated dependencies [6e18c76]
- Updated dependencies [4eb17ec]
- Updated dependencies [ca4d372]
- Updated dependencies [ec31e5d]
- Updated dependencies [4eb17ec]
- Updated dependencies [4eb17ec]
- Updated dependencies [4eb17ec]
- Updated dependencies [4eb17ec]
- Updated dependencies [934e614]
- Updated dependencies [d4da60c]
- Updated dependencies [b191ac7]
- Updated dependencies [9aaddd6]
- Updated dependencies [d1b1138]
- Updated dependencies [e820a7f]
- Updated dependencies [f2e532d]
- Updated dependencies [5f28fcd]
- Updated dependencies [5f28fcd]
- Updated dependencies [b8baffc]
- Updated dependencies [85d4511]
- Updated dependencies [8390a99]
- Updated dependencies [0be7ad5]
- Updated dependencies [bb1a76b]
- Updated dependencies [c0c40ed]
- Updated dependencies [b77bfb3]
- Updated dependencies [a8d5250]
  - @interfere/build@0.6.0
  - @interfere/constants@9.4.0
  - @interfere/helpers@9.3.5
  - @interfere/javascript@0.5.0
  - @interfere/react@13.1.0
  - @interfere/server-core@0.2.0
  - @interfere/types@11.1.0

### Network

- Headers:
  - 4eb17ec: browser `/drain/v1/{signal}` exports send `x-interfere-release-token` when the build baked a token, and send no `x-interfere-public-key` when there is no public key.
  - 4eb17ec: build-time and server `GET /v3/config` send `x-interfere-release-token` instead of `x-interfere-public-key` when the build has no public key. Builds with a public key are unchanged.
  - 5f28fcd: Node and Edge server exports to `/drain/v1/{signal}` send `x-interfere-release-token` when `INTERFERE_API_KEY` is unset and the build baked a release token, with no `authorization` or `x-api-key`, and send `x-interfere-public-key` alongside it when the public key is also set; runtimes with `INTERFERE_API_KEY` set are unchanged.
  - b8baffc: when `INTERFERE_API_KEY` is set, Node server exports (Next.js, Vite) add `authorization: Bearer <INTERFERE_API_KEY>` and Next.js Edge exports send `x-api-key`; every server export sends `x-interfere-public-key` when the public key is set.
  - b8c5bad: when Sentry is configured with `propagateTraceparent` for a request's URL, Interfere no longer writes its own `traceparent` on that request.
  - d1b1138: before consent, `baggage` carries `interfere.session.id` set to a page-scoped `page_<uuidv7>` id; previously no `baggage` was sent while analytics was denied.
  - 615b212: a backend host discovered from workspace settings receives no `traceparent`, `tracestate` or `baggage` until the CORS check passes, and afterwards only the headers that passed.
  - 4365430: `x-interfere-producer-version: <package>@<version>` is added to release preflight and upload requests from the `@interfere/next` build and to release preflight requests from `@interfere/vite`.
  - d1b1138: when `tracing.propagation` is unset, `traceparent` and `tracestate` are no longer written to URLs that Datadog RUM, Sentry with `propagateTraceparent` or another OpenTelemetry propagator writes them to; `baggage` is unchanged.
- Origins:
  - b8baffc: `POST {INTERFERE_API_URL}/drain/v1/{traces,logs,metrics}` (default `https://api.interfere.com`), server to server, no longer carries the `public_key` query parameter.
  - 615b212: the check is one credentialed `HEAD` request to the request's path without its query string, and a second without `baggage` if the first is rejected, at most once per host per browser session; it carries random trace ids and no application data.
  - 4365430: the configured Interfere API origin (`/v3/releases/*`, `GET` and `POST`), called from Node at build time with the secret key; browser requests are unchanged.
- Retries:
  - d1b1138: leave-time telemetry that doesn't fit the keepalive budget is delivered on the next page load instead of being dropped.
  - a8d5250: an export still in flight when the page starts leaving is cancelled and sent once more through the leave path, so its spans can arrive twice if the first request had already reached Interfere.
- Data sent and consent:
  - b70adcc: on the Edge runtime each span is sent once, and the SDK no longer sends `fetch POST /drain/v1/traces` spans that describe its own exports.
  - ead7ecb: the sign request now carries a `sha256` for each source map, and the SDK sends no bytes for maps the response marks `present`.
  - 4eb17ec: a production build calls `POST /v3/releases` once, before compiling, instead of after; when `/v3/releases/config` is refused or unavailable, the deploy target comes from the build environment, and a build that fails afterwards leaves an unready release behind.
  - 4eb17ec: builds with no repository or deploy target linked in Interfere now register their release and upload source maps, with the deploy target read from the build environment.
  - 5f28fcd: server telemetry from a runtime with only the baked release token is attributed to the service and release named in the token, and server capture turns on for such a runtime even without a public key.
  - b8baffc: with `INTERFERE_API_KEY` set, server telemetry belongs to the workspace and its `service.name` instead of being attributed through the public key; browser requests are unchanged.
  - 85d4511: release registration sends `service.name` in the JSON body, including releases published from retained Vite build output; the build-tools resolver also accepts `service.namespace`.
  - d1b1138: before consent, spans and logs carry the page-scoped id, no session or identify request is sent, and nothing is stored in cookies, `localStorage` or `sessionStorage`; when consent is granted, one `session.start` log record links the page id to the session.
  - af313ca: Turbopack builds on Next.js 15.4 and later now send the release and source-map upload requests that Webpack builds already sent.
  - 2876c24: spans and log records that were dropped before export when two OpenTelemetry copies mixed are now sent.
  - d1b1138: request spans sent to the trace endpoint replace `interfere.request` log records, with the same URL redaction, identity and consent rules, and each `fetch` response is cloned once to detect when its body completes.
  - b77bfb3: server spans no longer copy the legacy `session.id` and `device.id` baggage keys, so vendor baggage such as Datadog's `session.id` is never recorded.
- CORS:
  - 615b212: a discovered host must list the headers it should receive in `Access-Control-Allow-Headers`; a host that answers `Access-Control-Allow-Origin: *` fails the credentialed check and needs `tracing.propagateTo`.

## 13.0.4

### Patch Changes

- Updated dependencies [6107ecf]
  - @interfere/react@13.0.4

## 13.0.3

### Patch Changes

- f967bca: The README adds the browser setup step and the supported Next.js versions. The README and the service-name build warning name `withInterfere()` as the single place to set `serviceName`.
- 13e0639: Capture errors thrown inside Next.js `after()` callbacks. Next.js only logs these to the server console and never reports them to `onRequestError`, so they were invisible. Wrap the task with the new `withInterfereAfter` export from `@interfere/next/server` (`after(withInterfereAfter(async () => { ... }))`): a throw or rejection is captured once with the `auto.function.nextjs.after` mechanism and rethrown unchanged, and the response is never delayed.
- 13e0639: Capture the `x-forwarded-for` client IP header with server request errors by default, including Next.js Node and Edge request errors and Vite request middleware failures. A client IP address is personal data in many jurisdictions; if your privacy notice or data processing terms do not cover it, set `privacy: { clientIp: false }` in server `register()` to omit this header from captured exception events. Authorization and cookie headers remain excluded.
- 13e0639: Use neutral examples in SDK documentation and fixtures, and describe regression tests by their behavior. Remove internal issue references from public API documentation.

  Correct the SDK 13.0 migration notes: list breaking tracing changes together, consolidate repeated entries, and clarify that the canonical browser configuration is `tracing: { mode: "auto" }`, with `tracing: "auto"` still supported. Tracing ownership and header propagation are separate options; use `tracing.propagation: "external"` when another tool writes W3C headers.

- 13e0639: Include MIT licenses and package documentation, declare tree-shaking behavior, and document the supported Node.js and framework versions. Complete server adapter package metadata and link license badges to the included license.
- 13e0639: Remove workspace-only resolution conditions from stable and canary packages so consumers using custom conditions resolve shipped runtime and type files. Qualification tests now use the same sanitized package manifests while development source resolution stays unchanged.
- 13e0639: A throw from a proxy wrapped with `withInterfereProxy` now records one exception on the proxy span instead of two. Next.js bundles the proxy separately from `instrumentation.ts`, so the SDK's exception deduplication state is now shared across every bundled copy of the SDK in the process.
- 13e0639: Expand error and lifecycle coverage in Firefox and WebKit, verify Next.js instrumentation in the Edge runtime, and cover Windows source-map output paths.
- 13e0639: Turbopack server source maps now name their sources the same way browser maps do. Next.js writes server-side Turbopack maps with paths relative to the map file (for example `../../../app/api/error/route.js`), which resolve to the build machine's directories instead of your project. Before upload, those sources are rewritten to `turbopack:///[project]/app/api/error/route.js`, relative to the Turbopack root, so server and browser stack frames point at the same project files. Source contents and mappings are unchanged, and the maps on disk are left as Next.js wrote them.
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

- 2f4a4e8: Errors thrown in a Next.js proxy (`proxy.ts` or `middleware.ts`) are now delivered as soon as the request ends instead of waiting for the next batch export. Next.js bundles the proxy separately from `instrumentation.ts`, so the flush registered by `register()` was not visible there; the proxy now flushes the active OpenTelemetry provider directly.
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

- 76a8c1e: Deliver telemetry in Edge runtimes that do not provide `AbortSignal.any`. Preserve caller cancellation and export deadlines while cleaning up timers and abort listeners after each export.
- 76a8c1e: Server error capture in `@interfere/next` no longer reports framework control flow as errors. `captureError`, `onRequestError`, the global Node handlers and `withInterfereProxy` now skip dynamic-rendering bailouts, prerender interruptions, hanging-promise rejections, static generation bailouts, React postpones and aborted requests, in addition to `redirect()`, `notFound()`, `forbidden()` and `unauthorized()`.

  `withInterfereServerAction` records `redirect`, `not_found`, `forbidden` or `unauthorized` as the action outcome for those calls and no longer marks the action span as failed.

  Errors reported through `onRequestError` now carry Next's route type, router kind, render source and revalidation reason as `interfere.nextjs.*` attributes. Server action failures are tagged as server actions even without `withInterfereServerAction`, and proxy or middleware failures are tagged as proxy errors. A proxy throw is recorded once when both `withInterfereProxy` and `onRequestError` see it.

  Authorization and cookie headers remain excluded from server request error capture.

  A route handler or server component throw now exports one exception instead of two. Next.js records its own exception event on its request spans; Interfere now drops that event when it captures the same error through `onRequestError` or `captureError`. Exceptions that Interfere does not capture keep Next's event.

- 76a8c1e: Server errors captured by `onRequestError` no longer hold the response open. Pages Router pages, API routes and other requests previously waited up to 2 seconds for telemetry delivery before responding. Delivery now starts as soon as the request finishes, without delaying it, and is handed to the platform's `waitUntil` when one is available. The error reaches Interfere right away instead of waiting for the next request or the batch timer.
- 76a8c1e: Remove unreferenced source maps from Next.js build output before static export and standalone packaging, including when source-map discovery fails.
- 76a8c1e: Server `register()` now prints one warning when it adopts a tracer provider another tool registered first, such as Sentry or dd-trace with its OpenTelemetry bridge. Server errors still reach Interfere, but request traces arrive only if that provider exports them over OTLP; pass `register({ tracing: "application" })` to confirm the setup and silence the message. When dd-trace traces the process without its OpenTelemetry bridge, `register()` also warns once that Datadog and Interfere record separate traces and that dd-trace owns outbound trace headers.
- 76a8c1e: Browser `init()` now accepts `ignoreErrors`, `denyUrls`, `errors.sampleRate` and `beforeSend`, so you can drop noisy exceptions, exclude errors thrown by third-party scripts, sample error volume, and scrub or drop an exception before it leaves the page. The options apply to every capture source, including errors raised before remote configuration loads. If a `beforeSend` hook throws, the original exception is still sent.
- 76a8c1e: Uncaught errors are captured in more cases. Thrown strings and plain objects (for example `setTimeout(() => { throw "failed" })`) are now recorded as synthetic exceptions instead of being dropped, and Errors created in another same-origin realm such as an iframe are recorded with their original name, message and stack. Capture keeps working when application code replaces `window.onerror` after initialisation without chaining the previous handler, and a failure inside Interfere's capture no longer prevents your own `onerror` handler from running. The opaque "Script error." from cross-origin scripts loaded without CORS is still ignored.

  Repeated errors are rate limited: unhandled promise rejections, `console.error` errors and React root error callbacks now record at most five events per error name and message per minute, matching uncaught errors, so an error loop can no longer flood your project.

  Failed `<video>`, `<audio>` and `<source>` loads are now reported as resource errors alongside images, scripts and stylesheets.

  In Next.js production builds, client errors caught by your `global-error.tsx` or `error.tsx` are now captured. Next.js reports these through a reference to `console.error` that it saves while the page loads, so `@interfere/next/instrumentation-client` now installs its console hook before Next.js saves that reference.

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

- bbd0ba0: Default server tracing to auto, adopting an existing provider without a warning. Add `propagateTo` for exact HTTPS fetch propagation origins in managed Vercel drain mode.
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

- 7c8441a: `register({ tracing: "application" })` now delivers every server error to Interfere through its own exporter, as an exception span linked to your active trace and span ids, in addition to recording it on your active span. Errors arrive even when your provider exports nowhere Interfere reads; when your provider also exports to Interfere, the host-span copy carries `interfere.exception.delivery` and ingest keeps one copy. The new `errors` option (`"interfere"` default, `"host"`) selects the host span as the only copy for providers that already export their traces to Interfere.
- 3c5fbd0: Allow server `register({ consoleBridge: { level: "warn" } })` to bridge only warnings and errors. Support debug, info, warn, and error thresholds without patching lower-level console methods; boolean options keep their existing behavior.
- bb75e71: Export correlationSpanProcessor() for application-owned OpenTelemetry providers to attach browser session and device baggage to server spans without changing tracing ownership.
- 24f378e: Automatically propagate server fetch context to configured workspace backend hosts in managed Vercel drain mode. The `propagateTo` option adds exact HTTPS origins to the build-injected host list.
- 3dfaefa: Accept `company: { tier }` in `identity.set()` and send company tier changes with session identification.
- 8f92d43: Remove the surface-level tracing owner. Browser and server tracing modes are configured in application code and default to `auto`; `managed` and `application` remain explicit overrides. Remote configuration cannot change tracing ownership.
- c76563f: `identify()` now posts as soon as it is called, with the local session and device ids and whatever fingerprint is already known; it no longer waits for fingerprinting or a country lookup. A fingerprint that resolves later reaches the device through the next session sync, and is discarded if device consent was revoked or the identity was reset in the meantime. The browser no longer requests `cloudflare.com/cdn-cgi/trace` or parses the user agent for classification: country comes from the ingest edge and browser/OS classification from the server. With `tracing: false`, captured exceptions are exported again: each becomes one OTLP exception span posted directly to the trace drain, without constructing a tracer provider, exporter chain or instrumentation. `@interfere/types` removes `sdk/geo` (`cloudflareTraceSchema`, `CloudflareTrace`), whose only consumer was the removed geo lookup. It also removes `sdk/plugins/context/browser` (`browserContextSchema`, `deviceMetadataSchema` and their types), which lost its only consumer with the browser-side user-agent parsing. Exceptions exported with `tracing: false` go through the same URL redaction as the tracing path, and at most one export is in flight with up to 32 exceptions queued behind it.
- 3226d63: Keep unsampled server-side trace context. The browser SDK now extracts an unsampled `traceparent` (flags `00`) and its `tracestate` from the SSR meta tags as served instead of inventing a new sampled root; the initial page-view event carries it, sampled or not. Also, the Vite `traceMeta()` helper emits unsampled contexts too. The browser never decides sampling from the meta: its private tracer provider declares a `ParentBased(AlwaysOn)` sampler, and the Next server registers the same sampler unless `OTEL_TRACES_SAMPLER` is set. SSR trace meta without a fresh `interfere` tracestate stamp is now rejected, so a `traceparent` served from cached HTML can no longer link visitors together. The Next adapter owns cache-safe trace context, freshness stamps and ID generation internally. `withInterfere()` enables `clientTraceMetadata` whenever the browser SDK is configured; the browser accepts only fresh, stamped server context.
- f9f5849: Correlate browser telemetry by session, page view and user action instead of one ambient page trace. Every span, log record, exception and web-vital point the browser SDK emits carries `session.id` and `interfere.view.id`. Records produced synchronously inside a click or submit handler also carry `interfere.action.id`. A view starts at page load, on every URL change (history push, replace or back/forward; hash-only changes excluded) and after 30 minutes on one URL. Each view emits one `browser.navigation` page-view event, and each admitted click or submit emits one `interfere.user_action` event, both under the `@interfere/browser-events` scope. The initial page-view event carries the server render's trace context, sampled or not, as its link to the render.

  Nothing is parented to a page or navigation span any more. Requests, `span()` calls, long tasks and document-load spans start their own traces unless an explicit parent is active, so unrelated work on one page no longer shares a trace and an explicit `ROOT_CONTEXT` stays a root. Work after an `await` inside `span()` is a new root with the same session and view. The per-page `navigation` span and the per-click interaction span are gone: page views and user actions are log events that ship on the log cadence, so traces split into more, smaller trees with fewer spans in total.

- d44b603: Send the existing service identity on SDK config, session, replay, and release requests so workspace public keys can select the App when using workspace keys. Build config caching now separates services and namespaces, and retained Vite builds preserve the service name for later release publication. Existing App keys keep working.

### Patch Changes

- f584c2f: Preserve every shared JavaScript chunk alias and align its debug ID with the uploaded source map. Retain initiating callsites on XHR and transport failures, server error causes, and structured thrown values. Report partial client and server source-map coverage.

  Publish source maps on Next.js 15 after its final Webpack client compilation, preserving customer hooks and waiting for publication and cleanup before prerendering.

  Separate source maps for Vercel deployments built from the same Git commit. Keep build-time release identities in browser and server bundles during promotion, while retaining the original commit for source ownership.

- 38bc8fc: Move the server telemetry runtime into `@interfere/server-core`, a new runtime dependency of `@interfere/next` and `@interfere/vite`. It owns registration, provider ownership, exception normalization and delivery, URL privacy, console bridging and bounded provider flushing, so concurrent server initialization emits each log once and captured exceptions export once. Next keeps its framework error hooks and Vercel drain integration; Vite keeps Undici request instrumentation and metrics.

  Vite redacts sensitive URL query values, applies the outbound propagation policy when it injects headers, and exposes `flush()` with the shared privacy options. React initialization and close have explicit lifecycle states, clean up after a failed startup, keep consent when the provider omits the consent prop, and dispose page-wide Web Vitals observers with their kernel.

- e8b9987: Browser trace headers reach a backend host discovered from workspace configuration on `GET` and `HEAD` from the first request, and on other methods only after one header-bearing `GET` or `HEAD` to that origin has received a response in this browser session (memo in `sessionStorage`), so an unmodified CORS policy never breaks a first `POST`. The page origin and explicit `tracing.propagateTo` origins always carry their headers. An origin whose header-bearing request fails at the network layer before it has ever answered one is marked header-less for the browser session with one diagnostic naming `Access-Control-Allow-Headers`; nothing is retried and every request is still observed. The README section "Which headers go where" lists the rules.
- 2938037: Use an exact-origin set for browser request propagation and share the server correlation baggage processor across Next and Vite, preserving Next's correlationSpanProcessor export.
- 575cc65: Warn during build preflight when the inferred service name differs from the resolved App slug, with guidance to preserve the existing service name during upgrades.
- 374ff57: Fetch server remote configuration once during registration in every tracing ownership mode so server error capture honors remote plugin controls. Configuration loading runs in the background and failures do not interrupt application startup.
- 7fbfa72: Accept Next.js instrumentation error hooks directly, capturing non-Error throws and normalizing array request headers without caller casts.
- 3455df7: Document kit-free application-owned tracing, request-header/query sanitization and restricted browser capture. Identify the remaining request-error-only delivery gap instead of treating a full host trace drain as an equivalent migration.

  ### Network

  No runtime changes: headers, destinations, retries and transmitted data are unchanged. The migration guide preserves existing collection restrictions and does not recommend enabling a host trace drain for errors-only capture.

- 0beef77: Clarify how to preserve existing service names during the v13 upgrade and retain application-owned exporters alongside a Vercel Trace Drain.
- 2e4a93a: Browser `tracing.headers` chooses which headers Interfere writes to allowed origins: every one by default, a list such as `["baggage"]` restricts the page origin, configured backend hosts and bare `propagateTo` entries, and `false` writes no `traceparent`, `tracestate` or `baggage` anywhere while requests are still observed. A `tracing.propagateTo` entry `{ origin, headers }` receives exactly the listed headers. These header options and explicit external propagation ownership control SDK header writes.
- 420d2c0: Document what remote configuration can change, how packaged plugins load, and which tracing settings remain controlled by application code.
- 41463e2: Document release-age exclusions and lockfile verification for fresh SDK upgrades, including transitive Interfere packages.

  ### Network

  No runtime changes: headers, destinations, retries and transmitted data are unchanged.

- ec97a9c: Ship CHANGELOG.md inside the published tarball of every package so release notes travel with the installed version.
- a49895d: Document workspace keys, App routing, service naming, migration, and auto-create controls. Clarify public-key build diagnostics.
- 87479c2: Browser requests preserve caller-provided trace headers and record the trace ID sent to the server. When Interfere owns propagation, allowed requests without trace headers receive a fresh context. Set `tracing.propagation: "external"` when another SDK owns W3C headers; baggage remains independently configurable.
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

### Patch Changes

- 3455df7: Document kit-free application-owned tracing, request-header/query sanitization and restricted browser capture. Identify the remaining request-error-only delivery gap instead of treating a full host trace drain as an equivalent migration.

  ### Network

  No runtime changes: headers, destinations, retries and transmitted data are unchanged. The migration guide preserves existing collection restrictions and does not recommend enabling a host trace drain for errors-only capture.

- @interfere/react@13.0.0-rc.4

## 13.0.0-rc.3

### Minor Changes

- 7c8441a: `register({ tracing: "application" })` now delivers every server error to Interfere through its own exporter, as an exception span linked to your active trace and span ids, in addition to recording it on your active span. Errors arrive even when your provider exports nowhere Interfere reads; when your provider also exports to Interfere, the host-span copy carries `interfere.exception.delivery` and ingest keeps one copy. The new `errors` option (`"interfere"` default, `"host"`) selects the host span as the only copy for providers that already export their traces to Interfere.

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

- 24f378e: Automatically propagate server fetch context to configured workspace backend hosts in managed Vercel drain mode. The `propagateTo` option adds exact HTTPS origins to the build-injected host list.
- 3dfaefa: Accept `company: { tier }` in `identity.set()` and send company tier changes with session identification.
- 8f92d43: Remove the surface-level tracing owner. The SDKs no longer read a build-baked or remote `tracingOwnership`, and the warning about an option that differs from the surface setting is gone. The browser `tracing.mode` and the Next server `tracing` option keep `auto` as the default; the Vite server option keeps `managed`; `managed` and `application` in code are the only overrides.

### Patch Changes

- e8b9987: Browser trace headers reach a backend host discovered from workspace configuration on `GET` and `HEAD` from the first request, and on other methods only after one header-bearing `GET` or `HEAD` to that origin has received a response in this browser session (memo in `sessionStorage`), so an unmodified CORS policy never breaks a first `POST`. The page origin and explicit `tracing.propagateTo` origins always carry their headers. An origin whose header-bearing request fails at the network layer before it has ever answered one is marked header-less for the browser session with one diagnostic naming `Access-Control-Allow-Headers`; nothing is retried and every request is still observed. The README section "Which headers go where" lists the rules.
- 2938037: Use an exact-origin set for browser request propagation and share the server correlation baggage processor across Next and Vite, preserving Next's correlationSpanProcessor export.
- 575cc65: Warn during build preflight when the inferred service name differs from the resolved App slug, with guidance to preserve the existing service name during upgrades.
- 2e4a93a: Browser `tracing.headers` chooses which headers Interfere writes to allowed origins: every one by default, a list such as `["baggage"]` restricts the page origin, configured backend hosts and bare `propagateTo` entries, and `false` writes no `traceparent`, `tracestate` or `baggage` anywhere while requests are still observed. A `tracing.propagateTo` entry `{ origin, headers }` receives exactly the listed headers. The vendor back-off and these options are the only ways to stop header writes.
- ec97a9c: Ship CHANGELOG.md inside the published tarball of every package so release notes travel with the installed version.
- 87479c2: Every browser tracing mode writes `traceparent` and `tracestate` to allowed-origin requests that carry none, including `tracing: "application"` and `auto` after it backs off for a tracing vendor. A `traceparent` another tool wrote first is never replaced, the request span or observation records the trace id the request left with, and a request sent with no active span leaves with a fresh sampled root so the server span continues a trace Interfere also recorded. The tracing mode now decides only OpenTelemetry global ownership and whether requests are recorded as spans or observations; the auto-mode diagnostic says so. The one exception keeps single-writer wires: in `auto` and `application`, Datadog RUM whose `allowedTracingUrls` propagate `tracecontext` (the default propagator set) and Sentry with `propagateTraceparent` keep `traceparent` for themselves, since both append their own header after Interfere's, and Interfere merges baggage only.
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

### Major Changes

- bbd0ba0: Default server tracing to auto when no surface owner is configured, adopting an existing provider without a warning. Add propagateTo for exact HTTPS fetch propagation origins in managed Vercel drain mode.

### Minor Changes

- 3c5fbd0: Allow server `register({ consoleBridge: { level: "warn" } })` to bridge only warnings and errors. Support debug, info, warn, and error thresholds without patching lower-level console methods; boolean options keep their existing behavior.
- bb75e71: Export correlationSpanProcessor() for application-owned OpenTelemetry providers to attach browser session and device baggage to server spans without changing tracing ownership.
- d44b603: Send the existing service identity on SDK config, session, replay, and release requests so workspace public keys can select the App when gateway workspace-key routing is enabled. Build config caching now separates services and namespaces, and retained Vite builds preserve the service name for later release publication. Existing App keys keep working.

### Patch Changes

- 7fbfa72: Accept Next.js instrumentation error hooks directly, capturing non-Error throws and normalizing array request headers without caller casts.
- 0beef77: Clarify how to preserve existing service names during the v13 upgrade and retain application-owned exporters alongside a Vercel Trace Drain.
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

- 374ff57: Fetch server remote configuration once during registration in every tracing ownership mode so server error capture honors remote plugin controls. Configuration loading runs in the background and failures do not interrupt application startup.
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
- 0c6447b: Adds additional layers of protection against Interfere SDK failures affecting your server.
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

- 59dbd9e: `register()` no longer throws when another library, such as Sentry, registers an OpenTelemetry tracer provider first. Before this fix, a Vercel app that used Trace Drains returned a 500 on every route. Interfere now logs one warning, keeps capturing server errors through its own exporter, and skips request tracing while the other provider owns the global slot.
- Updated dependencies [8381980]
  - @interfere/react@12.4.1
  - @interfere/types@10.6.1
  - @interfere/build@0.4.3

## 12.4.0

### Patch Changes

- 9dd1bf0: A build that baked SDK plugin settings from an older Interfere API, missing a newer plugin's key, now keeps those settings instead of discarding them: the browser no longer refetches `/v3/config` on every page load, and a plugin you disabled stays disabled. A missing key reads as enabled.
- 29152de: Fix the README example for composing `buildInterfereOtelKit()` with `@vercel/otel`. The example no longer passes the removed `serverTracing` option, and it keeps `registerOTel()` working when the kit returns `null`.
- 9dd1bf0: Unmounting the Interfere provider immediately after it mounts (for example under React Strict Mode or a remounted layout) no longer leaves session replay or another lazily loaded plugin running against the disposed SDK instance.
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
- 50a4118: Fix instrumentation types for CommonJS-configured projects using bundler resolution.
- Updated dependencies [8a58c25]
  - @interfere/react@12.1.3

## 12.1.2

### Patch Changes

- dbc5398: Include MIT license files and UAParser Enterprise notices in published SDK packages.
- Updated dependencies [dbc5398]
  - @interfere/react@12.1.2

## 12.1.1

### Patch Changes

- c4e0754: Retry telemetry exports after 429 and 503 responses within bounded deadlines, honor Retry-After delays, and allow pending retries to finish during supported flush lifecycles.
- Updated dependencies [c4e0754]
  - @interfere/helpers@9.2.1
  - @interfere/react@12.1.1

## 12.1.0

### Minor Changes

- a231cb5: Builds no longer fail when Interfere is unavailable or `INTERFERE_API_KEY` is missing. Adds a `disable` option. Vite removes the boolean and `"required"` forms of `sourceMaps`, and `sourceMapReadinessTimeoutMs`; use `sourceMaps: { upload: false }` or `disable: true`.

### Patch Changes

- 4d17ef3: Add a `privacy` option that excludes private content from replay and interaction telemetry and redacts recorded URLs. URL credentials and sensitive query values are redacted by default; `privacy.urls` adjusts the query and fragment rules.
- bbf8037: Improve browser tracing extensibility and clarify render-error capture configuration. Add independent source-map upload controls and ensure cleanup runs after upload failures.
- 8a20a12: Keep Next.js application builds running when a Surface has no source or deployment target configured. Warn and skip release registration and source-map uploads until setup is complete. Negotiate compressed source-map uploads so older SDKs keep receiving a compatible upload capability.
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

- 2bea649: Improve browser-to-server session correlation.
- Updated dependencies [2bea649]
  - @interfere/react@12.0.4

## 12.0.3

### Patch Changes

- de8b22f: Improve Next.js tracing reliability and compatibility with Vercel OpenTelemetry.
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
- 27d080c: Stop baking trace context into cacheable HTML, and stamp what remains.
- 67ec54b: Require source maps for release readiness.

  Align Next.js and Vite build configuration for plugins, ingest URL, server tracing, and log sourcing. Both build integrations register releases and verify upload acceptance, and only remove local maps after the server accepts the complete upload.

- 87fad80: Share deployment metadata and source-map file utilities across Next and Vite. Both integrations call the generated API client endpoints directly. Source maps are removed only after durable acceptance of the complete upload.

### Patch Changes

- 2ec9c4f: Share bounded, ordered asynchronous mapping between the build plugins. Limit Vite source-map discovery and rewriting to 16 concurrent files while preserving duplicate-map selection.

  Retain sequential retry and chunk-upload behavior under performance linting.

- b8b6902: retry throttled and transient source-map uploads instead of failing the release
- 63df2c9: Refresh dependency ranges. `@interfere/nest` now accepts NestJS 11 or 12 as a peer.
- d3c3e02: Upload source maps through the API gateway in resumable chunks.
- a653b8d: Add the generated JavaScript API client with ESM, CommonJS, and TypeScript declarations. Next and Vite call its endpoints directly for build configuration, release creation, source-map uploads, and telemetry. Remove the shared build client and its combined release-preparation method. Preserve ordered uploads, expiration checks, and cleanup only after complete server acceptance.

  Build requests use Fern's retry policy for HTTP 408, 429, and 5xx responses. Network failures and HTTP 425 are no longer retried.

  Retire the unused CLI and Speakeasy SDK packages, their generation configuration, and compatibility wrappers. Remove legacy host remapping and Collector error aliases. Update framework setup instructions to use the generated release endpoints.

- a653b8d: Generate runtime configuration, session creation, replay upload, and telemetry ingestion methods alongside the release API. Publish these runtime operations to SDK and MCP documents without adding them to HTTP documentation.

  Use generated endpoint methods throughout the framework SDKs. Authenticate browser requests through the public-key header, preserve request timeouts and unload delivery, and report failed edge telemetry requests as failed exports.

- 2ec9c4f: Read Webpack chunks with bounded concurrency during source-map discovery while preserving traversal order when chunks reference the same map.
- 481344f: Fix loading the config plugin from native `next.config.mjs` files by using an explicit file extension for the Next.js constants import.
- c86a4a9: Declare the webpack development dependency used by the build-configuration tests. Runtime behavior is unchanged.
- ae89024: Declare `buildInterfereOtelKit` and `resolveOtelConfig` with their public result types so the published declarations no longer reference Zod.
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
- 05192d8: Bound turbopack source-map discovery and stop reading whole JS chunks.

  Discovery started every file read at once, so peak memory scaled with the release instead of the work in flight. Both passes now run 16 at a time.

  The chunk scan also read each `.js` file in full to match a `//# debugId=` comment that only ever appears on the last line. It now reads a 64 KB tail and falls back to the whole file when a larger chunk has no match in that window, so the result is unchanged.

- Updated dependencies [f1dedba]
- Updated dependencies [345b3d8]
  - @interfere/constants@9.0.6
  - @interfere/helpers@9.1.2
  - @interfere/react@11.2.1
  - @interfere/types@10.1.2
  - @interfere/sdk@11.2.1

## 11.2.0

### Minor Changes

- Server runtime release-slug resolution is improved.

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

### Minor Changes

- 1949d88: `withInterfere` now bakes a build sentinel; when an outer config wrapper drops the integration, `init()` detects the missing sentinel and warns instead of shipping an inert SDK silently. Detection tolerates ESM import hoisting (the documented `import "@interfere/next/instrumentation-client"` setup re-checks after the customer module body runs), and setups that authenticate through `INTERFERE_PUBLIC_KEY` or `NEXT_PUBLIC_INTERFERE_FORCE_ENABLE` from the environment keep sending events in a degraded mode (no release association or sourcemaps) rather than being disabled.

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

- 53bcaaa: Fix `ERR_MODULE_NOT_FOUND` on install by pinning `@interfere/types` to `^10.0.2`.
  - @interfere/sdk@11.0.5
  - @interfere/react@11.0.5

## 11.0.4

### Patch Changes

- 0a4a6f1: Trim internal BetterStack references from `_internalAdditional*` / `additional*` option doc comments. Comment-only cleanup; no API or runtime changes.
- 3536c05: Fail the build when client chunks are debug-ID-stamped but no client source maps were uploaded (e.g. a coexisting tool like Sentry deleted them before Interfere ran).
- 3536c05: Detect Turbopack as the default bundler on Next 16 `next build`, and only set the Next-16+ `turbopack.debugIds` config on Next 16+ (removes a spurious Next 15 warning).
- 3536c05: `withInterfere` now preserves the sync/async shape of the config it wraps in its return type.
- 2ecaada: Fix remote-config and deploy-environment globals not baking into Next.js 16 Turbopack production builds. `withInterfere()` now returns a `(phase) => NextConfig` function and reads the build phase from that argument instead of the unreliable `process.env.NEXT_PHASE`. An async config function (one that resolves to a Promise) is now awaited and composed instead of throwing.

  Composition: `withInterfere()` should always be the outermost Next config wrapper.

- 52e7a06: Standardize credential terminology to "public key" and "secret key" in build diagnostics, CLI help, plugin warnings, and READMEs. Env var names (`INTERFERE_API_KEY`, `INTERFERE_PUBLIC_KEY`), CLI flags (`--api-key`), and diagnostic codes are unchanged.
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

- 73b4fba: Reduce `@interfere/next`'s contribution to `vercel.functions.execution`.

  - Dropped `UndiciInstrumentation` (and its `ignoreUrls` / `propagateContextUrls` options). Next.js's `patchFetch` already emits outbound fetch spans in App Router, so ours was a duplicate. Pages Router outbound fetches are no longer traced by this SDK.
  - `compression` (default `"none"`, opt-in `"gzip"`) on all three OTLP exporters.
  - OTLP `timeoutMillis` 10 s → 2 s.
  - `register()` no longer fetches remote plugin config server-side. Server code treats every plugin as enabled; client SDK's remote-config bootstrap is unaffected.

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
- 830c568: Add `sourceMaps.purge` to `withInterfere` settings so customers can keep source maps on disk when another tool uploads from the same `.next` build output.

### Patch Changes

- 9883610: Republish to align packed devDependency pin with the current workspace version.
- a23cf2a: Fix browser events always resolving the `development` environment. The browser kernel now reads env vars via static `process.env.X` member expressions so bundlers can inline them (previously the whole-object lookup hit the empty browser process shim), and `withInterfere()` stamps the build-time deploy environment (`INTERFERE_ENVIRONMENT` ?? `VERCEL_ENV`) into the bundle as `__INTERFERE_ENVIRONMENT__`, which the kernel prefers over the env chain.
- 6c47775: Resolve published workspace dependency ranges from package manifests.
- f2281fc: Fix debugId round-trip from build plugins through the SDK to the collector for source map resolution.
- 62cd678: Accept regional Interfere secret keys in Next.js release metadata builds.
- e304910: Autoinstrumentation improvements.
- 2bc13b1: Fix: `identity.clear()` now rotates the `deviceId` and stamps it per span/log so post-logout events attribute to a fresh anonymous person instead of the previously-identified user.
- dc31127: Configurable ingest endpoint via `apiHost` — post telemetry to a subdomain of your own site (resolved to Interfere via DNS) or a regional endpoint. Ingest requests never carry cookies.

  **Breaking (`@interfere/next`):** proxy mode is removed. The `@interfere/next/route-handler` export is gone — delete your `app/api/interfere/[[...path]]/route.ts`. Telemetry now posts directly to Interfere; set `interfere.apiHost` for an ad-blocker-resistant subdomain setup.

- dc31127: Auth header improvements.
- 0edf4b9: Next.js Edge runtime exception handling improvements.
- 09ced0a: Widen Next.js and React peer dependency compatibility ranges to better match supported customer projects.
- 8171e78: Improved server-side exception handling.
- 9a9e2a3: bug fixes
- 8b1c635: Declare OpenAPI security schemes on the public collector contract. The SDK now exposes first-class auth: `publishableKey` (`?pk=…`) as a constructor option for ingestion operations, and per-operation `{ apiKey }` security on release management methods (`releases.*`, `sourceMaps.*`, `telemetry.report`) — replacing the custom `httpClient` interceptor pattern. The next/vite plugins pass the typed security argument internally.
- 8171e78: Instrument errors thrown in Next.js Server Actions, and enrich server-side error handling with route error context and captured request headers.
- 7a4a9fa: Server-side exception handling improvements.
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
- Updated dependencies [4c03108]
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
  - @interfere/helpers@9.0.1

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

### Minor Changes

- 830c568: Add `sourceMaps.purge` to `withInterfere` settings so customers can keep source maps on disk when another tool uploads from the same `.next` build output.

### Patch Changes

- f2281fc: Fix debugId round-trip from build plugins through the SDK to the collector for source map resolution.
- dc31127: Configurable ingest endpoint via `apiHost` — post telemetry to a subdomain of your own site (resolved to Interfere via DNS) or a regional endpoint. Ingest requests never carry cookies.

  **Breaking (`@interfere/next`):** proxy mode is removed. The `@interfere/next/route-handler` export is gone — delete your `app/api/interfere/[[...path]]/route.ts`. Telemetry now posts directly to Interfere; set `interfere.apiHost` for an ad-blocker-resistant subdomain setup.

- dc31127: Auth header improvements.
- Updated dependencies [f2281fc]
- Updated dependencies [cfaf24f]
- Updated dependencies [dc31127]
- Updated dependencies [4c03108]
- Updated dependencies [d5213b2]
- Updated dependencies [d5213b2]
  - @interfere/react@11.0.0-canary.7
  - @interfere/types@10.0.0-canary.6
  - @interfere/sdk@11.0.0-canary.7
  - @interfere/helpers@9.0.1-canary.2

## 11.0.0-canary.6

### Patch Changes

- 6c47775: Resolve published workspace dependency ranges from package manifests.
- e304910: Autoinstrumentation improvements.
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

- 62cd678: Accept regional Interfere secret keys in Next.js release metadata builds.
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

- 0edf4b9: Next.js Edge runtime exception handling improvements.
- 8171e78: Improved server-side exception handling.
- 8171e78: Instrument errors thrown in Next.js Server Actions, and enrich server-side error handling with route error context and captured request headers.
- 7a4a9fa: Server-side exception handling improvements.
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

- a23cf2a: Fix browser events always resolving the `development` environment. The browser kernel now reads env vars via static `process.env.X` member expressions so bundlers can inline them (previously the whole-object lookup hit the empty browser process shim), and `withInterfere()` stamps the build-time deploy environment (`INTERFERE_ENVIRONMENT` ?? `VERCEL_ENV`) into the bundle as `__INTERFERE_ENVIRONMENT__`, which the kernel prefers over the env chain.
- 09ced0a: Widen Next.js and React peer dependency compatibility ranges to better match supported customer projects.
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

## 10.0.0

### Patch Changes

- [#1772](https://github.com/interfere-inc/interfere/pull/1772) [`7fc320b`](https://github.com/interfere-inc/interfere/commit/7fc320b0512f4dcd222bd71b732a08ee9fad97a0) Thanks [@skve](https://github.com/skve)! - Added the ability to customize the path of the Interfere route handler (/api/interfere)

- [#1758](https://github.com/interfere-inc/interfere/pull/1758) [`a1fee17`](https://github.com/interfere-inc/interfere/commit/a1fee1786d08000e1ddb37d68582b2868c18932b) Thanks [@pjsny](https://github.com/pjsny)! - Forwards device information through the Next.js route proxy to enhance device-level analytics

- Updated dependencies [[`7b3ccf6`](https://github.com/interfere-inc/interfere/commit/7b3ccf636f3121de4205c0e8b2ddcaea5e3bd6a9), [`7fc320b`](https://github.com/interfere-inc/interfere/commit/7fc320b0512f4dcd222bd71b732a08ee9fad97a0)]:
  - @interfere/sdk@9.0.3
  - @interfere/constants@9.0.1
  - @interfere/react@10.0.0

## 9.0.2

### Patch Changes

- [#1704](https://github.com/interfere-inc/interfere/pull/1704) [`3279b68`](https://github.com/interfere-inc/interfere/commit/3279b6880d32a0be71773c92e54cd6de79262614) Thanks [@skve](https://github.com/skve)! - Now confirms release completion explicitly and non-complete releases are excluded from finding

- Updated dependencies [[`3279b68`](https://github.com/interfere-inc/interfere/commit/3279b6880d32a0be71773c92e54cd6de79262614)]:
  - @interfere/react@9.0.2
  - @interfere/sdk@9.0.2

## 9.0.1

### Patch Changes

- [#1710](https://github.com/interfere-inc/interfere/pull/1710) [`bf784f0`](https://github.com/interfere-inc/interfere/commit/bf784f07e3b1f781814ae2330d15aa6e7b416adb) Thanks [@pjsny](https://github.com/pjsny)! - Relax peer dependency ranges to standard caret semver. `@interfere/next` now accepts `next ^16.0.0` and `react`/`react-dom ^19.0.0`, and `@interfere/react` now accepts `react`/`react-dom ^19.0.0`. Previously, narrow `>=` floors (e.g. `>=16.2.4`, `>=19.2.5`) caused npm `ERESOLVE` failures for users on slightly older 19.x or 16.x releases, even though the peers were marked optional.

- Updated dependencies [[`bf784f0`](https://github.com/interfere-inc/interfere/commit/bf784f07e3b1f781814ae2330d15aa6e7b416adb)]:
  - @interfere/react@9.0.1

## 9.0.0

### Major Changes

- normalization release

### Patch Changes

- Updated dependencies []:
  - @interfere/constants@10.0.0
  - @interfere/helpers@2.0.0
  - @interfere/react@10.0.0
  - @interfere/types@10.0.0

## 8.1.6

### Patch Changes

- [#1665](https://github.com/interfere-inc/interfere/pull/1665) [`01e037e`](https://github.com/interfere-inc/interfere/commit/01e037e5dc3f6a013124dab44863d87a9be93029) Thanks [@skve](https://github.com/skve)! - Fix: fail releases with a warning if source map uploading fails

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

- Updated dependencies [[`01e037e`](https://github.com/interfere-inc/interfere/commit/01e037e5dc3f6a013124dab44863d87a9be93029), [`026b2d5`](https://github.com/interfere-inc/interfere/commit/026b2d506a851aa481d63a4df4eb5512109cad0a), [`01e037e`](https://github.com/interfere-inc/interfere/commit/01e037e5dc3f6a013124dab44863d87a9be93029)]:
  - @interfere/types@8.1.6
  - @interfere/sdk@8.1.6
  - @interfere/react@8.1.6
  - @interfere/helpers@1.0.1

## 8.1.2

### Patch Changes

- [`669f5e4`](https://github.com/interfere-inc/interfere/commit/669f5e40c24cf47b4335b8e51df09efe40112d29) Thanks [@skve](https://github.com/skve)! - Added telemetry for per-sdk breakdowns

- Updated dependencies [[`669f5e4`](https://github.com/interfere-inc/interfere/commit/669f5e40c24cf47b4335b8e51df09efe40112d29)]:
  - @interfere/constants@8.1.2
  - @interfere/react@8.1.2
  - @interfere/sdk@8.1.2
  - @interfere/types@8.1.2

## 8.1.0

### Minor Changes

- [`e3b34e8`](https://github.com/interfere-inc/interfere/commit/e3b34e8bb42717aa5058286cb5683c0d8b068497) Thanks [@skve](https://github.com/skve)! - Added automatic country code detection & arbitrary trait support

### Patch Changes

- [`edf7f5f`](https://github.com/interfere-inc/interfere/commit/edf7f5f33e72f292e565537f43bcc15d99e78dcb) Thanks [@skve](https://github.com/skve)! - Updated README with a better example on how to use the `identity` functions from `useInterfere()`

- [`4ff803c`](https://github.com/interfere-inc/interfere/commit/4ff803c785b8a6144596fd9206ff4216b231228e) Thanks [@skve](https://github.com/skve)! - Remove the legacy peer dependency requirement on @vercel/blob

- Updated dependencies [[`e3b34e8`](https://github.com/interfere-inc/interfere/commit/e3b34e8bb42717aa5058286cb5683c0d8b068497)]:
  - @interfere/types@8.1.0
  - @interfere/constants@8.1.0
  - @interfere/react@8.1.0
  - @interfere/sdk@8.1.0

## 8.0.0

### Major Changes

- [`de35bcb`](https://github.com/interfere-inc/interfere/commit/de35bcbba7416339d95b4b98407e052046d6cce4) Thanks [@skve](https://github.com/skve)! - fixed race condition in session tracking

### Patch Changes

- Updated dependencies [[`de35bcb`](https://github.com/interfere-inc/interfere/commit/de35bcbba7416339d95b4b98407e052046d6cce4)]:
  - @interfere/react@8.0.0
  - @interfere/constants@8.0.0
  - @interfere/sdk@8.0.0
  - @interfere/types@8.0.0

## 7.0.0

### Major Changes

- [#1193](https://github.com/interfere-inc/interfere/pull/1193) [`61cdc18`](https://github.com/interfere-inc/interfere/commit/61cdc1846a92d4e8e23963e41c8253d22330e36c) Thanks [@skve](https://github.com/skve)! - Fixed an issue whereby the service worker was overzealous in proxying requests

### Patch Changes

- Updated dependencies [[`61cdc18`](https://github.com/interfere-inc/interfere/commit/61cdc1846a92d4e8e23963e41c8253d22330e36c)]:
  - @interfere/react@7.0.0

## 6.0.0

### Major Changes

- [`4de11e1`](https://github.com/interfere-inc/interfere/commit/4de11e17a8a774139f47f06992b554370bae7150) Thanks [@skve](https://github.com/skve)! - Added remote config to plugin enablement

### Patch Changes

- Updated dependencies [[`4de11e1`](https://github.com/interfere-inc/interfere/commit/4de11e17a8a774139f47f06992b554370bae7150)]:
  - @interfere/constants@6.0.0
  - @interfere/react@6.0.0
  - @interfere/sdk@6.0.0
  - @interfere/types@6.0.0

## 5.0.1

### Patch Changes

- [`457d650`](https://github.com/interfere-inc/interfere/commit/457d650fc79360fc7b067893afb95aaa4a0f058a) Thanks [@skve](https://github.com/skve)! - Stop sending data if `NODE_ENV` is anything other than `production` and `NEXT_PUBLIC_INTERFERE_FORCE_ENABLE` is unset

- Updated dependencies [[`457d650`](https://github.com/interfere-inc/interfere/commit/457d650fc79360fc7b067893afb95aaa4a0f058a), [`ae0d409`](https://github.com/interfere-inc/interfere/commit/ae0d4093813cd6672f9439501dafa1961da0dc25)]:
  - @interfere/react@5.0.1

## 5.0.0

### Major Changes

- [`2ef1592`](https://github.com/interfere-inc/interfere/commit/2ef1592e1b45422faf73bfd731509d6458fcfa07) Thanks [@skve](https://github.com/skve)! - User & Session Tracking support

### Patch Changes

- [#1156](https://github.com/interfere-inc/interfere/pull/1156) [`d3614cd`](https://github.com/interfere-inc/interfere/commit/d3614cd7200702efb22dfa64a33b5109533347d1) Thanks [@skve](https://github.com/skve)! - Improved logging output noise for non-fatal errors

- Updated dependencies [[`2ef1592`](https://github.com/interfere-inc/interfere/commit/2ef1592e1b45422faf73bfd731509d6458fcfa07), [`d3614cd`](https://github.com/interfere-inc/interfere/commit/d3614cd7200702efb22dfa64a33b5109533347d1)]:
  - @interfere/constants@5.0.0
  - @interfere/react@5.0.0
  - @interfere/sdk@5.0.0
  - @interfere/types@5.0.0

## 4.0.0

### Major Changes

- fix bundle

### Patch Changes

- Updated dependencies []:
  - @interfere/constants@4.0.0
  - @interfere/react@4.0.0
  - @interfere/sdk@4.0.0
  - @interfere/types@4.0.0

## 3.0.0

### Major Changes

- [#1150](https://github.com/interfere-inc/interfere/pull/1150) [`7e882fc`](https://github.com/interfere-inc/interfere/commit/7e882fc8693b7c011c18e75aec0c352f7808062e) Thanks [@skve](https://github.com/skve)! - Updated durability during periods of service outages, performance and debugging improvements

### Patch Changes

- Updated dependencies [[`e77dde8`](https://github.com/interfere-inc/interfere/commit/e77dde8cd99f9d6ee93cba53f2be1960b1941d6f), [`7e882fc`](https://github.com/interfere-inc/interfere/commit/7e882fc8693b7c011c18e75aec0c352f7808062e)]:
  - @interfere/react@3.0.0

## 1.0.5

### Patch Changes

- Fixed a duplex ReadableStream bug

## 1.0.4

### Patch Changes

- Fixed a regression with session tracking

## 1.0.3

### Patch Changes

- [`8102e08`](https://github.com/interfere-inc/interfere/commit/8102e0858d1e97953a62f76f0774511955c05422) Thanks [@skve](https://github.com/skve)! - Fixed catalog dependency resolution

- Updated dependencies [[`8102e08`](https://github.com/interfere-inc/interfere/commit/8102e0858d1e97953a62f76f0774511955c05422)]:
  - @interfere/constants@1.0.3
  - @interfere/react@1.0.3
  - @interfere/types@1.0.3
  - @interfere/sdk@1.0.3

## 1.0.2

### Patch Changes

- fix types

- Updated dependencies []:
  - @interfere/sdk@1.0.2
  - @interfere/constants@1.0.2
  - @interfere/react@1.0.2
  - @interfere/types@1.0.2

## 1.0.1

### Patch Changes

- [`032905b`](https://github.com/interfere-inc/interfere/commit/032905b19f2d916d4709251cf632957f127ca917) Thanks [@skve](https://github.com/skve)! - Fixed package exports

- Updated dependencies [[`032905b`](https://github.com/interfere-inc/interfere/commit/032905b19f2d916d4709251cf632957f127ca917)]:
  - @interfere/constants@1.0.1
  - @interfere/react@1.0.1
  - @interfere/sdk@1.0.1
  - @interfere/types@1.0.1

## 1.0.0

### Major Changes

- [`17fe701`](https://github.com/interfere-inc/interfere/commit/17fe701227f2a55644eb77b254888a5a0657036d) Thanks [@skve](https://github.com/skve)! - feat: release v1

### Minor Changes

- [`949b3e2`](https://github.com/interfere-inc/interfere/commit/949b3e27002091477e4fdbd37e6c77d6de4ff780) Thanks [@skve](https://github.com/skve)! - Fixed dependency graph

- [#1096](https://github.com/interfere-inc/interfere/pull/1096) [`5e7582c`](https://github.com/interfere-inc/interfere/commit/5e7582cc38114b7af8847deb19dad4980a41182b) Thanks [@skve](https://github.com/skve)! - Added consent management features

### Patch Changes

- [`99b741d`](https://github.com/interfere-inc/interfere/commit/99b741d9cbc336d5194a4c36acd7c8afdad6d7ba) Thanks [@skve](https://github.com/skve)! - bump

- [`9694ddf`](https://github.com/interfere-inc/interfere/commit/9694ddf9a6b9588f8902e4d5fe74fa9fd1db334c) Thanks [@skve](https://github.com/skve)! - fix me baby one more time

- [`bc3cdb3`](https://github.com/interfere-inc/interfere/commit/bc3cdb362c595217c1ffe70e462c1e994a3043b1) Thanks [@skve](https://github.com/skve)! - bump

- [`6e791b6`](https://github.com/interfere-inc/interfere/commit/6e791b6e05151da43259bdb026b126949e2f6470) Thanks [@skve](https://github.com/skve)! - bump

- [`6b29d95`](https://github.com/interfere-inc/interfere/commit/6b29d95aa8f7c5742299af4070938d8d12b914e3) Thanks [@skve](https://github.com/skve)! - simplify state management

- [`4506e8d`](https://github.com/interfere-inc/interfere/commit/4506e8d301a06600345dff51c7e141e783acee52) Thanks [@skve](https://github.com/skve)! - Updated readme

- [`2847cd7`](https://github.com/interfere-inc/interfere/commit/2847cd75cd9a067e6b265864f1ed01c01ffe8a22) Thanks [@skve](https://github.com/skve)! - Improvements, Native Next.js 16 support, and huge performance improvements.

- Updated dependencies [[`99b741d`](https://github.com/interfere-inc/interfere/commit/99b741d9cbc336d5194a4c36acd7c8afdad6d7ba), [`2070869`](https://github.com/interfere-inc/interfere/commit/207086911a6dc74b28798690208d01b24240be68), [`bc3cdb3`](https://github.com/interfere-inc/interfere/commit/bc3cdb362c595217c1ffe70e462c1e994a3043b1), [`6e791b6`](https://github.com/interfere-inc/interfere/commit/6e791b6e05151da43259bdb026b126949e2f6470), [`949b3e2`](https://github.com/interfere-inc/interfere/commit/949b3e27002091477e4fdbd37e6c77d6de4ff780), [`17fe701`](https://github.com/interfere-inc/interfere/commit/17fe701227f2a55644eb77b254888a5a0657036d), [`5e7582c`](https://github.com/interfere-inc/interfere/commit/5e7582cc38114b7af8847deb19dad4980a41182b), [`6e791b6`](https://github.com/interfere-inc/interfere/commit/6e791b6e05151da43259bdb026b126949e2f6470), [`4506e8d`](https://github.com/interfere-inc/interfere/commit/4506e8d301a06600345dff51c7e141e783acee52), [`2847cd7`](https://github.com/interfere-inc/interfere/commit/2847cd75cd9a067e6b265864f1ed01c01ffe8a22)]:
  - @interfere/constants@1.0.0
  - @interfere/react@1.0.0
  - @interfere/sdk@1.0.0
  - @interfere/types@1.0.0

## 1.0.0-alpha.10

### Major Changes

- [`17fe701`](https://github.com/interfere-inc/interfere/commit/17fe701227f2a55644eb77b254888a5a0657036d) Thanks [@skve](https://github.com/skve)! - feat: release v1

### Patch Changes

- Updated dependencies [[`17fe701`](https://github.com/interfere-inc/interfere/commit/17fe701227f2a55644eb77b254888a5a0657036d)]:
  - @interfere/constants@1.0.0-alpha.5
  - @interfere/react@1.0.0-alpha.7
  - @interfere/sdk@1.0.0-alpha.6
  - @interfere/types@1.0.0-alpha.6

## 0.2.0-alpha.9

### Patch Changes

- Updated dependencies [[`2070869`](https://github.com/interfere-inc/interfere/commit/207086911a6dc74b28798690208d01b24240be68)]:
  - @interfere/react@0.2.0-alpha.6

## 0.2.0-alpha.8

### Patch Changes

- bump

- Updated dependencies []:
  - @interfere/constants@0.2.0-alpha.4
  - @interfere/react@0.2.0-alpha.5
  - @interfere/sdk@0.2.0-alpha.5
  - @interfere/types@0.2.0-alpha.5

## 0.2.0-alpha.7

### Minor Changes

- Added consent management features

### Patch Changes

- Updated dependencies []:
  - @interfere/react@0.2.0-alpha.4
  - @interfere/types@0.2.0-alpha.4

## 0.2.0-alpha.6

### Patch Changes

- bump

- Updated dependencies []:
  - @interfere/sdk@0.2.0-alpha.4
  - @interfere/constants@0.2.0-alpha.3
  - @interfere/react@0.2.0-alpha.3
  - @interfere/types@0.2.0-alpha.3

## 0.2.0-alpha.5

### Patch Changes

- Updated dependencies []:
  - @interfere/sdk@0.2.0-alpha.3

## 0.2.0-alpha.4

### Patch Changes

- bump

- Updated dependencies []:
  - @interfere/constants@0.2.0-alpha.2
  - @interfere/react@0.2.0-alpha.2
  - @interfere/sdk@0.2.0-alpha.2
  - @interfere/types@0.2.0-alpha.2

## 0.2.0-alpha.3

### Patch Changes

- [`9694ddf`](https://github.com/interfere-inc/interfere/commit/9694ddf9a6b9588f8902e4d5fe74fa9fd1db334c) Thanks [@skve](https://github.com/skve)! - fix me baby one more time

## 0.2.0-alpha.2

### Patch Changes

- [`6b29d95`](https://github.com/interfere-inc/interfere/commit/6b29d95aa8f7c5742299af4070938d8d12b914e3) Thanks [@skve](https://github.com/skve)! - simplify state management

## 0.2.0-alpha.1

### Patch Changes

- Fix silent initialization issues

- Updated dependencies []:
  - @interfere/constants@0.2.0-alpha.1
  - @interfere/react@0.2.0-alpha.1
  - @interfere/types@0.2.0-alpha.1
  - @interfere/sdk@0.2.0-alpha.1

## 0.1.0-alpha.2

### Minor Changes

- [`949b3e2`](https://github.com/interfere-inc/interfere/commit/949b3e27002091477e4fdbd37e6c77d6de4ff780) Thanks [@skve](https://github.com/skve)! - Fixed dependency graph

### Patch Changes

- [`4506e8d`](https://github.com/interfere-inc/interfere/commit/4506e8d301a06600345dff51c7e141e783acee52) Thanks [@skve](https://github.com/skve)! - Updated readme

- Updated dependencies [[`949b3e2`](https://github.com/interfere-inc/interfere/commit/949b3e27002091477e4fdbd37e6c77d6de4ff780), [`4506e8d`](https://github.com/interfere-inc/interfere/commit/4506e8d301a06600345dff51c7e141e783acee52)]:
  - @interfere/constants@0.1.0-alpha.2
  - @interfere/react@0.1.0-alpha.26
  - @interfere/sdk@0.1.0-alpha.32
  - @interfere/types@0.1.0-alpha.3

## 0.1.0-alpha.1

### Patch Changes

- [`2847cd7`](https://github.com/interfere-inc/interfere/commit/2847cd75cd9a067e6b265864f1ed01c01ffe8a22) Thanks [@skve](https://github.com/skve)! - Improvements, Native Next.js 16 support, and huge performance improvements.

- Updated dependencies [[`2847cd7`](https://github.com/interfere-inc/interfere/commit/2847cd75cd9a067e6b265864f1ed01c01ffe8a22)]:
  - @interfere/constants@0.0.2-alpha.1
  - @interfere/react@0.1.0-alpha.1
  - @interfere/sdk@0.0.1-alpha.31
  - @interfere/types@0.1.0-alpha.2
