<p align="center">
  <a href="https://interfere.com">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://images.interfere.com/cdn-cgi/imagedelivery/ZQ3TDZdJHXvYNyL5BNulpQ/v2/header/logo-dark/format=auto">
      <img src="https://images.interfere.com/cdn-cgi/imagedelivery/ZQ3TDZdJHXvYNyL5BNulpQ/v2/header/logo-light/format=auto" height="64" alt="Interfere">
    </picture>
  </a>
  <h1 align="center">@interfere/next</h1>
</p>

<p align="center">
  <a href="https://www.npmjs.com/package/@interfere/next"><img src="https://img.shields.io/npm/v/@interfere/next.svg" alt="npm version" /></a>
  <a href="./LICENSE"><img src="https://img.shields.io/npm/l/@interfere/next.svg" alt="License" /></a>
  <a href="https://www.npmjs.com/package/@interfere/next"><img src="https://img.shields.io/npm/dm/@interfere/next.svg" alt="npm downloads" /></a>
</p>

<p align="center">
  Next.js SDK for <a href="https://interfere.com">Interfere</a> — error tracking, session replay, and analytics for modern product teams.
</p>

<p align="center">
  <a href="https://interfere.com/docs">Documentation</a>
  ·
  <a href="mailto:support@interfere.com">Get Help</a>
</p>

---

## Getting Started

### Prerequisites

- Next.js `^15.3.0 || ^16.0.0`
- React 19 (`^19.0.0`)
- Node.js 24.x

### Installation

```bash
npm install @interfere/next
```

### Installing a fresh release

Release-age policies can block a newly published SDK, including its transitive `@interfere/javascript` and `@interfere/build` packages. A pnpm policy of `minimumReleaseAge: 10080` waits seven days; Bun's `minimumReleaseAge = 1209600` waits fourteen days. Stable publication starts its own clock after RC qualification.

For an approved early preview, retain existing exclusions and add the required Interfere packages to pnpm's `minimumReleaseAgeExclude` (which supports `"@interfere/*"`) or Bun's `install.minimumReleaseAgeExcludes`. Regenerate and commit the lockfile, restore frozen-lockfile enforcement, and verify the normal CI install.

### Quick Start

**1. Wrap your Next.js config**

```ts
// next.config.ts
import { withInterfere } from "@interfere/next/config";
import type { NextConfig } from "next";

const config: NextConfig = {};

export default withInterfere(config);
```

**2. Wire instrumentation**

```ts
// instrumentation.ts
export { onRequestError, register } from "@interfere/next/instrumentation";
```

**3. Add the provider**

```tsx
// app/layout.tsx
import { InterfereProvider } from "@interfere/next/provider";

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>
        <InterfereProvider>{children}</InterfereProvider>
      </body>
    </html>
  );
}
```

`InterfereProvider` captures render errors by default and rethrows them so your host error boundary keeps control of the fallback. Set `captureRenderErrors={false}` to disable this capture.

**4. Load the browser SDK**

```ts
// instrumentation-client.ts
import "@interfere/next/instrumentation-client";
```

Next.js loads `instrumentation-client.ts` before hydration, and a production build with `INTERFERE_PUBLIC_KEY` set stops when the file is missing. To pass options, replace the import with `void init({ … })` from `@interfere/next/instrument-client`, as the sections below show. Do not `await` it: top-level `await` turns the file into an async module, and bundlers warn when the browser targets do not support it.

### Service identity

When release preflight resolves an App whose slug differs from the inferred service name, the build warns once and continues. Set `serviceName` in `withInterfere()` to preserve your existing name; an explicit build option or `OTEL_SERVICE_NAME` suppresses the warning. A workspace key with no resolved App produces no name warning.

`withInterfere` uses the application's unscoped `package.json` name as the default `service.name` for browser, server, and edge telemetry. For example, `@acme/platform` becomes `platform`. A build-time `OTEL_SERVICE_NAME` overrides the package name, or set a shared default explicitly:

```ts
export default withInterfere({ serviceName: "platform" }, config);
```

`withInterfere` injects this name into the browser, server and edge bundles, so `init()` and `register()` need no `serviceName`. Pass one there only to override a single runtime: an explicit `serviceName` in browser `init()` or server/edge `register()` takes precedence, and server and edge runtimes also honor a runtime `OTEL_SERVICE_NAME` ahead of the build default. If no name can be inferred or configured, OpenTelemetry's unknown-service fallback remains.

When upgrading from 12.x, keep `serviceName` explicit in `withInterfere()` if your existing name differs from the unscoped package name, and keep an application-owned provider's service name aligned too. This includes names previously set through `OTEL_SERVICE_NAME`. In the Interfere dashboard, open your workspace's **Services** page, find the service linked to your existing App, and copy its service name; you can also inspect the `service.name` resource attribute on a recent span. With workspace keys, changing this name can select or create a different App.

## What remote configuration can change

The SDK config response contains `plugins` and `protocolVersion`, plus optional `backendHosts`, `ingestUrl`, `anonymousReplayPolicy`, `logSourcing`, `platform` and `serverTracing` fields. The browser resolver consumes the plugin settings, backend hosts, protocol version and ingest URL; a build with baked plugin settings uses those settings and baked backend hosts instead of fetching runtime config. `ingestUrl` selects the origin for telemetry uploads, separately from the config API origin.

Remote plugin settings can disable shipped plugins; they cannot override local disables or consent. Lazy plugins load through dynamic imports of code-split chunks shipped in the installed package, not executable code fetched from an Interfere CDN; core error capture is bundled eagerly. Remote configuration cannot enable code the installed version does not ship.

Remote `backendHosts` become discovered HTTPS origins. They receive trace headers on `GET`/`HEAD` initially; other methods receive them only after a header-bearing `GET`/`HEAD` to that origin receives a response in this browser session. Explicit `tracing.propagateTo` origins and their header lists are declared in application code and take precedence over discovered hosts. `tracing.headers: false` disables SDK header writes everywhere. Remote configuration cannot override either setting.

## Workspace keys

Every workspace receives a **Default** workspace public key. Use one workspace key across its Apps; additional keys can be created or rotated in **Settings → Developer → Public keys**. App keys (formerly surface keys) are deprecated but still accepted, with no removal date announced. An App key stays bound to its App and bypasses workspace routing.

With a workspace key, App resolution uses:

1. The `x-interfere-service` header, with optional `x-interfere-service-namespace`.
2. Otherwise, OTLP `service.name` and `service.namespace` resource attributes.
3. Without a usable identity, the workspace's default App. `unknown_service` never creates an App.

For a named service, an existing service-to-App association wins; otherwise a matching App name or slug is used. First sight of an unmatched service can auto-create an App, limited to **20 new Apps per request** and **50 auto-created Apps per workspace**. If creation is disabled or a budget is exhausted, routing falls back to the default App; without an eligible default, the request is rejected. A missing or deleted default falls back to the oldest eligible App.

In **Settings → Developer → Public keys**, choose **Default app** and toggle **Create apps from telemetry** (`auto_create_apps`). An App marked `explicit_only` is excluded from implicit name matching and default fallback; use an existing service association or its App key to target it.

Name an App in the dashboard when creating it, or edit its title on the App page. Keep the SDK's `serviceName` stable: renaming the dashboard title does not change an existing service association. Open **Services** to find the service linked to an App before migrating keys. SDK requests use `serviceName` for `x-interfere-service`; a configured runtime `serviceNamespace` supplies the namespace header.

Set the name once in `withInterfere({ serviceName: "storefront" }, config)`; the build passes it to browser `init()` and server/edge `register()`. Build identity uses the explicit option, then `OTEL_SERVICE_NAME`, then the package name without its scope. Keep an application-owned OpenTelemetry provider aligned too.

### Environment Variables

| Variable               | Required                 | Description                                                                                                                                                                                                                                                                                                     |
| ---------------------- | ------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `INTERFERE_PUBLIC_KEY` | Yes                      | Public workspace or App key in the `interfere_public_us_*` or `interfere_public_eu_*` format. Public by design; used for ingestion, and with serviceName at build time to resolve the App that a release belongs to. A build without it logs a warning and sends no events.                                     |
| `INTERFERE_API_KEY`    | Yes for release metadata | Workspace secret key in the `interfere_secret_us_*` or `interfere_secret_eu_*` format, with the **Publish releases** (`release:write`) permission. One key publishes releases for every app in the workspace. Server-side build credential; never expose in browser code. A build without it skips the release. |
| `INTERFERE_API_URL`    | No                       | Override the ingest endpoint in `direct` mode (e.g. a regional endpoint). Defaults to Interfere's endpoint.                                                                                                                                                                                                     |

### Connect releases to Vercel

In Interfere, connect Vercel and link this Surface to its Vercel project. Also link the Surface to its source repository. These links let the build register releases and upload source maps; installing the SDK does not select a project for you.

An incomplete Surface setup produces a build warning and skips release registration and source-map upload. It does not fail the application build or disable SDK monitoring. Source-map cleanup still follows your `purge` setting. Complete the links and redeploy to enable release attribution and source-mapped stack traces.

For Vercel deployments, choose the Vercel project rather than CLI as the deployment target. CLI registers a generic deployment and does not provide the Vercel project mapping used for deployment events and drains.

### Source-map uploads and cleanup

Source-map uploads and cleanup are enabled by default. To upload only in production, select the deployment environment explicitly:

```ts
import { withInterfere } from "@interfere/next/config";

export default withInterfere(
  {
    sourceMaps: {
      upload: process.env.INTERFERE_ENVIRONMENT === "production",
      purge: true,
    },
  },
  nextConfig,
);
```

Set `INTERFERE_ENVIRONMENT` to the deployment environment, such as `staging` or `production`. `NODE_ENV` is usually `production` for both staging and production builds, so it cannot distinguish them.

With `upload: false`, the SDK still registers the release and retains browser/server monitoring. It skips source-map signing, upload, and finalization requests. Stack traces cannot be resolved to original source in environments without uploaded maps. Release registration still requires the build API key.

`purge` controls cleanup independently. When true, the SDK deletes discovered maps and removes their source-map references after the pipeline, including upload failures or intentionally disabled uploads. Cleanup failures are reported separately without replacing the original pipeline error. Set `purge: false` to retain maps for another uploader. Setting both options to false also skips source-map discovery in the build hook.

A production build registers a release when `INTERFERE_API_KEY` is set, so set it on builds that you deploy. Without it, the build logs a warning and skips the release. `upload: false` skips only the source maps; it does not skip the release.

To turn Interfere off for a build, pass `withInterfere({ disable: true }, nextConfig)`. The build skips the release, the source-map upload and every Interfere check, and the browser and server SDKs send no events.

Only configuration that is wrong fails a production build: an `INTERFERE_API_KEY` with the wrong prefix, a missing commit SHA when the key is set, an invalid `apiHost`, or a public key without an `instrumentation-client` file. A missing `INTERFERE_PUBLIC_KEY` logs a warning; with a workspace secret key, the build also cannot publish its release. Errors from the Interfere API, including a rejected secret key, are logged and do not fail the build.

### Migrating From 0.x

Upgrade all `@interfere/*` SDK packages together. Keep the env var names, but replace old `int_pub_*` / `interfere_pk_*` values with the dashboard's regional public key, and replace old `ak_*` / `interfere_ak_*` build keys with the regional Interfere secret key.

Secret keys that were created for one app keep working. New secret keys belong to the workspace, so set `INTERFERE_PUBLIC_KEY` on the build machine as well.

## Server tracing and Vercel drains

The standard instrumentation export works on Vercel and other Next.js hosts. In `full` mode, Interfere exports application spans directly. When the build config selects `drain`, the Node SDK initializes `@vercel/otel` with its default processors so Vercel can forward application spans through the configured Trace Drain. Platform spans and application spans remain separate observations.

The mode and the hosting platform come only from the surface configuration that the build injects. When that configuration reports the Vercel platform (the Surface is linked to a Vercel project), direct trace and log exports are capped at 2 s so a flush cannot hold the invocation open. Edge always exports directly because Vercel does not forward custom Edge spans through Trace Drains. Edge reads the build-injected release identity, falling back to the configured commit SHA.

| Server `register()` option | Default       | Behavior                                                                                                                                                                                                                                                      |
| -------------------------- | ------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `consoleBridge`            | `true`        | `false` disables console logs; `{ level: "warn" }` bridges warnings and errors only. Levels are `debug`, `info` (including `log`), `warn`, and `error`. Omitted `level` means `debug`; lower-level methods remain untouched. Log drain mode skips the bridge. |
| `errors`                   | `"interfere"` | Application mode only. `"interfere"` records an error on your active span and also delivers it through Interfere's exporter as a linked exception span; `"host"` records it on your span only, for providers that export their traces to Interfere.           |

### When your application already owns OpenTelemetry

Keep your provider and tell Interfere not to register one:

```ts
import { registerOTel } from "@vercel/otel";
import {
  correlationSpanProcessor,
  register as registerInterfere,
} from "@interfere/next/instrumentation";

export { onRequestError } from "@interfere/next/instrumentation";

export async function register() {
  registerOTel({
    serviceName: "my-app",
    spanProcessors: [correlationSpanProcessor(), "auto"],
  });
  await registerInterfere({ tracing: "application" });
}
```

`correlationSpanProcessor()` returns a fresh standard OpenTelemetry span processor for your provider. Install it when constructing the provider to copy Interfere session and device baggage onto request and child spans; unrelated vendor baggage stays unchanged. Keep W3C baggage extraction enabled in your host propagator. On Vercel, `"auto"` retains Vercel's export processors, and a configured Interfere Trace Drain receives the enriched spans alongside your existing exporter. The processor does not install a provider, propagator or exporter.

`register({ tracing: "application" })` writes no OpenTelemetry global and installs no instrumentation. Every server error reaches Interfere through Interfere's own exporter: an error raised inside one of your spans is recorded on that span and also shipped as an Interfere exception span that links to your trace and span ids and carries the same exception attributes and request context; an error with no active span ships as that exception span alone. Your provider needs no exporter pointed at Interfere for errors to arrive, and ordinary host spans stay wherever your provider sends them. When your provider does export its traces to Interfere, the host-span copy carries `interfere.exception.delivery: "interfere"` and ingest keeps one copy, so nothing appears twice; pass `errors: "host"` to record on your span only and let your exporter deliver it. The `console.*` bridge emits into the logger provider you registered, or into a private Interfere provider when you registered none. Interfere emits no SSR trace meta in this mode, and `withInterfere()` leaves `clientTraceMetadata` to your propagator. To receive your traces as well, ship them to Interfere with the standard exporter variables, or through a collector that fans out:

```bash
OTEL_EXPORTER_OTLP_ENDPOINT="https://api.interfere.com/drain"
OTEL_EXPORTER_OTLP_PROTOCOL="http/protobuf"
OTEL_EXPORTER_OTLP_HEADERS="Authorization=Bearer%20${INTERFERE_API_KEY}"
```

On Vercel, application mode also supports the Interfere Trace Drain. Link the App to its Vercel project, configure the Interfere Trace Drain, and retain `"auto"` in your host `registerOTel({ spanProcessors: [...] })` configuration alongside your correlation processor. Keep your existing `traceExporter` or exporter processor for Datadog, Langfuse or another destination; the drain forwards the enriched application spans to Interfere. Repointing `OTEL_EXPORTER_OTLP_*` to Interfere replaces the host's environment-configured destination, so use the drain to preserve that existing export path. Outside Vercel, configure collector fan-out or multiple exporters when constructing your provider. Custom Edge spans still require direct OTLP export.

The key needs the **Send telemetry** permission. Interfere needs no sampler, ID generator or propagator of its own to ingest those traces. Set the tracing owner on the surface in the dashboard so the browser bundle and the server agree; the `tracing` option overrides the surface value for one process and logs once when the two differ.

With no surface owner configured, the default is `tracing: "auto"`: register your application's provider first and Interfere adopts its active spans without a warning; with no registered provider, Interfere uses managed tracing. An explicit `tracing` option overrides the build-injected surface owner. `tracing: "managed"` registers Interfere's tracer provider, context manager, propagator chain and logger provider. If another library, such as Sentry, registered a tracer provider first, managed `register()` still completes: it logs one warning naming `tracing: "application"`, keeps capturing server errors through its own exporter and writes no other OpenTelemetry global. Choose application mode when that provider is intentional.

`PrerenderSafeIdGenerator` was removed in 13.0.1. Managed `register()` installs its own generator.

Interfere's server code never fails your app. If `register()`, `onRequestError`, `captureError`, `flush()`, `withInterfereProxy`, `withInterfereServerAction` or `withInterfereAfter` hits an error inside Interfere, it logs one `[interfere] … failed inside the Interfere SDK` warning for that call site and your code continues without Interfere. The wrappers always rethrow your own error unchanged.

`@vercel/otel@2.1.3` supports OTel API `>=1.9.0 <2`, stable SDK peers `>=2 <3`, and experimental peers `>=0.200.0 <0.300.0`. The SDK declares all required peers as dependencies, including `sdk-metrics`. Custom compositions must use compatible versions too.

### Errors inside `after()`

Next.js does not report a throw inside an `after()` callback to `onRequestError`; it only logs it to the server console. Wrap the task with `withInterfereAfter` so the failure is captured (mechanism `auto.function.nextjs.after`) and then rethrown, which keeps Next's own log. The callback still runs after the response is sent, so the response is not delayed. Wrapping a promise works the same way.

```ts
import { after } from "next/server";
import { withInterfereAfter } from "@interfere/next/server";

after(
  withInterfereAfter(async () => {
    await recordAnalytics();
  }),
);
```

### Migrating a private request-error-only provider

The removed `buildInterfereOtelKit` has no drop-in replacement for a private `BasicTracerProvider` that forwards only exception spans. `register({ tracing: "application" })` plus `correlationSpanProcessor()` preserves host-provider ownership and correlation, but is not an errors-only export option: errors on an active recording host span reach Interfere only if that span is exported there. Omitting Interfere OTLP settings does not create an independent copy, and adding a Vercel Trace Drain forwards ordinary host spans too. Independent error delivery is planned for rc.3: application mode will send an isolated exception linked to the host context by default (`errors: "interfere"`), with `errors: "host"` for intentional host-only delivery. This is not available in rc.1/rc.2; keep the migration blocked until the exact candidate includes the fix and proves the restricted delivery contract.

Preserve `consoleBridge: false`, existing process error handlers, empty captured request headers and query stripping. In the browser, `init({ consent: false, tracing: { mode: "application", headers: false, captureFailedRequests: false } })` denies analytics/replay, stops Interfere headers and disables failed-response promotion. Keep existing device/log/replay plugin restrictions: consent alone does not disable logs or session storage. `headers: false` requires rc.2 or a later candidate containing that option. Existing Datadog/Sentry settings remain application-owned.

### Server fetch propagation

In managed Vercel drain mode, `register()` automatically enables Vercel fetch instrumentation for the backend hosts configured for your workspace, using the same build-time configuration as the browser and treating each host as an exact `https://` origin. Add additional origins with `register({ propagateTo: ["https://api.example.com"] })`; duplicates are removed. Entries must be exact HTTPS origins, with an explicit port when needed; paths, credentials and wildcards are rejected. Matching never includes sibling or lookalike hosts. Vercel's deployment-host and localhost defaults remain in effect when fetch instrumentation is enabled. With no configured backend hosts or `propagateTo` option, no fetch instrumentation is installed.

Managed direct-export mode does not install fetch instrumentation, even with configured backend hosts or `propagateTo`. `propagateTo` also does not add fetch instrumentation on Edge or configure an application-owned provider. Configure your host's fetch instrumentation in application mode. Additional browser origins remain separately configured through `init({ tracing: { propagateTo } })`.

### Browser session correlation

Unless external propagation is selected, the SDK writes `traceparent` and `tracestate` to instrumented requests that carry none, keeps a `traceparent` another tool wrote first and records the trace id each request left with on the request observation. `tracing.headers` restricts the headers written (a list) or turns them off everywhere (`false`) while requests stay observed, and a `tracing.propagateTo` entry `{ origin, headers }` names exactly the headers one origin receives. A backend host configured for your workspace carries headers on `GET` and `HEAD` from the first request and on other methods once one header-bearing `GET` or `HEAD` to it has answered in this browser session, so its CORS policy is proved before a `POST` carries headers; the page origin and `propagateTo` origins always carry them. The SDK never resends an application request. With baggage enabled (the default), it sends `interfere.session.id` in W3C baggage on instrumented requests to the page origin, to exact HTTP(S) origins listed in `tracing.propagateTo`, and to the backend hosts configured for your workspace as exact `https://` origins, unless analytics consent is explicitly denied. Sibling subdomains, other schemes and other ports need their own `propagateTo` entry. Cross-origin APIs must allow `traceparent, tracestate, baggage` in `Access-Control-Allow-Headers`. The SDK diagnoses a suspected CORS rejection once per origin and never retries the request or strips headers. The server adapter's baggage processor attaches the session ID to request and child spans; `traceparent` continues to determine trace linkage. Set `init({ tracing: { propagateTraceBaggage: false } })` to disable Interfere baggage injection while retaining permitted trace-context headers and browser session/device attributes. This also disables baggage injection through Interfere’s managed OpenTelemetry propagator. Existing caller/vendor baggage is not removed; backend session association then depends on available identity-bearing browser telemetry in the same trace.

### External trace propagation

Set `tracing: { propagation: "external" }` when another instrumentation layer owns outgoing trace context. Interfere then writes neither `traceparent` nor `tracestate`, including through its managed propagator. Existing application headers remain intact. Baggage retains its independent default-on policy; set `propagateTraceBaggage: false` when the API does not accept it.

`auto` with external propagation resolves to application mode and claims no OpenTelemetry globals. Explicit `managed` mode can still record Interfere spans and claim an available context manager, but does not register a propagator. With default propagation, `auto` checks OpenTelemetry registration ownership only; it does not inspect other SDK globals or guess which private request wrappers will run later.

When external instrumentation privately clones request headers, call the generic helper from its supported request-completion hook:

```ts
import { correlateRequestTrace } from "@interfere/next/request-trace";

correlateRequestTrace(response, sentRequestHeaders);
```

Pass the exact fetch `Response` and the final **request** headers, not response headers or headers reconstructed from a URL. The helper reads W3C trace context, preserves request-time identity and timestamp, and changes neither headers nor callback return values. XHR correlation reads final intercepted request headers directly. No request is replayed and no correlation is guessed from URLs or timing.

A request without valid trace context or without an available completion hook remains unlinked. Delayed callbacks after consent changes or SDK disposal do not emit correlation. Browser telemetry records the outgoing trace ID in `interfere.request.trace_id`, which can differ from an Interfere span's own trace ID. Baggage independently carries identity to the backend; disabling it requires identity-bearing browser telemetry to be retained and indexed for trace-based association.

## Upgrading from 12.x

Upgrade every `@interfere/*` package together. Most apps change nothing: `export { onRequestError, register } from "@interfere/next/instrumentation"` and `<InterfereProvider>` are the whole setup. The 13.0 release notes carry the full option table; the items below are the ones that need an edit.

| 12.x                                                                  | 13.x                                                                                                                                                                 |
| --------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Your 12.x service name differs from the package name                  | Keep `serviceName` explicit in `withInterfere()`. Copy the name from dashboard **Services** to preserve workspace-key App routing.                                   |
| Your Vercel provider already exports to another backend               | Keep that exporter and `"auto"` span processors; configure the Interfere Trace Drain. The `OTEL_EXPORTER_OTLP_*` example replaces the existing destination.          |
| The 12.x server OTel kit composed into your own provider              | Removed. Use `register({ tracing: "application" })` and export your provider's traces to Interfere over OTLP, as shown above.                                        |
| A private provider that forwarded only exception spans to Interfere   | `register({ tracing: "application" })` delivers every server error through Interfere's exporter by default; your provider's spans and exporters are untouched.       |
| `kit.spanProcessors` supplied browser session correlation             | Add `correlationSpanProcessor()` to your host provider at construction; retain `"auto"` with `registerOTel` for the Vercel Trace Drain.                              |
| `PrerenderSafeIdGenerator`, `kit.idGenerator`, `kit.propagators`      | The kit fields and `PrerenderSafeIdGenerator` are removed. Managed `register()` owns the generator and the propagator order; application mode keeps your provider's. |
| `<TraceMeta />` from `@interfere/next/server`                         | Remove it. `withInterfere()` injects stamped trace meta through Next's `clientTraceMetadata`; an unstamped `traceparent` meta tag no longer links.                   |
| `<InterfereProvider errorBoundary={false}>`                           | `<InterfereProvider captureRenderErrors={false}>`                                                                                                                    |
| `init({ tracing: { apiHosts: ["api.example.com"] } })`                | `init({ tracing: { propagateTo: ["https://api.example.com"] } })`; `apiHosts` was removed in 13.0.1.                                                                 |
| `init({ tracing: { propagateContextUrls } })` and same-site wildcards | Removed. Only the page origin, `propagateTo` entries and configured backend hosts receive trace headers.                                                             |

## Filtering errors

Browser `init()` takes four options that decide which exceptions leave the page. They apply to every capture source: uncaught errors, unhandled rejections, render errors, console errors and `capture()`. They do not apply to failed-request exceptions, which `tracing.captureFailedRequests` controls.

```ts
// instrumentation-client.ts
import { init } from "@interfere/next/instrument-client";

void init({
  ignoreErrors: ["ResizeObserver loop", /^AbortError: /],
  denyUrls: [/^https:\/\/widgets\.example\//, "cdn.ads.example"],
  errors: { sampleRate: 0.5 },
  beforeSend: (exception) => {
    if (exception instanceof Error) {
      exception.message = exception.message.replace(
        /token=\S+/g,
        "token=[redacted]",
      );
    }

    return exception;
  },
});
```

| Option              | Effect                                                                                                                                                                                                                                                                      |
| ------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `ignoreErrors`      | Drops an exception when its message, or `Type: message`, matches. Strings match as substrings; regular expressions are tested.                                                                                                                                              |
| `denyUrls`          | Drops an exception when the script URL of its innermost stack frame matches. When the stack has no location, the `window.onerror` source URL is used.                                                                                                                       |
| `errors.sampleRate` | Sends this fraction of exceptions, from `0` (none) to `1` (all, the default).                                                                                                                                                                                               |
| `beforeSend`        | Runs last, for exceptions the other options kept. Mutate the exception, return a replacement, or return `null` to drop it. The second argument carries `originalException` and `mechanism`. If the hook throws or returns anything else, the exception it received is sent. |

## Ingest Endpoint

By default the SDK posts telemetry to Interfere's endpoint. Ingest requests never carry cookies. Set `apiHost` to a subdomain of your own site that resolves to Interfere via a DNS record — this survives ad-blockers with no server-side component:

```ts
// next.config.ts
export default withInterfere({
  interfere: { apiHost: "https://in.yourapp.com" },
});
```

Add a CNAME record pointing your subdomain at the target shown in your dashboard; Interfere provisions the TLS certificate automatically. To target a regional endpoint instead, set `apiHost` (or the `INTERFERE_API_URL` env) to it.

## Identity Management

Link sessions to your authenticated users with `identity.set()`:

```tsx
import { useInterfere } from "@interfere/next/provider";

function useInterfereIdentity() {
  const { identity } = useInterfere();

  // Clerk, Auth0, etc.
  const { user } = useAuthProvider();

  useEffect(() => {
    if (user) {
      identity.set({
        company: { tier: "Enterprise" },
        identifier: user.id,
        name: user.name,
        email: user.email,
        source: { type: "clerk", name: "Clerk" },
      });
    } else {
      identity.clear();
    }
  }, [user]);

  return null;
}
```

`company.tier` sets the company’s tier in Interfere; tiers are managed under Settings or the tiers API.

### Parameters

| Field        | Required | Description                                                                                                                         |
| ------------ | -------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| `identifier` | Yes      | Unique user ID (your internal ID, not email)                                                                                        |
| `source`     | Yes      | Auth source: `{ type: "clerk", name: "Clerk" }`, `{ type: "auth0", name: "Auth0" }`, or `{ type: "custom", name: "Your Provider" }` |
| `name`       | No       | Display name                                                                                                                        |
| `email`      | No       | Email address                                                                                                                       |
| `avatar`     | No       | Avatar URL                                                                                                                          |
| `company`    | No       | Company tier: `{ tier: "Enterprise" }` (trimmed, 1–100 characters).                                                                 |
| `traits`     | No       | Arbitrary key-value metadata (`Record<string, unknown>`)                                                                            |

### API

| Method                 | Description                                                                                                                  |
| ---------------------- | ---------------------------------------------------------------------------------------------------------------------------- |
| `identity.set(params)` | Link the current session to a user. Repeated identity calls are deduplicated; changing the company tier sends a new request. |
| `identity.clear()`     | Clears the linked identity and rotates the session. Call on logout.                                                          |
| `identity.get()`       | Returns the current `IdentifyParams`, or `null` if no identity has been set.                                                 |

Identity is automatically cleared when the SDK is closed. Call `identity.clear()` on logout to start a fresh anonymous session.

## Consent Management

Omitting consent preserves the SDK defaults. To deny analytics and replay from startup, pass `consent: false` to `init()` before mounting the provider. Use the provider’s `consent` prop to keep the current decision synchronized:

```tsx
// app/layout.tsx
import { InterfereProvider } from "@interfere/next/provider";

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>
        <InterfereProvider consent={{ analytics: true, replay: false }}>
          {children}
        </InterfereProvider>
      </body>
    </html>
  );
}
```

### Consent categories

| Category    | Plugins                               | Gated?    |
| ----------- | ------------------------------------- | --------- |
| `necessary` | Error tracking                        | Always on |
| `analytics` | Page events, rage clicks, fingerprint | Yes       |
| `replay`    | Session replay                        | Yes       |

Omitting the `consent` prop preserves consent supplied to `init()`, including later updates through `consent.set()`. If no initial consent was supplied, omitting the prop allows all features. Consent controls analytics and replay permission; it does not enable plugins disabled in configuration. Objects replace the current decision, and omitted categories remain allowed. Errors, logs, tracing, and session storage are not disabled by this setting.

### Imperative API

Use `consent.set()` and `consent.get()` from the `useInterfere` hook:

```tsx
const { consent } = useInterfere();

consent.set(false);
consent.set(true);
consent.set({ analytics: true, replay: false });
consent.get();
```

### Integration with consent libraries

Works with any consent management platform — c15t, CookieYes, OneTrust, etc.:

```tsx
import { InterfereProvider } from "@interfere/next/provider";

function Layout({ children }: { children: React.ReactNode }) {
  const { has } = useConsentManager(); // from your CMP

  return (
    <InterfereProvider
      consent={{ analytics: has("measurement"), replay: has("experience") }}
    >
      {children}
    </InterfereProvider>
  );
}
```

### Initial consent via bootstrap

To set consent before React renders (avoiding any window where non-consented plugins might load), pass it to `init()`:

```ts
// instrumentation-client.ts
import { init } from "@interfere/next/instrument-client";

void init({ consent: false });
```

The provider's `consent` prop will then keep it in sync as the user updates their preferences.

## Privacy

### Private elements

Mark elements whose content the SDK never records in browser `init()`. The selectors apply to session replay and to click, submit and rage-click records from the first capture, even when replay is disabled:

```ts
// instrumentation-client.ts
import { init } from "@interfere/next/instrument-client";

void init({
  privacy: { blockSelector: "[data-sensitive]" },
});
```

The `interfere-block` and `interfere-mask` classes always apply. Interaction records keep the event, tag and field label of a private target, never its text or value. `replay.blockSelector` and `replay.maskTextSelector` on the provider apply to replay only. `<InterfereProvider plugins={{ pageEvents: { interactions: false } }}>` drops click and submit spans but keeps navigation, document-load and resource timing. The [`@interfere/react` README](https://www.npmjs.com/package/@interfere/react#privacy) lists every recorded field.

### Server client IPs

Server request errors capture the `x-forwarded-for` header by default. To omit it from captured exception events, set `privacy: { clientIp: false }` in `register()` from `@interfere/next/instrumentation`. This option applies to both Node and Edge runtimes and can be combined with `privacy.urls`. Authorization and cookie headers remain excluded.

### URL redaction

The browser SDK and the server adapter always redact URL credentials, and redact the values of built-in sensitive query parameters (such as `token`, `password`, `sig` and `X-Amz-Signature`) by default. Parameter names stay, and redacted values become `REDACTED`. Use `privacy.urls` in browser `init()` and in the server `register()` (Node and Edge) to change this:

```ts
// instrumentation-client.ts
import { init } from "@interfere/next/instrument-client";

void init({ privacy: { urls: { allow: ["page", "sort"] } } });
```

```ts
// instrumentation.ts
import { register } from "@interfere/next/instrumentation";

await register({ privacy: { urls: { deny: ["email"] } } });
```

`{ allow }` redacts every value except the listed names. `{ deny }` adds names to the built-in list. Both keep fragments unless you pass `fragment: true`. `true` redacts every value and removes fragments. `false` keeps query values. Requests, routing and trace propagation keep the original URL. The [`@interfere/react` README](https://www.npmjs.com/package/@interfere/react#url-redaction) lists the built-in names and which records are covered.

On the server, the adapter's processor redacts each span while it ends. In managed mode every processor and exporter on the same provider then receives the redacted span, including Vercel's `"auto"` drain processors and any second exporter; in application mode it covers Interfere's own exception spans. Because sensitive-name redaction is on by default, those exporters now receive `REDACTED` for the built-in names too; pass `privacy: { urls: false }` to keep query values. Providers built on `@opentelemetry/sdk-trace-base` older than 2.3 lack the `onEnding` hook; there the SDK redacts attributes and exception events in `onEnd`, which only reaches processors listed after Interfere's and leaves span names and status messages unchanged.

## Browser span processors

Pass standard OpenTelemetry processors to browser `init()` in `instrumentation-client.ts`:

```ts
import { init } from "@interfere/next/instrument-client";
import type { SpanProcessor } from "@opentelemetry/sdk-trace-base";

const dropTenant: SpanProcessor = {
  onStart() {},
  onEnd(span) {
    delete span.attributes["app.tenant_name"];
  },
  async forceFlush() {},
  async shutdown() {},
};

void init({ tracing: { spanProcessors: [dropTenant] } });
```

These processors run in array order after built-in session and device enrichment and before internal wrapper processors and the Interfere batch exporter. Instrumentation and export remain enabled. Complete mutations to `span.attributes` synchronously in `onEnd`; asynchronous callbacks are not awaited, and `setAttribute` cannot change ended spans. Use `privacy.urls` for query values and fragments.

The SDK owns processor flushing and shutdown. Give each SDK initialization fresh processor instances, and do not share them with another provider. `tracing: false` skips browser tracing setup. These options do not configure server tracing or `withInterfere()`.

## What's Included

- **Error tracking** — automatic capture of server and client errors with rich stack traces
- **Session replay** — full visual playback of user sessions
- **Source maps** — automatic upload during production builds
- **Release tracking** — links builds to Git commits and deployments
- **Server capture** — `captureError()` for manual server-side error reporting

## License

MIT

## Supported runtimes

Node.js 24.x is the supported Node.js release line and is exercised in CI. Browser entry points run in the browser; Node.js is required for server execution and build tooling.

| Integration | Supported versions                           |
| ----------- | -------------------------------------------- |
| Next.js     | `^15.3.0` or `^16.0.0`                       |
| React       | React 19 (`^19.0.0`)                         |
| Vite        | Vite 8.2.4 or later within Vite 8 (`^8.2.4`) |

Next.js adapters accept `^15.3.0 || ^16.0.0`. Next.js 14 and React 18 are not supported.
