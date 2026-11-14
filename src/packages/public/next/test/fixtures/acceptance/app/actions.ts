/* oxlint-disable typescript/require-await */
"use server";

import { withInterfereServerAction } from "@interfere/next/server";

export async function succeed() {
  return withInterfereServerAction(
    "acceptance-success",
    async () => "action-result",
  );
}

export async function fail() {
  return withInterfereServerAction("acceptance-failure", async () => {
    throw new Error("next-action-error");
  });
}
