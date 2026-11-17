import type { PageSessionId, SessionId } from "./session.ts";

const UUIDV7 =
  "[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-7[0-9a-fA-F]{3}-[89abAB][0-9a-fA-F]{3}-[0-9a-fA-F]{12}";

/** Regex source for the wire session shape, kept in step with sessionIdSchema so SQL readers apply the same rule. */
export const sessionIdWirePattern = `^(server_)?${UUIDV7}$`;

const sessionIdPattern = new RegExp(sessionIdWirePattern, "u");

const pageSessionIdPattern = new RegExp(`^page_${UUIDV7}$`, "u");

/** Whether a stored value is a wire session id, without loading zod into browser bundles. */
export function isSessionId(
  value: string | null | undefined,
): value is SessionId {
  return value !== undefined && value !== null && sessionIdPattern.test(value);
}

function isPageSessionId(value: string): value is PageSessionId {
  return pageSessionIdPattern.test(value);
}

/** Brands a freshly minted UUIDv7 as a session id, throwing when it is not one. */
export function toSessionId(value: string) {
  if (!isSessionId(value)) {
    throw new TypeError(`Invalid session id: ${value}`);
  }

  return value;
}

/** Brands a freshly minted `page_<uuidv7>` as a page session id, throwing when it is not one. */
export function toPageSessionId(value: string) {
  if (!isPageSessionId(value)) {
    throw new TypeError(`Invalid page session id: ${value}`);
  }

  return value;
}
