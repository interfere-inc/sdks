# @interfere/javascript

## 0.5.0

### Minor Changes

- db2b560: Problem and error body types now share one `Interfere.ErrorService` type for their `service` field, which replaces per-type `Service` namespaces such as `ApiKeyAuthProblem.Service`.
- ead7ecb: Source-map uploads now send each map's SHA-256 first and skip uploading maps your Company already stores.

  ### Network
  - Data sent and consent: the sign request now carries a `sha256` for each source map, and the SDK sends no bytes for maps the response marks `present`.

- 4eb17ec: `config.getSdkConfig`, `sessions.createSession` and `sessions.identifySession` accept a release token through the `releaseTokenHeader` option and send it as the `x-interfere-release-token` header.

  ### Network
  - Headers: `GET /v3/config`, `POST /v3/session` and `POST /v3/session/identify` send `x-interfere-release-token` when the caller passes `releaseTokenHeader`; requests from callers that don't pass it are unchanged.

- ca4d372: `drain.otlpExport` accepts a release token through the `releaseTokenHeader` option and sends it as the `x-interfere-release-token` header.

  ### Network
  - Headers: `POST /drain/v1/{signal}` sends `x-interfere-release-token` when the caller passes `releaseTokenHeader`; requests from callers that don't pass it are unchanged.

- ec31e5d: `createRelease` responses now include a signed release `token` when the release belongs to a service.

  ### Network
  - Data sent and consent: the release registration response now also returns `token`, a signed value holding the workspace, service and release ids, the release slug, the issue time and the key id; it is public by design.

- 4eb17ec: `createRelease` now registers a release for every service, and its response's legacy `surface` field is `null` for a service that has none.
- e820a7f: Source-map manifests now accept `rolldown` as the bundler, for Vite 8 builds.
- 0be7ad5: A build with a workspace API key now publishes source maps for every service: `getReleasesConfig` answers with a `null` legacy `surface` field for a service that has none, and the build plugins read the config without it.

### Patch Changes

- 4d63981: Every public package is now published with npm trusted publishing and a provenance attestation.

## 0.4.1

### Patch Changes

- 13e0639: Include MIT licenses and package documentation, declare tree-shaking behavior, and document the supported Node.js and framework versions. Complete server adapter package metadata and link license badges to the included license.

## 0.4.0

### Minor Changes

- 3dfaefa: Accept `company: { tier }` in `identity.set()` and send company tier changes with session identification.
- d44b603: Send the existing service identity on SDK config, session, replay, and release requests so workspace public keys can select the App when gateway workspace-key routing is enabled. Build config caching now separates services and namespaces, and retained Vite builds preserve the service name for later release publication. Existing App keys keep working.

### Patch Changes

- f584c2f: Preserve every shared JavaScript chunk alias and align its debug ID with the uploaded source map. Retain initiating callsites on XHR and transport failures, server error causes, and structured thrown values. Report partial client and server source-map coverage and verify delivered source evidence during packed SDK qualification.

  Publish source maps on Next.js 15 after its final Webpack client compilation, preserving customer hooks and waiting for publication and cleanup before prerendering.

- 420d2c0: Publish the repository field so npm links the source directory.
- ec97a9c: Ship CHANGELOG.md inside the published tarball of every package so release notes travel with the installed version.

## 0.4.0-rc.2

### Patch Changes

- 420d2c0: Publish the repository field so npm links the source directory.

## 0.4.0-rc.1

### Minor Changes

- 3dfaefa: Accept `company: { tier }` in `identity.set()` and send company tier changes with session identification.

### Patch Changes

- ec97a9c: Ship CHANGELOG.md inside the published tarball of every package so release notes travel with the installed version.

## 0.4.0-rc.0

### Minor Changes

- d44b603: Send the existing service identity on SDK config, session, replay, and release requests so workspace public keys can select the App when gateway workspace-key routing is enabled. Build config caching now separates services and namespaces, and retained Vite builds preserve the service name for later release publication. Existing App keys keep working.

## 0.3.3

### Patch Changes

- 3a4f991: Accept an optional `replayTrigger` on session creation so the browser SDK can register an anonymous session's replay once the visitor hits an error.
- 4676d54: Expose discovered backend hosts in SDK configuration responses.
- e1f7ee8: Send the upload session's grant on source-map chunk and batch uploads, so each chunk skips per-request API-key verification after the sign call has authorized the session.

## 0.3.2

### Patch Changes

- d5a2d36: Read each app's anonymous session replay policy from its remote configuration and bake it into Next.js and Vite builds, so the browser SDK can record visitors who are not signed in only after an error.

## 0.3.1

### Patch Changes

- 4d4457c: The 2 s cap on direct OTLP trace and log exports now follows the surface's hosting platform from the Interfere configuration, not the `VERCEL` environment variable. The configuration reports `platform: "vercel"` for a Next.js surface linked to a Vercel project, and `"cloudflare"` for a surface linked to a Cloudflare Worker. The build injects it the same way it injects the drain modes.

  - The cap applies when the injected platform is `vercel`. A surface with no linked platform exports with the standard timeouts, even when `VERCEL` is set.
  - When the build could not load the configuration, the SDK falls back to `VERCEL` for this decision. That fallback is removed in the next major.
  - `VERCEL_ENV` and `VERCEL_TARGET_ENV` still serve only as fallbacks for the environment label.
  - `@interfere/types`: adds `@interfere/types/integrations/platforms/hosting` (`hostingPlatformSchema`, `HostingPlatform`, `PLATFORM_GLOBAL`). `resolveServerTracing` takes the injected `platform`.
  - `@interfere/javascript`: `SdkConfigResponse` and the release config types gain the optional `platform` field.

## 0.3.0

### Minor Changes

- f9d411d: Regenerate the JavaScript API client from the current API schema and include the browser mapping that prevents bundlers from resolving Node's fs module.

## 0.2.1

### Patch Changes

- b52524b: Improve JavaScript client compatibility across browser, Edge, and Node.js runtimes.

## 0.2.0

### Minor Changes

- a653b8d: Add the generated JavaScript API client with ESM, CommonJS, and TypeScript declarations. Next and Vite call its endpoints directly for build configuration, release creation, source-map uploads, and telemetry. Remove the shared build client and its combined release-preparation method. Preserve ordered uploads, expiration checks, and cleanup only after complete server acceptance.

  Build requests use Fern's retry policy for HTTP 408, 429, and 5xx responses. Network failures and HTTP 425 are no longer retried.

  Retire the unused CLI and Speakeasy SDK packages, their generation configuration, and compatibility wrappers. Remove legacy host remapping and Collector error aliases. Update framework setup instructions to use the generated release endpoints.

- a653b8d: Generate runtime configuration, session creation, replay upload, and telemetry ingestion methods alongside the release API. Publish these runtime operations to SDK and MCP documents without adding them to HTTP documentation.

  Use generated endpoint methods throughout the framework SDKs. Authenticate browser requests through the public-key header, preserve request timeouts and unload delivery, and report failed edge telemetry requests as failed exports.

- 65b3ebb: Use a source-map upload-session response containing the expiry and file paths for v3 uploads. Upload chunks through the generated client with a path and byte offset. Legacy signing schemas and v1 tokenized upload URLs remain available.

  Remove unused session request headers, build dependencies, and Vite release-result fields.

- 78bf77a: Remove replay chunk transport from the public API client. The replay recorder now owns its internal upload transport and preserves authentication headers, compression, and failure handling. Public API discovery groups operations by product and excludes internal bookkeeping.
