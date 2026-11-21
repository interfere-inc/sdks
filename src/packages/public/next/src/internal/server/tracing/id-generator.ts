import { RandomIdGenerator } from "@opentelemetry/sdk-trace";
import type { IdGenerator } from "@opentelemetry/sdk-trace";
import { CounterIdGenerator } from "./counter-id-generator";

/**
 * Generates random telemetry IDs, with counter IDs when runtime randomness is
 * unavailable. Internal to the managed Next adapter: `register()` installs it
 * itself, and the deprecated public export wraps it until the kit is removed.
 */
export class PrerenderSafeIdGenerator implements IdGenerator {
  private readonly random = new RandomIdGenerator();
  private readonly fallback = new CounterIdGenerator();

  generateTraceId() {
    try {
      return this.random.generateTraceId();
    } catch {
      return this.fallback.generateTraceId();
    }
  }

  generateSpanId() {
    try {
      return this.random.generateSpanId();
    } catch {
      return this.fallback.generateSpanId();
    }
  }
}
