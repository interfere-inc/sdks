# @interfere/constants

## 9.3.3

### Patch Changes

- 13e0639: Include MIT licenses and package documentation, declare tree-shaking behavior, and document the supported Node.js and framework versions. Complete server adapter package metadata and link license badges to the included license.
- 13e0639: Remove workspace-only resolution conditions from stable and canary packages so consumers using custom conditions resolve shipped runtime and type files. Qualification tests now use the same sanitized package manifests while development source resolution stays unchanged.

## 9.3.2

### Patch Changes

- ec97a9c: Ship CHANGELOG.md inside the published tarball of every package so release notes travel with the installed version.

## 9.3.2-rc.0

### Patch Changes

- ec97a9c: Ship CHANGELOG.md inside the published tarball of every package so release notes travel with the installed version.

## 9.3.1

### Patch Changes

- 2f2934a: Enable development telemetry when a public key is configured and remove undocumented environment overrides in favor of existing SDK options and automatic defaults.

## 9.3.0

### Minor Changes

- 47943fb: Send the public key on release uploads so workspace secret keys can publish releases.

## 9.2.0

### Minor Changes

- af4aa48: Export each SDK's default OpenTelemetry `service.name` from `@interfere/constants/service-names`. The SDKs read their defaults from it, so the reported service names are unchanged.

## 9.1.0

### Minor Changes

- 1f01b42: Send telemetry to `api.interfere.com` using new api routes.

### Patch Changes

- 63df2c9: Refresh dependency ranges. `@interfere/nest` now accepts NestJS 11 or 12 as a peer.
- a653b8d: Add the generated JavaScript API client with ESM, CommonJS, and TypeScript declarations. Next and Vite call its endpoints directly for build configuration, release creation, source-map uploads, and telemetry. Remove the shared build client and its combined release-preparation method. Preserve ordered uploads, expiration checks, and cleanup only after complete server acceptance.

  Build requests use Fern's retry policy for HTTP 408, 429, and 5xx responses. Network failures and HTTP 425 are no longer retried.

  Retire the unused CLI and Speakeasy SDK packages, their generation configuration, and compatibility wrappers. Remove legacy host remapping and Collector error aliases. Update framework setup instructions to use the generated release endpoints.

- a653b8d: Generate runtime configuration, session creation, replay upload, and telemetry ingestion methods alongside the release API. Publish these runtime operations to SDK and MCP documents without adding them to HTTP documentation.

  Use generated endpoint methods throughout the framework SDKs. Authenticate browser requests through the public-key header, preserve request timeouts and unload delivery, and report failed edge telemetry requests as failed exports.

- 47bbcbf: Send browser telemetry producer versions through the `x-interfere-producer-version` header and remove the `_pv` query parameter.
- 265a458: Fail package test commands when no tests are discovered.
- 5b8542b: Preserve inferred capture attribute types and verify optional telemetry fields with native exporter tests.

  Preserve deep-merge behavior for nested values and use schema-owned field types in telemetry contracts.

  Preserve inferred release-slug types and validate baked configuration before using it.

  Treat inherited object-property names as unknown release error codes and hostnames rather than matching them as configured entries.

  Use the generated request contracts for handwritten source-map upload helpers while preserving finalize-response validation.

  Validate test application manifests before creating temporary directories so malformed manifests do not leave unused directories behind.

  Validate source maps before patching JavaScript debug IDs in Vite and Next builds, preserving the original artifacts when a map is malformed.

  Preserve special own keys such as `__proto__` as data when omitting undefined properties, without changing the result prototype.

  Validate Vite source maps before injecting debug IDs into JavaScript chunks, so malformed maps leave the chunk unchanged.

## 9.0.6

### Patch Changes

- f1dedba: improved types

## 9.0.5

### Patch Changes

- c7f7d47: Add blank lines around multiline statements and before returns, from enforcing the internal padding-between-statements lint rule repo-wide. Whitespace only, no behavior or API difference.

## 9.0.4

### Patch Changes

- 7b80d3f: Remove lint suppression comments that no longer suppress anything.
- e347b65: Internal lint modernization: apply oxlint autofixes across previously quarantined rules (import specifier style, catch variable naming, DOM idiom updates, and related mechanical cleanups). No behavior changes intended.
- 9a8843a: Bump the shared Vitest catalog from 4.1.9 to 4.1.10

## 9.0.3

### Patch Changes

- 42b109e: Republish SDK packages whose content drifted from their published versions after upstream schema changes (org entitlements). Version bump resyncs the registry with main so the publish gate passes.

## 9.0.2

### Patch Changes

- 9883610: Republish to align packed devDependency pin with the current workspace version.
- 7c6bf32: Ship CommonJS entrypoints for the NestJS server SDK dependency chain.
- ac95069: Refresh published README artifacts after markdown lint updates.
- d265875: Refresh public package artifacts after build output drift.
- 5e12a55: Republish to realign packed manifests with current catalog and workspace versions.
- 7ba7566: Republish to swap the TypeScript devDependency from `@typescript/native-preview` to `typescript`.

## 9.0.2-canary.4

### Patch Changes

- 9883610: Republish to align packed devDependency pin with the current workspace version.

## 9.0.2-canary.3

### Patch Changes

- 5e12a55: Republish to realign packed manifests with current catalog and workspace versions.

## 9.0.2-canary.2

### Patch Changes

- ac95069: Refresh published README artifacts after markdown lint updates.

## 9.0.2-canary.1

### Patch Changes

- d265875: Refresh public package artifacts after build output drift.

## 9.0.2-canary.0

### Patch Changes

- [#2107](https://github.com/interfere-inc/interfere/pull/2107) [`e62516e`](https://github.com/interfere-inc/interfere/commit/e62516eac5997c3de92160bae1052b68e2585520) Thanks [@pjsny](https://github.com/pjsny)! - Ship CommonJS entrypoints for the NestJS server SDK dependency chain.

## 9.0.1

### Patch Changes

- [#1772](https://github.com/interfere-inc/interfere/pull/1772) [`7fc320b`](https://github.com/interfere-inc/interfere/commit/7fc320b0512f4dcd222bd71b732a08ee9fad97a0) Thanks [@skve](https://github.com/skve)! - Added the ability to customize the path of the Interfere route handler (/api/interfere)

## 9.0.0

### Major Changes

- normalization release

## 8.1.2

### Patch Changes

- [`669f5e4`](https://github.com/interfere-inc/interfere/commit/669f5e40c24cf47b4335b8e51df09efe40112d29) Thanks [@skve](https://github.com/skve)! - Added telemetry for per-sdk breakdowns

## 8.1.0

### Minor Changes

- [`e3b34e8`](https://github.com/interfere-inc/interfere/commit/e3b34e8bb42717aa5058286cb5683c0d8b068497) Thanks [@skve](https://github.com/skve)! - Added automatic country code detection & arbitrary trait support

## 8.0.0

### Major Changes

- [`de35bcb`](https://github.com/interfere-inc/interfere/commit/de35bcbba7416339d95b4b98407e052046d6cce4) Thanks [@skve](https://github.com/skve)! - fixed race condition in session tracking

## 6.0.0

### Major Changes

- [`4de11e1`](https://github.com/interfere-inc/interfere/commit/4de11e17a8a774139f47f06992b554370bae7150) Thanks [@skve](https://github.com/skve)! - Added remote config to plugin enablement

## 5.0.0

### Major Changes

- [`2ef1592`](https://github.com/interfere-inc/interfere/commit/2ef1592e1b45422faf73bfd731509d6458fcfa07) Thanks [@skve](https://github.com/skve)! - User & Session Tracking support

## 4.0.0

### Major Changes

- fix bundle

## 1.0.3

### Patch Changes

- [`8102e08`](https://github.com/interfere-inc/interfere/commit/8102e0858d1e97953a62f76f0774511955c05422) Thanks [@skve](https://github.com/skve)! - Fixed catalog dependency resolution

## 1.0.2

### Patch Changes

- fix types

## 1.0.1

### Patch Changes

- [`032905b`](https://github.com/interfere-inc/interfere/commit/032905b19f2d916d4709251cf632957f127ca917) Thanks [@skve](https://github.com/skve)! - Fixed package exports

## 1.0.0

### Major Changes

- [`17fe701`](https://github.com/interfere-inc/interfere/commit/17fe701227f2a55644eb77b254888a5a0657036d) Thanks [@skve](https://github.com/skve)! - feat: release v1

### Minor Changes

- [`949b3e2`](https://github.com/interfere-inc/interfere/commit/949b3e27002091477e4fdbd37e6c77d6de4ff780) Thanks [@skve](https://github.com/skve)! - Fixed dependency graph

### Patch Changes

- [`99b741d`](https://github.com/interfere-inc/interfere/commit/99b741d9cbc336d5194a4c36acd7c8afdad6d7ba) Thanks [@skve](https://github.com/skve)! - bump

- [`bc3cdb3`](https://github.com/interfere-inc/interfere/commit/bc3cdb362c595217c1ffe70e462c1e994a3043b1) Thanks [@skve](https://github.com/skve)! - bump

- [`6e791b6`](https://github.com/interfere-inc/interfere/commit/6e791b6e05151da43259bdb026b126949e2f6470) Thanks [@skve](https://github.com/skve)! - bump

- [`4506e8d`](https://github.com/interfere-inc/interfere/commit/4506e8d301a06600345dff51c7e141e783acee52) Thanks [@skve](https://github.com/skve)! - Updated readme

- [`2847cd7`](https://github.com/interfere-inc/interfere/commit/2847cd75cd9a067e6b265864f1ed01c01ffe8a22) Thanks [@skve](https://github.com/skve)! - Improvements, Native Next.js 16 support, and huge performance improvements.

## 1.0.0-alpha.5

### Major Changes

- [`17fe701`](https://github.com/interfere-inc/interfere/commit/17fe701227f2a55644eb77b254888a5a0657036d) Thanks [@skve](https://github.com/skve)! - feat: release v1

## 0.2.0-alpha.4

### Patch Changes

- bump

## 0.2.0-alpha.3

### Patch Changes

- bump

## 0.2.0-alpha.2

### Patch Changes

- bump

## 0.2.0-alpha.1

### Patch Changes

- Fix silent initialization issues

## 0.1.0-alpha.2

### Minor Changes

- [`949b3e2`](https://github.com/interfere-inc/interfere/commit/949b3e27002091477e4fdbd37e6c77d6de4ff780) Thanks [@skve](https://github.com/skve)! - Fixed dependency graph

### Patch Changes

- [`4506e8d`](https://github.com/interfere-inc/interfere/commit/4506e8d301a06600345dff51c7e141e783acee52) Thanks [@skve](https://github.com/skve)! - Updated readme

## 0.0.2-alpha.1

### Patch Changes

- [`2847cd7`](https://github.com/interfere-inc/interfere/commit/2847cd75cd9a067e6b265864f1ed01c01ffe8a22) Thanks [@skve](https://github.com/skve)! - Improvements, Native Next.js 16 support, and huge performance improvements.

## 0.0.2-alpha.0

### Patch Changes

- [`477492c`](https://github.com/interfere-inc/repo/commit/477492c61dc69df8e270871b57483bd2bfcb7100) Thanks [@skve](https://github.com/skve)! - Progress
