import { z } from "zod";

import { sessionIdSchema } from "./session";

/** Baggage key the v13 browser SDK uses for the Interfere session id. */
export const INTERFERE_SESSION_ID_KEY = "interfere.session.id";

/** Baggage key the v13 browser SDK uses for the consented Interfere device id. */
export const INTERFERE_DEVICE_ID_KEY = "interfere.device.id";

/** Semantic-convention key the v12 browser SDK propagates and every Interfere browser signal still carries. */
export const LEGACY_SESSION_ID_KEY = "session.id";

/** Semantic-convention key the v12 browser SDK propagates and every Interfere browser signal still carries. */
export const LEGACY_DEVICE_ID_KEY = "device.id";

const deviceIdWire = z.uuid();

/**
 * Decide whether a baggage entry may be copied onto a server span.
 *
 * Namespaced interfere.* entries are always copied. The legacy semantic-convention keys are copied
 * only while their value parses as an Interfere identifier (session: UUIDv7 or server_<uuidv7>;
 * device: UUID), so vendor baggage such as Datadog's session.id, user.id and account.id is never recorded.
 */
export function acceptsCorrelationBaggageEntry(key: string, value: string) {
  if (key.startsWith("interfere.")) {
    return true;
  }

  if (key === LEGACY_SESSION_ID_KEY) {
    return sessionIdSchema.safeParse(value).success;
  }

  return key === LEGACY_DEVICE_ID_KEY && deviceIdWire.safeParse(value).success;
}
