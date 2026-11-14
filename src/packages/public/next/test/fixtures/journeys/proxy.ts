import { NextResponse } from "next/server";
import type { NextRequest } from "next/server";
import { withInterfereProxy } from "@interfere/next/server";

export const proxy = withInterfereProxy((request: NextRequest) => {
  const marker = request.nextUrl.searchParams.get("marker");

  if (marker) {
    throw new Error(marker);
  }

  return NextResponse.next();
});

export const config = { matcher: "/proxy-failure" };
