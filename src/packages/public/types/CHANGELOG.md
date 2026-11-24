# @interfere/types

## 11.2.2

### Patch Changes

- 7358d82: `@opentelemetry/api` is now a peer dependency (`^1.9.0`) instead of an exact dependency, so applications on any 1.x release of the API share one copy with the SDK instead of installing a second one.

## 11.2.1

### Patch Changes

- 5d39a3c: React component frames from production builds keep their source URL, line, and column, so Interfere maps each component frame to your source code.

- 64e0dab: `toException`, `resolveGithubReleaseSource`, `sourceMapBatchDigest` and `parseReactComponentStack` now return their real types instead of `any`, so with `exactOptionalPropertyTypes` enabled code that passes their results on may need to omit optional properties instead of assigning `undefined` to them (for example with `omitUndefined` from `@interfere/helpers`).
- 989e833: `vite dev` with an `INTERFERE_API_KEY` that has the `release:write` scope and no `VITE_INTERFERE_PUBLIC_KEY` now mints a release token for the service's shared local-development release, so browser telemetry is sent in local development without a public key, and the dev server warns when it cannot mint a token.
- Updated dependencies [64e0dab]
  - @interfere/helpers@9.3.6

### Network

- Data sent and consent:
  - 5d39a3c: each React component frame in a captured exception includes the URL, line, and column of the component code, in the frames for Chrome, Firefox, and Safari.

## 11.2.0

### Minor Changes

- 3ef4bdb: `createRelease` and `getReleasesConfig` responses carry `service` (`ReleaseServiceRef`: id, name and namespace) for the release's service, and `surface` is deprecated in its favour.

## 11.1.0

### Minor Changes

- b8c5bad: The browser SDK no longer bundles zod, which makes the eager browser bundle about 25 KiB smaller gzipped, and Interfere now leaves `traceparent` to a Sentry client configured with `propagateTraceparent`.

- 9aaddd6: Server and browser telemetry from built bundles now report the `production` environment, or the Wrangler environment of a `CLOUDFLARE_ENV` build, without `INTERFERE_ENVIRONMENT`; set it only for other preview or staging deploys.
- ead7ecb: Source-map uploads now send each map's SHA-256 first and skip uploading maps your Company already stores.

- 042188e: `CreateReleaseRequest` accepts an optional `service` (`{ name, namespace? }`), so a workspace secret key can register a release without a public key.
- 6e18c76: When release registration succeeds, `@interfere/vite` bakes its release token into the browser bundle for the browser SDK to use without exposing the build secret.

- ec31e5d: `createRelease` responses now include a signed release `token` when the release belongs to a service.

- 4eb17ec: `createRelease` now registers a release for every service, and its response's legacy `surface` field is `null` for a service that has none.
- e820a7f: Source-map manifests now accept `rolldown` as the bundler, for Vite 8 builds.
- f2e532d: The build diagnostic for a release that Interfere can't accept for a workspace secret key's service now asks for the service the build publishes for (`serviceName` or `OTEL_SERVICE_NAME`) instead of asking only for a public key, and also covers a server answer of `RELEASE_SERVICE_REQUIRED`.
- 5f28fcd: Server runtimes without `INTERFERE_API_KEY` now send telemetry with the release token the build baked into the server bundle, so the Next.js Node and Edge runtimes and the Vite Node server pick `INTERFERE_API_KEY`, then the baked release token, then the public key.

- 0be7ad5: A build with a workspace API key now publishes source maps for every service: `getReleasesConfig` answers with a `null` legacy `surface` field for a service that has none, and the build plugins read the config without it.
- c0c40ed: Browser network errors such as `Failed to fetch` now record whether the browser was offline and whether the page was hidden or being left, so Interfere can tell a visitor's network from a failure in your app.

### Patch Changes

- 4d63981: Every public package is now published with npm trusted publishing and a provenance attestation.
- d1b1138: Breaking: consent is now one option and one setter, `init({ consent: "required" })` and `consent.set()`, which replace the boolean and object `consent` options and keep the SDK from storing anything on the device until consent is granted.

- b77bfb3: Server spans now read session and device ids only from `interfere.*` baggage, so browsers still on SDK 12.x or earlier must upgrade to 13.x to keep server spans linked to their sessions.

- Updated dependencies [4d63981]
  - @interfere/helpers@9.3.5

### Network

- Headers:
  - b8c5bad: when Sentry is configured with `propagateTraceparent` for a request's URL, Interfere no longer writes its own `traceparent` on that request.
  - 6e18c76: browser `/drain/v1/{signal}` exports and `/v3/replay/upload` send `x-interfere-release-token` when the build baked a token; `/v3/config` and `/v3/session` also receive it, with legacy public-key headers retained for older deployments.
  - 5f28fcd: Node and Edge server exports to `/drain/v1/{signal}` send `x-interfere-release-token` when `INTERFERE_API_KEY` is unset and the build baked a release token, with no `authorization` or `x-api-key`, and send `x-interfere-public-key` alongside it when the public key is also set; runtimes with `INTERFERE_API_KEY` set are unchanged.
  - d1b1138: before consent, `baggage` carries `interfere.session.id` set to a page-scoped `page_<uuidv7>` id; previously no `baggage` was sent while analytics was denied.
- Data sent and consent:
  - ead7ecb: the sign request now carries a `sha256` for each source map, and the SDK sends no bytes for maps the response marks `present`.
  - ec31e5d: the release registration response now also returns `token`, a signed value holding the workspace, service and release ids, the release slug, the issue time and the key id; it is public by design.
  - 5f28fcd: server telemetry from a runtime with only the baked release token is attributed to the service and release named in the token, and server capture turns on for such a runtime even without a public key.
  - c0c40ed: exception events for transport failures carry up to four more attributes: `error.type`, `interfere.page.visibility`, `interfere.page.leaving` and, when offline, `network.connection.type`; no URLs or identifiers are added.
  - d1b1138: before consent, spans and logs carry the page-scoped id, no session or identify request is sent, and nothing is stored in cookies, `localStorage` or `sessionStorage`; when consent is granted, one `session.start` log record links the page id to the session.
  - b77bfb3: server spans no longer copy the legacy `session.id` and `device.id` baggage keys, so vendor baggage such as Datadog's `session.id` is never recorded.
- CORS:
  - 6e18c76: custom proxies must allow `x-interfere-release-token` on browser telemetry, configuration, session and replay requests.

## 11.0.2

### Patch Changes

- 13e0639: Capture errors thrown inside Next.js `after()` callbacks. Next.js only logs these to the server console and never reports them to `onRequestError`, so they were invisible. Wrap the task with the new `withInterfereAfter` export from `@interfere/next/server` (`after(withInterfereAfter(async () => { ... }))`): a throw or rejection is captured once with the `auto.function.nextjs.after` mechanism and rethrown unchanged, and the response is never delayed.
- 13e0639: Use neutral examples in SDK documentation and fixtures, and describe regression tests by their behavior. Remove internal issue references from public API documentation.

  Correct the SDK 13.0 migration notes: list breaking tracing changes together, consolidate repeated entries, and clarify that the canonical browser configuration is `tracing: { mode: "auto" }`, with `tracing: "auto"` still supported. Tracing ownership and header propagation are separate options; use `tracing.propagation: "external"` when another tool writes W3C headers.

- 13e0639: Include MIT licenses and package documentation, declare tree-shaking behavior, and document the supported Node.js and framework versions. Complete server adapter package metadata and link license badges to the included license.
- 13e0639: Remove workspace-only resolution conditions from stable and canary packages so consumers using custom conditions resolve shipped runtime and type files. Qualification tests now use the same sanitized package manifests while development source resolution stays unchanged.
- 13e0639: Errors thrown from another realm, such as an iframe, keep their original stack trace in Firefox. Previously the SDK replaced it with its own stack.
- Updated dependencies [13e0639]
- Updated dependencies [13e0639]
  - @interfere/helpers@9.3.4

## 11.0.1

### Patch Changes

- 30a48a6: A DOM `Event` thrown or passed as a rejection reason is now recorded as a non-error exception named after the event class, with the event type and a description of its target (tag, id and URL), instead of `Non-Error captured with keys: isTrusted`.
- 76a8c1e: Continue builds with a warning when a commit SHA is unavailable, skipping release publication. Detect commit SHAs from GitLab, Netlify, Render, CircleCI, Bitbucket, Azure Pipelines, Buildkite, Railway, AWS Amplify, and Heroku environments while preserving explicit source ID overrides.

  Remove generated Next.js source maps when release publication is skipped unless source-map purging is explicitly disabled.

- 76a8c1e: Removed deprecated SDK APIs:

  - Removed `PrerenderSafeIdGenerator` from `@interfere/next/instrumentation`; use `register()` to install managed tracing, or keep your provider's generator in application mode.
  - Removed the `KernelInternalOptions` re-export from `@interfere/next/instrument-client`; use `KernelOptions` for `init()` options.
  - Removed `tracing.apiHosts`; use `tracing.propagateTo` with exact origins such as `https://api.example.com`.
  - Removed the Vite provider's `errorBoundary` prop; use `captureRenderErrors`.
  - Removed `uploadSourceMapsMetadata` and `UploadSourceMapsMetadata` from `@interfere/types`; use the source-map upload session and completion schemas. Existing legacy server uploads remain supported.

- 76a8c1e: Browser `init()` now accepts `ignoreErrors`, `denyUrls`, `errors.sampleRate` and `beforeSend`, so you can drop noisy exceptions, exclude errors thrown by third-party scripts, sample error volume, and scrub or drop an exception before it leaves the page. The options apply to every capture source, including errors raised before remote configuration loads. If a `beforeSend` hook throws, the original exception is still sent.
- 76a8c1e: Redact sensitive URL values in captured console logs, limit each body to 16 KiB, and cap capture at 100 records per minute. Console log capture now follows analytics consent and stops when that consent is denied or revoked.
- 76a8c1e: Exception capture handles more edge cases:

  - Browser-extension noise is dropped only when the top frame of the stack is an extension (or Safari masked) URL. App errors that merely call through extension code are now reported.
  - `AggregateError.errors` (for example from `Promise.any`) appear in the exception chain after the aggregate, followed by its `cause`. Errors that reference themselves are listed once.
  - Exception messages, stacks, cause-chain entries and non-Error payloads are bounded in length and breadth, so a very large error (such as a multi-megabyte message) is still delivered, including with keepalive while the page unloads. The limits and helpers are exported from `@interfere/types/sdk/errors/truncate`.
  - Query tokens in the file URLs of fallback and component-stack frames are redacted with the same URL redaction policy as the rest of the event.

- Updated dependencies [76a8c1e]
  - @interfere/helpers@9.3.3

## 11.0.0

### Major Changes

- 3ff22ac: Make browser tracing ownership explicit. Replace `tracing: true` with the canonical options object, such as `tracing: { mode: "auto" }`. The string shorthand `tracing: "auto"` is also accepted, as are `"managed"`, `"application"` and `false`. The default is `auto`; application code controls the mode. `auto` respects an existing OpenTelemetry propagator, `managed` records request spans, and `application` records request observations without claiming OpenTelemetry globals. Header propagation is independent: set `tracing.propagation: "external"` when another tool owns W3C trace headers. Existing caller headers are preserved, and baggage has its own opt-out. With `tracing: false`, exceptions still export through the capture-only transport.

- c76563f: `identify()` now posts as soon as it is called, with the local session and device ids and whatever fingerprint is already known; it no longer waits for fingerprinting or a country lookup. A fingerprint that resolves later reaches the device through the next session sync, and is discarded if device consent was revoked or the identity was reset in the meantime. The browser no longer requests `cloudflare.com/cdn-cgi/trace` or parses the user agent for classification: country comes from the ingest edge and browser/OS classification from the server. With `tracing: false`, captured exceptions are exported again: each becomes one OTLP exception span posted directly to the trace drain, without constructing a tracer provider, exporter chain or instrumentation. `@interfere/types` removes `sdk/geo` (`cloudflareTraceSchema`, `CloudflareTrace`), whose only consumer was the removed geo lookup. It also removes `sdk/plugins/context/browser` (`browserContextSchema`, `deviceMetadataSchema` and their types), which lost its only consumer with the browser-side user-agent parsing. Exceptions exported with `tracing: false` go through the same URL redaction as the tracing path, and at most one export is in flight with up to 32 exceptions queued behind it.

### Minor Changes

- 68a9ee0: Remove the SDK 9 event envelope types, which no SDK 10 code path sends. `@interfere/types` no longer exports `sdk/envelope`, `sdk/plugins/payload/page-events`, `sdk/plugins/payload/rage-click`, `sdk/plugins/payload/replay`, `sdk/plugins/context/edge`, `sdk/plugins/context/node`, `errorEnvelopePayloadSchema` or `EVENT_TYPE_TO_PLUGIN`. `@interfere/react` drops its unused per-event consent lookup.
- 3dfaefa: Accept `company: { tier }` in `identity.set()` and send company tier changes with session identification.
- 8f92d43: Remove the surface-level tracing owner. Browser and server tracing modes are configured in application code and default to `auto`; `managed` and `application` remain explicit overrides. Remote configuration cannot change tracing ownership.
- 3226d63: Keep unsampled server-side trace context. The browser SDK now extracts an unsampled `traceparent` (flags `00`) and its `tracestate` from the SSR meta tags as served instead of inventing a new sampled root; the initial page-view event carries it, sampled or not. Also, the Vite `traceMeta()` helper emits unsampled contexts too. The browser never decides sampling from the meta: its private tracer provider declares a `ParentBased(AlwaysOn)` sampler, and the Next server registers the same sampler unless `OTEL_TRACES_SAMPLER` is set. SSR trace meta without a fresh `interfere` tracestate stamp is now rejected, so a `traceparent` served from cached HTML can no longer link visitors together. The Next adapter owns cache-safe trace context, freshness stamps and ID generation internally. `withInterfere()` enables `clientTraceMetadata` whenever the browser SDK is configured; the browser accepts only fresh, stamped server context.

### Patch Changes

- f584c2f: Preserve every shared JavaScript chunk alias and align its debug ID with the uploaded source map. Retain initiating callsites on XHR and transport failures, server error causes, and structured thrown values. Report partial client and server source-map coverage.

  Publish source maps on Next.js 15 after its final Webpack client compilation, preserving customer hooks and waiting for publication and cleanup before prerendering.

- 2938037: Use an exact-origin set for browser request propagation and share the server correlation baggage processor across Next and Vite, preserving Next's correlationSpanProcessor export.
- ec97a9c: Ship CHANGELOG.md inside the published tarball of every package so release notes travel with the installed version.
- 7c8441a: Add `sdk/exception-delivery` with `EXCEPTION_DELIVERY_ATTRIBUTE` and `EXCEPTION_DELIVERY`, the exception event attribute that marks the host-span copy of an error the server SDK also delivers through its own exporter.
- Updated dependencies [ec97a9c]
  - @interfere/helpers@9.3.2

## 11.0.0-rc.2

### Patch Changes

- 7c8441a: Add `sdk/exception-delivery` with `EXCEPTION_DELIVERY_ATTRIBUTE` and `EXCEPTION_DELIVERY`, the exception event attribute that marks the host-span copy of an error the server SDK also delivers through its own exporter.

## 11.0.0-rc.1

### Minor Changes

- 3dfaefa: Accept `company: { tier }` in `identity.set()` and send company tier changes with session identification.
- 8f92d43: Remove the surface-level tracing owner. The SDKs no longer read a build-baked or remote `tracingOwnership`, and the warning about an option that differs from the surface setting is gone. The browser `tracing.mode` and the Next server `tracing` option keep `auto` as the default; the Vite server option keeps `managed`; `managed` and `application` in code are the only overrides.

### Patch Changes

- 2938037: Use an exact-origin set for browser request propagation and share the server correlation baggage processor across Next and Vite, preserving Next's correlationSpanProcessor export.
- ec97a9c: Ship CHANGELOG.md inside the published tarball of every package so release notes travel with the installed version.
- Updated dependencies [ec97a9c]
  - @interfere/helpers@9.3.2-rc.0

## 11.0.0-rc.0

### Major Changes

- c76563f: `identify()` now posts as soon as it is called, with the local session and device ids and whatever fingerprint is already known; it no longer waits for fingerprinting or a country lookup. A fingerprint that resolves later reaches the device through the next session sync, and is discarded if device consent was revoked or the identity was reset in the meantime. The browser no longer requests `cloudflare.com/cdn-cgi/trace` or parses the user agent for classification: country comes from the ingest edge and browser/OS classification from the server. With `tracing: false`, captured exceptions are exported again: each becomes one OTLP exception span posted directly to the trace drain, without constructing a tracer provider, exporter chain or instrumentation. `@interfere/types` removes `sdk/geo` (`cloudflareTraceSchema`, `CloudflareTrace`), whose only consumer was the removed geo lookup. It also removes `sdk/plugins/context/browser` (`browserContextSchema`, `deviceMetadataSchema` and their types), which lost its only consumer with the browser-side user-agent parsing. Exceptions exported with `tracing: false` go through the same URL redaction as the tracing path, and at most one export is in flight with up to 32 exceptions queued behind it.

### Minor Changes

- 68a9ee0: Remove the SDK 9 event envelope types, which no SDK 10 code path sends. `@interfere/types` no longer exports `sdk/envelope`, `sdk/plugins/payload/page-events`, `sdk/plugins/payload/rage-click`, `sdk/plugins/payload/replay`, `sdk/plugins/context/edge`, `sdk/plugins/context/node`, `errorEnvelopePayloadSchema` or `EVENT_TYPE_TO_PLUGIN`. `@interfere/react` drops its unused per-event consent lookup.
- 3ff22ac: Make browser tracing ownership explicit. `init({ tracing })` now takes `false` or an options object with `mode: "managed" | "application"`; the boolean `true` form is gone. Managed mode (the default) registers Interfere's context manager and W3C + baggage propagator only when the OpenTelemetry registration call succeeds, records what it registered and releases exactly that on `close()`; when another propagator already owns trace headers it warns once, injects nothing and observes requests as events. Application mode never writes an OpenTelemetry API global, injects no header and creates no request span: requests are recorded as `interfere.request` log events that carry the application's active span context, and exceptions still record on the host's active span. The default mode comes from the workspace surface setting (`tracingOwnership`, baked by the Next and Vite builds or read from the remote configuration); an explicit `mode` that disagrees with it logs one warning. Every patch Interfere installs (`fetch`, `XMLHttpRequest`, `history`, `console`, `window.onerror`) is restored only while it is still the installed function and goes inert beneath a wrapper another SDK installed later. With `tracing: false` the kernel no longer loads the OTLP exporter chain at all; exceptions on that path are still not exported, as in 12.x, until the capture-only transport lands. Propagated baggage uses the namespaced keys `interfere.session.id` and `interfere.device.id`.
- 3226d63: Keep unsampled server-side trace context. The browser SDK now extracts an unsampled `traceparent` (flags `00`) and its `tracestate` from the SSR meta tags as served instead of inventing a new sampled root; the initial page-view event carries it, sampled or not. Also, the Vite `traceMeta()` helper emits unsampled contexts too. The browser never decides sampling from the meta: its private tracer provider declares a `ParentBased(AlwaysOn)` sampler, and the Next server registers the same sampler unless `OTEL_TRACES_SAMPLER` is set. SSR trace meta without a fresh `interfere` tracestate stamp is now rejected, so a `traceparent` served from cached HTML can no longer link visitors together. The Next adapter owns the cache-safe trace-context, freshness-stamp and ID-generator pieces internally: `register()` fixes their order, the kit's `propagators` is a complete ordered list to spread as-is, and `traceContextPropagator`, `idGenerator` and `PrerenderSafeIdGenerator` are deprecated. `withInterfere()` enables `clientTraceMetadata` only when the surface's tracing owner is Interfere (`tracingOwnership` in the remote configuration, default `managed`).

## 10.6.3

### Patch Changes

- e1f7ee8: Send the upload session's grant on source-map chunk and batch uploads, so each chunk skips per-request API-key verification after the sign call has authorized the session.

## 10.6.2

### Patch Changes

- d5a2d36: Read each app's anonymous session replay policy from its remote configuration and bake it into Next.js and Vite builds, so the browser SDK can record visitors who are not signed in only after an error.
- 2f2934a: Enable development telemetry when a public key is configured and remove undocumented environment overrides in favor of existing SDK options and automatic defaults.
- c2428dd: Keep automatic tracing within the correct site boundaries on shared hosting platforms. Preserve discovered backend hosts in production builds so browser requests can connect to backend traces without manual host configuration.
- Updated dependencies [461082f]
  - @interfere/helpers@9.3.1

## 10.6.1

### Patch Changes

- 8381980: Browser requests now carry trace context to the page's own site (same registrable domain, any subdomain or port) and to backend hosts Interfere reports in remote config, so browser-to-backend correlation works without `tracing.apiHosts`. `apiHosts` remains an additive override; third-party hosts never receive the headers.

## 10.6.0

### Minor Changes

- c949553: `ReleaseStatus` adds `unknown`: a release that stayed pending or building for 24 hours without its provider reporting a final state is marked `unknown` instead of staying in progress forever.

### Patch Changes

- 9dd1bf0: A build that baked SDK plugin settings from an older Interfere API, missing a newer plugin's key, now keeps those settings instead of discarding them: the browser no longer refetches `/v3/config` on every page load, and a plugin you disabled stays disabled. A missing key reads as enabled.

## 10.5.1

### Patch Changes

- 4d4457c: The 2 s cap on direct OTLP trace and log exports now follows the surface's hosting platform from the Interfere configuration, not the `VERCEL` environment variable. The configuration reports `platform: "vercel"` for a Next.js surface linked to a Vercel project, and `"cloudflare"` for a surface linked to a Cloudflare Worker. The build injects it the same way it injects the drain modes.

  - The cap applies when the injected platform is `vercel`. A surface with no linked platform exports with the standard timeouts, even when `VERCEL` is set.
  - When the build could not load the configuration, the SDK falls back to `VERCEL` for this decision. That fallback is removed in the next major.
  - `VERCEL_ENV` and `VERCEL_TARGET_ENV` still serve only as fallbacks for the environment label.
  - `@interfere/types`: adds `@interfere/types/integrations/platforms/hosting` (`hostingPlatformSchema`, `HostingPlatform`, `PLATFORM_GLOBAL`). `resolveServerTracing` takes the injected `platform`.
  - `@interfere/javascript`: `SdkConfigResponse` and the release config types gain the optional `platform` field.

## 10.5.0

### Minor Changes

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

## 10.4.0

### Minor Changes

- 47943fb: Send the public key on release uploads so workspace secret keys can publish releases.

### Patch Changes

- 4fff52d: Stamp a default `service.name` into Vite builds. `@interfere/vite/plugin` now derives one from the `serviceName` plugin option, then `VITE_INTERFERE_SERVICE_NAME`, then the project package name without its scope (`@acme/storefront` → `storefront`), and `@interfere/vite/init` bridges it onto the page. `@interfere/react` reads that stamped name when `init()` is called without `serviceName`, so a frontend that never set one no longer reports OpenTelemetry's `unknown_service`. An explicit `init({ serviceName })` still wins. `@interfere/types/sdk/service-name` adds the `SERVICE_NAME_GLOBAL` wire name.
- Updated dependencies [67b57b8]
- Updated dependencies [cb80e0a]
  - @interfere/helpers@9.3.0

## 10.3.0

### Minor Changes

- a231cb5: Builds no longer fail when Interfere is unavailable or `INTERFERE_API_KEY` is missing. Adds a `disable` option. Vite removes the boolean and `"required"` forms of `sourceMaps`, and `sourceMapReadinessTimeoutMs`; use `sourceMaps: { upload: false }` or `disable: true`.

### Patch Changes

- 4d17ef3: Add a `privacy` option that excludes private content from replay and interaction telemetry and redacts recorded URLs. URL credentials and sensitive query values are redacted by default; `privacy.urls` adjusts the query and fragment rules.

## 10.2.2

### Patch Changes

- 99f3752: Negotiate gzip source-map uploads with the server and retain uncompressed 512 KiB chunks for older deployments.
- 8ffe6fb: Accept source-map upload batches up to 8 MiB.

## 10.2.1

### Patch Changes

- 99a1f17: Support boolean consent shorthand.

## 10.2.0

### Minor Changes

- 5b005aa: Add bounded source-map batch upload request and response schemas.
- 40501f3: Batch small source-map files when the upload session advertises compatible support, using one bounded upload pool and preserving the existing path for older servers and large files.
- 65b3ebb: Use a source-map upload-session response containing the expiry and file paths for v3 uploads. Upload chunks through the generated client with a path and byte offset. Legacy signing schemas and v1 tokenized upload URLs remain available.

  Remove unused session request headers, build dependencies, and Vite release-result fields.

- 67ec54b: Require source maps for release readiness.

  Align Next.js and Vite build configuration for plugins, ingest URL, server tracing, and log sourcing. Both build integrations register releases and verify upload acceptance, and only remove local maps after the server accepts the complete upload.

- 87fad80: Share deployment metadata and source-map file utilities across Next and Vite. Both integrations call the generated API client endpoints directly. Source maps are removed only after durable acceptance of the complete upload.
- 88ce674: Add optional source excerpts and their starting line numbers to resolved error frames.
- 27d080c: Add `tracestate` stamping

### Patch Changes

- 0efde29: Fix source imports used by Node when loading development build configurations.
- 63df2c9: Refresh dependency ranges. `@interfere/nest` now accepts NestJS 11 or 12 as a peer.
- a653b8d: Add the generated JavaScript API client with ESM, CommonJS, and TypeScript declarations. Next and Vite call its endpoints directly for build configuration, release creation, source-map uploads, and telemetry. Remove the shared build client and its combined release-preparation method. Preserve ordered uploads, expiration checks, and cleanup only after complete server acceptance.

  Build requests use Fern's retry policy for HTTP 408, 429, and 5xx responses. Network failures and HTTP 425 are no longer retried.

  Retire the unused CLI and Speakeasy SDK packages, their generation configuration, and compatibility wrappers. Remove legacy host remapping and Collector error aliases. Update framework setup instructions to use the generated release endpoints.

- 5ff7610: Update stack-parser documentation to point to its current ingestion package after Collector retirement.
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
- bbfda83: Read injected SDK settings without broad global type assertions, preserving schema validation and existing defaults.
- d25952d: Fix source map uploads for some server-rendered builds, and clarify build errors caused by network restrictions.

## 10.1.2

### Patch Changes

- f1dedba: improved types

## 10.1.1

### Patch Changes

- c7f7d47: Add blank lines around multiline statements and before returns, from enforcing the internal padding-between-statements lint rule repo-wide. Whitespace only, no behavior or API difference.

## 10.1.0

### Minor Changes

- 1a40a00: Widen `ConfiguredSourceProvider` to include `ghe_cloud` and `ghe_server`.
- 1949d88: `withInterfere` now bakes a build sentinel; when an outer config wrapper drops the integration, `init()` detects the missing sentinel and warns instead of shipping an inert SDK silently. Detection tolerates ESM import hoisting (the documented `import "@interfere/next/instrumentation-client"` setup re-checks after the customer module body runs), and setups that authenticate through `INTERFERE_PUBLIC_KEY` or `NEXT_PUBLIC_INTERFERE_FORCE_ENABLE` from the environment keep sending events in a degraded mode (no release association or sourcemaps) rather than being disabled.
- 18f2746: Improved release slug / SHA deduction.

### Patch Changes

- e347b65: Internal lint modernization: apply oxlint autofixes across previously quarantined rules (import specifier style, catch variable naming, DOM idiom updates, and related mechanical cleanups). No behavior changes intended.
- 18f2746: Bug fixes & performance improvements
- ea785f4: Type `inferRuntime`'s global detection structurally via `globalThis` so importing `@interfere/types/sdk/runtime` no longer requires `lib.dom` or `@types/node` in the consumer's TypeScript program. `readLogSourcingMode` and `readServerTracingMode` now read `process.env` the same way (via the new `readProcessEnv`), dropping their bare `process` references too. Runtime behavior is unchanged.
- 9a8843a: Bump the shared Vitest catalog from 4.1.9 to 4.1.10

## 10.0.2

### Patch Changes

- 2ecaada: Fix remote-config and deploy-environment globals not baking into Next.js 16 Turbopack production builds. `withInterfere()` now returns a `(phase) => NextConfig` function and reads the build phase from that argument instead of the unreliable `process.env.NEXT_PHASE`. An async config function (one that resolves to a Promise) is now awaited and composed instead of throwing.

  Composition: `withInterfere()` should always be the outermost Next config wrapper.

- c6ecb1e: Public keys now mint with the `interfere_public_{us,eu}_` prefix. Legacy `interfere_pub_{us,eu}_` and `interfere_pk_` keys remain accepted. The publishable-key OpenAPI description and the API-key diagnostic hint now reference the new prefix.
- 52e7a06: Standardize credential terminology to "public key" and "secret key" in build diagnostics, CLI help, plugin warnings, and READMEs. Env var names (`INTERFERE_API_KEY`, `INTERFERE_PUBLIC_KEY`), CLI flags (`--api-key`), and diagnostic codes are unchanged.
- b562bb3: Vercel integration metadata `projects` values are now `string | null`. A project whose display name can't be resolved is stored as `null` (rendered as an untitled project) instead of falling back to the raw project ID.

## 10.0.1

### Patch Changes

- 2f8aad9: Add `logSourcing: "sdk" | "drain"` alongside `serverTracing`, resolved from `/v2/config` at build time and readable via `readLogSourcingMode()`.

  In `drain` mode the SDK skips its OTLP log exporter and console bridge (Next + Vite); the trace exporter is likewise skipped in `serverTracing: "drain"`. With both set, the SDK ships no telemetry and Vercel's platform drains carry it end to end.

- 42b109e: Republish SDK packages whose content drifted from their published versions after upstream schema changes (org entitlements). Version bump resyncs the registry with main so the publish gate passes.
- 32a1acf: bug fixes & performance improvements

## 10.0.0

### Major Changes

- dcdec1f: Require Clerk identify payloads to use raw Clerk user ids starting with `user_`.

### Patch Changes

- 9883610: Republish to align packed devDependency pin with the current workspace version.
- 7ba7566: Add a WorkOS identify source and support a config-driven ingest URL for BYO-domain ingest.
- 7c6bf32: Ship CommonJS entrypoints for the NestJS server SDK dependency chain.
- b19d109: Enhance exception capture in browser.
- f2281fc: Fix debugId round-trip from build plugins through the SDK to the collector for source map resolution.
- 4893278: Improve reliability for automatic browser error reporting.
- 6514e2a: `captureFailedRequests` is now on by default, capturing failed fetch/XHR requests (4xx/5xx) along with network transport-level failures. Pass `captureFailedRequests: false` to disable, or `{ statusCodes }` to tune the matched ranges.
- 62cd678: Accept regional Interfere secret keys in Next.js release metadata builds.
- e304910: Autoinstrumentation improvements.
- dc31127: Configurable ingest endpoint via `apiHost` — post telemetry to a subdomain of your own site (resolved to Interfere via DNS) or a regional endpoint. Ingest requests never carry cookies.

  **Breaking (`@interfere/next`):** proxy mode is removed. The `@interfere/next/route-handler` export is gone — delete your `app/api/interfere/[[...path]]/route.ts`. Telemetry now posts directly to Interfere; set `interfere.apiHost` for an ad-blocker-resistant subdomain setup.

- d265875: Refresh public package artifacts after build output drift.
- 0edf4b9: Next.js Edge runtime exception handling improvements.
- ed30c31: Improved resource load failures capture.
- 8171e78: Instrument errors thrown in Next.js Server Actions, and enrich server-side error handling with route error context and captured request headers.

## 10.0.0-canary.7

### Patch Changes

- 9883610: Republish to align packed devDependency pin with the current workspace version.

## 10.0.0-canary.6

### Patch Changes

- f2281fc: Fix debugId round-trip from build plugins through the SDK to the collector for source map resolution.
- dc31127: Configurable ingest endpoint via `apiHost` — post telemetry to a subdomain of your own site (resolved to Interfere via DNS) or a regional endpoint. Ingest requests never carry cookies.

  **Breaking (`@interfere/next`):** proxy mode is removed. The `@interfere/next/route-handler` export is gone — delete your `app/api/interfere/[[...path]]/route.ts`. Telemetry now posts directly to Interfere; set `interfere.apiHost` for an ad-blocker-resistant subdomain setup.

## 10.0.0-canary.5

### Major Changes

- dcdec1f: Require Clerk identify payloads to use raw Clerk user ids starting with `user_`.

### Patch Changes

- e304910: Autoinstrumentation improvements.

## 9.0.3-canary.4

### Patch Changes

- 62cd678: Accept regional Interfere secret keys in Next.js release metadata builds.

## 9.0.3-canary.3

### Patch Changes

- d265875: Refresh public package artifacts after build output drift.

## 9.0.3-canary.2

### Patch Changes

- b19d109: Enhance exception capture in browser.
- 6514e2a: `captureFailedRequests` is now on by default, capturing failed fetch/XHR requests (4xx/5xx) along with network transport-level failures. Pass `captureFailedRequests: false` to disable, or `{ statusCodes }` to tune the matched ranges.
- 0edf4b9: Next.js Edge runtime exception handling improvements.
- ed30c31: Improved resource load failures capture.
- 8171e78: Instrument errors thrown in Next.js Server Actions, and enrich server-side error handling with route error context and captured request headers.

## 9.0.3-canary.1

### Patch Changes

- 4893278: Improve reliability for automatic browser error reporting.

## 9.0.3-canary.0

### Patch Changes

- [#2107](https://github.com/interfere-inc/interfere/pull/2107) [`e62516e`](https://github.com/interfere-inc/interfere/commit/e62516eac5997c3de92160bae1052b68e2585520) Thanks [@pjsny](https://github.com/pjsny)! - Ship CommonJS entrypoints for the NestJS server SDK dependency chain.

## 9.0.2

### Patch Changes

- [#1974](https://github.com/interfere-inc/interfere/pull/1974) [`7ae98ce`](https://github.com/interfere-inc/interfere/commit/7ae98ce8a50964b0745844f00af5737d2ce4d77e) Thanks [@pjsny](https://github.com/pjsny)! - Ship `releases/slug` subpath.

## 9.0.0

### Major Changes

- normalization release

### Patch Changes

- Updated dependencies []:
  - @interfere/constants@10.0.0

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

## 8.1.2

### Patch Changes

- [`669f5e4`](https://github.com/interfere-inc/interfere/commit/669f5e40c24cf47b4335b8e51df09efe40112d29) Thanks [@skve](https://github.com/skve)! - Added telemetry for per-sdk breakdowns

- Updated dependencies [[`669f5e4`](https://github.com/interfere-inc/interfere/commit/669f5e40c24cf47b4335b8e51df09efe40112d29)]:
  - @interfere/constants@8.1.2

## 8.1.1

### Patch Changes

- [#1541](https://github.com/interfere-inc/interfere/pull/1541) [`2cb008f`](https://github.com/interfere-inc/interfere/commit/2cb008f4af34f6af83dad810af0fea99e93fbd01) Thanks [@pjsny](https://github.com/pjsny)! - Skip ingesting client errors when the root stack includes a browser extension script URL (`chrome-extension:`, `moz-extension:`, `safari-web-extension:`, `safari-extension:`, `ms-browser-extension:`), cutting noise from wallets, blockers, and similar injectors.

  Adds `shouldDropBrowserExtensionNoise()` in `@interfere/types` and applies it in the React errors plugin and `ErrorBoundary` before capture.

## 8.1.0

### Minor Changes

- [`e3b34e8`](https://github.com/interfere-inc/interfere/commit/e3b34e8bb42717aa5058286cb5683c0d8b068497) Thanks [@skve](https://github.com/skve)! - Added automatic country code detection & arbitrary trait support

### Patch Changes

- Updated dependencies [[`e3b34e8`](https://github.com/interfere-inc/interfere/commit/e3b34e8bb42717aa5058286cb5683c0d8b068497)]:
  - @interfere/constants@8.1.0

## 8.0.0

### Major Changes

- [`de35bcb`](https://github.com/interfere-inc/interfere/commit/de35bcbba7416339d95b4b98407e052046d6cce4) Thanks [@skve](https://github.com/skve)! - fixed race condition in session tracking

### Patch Changes

- Updated dependencies [[`de35bcb`](https://github.com/interfere-inc/interfere/commit/de35bcbba7416339d95b4b98407e052046d6cce4)]:
  - @interfere/constants@8.0.0

## 6.0.0

### Major Changes

- [`4de11e1`](https://github.com/interfere-inc/interfere/commit/4de11e17a8a774139f47f06992b554370bae7150) Thanks [@skve](https://github.com/skve)! - Added remote config to plugin enablement

### Patch Changes

- Updated dependencies [[`4de11e1`](https://github.com/interfere-inc/interfere/commit/4de11e17a8a774139f47f06992b554370bae7150)]:
  - @interfere/constants@6.0.0

## 5.0.0

### Major Changes

- [`2ef1592`](https://github.com/interfere-inc/interfere/commit/2ef1592e1b45422faf73bfd731509d6458fcfa07) Thanks [@skve](https://github.com/skve)! - User & Session Tracking support

### Patch Changes

- [#1156](https://github.com/interfere-inc/interfere/pull/1156) [`d3614cd`](https://github.com/interfere-inc/interfere/commit/d3614cd7200702efb22dfa64a33b5109533347d1) Thanks [@skve](https://github.com/skve)! - Improved logging output noise for non-fatal errors

- Updated dependencies [[`2ef1592`](https://github.com/interfere-inc/interfere/commit/2ef1592e1b45422faf73bfd731509d6458fcfa07)]:
  - @interfere/constants@5.0.0

## 4.0.0

### Major Changes

- fix bundle

### Patch Changes

- Updated dependencies []:
  - @interfere/constants@4.0.0

## 1.0.3

### Patch Changes

- [`8102e08`](https://github.com/interfere-inc/interfere/commit/8102e0858d1e97953a62f76f0774511955c05422) Thanks [@skve](https://github.com/skve)! - Fixed catalog dependency resolution

- Updated dependencies [[`8102e08`](https://github.com/interfere-inc/interfere/commit/8102e0858d1e97953a62f76f0774511955c05422)]:
  - @interfere/constants@1.0.3

## 1.0.2

### Patch Changes

- fix types

- Updated dependencies []:
  - @interfere/constants@1.0.2

## 1.0.1

### Patch Changes

- [`032905b`](https://github.com/interfere-inc/interfere/commit/032905b19f2d916d4709251cf632957f127ca917) Thanks [@skve](https://github.com/skve)! - Fixed package exports

- Updated dependencies [[`032905b`](https://github.com/interfere-inc/interfere/commit/032905b19f2d916d4709251cf632957f127ca917)]:
  - @interfere/constants@1.0.1

## 1.0.0

### Major Changes

- [`17fe701`](https://github.com/interfere-inc/interfere/commit/17fe701227f2a55644eb77b254888a5a0657036d) Thanks [@skve](https://github.com/skve)! - feat: release v1

### Minor Changes

- [`949b3e2`](https://github.com/interfere-inc/interfere/commit/949b3e27002091477e4fdbd37e6c77d6de4ff780) Thanks [@skve](https://github.com/skve)! - Fixed dependency graph

- [#1096](https://github.com/interfere-inc/interfere/pull/1096) [`5e7582c`](https://github.com/interfere-inc/interfere/commit/5e7582cc38114b7af8847deb19dad4980a41182b) Thanks [@skve](https://github.com/skve)! - Added consent management features

### Patch Changes

- [`99b741d`](https://github.com/interfere-inc/interfere/commit/99b741d9cbc336d5194a4c36acd7c8afdad6d7ba) Thanks [@skve](https://github.com/skve)! - bump

- [`bc3cdb3`](https://github.com/interfere-inc/interfere/commit/bc3cdb362c595217c1ffe70e462c1e994a3043b1) Thanks [@skve](https://github.com/skve)! - bump

- [`6e791b6`](https://github.com/interfere-inc/interfere/commit/6e791b6e05151da43259bdb026b126949e2f6470) Thanks [@skve](https://github.com/skve)! - bump

- [`4506e8d`](https://github.com/interfere-inc/interfere/commit/4506e8d301a06600345dff51c7e141e783acee52) Thanks [@skve](https://github.com/skve)! - Updated readme

- [`2847cd7`](https://github.com/interfere-inc/interfere/commit/2847cd75cd9a067e6b265864f1ed01c01ffe8a22) Thanks [@skve](https://github.com/skve)! - Improvements, Native Next.js 16 support, and huge performance improvements.

- Updated dependencies [[`99b741d`](https://github.com/interfere-inc/interfere/commit/99b741d9cbc336d5194a4c36acd7c8afdad6d7ba), [`bc3cdb3`](https://github.com/interfere-inc/interfere/commit/bc3cdb362c595217c1ffe70e462c1e994a3043b1), [`6e791b6`](https://github.com/interfere-inc/interfere/commit/6e791b6e05151da43259bdb026b126949e2f6470), [`949b3e2`](https://github.com/interfere-inc/interfere/commit/949b3e27002091477e4fdbd37e6c77d6de4ff780), [`17fe701`](https://github.com/interfere-inc/interfere/commit/17fe701227f2a55644eb77b254888a5a0657036d), [`4506e8d`](https://github.com/interfere-inc/interfere/commit/4506e8d301a06600345dff51c7e141e783acee52), [`2847cd7`](https://github.com/interfere-inc/interfere/commit/2847cd75cd9a067e6b265864f1ed01c01ffe8a22)]:
  - @interfere/constants@1.0.0

## 1.0.0-alpha.6

### Major Changes

- [`17fe701`](https://github.com/interfere-inc/interfere/commit/17fe701227f2a55644eb77b254888a5a0657036d) Thanks [@skve](https://github.com/skve)! - feat: release v1

### Patch Changes

- Updated dependencies [[`17fe701`](https://github.com/interfere-inc/interfere/commit/17fe701227f2a55644eb77b254888a5a0657036d)]:
  - @interfere/constants@1.0.0-alpha.5

## 0.2.0-alpha.5

### Patch Changes

- bump

- Updated dependencies []:
  - @interfere/constants@0.2.0-alpha.4

## 0.2.0-alpha.4

### Minor Changes

- Added consent management features

## 0.2.0-alpha.3

### Patch Changes

- bump

- Updated dependencies []:
  - @interfere/constants@0.2.0-alpha.3

## 0.2.0-alpha.2

### Patch Changes

- bump

- Updated dependencies []:
  - @interfere/constants@0.2.0-alpha.2

## 0.2.0-alpha.1

### Patch Changes

- Fix silent initialization issues

- Updated dependencies []:
  - @interfere/constants@0.2.0-alpha.1

## 0.1.0-alpha.3

### Minor Changes

- [`949b3e2`](https://github.com/interfere-inc/interfere/commit/949b3e27002091477e4fdbd37e6c77d6de4ff780) Thanks [@skve](https://github.com/skve)! - Fixed dependency graph

### Patch Changes

- [`4506e8d`](https://github.com/interfere-inc/interfere/commit/4506e8d301a06600345dff51c7e141e783acee52) Thanks [@skve](https://github.com/skve)! - Updated readme

- Updated dependencies [[`949b3e2`](https://github.com/interfere-inc/interfere/commit/949b3e27002091477e4fdbd37e6c77d6de4ff780), [`4506e8d`](https://github.com/interfere-inc/interfere/commit/4506e8d301a06600345dff51c7e141e783acee52)]:
  - @interfere/constants@0.1.0-alpha.2

## 0.1.0-alpha.2

### Patch Changes

- [`2847cd7`](https://github.com/interfere-inc/interfere/commit/2847cd75cd9a067e6b265864f1ed01c01ffe8a22) Thanks [@skve](https://github.com/skve)! - Improvements, Native Next.js 16 support, and huge performance improvements.

- Updated dependencies [[`2847cd7`](https://github.com/interfere-inc/interfere/commit/2847cd75cd9a067e6b265864f1ed01c01ffe8a22)]:
  - @interfere/constants@0.0.2-alpha.1

## 0.1.0-alpha.1

### Minor Changes

- [#563](https://github.com/interfere-inc/interfere/pull/563) [`f27c758`](https://github.com/interfere-inc/interfere/commit/f27c758628076a985ce182cd0981989485425979) Thanks [@pjsny](https://github.com/pjsny)! - Add opt-in offline storage with automatic retry on reconnect

  **BREAKING:** Offline storage is now disabled by default. To persist failed envelopes to IndexedDB, explicitly enable it:

  ```ts
  createClient({
    surfaceToken: "...",
    offline: { enabled: true },
  });
  ```

  Changes:

  - Make IndexedDB persistence opt-in via `offline.enabled` config (default: `false`)
  - Remove localStorage backend—IndexedDB is the sole persistent storage option
  - Auto-flush stored envelopes when browser `online` event fires
  - Log envelope loss with hint when offline storage is disabled and retries fail
  - Default `maxQueueSize` is now 100 (was 1000)

## 0.0.2-alpha.0

### Patch Changes

- [`477492c`](https://github.com/interfere-inc/repo/commit/477492c61dc69df8e270871b57483bd2bfcb7100) Thanks [@skve](https://github.com/skve)! - Progress

- Updated dependencies [[`477492c`](https://github.com/interfere-inc/repo/commit/477492c61dc69df8e270871b57483bd2bfcb7100)]:
  - @interfere/constants@0.0.2-alpha.0

## 0.0.7

### Patch Changes

- [`8efa26f`](https://github.com/interfere-inc/repo/commit/8efa26f60bf2a715ce8e57008f94ec5650bbc49e) Thanks [@skve](https://github.com/skve)! - new structure

## 0.0.6

### Patch Changes

- [`8280db1`](https://github.com/interfere-inc/repo/commit/8280db16bd4deaa06cd14697b78be7da07327939) Thanks [@skve](https://github.com/skve)! - fixed crash

## 0.0.5

### Patch Changes

- [`cfb8e19`](https://github.com/interfere-inc/repo/commit/cfb8e1900fcb273bfd78f5426f3bcfe4c85d58cf) Thanks [@skve](https://github.com/skve)! - deleted unused code

## 0.0.4

### Patch Changes

- [`fc0c0a0`](https://github.com/interfere-inc/repo/commit/fc0c0a06032afdfcb1ae607b7e89f05f8a82f7e6) Thanks [@skve](https://github.com/skve)! - fixed provider

## 0.0.3

### Patch Changes

- 631c58e: added config types

## 0.0.2

### Patch Changes

- a3d7e4d: added types pkg
