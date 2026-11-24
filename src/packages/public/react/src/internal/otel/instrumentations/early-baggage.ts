import {
  INTERFERE_DEVICE_ID_KEY,
  INTERFERE_SESSION_ID_KEY,
} from "@interfere/types/data/correlation-baggage";

import type { PropagationPolicy } from "./header-policy.js";
import { readDenied } from "./denied-origins.js";

export interface EarlyBaggageInput {
  getDeviceId: () => string | null;
  getSessionId: () => string | null;
  /** Requests whose absolute URL equals a string entry or matches a RegExp entry carry nothing. */
  ignoreUrls: readonly (string | RegExp)[];
  policy: PropagationPolicy;
}

export interface EarlyBaggage {
  /** Restores the patched `fetch` and `XMLHttpRequest` methods that nothing has wrapped since, and stops stamping through any that remain. */
  dispose: () => void;
}

type OpenArgs = [
  method: string,
  url: string | URL,
  async?: boolean,
  username?: string | null,
  password?: string | null,
];

function member(key: string, value: string | null) {
  return value
    ? [`${encodeURIComponent(key)}=${encodeURIComponent(value)}`]
    : [];
}

/** Appends session baggage to `fetch` and `XMLHttpRequest` requests for explicit, undenied origins whose policy includes `baggage`, until disposed. */
export function installEarlyBaggage(input: EarlyBaggageInput) {
  const denied = readDenied();

  let active = true;

  const baggageFor = (url: string) => {
    if (!active) {
      return;
    }

    let target: URL;

    try {
      target = new URL(url, globalThis.location.href);
    } catch {
      return;
    }

    const entry = input.policy.get(target.origin);

    if (
      !entry ||
      entry.discovered ||
      !entry.headers.has("baggage") ||
      denied.has(target.origin) ||
      input.ignoreUrls.some((pattern) =>
        pattern instanceof RegExp
          ? pattern.test(target.href)
          : pattern === target.href,
      )
    ) {
      return;
    }

    const members = [
      ...member(INTERFERE_SESSION_ID_KEY, input.getSessionId()),
      ...member(INTERFERE_DEVICE_ID_KEY, input.getDeviceId()),
    ];

    return members.length > 0 ? members.join(",") : undefined;
  };

  const fetchBefore = globalThis.fetch;

  const fetchOurs = new Proxy(fetchBefore, {
    async apply(
      target,
      receiver: typeof globalThis | null | undefined,
      args: Parameters<typeof globalThis.fetch>,
    ) {
      const [resource, init] = args;

      const value = baggageFor(
        resource instanceof Request ? resource.url : String(resource),
      );

      if (value === undefined) {
        return target.apply(receiver, args);
      }

      const headers = new Headers(
        init?.headers ??
          (resource instanceof Request ? resource.headers : undefined),
      );

      headers.append("baggage", value);

      return target.call(receiver, resource, { ...init, headers });
    },
  });

  globalThis.fetch = fetchOurs;

  const xhr = XMLHttpRequest.prototype;
  // oxlint-disable-next-line typescript/unbound-method -- Restored by identity on disposal.
  const openBefore = xhr.open;
  // oxlint-disable-next-line typescript/unbound-method -- Restored by identity on disposal.
  const sendBefore = xhr.send;

  const urls = new WeakMap<XMLHttpRequest, string>();

  function openOurs(this: XMLHttpRequest, ...args: OpenArgs) {
    urls.set(this, String(args[1]));

    const open: (this: XMLHttpRequest, ...values: OpenArgs) => void =
      openBefore;

    open.apply(this, args);
  }

  function sendOurs(
    this: XMLHttpRequest,
    ...args: Parameters<typeof sendBefore>
  ) {
    const url = urls.get(this);

    const value =
      url === undefined || this.readyState !== XMLHttpRequest.OPENED
        ? undefined
        : baggageFor(url);

    if (value !== undefined) {
      this.setRequestHeader("baggage", value);
    }

    const send: (this: XMLHttpRequest, ...values: typeof args) => void =
      sendBefore;

    send.apply(this, args);
  }

  xhr.open = openOurs;

  xhr.send = sendOurs;

  return {
    dispose: () => {
      active = false;

      if (globalThis.fetch === fetchOurs) {
        globalThis.fetch = fetchBefore;
      }

      if (xhr.open === openOurs) {
        xhr.open = openBefore;
      }

      if (xhr.send === sendOurs) {
        xhr.send = sendBefore;
      }
    },
  } satisfies EarlyBaggage;
}
