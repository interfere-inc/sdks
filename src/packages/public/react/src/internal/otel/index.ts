/**
 * Internal OTel module index — what the kernel `await import()`s when
 * tracing is enabled. Splitting the OTel surface into its own entry point
 * gives the bundler a clean code-split point: customers who init with
 * `tracing: false` never pay the OTel-SDK bundle cost.
 *
 * This is **internal** — there's no customer-facing OTel re-export.
 * Customers reach OTel via the (Phase 3) high-level `span()` /
 * `breadcrumb()` / `capture()` helpers; raw OTel access is not part of
 * the SDK's public surface.
 */

export {
  type BundledInstrumentations,
  type CaptureFailedRequestsOption,
  type CaptureFailedRequestsOptions,
  type HttpStatusCodeRange,
  registerBundledInstrumentations,
} from "./instrumentations/instrumentations.js";

export { installViewTracker, type ViewTracker } from "./navigation.js";

export { claimTracingOwnership } from "./ownership.js";

export { readPropagationFromDocument } from "./propagation.js";

export { buildOtelProvider, type OtelProviderHandle } from "./provider.js";

export { captureWebVitals } from "./web-vitals.js";

export { resolveTracingMode, wireOtel } from "./wire.js";
