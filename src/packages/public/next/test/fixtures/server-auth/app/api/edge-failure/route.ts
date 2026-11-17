export const runtime = "edge";

export function GET() {
  throw new Error("next-edge-route-error");
}
