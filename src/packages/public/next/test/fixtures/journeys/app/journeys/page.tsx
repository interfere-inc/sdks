/* oxlint-disable import/no-default-export, react-perf/jsx-no-new-function-as-prop, typescript/no-misused-promises */
"use client";

import Link from "next/link";
import { useState } from "react";
import { capture, close } from "@interfere/next/instrument-client";
import { useSession } from "@interfere/next/provider";
import { redirectAction, submit } from "./actions";

export default function Page() {
  const session = useSession();
  const [marker, setMarker] = useState("journey");
  const [result, setResult] = useState("hydrated");

  return (
    <main>
      <output aria-label="Session">{session}</output>
      <output aria-label="Result">{result}</output>
      <label>
        Marker
        <input
          value={marker}
          onChange={(event) => setMarker(event.target.value)}
        />
      </label>
      <button type="button" onClick={() => capture(new Error(marker))}>
        Capture
      </button>
      <button
        type="button"
        onClick={async () => {
          const response = await fetch(
            `/api/journey?marker=${encodeURIComponent(marker)}`,
          );

          setResult(`route-${response.status}`);
        }}
      >
        Fail route
      </button>
      <button
        type="button"
        onClick={async () => {
          try {
            setResult(await submit(marker));
          } catch {
            setResult("action-rejected");
          }
        }}
      >
        Submit
      </button>
      <button
        type="button"
        onClick={async () => {
          await close();

          setResult("closed");
        }}
      >
        Close SDK
      </button>
      <button type="button" onClick={async () => redirectAction()}>
        Redirect action
      </button>
      <Link href="/destination">Client navigation</Link>
    </main>
  );
}
