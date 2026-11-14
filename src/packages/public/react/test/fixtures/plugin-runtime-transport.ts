import { gunzipSync } from "fflate";
import { HttpResponse, http } from "msw";

import { createCountSignal } from "./upload-signal.js";

const uploads: { request: Request; events: unknown[] }[] = [];

const uploadSignal = createCountSignal(uploads);

export const pluginTransport = {
  uploads,
  uploadsReached: uploadSignal.reached,
  handlers: [
    http.post(
      "https://plugin-runtime.invalid/v3/replay/upload/*",
      async ({ request }) => {
        if (request.headers.get("content-encoding") !== "gzip") {
          throw new Error("Expected native gzip replay upload");
        }

        const bytes = new Uint8Array(await request.clone().arrayBuffer());

        const events: unknown[] = new TextDecoder()
          .decode(gunzipSync(bytes))
          .trim()
          .split("\n")
          .map((line) => {
            const decodedValue: unknown = JSON.parse(line);

            return decodedValue;
          });

        uploads.push({ request, events });

        uploadSignal.notify();

        return HttpResponse.json({ objectKey: "replays/recorded-tail.json" });
      },
    ),
    ...[
      "/v3/session",
      "/drain/v1/traces",
      "/drain/v1/logs",
      "/drain/v1/metrics",
    ].map((path) =>
      http.post(`https://plugin-runtime.invalid${path}`, () =>
        HttpResponse.json({}),
      ),
    ),
  ],
};
