import type { TracingMode } from "@interfere/types/sdk/remote-config/tracing-mode";
import type { TracingOwnership } from "@interfere/types/sdk/remote-config/tracing-ownership";
import { context, propagation } from "@opentelemetry/api";
import type { ContextManager, TextMapPropagator } from "@opentelemetry/api";
import { createLogger } from "../../util/log.js";

const log = createLogger("tracing");

export const PROPAGATOR_OWNED_ELSEWHERE_WARNING =
  '[interfere] another propagator owns trace headers; Interfere preserves it. Set tracing.propagation to "external" when another instrumentation layer writes request trace context.';

/** Selects whether Interfere or another instrumentation layer writes W3C trace context. */
export type TracePropagationOwnership = "interfere" | "external";

export interface TracingOwnershipInput {
  contextManager: ContextManager;
  mode: TracingMode;
  propagation?: TracePropagationOwnership;
  propagator: TextMapPropagator;
}

export interface OwnershipController {
  readonly mode: TracingOwnership;
  readonly ownsContextManager: boolean;
  readonly ownsPropagator: boolean;
  release: () => void;
  readonly writesTraceContext: boolean;
}

/** Claims only free OpenTelemetry globals; external propagation never registers Interfere's propagator. */
export function claimTracingOwnership(input: TracingOwnershipInput) {
  const { contextManager, mode: requested, propagator } = input;
  const writesTraceContext = input.propagation !== "external";
  let ownsContextManager = false;
  let ownsPropagator = false;
  let released = false;

  let mode: TracingOwnership =
    requested === "managed" ? "managed" : "application";

  if (writesTraceContext && requested !== "application") {
    ownsPropagator = propagation.setGlobalPropagator(propagator);

    if (requested === "auto" && ownsPropagator) {
      mode = "managed";
    }

    if (!ownsPropagator && requested === "managed") {
      console.warn(PROPAGATOR_OWNED_ELSEWHERE_WARNING);
    }
  }

  if (mode === "managed") {
    contextManager.enable();

    ownsContextManager = context.setGlobalContextManager(contextManager);

    if (!ownsContextManager) {
      contextManager.disable();

      log.debug(
        "another context manager is registered; Interfere uses it unchanged",
      );
    }
  }

  return {
    mode,
    ownsContextManager,
    ownsPropagator,
    writesTraceContext,
    release: () => {
      if (released) {
        return;
      }

      released = true;

      if (ownsContextManager) {
        context.disable();

        contextManager.disable();
      }

      if (ownsPropagator) {
        propagation.disable();
      }
    },
  } satisfies OwnershipController;
}
