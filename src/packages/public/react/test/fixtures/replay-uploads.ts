import { gunzipSync } from "fflate";
import { HttpResponse, http } from "msw";

import { createCountSignal, createMatchSignal } from "./upload-signal.js";

const requests: Request[] = [];

const uploads: {
  request: Request;
  text: string;
  events: unknown[];
  metadata: unknown;
}[] = [];

const uploadSignal = createCountSignal(uploads);
const uploadMatchSignal = createMatchSignal(uploads);
const requestSignal = createCountSignal(requests);

async function record(request: Request) {
  requests.push(request);

  requestSignal.notify();

  const bytes = new Uint8Array(await request.clone().arrayBuffer());

  const text = new TextDecoder().decode(
    request.headers.get("content-encoding") === "gzip"
      ? gunzipSync(bytes)
      : bytes,
  );

  const lines = text.trim();

  const events: unknown[] = lines
    ? lines.split("\n").map((line) => {
        const decodedValue: unknown = JSON.parse(line);

        return decodedValue;
      })
    : [];

  const metadata: unknown = JSON.parse(
    atob(request.headers.get("x-replay-meta") ?? ""),
  );

  uploads.push({ request, text, events, metadata });

  uploadSignal.notify();

  uploadMatchSignal.notify();
}

export const replayUploadUrl = "https://replay.invalid/v3/replay/upload/:id";

function reset() {
  requests.length = 0;

  uploads.length = 0;
}

export const replayUploads = {
  requests,
  uploads,
  record,
  reset,
  uploadsReached: uploadSignal.reached,
  uploadMatching: uploadMatchSignal.reached,
  requestsReached: requestSignal.reached,
};

export const replayHandlers = [
  http.post("https://replay.invalid/v3/session", () => HttpResponse.json({})),
  http.post(replayUploadUrl, async ({ request }) => {
    await record(request);

    return HttpResponse.json({ objectKey: "replays/recorded.json" });
  }),
];
