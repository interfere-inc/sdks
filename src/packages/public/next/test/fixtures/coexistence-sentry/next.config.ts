import { withSentryConfig } from "@sentry/nextjs/config";
import { withInterfere } from "@interfere/next/config";
import { parse } from "node:path";

export default withInterfere(
  withSentryConfig(
    {
      experimental: { cpus: 2 },
      turbopack: { root: parse(import.meta.dirname).root },
    },
    {
      silent: true,
      telemetry: false,
      sourcemaps: { disable: true },
      release: { create: false, finalize: false },
    },
  ),
);
