# @interfere/build

Build utilities shared by Interfere framework integrations. Requires Node.js.

Use `@interfere/javascript` to call release endpoints directly. This package
provides build configuration resolution, release metadata, ordered source-map
chunk uploads, and local artifact cleanup. Framework integrations own the sequence
of API calls and only delete source maps after the server accepts the full batch.

Source-map uploads use 20 concurrent workers by default. Set
the `uploadSourceMapChunks` `concurrency` option takes precedence. Next.js and Vite
share this setting. Pass the variable through your build runner when it filters
environment variables, such as Turborepo's strict environment mode.

Next.js and Vite use gzip only when the signed upload session advertises support,
with identity uploads for payloads that do not shrink. Older servers receive
uncompressed chunks of at most 512 KiB. Servers advertising gzip accept chunks
of up to 8 MiB. Chunk offsets and reported byte counts refer to the original,
uncompressed content.

When calling `uploadSourceMapChunks` directly, construct the API client with
`fetch: sourceMapUploadFetch` from `@interfere/build/source-map-upload-fetch` and
pass `compression: "gzip"` to the uploader. Direct callers retain identity uploads
unless they opt in; Next.js and Vite configure both automatically.

`resolveBuildServiceName` from `@interfere/build/service-name` supplies the same
service identity policy to Next.js and Vite: an explicit plugin option, then
`OTEL_SERVICE_NAME`, then the app's unscoped `package.json` name.
Blank values are skipped; a missing or invalid package name leaves the default
unset. Each framework injects this value into its browser and server entry points,
where explicit runtime service names still take precedence.

## Supported runtimes

Node.js 24.x is the supported Node.js release line and is exercised in CI.

Modules are safe to tree shake (`sideEffects: false`). Call initialization, build, or test helpers explicitly when their behavior is needed.
