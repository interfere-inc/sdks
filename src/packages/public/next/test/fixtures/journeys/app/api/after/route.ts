import { after } from "next/server";
import { withInterfereAfter } from "@interfere/next/server";

export function GET(request: Request) {
  const marker =
    new URL(request.url).searchParams.get("marker") ?? "journey-after-error";

  after(
    withInterfereAfter(async () => {
      await new Promise((resolve) => {
        setTimeout(resolve, 250);
      });

      throw new Error(marker);
    }),
  );

  return Response.json({ scheduled: true });
}
