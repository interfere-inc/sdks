# @interfere/helpers

## 9.3.5

### Patch Changes

- 4d63981: Every public package is now published with npm trusted publishing and a provenance attestation.

## 9.3.4

### Patch Changes

- 13e0639: Include MIT licenses and package documentation, declare tree-shaking behavior, and document the supported Node.js and framework versions. Complete server adapter package metadata and link license badges to the included license.
- 13e0639: Remove workspace-only resolution conditions from stable and canary packages so consumers using custom conditions resolve shipped runtime and type files. Qualification tests now use the same sanitized package manifests while development source resolution stays unchanged.

## 9.3.3

### Patch Changes

- 76a8c1e: Deliver telemetry in Edge runtimes that do not provide `AbortSignal.any`. Preserve caller cancellation and export deadlines while cleaning up timers and abort listeners after each export.

## 9.3.2

### Patch Changes

- ec97a9c: Ship CHANGELOG.md inside the published tarball of every package so release notes travel with the installed version.

## 9.3.2-rc.0

### Patch Changes

- ec97a9c: Ship CHANGELOG.md inside the published tarball of every package so release notes travel with the installed version.

## 9.3.1

### Patch Changes

- 461082f: Browser telemetry retries transient gateway failures and splits large trace and log batches before they exceed the ingestion limit. Hidden-page exports use bounded durable carryover so queued batches survive tab closure, with consent and identity cleanup.

## 9.3.0

### Minor Changes

- 67b57b8: Add `@interfere/helpers/retry` and `@interfere/helpers/retry-after`: one retry loop and one Retry-After parser (seconds or HTTP-date, optional cap). Session retry and the service-worker queue now share `isRetryableStatus`, so 408 and 425 responses are retried like 429 and 5xx.
- cb80e0a: Add `@interfere/helpers/with-deadline`: runs an operation with an `AbortSignal` that fires a `TimeoutError` after the given delay and clears the timer as soon as the operation settles. Unlike `AbortSignal.timeout`, a finished call leaves no pending timer that keeps a Worker or Durable Object invocation open.

## 9.2.1

### Patch Changes

- c4e0754: Retry telemetry exports after 429 and 503 responses within bounded deadlines, honor Retry-After delays, and allow pending retries to finish during supported flush lifecycles.

## 9.2.0

### Minor Changes

- 2ec9c4f: Share bounded, ordered asynchronous mapping between the build plugins. Limit Vite source-map discovery and rewriting to 16 concurrent files while preserving duplicate-map selection.

  Retain sequential retry and chunk-upload behavior under performance linting.

- b8b6902: retry throttled and transient source-map uploads instead of failing the release

### Patch Changes

- 63df2c9: Refresh dependency ranges. `@interfere/nest` now accepts NestJS 11 or 12 as a peer.
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

## 9.1.2

### Patch Changes

- f1dedba: improved types

## 9.1.1

### Patch Changes

- c7f7d47: Add blank lines around multiline statements and before returns, from enforcing the internal padding-between-statements lint rule repo-wide. Whitespace only, no behavior or API difference.

## 9.1.0

### Minor Changes

- 9162968: Add `@interfere/helpers/get-initials` for deriving avatar initials from a display name, falling back to an identifier.

### Patch Changes

- 42d939d: Refine `OmitUndefined<T>` so required keys stay required instead of widening every key to optional; only keys whose type includes `undefined` become optional.
- e347b65: Internal lint modernization: apply oxlint autofixes across previously quarantined rules (import specifier style, catch variable naming, DOM idiom updates, and related mechanical cleanups). No behavior changes intended.

## 9.0.3

### Patch Changes

- d5d405c: Republish to align with workspace contents.

## 9.0.2

### Patch Changes

- 475f125: Move `consent` and `plugins` configuration from `init()` to `<InterfereProvider>`. Enhance PII handling.

## 9.0.1

### Patch Changes

- ac95069: Refresh published README artifacts after markdown lint updates.
- d265875: Refresh public package artifacts after build output drift.
- 4c03108: Minor updates
- 7ba7566: Republish to swap the TypeScript devDependency from `@typescript/native-preview` to `typescript`.

## 9.0.1-canary.2

### Patch Changes

- 4c03108: Minor updates

## 9.0.1-canary.1

### Patch Changes

- ac95069: Refresh published README artifacts after markdown lint updates.

## 9.0.1-canary.0

### Patch Changes

- d265875: Refresh public package artifacts after build output drift.

## 2.0.0

### Major Changes

- normalization release

## 1.0.1

### Patch Changes

- [#1665](https://github.com/interfere-inc/interfere/pull/1665) [`01e037e`](https://github.com/interfere-inc/interfere/commit/01e037e5dc3f6a013124dab44863d87a9be93029) Thanks [@skve](https://github.com/skve)! - feat: initial release

## 1.0.0

### Major Changes

- Initial release. Extracts the shared `deepMerge`, `DeepPartial`, and `omitUndefined` helpers from `@interfere/test-utils` and `@interfere/types` into their own package so they can be consumed by runtime code without pulling in test-only dependencies.
