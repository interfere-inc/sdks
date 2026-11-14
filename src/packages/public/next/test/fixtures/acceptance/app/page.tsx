/* oxlint-disable import/no-default-export, react-perf/jsx-no-new-function-as-prop, typescript/no-misused-promises */
"use client";

import { useState } from "react";
import { capture } from "@interfere/next/instrument-client";
import { useSession } from "@interfere/next/provider";
import { fail, succeed } from "./actions";

export default function Page() {
  const session = useSession();
  const [result, setResult] = useState("hydrated");

  return (
    <main>
      <output aria-label="Result">{result}</output>
      <output aria-label="Session">{session}</output>
      <button
        type="button"
        onClick={() => capture(new Error("next-client-error"))}
      >
        Capture client error
      </button>
      <button type="button" onClick={async () => setResult(await succeed())}>
        Successful action
      </button>
      <button
        type="button"
        onClick={async () => {
          try {
            await fail();
          } catch {
            setResult("action-rejected");
          }
        }}
      >
        Failing action
      </button>
      <button
        type="button"
        onClick={async () => {
          const response = await fetch("/api/failure");

          setResult(`route-${response.status}`);
        }}
      >
        Failing route
      </button>
    </main>
  );
}
