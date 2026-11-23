import type { NonErrorException } from "../errors.ts";
import type { ErrorMechanism } from "../plugins/payload/errors";

/** A substring or regular expression matched against exception messages or script URLs. */
export type ErrorFilterPattern = string | RegExp;

/** Context passed to {@link BeforeSend} alongside the exception. */
export interface BeforeSendHint {
  /** How the exception was captured, including whether it was handled. */
  readonly mechanism: ErrorMechanism;
  /** The exception as captured, before the hook ran. */
  readonly originalException: Error | NonErrorException;
}

/**
 * Called for every exception that passes `ignoreErrors`, `denyUrls` and sampling. Return the
 * exception (mutated or replaced) to send it, or `null` to drop it. If the hook throws, the
 * original exception is sent.
 */
export type BeforeSend = (
  exception: Error | NonErrorException,
  hint: BeforeSendHint,
) => Error | NonErrorException | null;

/** Client-side exception filtering, applied to every capture source before export. */
export interface ErrorFilterOptions {
  /** Last chance to scrub, replace or drop an exception before it is sent. */
  beforeSend?: BeforeSend | undefined;
  /** Drops exceptions whose top stack frame comes from a matching script URL. */
  denyUrls?: readonly ErrorFilterPattern[] | undefined;
  /** Exception sampling. */
  errors?: { sampleRate?: number | undefined } | undefined;
  /** Drops exceptions whose message, or `Type: message`, matches. Strings match as substrings. */
  ignoreErrors?: readonly ErrorFilterPattern[] | undefined;
}
