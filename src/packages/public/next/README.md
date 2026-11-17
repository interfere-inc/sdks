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
  <a href="https://interfere.com/docs/sdk/next-js">Documentation</a>
  ·
  <a href="mailto:support@interfere.com">Get help</a>
</p>

---

The Next.js SDK for [Interfere](https://interfere.com). It captures errors in the browser, on the server and at the edge. It records session replays, page views and the requests your pages make, and joins them into one trace per user action, so you can follow a bug from the click to the server error behind it. At build time it registers each release and uploads source maps, so stack traces point at your original code.

## Requirements

- Next.js 15.3 or later, or Next.js 16
- React 19
- Node.js 24

## Install

```bash
npm install @interfere/next
```

## Set up

Set two environment variables where your app builds and runs. Both are in the Interfere dashboard under **Settings → Developer**.

```bash
INTERFERE_PUBLIC_KEY=interfere_public_us_...   # sends browser telemetry; safe to ship to the browser
INTERFERE_API_KEY=interfere_secret_us_...      # registers releases, uploads source maps and sends server telemetry; keep it secret
```

Then add four files.

**1. Wrap your Next.js config.**

```ts
// next.config.ts
import { withInterfere } from "@interfere/next/config";
import type { NextConfig } from "next";

const nextConfig: NextConfig = {};

export default withInterfere(nextConfig);
```

If another tool also wraps your config, make `withInterfere()` the outermost wrapper.

**2. Capture server and edge errors.**

```ts
// instrumentation.ts
export { onRequestError, register } from "@interfere/next/instrumentation";
```

**3. Start the browser SDK.**

```ts
// instrumentation-client.ts
import "@interfere/next/instrumentation-client";
```

Next.js runs this file before your page hydrates. A production build with `INTERFERE_PUBLIC_KEY` set fails when this file is missing.

**4. Add the provider to your root layout.**

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

The provider reports render errors and then rethrows them, so your own error boundaries still choose what to show. It also gives components access to the `useInterfere()` hook.

Deploy, and errors, sessions and requests appear in Interfere.

### Passing options

To pass browser options, call `init()` instead of the bare import in step 3. Don't `await` it.

```ts
// instrumentation-client.ts
import { init } from "@interfere/next/instrument-client";

void init({
  tracing: { propagateTo: ["https://api.example.com"] },
});
```

To pass server options, call `register()` yourself:

```ts
// instrumentation.ts
import { register as registerInterfere } from "@interfere/next/instrumentation";

export { onRequestError } from "@interfere/next/instrumentation";

export async function register() {
  await registerInterfere({ privacy: { clientIp: false } });
}
```

## Identify users

Link sessions to the person who is signed in. Call `identity.clear()` when they sign out.

```tsx
"use client";

import { useInterfere } from "@interfere/next/provider";
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

`identifier` and `source` are required. `source.type` is `"clerk"`, `"auth0"`, `"workos"` or `"custom"`. A Clerk `identifier` must be the Clerk user id, which starts with `user_`. Optional fields are `name`, `email`, `avatar`, `company: { tier }` and `traits`.

## Report errors yourself

Errors that reach Next.js are captured for you. To report one you handled:

```ts
// In the browser
import { capture } from "@interfere/next/instrument-client";

capture(error);
```

```ts
// On the server
import { captureError } from "@interfere/next/server";

captureError(error);
```

Next.js doesn't report errors thrown inside `after()`. Wrap the callback to capture them:

```ts
import { after } from "next/server";
import { withInterfereAfter } from "@interfere/next/server";

after(
  withInterfereAfter(async () => {
    await recordAnalytics();
  }),
);
```

The SDK never breaks your app. If something fails inside Interfere, it logs one `[interfere]` warning and your code carries on.

## Using Datadog, Sentry or OpenTelemetry too

You don't need to change anything. Interfere works alongside them.

**In the browser**, Interfere checks each request for another tool that writes the `traceparent` header: Datadog RUM (for its `allowedTracingUrls`), Sentry (with `propagateTraceparent`) or another OpenTelemetry propagator. If one is there, Interfere leaves the header to it, so the request carries a single `traceparent` and stays in that tool's trace. Interfere records the request either way. Interfere never removes or replaces headers that your code or another tool set.

If another tool writes `traceparent` and Interfere doesn't recognise it, such as New Relic or an in-house interceptor, set `tracing: { propagation: "external" }` in `init()`. Interfere then writes no `traceparent` or `tracestate` and still records every request.

**On the server**, `register()` checks whether an OpenTelemetry tracer provider is already registered.

- With none, Interfere sets up tracing for you.
- With one, such as Sentry or `@vercel/otel`, Interfere uses it and leaves its exporters alone. Server errors still reach Interfere. Your traces reach Interfere only if that provider exports them to Interfere. Interfere logs one message saying so.
- If `dd-trace` traces the same process without its OpenTelemetry bridge, Datadog and Interfere record separate traces. Set `DD_TRACE_OTEL_ENABLED=true` and register Datadog's tracer provider before Interfere to keep one trace.

To keep your own provider on purpose, register it first and add Interfere's span processor to it, so your server spans carry the browser session:

```ts
// instrumentation.ts
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

`tracing: "application"` confirms the setup and silences the message. To send your provider's traces to Interfere, use the Interfere Trace Drain on Vercel, or point your OTLP exporter at Interfere. The [Next.js docs](https://interfere.com/docs/sdk/next-js) cover both.

## Privacy and consent

### Consent

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
import { consent } from "@interfere/next/instrument-client";

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

#### Connecting your consent manager

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

### What is never recorded

- What a form field holds, in click, submit and rage-click records.
- The content of elements with the `interfere-block` or `interfere-mask` class. Add your own selectors with `privacy: { blockSelector: "[data-sensitive]" }` in `init()`.
- Credentials in URLs, and the values of sensitive query parameters such as `token`, `password` and `sig`. Use `privacy.urls` to redact more.
- Cookies and `Authorization` headers on the server.

On the server, errors include the client IP from `x-forwarded-for`. Pass `privacy: { clientIp: false }` to `register()` to leave it out.

The [`@interfere/react` README](https://www.npmjs.com/package/@interfere/react#privacy) lists exactly what each record keeps.

## Options

Most apps need none of these.

### `withInterfere(settings, nextConfig)`

Pass settings as the first argument: `withInterfere({ serviceName: "storefront" }, nextConfig)`.

| Setting             | Default                            | What it does                                                                                           |
| ------------------- | ---------------------------------- | ------------------------------------------------------------------------------------------------------ |
| `serviceName`       | Your `package.json` name, unscoped | Names this app in Interfere. `OTEL_SERVICE_NAME` overrides the package name.                           |
| `sourceMaps.upload` | `true`                             | Uploads source maps at build time.                                                                     |
| `sourceMaps.purge`  | `true`                             | Deletes source maps from the build output afterwards, so they aren't served. Set `false` to keep them. |
| `disable`           | `false`                            | Turns Interfere off for this build. No release, no upload, no events.                                  |

### Browser `init()`

| Option                          | Default            | What it does                                                                                                                            |
| ------------------------------- | ------------------ | --------------------------------------------------------------------------------------------------------------------------------------- |
| `consent`                       | All allowed        | `"required"` holds analytics and replay until you grant consent. See [Consent](#consent).                                               |
| `tracing.propagateTo`           | `[]`               | Other origins your pages call, such as `"https://api.example.com"`. They receive trace headers so their spans join the browser's trace. |
| `tracing.headers`               | All three          | Which trace headers Interfere may add: `traceparent`, `tracestate`, `baggage`. `false` adds none.                                       |
| `tracing.propagateTraceBaggage` | `true`             | `false` stops Interfere sending the session id in the `baggage` header.                                                                 |
| `tracing.captureFailedRequests` | 4xx and 5xx        | Reports failed responses from your own origin and `propagateTo` origins as errors. `false` turns this off.                              |
| `tracing`                       | On                 | `false` turns off browser tracing: request spans, page views, clicks and web vitals. Errors are still captured.                         |
| `privacy`                       | Built-in redaction | `blockSelector`, `maskTextSelector` and `urls`. See [Privacy](#what-is-never-recorded).                                                 |
| `ignoreErrors`                  | None               | Drops errors whose message matches a string or regular expression.                                                                      |
| `denyUrls`                      | None               | Drops errors thrown from scripts whose URL matches, such as browser extensions or third-party widgets.                                  |
| `errors.sampleRate`             | `1`                | Sends this fraction of errors, from `0` to `1`.                                                                                         |
| `beforeSend`                    | None               | Edits an error before it is sent. Return `null` to drop it.                                                                             |

Your app's own origin always receives trace headers, as do the backend hosts listed for your workspace in the dashboard.

### Server `register()`

| Option             | Default            | What it does                                                                                                          |
| ------------------ | ------------------ | --------------------------------------------------------------------------------------------------------------------- |
| `consoleBridge`    | `true`             | Sends server `console` output to Interfere. `false` turns it off; `{ level: "warn" }` sends warnings and errors only. |
| `privacy.clientIp` | `true`             | `false` leaves the client IP out of server errors.                                                                    |
| `privacy.urls`     | Built-in redaction | Same as the browser option.                                                                                           |
| `tracing`          | `"auto"`           | `"application"` keeps your own OpenTelemetry provider. See [above](#using-datadog-sentry-or-opentelemetry-too).       |

### Environment variables

| Variable                | Required                     | What it does                                                                                                                                                                       |
| ----------------------- | ---------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `INTERFERE_PUBLIC_KEY`  | Yes                          | Your workspace public key. Without it the browser sends nothing.                                                                                                                   |
| `INTERFERE_API_KEY`     | For releases and server data | A workspace secret key. Builds need **Source maps**; the server runtime needs **Send telemetry**, and without this key it sends with the public key. Never ship it to the browser. |
| `INTERFERE_ENVIRONMENT` | No                           | Labels the deployment, such as `staging` or `production`. Defaults to `VERCEL_ENV` on Vercel.                                                                                      |
| `INTERFERE_SOURCE_ID`   | No                           | The commit SHA, if your CI doesn't expose one (`VERCEL_GIT_COMMIT_SHA`, `GITHUB_SHA`) and the build has no git checkout.                                                           |

## Troubleshooting

### Requests to my API fail with a CORS error

Interfere adds `traceparent`, `tracestate` and `baggage` headers to requests to your own origin, to `tracing.propagateTo` origins and to your workspace's backend hosts. A cross-origin API has to allow them:

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

If a request carrying these headers fails at the network level, and that origin has never answered one, Interfere logs one warning naming the origin. It then stops sending headers there for the rest of the browser session. It never retries your request.

For a backend host from your workspace settings, Interfere adds no headers until its own credentialed `HEAD` check to that host passes CORS in this browser session, and then only the headers that passed, so a strict CORS policy can't break your requests.

### No data arrives

- Check that `INTERFERE_PUBLIC_KEY` is set on the build. Without it the build logs a warning and the SDK stays off.
- For server errors and traces, check that `INTERFERE_API_KEY` is set where the server runs and has the **Send telemetry** permission.
- Look for `[interfere]` warnings in the build output and the browser console. They name the cause.
- If the warning says `withInterfere()` did not run, another config wrapper dropped it. Make `withInterfere()` the outermost wrapper.
- Check that `instrumentation.ts` and `instrumentation-client.ts` are in your project root, or in `src/` if you use one.
- Ad blockers can block telemetry. Serve it from your own domain by setting `interfere: { apiHost: "https://in.example.com" }` in your Next.js config and adding the DNS record shown in the dashboard.
- `withInterfere({ disable: true }, …)` turns everything off.

### Stack traces point at minified code

- Set `INTERFERE_API_KEY` on the build. Without it the build skips the release and the upload, and logs a warning.
- In Interfere, link the app to its repository, and to its Vercel project if you deploy on Vercel. Without these links the build skips the upload and logs a warning.
- The build needs a commit SHA. Most CI systems provide one. Otherwise set `INTERFERE_SOURCE_ID`.
- Check that `sourceMaps.upload` isn't `false`.

A failed upload never fails your build. Only a misconfiguration fails it, such as a secret key with the wrong prefix.

### A fresh release won't install

pnpm's `minimumReleaseAge` and Bun's `minimumReleaseAge` hold back new versions. To install early, add `@interfere/*` to `minimumReleaseAgeExclude` (pnpm) or `install.minimumReleaseAgeExcludes` (Bun).

## Upgrading from 12.x

Upgrade every `@interfere/*` package together. Most apps change nothing else.

| 12.x                                                    | 13.x                                                                                                                                           |
| ------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| `<InterfereProvider errorBoundary={false}>`             | `<InterfereProvider captureRenderErrors={false}>`                                                                                              |
| `init({ tracing: { apiHosts: ["api.example.com"] } })`  | `init({ tracing: { propagateTo: ["https://api.example.com"] } })`                                                                              |
| `tracing.propagateContextUrls`, wildcard origins        | Removed. List each origin in `propagateTo`.                                                                                                    |
| `<TraceMeta />` from `@interfere/next/server`           | Remove it. `withInterfere()` adds the trace metadata.                                                                                          |
| The server OpenTelemetry kit (`buildInterfereOtelKit`)  | Removed. Use `register({ tracing: "application" })` with `correlationSpanProcessor()`, as [above](#using-datadog-sentry-or-opentelemetry-too). |
| A 12.x service name that differs from your package name | Set `serviceName` in `withInterfere()` to keep it. Find it under **Services** in the dashboard.                                                |

The [13.0 release notes](./CHANGELOG.md) list every change.

## Learn more

- [Next.js guide](https://interfere.com/docs/sdk/next-js)
- [Interfere documentation](https://interfere.com/docs)
- [`@interfere/react`](https://www.npmjs.com/package/@interfere/react), for privacy details and the React API
- [Email support](mailto:support@interfere.com)

## License

MIT
