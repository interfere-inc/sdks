"use server";

import { redirect } from "next/navigation";
import { withInterfereServerAction } from "@interfere/next/server";

export async function submit(value: string) {
  return withInterfereServerAction(
    "journey-submit",
    () => {
      if (value.startsWith("fail-")) {
        throw new Error(value);
      }

      return `accepted-${value}`;
    },
    { args: [value] },
  );
}

export async function redirectAction() {
  return withInterfereServerAction("journey-redirect", () =>
    redirect("/destination"),
  );
}
