# @interfere/test-utils

Test helpers for Interfere integrations:

- `@interfere/test-utils/vitest/telemetry-receiver` creates a local HTTP receiver for telemetry assertions.
- `@interfere/test-utils/vitest/otlp` reads exported traces and logs from recorded requests.
- `@interfere/test-utils/vitest/server-contract` defines adapter lifecycle and privacy tests.
- `@interfere/test-utils/vitest/package-contract` packs a package into an isolated consumer directory.
- `@interfere/test-utils/playwright/protection-bypass` helps test deployments protected by an access token.

Call the helpers explicitly to create receivers or register tests, and close receivers and clean up packed consumers when tests finish. Vitest helpers that register tests require Vitest in the consuming test suite.

## License

[MIT](./LICENSE)

## Supported runtimes

Node.js 24.x is the supported Node.js release line and is exercised in CI.

Modules are safe to tree shake (`sideEffects: false`). Call initialization, build, or test helpers explicitly when their behavior is needed.
