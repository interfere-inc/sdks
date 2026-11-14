import { z } from "zod";

/** Version of the capture protocol the current backend speaks; bumped only for an incompatible change. */
export const CAPTURE_PROTOCOL_VERSION = 1;

export const protocolVersionSchema = z.int().positive().meta({
  description:
    "Capture protocol version. Additive fields never bump it; readers tolerate versions they do not know.",
  id: "CaptureProtocolVersion",
  title: "CaptureProtocolVersion",
});
