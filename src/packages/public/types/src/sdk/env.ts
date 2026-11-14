import { z } from "zod";

const interfereEnvValueSchema = z.string().trim().min(1);

// oxlint-disable-next-line anti-slop/no-unknown-parameters -- This schema-backed boundary rejects non-string customer environment values before trimming them.
export function parseEnvValue(value: unknown) {
  const parsed = interfereEnvValueSchema.safeParse(value);

  if (!parsed.success) {
    return null;
  }

  return parsed.data;
}

export function readFirstEnvValue(
  env: Record<string, string | undefined>,
  keys: readonly string[],
) {
  for (const key of keys) {
    const value = parseEnvValue(env[key]);

    if (value !== null) {
      return value;
    }
  }

  return null;
}
