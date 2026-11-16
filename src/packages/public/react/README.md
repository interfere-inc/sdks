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
  <a href="https://interfere.com/docs">Documentation</a>
  ·
  <a href="mailto:support@interfere.com">Get help</a>
</p>

---

The browser SDK for [Interfere](https://interfere.com) in React apps. It captures errors, including render errors, and records session replays, page views, rage clicks and the requests your pages make. It joins them into one trace per user action, so you can follow a bug from the click to the server error behind it.

You don't install this package on its own. The framework SDKs include it and set it up at build time:

| Your app                | Install                                                            | Guide                                                   |
| ----------------------- | ------------------------------------------------------------------ | ------------------------------------------------------- |
| Next.js                 | [`@interfere/next`](https://www.npmjs.com/package/@interfere/next) | [Next.js guide](https://interfere.com/docs/sdk/next-js) |
| Vite, or TanStack Start | [`@interfere/vite`](https://www.npmjs.com/package/@interfere/vite) | [Vite guide](https://interfere.com/docs/sdk/vite)       |

This README covers what they share: the browser options, privacy, consent and the React API.

## Requirements

- React 19

## Set up with Vite

```bash
npm install @interfere/vite
```

Set `VITE_INTERFERE_PUBLIC_KEY` to your workspace public key, from **Settings → Developer** in the Interfere dashboard. Then:

**1. Add the Vite plugin.**

```ts
// vite.config.ts
import { interfere } from "@interfere/vite/plugin";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [react(), interfere()],
});
```

**2. Start the SDK before rendering.**

```tsx
// src/main.tsx
import { init } from "@interfere/vite/init";
import { createRoot } from "react-dom/client";
import { App } from "./app";

void init();

createRoot(document.getElementById("root")!).render(<App />);
```

**3. Add the provider.**

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

The provider reports render errors and then rethrows them, so your own error boundaries still choose what to show. Pass `captureRenderErrors={false}` to turn this off.

For Next.js, follow the [`@interfere/next` README](https://www.npmjs.com/package/@interfere/next). Options go to `init()` from `@interfere/next/instrument-client` instead.

## Use it in components

`useInterfere()` works inside the provider. It returns `identity`, `consent` and `session`.

```tsx
import { useInterfere } from "@interfere/vite/provider";
import { useEffect } from "react";

export function IdentifyUser({ user }: { user: User | null }) {
  const { identity } = useInterfere();

  useEffect(() => {
    if (user) {
      void identity.set({
        identifier: user.id,
        email: user.email,
        name: user.name,
        source: { type: "clerk", name: "Clerk" },
      });
    } else {
      identity.clear();
    }
  }, [identity, user]);

  return null;
}
```

| `identity.set()` field | Required | Notes                                                                                     |
| ---------------------- | -------- | ----------------------------------------------------------------------------------------- |
| `identifier`           | Yes      | Your user id, not an email. A Clerk id must start with `user_`.                           |
| `source`               | Yes      | `{ type: "clerk", name: "Clerk" }`, `"auth0"`, `"workos"`, or `{ type: "custom", name }`. |
| `name`, `email`        | No       |                                                                                           |
| `avatar`               | No       | An image URL.                                                                             |
| `company`              | No       | `{ tier: "Enterprise" }` sets the company's tier in Interfere.                            |
| `traits`               | No       | Any other key-value data.                                                                 |

`identity.clear()` forgets the user and starts a new session. Call it on sign-out. `session.getId()` returns the current session id; `useSession()` is a shortcut.

### Report errors and time work

```ts
import { capture, span } from "@interfere/vite/init"; // or "@interfere/next/instrument-client"

capture(error);

await span("checkout", async () => {
  await submitOrder(cart);
});
```

`capture()` reports an error you handled. `span()` times a piece of work and records an error if it throws. Its timing covers everything `fn` awaits. But requests that start after its first `await` belong to the page, not to the span. To make them children of the span, run them in its context:

```ts
import { context, trace } from "@opentelemetry/api";

await span("checkout", async (checkout) => {
  const cart = await loadCart();

  await context.with(trace.setSpan(context.active(), checkout), () =>
    submitOrder(cart),
  );
});
```

Pass `{ root: true }` as the third argument to start a new trace.

## Using Datadog, Sentry or OpenTelemetry too

You don't need to change anything. Interfere checks each request for another tool that writes the `traceparent` header: Datadog RUM (for its `allowedTracingUrls`), Sentry (with `propagateTraceparent`) or another OpenTelemetry propagator. If one is there, Interfere leaves the header to it, so the request carries a single `traceparent` and stays in that tool's trace. Interfere records the request either way. Interfere never removes or replaces headers that your code or another tool set, and it leaves other tools' OpenTelemetry providers on the page alone.

If another tool writes `traceparent` and Interfere doesn't recognise it, such as New Relic or an in-house interceptor, set `tracing: { propagation: "external" }` in `init()`. Interfere then writes no `traceparent` or `tracestate` and still records every request.

If another tool copies request headers before sending them, Interfere can't see the `traceparent` that went out. Report it from that tool's request-completion hook:

```ts
import { correlateRequestTrace } from "@interfere/react/request-trace";

correlateRequestTrace(response, sentRequestHeaders);
```

Pass the fetch `Response` and the request headers that were actually sent.

## Consent

Interfere always captures errors. Analytics and session replay can wait for consent:

| Category    | Covers                                                      | Can be denied |
| ----------- | ----------------------------------------------------------- | ------------- |
| `necessary` | Errors                                                      | No            |
| `analytics` | Page views, clicks, rage clicks, device id and browser logs | Yes           |
| `replay`    | Session replay                                              | Yes           |

Without a consent setting, every category is allowed. To start with analytics and replay denied, pass `consent: false` to `init()`. Then keep the provider in step with your consent banner:

```tsx
const { has } = useConsentManager(); // your consent library

<InterfereProvider
  consent={{ analytics: has("measurement"), replay: has("experience") }}
>
  <YourApp />
</InterfereProvider>;
```

`true` or `false` sets both categories. An object sets each one, and a category you leave out is allowed. From a component, `consent.set()` and `consent.get()` do the same. While analytics is denied, the session id is not sent to your APIs. Consent doesn't turn off error capture or request tracing.

## Privacy

### Private elements

Elements with the `interfere-block` or `interfere-mask` class are kept out of replays and out of click, submit and rage-click records. Add your own selectors in `init()`:

```ts
void init({
  privacy: {
    blockSelector: "[data-sensitive]",
    maskTextSelector: "[data-mask-text]",
  },
});
```

A selector matches the element and everything inside it. It applies from the first capture, whether or not replay is on. An invalid selector is dropped with a console warning, and the valid ones still apply.

### Form fields in replays

By default, replays mask the value of password fields and record what people type into every other field. To mask more, pass replay options to the provider:

```tsx
<InterfereProvider plugins={{ replay: { maskAllInputs: true } }}>
```

| Replay option                       | What it does                                                                                                             |
| ----------------------------------- | ------------------------------------------------------------------------------------------------------------------------ |
| `maskAllInputs`                     | `true` masks the value of every field.                                                                                   |
| `maskInputOptions`                  | Masks fields by type, such as `{ email: true, tel: true }`. It replaces the default, so add `password: true` to keep it. |
| `maskInputFn`, `maskTextFn`         | Your own function for masked values and masked text.                                                                     |
| `blockClass`, `blockSelector`       | Blocks more elements in replays only, alongside `interfere-block` and `privacy.blockSelector`.                           |
| `maskTextClass`, `maskTextSelector` | Masks more text in replays only, alongside `interfere-mask` and `privacy.maskTextSelector`.                              |
| `ignoreClass`, `ignoreSelector`     | Fields whose input replays don't record, alongside `interfere-ignore`.                                                   |
| `recordCanvas`                      | `true` records `<canvas>` content as WebP images, two frames a second. Off by default.                                   |

`blockClass` and `maskTextClass` take a class name or a regular expression tested against each class name. Replay selectors follow the same rule as `privacy` selectors: an invalid one is dropped with a console warning, and the rest still apply.

### Interaction records

Click, submit and rage-click records keep what the person did and which control they used. A record never includes what a private element shows or holds. A click on a private account-number field records that the "Account number" field was clicked, not the number.

| Target                                        | Recorded                                                                                                                        |
| --------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| Blocked (`interfere-block`, `blockSelector`)  | Event type, timing, tag name and field label.                                                                                   |
| Masked (`interfere-mask`, `maskTextSelector`) | Event type, timing, tag name, `role` and field label.                                                                           |
| Anything else                                 | Tag, `id`, `name`, `role`, `aria-label`, field label and up to 120 characters of text. Text inside private elements is skipped. |

The field label comes from your markup only:

1. A `data-interfere-label` attribute on the element or a parent, such as `<div data-sensitive data-interfere-label="Security code">`. Use it when the visible label sits inside the private region.
2. Otherwise, the field's `<label>` or `aria-labelledby` text, unless that label is itself private.

A private element's own `aria-label`, `placeholder`, `title`, text and value are never recorded, because apps often put the current value there. Form fields (`input`, `select`, `textarea`) never contribute text, even when they aren't private. Link URLs (`href`) are never recorded.

To keep page views and load timing without click and submit records, pass `plugins={{ pageEvents: { interactions: false } }}` to the provider. `pageEvents: false` turns off page events entirely, and `rageClick: false` turns off rage clicks.

### URL redaction

URLs are redacted before they leave the browser. Credentials in a URL (`https://user:pass@host`) always become `REDACTED:REDACTED`. Query parameter names stay, and their values are redacted according to `privacy.urls`:

| `privacy.urls`        | Values redacted                                 | Fragment |
| --------------------- | ----------------------------------------------- | -------- |
| Not set (default)     | Built-in sensitive names                        | Kept     |
| `{ deny: ["email"] }` | Built-in sensitive names and the names you list | Kept     |
| `{ allow: ["page"] }` | Every name except the ones you list             | Kept     |
| `true`                | Every name                                      | Removed  |
| `false`               | None                                            | Kept     |

```ts
void init({ privacy: { urls: { allow: ["page", "sort"], fragment: true } } });
```

The built-in sensitive names are `AWSAccessKeyId`, `sig`, `code` and `key`, plus any name containing `token`, `secret`, `password`, `passwd`, `auth`, `credential`, `session`, `signature`, `api_key` or `apikey`. Names match without regard to case. A name in `deny` is always redacted; a name in `allow` is kept, even when it is built in. `fragment: true` removes fragments, including hash-router paths.

Your app still uses the original URL for requests and routing. Path segments aren't redacted, so a token in a path such as `/share/abc123` is still recorded.

| Where URLs appear                                                                                                           | Redacted |
| --------------------------------------------------------------------------------------------------------------------------- | -------- |
| URL attributes and names of Interfere's spans, such as `GET /a?id=1`                                                        | Yes      |
| Navigation records, including the first page load                                                                           | Yes      |
| Absolute URLs in error messages, including messages for failed requests, resource loads and CSP violations                  | Yes      |
| Replay page URLs and `href`, `src`, `srcset`, `action`, `formaction`, `poster`, `cite` and `data` attributes                | Yes      |
| Spans from your own OpenTelemetry provider, stack frames, relative paths in error messages, CSS in replays and console logs | No       |

Replays show images and links from their recorded URLs, so an image behind a redacted signed URL (`?X-Amz-Signature=REDACTED`) won't load in the player.

## Options

Pass these to `init()`. Most apps need none of them.

| Option                          | Default            | What it does                                                                                                                            |
| ------------------------------- | ------------------ | --------------------------------------------------------------------------------------------------------------------------------------- |
| `consent`                       | All allowed        | `false` denies analytics and replay until you grant them. See [Consent](#consent).                                                      |
| `tracing.propagateTo`           | `[]`               | Other origins your pages call, such as `"https://api.example.com"`. They receive trace headers so their spans join the browser's trace. |
| `tracing.headers`               | All three          | Which trace headers Interfere may add: `traceparent`, `tracestate`, `baggage`. `false` adds none.                                       |
| `tracing.propagateTraceBaggage` | `true`             | `false` stops Interfere sending the session id in the `baggage` header.                                                                 |
| `tracing.captureFailedRequests` | 4xx and 5xx        | Reports failed responses from your own origin and `propagateTo` origins as errors. `false` turns this off.                              |
| `tracing.ignoreUrls`            | None               | Requests to matching URLs aren't recorded.                                                                                              |
| `tracing.webVitals`             | `true`             | `false` stops recording Core Web Vitals.                                                                                                |
| `tracing.spanProcessors`        | None               | Your own OpenTelemetry span processors. See below.                                                                                      |
| `tracing`                       | On                 | `false` turns off browser tracing: request spans, page views, clicks and web vitals. Errors are still captured.                         |
| `privacy`                       | Built-in redaction | See [Privacy](#privacy).                                                                                                                |
| `ignoreErrors`                  | None               | Drops errors whose message, or `Type: message`, matches. Strings match anywhere in the message.                                         |
| `denyUrls`                      | None               | Drops errors thrown from scripts whose URL matches, such as browser extensions or third-party widgets.                                  |
| `errors.sampleRate`             | `1`                | Sends this fraction of errors, from `0` to `1`.                                                                                         |
| `beforeSend`                    | None               | Runs last on each error. Edit it, return a replacement, or return `null` to drop it.                                                    |
| `serviceName`                   | Set by the build   | Names this app in Interfere. The build plugin sets it from your `package.json` name.                                                    |

`ignoreErrors`, `denyUrls`, `errors.sampleRate` and `beforeSend` apply to every error source except failed requests, which `tracing.captureFailedRequests` controls.

Your app's own origin always receives trace headers, as do the backend hosts listed for your workspace in the dashboard. Another origin receives them only through `propagateTo`.

### Span processors

`tracing.spanProcessors` takes standard OpenTelemetry processors. They run after Interfere adds session and device details and before export. This one removes an attribute:

```ts
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

Edit `span.attributes` synchronously in `onEnd`; `setAttribute` can't change an ended span. Interfere flushes and shuts down your processors, so give each `init()` new instances.

## Troubleshooting

### Requests to my API fail with a CORS error

A cross-origin API has to allow the headers Interfere adds:

```http
Access-Control-Allow-Headers: traceparent, tracestate, baggage
```

If you can't change the API, send it fewer headers:

```ts
void init({
  tracing: {
    propagateTo: [
      { origin: "https://api.example.com", headers: ["traceparent"] },
    ],
  },
});
```

| Origin                                      | Headers it receives                        | When                                                                                        |
| ------------------------------------------- | ------------------------------------------ | ------------------------------------------------------------------------------------------- |
| Your page's origin                          | `tracing.headers` (all three by default)   | Always                                                                                      |
| A `tracing.propagateTo` origin              | `tracing.headers`, or the entry's own list | Always                                                                                      |
| A backend host from your workspace settings | `tracing.headers`                          | On `GET` and `HEAD` at first; on other methods once one of those has succeeded this session |
| Anything else                               | None                                       | Never                                                                                       |

If a request carrying these headers fails at the network level, and that origin has never answered one, Interfere logs one warning naming the origin. It then stops sending headers there for the rest of the browser session. It never retries your request.

### No data arrives

- Check that the public key is set on the build: `VITE_INTERFERE_PUBLIC_KEY` for Vite, `INTERFERE_PUBLIC_KEY` for Next.js. Without it the SDK logs a warning and stays off.
- Look for `[interfere]` warnings in the browser console. They name the cause.
- `init()` must run before your app renders.
- Ad blockers can block telemetry. Serve it from your own domain with the `apiHost` option and the DNS record shown in the dashboard.
- Interfere ignores known bots.

### Data is sent late

Spans are sent every 60 seconds, or sooner once 200 are waiting, and logs every 5 seconds. When the page is hidden or closed, everything queued is sent at once. Spans still queued when a tab crashes are lost.

## Upgrading from 12.x

Upgrade every `@interfere/*` package together.

| 12.x                                                      | 13.x                                                           |
| --------------------------------------------------------- | -------------------------------------------------------------- |
| `tracing.apiHosts: ["api.example.com"]`                   | `tracing.propagateTo: ["https://api.example.com"]`             |
| `tracing.propagateContextUrls`, wildcard origins          | Removed. List each origin in `propagateTo`.                    |
| `<InterfereProvider errorBoundary={false}>`               | `<InterfereProvider captureRenderErrors={false}>`              |
| A failed cross-origin request was retried without headers | Not retried. Interfere logs a warning once per origin instead. |
| Interfere replaced a `traceparent` another tool wrote     | Interfere keeps it.                                            |

The [13.0 release notes](./CHANGELOG.md) list every change.

## Learn more

- [Interfere documentation](https://interfere.com/docs)
- [Next.js guide](https://interfere.com/docs/sdk/next-js) and [Vite guide](https://interfere.com/docs/sdk/vite)
- [Email support](mailto:support@interfere.com)

## License

MIT
