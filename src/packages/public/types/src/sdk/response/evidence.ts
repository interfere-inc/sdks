import { isPlainRecord, isString } from "../guards/json.ts";

export const RESPONSE_EVIDENCE_MAX_BYTES = 64 * 1024;

const READ_BUDGET_MS = 1000;

const ERROR_DETAIL_MAX_CHARS = 256;

const ERROR_DETAIL_READ_CHARS = 1024;

const codePattern = /^[A-Za-z0-9_.:-]{1,64}$/u;

const NON_ERROR_CODES = new Set([
  "0",
  "none",
  "null",
  "false",
  "ok",
  "success",
]);

const EMAIL = /[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}/giu;

const CARD = /\b\d(?:[ -]?\d){12,18}\b/gu;

const PHONE =
  /(?:\+\d{8,15}\b)|(?:(?:\+\d{1,3}[ .-]?)?(?:\(\d{3}\)|\b\d{3})[ .-]\d{3}[ .-]\d{4}\b)/gu;

function isCode(value: unknown): value is string {
  return isString(value) && codePattern.test(value);
}

function isApplicationCode(value: unknown): value is string {
  return isCode(value) && !NON_ERROR_CODES.has(value.toLowerCase());
}

export interface ResponseEvidence {
  status: "captured" | "skipped";
  reason?: string;
  codes: string[];
  applicationError: boolean;
}

export function skippedResponseEvidence(reason: string) {
  return {
    status: "skipped",
    reason,
    codes: [] as string[],
    applicationError: false,
  } satisfies ResponseEvidence;
}

// oxlint-disable-next-line anti-slop/no-unknown-parameters -- This boundary validates untrusted JSON from fetch or XHR before retaining only protocol discriminators.
export function responseDiscriminators(body: unknown, status: number) {
  if (!isPlainRecord(body)) {
    return skippedResponseEvidence("unsupported-shape");
  }

  const { code, data, error, errors, error_code, errorCode } = body;

  const errorCandidates = [
    ...(status === 200 ? [] : [code]),
    error_code,
    errorCode,
    ...(isPlainRecord(error) ? [error["code"]] : []),
    ...((status !== 200 || data === null || data === undefined) &&
    Array.isArray(errors)
      ? errors.slice(0, 5)
      : []
    ).flatMap((item) => {
      const extensions = isPlainRecord(item) ? item["extensions"] : undefined;

      return isPlainRecord(extensions) ? [extensions["code"]] : [];
    }),
  ];

  const discriminator = status === 200 ? isApplicationCode : isCode;

  const codes = errorCandidates.filter((value): value is string =>
    discriminator(value),
  );

  return {
    status: "captured",
    codes: [...new Set(codes)],
    applicationError: status === 200 && codes.length > 0,
  } satisfies ResponseEvidence;
}

/** Masks email addresses, card numbers and phone numbers in error text before it leaves the process. */
export function redactErrorDetail(text: string) {
  return text
    .replaceAll(EMAIL, "[email]")
    .replaceAll(CARD, "[card]")
    .replaceAll(PHONE, "[phone]");
}

// oxlint-disable-next-line anti-slop/no-unknown-parameters -- This boundary reads untrusted JSON error bodies and keeps only one bounded, redacted message.
export function responseErrorDetail(body: unknown) {
  if (!isPlainRecord(body)) {
    return null;
  }

  const { detail, error, errors, message, title } = body;
  const first: unknown = Array.isArray(errors) ? errors[0] : undefined;

  const text = [
    message,
    detail,
    title,
    error,
    isPlainRecord(error) ? error["message"] : undefined,
    isPlainRecord(first) ? first["message"] : undefined,
  ].find((value): value is string => isString(value) && value.trim() !== "");

  if (text === undefined) {
    return null;
  }

  const redacted = redactErrorDetail(
    text.trim().slice(0, ERROR_DETAIL_READ_CHARS),
  );

  return redacted.length > ERROR_DETAIL_MAX_CHARS
    ? `${redacted.slice(0, ERROR_DETAIL_MAX_CHARS - 1)}…`
    : redacted;
}

/** Why a response body is not read, or null when it is bounded JSON; a missing length is allowed only when the stream read enforces the bound. */
export function responseSkipReason(
  getHeader: (name: string) => string | null,
  { lengthRequired }: { lengthRequired: boolean },
) {
  const length = getHeader("content-length");
  const type = getHeader("content-type")?.split(";", 1)[0]?.trim();

  if (length === null || !/^\d+$/u.test(length)) {
    if (lengthRequired || length !== null) {
      return "no-content-length";
    }
  } else if (Number(length) > RESPONSE_EVIDENCE_MAX_BYTES) {
    return "over-64KiB";
  }

  if (!(type === "application/json" || type?.endsWith("+json"))) {
    return "non-json";
  }

  return null;
}

/** Parses at most 64 KiB of a response clone within one second, never consuming the application's body; `cancelFailed` hears a clone cancellation that fails after the read settles. */
export async function readJsonResponse(
  response: Response,
  // oxlint-disable-next-line anti-slop/no-unknown-parameters -- Stream cancellation rejects with whatever the host runtime throws.
  cancelFailed: (reason: unknown) => void,
) {
  let clone: Response;

  try {
    clone = response.clone();
  } catch {
    return { skipped: "unreadable" } as const;
  }

  const reader = clone.body?.getReader();

  if (!reader) {
    return { skipped: "empty-body" } as const;
  }

  let timer: ReturnType<typeof setTimeout> | undefined;

  const expired = new Promise<undefined>((resolve) => {
    timer = setTimeout(() => resolve(undefined), READ_BUDGET_MS);
  });

  const decoder = new TextDecoder();
  let bytes = 0;
  let text = "";

  try {
    while (true) {
      // oxlint-disable-next-line eslint/no-await-in-loop -- Read each bounded stream chunk before requesting another.
      const chunk = await Promise.race([reader.read(), expired]);

      if (!chunk) {
        return { skipped: "timeout" } as const;
      }

      if (chunk.done) {
        break;
      }

      bytes += chunk.value.byteLength;

      if (bytes > RESPONSE_EVIDENCE_MAX_BYTES) {
        return { skipped: "over-64KiB" } as const;
      }

      text += decoder.decode(chunk.value, { stream: true });
    }

    const body: unknown = JSON.parse(text + decoder.decode());

    return { body } as const;
  } catch {
    return { skipped: "invalid-json" } as const;
  } finally {
    clearTimeout(timer);

    // oxlint-disable-next-line promise/prefer-await-to-then -- Tee cancellation can wait for the app to consume its branch; never delay telemetry settlement on it.
    void reader.cancel().catch(cancelFailed);
  }
}

/** Collects bounded response identifiers from a clone, never consuming the application's body. */
export async function fetchResponseEvidence(
  response: Response,
  // oxlint-disable-next-line anti-slop/no-unknown-parameters -- Stream cancellation rejects with whatever the host runtime throws.
  cancelFailed: (reason: unknown) => void,
) {
  const reason = responseSkipReason((name) => response.headers.get(name), {
    lengthRequired: true,
  });

  if (reason) {
    return skippedResponseEvidence(reason);
  }

  const read = await readJsonResponse(response, cancelFailed);

  return "skipped" in read
    ? skippedResponseEvidence(read.skipped)
    : responseDiscriminators(read.body, response.status);
}
