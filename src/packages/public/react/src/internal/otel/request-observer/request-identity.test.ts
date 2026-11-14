import { ROOT_CONTEXT, trace } from "@opentelemetry/api";
import { expect, test } from "vitest";
import {
  requestIdentity,
  snapshotRequestIdentity,
} from "./request-identity.js";

test("request identity survives a delayed correlation and changed active span without inventing a device", () => {
  let session = "initial-session";
  let device: string | null = null;

  const initial = snapshotRequestIdentity(ROOT_CONTEXT, {
    getSessionId: () => session,
    getDeviceId: () => device,
  });

  session = "rotated-session";

  device = "new-device";

  const propagated = trace.setSpanContext(initial, {
    traceId: "1234567890abcdef1234567890abcdef",
    spanId: "1234567890abcdef",
    traceFlags: 1,
  });

  expect(requestIdentity(propagated)).toStrictEqual({
    "session.id": "initial-session",
  });

  expect(requestIdentity(ROOT_CONTEXT)).toBeUndefined();
});
