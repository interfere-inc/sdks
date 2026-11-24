import { waitUntil } from "cloudflare:workers";

import { runtime } from "../runtime";

/** Workers cancel a request's timers when it ends, so each finished request exports its own spans inside its lifetime. */
export function flushAfterRequest() {
  waitUntil(runtime.flush());
}
