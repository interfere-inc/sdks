import {
  isNonErrorException,
  shouldDropBrowserExtensionNoise,
  shouldDropUnresolvableStack,
} from "@interfere/types/sdk/errors";
import type { NonErrorException } from "@interfere/types/sdk/errors";
import type { ErrorFilterOptions } from "@interfere/types/sdk/options/filter-options";

import { context, trace } from "@opentelemetry/api";

import type { SessionTracker } from "../../tracking/api.js";
import { identityAttributes } from "../../tracking/view.js";
import type { ViewIds } from "../../tracking/view.js";
import {
  buildExceptionEventAttrs,
  recordExceptionOnSpan,
} from "../capture/exception-span.js";
import type { RecordExceptionOpts } from "../capture/exception-span.js";
import { filterException } from "../capture/error-filters.js";
import type { StartupExceptionBuffer } from "../capture/startup-buffer.js";
import { errorFingerprint } from "../capture/turn-dedupe.js";
import type { createTurnDedupe } from "../capture/turn-dedupe.js";
import type { TelemetrySink } from "../capture/sink.js";

interface ExceptionState {
  filters: ErrorFilterOptions;
  sameTurn: ReturnType<typeof createTurnDedupe>;
  seen: WeakSet<Error>;
  sink: TelemetrySink | null;
  startup: StartupExceptionBuffer;
  tracker: SessionTracker;
  views: ViewIds;
}

export function recordKernelException(
  kernel: ExceptionState,
  value: Error | NonErrorException,
  opts: RecordExceptionOpts,
) {
  if (!isNonErrorException(value)) {
    if (kernel.seen.has(value)) {
      return;
    }

    kernel.seen.add(value);

    if (!kernel.sameTurn(errorFingerprint(value), opts.mechanism.type)) {
      return;
    }
  }

  if (!isNonErrorException(value)) {
    const stacks: string[] = [];
    let current: Error | undefined = value;

    for (let depth = 0; current && depth < 5; depth += 1) {
      if (current.stack) {
        stacks.push(current.stack);
      }

      current = current.cause instanceof Error ? current.cause : undefined;
    }

    if (
      shouldDropBrowserExtensionNoise(stacks) ||
      shouldDropUnresolvableStack(stacks)
    ) {
      return;
    }
  }

  const exception = filterException(kernel.filters, value, opts);

  if (!exception) {
    return;
  }

  kernel.tracker.noteException();

  const attrs = {
    ...buildExceptionEventAttrs(exception, opts),
    ...identityAttributes({
      getActionId: () => kernel.views.actionId(),
      getViewId: () => kernel.views.viewId(),
    }),
  };

  if (
    kernel.startup.holding &&
    kernel.startup.push({
      attrs,
      handled: opts.mechanism.handled,
      origin: { at: Date.now(), context: context.active() },
      value: exception,
    })
  ) {
    return;
  }

  const active = trace.getActiveSpan();

  if (active?.isRecording()) {
    recordExceptionOnSpan(active, exception, attrs, opts.mechanism.handled);

    return;
  }

  kernel.sink?.recordException(exception, attrs, opts.mechanism.handled);
}
