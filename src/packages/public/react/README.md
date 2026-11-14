<p align="center">
  <a href="https://interfere.com">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://images.interfere.com/cdn-cgi/imagedelivery/ZQ3TDZdJHXvYNyL5BNulpQ/v2/header/logo-dark/format=auto">
      <img src="https://images.interfere.com/cdn-cgi/imagedelivery/ZQ3TDZdJHXvYNyL5BNulpQ/v2/header/logo-light/format=auto" height="64" alt="Interfere">
    </picture>
  </a>
  <h1 align="center">@interfere/react</h1>
</p>

<p align="center">
  <a href="https://www.npmjs.com/package/@interfere/react"><img src="https://img.shields.io/npm/v/@interfere/react.svg" alt="npm version" /></a>
  <a href="./LICENSE"><img src="https://img.shields.io/npm/l/@interfere/react.svg" alt="License" /></a>
  <a href="https://www.npmjs.com/package/@interfere/react"><img src="https://img.shields.io/npm/dm/@interfere/react.svg" alt="npm downloads" /></a>
</p>

<p align="center">
  Client-side React SDK for <a href="https://interfere.com">Interfere</a> — error tracking, session replay, and analytics.
</p>

<p align="center">
  <a href="https://interfere.com/docs">Documentation</a>
  ·
  <a href="mailto:support@interfere.com">Get Help</a>
</p>

---

## Framework Guides

| Framework | Package                                                                                 | Guide                        |
| --------- | --------------------------------------------------------------------------------------- | ---------------------------- |
| Next.js   | [`@interfere/next`](https://www.npmjs.com/package/@interfere/next)                      | [docs/next.md](docs/next.md) |
| Vite      | [`@interfere/vite`](https://www.npmjs.com/package/@interfere/vite) + `@interfere/react` | [docs/vite.md](docs/vite.md) |

## Installation

```bash
npm install @interfere/react
```

### Installing a fresh release

Release-age policies can block a newly published SDK, including its transitive `@interfere/javascript` and `@interfere/build` packages. A pnpm policy of `minimumReleaseAge: 10080` waits seven days; Bun's `minimumReleaseAge = 1209600` waits fourteen days. Stable publication starts its own clock after RC qualification.

For an approved early preview, retain existing exclusions and add the required Interfere packages to pnpm's `minimumReleaseAgeExclude` (which supports `"@interfere/*"`) or Bun's `install.minimumReleaseAgeExcludes`. Regenerate and commit the lockfile, restore frozen-lockfile enforcement, and verify the normal CI install.

### Prerequisites

- React 19 (`^19.0.0`)

## Quick Start

```tsx
import { InterfereProvider } from "@interfere/react/provider";

function App() {
  return (
    <InterfereProvider>
      <YourApp />
    </InterfereProvider>
  );
}
```

`InterfereProvider` captures render errors by default and rethrows them so your host error boundary keeps control of the fallback. Set `captureRenderErrors={false}` to disable this capture.

## What remote configuration can change

The SDK config response contains `plugins` and `protocolVersion`, plus optional `backendHosts`, `ingestUrl`, `anonymousReplayPolicy`, `logSourcing`, `platform` and `serverTracing` fields. The browser resolver consumes the plugin settings, backend hosts, protocol version, ingest URL and anonymous replay policy; a build with baked plugin settings uses those settings, baked backend hosts and the baked anonymous replay policy instead of fetching runtime config.

The anonymous replay policy decides what replay records before `identify()` succeeds. `always` records every session. `on_error` keeps the last minute or so of replay in memory, starting at a full snapshot taken every 30 seconds, and uploads nothing until the session hits an error: an uncaught error, an unhandled rejection, a captured exception or a failed request. The SDK then uploads the buffered replay and records the rest of the session normally. `never` records nothing while the session is anonymous. A successful `identify()` switches every policy to normal recording; with `never`, recording starts from a fresh full snapshot. `ingestUrl` selects the origin for telemetry uploads, separately from the config API origin.

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

Configure `serviceName: "storefront"` in your framework SDK's browser `init()`, its build plugin (`withInterfere()` for Next.js or `interfere()` for Vite), and server `register()` when used. `InterfereProvider` does not name the service; use the framework setup linked above.

### Environment variables

| Variable                    | Description                                              |
| --------------------------- | -------------------------------------------------------- |
| `INTERFERE_PUBLIC_KEY`      | Public workspace or App key for the Next.js integration. |
| `VITE_INTERFERE_PUBLIC_KEY` | Public workspace or App key for the Vite integration.    |

Your framework build integration supplies the key to the browser SDK.

## Features

- **Error tracking** — automatic capture of uncaught errors and unhandled rejections
- **Session replay** — full visual playback of user sessions
- **Page analytics** — SPA-aware pageviews and UI interaction events
- **Rage click detection** — surface frustrated user behavior
- **User identity** — attach user context to sessions
- **Offline resilient** — events are persisted and delivered in the background

## Hooks

```tsx
import { useInterfere } from "@interfere/react/provider";

function MyComponent() {
  const { identity, session } = useInterfere();

  identity.set({
    company: { tier: "Enterprise" },
    identifier: "usr_123",
    email: "jane@example.com",
    name: "Jane",
    source: { type: "clerk", name: "Clerk" },
  });

  const sessionId = session.getId();
}
```

`company.tier` sets the company’s tier in Interfere; tiers are managed under Settings or the tiers API.

## Session correlation

By default, the SDK writes W3C trace context to permitted fetch and XHR requests only when they carry none, preserves existing caller headers, and records the outgoing trace ID when observable. `tracing.mode` controls OpenTelemetry globals and request recording: `auto` claims available globals and backs off when another OpenTelemetry propagator is registered; `managed` records spans; `application` claims no globals and records request observations. Set `tracing.propagation: "external"` independently when another layer owns trace headers.

`tracing.headers` restricts which headers Interfere may write; `false` disables all SDK header injection. Explicit `propagateTo` entries can restrict an origin further. External propagation always suppresses SDK trace-context injection, even if a header list includes it. The baggage flag independently suppresses SDK baggage injection. Caller and external headers are never removed.

With baggage enabled, instrumented requests carry `interfere.session.id` and the consented device ID to the page origin, exact HTTP(S) origins in `tracing.propagateTo`, and configured backend hosts as exact HTTPS origins. Sibling subdomains, schemes and ports require their own origins. Cross-origin APIs must allow the headers actually sent. Interfere diagnoses suspected CORS failures without retrying application requests.

Explicitly denying analytics consent stops session baggage propagation while retaining trace propagation. `tracing: false` disables SDK instrumentation. Existing application-owned propagators are not replaced.

### Disable baggage propagation

Set `tracing: { propagateTraceBaggage: false }` to stop Interfere from adding baggage while retaining permitted `traceparent` and `tracestate` propagation. Baggage is enabled by default. The flag applies to every origin, including explicit `propagateTo` header lists; existing `headers: false` and header subsets still restrict what can be written. This also disables outgoing baggage injection through Interfere’s managed OpenTelemetry propagator while preserving inbound extraction. It does not remove baggage supplied by your code or another SDK. Browser spans retain their session and consented device attributes. Backend spans can be associated through a shared trace ID when the browser identity-bearing telemetry is available, but no longer receive those identifiers directly through Interfere baggage.

### External trace propagation

Set `tracing: { propagation: "external" }` when another instrumentation layer owns outgoing trace context. Interfere then writes neither `traceparent` nor `tracestate`, including through its managed propagator. Existing application headers remain intact. Baggage retains its independent default-on policy; set `propagateTraceBaggage: false` when the API does not accept it.

`auto` with external propagation resolves to application mode and claims no OpenTelemetry globals. Explicit `managed` mode can still record Interfere spans and claim an available context manager, but does not register a propagator. With default propagation, `auto` checks OpenTelemetry registration ownership only; it does not inspect other SDK globals or guess which private request wrappers will run later.

When external instrumentation privately clones request headers, call the generic helper from its supported request-completion hook:

```ts
import { correlateRequestTrace } from "@interfere/react/request-trace";

correlateRequestTrace(response, sentRequestHeaders);
```

Pass the exact fetch `Response` and the final **request** headers, not response headers or headers reconstructed from a URL. The helper reads W3C trace context, preserves request-time identity and timestamp, and changes neither headers nor callback return values. XHR correlation reads final intercepted request headers directly. No request is replayed and no correlation is guessed from URLs or timing.

A request without valid trace context or without an available completion hook remains unlinked. Delayed callbacks after consent changes or SDK disposal do not emit correlation. Browser telemetry records the outgoing trace ID in `interfere.request.trace_id`, which can differ from an Interfere span's own trace ID. Baggage independently carries identity to the backend; disabling it requires identity-bearing browser telemetry to be retained and indexed for trace-based association.

### Which headers go where

| Origin                                       | Headers                                                      | When                                                                                                                                                                |
| -------------------------------------------- | ------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| The page origin                              | the `tracing.headers` set (all by default)                   | always                                                                                                                                                              |
| A `tracing.propagateTo` origin               | the `tracing.headers` set, or the entry's own `headers` list | always: you asserted it                                                                                                                                             |
| A backend host configured for your workspace | the `tracing.headers` set                                    | on `GET` and `HEAD` from the first request; on other methods once one header-bearing `GET` or `HEAD` to that origin has received a response in this browser session |
| Any other origin                             | none                                                         | never                                                                                                                                                               |

A backend host is discovered from your workspace configuration rather than asserted by your code, so its CORS policy is proved before a `POST` carries headers: an unmodified `Access-Control-Allow-Headers` can never break a first request. The proof is kept in `sessionStorage` and survives reloads. If a header-bearing request to any origin fails at the network layer before that origin has ever answered one, Interfere logs one diagnostic naming `Access-Control-Allow-Headers`, sends no trace headers to that origin for the rest of the browser session and still records the request. The SDK never resends an application request: a failed call stays failed for your code to handle. Add the origin to `propagateTo` to assert its policy, or give it a narrower `headers` list.

## Upgrading from 12.x

Upgrade every `@interfere/*` package together. Browser options that change:

| 12.x                                                            | 13.x                                                                                                                                           |
| --------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| `tracing.apiHosts: ["api.example.com"]`                         | `tracing.propagateTo: ["https://api.example.com"]`; `apiHosts` was removed in 13.0.1.                                                          |
| `tracing.propagateContextUrls`, same-site wildcard propagation  | Removed. Trace headers reach the page origin, `propagateTo` entries and configured backend hosts only.                                         |
| `<InterfereProvider errorBoundary={false}>`                     | `<InterfereProvider captureRenderErrors={false}>`                                                                                              |
| Retry of a failed cross-origin request without trace headers    | Removed. One call is one request; a suspected CORS rejection is logged once per origin and stamped on the request observation.                 |
| Interfere's `traceparent` replaced one another tool wrote first | Written only to requests that carry none, in every tracing mode; a `traceparent` another tool wrote first is kept and recorded on the request. |

The 13.0 release notes carry the complete table, including the server-side options of `@interfere/next` and `@interfere/vite`.

## Filtering errors

Browser `init()` takes four options that decide which exceptions leave the page. They apply to every capture source: uncaught errors, unhandled rejections, render errors, console errors and `capture()`. They do not apply to failed-request exceptions, which `tracing.captureFailedRequests` controls.

```ts
// Vite shown; Next.js imports init from "@interfere/next/instrument-client"
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

## Privacy

Pass `privacy` to your framework's browser `init()` to mark elements whose content the SDK never records. The same selectors apply to session replay and to click, submit and rage-click records, from the first capture:

```ts
const options: KernelOptions = {
  privacy: {
    blockSelector: "[data-sensitive]",
    maskTextSelector: "[data-mask-text]",
  },
};
```

The `interfere-block` and `interfere-mask` classes always apply. A selector matches the target element or any ancestor. The selectors apply even when replay is disabled.

`replay.blockSelector` and `replay.maskTextSelector` on the provider apply to replay only.

### Interaction records

Click, submit and rage-click records keep what the person did and which control they used, but never what a private element shows or holds. A click on a private account-number field records that the "Account number" field was clicked, not the number. Field values are never recorded.

| Target                                        | Recorded                                                                                                                                     |
| --------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- |
| Blocked (`interfere-block`, `blockSelector`)  | Event type, timing, tag name and field label. `ui.target.privacy` is `blocked`.                                                              |
| Masked (`interfere-mask`, `maskTextSelector`) | Event type, timing, tag name, `role` and field label. `ui.target.privacy` is `masked`.                                                       |
| Public                                        | Tag, `id`, `name`, `role`, `aria-label`, field label and up to 120 characters of text. Text inside blocked or masked descendants is skipped. |

The field label (`ui.target.label`) comes from developer-authored sources only:

1. A `data-interfere-label` attribute on the target or an ancestor, such as `<div data-sensitive data-interfere-label="Security code">`. Use this when the visible label sits inside the private region.
2. Otherwise, the text of the field's `<label>` elements and `aria-labelledby` targets, when those labels are not themselves blocked or masked.

A private element's own `aria-label`, `placeholder`, `title`, text and value are never recorded, because applications often put the current value there. Form fields (`input`, `select`, `textarea`) never contribute text, even when public.

For blocked and masked targets the SDK also replaces OpenTelemetry's id-based `target_xpath` with a positional path. Target URLs (`href`) are never recorded. An invalid selector blocks every interaction target and logs a warning.

To keep navigation, document-load and resource timing without click and submit spans, pass `pageEvents: { interactions: false }`. `pageEvents: false` still turns off all page events, and `rageClick: false` turns off rage clicks. Denying analytics consent stops page events and rage clicks.

## URL redaction

The SDK redacts URLs before they leave the browser. Parameter names stay, redacted values become `REDACTED`, and credentials in a URL (`https://user:pass@host`) always become `REDACTED:REDACTED`. Configure query and fragment redaction with `privacy.urls`:

| `privacy.urls`        | Query values redacted                         | Fragment |
| --------------------- | --------------------------------------------- | -------- |
| Omitted (default)     | Built-in sensitive names only                 | Kept     |
| `{ deny: ["email"] }` | Built-in sensitive names and the listed names | Kept     |
| `{ allow: ["page"] }` | Every name except the listed names            | Kept     |
| `true`                | Every name                                    | Removed  |
| `false`               | None                                          | Kept     |

```ts
const options: KernelOptions = {
  privacy: { urls: { allow: ["page", "sort"], fragment: true } },
};
```

`fragment: true` removes fragments, including hash-router paths. The built-in sensitive names follow the OpenTelemetry URL conventions and common credential names: `AWSAccessKeyId`, `sig`, `code` and `key`, plus any name that contains `token`, `secret`, `password`, `passwd`, `auth`, `credential`, `session`, `signature`, `api_key` or `apikey`. Names match case-insensitively after URL decoding. A name in `deny` is always redacted; a name in `allow` keeps its value, even when it is a built-in sensitive name.

A parameter without `=` (such as `?eyJhbGci…`) is redacted whole when its name is redacted, because a bare value can be a token. The SDK splits each value at the first `?` and `#` without parsing it, so relative paths and malformed values follow the same rule. A query inside a hash route belongs to the fragment and is kept with it. Requests, routing and trace propagation still use the original URL. Path segments are not redacted, so a token in a path such as `/share/abc123` is still recorded.

| Record                                                                                                                                                                                | Covered                                                 |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------- |
| `url.full`, `url.path`, `url.original`, `http.url`, `http.target`, `http.referrer` and `referer` header attributes on SDK spans and span events, and span names such as `GET /a?id=1` | Yes                                                     |
| `url.query` and `url.fragment` attributes                                                                                                                                             | Values redacted; fragment removed with `fragment: true` |
| Navigation logs, including the initial page load                                                                                                                                      | Yes                                                     |
| Absolute URLs in exception messages, the cause chain and the span status, including SDK messages for failed requests, resource loads and CSP violations                               | Yes                                                     |
| Session replay page URLs and `href`, `src`, `srcset`, `action`, `formaction`, `poster`, `cite` and `data` attributes                                                                  | Yes                                                     |
| Spans on your own OpenTelemetry provider, URLs in stack frames, relative paths in exception messages, inlined CSS in replay and console logs                                          | No                                                      |

Replay plays images and links back from their recorded URLs, so a redacted signed URL (for example `?X-Amz-Signature=REDACTED`) no longer loads in the player.

The SDK redacts each span while it ends, so `onEnd` in `tracing.spanProcessors` and the Interfere exporter receive redacted values. `onStart` still sees the URL the instrumentation set at span creation. The browser provider is private to the SDK, so other OpenTelemetry exporters on the page are unaffected.

## Browser span processors

Framework browser `init()` options accept `tracing.spanProcessors`, an array of standard OpenTelemetry `SpanProcessor` instances. For example, these options remove an application attribute before export:

```ts
import type { KernelOptions } from "@interfere/react/internal/kernel";
import type { SpanProcessor } from "@opentelemetry/sdk-trace-base";

const dropTenant: SpanProcessor = {
  onStart() {},
  onEnd(span) {
    delete span.attributes["app.tenant_name"];
  },
  async forceFlush() {},
  async shutdown() {},
};

const options: KernelOptions = {
  tracing: { spanProcessors: [dropTenant] },
};
```

Pass `options` to your framework's browser `init()`. Processors are additive. Built-in session and device enrichment runs first, followed by your processors in array order, internal wrapper processors, and the Interfere batch exporter. Default instrumentation and export remain enabled.

Finish attribute mutations synchronously in `onEnd`, before it returns. OpenTelemetry does not await asynchronous work in `onStart` or `onEnd`. Ended spans cannot be changed through `setAttribute`; edit `span.attributes` as above. Use `privacy.urls` for query values and fragments.

The SDK calls each processor's `forceFlush` during flush and `shutdown` during disposal. Use a fresh processor instance for each SDK initialization and do not share it with another provider. `tracing: false` skips tracing setup, including custom processors.

## Trace export and context

The SDK batches ended spans and sends them every 60 seconds, or sooner when 200 spans are waiting. When the page is hidden or unloaded, it sends everything still queued at once with a keepalive request. Spans still queued when a tab is killed without a hide event, such as a crash, are lost. Logs are sent every 5 seconds.

The SDK tracks the active span without zone.js. A click or submit span ends when its synchronous event handler returns. Spans started after an `await` in that handler, including fetch and XHR requests, are children of the page span instead of the click span. If your app loads zone.js itself, as Angular does, the SDK does not use it.

`span(name, fn)` follows the same rule. The span's duration covers `fn` and, when `fn` returns a promise, all of its awaited work, and a throw or rejection is recorded on the span. The span is the active context only until the first `await` in `fn`, so spans and requests started after it are not its children. To parent them, capture the span and run the work in its context:

```ts
import { context, trace } from "@opentelemetry/api";
import { span } from "@interfere/react/api";

await span("checkout", async (checkout) => {
  const cart = await loadCart();

  await context.with(trace.setSpan(context.active(), checkout), () =>
    submitOrder(cart),
  );
});
```

Pass `{ root: true }` as the third argument to start a span in a new trace.

## Documentation

Visit [interfere.com/docs](https://interfere.com/docs) for the full documentation.

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
