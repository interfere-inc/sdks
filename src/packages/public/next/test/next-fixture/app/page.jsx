/* oxlint-disable import/no-default-export */
"use client";

import { useState } from "react";

export default function Page() {
  const [count, setCount] = useState(0);

  return (
    <button
      type="button"
      // oxlint-disable-next-line react-perf/jsx-no-new-function-as-prop -- This native test-fixture button has no memoized child or identity-sensitive subscriber; its callback only drives the SDK interaction test.
      onClick={() => setCount(count + 1)}
    >
      Native config: {count}
    </button>
  );
}
