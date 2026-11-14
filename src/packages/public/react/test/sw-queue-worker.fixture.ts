/// <reference lib="webworker" />
import { installIngestQueue } from "../src/internal/sw-queue.js";

declare const self: ServiceWorkerGlobalScope;

const queueName = new URL(self.location.href).searchParams.get("queue");

if (!queueName) {
  throw new Error("Missing queue name");
}

installIngestQueue({ queueName });

self.addEventListener("install", (event) => {
  event.waitUntil(self.skipWaiting());
});

self.addEventListener("activate", (event) => {
  event.waitUntil(self.clients.claim());
});
