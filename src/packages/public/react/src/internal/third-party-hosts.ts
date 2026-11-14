const VENDOR_HOSTS = [
  "analytics.google.com",
  "google-analytics.com",
  "googleadservices.com",
  "googlesyndication.com",
  "googletagmanager.com",
  "googleapis.com",
  "doubleclick.net",
  "facebook.com",
  "facebook.net",
  "fbcdn.net",
  "px.ads.linkedin.com",
  "snap.licdn.com",
  "analytics.tiktok.com",
  "ads-twitter.com",
  "analytics.twitter.com",
  "bat.bing.com",
  "clarity.ms",
  "alb.reddit.com",
  "pixel-config.reddit.com",
  "ct.pinterest.com",
  "q.quora.com",
  "pixels.spotify.com",
  "sentry.io",
  "posthog.com",
  "intercom.io",
  "intercomcdn.com",
  "bam.nr-data.net",
  "bugsnag.com",
  "rollbar.com",
  "logrocket.io",
  "logrocket.com",
  "lr-ingest.io",
  "lr-ingest.com",
  "fullstory.com",
  "amplitude.com",
  "segment.io",
  "segment.com",
  "hotjar.com",
  "hotjar.io",
  "mixpanel.com",
  "heapanalytics.com",
  "pendo.io",
  "hs-analytics.net",
  "track.hubspot.com",
  "hockeystack.com",
  "vector.co",
  "conversion.ai",
  "featurebase.app",
  "events.launchdarkly.com",
  "events.mapbox.com",
  "litix.io",
  "cloudflareinsights.com",
  "vitals.vercel-insights.com",
  "plausible.io",
  "usefathom.com",
];

function escapeRegExp(value: string) {
  return value.replaceAll(/[.*+?^${}()|[\]\\]/gu, String.raw`\$&`);
}

function hostPattern(host: string) {
  return new RegExp(
    String.raw`\/\/(?:[^/?#@]*@)?(?:[^/?#.]+\.)*${escapeRegExp(host)}(?::\d+)?(?:[/?#]|$)`,
    "iu",
  );
}

/**
 * Requests and resource loads the SDK never surfaces: analytics, ads,
 * replay, monitoring and auth vendors that otherwise turn into orphan-span
 * and resource-error noise. A vendor host matches itself and any subdomain;
 * the path patterns catch vendors commonly proxied through the customer's
 * own origin. App-specific noise belongs in `tracing.ignoreUrls`.
 *
 * Dependency-free on purpose so the error-only bundle can reuse it without
 * pulling in the OTel SDK that `internal/otel/*` carries.
 */
export const THIRD_PARTY_IGNORE: readonly RegExp[] = [
  ...VENDOR_HOSTS.map((host) => hostPattern(host)),
  /(?:^|\/\/)clerk\./u,
  /\.clerk\./u,
  /\/\/(?:[^/?#]+\.)?browser-intake-[\w.-]*datadoghq\.(?:com|eu)(?:[:/?#]|$)/iu,
  /\/\/(?:[^/?#]+\.)?browser-intake-[\w.-]*ddog-gov\.com(?:[:/?#]|$)/iu,
  /\/\/(?:www\.)?google\.[a-z.]+\/(?:rmkt|pagead|ccm|ads)\//iu,
  /\/_vercel\/(?:insights|speed-insights)\//u,
  /\/\.well-known\/vercel\//u,
  /\/[\da-f]{16}\/(?:view|vitals|event|script\.js)(?:[?#]|$)/u,
  /\/i\/v0\/e\/?(?:[?#]|$)/u,
  /\/(?:decide|flags)\/?\?v=/u,
  /\/__nextjs_/u,
];
