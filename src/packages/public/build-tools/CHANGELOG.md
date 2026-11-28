# @interfere/build

## 0.7.0

### Minor Changes

- cc05042: Builds no longer stamp `__INTERFERE_PUBLIC_KEY__` into bundles and the build, server and release-diagnostic helpers no longer accept a public key, because the release token is the browser credential.

### Patch Changes

- Updated dependencies [aea0bef]
- Updated dependencies [cc05042]
  - @interfere/types@11.3.0
  - @interfere/javascript@0.7.0

## 0.6.4

### Patch Changes

- Updated dependencies [0c9298c]
  - @interfere/types@11.2.3

## 0.6.3

### Patch Changes

- Updated dependencies [7358d82]
  - @interfere/types@11.2.2

## 0.6.2

### Patch Changes

- Updated dependencies [5d39a3c]
- Updated dependencies [64e0dab]
- Updated dependencies [989e833]
  - @interfere/types@11.2.1

## 0.6.1

### Patch Changes

- Updated dependencies [3ef4bdb]
  - @interfere/types@11.2.0
  - @interfere/javascript@0.6.0

## 0.6.0

### Minor Changes

- ead7ecb: Source-map uploads now send each map's SHA-256 first and skip uploading maps your Company already stores.

  ### Network
  - Data sent and consent: the sign request now carries a `sha256` for each source map, and the SDK sends no bytes for maps the response marks `present`.

- 4eb17ec: A Vite build with a workspace API key and no public key now registers its release and bakes the release token, and `resolveReleaseRequest` accepts a `null` config, taking the deploy target from the build environment.

  ### Network
  - Data sent and consent: a build whose workspace key has no public key now registers its release with `POST /v3/releases`, sending the `x-interfere-service` header, the commit and the deploy target read from the build environment; builds with a public key are unchanged.

- 4eb17ec: When your workspace accepts release tokens, SDK settings no longer need a public key: a build with `INTERFERE_API_KEY` and no public key bakes its settings (server tracing, log sourcing, plugins, ingest URL, backend hosts, anonymous replay) by fetching `/v3/config` with the release token it registered, the server fetches its remote plugin config with the token baked into the server bundle, and `createBuildConfigResolver` now takes a credential, `{ publicKey }` or `{ releaseToken }`, in place of the public key string.

  ### Network
  - Headers: build-time and server `GET /v3/config` send `x-interfere-release-token` instead of `x-interfere-public-key` when the build has no public key. Builds with a public key are unchanged.

- 4eb17ec: Breaking: `resolveReleaseRequest` now returns the release request itself and never skips a release, so builds register releases and upload source maps without a repository or deploy target linked in Interfere.

  ### Network
  - Data sent and consent: builds with no repository or deploy target linked in Interfere now register their release and upload source maps, with the deploy target read from the build environment.

- 4eb17ec: Breaking: `missingReleaseBinding` is removed, since a missing repository or deploy target no longer disables a build.
- 85d4511: Builds register releases with their service identity in the request body using only `INTERFERE_API_KEY`.

  ### Network
  - Data sent and consent: release registration sends `service.name` in the JSON body, including releases published from retained Vite build output; the build-tools resolver also accepts `service.namespace`.

### Patch Changes

- 4d63981: Every public package is now published with npm trusted publishing and a provenance attestation.
- 0be7ad5: A build with a workspace API key now publishes source maps for every service: `getReleasesConfig` answers with a `null` legacy `surface` field for a service that has none, and the build plugins read the config without it.
- Updated dependencies [4d63981]
- Updated dependencies [b8c5bad]
- Updated dependencies [9aaddd6]
- Updated dependencies [d1b1138]
- Updated dependencies [db2b560]
- Updated dependencies [ead7ecb]
- Updated dependencies [042188e]
- Updated dependencies [6e18c76]
- Updated dependencies [4eb17ec]
- Updated dependencies [ca4d372]
- Updated dependencies [ec31e5d]
- Updated dependencies [4eb17ec]
- Updated dependencies [e820a7f]
- Updated dependencies [f2e532d]
- Updated dependencies [5f28fcd]
- Updated dependencies [0be7ad5]
- Updated dependencies [c0c40ed]
- Updated dependencies [b77bfb3]
  - @interfere/javascript@0.5.0
  - @interfere/types@11.1.0

## 0.5.2

### Patch Changes

- 13e0639: Use neutral examples in SDK documentation and fixtures, and describe regression tests by their behavior. Remove internal issue references from public API documentation.

  Correct the SDK 13.0 migration notes: list breaking tracing changes together, consolidate repeated entries, and clarify that the canonical browser configuration is `tracing: { mode: "auto" }`, with `tracing: "auto"` still supported. Tracing ownership and header propagation are separate options; use `tracing.propagation: "external"` when another tool writes W3C headers.

- 13e0639: Include MIT licenses and package documentation, declare tree-shaking behavior, and document the supported Node.js and framework versions. Complete server adapter package metadata and link license badges to the included license.
- 13e0639: Remove workspace-only resolution conditions from stable and canary packages so consumers using custom conditions resolve shipped runtime and type files. Qualification tests now use the same sanitized package manifests while development source resolution stays unchanged.
- Updated dependencies [13e0639]
- Updated dependencies [13e0639]
- Updated dependencies [13e0639]
- Updated dependencies [13e0639]
- Updated dependencies [13e0639]
  - @interfere/types@11.0.2
  - @interfere/javascript@0.4.1

## 0.5.1

### Patch Changes

- Updated dependencies [30a48a6]
- Updated dependencies [76a8c1e]
- Updated dependencies [76a8c1e]
- Updated dependencies [76a8c1e]
- Updated dependencies [76a8c1e]
- Updated dependencies [76a8c1e]
  - @interfere/types@11.0.1

## 0.5.0

### Minor Changes

- d44b603: Send the existing service identity on SDK config, session, replay, and release requests so workspace public keys can select the App when using workspace keys. Build config caching now separates services and namespaces, and retained Vite builds preserve the service name for later release publication. Existing App keys keep working.

### Patch Changes

- f584c2f: Separate source maps for Vercel deployments built from the same Git commit. Keep build-time release identities in browser and server bundles during promotion, while retaining the original commit for source ownership.
- 35cbb18: Strip each built chunk once during source-map cleanup. Turbopack can give two chunks one debug ID, and concurrent strips of a shared chunk could leave it empty, which broke prerendering with "module factory is not available".
- ec97a9c: Ship CHANGELOG.md inside the published tarball of every package so release notes travel with the installed version.
- Updated dependencies [f584c2f]
- Updated dependencies [68a9ee0]
- Updated dependencies [2938037]
- Updated dependencies [420d2c0]
- Updated dependencies [3dfaefa]
- Updated dependencies [8f92d43]
- Updated dependencies [c76563f]
- Updated dependencies [3ff22ac]
- Updated dependencies [3226d63]
- Updated dependencies [ec97a9c]
- Updated dependencies [7c8441a]
- Updated dependencies [d44b603]
  - @interfere/types@11.0.0
  - @interfere/javascript@0.4.0

## 0.5.0-rc.4

### Patch Changes

- 35cbb18: Strip each built chunk once during source-map cleanup. Turbopack can give two chunks one debug ID, and concurrent strips of a shared chunk could leave it empty, which broke prerendering with "module factory is not available".
- Updated dependencies [420d2c0]
  - @interfere/javascript@0.4.0-rc.2

## 0.5.0-rc.3

### Patch Changes

- Updated dependencies [7c8441a]
  - @interfere/types@11.0.0-rc.2

## 0.5.0-rc.2

### Patch Changes

- ec97a9c: Ship CHANGELOG.md inside the published tarball of every package so release notes travel with the installed version.
- Updated dependencies [2938037]
- Updated dependencies [3dfaefa]
- Updated dependencies [8f92d43]
- Updated dependencies [ec97a9c]
  - @interfere/types@11.0.0-rc.1
  - @interfere/javascript@0.4.0-rc.1

## 0.5.0-rc.1

### Minor Changes

- d44b603: Send the existing service identity on SDK config, session, replay, and release requests so workspace public keys can select the App when gateway workspace-key routing is enabled. Build config caching now separates services and namespaces, and retained Vite builds preserve the service name for later release publication. Existing App keys keep working.

### Patch Changes

- Updated dependencies [d44b603]
  - @interfere/javascript@0.4.0-rc.0

## 0.4.6-rc.0

### Patch Changes

- Updated dependencies [68a9ee0]
- Updated dependencies [c76563f]
- Updated dependencies [3ff22ac]
- Updated dependencies [3226d63]
  - @interfere/types@11.0.0-rc.0

## 0.4.5

### Patch Changes

- e1f7ee8: Send the upload session's grant on source-map chunk and batch uploads, so each chunk skips per-request API-key verification after the sign call has authorized the session.
- Updated dependencies [3a4f991]
- Updated dependencies [4676d54]
- Updated dependencies [e1f7ee8]
  - @interfere/javascript@0.3.3
  - @interfere/types@10.6.3

## 0.4.4

### Patch Changes

- 6347222: Check surface bindings before compiling production telemetry. Incomplete surfaces leave the SDK inactive with one setup link, while temporary configuration outages keep capture enabled.
- 2f2934a: Enable development telemetry when a public key is configured and remove undocumented environment overrides in favor of existing SDK options and automatic defaults.
- 152d420: Keep customer builds running when release configuration returns malformed data, and exercise production SDK journeys against accurate release configuration fixtures.
- Updated dependencies [d5a2d36]
- Updated dependencies [2f2934a]
- Updated dependencies [c2428dd]
  - @interfere/types@10.6.2
  - @interfere/javascript@0.3.2

## 0.4.3

### Patch Changes

- Updated dependencies [8381980]
  - @interfere/types@10.6.1

## 0.4.2

### Patch Changes

- Updated dependencies [9dd1bf0]
- Updated dependencies [c949553]
  - @interfere/types@10.6.0

## 0.4.1

### Patch Changes

- 4d4457c: The 2 s cap on direct OTLP trace and log exports now follows the surface's hosting platform from the Interfere configuration, not the `VERCEL` environment variable. The configuration reports `platform: "vercel"` for a Next.js surface linked to a Vercel project, and `"cloudflare"` for a surface linked to a Cloudflare Worker. The build injects it the same way it injects the drain modes.

  - The cap applies when the injected platform is `vercel`. A surface with no linked platform exports with the standard timeouts, even when `VERCEL` is set.
  - When the build could not load the configuration, the SDK falls back to `VERCEL` for this decision. That fallback is removed in the next major.
  - `VERCEL_ENV` and `VERCEL_TARGET_ENV` still serve only as fallbacks for the environment label.
  - `@interfere/types`: adds `@interfere/types/integrations/platforms/hosting` (`hostingPlatformSchema`, `HostingPlatform`, `PLATFORM_GLOBAL`). `resolveServerTracing` takes the injected `platform`.
  - `@interfere/javascript`: `SdkConfigResponse` and the release config types gain the optional `platform` field.

- Updated dependencies [4d4457c]
  - @interfere/types@10.5.1
  - @interfere/javascript@0.3.1

## 0.4.0

### Minor Changes

- 7582c3e: Share application service-name inference between Next.js and Vite. Both build integrations use the explicit plugin option, then `OTEL_SERVICE_NAME`, then the application's unscoped package name. Vite retains `VITE_INTERFERE_SERVICE_NAME` as a fallback before package inference. Browser and server telemetry consume the inferred default, including Next.js edge telemetry and Vite server registration, while explicit runtime names retain precedence.

### Patch Changes

- Updated dependencies [0c772cc]
  - @interfere/types@10.5.0

## 0.3.3

### Patch Changes

- Updated dependencies [4fff52d]
- Updated dependencies [47943fb]
  - @interfere/types@10.4.0

## 0.3.2

### Patch Changes

- Updated dependencies [f9d411d]
  - @interfere/javascript@0.3.0

## 0.3.1

### Patch Changes

- 8a20a12: Keep Next.js application builds running when a Surface has no source or deployment target configured. Warn and skip release registration and source-map uploads until setup is complete. Negotiate compressed source-map uploads so older SDKs keep receiving a compatible upload capability.
- Updated dependencies [4d17ef3]
- Updated dependencies [a231cb5]
  - @interfere/types@10.3.0

## 0.3.0

### Minor Changes

- 15efdb8: Compress source-map uploads, use larger streaming chunks, and make concurrency configurable.

### Patch Changes

- 99f3752: Negotiate gzip source-map uploads with the server and retain uncompressed 512 KiB chunks for older deployments.
- Updated dependencies [99f3752]
- Updated dependencies [8ffe6fb]
  - @interfere/types@10.2.2

## 0.2.2

### Patch Changes

- Updated dependencies [99a1f17]
  - @interfere/types@10.2.1

## 0.2.1

### Patch Changes

- Updated dependencies [b52524b]
  - @interfere/javascript@0.2.1

## 0.2.0

### Minor Changes

- 40501f3: Batch small source-map files when the upload session advertises compatible support, using one bounded upload pool and preserving the existing path for older servers and large files.
- 21309b2: Upload source maps separately for each deploy environment, so preview and production builds of the same commit no longer conflict. Server-side events now report the build's deploy environment, which matches browser events and the uploaded source maps. Server events that previously fell back to `NODE_ENV` can now carry a different environment label.
- a653b8d: Add the generated JavaScript API client with ESM, CommonJS, and TypeScript declarations. Next and Vite call its endpoints directly for build configuration, release creation, source-map uploads, and telemetry. Remove the shared build client and its combined release-preparation method. Preserve ordered uploads, expiration checks, and cleanup only after complete server acceptance.

  Build requests use Fern's retry policy for HTTP 408, 429, and 5xx responses. Network failures and HTTP 425 are no longer retried.

  Retire the unused CLI and Speakeasy SDK packages, their generation configuration, and compatibility wrappers. Remove legacy host remapping and Collector error aliases. Update framework setup instructions to use the generated release endpoints.

- a653b8d: Generate runtime configuration, session creation, replay upload, and telemetry ingestion methods alongside the release API. Publish these runtime operations to SDK and MCP documents without adding them to HTTP documentation.

  Use generated endpoint methods throughout the framework SDKs. Authenticate browser requests through the public-key header, preserve request timeouts and unload delivery, and report failed edge telemetry requests as failed exports.

- 87fad80: Share deployment metadata and source-map file utilities across Next and Vite. Both integrations call the generated API client endpoints directly. Source maps are removed only after durable acceptance of the complete upload.

### Patch Changes

- 0efde29: Fix source imports used by Node when loading development build configurations.
- 3a309ab: Remove the unused Zod runtime dependency.
- 65b3ebb: Use a source-map upload-session response containing the expiry and file paths for v3 uploads. Upload chunks through the generated client with a path and byte offset. Legacy signing schemas and v1 tokenized upload URLs remain available.

  Remove unused session request headers, build dependencies, and Vite release-result fields.

- 39c53c1: Fix the client release URL path.
- 265a458: Fail package test commands when no tests are discovered.
- ffaa95b: Fix build output that referenced deleted source maps after upload.
- 5b8542b: Preserve inferred capture attribute types and verify optional telemetry fields with native exporter tests.

  Preserve deep-merge behavior for nested values and use schema-owned field types in telemetry contracts.

  Preserve inferred release-slug types and validate baked configuration before using it.

  Treat inherited object-property names as unknown release error codes and hostnames rather than matching them as configured entries.

  Use the generated request contracts for handwritten source-map upload helpers while preserving finalize-response validation.

  Validate test application manifests before creating temporary directories so malformed manifests do not leave unused directories behind.

  Validate source maps before patching JavaScript debug IDs in Vite and Next builds, preserving the original artifacts when a map is malformed.

  Preserve special own keys such as `__proto__` as data when omitting undefined properties, without changing the result prototype.

  Validate Vite source maps before injecting debug IDs into JavaScript chunks, so malformed maps leave the chunk unchanged.

- fac5408: Strengthen event validation and type handling, correct promise handling, and update test runner integration.
- Updated dependencies [5b005aa]
- Updated dependencies [40501f3]
- Updated dependencies [0efde29]
- Updated dependencies [63df2c9]
- Updated dependencies [a653b8d]
- Updated dependencies [a653b8d]
- Updated dependencies [65b3ebb]
- Updated dependencies [78bf77a]
- Updated dependencies [67ec54b]
- Updated dependencies [5ff7610]
- Updated dependencies [87fad80]
- Updated dependencies [88ce674]
- Updated dependencies [5b8542b]
- Updated dependencies [fac5408]
- Updated dependencies [27d080c]
- Updated dependencies [bbfda83]
- Updated dependencies [d25952d]
  - @interfere/types@10.2.0
  - @interfere/javascript@0.2.0
