import { propagation } from "@opentelemetry/api";
import { z } from "zod";

import { getGlobal } from "../../../util/global.js";

function callable<T>() {
  return z.custom<T>(
    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Vendor globals expose methods and URL matchers as functions; only a callable can be one.
    (value) => typeof value === "function",
  );
}

const urlMatcherSchema = z.union([
  z.string().transform((text) => ({ kind: "text" as const, text })),
  z
    .instanceof(RegExp)
    .transform((pattern) => ({ kind: "pattern" as const, pattern })),
  callable<(url: string) => boolean>().transform((test) => ({
    kind: "function" as const,
    test,
  })),
]);

const datadogRumSchema = z.object({
  getInitConfiguration: callable<() => object | undefined>(),
});

const datadogConfigSchema = z.object({
  allowedTracingUrls: z
    .array(
      z.union([
        z.object({
          match: urlMatcherSchema,
          propagatorTypes: z.array(z.string()).optional(),
        }),
        urlMatcherSchema.transform((match) => ({
          match,
          propagatorTypes: undefined,
        })),
      ]),
    )
    .optional(),
});

const sentryCarrierSchema = z.looseObject({ version: z.string() });

const sentryHubSchema = z.object({
  defaultCurrentScope: z.object({
    getClient: callable<() => object | undefined>(),
  }),
});

const sentryClientSchema = z.object({
  getOptions: callable<() => object | undefined>(),
});

const sentryOptionsSchema = z.object({
  propagateTraceparent: z.boolean().optional(),
  tracePropagationTargets: z.array(urlMatcherSchema).optional(),
});

function matches(
  matcher: z.output<typeof urlMatcherSchema>,
  url: string,
  prefix: boolean,
) {
  if (matcher.kind === "pattern") {
    return matcher.pattern.test(url);
  }

  if (matcher.kind === "text") {
    return prefix ? url.startsWith(matcher.text) : url.includes(matcher.text);
  }

  try {
    return matcher.test(url);
  } catch {
    return false;
  }
}

function datadogWrites(url: string) {
  const rum = datadogRumSchema.safeParse(getGlobal("DD_RUM")).data;

  const config = datadogConfigSchema.safeParse(
    rum?.getInitConfiguration(),
  ).data;

  return (config?.allowedTracingUrls ?? []).some(
    (option) =>
      (option.propagatorTypes?.includes("tracecontext") ?? true) &&
      matches(option.match, url, true),
  );
}

function sentryWrites(url: string) {
  const carrier = sentryCarrierSchema.safeParse(getGlobal("__SENTRY__")).data;

  const hub = sentryHubSchema.safeParse(
    carrier ? carrier[carrier.version] : undefined,
  ).data;

  const client = sentryClientSchema.safeParse(
    hub?.defaultCurrentScope.getClient(),
  ).data;

  const options = sentryOptionsSchema.safeParse(client?.getOptions()).data;

  if (!options?.propagateTraceparent) {
    return false;
  }

  const targets = options.tracePropagationTargets;

  if (!targets) {
    return new URL(url).origin === globalThis.location.origin;
  }

  return targets.some((target) => matches(target, url, false));
}

/**
 * The tool that will write `traceparent` on a request to `url`, read from
 * that tool's own configuration when the request is sent, so a tool
 * initialised after Interfere is found from its first request. Interfere
 * never registers a global propagator, so any registered one is another
 * tool's. Datadog RUM
 * appends its header unconditionally, so writing ours first would send two.
 */
export function traceContextWriterFor(url: string) {
  try {
    if (datadogWrites(url)) {
      return "datadog";
    }

    if (sentryWrites(url)) {
      return "sentry";
    }
  } catch {
    return;
  }

  if (propagation.fields().includes("traceparent")) {
    return "opentelemetry";
  }

  return;
}
