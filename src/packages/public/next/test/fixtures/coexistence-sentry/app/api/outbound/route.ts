export const dynamic = "force-dynamic";

export async function GET() {
  const response = await fetch(`${process.env["OUTBOUND_URL"]}/outbound`, {
    cache: "no-store",
  });

  return Response.json({ status: response.status });
}
