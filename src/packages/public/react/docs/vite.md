# Vite

## Prerequisites

- Vite `>=5`
- React `>=19`

## Installation

```bash
npm install @interfere/react @interfere/vite
```

## Quick Start

**1. Add the Vite plugin**

```ts
// vite.config.ts
import { interfere } from "@interfere/vite/plugin";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [react(), interfere()],
});
```

**2. Initialize the SDK**

```ts
// src/main.tsx (before ReactDOM.createRoot)
import { init } from "@interfere/vite/init";

init();
```

**3. Add the provider**

```tsx
// src/app.tsx
import { InterfereProvider } from "@interfere/react/provider";

function App() {
  return (
    <InterfereProvider>
      <YourApp />
    </InterfereProvider>
  );
}
```

## Environment Variables

| Variable                    | Required | Description                                                                                                       |
| --------------------------- | -------- | ----------------------------------------------------------------------------------------------------------------- |
| `VITE_INTERFERE_PUBLIC_KEY` | Yes      | Surface publishable key in the `interfere_public_us_*` or `interfere_public_eu_*` format (injected at build time) |
| `VITE_INTERFERE_BUILD_ID`   | No       | Custom build identifier (defaults to git SHA)                                                                     |

Alternatively, pass `publicKey` / `buildId` directly to the plugin:

```ts
interfere({ buildId: "my-custom-build-id" });
```

## Identity Management

Link sessions to your authenticated users:

```tsx
import { useInterfere } from "@interfere/react/provider";

function useInterfereIdentity() {
  const { identity } = useInterfere();
  const { user } = useAuthProvider(); // Clerk, Auth0, etc.

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
}
```

`company.tier` sets the company’s tier in Interfere; tiers are managed under Settings or the tiers API.

### Parameters

| Field        | Required | Description                                                                                                                         |
| ------------ | -------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| `identifier` | Yes      | Unique user ID                                                                                                                      |
| `source`     | Yes      | Auth source: `{ type: "clerk", name: "Clerk" }`, `{ type: "auth0", name: "Auth0" }`, or `{ type: "custom", name: "Your Provider" }` |
| `name`       | No       | Display name                                                                                                                        |
| `email`      | No       | Email address                                                                                                                       |
| `avatar`     | No       | Avatar URL                                                                                                                          |
| `company`    | No       | Company tier: `{ tier: "Enterprise" }` (trimmed, 1–100 characters).                                                                 |
| `traits`     | No       | Arbitrary key-value metadata (`Record<string, unknown>`)                                                                            |

## Consent Management

By default, all SDK features are active. If your app must collect consent first:

```ts
// src/main.tsx
import { consent, init } from "@interfere/vite/init";

init({ consent: "required" });

consent.set(true); // once the visitor accepts; `{ replay: false }` grants analytics only, `false` revokes
```

See the `@interfere/vite` README for what is captured before consent and for consent-manager recipes.

## What's Included

- **Error tracking** -- automatic capture of uncaught errors and unhandled rejections
- **Session replay** -- full visual playback of user sessions
- **Page analytics** -- SPA-aware pageviews and UI interaction events
- **Rage click detection** -- surface frustrated user behavior
- **User identity** -- attach user context to sessions
- **Offline resilient** -- events are persisted and delivered in the background
