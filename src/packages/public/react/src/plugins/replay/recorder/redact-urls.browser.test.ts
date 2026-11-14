import {
  redactUrl,
  resolveUrlRedaction,
} from "@interfere/types/sdk/url-redaction";

import { EventType, IncrementalSource } from "@rrweb/types";
import type { eventWithTime } from "@rrweb/types";

import { toRecordOptions } from "../config.js";
import { startReplayRecorder } from "./recorder.js";
import { redactReplayEvent } from "./redact-urls.js";
import { expect, onTestFinished, test, vi } from "vitest";

const SECRET = "replay-secret";

test("redacts page, link, image and mutation URLs in recorded replay events", async () => {
  expect.hasAssertions();

  const priorUrl = location.href;

  history.replaceState(history.state, "", `/replay?token=${SECRET}&page=2`);

  const host = document.createElement("div");

  host.innerHTML = `
    <a href="/orders?token=${SECRET}&tab=open">Orders</a>
    <img src="/avatar.png?X-Amz-Signature=${SECRET}" srcset="/a.png?sig=${SECRET} 1x, /b.png?sig=${SECRET} 2x">
    <form action="/submit?session_id=${SECRET}"></form>
  `;

  document.body.append(host);

  const policy = resolveUrlRedaction(undefined);
  const events: eventWithTime[] = [];

  const recorder = await startReplayRecorder({
    onEvent: (event) => {
      redactReplayEvent(event, (url) => redactUrl(url, policy));

      events.push(event);
    },
    onFault: () => undefined,
    onMutationStorm: () => undefined,
    recordOptions: toRecordOptions({}),
  });

  onTestFinished(() => {
    recorder?.stop();

    host.remove();

    history.replaceState(history.state, "", priorUrl);
  });

  const late = document.createElement("a");

  late.href = `/late?access_token=${SECRET}`;

  host.append(late);

  host.querySelector("a")?.setAttribute("href", `/changed?token=${SECRET}`);

  await vi.waitFor(() => {
    expect(
      events.some(
        (event) =>
          event.type === EventType.IncrementalSnapshot &&
          event.data.source === IncrementalSource.Mutation,
      ),
    ).toBe(true);
  });

  const recorded = JSON.stringify(events);

  expect(recorded).not.toContain(SECRET);

  expect(recorded).toContain("/replay?token=REDACTED&page=2");

  expect(recorded).toContain("/orders?token=REDACTED&tab=open");

  expect(recorded).toContain("/late?access_token=REDACTED");

  expect(recorded).toContain("/changed?token=REDACTED");

  expect(recorded).toContain("/b.png?sig=REDACTED 2x");
});
