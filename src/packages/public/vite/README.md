<p align="center">
  <a href="https://interfere.com">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://images.interfere.com/cdn-cgi/imagedelivery/ZQ3TDZdJHXvYNyL5BNulpQ/v2/header/logo-dark/format=auto">
      <img src="https://images.interfere.com/cdn-cgi/imagedelivery/ZQ3TDZdJHXvYNyL5BNulpQ/v2/header/logo-light/format=auto" height="64" alt="Interfere">
    </picture>
  </a>
  <h1 align="center">@interfere/vite</h1>
</p>

<p align="center">
  <a href="https://www.npmjs.com/package/@interfere/vite"><img src="https://img.shields.io/npm/v/@interfere/vite.svg" alt="npm version" /></a>
  <a href="./LICENSE"><img src="https://img.shields.io/npm/l/@interfere/vite.svg" alt="License" /></a>
  <a href="https://www.npmjs.com/package/@interfere/vite"><img src="https://img.shields.io/npm/dm/@interfere/vite.svg" alt="npm downloads" /></a>
</p>

<p align="center">
  Vite SDK for <a href="https://interfere.com">Interfere</a> — error tracking, session replay, and analytics for React apps shipped with Vite or TanStack Start.
</p>

<p align="center">
  <a href="https://interfere.com/docs">Documentation</a>
  ·
  <a href="mailto:support@interfere.com">Get Help</a>
</p>
---

## Getting Started

### Prerequisites

- Vite 8.2.4 or later within Vite 8 (`^8.2.4`)
- React 19 (`^19.0.0`)

### Installation

```bash
npm install @interfere/vite
```

### Installing a fresh release

Release-age policies can block a newly published SDK, including its transitive `@interfere/javascript` and `@interfere/build` packages. A pnpm policy of `minimumReleaseAge: 10080` waits seven days; Bun's `minimumReleaseAge = 1209600` waits fourteen days. Stable publication starts its own clock after RC qualification.

For an approved early preview, retain existing exclusions and add the required Interfere packages to pnpm's `minimumReleaseAgeExclude` (which supports `"@interfere/*"`) or Bun's `install.minimumReleaseAgeExcludes`. Regenerate and commit the lockfile, restore frozen-lockfile enforcement, and verify the normal CI install.

### Quick Start (vanilla Vite SPA)

**1. Add the Vite plugin**

```ts
// vite.config.ts
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import { interfere } from "@interfere/vite/plugin";

export default defineConfig({
  plugins: [react(), interfere()],
});
```

**2. Initialize the SDK**

```ts
// src/main.tsx
import { init } from "@interfere/vite/init";

import { createRoot } from "react-dom/client";
import { App } from "./app";

init();

createRoot(document.getElementById("root")!).render(<App />);
```

**3. Add the provider**

```tsx
// src/app.tsx
import { InterfereProvider } from "@interfere/vite/provider";

export function App() {
  return (
    <InterfereProvider>
      <YourApp />
    </InterfereProvider>
  );
}
```

That's it. The provider auto-resolves the kernel created by `init()`; you don't have to wire `useSyncExternalStore` yourself.

`InterfereProvider` captures render errors by default and rethrows them so your host error boundary keeps control of the fallback. Set `captureRenderErrors={false}` to disable this capture. The deprecated `errorBoundary` prop was removed in 13.0.1; use `captureRenderErrors` instead.

### Quick Start (TanStack Start + Nitro)

Browser events post directly to Interfere; the server's own telemetry is registered via `@interfere/vite/server`. Call `init()` from the file that boots your router, call `register()` from a server entry, add the request and function middleware in `src/start.ts`, and put `<InterfereProvider>` in `__root.tsx`:

```ts
// src/router.tsx
import { init } from "@interfere/vite/init";
import { createRouter } from "@tanstack/react-router";
import { routeTree } from "./routeTree.gen";

if (typeof window !== "undefined") {
  init();
}

export const getRouter = () =>
  createRouter({ routeTree, scrollRestoration: true });
```

```ts
// src/server.ts
import { register } from "@interfere/vite/server";
import handler, { createServerEntry } from "@tanstack/react-start/server-entry";

void register();

export default createServerEntry({
  fetch: async (request) => handler.fetch(request),
});
```

```ts
// src/start.ts
import {
  interfereFunctionMiddleware,
  interfereRequestMiddleware,
} from "@interfere/vite/tanstack-start";
import { createStart } from "@tanstack/react-start";

export const startInstance = createStart(() => ({
  functionMiddleware: [interfereFunctionMiddleware],
  requestMiddleware: [interfereRequestMiddleware],
}));
```

`src/router.tsx` runs in the browser too, so it must not import `@interfere/vite/server`: that would put the Node SDK into your client bundle. Start runs `src/server.ts` only on the server. `interfereRequestMiddleware` opens a server span for each request. `interfereFunctionMiddleware` captures errors thrown by `createServerFn` handlers. Start turns those errors into a response for the client, so the request middleware never sees them. Redirects and `notFound()` are not reported as errors. Call `traceMeta()` from `@interfere/vite/tanstack-start` in your root route's `head()` to link each page to its server request.

```tsx
// src/routes/__root.tsx
import { InterfereProvider } from "@interfere/vite/provider";
import { Outlet } from "@tanstack/react-router";

function RootComponent() {
  return (
    <InterfereProvider>
      <Outlet />
    </InterfereProvider>
  );
}
```

### TanStack Start on Cloudflare Workers

Workers allow network requests only while a request is being handled, so call `register()` inside your Worker's `fetch` handler rather than at module scope. It initializes once per isolate; later calls return straight away. Enable `nodejs_compat` with a `compatibility_date` of 2025-08-15 or later, and set `VITE_INTERFERE_PUBLIC_KEY` as a Worker variable so the server reads it from `process.env`:

```ts
// src/worker.ts
import { register } from "@interfere/vite/server";
import handler from "@tanstack/react-start/server-entry";

export default {
  async fetch(request) {
    await register({ serviceName: "storefront" });

    return handler.fetch(request);
  },
} satisfies ExportedHandler;
```

Keep the request and function middleware in `src/start.ts` as above. Each server span continues the browser's `traceparent`, so a page's browser and server spans share one trace. Cloudflare's own Workers traces cannot do this, because they start a new trace for every request. When a request span ends, its spans are exported within that request's `waitUntil`. You don't need to call `flush()` yourself.

The browser guard matters for TanStack Start because prerender can evaluate the router import graph on the server, while the Interfere kernel uses browser-only APIs. For an ad-blocker-resistant setup, set `interfere({ apiHost: "https://in.yourapp.com" })` — a subdomain of your own site that resolves to Interfere via a DNS record.

Render `<TraceMeta />` from `@interfere/vite/server` in the document `<head>` so the browser SDK links each page to its server render. It emits `traceparent`, `tracestate` and `baggage` meta tags for the active request span, including the freshness stamp the browser requires, and renders nothing when no span is active.

| Server `register()` option | Default       | Behavior                                                                                                                                                                                                                                                      |
| -------------------------- | ------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `tracing`                  | `"auto"`      | Adopts an already registered tracer provider; otherwise manages tracing. Set `"managed"` or `"application"` to override in code.                                                                                                                              |
| `errors`                   | `"interfere"` | Delivers captured errors through Interfere, linked to the active host span. `"host"` keeps a recording host span as the only copy; without one, Interfere still delivers the error.                                                                           |
| `consoleBridge`            | `true`        | `false` disables console logs; `{ level: "warn" }` bridges warnings and errors only. Levels are `debug`, `info` (including `log`), `warn`, and `error`. Omitted `level` means `debug`; lower-level methods remain untouched. Log drain mode skips the bridge. |

Request middleware failures capture the `x-forwarded-for` client IP header by default. Set `register({ privacy: { clientIp: false } })` to omit it from captured exception events. For manual `captureError()` calls, pass the request's `headers` alongside its `method` and `path` to include the same header. Authorization and cookie headers are not captured.

### When your application already owns OpenTelemetry

Register your provider before calling Interfere's `register()`. The default `tracing: "auto"` adopts it and logs one diagnostic message at debug level. Use `tracing: "application"` to select application ownership explicitly:

```ts
// server entry
import { register } from "@interfere/vite/server";

await register({ tracing: "application" });
```

`register({ tracing: "application" })` writes no OpenTelemetry global and installs no instrumentation. By default, captured errors reach Interfere through its private provider as exception spans linked to your active host span, even when your provider exports nowhere Interfere reads. Recording host spans also keep the exception event and unhandled-error status. The host event carries a delivery marker so ingest keeps one copy when both providers export to Interfere. Set `errors: "host"` when your host provider already delivers errors to Interfere and should be the only copy; errors without a recording host span still ship through Interfere. The `console.*` bridge emits into the logger provider you registered, or into a private Interfere provider when you registered none. `TraceMeta` and `traceMeta()` emit nothing in this mode. To send ordinary host traces as well as captured errors, configure your provider with the standard exporter variables, or use a collector that fans out:

```bash
OTEL_EXPORTER_OTLP_ENDPOINT="https://api.interfere.com/drain"
OTEL_EXPORTER_OTLP_PROTOCOL="http/protobuf"
OTEL_EXPORTER_OTLP_HEADERS="Authorization=Bearer%20${INTERFERE_API_KEY}"
```

The key needs the **Send telemetry** permission. Tracing defaults to automatic detection; code is the only place to override it, with no dashboard setting. Configure browser ownership separately with `init({ tracing: { mode: "application" } })` when your browser already has a tracing owner.

With no registered tracer provider, the default `tracing: "auto"` registers Interfere's tracer, logger and meter providers, propagator chain and outbound `fetch` instrumentation. Explicit `tracing: "managed"` selects this behavior too. If another library registered a tracer provider first, explicit managed `register()` logs one warning naming `tracing: "application"`, keeps capturing errors through its own isolated provider and writes no other OpenTelemetry global.

### Browser trace headers

The browser SDK writes `traceparent`, `tracestate` and `baggage` to the page origin and to the exact origins in `init({ tracing: { propagateTo } })` on every request, and to the backend hosts configured for your workspace only after Interfere's own credentialed `HEAD` check to that host passes CORS in this browser session, with only the headers that passed. `tracing.headers` restricts the set or, as `false`, turns header writes off everywhere; a `propagateTo` entry `{ origin, headers }` names exactly the headers one origin receives. Cross-origin APIs must allow `traceparent, tracestate, baggage` in `Access-Control-Allow-Headers`. The SDK never resends an application request.

## First-Time Setup

The SDK splits its credentials by concern: a **public key** travels with the browser bundle (it's not a secret), and a **secret key** stays server-side (CI / GSM / `.env.local`).

1. **Install** `@interfere/vite` and add the plugin (above).
2. **Public key** — copy the dashboard's regional public key (`interfere_public_us_*` or `interfere_public_eu_*`) into `.env.local` as `VITE_INTERFERE_PUBLIC_KEY=...`. Public by design; safe to commit to a public repo.
3. **Secret key** — create a workspace secret key with the **Source maps** (`release:write`) permission, plus **Send telemetry** (`org:telemetry:write`) for server telemetry, and put the regional value (`interfere_secret_us_*` or `interfere_secret_eu_*`) in `.env.local` as `INTERFERE_API_KEY=...`. One key publishes releases for every service in the workspace; each release belongs to the service named by `serviceName`. **Server-side; never commit.**
4. **First build** — run `bun run build`. The plugin's `closeBundle` hook publishes release metadata and preflight-confirms the release. After this the commit's slug is accepted at ingest.
5. **Dev cycle** — `bun run dev` then exercises the bundle in your browser. Events flow with the commit's preflight-confirmed slug (the build did the work). New commit? Re-run `bun run build` so the new commit's slug is preflighted.

For **Vercel preview / production**, set `INTERFERE_API_KEY` in your `.env.local` or via GSM (or directly as a Vercel env var). Each preview build runs the same `bun run build` flow in CI; no local-vs-preview divergence.

## What remote configuration can change

The SDK config response contains `plugins` and `protocolVersion`, plus optional `backendHosts`, `ingestUrl`, `anonymousReplayPolicy`, `logSourcing`, `platform` and `serverTracing` fields. The browser resolver consumes the plugin settings, backend hosts, protocol version and ingest URL; a build with baked plugin settings uses those settings and baked backend hosts instead of fetching runtime config. `ingestUrl` selects the origin for telemetry uploads, separately from the config API origin.

Remote plugin settings can disable shipped plugins; they cannot override local disables or consent. Lazy plugins load through dynamic imports of code-split chunks shipped in the installed package, not executable code fetched from an Interfere CDN; core error capture is bundled eagerly. Remote configuration cannot enable code the installed version does not ship.

Remote `backendHosts` become discovered HTTPS origins. They receive no trace header on any request until Interfere's own credentialed `HEAD` check to that origin passes CORS in this browser session, and then only the headers that passed (`baggage` is dropped when only the check without it passes). Explicit `tracing.propagateTo` origins and their header lists are declared in application code and take precedence over discovered hosts. `tracing.headers: false` disables SDK header writes everywhere. Remote configuration cannot override either setting.

## Services and workspace keys

Interfere organizes telemetry by service. `serviceName`, with an optional `serviceNamespace`, names the service, and every request carries it: SDK requests send `x-interfere-service` and `x-interfere-service-namespace`, and OTLP requests use the `service.name` and `service.namespace` resource attributes. Interfere records a service the first time it sees one, within the workspace's service limit.

Every workspace receives a **Default** workspace public key. Use one workspace key for all of the workspace's services; additional keys can be created or rotated in **Settings → Developer → Public keys**. Servers, CI and collectors use a workspace secret key with **Send telemetry** (`org:telemetry:write`) and name the service the same way.

Apps are being folded into services, one website service per App. Until that finishes, each service also resolves to an App, which holds its website settings:

1. An existing service-to-App association wins.
2. Otherwise, an App whose name or slug matches the service is used.
3. Otherwise, the request goes to the workspace's **Default app** (choose it in **Settings → Developer → Public keys**). Without an eligible default, the request is rejected. A missing or deleted default falls back to the oldest eligible App. A request without a service identity also goes to the default App.

Interfere never creates an App from telemetry. An App marked `explicit_only` is excluded from name matching and the default fallback; target it through an existing service association or its App key. App keys (formerly surface keys) are deprecated but still accepted, with no removal date announced. An App key stays bound to its App and skips this resolution.

Keep `serviceName` stable: renaming an App's dashboard title does not change an existing service association. Open **Services** to find the service linked to an App before migrating keys.

Set the same name in `interfere({ serviceName: "storefront" })`, browser `init({ serviceName: "storefront" })`, and server `register({ serviceName: "storefront" })`. Build identity uses the explicit option, then `OTEL_SERVICE_NAME`, then the package name without its scope. Keep an application-owned OpenTelemetry provider aligned too.

## Environment Variables

| Variable                    | Required              | Description                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
| --------------------------- | --------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `VITE_INTERFERE_PUBLIC_KEY` | Yes                   | Public workspace or App key in the `interfere_public_us_*` or `interfere_public_eu_*` format. Stamped into the page by the plugin at build time. A production build without it logs a warning and the browser sends no events.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `INTERFERE_API_KEY`         | Yes for deploy builds | Workspace secret key in the `interfere_secret_us_*` or `interfere_secret_eu_*` format, with the **Source maps** (`release:write`) permission. Used by the plugin at build time to register the release and upload source maps for the App resolved from `VITE_INTERFERE_PUBLIC_KEY` and `serviceName`, and by the server runtime (`register()`, request middleware) to send server telemetry, which needs the **Send telemetry** (`org:telemetry:write`) permission. Without it the server sends with `VITE_INTERFERE_PUBLIC_KEY`. The plugin mirrors it from `.env` files into the dev server's process. A production build without it logs a warning and skips the release. A key with the wrong prefix fails the build. If Interfere rejects the key, the build logs an error and continues without a release. |
| `INTERFERE_API_URL`         | No                    | Override the collector URL the build pipeline targets. Defaults to `https://api.interfere.com`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |

## Plugin Options

```ts
interfere({
  commitSha: "abcd0000abcd0000abcd0000abcd0000abcd0000",
});
```

| Option         | Default                            | Description                                                                                                                                                                                                                                                                                                                                          |
| -------------- | ---------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `apiHost`      | —                                  | The endpoint the browser SDK posts to. Defaults to Interfere's endpoint. Set it to a subdomain of your own site (e.g. `"https://in.yourapp.com"`) that resolves to Interfere via a DNS record to survive ad-blockers, or to a regional endpoint. Add a CNAME pointing at the target shown in your dashboard; Interfere provisions TLS automatically. |
| `commitSha`    | `git rev-parse HEAD`               | Commit SHA used to derive the deterministic release slug.                                                                                                                                                                                                                                                                                            |
| `sourceMaps`   | `{ upload: true }`                 | `{ upload: false }` registers the release without uploading source maps. Interfere API failures, including a rejected secret key, timeouts, network failures and HTTP error responses, never fail the build: they are logged, local source maps are deleted and the build finishes.                                                                  |
| `disable`      | `false`                            | Turns Interfere off for this build: no release, no source-map upload, no build checks, and the browser and server SDKs send no events.                                                                                                                                                                                                               |
| `serviceName`  | `OTEL_SERVICE_NAME` → package name | Build default for browser and server OTel `service.name`, resolved by the same shared policy as Next.js. Uses the unscoped package name when no override is set. Explicit runtime service names and server runtime `OTEL_SERVICE_NAME` take precedence over the build default.                                                                       |
| `buildReceipt` | disabled                           | Private JSON path inside the project and outside public/build output directories. Records consumed SDK config and fetch status, original release/build identity, source publication status when available, and hashes of native Vite output files. Keep this artifact private; missing or fallback evidence cannot establish safe deployment reuse.  |

Build logs report release creation, signing, session lookup, upload, finalization, receipt capture and cleanup in milliseconds, plus transferred bytes. Finalization confirms durable acceptance. When Vite builds several environments, the plugin uploads once after all of them finish. It does not replace runtime stack/source canaries or continually rehash every previously published map.

Ingest requests never carry cookies. `apiHost` posts to your own site's subdomain, surviving ad-blockers with no server-side component.

### Service identity

When release preflight resolves an App whose slug differs from the inferred service name, the build warns once and continues. Set `serviceName` explicitly in `interfere()`, browser `init()` and server `register()` to preserve your existing name; an explicit build option or `OTEL_SERVICE_NAME` suppresses the warning. A workspace key with no resolved App produces no name warning.

## Migrating From 0.x

Upgrade all `@interfere/*` SDK packages together. Keep the env var names, but replace old `int_pub_*` / `interfere_pk_*` values with the dashboard's regional public key, and replace old `ak_*` / `interfere_ak_*` build keys with the regional Interfere secret key.

Secret keys that were created for one app keep working. New secret keys belong to the workspace, so set `VITE_INTERFERE_PUBLIC_KEY` on the build machine as well.

## Identity Management

Link sessions to your authenticated users with `identity.set()`:

```tsx
import { useInterfere } from "@interfere/vite/provider";

function useInterfereIdentity() {
  const { identity } = useInterfere();
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

## Filtering errors

Browser `init()` takes four options that decide which exceptions leave the page. They apply to every capture source: uncaught errors, unhandled rejections, render errors, console errors and `capture()`. They do not apply to failed-request exceptions, which `tracing.captureFailedRequests` controls.

```ts
// src/main.tsx
import { init } from "@interfere/vite/init";

init({
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

## Consent

Interfere captures everything by default. If your app must collect consent before analytics, set one option:

```ts
void init({ consent: "required" });
```

Until the visitor grants consent, Interfere:

- captures errors, request spans and timings, and links each request to your backend with a page-scoped id in `baggage`;
- reads and writes nothing on the device: no cookies, no `localStorage` or `sessionStorage`, no device id and no fingerprint;
- records no page views, clicks, rage clicks, console logs or session replay.

The page-scoped id lives in memory and changes on every page load. When consent is granted, Interfere starts the persisted session and records a `session.start` event whose `session.previous_id` is the page id, so requests made before consent stay linked to the session.

Record the visitor's decision with `consent.set`:

```ts
import { consent } from "@interfere/vite/init";

consent.set(true); // analytics and replay
consent.set({ replay: false }); // analytics without replay
consent.set(false); // deny or revoke; erases Interfere's device id and stored session
consent.get(); // { analytics: boolean, replay: boolean }
```

Calls made before `init()` finishes are applied when it does. In React, `useInterfere().consent` is the same object. If your app may send nothing before consent, not even errors, call `init()` only once consent is granted.

| Category    | Covers                                                                      | Before consent |
| ----------- | --------------------------------------------------------------------------- | -------------- |
| `necessary` | Errors, request spans and timings, with a page-scoped id                    | Captured       |
| `analytics` | Persisted session, device id, page views, clicks, rage clicks, browser logs | Withheld       |
| `replay`    | Session replay                                                              | Withheld       |

### Connecting your consent manager

Each recipe calls `consent.set` whenever the manager's decision changes. Map the category your banner uses for analytics or statistics.

**c15t** (inside `ConsentManagerProvider`):

```tsx
import { useConsentManager } from "@c15t/react";
import { useEffect } from "react";

export function InterfereConsent() {
  const { has } = useConsentManager();
  const analytics = has("measurement");
  const replay = has("experience");

  useEffect(() => {
    consent.set(analytics ? { replay } : false);
  }, [analytics, replay]);

  return null;
}
```

Without React, subscribe to c15t's store: `consentStore.subscribe((state) => consent.set(state.has("measurement")))`.

**OneTrust** (`C0002` is OneTrust's default Performance group; use yours):

```ts
const previous = window.OptanonWrapper;

window.OptanonWrapper = () => {
  previous?.();
  consent.set(window.OnetrustActiveGroups?.includes(",C0002,") ?? false);
};
```

**Cookiebot**:

```ts
const sync = () => consent.set(window.Cookiebot?.consent?.statistics === true);

window.addEventListener("CookiebotOnConsentReady", sync);
```

**IAB TCF v2.2** (any TCF CMP; Purpose 1 stores data on the device and Purpose 8 measures content performance):

```ts
window.__tcfapi?.("addEventListener", 2, (data, ok) => {
  if (ok && data.eventStatus !== "cmpuishown") {
    consent.set(
      data.gdprApplies === false ||
        (data.purpose.consents[1] === true &&
          data.purpose.consents[8] === true),
    );
  }
});
```

**Google Consent Mode** is set by your consent manager; connect Interfere to that manager's event as above.

## What's Included

- **Build metadata injection** — automatic Git SHA-derived release slug
- **Error tracking** — automatic capture of uncaught errors and unhandled rejections
- **Session replay** — full visual playback of user sessions
- **Page analytics** — SPA-aware pageviews and UI interaction events
- **Rage click detection** — surface frustrated user behavior
- **Consent gating** — fine-grained control over which features activate

## License

MIT

## Supported runtimes

Node.js 24.x is the supported Node.js release line and is exercised in CI. Browser entry points run in the browser. `@interfere/vite/server` runs on Node.js, and on Cloudflare Workers with `nodejs_compat` (compatibility date 2025-08-15 or later). Build tooling requires Node.js.

| Integration | Supported versions                           |
| ----------- | -------------------------------------------- |
| Next.js     | `^15.3.0` or `^16.0.0`                       |
| React       | React 19 (`^19.0.0`)                         |
| Vite        | Vite 8.2.4 or later within Vite 8 (`^8.2.4`) |

Next.js adapters accept `^15.3.0 || ^16.0.0`. Next.js 14 and React 18 are not supported.

Modules are safe to tree shake (`sideEffects: false`). Call initialization, build, or test helpers explicitly when their behavior is needed.
