# @interfere/server-core

## 0.2.2

### Patch Changes

- c7b2dbd: Bun applications now load the installed Undici transport when exporting server traces, logs and metrics.
- 72fabef: Node telemetry exporters negotiate HTTP/2 with compatible collectors and support HTTP/1.1, and browser fetch and XHR tracing sanitizes request URLs before they reach span processors.

- e8e3357: The React, Next.js, and Vite integrations now use OpenTelemetry 3 for tracing and log export, including in applications that already configure their own trace provider.
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

## 0.2.1

### Patch Changes

- 2279673: On Cloudflare Workers, a server flush no longer waits on a flush started by another request, so it no longer fails with "This promise was created by a request that has since ended".
- 7a535a6: On Cloudflare Workers, `register()` no longer waits on startup begun by another request, so a request that ends during cold start can no longer fail later requests with "This promise was created by a request that has since ended" or leave the SDK uninitialised.
- Updated dependencies [3ef4bdb]
  - @interfere/types@11.2.0

## 0.2.0

### Minor Changes

- 5f28fcd: Breaking: `serverCredential()` in `@interfere/server-core/credential` takes the release token as its second argument (`apiKey, releaseToken, publicKey`) and returns `{ releaseToken, publicKey? }` when there is no API key and a token is set, which `serverCredentialHeaders()` sends as `x-interfere-release-token` plus `x-interfere-public-key` when the public key is set.
- b8baffc: Server runtimes now send telemetry with `INTERFERE_API_KEY` when it is set, and send the public key in the `x-interfere-public-key` header instead of a URL query parameter.

### Patch Changes

- 4d63981: Every public package is now published with npm trusted publishing and a provenance attestation.
- 2876c24: Browser bundles now carry one copy of each OpenTelemetry package, which makes the telemetry chunk about 1.1 KiB smaller gzipped and stops most of a page's spans and log records from being lost before export.

- b77bfb3: Server spans now read session and device ids only from `interfere.*` baggage, so browsers still on SDK 12.x or earlier must upgrade to 13.x to keep server spans linked to their sessions.

- Updated dependencies [4d63981]
- Updated dependencies [b8c5bad]
- Updated dependencies [9aaddd6]
- Updated dependencies [d1b1138]
- Updated dependencies [ead7ecb]
- Updated dependencies [042188e]
- Updated dependencies [6e18c76]
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
  - @interfere/types@11.1.0

### Network

- Headers:
  - b8baffc: when `INTERFERE_API_KEY` is set, Node server exports (Next.js, Vite) add `authorization: Bearer <INTERFERE_API_KEY>` and Next.js Edge exports send `x-api-key`; every server export sends `x-interfere-public-key` when the public key is set.
- Origins:
  - b8baffc: `POST {INTERFERE_API_URL}/drain/v1/{traces,logs,metrics}` (default `https://api.interfere.com`), server to server, no longer carries the `public_key` query parameter.
- Data sent and consent:
  - b8baffc: with `INTERFERE_API_KEY` set, server telemetry belongs to the workspace and its `service.name` instead of being attributed through the public key; browser requests are unchanged.
  - 2876c24: spans and log records that were dropped before export when two OpenTelemetry copies mixed are now sent.
  - b77bfb3: server spans no longer copy the legacy `session.id` and `device.id` baggage keys, so vendor baggage such as Datadog's `session.id` is never recorded.

## 0.1.3

### Patch Changes

- 13e0639: Capture the `x-forwarded-for` client IP header with server request errors by default, including Next.js Node and Edge request errors and Vite request middleware failures. A client IP address is personal data in many jurisdictions; if your privacy notice or data processing terms do not cover it, set `privacy: { clientIp: false }` in server `register()` to omit this header from captured exception events. Authorization and cookie headers remain excluded.
- 13e0639: Include MIT licenses and package documentation, declare tree-shaking behavior, and document the supported Node.js and framework versions. Complete server adapter package metadata and link license badges to the included license.
- 13e0639: Remove workspace-only resolution conditions from stable and canary packages so consumers using custom conditions resolve shipped runtime and type files. Qualification tests now use the same sanitized package manifests while development source resolution stays unchanged.
- Updated dependencies [13e0639]
- Updated dependencies [13e0639]
- Updated dependencies [13e0639]
- Updated dependencies [13e0639]
- Updated dependencies [13e0639]
  - @interfere/types@11.0.2
  - @interfere/constants@9.3.3
  - @interfere/helpers@9.3.4

## 0.1.2

### Patch Changes

- 76a8c1e: Server `register()` now prints one warning when it adopts a tracer provider another tool registered first, such as Sentry or dd-trace with its OpenTelemetry bridge. Server errors still reach Interfere, but request traces arrive only if that provider exports them over OTLP; pass `register({ tracing: "application" })` to confirm the setup and silence the message. When dd-trace traces the process without its OpenTelemetry bridge, `register()` also warns once that Datadog and Interfere record separate traces and that dd-trace owns outbound trace headers.
- 76a8c1e: Server error handling now follows the host's lifecycle instead of overriding it.

  - After an uncaught exception, the SDK captures the error, flushes telemetry, and exits with code 1 only when no other `uncaughtException` listener exists, which is when Node itself would have exited. Under `next start`, which keeps its own listener, the server keeps serving instead of being shut down by the SDK in production.
  - `@interfere/vite/server` now captures uncaught exceptions and unhandled rejections in the server process. The `captureUncaughtException` and `captureUnhandledRejection` options turn this off.
  - Telemetry is flushed on `SIGTERM`, `SIGINT` and when the event loop drains, for at most 2 seconds. If your app handles the signal itself, shutdown stays yours. Otherwise the SDK flushes and then lets the signal terminate the process as it would have.
  - `register()` from `@interfere/next` does nothing during `next build`: it installs no process handlers and makes no network calls.
  - Registering again, for example after a dev-server module reload, replaces the SDK's process listeners rather than adding more.

- 76a8c1e: Server errors are now delivered even when the incoming request arrived with an unsampled trace parent (for example from a browser SDK sampling below 100%) or when `OTEL_TRACES_SAMPLER` drops the trace. Other spans still follow the configured sampling. Span attribute values and error status messages are capped at 64 KiB so one very large error cannot push an export batch past the ingestion size limit; `OTEL_ATTRIBUTE_VALUE_LENGTH_LIMIT` and `OTEL_SPAN_ATTRIBUTE_VALUE_LENGTH_LIMIT` still take precedence.
- Updated dependencies [30a48a6]
- Updated dependencies [76a8c1e]
- Updated dependencies [76a8c1e]
- Updated dependencies [76a8c1e]
- Updated dependencies [76a8c1e]
- Updated dependencies [76a8c1e]
- Updated dependencies [76a8c1e]
  - @interfere/types@11.0.1
  - @interfere/helpers@9.3.3

## 0.1.1

### Patch Changes

- 38bc8fc: Move the server telemetry runtime into `@interfere/server-core`, a new runtime dependency of `@interfere/next` and `@interfere/vite`. It owns registration, provider ownership, exception normalization and delivery, URL privacy, console bridging and bounded provider flushing, so concurrent server initialization emits each log once and captured exceptions export once. Next keeps its framework error hooks and Vercel drain integration; Vite keeps Undici request instrumentation and metrics.

  Vite redacts sensitive URL query values, applies the outbound propagation policy when it injects headers, and exposes `flush()` with the shared privacy options. React initialization and close have explicit lifecycle states, clean up after a failed startup, keep consent when the provider omits the consent prop, and dispose page-wide Web Vitals observers with their kernel.

- Updated dependencies [f584c2f]
- Updated dependencies [68a9ee0]
- Updated dependencies [2938037]
- Updated dependencies [3dfaefa]
- Updated dependencies [8f92d43]
- Updated dependencies [c76563f]
- Updated dependencies [3ff22ac]
- Updated dependencies [3226d63]
- Updated dependencies [ec97a9c]
- Updated dependencies [7c8441a]
  - @interfere/types@11.0.0
  - @interfere/constants@9.3.2
  - @interfere/helpers@9.3.2
