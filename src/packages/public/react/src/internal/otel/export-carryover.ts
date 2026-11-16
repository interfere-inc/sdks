import { TELEMETRY_EXPORT_TIMEOUT_MS } from "@interfere/helpers/retry-telemetry";
import type { ExportResult } from "@opentelemetry/core";
import { ExportResultCode } from "@opentelemetry/core";
import { z } from "zod";
import { toError } from "@interfere/types/sdk/errors";

import { isPageHidden, onPageShown } from "../page-lifecycle/page-lifecycle.js";
import { isPageLeaving } from "../page-lifecycle/unloading.js";
import { onKeepaliveBodyReleased } from "../page-lifecycle/keepalive-budget.js";
import type { SignalSender } from "./exporter.js";
import {
  entrySchema,
  isTerminal,
  markDispatched,
} from "./transport/carryover-entry.js";
import type { RetainedSend } from "./transport/carryover-entry.js";

export type { RetainedSend } from "./transport/carryover-entry.js";

const PREFIX = "interfere:export:v1:";
const RESET_KEY = "interfere:export-reset:v1";
const MAX_BYTES = 1024 * 1024;
const MAX_ENTRIES = 64;
const MAX_AGE_MS = 24 * 60 * 60 * 1000;
const REPLAY_HOLD_MS = TELEMETRY_EXPORT_TIMEOUT_MS;

const encoder = new TextEncoder();
let generation = 0;

let lastCreated = 0;

const callbacks = new Map<
  string,
  { scope: string; settle: (result: ExportResult) => void }
>();

const controllers = new Map<AbortController, string>();
const inFlight = new Set<string>();

function settleEntry(key: string, result: ExportResult) {
  const terminal = isTerminal(result);

  if (result.code === ExportResultCode.SUCCESS || terminal) {
    localStorage.removeItem(key);
  } else {
    markDispatched(key, undefined);
  }

  callbacks
    .get(key)
    ?.settle(terminal ? result : { code: ExportResultCode.SUCCESS });

  callbacks.delete(key);

  return result.code === ExportResultCode.SUCCESS || terminal;
}

function storageKeys() {
  return Array.from({ length: localStorage.length }, (_, index) =>
    localStorage.key(index),
  ).filter((key): key is string => key !== null);
}

function entries() {
  const result = [];

  for (const key of storageKeys()) {
    if (!key.startsWith(PREFIX)) {
      continue;
    }

    let value: unknown;

    try {
      value = JSON.parse(localStorage.getItem(key) ?? "null");
    } catch {
      localStorage.removeItem(key);

      continue;
    }

    const entry = entrySchema.safeParse(value);

    if (!entry.success || Date.now() - entry.data.created > MAX_AGE_MS) {
      localStorage.removeItem(key);

      continue;
    }

    result.push({ ...entry.data, key });
  }

  // oxlint-disable-next-line unicorn/no-array-sort -- Sort this private array in place; toSorted is outside the shipped browser baseline.
  return result.sort((a, b) => a.created - b.created);
}

function cancelPending(scopePrefix = "") {
  generation += 1;

  for (const [controller, scope] of controllers) {
    if (scope.startsWith(scopePrefix)) {
      controller.abort();
    }
  }

  for (const [key, { scope, settle }] of callbacks) {
    if (!scope.startsWith(scopePrefix)) {
      continue;
    }

    callbacks.delete(key);

    settle({
      code: ExportResultCode.FAILED,
      error: new Error("Telemetry carryover cleared"),
    });
  }
}

// oxlint-disable-next-line anti-slop/no-runtime-typeof -- Server hosts have no browser storage events.
if (typeof globalThis.addEventListener === "function") {
  globalThis.addEventListener("storage", (event: StorageEvent) => {
    if (event.key === RESET_KEY && event.newValue !== null) {
      let scopePrefix = "";

      try {
        const reset = z
          .object({ scopePrefix: z.string() })
          .safeParse(JSON.parse(event.newValue));

        if (reset.success) {
          ({ scopePrefix } = reset.data);
        }
      } catch {
        scopePrefix = "";
      }

      cancelPending(scopePrefix);
    }
  });
}

/** Removes retained exports when consent is revoked or identity is cleared. */
export function clearExportCarryover(scopePrefix = "") {
  cancelPending(scopePrefix);

  try {
    for (const entry of entries()) {
      if (entry.scope.startsWith(scopePrefix)) {
        localStorage.removeItem(entry.key);
      }
    }

    localStorage.setItem(
      RESET_KEY,
      JSON.stringify({ scopePrefix, id: crypto.randomUUID() }),
    );

    localStorage.removeItem(RESET_KEY);
  } catch {
    return;
  }
}

/** Retains hidden-page exports until acknowledged, bounded by age, count and bytes; every send of a retained export carries its stable carryover id so the gateway can drop a replay it already accepted. */
export function withExportCarryover(
  scope: string,
  send: SignalSender,
  allowed: () => boolean,
  shouldPersist: () => boolean = isPageHidden,
) {
  let disposed = false;
  let draining = false;
  let wakeRequested = false;
  let heldTimer: ReturnType<typeof setTimeout> | undefined;
  let budgetBlocked = false;

  function retainedSend(key: string) {
    budgetBlocked = false;

    return {
      id: key.slice(PREFIX.length),
      onKeepalive: () => {
        markDispatched(key, Date.now());
      },
      onBudgetExhausted: () => {
        budgetBlocked = true;
      },
    } satisfies RetainedSend;
  }

  const drain = async () => {
    if (
      disposed ||
      !allowed() ||
      !("navigator" in globalThis) ||
      !("locks" in navigator) ||
      draining
    ) {
      return;
    }

    const currentGeneration = generation;

    draining = true;

    try {
      await navigator.locks.request(PREFIX + scope, async () => {
        for (const [key, pending] of callbacks) {
          if (pending.scope === scope && !localStorage.getItem(key)) {
            pending.settle({ code: ExportResultCode.SUCCESS });

            callbacks.delete(key);
          }
        }

        for (const entry of entries().filter((item) => item.scope === scope)) {
          if (disposed || !allowed() || currentGeneration !== generation) {
            break;
          }

          if (inFlight.has(entry.key) || !localStorage.getItem(entry.key)) {
            continue;
          }

          const heldFor =
            entry.dispatched === undefined || callbacks.has(entry.key)
              ? 0
              : entry.dispatched + REPLAY_HOLD_MS - Date.now();

          if (heldFor > 0) {
            clearTimeout(heldTimer);

            heldTimer = setTimeout(() => {
              void drain();
            }, heldFor);

            continue;
          }

          const controller = new AbortController();

          controllers.set(controller, scope);

          // oxlint-disable-next-line no-await-in-loop -- Drain in FIFO order under the cross-tab lock and stay within the keepalive budget.
          const result = await new Promise<ExportResult>((resolve) => {
            send(
              encoder.encode(entry.body),
              resolve,
              controller.signal,
              retainedSend(entry.key),
            );
          });

          controllers.delete(controller);

          if (!settleEntry(entry.key, result)) {
            for (const pending of entries().filter(
              (item) => item.scope === scope && !inFlight.has(item.key),
            )) {
              callbacks
                .get(pending.key)
                ?.settle({ code: ExportResultCode.SUCCESS });

              callbacks.delete(pending.key);
            }

            break;
          }
        }
      });
    } catch (error) {
      for (const [key, pending] of callbacks) {
        if (pending.scope !== scope) {
          continue;
        }

        pending.settle({
          code: ExportResultCode.FAILED,
          error: toError(error),
        });

        callbacks.delete(key);
      }
    } finally {
      draining = false;

      const shouldWake = wakeRequested;

      wakeRequested = false;

      if (
        shouldWake ||
        currentGeneration !== generation ||
        [...callbacks].some(
          ([key, pending]) => pending.scope === scope && !inFlight.has(key),
        )
      ) {
        void drain();
      }
    }
  };

  // WARNING: A drain that ends through its failure path settles every pending callback and parks the rest, so a page shown while it runs must be remembered or the parked exports wait for the next lifecycle event.
  const wake = () => {
    if (draining) {
      wakeRequested = true;
    }

    void drain();
  };

  const unsubscribe = onPageShown(wake);

  const unsubscribeBudget = onKeepaliveBodyReleased(() => {
    if (budgetBlocked) {
      budgetBlocked = false;

      wake();
    }
  });

  queueMicrotask(() => {
    void drain();
  });

  function retain(
    payload: Uint8Array,
    callback: (result: ExportResult) => void,
  ) {
    const body = new TextDecoder().decode(payload);
    const retained = entries();

    const bytes = retained.reduce(
      (total, entry) => total + encoder.encode(entry.body).byteLength,
      payload.byteLength,
    );

    if (retained.length >= MAX_ENTRIES || bytes > MAX_BYTES) {
      callback({
        code: ExportResultCode.FAILED,
        error: new Error("Telemetry carryover capacity exceeded"),
      });

      return null;
    }

    const key = PREFIX + crypto.randomUUID();

    lastCreated = Math.max(Date.now(), lastCreated + 0.001);

    localStorage.setItem(
      key,
      JSON.stringify({ body, created: lastCreated, scope }),
    );

    const afterWrite = entries();

    if (
      afterWrite.length > MAX_ENTRIES ||
      afterWrite.reduce(
        (total, entry) => total + encoder.encode(entry.body).byteLength,
        0,
      ) > MAX_BYTES
    ) {
      localStorage.removeItem(key);

      callback({
        code: ExportResultCode.FAILED,
        error: new Error("Telemetry carryover capacity exceeded"),
      });

      return null;
    }

    callbacks.set(key, { scope, settle: callback });

    return key;
  }

  function retainOnFailure(
    payload: Uint8Array,
    callback: (result: ExportResult) => void,
  ) {
    try {
      if (retain(payload, callback) === null) {
        return false;
      }
    } catch {
      return false;
    }

    void drain();

    return true;
  }

  function canRetain(payload: Uint8Array | undefined): payload is Uint8Array {
    return payload !== undefined && allowed() && "locks" in navigator;
  }

  return {
    dispose: () => {
      disposed = true;

      clearTimeout(heldTimer);

      unsubscribe();

      unsubscribeBudget();
    },
    // WARNING: An unloading page is torn down before navigator.locks grants the drain lock, so a payload that must leave with the page goes through deliver, not send.
    deliver: ((payload, callback) => {
      if (!canRetain(payload)) {
        send(payload, callback);

        return;
      }

      let key: string | null;

      try {
        key = retain(payload, callback);
      } catch {
        send(payload, callback);

        return;
      }

      if (key === null) {
        return;
      }

      const retainedKey = key;
      const controller = new AbortController();

      inFlight.add(retainedKey);

      controllers.set(controller, scope);

      send(
        payload,
        (result) => {
          controllers.delete(controller);

          inFlight.delete(retainedKey);

          if (callbacks.has(retainedKey)) {
            settleEntry(retainedKey, result);
          }
        },
        controller.signal,
        retainedSend(retainedKey),
      );
    }) satisfies SignalSender,
    send: ((payload, callback) => {
      if (!shouldPersist() || !canRetain(payload)) {
        // WARNING: a send that began while the page was visible is not retained, but the page can hide or start leaving before it dispatches and the keepalive budget can already be spent; a body that fails then is retained for the next load instead of dropped.
        send(payload, (result) => {
          if (
            result.code !== ExportResultCode.SUCCESS &&
            !isTerminal(result) &&
            (isPageHidden() || isPageLeaving()) &&
            canRetain(payload) &&
            retainOnFailure(payload, callback)
          ) {
            return;
          }

          callback(result);
        });

        return;
      }

      try {
        if (retain(payload, callback) !== null) {
          void drain();
        }
      } catch {
        send(payload, callback);
      }
    }) satisfies SignalSender,
  };
}
