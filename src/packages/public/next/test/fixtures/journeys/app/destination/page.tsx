/* oxlint-disable import/no-default-export, react-perf/jsx-no-new-function-as-prop */
"use client";

import Link from "next/link";
import { capture } from "@interfere/next/instrument-client";
import { useSession } from "@interfere/next/provider";

export default function Destination() {
  const session = useSession();

  return (
    <main>
      <h1>Destination</h1>
      <output aria-label="Session">{session}</output>
      <button
        type="button"
        onClick={() => capture(new Error("destination-error"))}
      >
        Capture destination
      </button>
      <Link href="/journeys">Return</Link>
    </main>
  );
}
