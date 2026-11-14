/// <reference lib="webworker" />

import { installIngestQueue } from "@interfere/react/sw-runtime";

declare const self: ServiceWorkerGlobalScope;

installIngestQueue({
  queueName: "acceptance",
  match: (url) =>
    url.pathname.startsWith("/drain/") ||
    url.pathname.startsWith("/v3/session"),
});

self.addEventListener("install", (event) =>
  event.waitUntil(self.skipWaiting()),
);

self.addEventListener("activate", (event) =>
  event.waitUntil(self.clients.claim()),
);
