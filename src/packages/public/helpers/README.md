<p align="center">
  <a href="https://interfere.com">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://images.interfere.com/cdn-cgi/imagedelivery/ZQ3TDZdJHXvYNyL5BNulpQ/v2/header/logo-dark/format=auto">
      <img src="https://images.interfere.com/cdn-cgi/imagedelivery/ZQ3TDZdJHXvYNyL5BNulpQ/v2/header/logo-light/format=auto" height="64" alt="Interfere">
    </picture>
  </a>
  <h1 align="center">@interfere/helpers</h1>
</p>

<p align="center">
  <a href="https://www.npmjs.com/package/@interfere/helpers"><img src="https://img.shields.io/npm/v/@interfere/helpers.svg" alt="npm version" /></a>
  <a href="./LICENSE"><img src="https://img.shields.io/npm/l/@interfere/helpers.svg" alt="License" /></a>
</p>

<p align="center">
  Small, framework-agnostic helpers shared across <a href="https://interfere.com">Interfere</a> packages.
</p>

---

## Installation

```bash
npm install @interfere/helpers
```

## Subpath exports

- `@interfere/helpers/deep-merge` — `deepMerge(base, ...overrides)` for recursively merging plain objects.
- `@interfere/helpers/deep-partial` — the `DeepPartial<T>` type utility.
- `@interfere/helpers/omit-undefined` — `omitUndefined(value)` and its matching `OmitUndefined<T>` type.

## License

MIT

## Supported runtimes

Node.js 24.x is the supported Node.js release line and is exercised in CI.
