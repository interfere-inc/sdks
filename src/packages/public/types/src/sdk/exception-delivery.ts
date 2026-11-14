/**
 * Exception event attribute the server SDK sets on the copy it records onto an
 * application-owned span when it also delivers the same exception through its
 * own exporter. Ingest skips events carrying `EXCEPTION_DELIVERY.interfere`, so
 * a host provider that exports to Interfere never produces a duplicate.
 */
export const EXCEPTION_DELIVERY_ATTRIBUTE = "interfere.exception.delivery";

/** Values of {@link EXCEPTION_DELIVERY_ATTRIBUTE}. */
export const EXCEPTION_DELIVERY = {
  /** Interfere's own exception span is the delivered copy. */
  interfere: "interfere",
} as const;
