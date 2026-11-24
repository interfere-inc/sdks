# @interfere/cli

## 1.21.0

### Minor Changes

- Regenerate CLI commands and API types from the current API schema.

## 1.20.0

### Minor Changes

- Regenerate CLI commands and API types from the current API schema.

## 1.19.0

### Minor Changes

- Regenerate CLI commands and API types from the current API schema.

## 1.18.0

### Minor Changes

- Regenerate CLI commands and API types from the current API schema.

## 1.17.0

### Minor Changes

- Regenerate CLI commands and API types from the current API schema.

### Patch Changes

- 4d63981: Every public package is now published with npm trusted publishing and a provenance attestation.
- 28f75f2: The `@interfere/cli` and `@interfere/vite` READMEs now describe sending telemetry by service, with a workspace secret key for servers, build pipelines and collectors.
- c3e1497: The `@interfere/vite` and `@interfere/cli` READMEs drop their outdated description of how telemetry from a newly seen service is routed.

## 1.16.0

### Minor Changes

- Regenerate CLI commands and API types from the current API schema.

## 1.15.0

### Minor Changes

- Regenerate CLI commands and API types from the current API schema.

### Patch Changes

- 91670c6: The `completion` and `man` help examples now name the `interfere` binary instead of a `<CLI>` placeholder, so `interfere completion zsh > "${fpath[1]}/_interfere"` can be copied as shown.

## 1.14.0

### Minor Changes

- Regenerate CLI commands and API types from the current API schema.

## 1.13.0

### Minor Changes

- Regenerate CLI commands and API types from the current API schema.
- 036525e: Regenerate CLI commands and API types from the current API schema.
- 895611e: Regenerate CLI commands and API types from the current API schema.
- 2b1324c: Regenerate CLI commands and API types from the current API schema.
- 1258d51: Regenerate CLI commands and API types from the current API schema.
- d1c64bc: Regenerate CLI commands and API types from the current API schema.
- cdecd28: Regenerate CLI commands and API types from the current API schema.

### Patch Changes

- 420d2c0: Publish the repository field so npm links the source directory.
- ec97a9c: Ship CHANGELOG.md inside the published tarball of every package so release notes travel with the installed version.
- a49895d: Document workspace keys, App routing, service naming, migration, and auto-create controls.

## 1.13.0-rc.5

### Minor Changes

- Regenerate CLI commands and API types from the current API schema.

## 1.13.0-rc.4

### Minor Changes

- Regenerate CLI commands and API types from the current API schema.

### Patch Changes

- 420d2c0: Publish the repository field so npm links the source directory.

## 1.13.0-rc.3

### Minor Changes

- Regenerate CLI commands and API types from the current API schema.

### Patch Changes

- a49895d: Document workspace keys, App routing, service naming, migration, and auto-create controls.

## 1.13.0-rc.2

### Minor Changes

- Regenerate CLI commands and API types from the current API schema.

### Patch Changes

- ec97a9c: Ship CHANGELOG.md inside the published tarball of every package so release notes travel with the installed version.

## 1.13.0-rc.1

### Minor Changes

- Regenerate CLI commands and API types from the current API schema.

## 1.13.0-rc.0

### Minor Changes

- Regenerate CLI commands and API types from the current API schema.

## 1.12.0

### Minor Changes

- Regenerate CLI commands and API types from the current API schema.

## 1.11.0

### Minor Changes

- Regenerate CLI commands and API types from the current API schema.

## 1.10.0

### Minor Changes

- Regenerate CLI commands and API types from the current API schema.

## 1.9.0

### Minor Changes

- Regenerate CLI commands and API types from the current API schema.

## 1.8.0

### Minor Changes

- Regenerate CLI commands and API types from the current API schema.

## 1.7.0

### Minor Changes

- Regenerate CLI commands and API types from the current API schema.

## 1.6.0

### Minor Changes

- Regenerate CLI commands and API types from the current API schema.

## 1.5.0

### Minor Changes

- Regenerate CLI commands and API types from the current API schema.

## 1.4.0

### Minor Changes

- Regenerate CLI commands and API types from the current API schema.

## 1.3.0

### Minor Changes

- Regenerate CLI commands and API types from the current API schema.

## 1.2.0

### Minor Changes

- Regenerate CLI commands and API types from the current API schema.

## 1.1.0

### Minor Changes

- f9d411d: Regenerate the CLI from the current API schema. Add application user and company queries, investigation admission and previous-account queries, replay chunk indexes, OpenTelemetry setup, service activity, and SDK status. Refresh connection and source-map request/response types and help text. Remove the obsolete problem archive command; use dismiss and restore for the current problem lifecycle. Publish matching native binaries for all five supported platforms.
- 8276f44: Add `interfere login --device`, the OAuth device authorization flow for headless and remote machines. `interfere auth login` now falls back to it automatically over SSH, in CI, or without a display, and both commands store the credential in the same place as browser sign-in.
- Regenerate CLI commands and API types from the current API schema.

## 1.0.0

### Major Changes

- 78bf77a: Replace the CLI with a native command-line client generated from the public API contract. Add browser authentication, workspace profiles, machine-readable command discovery, request previews, and problem inspection. Distribute native binaries through platform-specific npm packages.
