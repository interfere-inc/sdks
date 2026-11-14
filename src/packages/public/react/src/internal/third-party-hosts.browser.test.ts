import { isUrlIgnored } from "@opentelemetry/core";

import { THIRD_PARTY_IGNORE } from "./third-party-hosts.js";
import { describe, expect, test } from "vitest";

const ignored = (url: string) => isUrlIgnored(url, [...THIRD_PARTY_IGNORE]);

describe("third-party host ignore list", () => {
  test.each([
    "https://browser-intake-datadoghq.com/api/v2/rum?ddsource=browser",
    "https://browser-intake-us5-datadoghq.com/api/v2/replay",
    "https://rum.browser-intake-datadoghq.eu/api/v2/logs",
    "https://browser-intake-ddog-gov.com/api/v2/rum",
    "https://analytics.google.com/g/collect?v=2",
    "https://region1.analytics.google.com/g/collect",
    "https://www.google-analytics.com/g/collect",
    "https://www.google.com/rmkt/collect/17133147689/",
    "https://pixels.spotify.com/v1/ingest",
    "https://alb.reddit.com/rp",
    "https://api.vector.co/pixel/ip-info",
    "https://p.conversion.ai/api/v1/e/t",
    "https://acme.featurebase.app/api/v1/user/identify",
    "https://data.hockeystack.com/send",
    "https://r.lr-ingest.io/i",
    "https://rs.fullstory.com/rec/bundle",
    "https://api2.amplitude.com/2/httpapi",
    "https://api.segment.io/v1/t",
    "https://o123.ingest.us.sentry.io/api/1/envelope/",
    "https://us.i.posthog.com/e/",
    "https://ph.example.com/i/v0/e/?ip=0",
    "https://app.example.com/039c3dc4009cd962/vitals",
    "https://app.example.com/_vercel/insights/view",
    "https://app.example.com/.well-known/vercel/jwe",
    "https://clerk.example.com/v1/client",
  ])("ignores vendor traffic %s", (url) => {
    expect(ignored(url)).toBe(true);
  });

  test.each([
    "https://app.example.com/api/items",
    "https://api.knock.app/v1/users/1/feeds/2",
    "https://www.google.com/search?q=interfere",
    "https://notsegment.io.example.com/api",
    "https://app.example.com/api/events?ref=analytics.google.com",
    "https://app.example.com/api/view",
  ])("leaves application traffic instrumented: %s", (url) => {
    expect(ignored(url)).toBe(false);
  });
});
