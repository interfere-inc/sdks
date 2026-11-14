# @interfere/server-core

Shared telemetry runtime for Interfere server adapters. Most applications should install `@interfere/next` or `@interfere/vite`, which configure this package for their framework.

Adapter authors can use `@interfere/server-core/runtime` for telemetry export, `@interfere/server-core/node` for Node.js context and tracing, and `@interfere/server-core/ownership` to coordinate with an existing OpenTelemetry provider. The `console-bridge`, `contain`, and `contained-span-processor` entry points provide logging and error containment helpers.

Initialization is explicit. Importing the package does not register a global OpenTelemetry provider or install process handlers.

`startNodeRuntime()` accepts `options.privacy.clientIp`, which defaults to `true`. When set to `false`, the capture policy removes `http.request.header.x-forwarded-for` before recording exception events on either the host or Interfere provider. Adapters supply the forwarded header when request context is available.

## License

[MIT](./LICENSE)

## Supported runtimes

Node.js 24.x is the supported Node.js release line and is exercised in CI.
