# Interfere SDK

The public `@interfere/*` npm packages, published with [npm trusted publishing](https://docs.npmjs.com/trusted-publishers) and provenance.

This repository is a read-only mirror. Development, review and versioning happen in the Interfere monorepo; every push there that changes a public package replaces the contents of this repository with a fresh export. Pull requests opened here cannot be merged; please open an issue instead.

| Package                        | Directory                               |
| ------------------------------ | --------------------------------------- |
| `@interfere/react`             | `src/packages/public/react`             |
| `@interfere/next`              | `src/packages/public/next`              |
| `@interfere/vite`              | `src/packages/public/vite`              |
| `@interfere/javascript`        | `src/packages/public/javascript`        |
| `@interfere/build`             | `src/packages/public/build-tools`       |
| `@interfere/server-core`       | `src/packages/public/server-core`       |
| `@interfere/types`             | `src/packages/public/types`             |
| `@interfere/constants`         | `src/packages/public/constants`         |
| `@interfere/helpers`           | `src/packages/public/helpers`           |
| `@interfere/test-utils`        | `src/packages/public/test-utils`        |
| `@interfere/typescript-config` | `src/packages/public/typescript-config` |
| `@interfere/cli`               | `src/packages/public/cli`               |
| `@interfere/cli-<platform>`    | `src/packages/public/cli/platforms/*`   |

## Development

```sh
bun install
bun run build
bun run typecheck
bun run test
```

The root scripts skip `@interfere/cli`, whose native binary needs Rust (pinned in `rust-toolchain.toml`). Its Rust client in `src/packages/public/cli/generated` is generated from `src/packages/public/cli/fern/openapi-cli-3.1.json`; `.github/workflows/cli.yml` regenerates it to prove the export is reproducible, then builds, tests and packs the binary for every platform in `platforms.json`.

## Publishing

`.github/workflows/publish.yml` runs on every push to `main` and is composed of [changesets/action](https://github.com/changesets/action) subactions. After CI passes, a `pack` job without publish credentials plans every package whose `version` is not yet on npm, builds, restores the CLI binaries that CI built on each platform, and packs those versions in dependency order. The `publish` job, in the `npm` environment, publishes the packed tarballs through npm trusted publishing, which attaches provenance, then pushes a `<name>@<version>` tag and GitHub release for each version it published. Both jobs run only when the repository variable `NPM_PUBLISH_ENABLED` is `true`; it is declared `false` in IaC until the npm cutover.
