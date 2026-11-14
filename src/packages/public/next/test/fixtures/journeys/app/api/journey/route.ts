import { notFound, redirect } from "next/navigation";

export function GET(request: Request) {
  const query = new URL(request.url).searchParams;
  const mode = query.get("mode");

  if (mode === "redirect") {
    redirect("/api/journey?mode=healthy");
  }

  if (mode === "missing") {
    notFound();
  }

  if (mode === "healthy") {
    return Response.json({ healthy: true });
  }

  if (mode === "stream") {
    return new Response(
      new ReadableStream({
        start(controller) {
          controller.enqueue(new TextEncoder().encode("first\n"));

          controller.enqueue(new TextEncoder().encode("second\n"));

          controller.close();
        },
      }),
      {
        headers: {
          "content-type": "text/plain",
          "x-customer-response": "preserved",
        },
      },
    );
  }

  throw new Error(query.get("marker") ?? "journey-route-error");
}
