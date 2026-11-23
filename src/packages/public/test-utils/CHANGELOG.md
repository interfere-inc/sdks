# @interfere/test-utils

## 10.1.5

### Patch Changes

- 8f776d8: The server contract imports the installed Vitest instead of a bundled copy, so its hooks and assertions register with the running test.
- Updated dependencies [64e0dab]
  - @interfere/helpers@9.3.6

## 10.1.4

### Patch Changes

- 4d63981: Every public package is now published with npm trusted publishing and a provenance attestation.
- Updated dependencies [4d63981]
  - @interfere/helpers@9.3.5

## 10.1.3

### Patch Changes

- 13e0639: Include MIT licenses and package documentation, declare tree-shaking behavior, and document the supported Node.js and framework versions. Complete server adapter package metadata and link license badges to the included license.
- 13e0639: Remove workspace-only resolution conditions from stable and canary packages so consumers using custom conditions resolve shipped runtime and type files. Qualification tests now use the same sanitized package manifests while development source resolution stays unchanged.
- Updated dependencies [13e0639]
- Updated dependencies [13e0639]
  - @interfere/helpers@9.3.4

## 10.1.2

### Patch Changes

- 58353d7: The telemetry receiver now saves the response body of each forwarded request that gets a non-2xx response, in the new optional `response` field of `ReceivedRequest`. A forwarded request that throws is now recorded as 500 instead of 200.

## 10.1.1

### Patch Changes

- Updated dependencies [76a8c1e]
  - @interfere/helpers@9.3.3

## 10.1.0

### Minor Changes

- 2b931b4: `application()` accepts `{ tarballs }`, a map from package name to a prebuilt tarball, so a consumer can install a listed `@interfere/*` package from a release pack directory instead of packing it from the workspace.

### Patch Changes

- f584c2f: Preserve every shared JavaScript chunk alias and align its debug ID with the uploaded source map. Retain initiating callsites on XHR and transport failures, server error causes, and structured thrown values. Report partial client and server source-map coverage and verify delivered source evidence during packed SDK qualification.

  Publish source maps on Next.js 15 after its final Webpack client compilation, preserving customer hooks and waiting for publication and cleanup before prerendering.

- ec97a9c: Ship CHANGELOG.md inside the published tarball of every package so release notes travel with the installed version.
- 7c8441a: `exportedSpans()` and `exceptions()` keep each span's `links` (trace and span ids).
- Updated dependencies [ec97a9c]
  - @interfere/helpers@9.3.2

## 10.1.0-rc.2

### Patch Changes

- 7c8441a: `exportedSpans()` and `exceptions()` keep each span's `links` (trace and span ids).

## 10.1.0-rc.1

### Patch Changes

- ec97a9c: Ship CHANGELOG.md inside the published tarball of every package so release notes travel with the installed version.
- Updated dependencies [ec97a9c]
  - @interfere/helpers@9.3.2-rc.0

## 10.1.0-rc.0

### Minor Changes

- 2b931b4: `application()` accepts `{ tarballs }`, a map from package name to a prebuilt tarball, so a consumer can install a listed `@interfere/*` package from a release pack directory instead of packing it from the workspace.

## 10.0.3

### Patch Changes

- 5579c57: Expose captured application output for assertions in packed application tests.
- 152d420: Keep customer builds running when release configuration returns malformed data, and exercise production SDK journeys against accurate release configuration fixtures.
- Updated dependencies [461082f]
  - @interfere/helpers@9.3.1

## 10.0.2

### Patch Changes

- Updated dependencies [67b57b8]
- Updated dependencies [cb80e0a]
  - @interfere/helpers@9.3.0

## 10.0.1

### Patch Changes

- Updated dependencies [c4e0754]
  - @interfere/helpers@9.2.1

## 10.0.0

### Major Changes

- 7cbddb5: Remove the MSW server singleton, lifecycle wrapper, request interceptors, and associated types from `vitest/msw/node`. Use `setupServer` from `msw/node`, `http` and `HttpResponse` from `msw`, and your test runner's lifecycle hooks directly. Capture native `Request` objects in handlers when asserting requests.
- a66079f: Remove the Vitest and Playwright configuration wrappers, Node/browser/Worker presets, test tags, and viewport constants. Use each runner's native configuration APIs, Playwright device descriptors, and explicit project settings instead. The package no longer installs framework configuration plugins or loads environment files.

  Replace `expectPackedPackageFiles`, `expectNodeCanLoadPackageSpecifiers`, and `PACKAGE_CONTRACT_TIMEOUT_MS` with `createPackedPackage(packageJsonUrl)`. Use native assertions against the returned files and manifest, and run Node against `consumerDir` when checking package resolution. Always await `cleanup()` in a `finally` block. Vitest is no longer a peer dependency.

  The packing lifecycle isolates concurrent archives, rejects paths outside the owned consumer, and cleans temporary resources when setup fails. Deployment protection headers remain available.

- cc6197a: Remove unused browser HTTP interception helpers and configuration wrappers.

  - Replace `vitest/msw/browser` and `vitest/msw/http` imports with native `setupWorker` from `msw/browser`, `http` and `HttpResponse` from `msw`, and Vitest lifecycle hooks. Request capture belongs in the test's handler.
  - Import retained `HttpMethod` and `JsonValue` types from `vitest/msw/node`.
  - Replace `vitest/presets/vite` with native Vitest configuration and `playwright` from `@vitest/browser-playwright`. Set browser instances and viewports explicitly.
  - Replace `vitest/presets/integration` with native Vitest `test.globalSetup` pointing to resource lifecycle modules owned by the consuming project. Declare resource isolation and teardown explicitly.

- 19a0af0: The Workers Vitest preset now requires `@cloudflare/vitest-plugin` instead of `@cloudflare/vitest-pool-workers`. Consumers of this preset must replace the old optional peer dependency with the new plugin. Local Worker coverage runs now write LCOV reports alongside the existing summary and HTML output.

### Minor Changes

- 7a06dd6: `dualViewport()` accepts `mobileTests` and `exclude` options so packages that keep mobile specs outside `src/**/mobile/` no longer re-key the instances by index

### Patch Changes

- fa32955: Pin the optional Cloudflare Vitest plugin peer dependency to version 1.1.8.
- 63df2c9: Refresh dependency ranges. `@interfere/nest` now accepts NestJS 11 or 12 as a peer.
- dc6f82b: Link independent package dependencies with bounded concurrency when checking packed package contracts.
- 7d3b14e: Consolidate optional SDK and test settings while preserving configured defaults, disabled options, and omitted properties.
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
- Updated dependencies [2ec9c4f]
- Updated dependencies [b8b6902]
- Updated dependencies [63df2c9]
- Updated dependencies [5b8542b]
- Updated dependencies [fac5408]
  - @interfere/helpers@9.2.0

## 9.1.2

### Patch Changes

- f1dedba: improved types
- Updated dependencies [f1dedba]
  - @interfere/helpers@9.1.2

## 9.1.1

### Patch Changes

- c7f7d47: Add blank lines around multiline statements and before returns, from enforcing the internal padding-between-statements lint rule repo-wide. Whitespace only, no behavior or API difference.
- Updated dependencies [c7f7d47]
  - @interfere/helpers@9.1.1

## 9.1.0

### Minor Changes

- 87d3a7f: Add `redisPerWorkerDatabase` to `defineIntegrationConfig`.
- 3514661: Tune the shared Vitest config for local and agent-driven runs. Coverage still runs everywhere, but locally it reports through `text-summary` instead of the per-file `text` table, so a run ends with a six-line summary rather than a screen of output, and it now writes `lcov.info` locally as well as in CI. Browser Mode runs headless unless `UI=1`, so browser projects no longer open a window each. The JUnit reporter writes `test-results/junit.xml` in every environment, giving local runs a machine-readable result alongside the human one.
- 911ecd4: Add `@interfere/test-utils/vitest/presets/workers`, a `workersProject()` preset that runs Cloudflare Worker tests inside workerd via the Workers Vitest integration. `@cloudflare/vitest-pool-workers` is an optional peer dependency.

### Patch Changes

- e347b65: Internal lint modernization: apply oxlint autofixes across previously quarantined rules (import specifier style, catch variable naming, DOM idiom updates, and related mechanical cleanups). No behavior changes intended.
- 9a8843a: Bump the shared Vitest catalog from 4.1.9 to 4.1.10
- Updated dependencies [9162968]
- Updated dependencies [42d939d]
- Updated dependencies [e347b65]
  - @interfere/helpers@9.1.0

## 9.0.4

### Patch Changes

- Updated dependencies [d5d405c]
  - @interfere/helpers@9.0.3

## 9.0.3

### Patch Changes

- 32a1acf: bug fixes & performance improvements

## 9.0.2

### Patch Changes

- Updated dependencies [475f125]
  - @interfere/helpers@9.0.2

## 9.0.1

### Patch Changes

- 6c47775: Resolve published workspace dependency ranges from package manifests.
- d265875: Refresh public package artifacts after build output drift.
- 4c03108: Minor updates
- 7ba7566: Republish to swap the TypeScript devDependency from `@typescript/native-preview` to `typescript`.
- Updated dependencies [ac95069]
- Updated dependencies [d265875]
- Updated dependencies [4c03108]
- Updated dependencies [7ba7566]
  - @interfere/helpers@9.0.1

## 9.0.1-canary.2

### Patch Changes

- 4c03108: Minor updates
- Updated dependencies [4c03108]
  - @interfere/helpers@9.0.1-canary.2

## 9.0.1-canary.1

### Patch Changes

- 6c47775: Resolve published workspace dependency ranges from package manifests.

## 9.0.1-canary.0

### Patch Changes

- d265875: Refresh public package artifacts after build output drift.
- Updated dependencies [d265875]
  - @interfere/helpers@9.0.1-canary.0

## 2.0.0

### Major Changes

- normalization release

### Patch Changes

- Updated dependencies []:
  - @interfere/helpers@2.0.0

## 1.0.1

### Patch Changes

- [`669f5e4`](https://github.com/interfere-inc/interfere/commit/669f5e40c24cf47b4335b8e51df09efe40112d29) Thanks [@skve](https://github.com/skve)! - Added telemetry for per-sdk breakdowns
