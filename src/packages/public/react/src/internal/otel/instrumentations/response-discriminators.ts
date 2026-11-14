import { z } from "zod";

const code = z.string().regex(/^[A-Za-z0-9_.:-]{1,64}$/u);

const applicationCode = code.refine(
  (value) =>
    !["0", "none", "null", "false", "ok", "success"].includes(
      value.toLowerCase(),
    ),
);

const nestedCode = z.object({ code: z.unknown().optional() });
const graphqlCode = z.object({ extensions: nestedCode.optional() });

const codesSchema = z.object({
  code: z.unknown().optional(),
  error_code: z.unknown().optional(),
  errorCode: z.unknown().optional(),
  error: z.unknown().optional(),
  errors: z.unknown().optional(),
  data: z.unknown().optional(),
});

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
  const parsed = codesSchema.safeParse(body);

  if (!parsed.success) {
    return skippedResponseEvidence("unsupported-shape");
  }

  const { data } = parsed;
  const nested = nestedCode.safeParse(data.error);

  const errorCandidates = [
    ...(status === 200 ? [] : [data.code]),
    data.error_code,
    data.errorCode,
    ...(nested.success ? [nested.data.code] : []),
    ...((status !== 200 || data.data === null || data.data === undefined) &&
    Array.isArray(data.errors)
      ? data.errors.slice(0, 5)
      : []
    ).flatMap((item) => {
      const result = graphqlCode.safeParse(item);

      return result.success ? [result.data.extensions?.code] : [];
    }),
  ];

  const discriminator = status === 200 ? applicationCode : code;

  const errors = errorCandidates.flatMap((value) => {
    const result = discriminator.safeParse(value);

    return result.success ? [result.data] : [];
  });

  return {
    status: "captured",
    codes: [...new Set(errors)],
    applicationError: status === 200 && errors.length > 0,
  } satisfies ResponseEvidence;
}
