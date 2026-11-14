/* oxlint-disable import/no-default-export, react-perf/jsx-no-new-function-as-prop */
"use client";

import { capture } from "@interfere/next/instrument-client";
import { useInterfere, useSession } from "@interfere/next/provider";

export default function Journey() {
  const interfere = useInterfere();
  const session = useSession();

  return (
    <main>
      <output aria-label="Session">{session}</output>
      <button
        type="button"
        onClick={() => capture(new Error("example-journey-error"))}
      >
        Capture journey error
      </button>
      <button
        type="button"
        onClick={() =>
          interfere.consent.set({ analytics: false, replay: false })
        }
      >
        Deny analytics
      </button>
      <button
        type="button"
        onClick={() => interfere.consent.set({ analytics: true, replay: true })}
      >
        Grant analytics
      </button>
    </main>
  );
}
