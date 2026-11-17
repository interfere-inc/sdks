import { isPlainRecord, isString } from "@interfere/types/sdk/guards/json";

const codePattern = /^[A-Za-z0-9_.:-]{1,64}$/u;

const NON_ERROR_CODES = new Set([
  "0",
  "none",
  "null",
  "false",
  "ok",
  "success",
]);

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
