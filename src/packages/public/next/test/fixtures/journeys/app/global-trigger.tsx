/* oxlint-disable react/set-state-in-effect */
"use client";

import { useEffect, useState } from "react";

export function GlobalTrigger() {
  const [marker, setMarker] = useState<string | null>(null);

  useEffect(() => {
    if (location.pathname === "/global-failure") {
      setMarker(location.hash.slice(1) || "journey-global-error");
    }
  }, []);

  if (marker) {
    throw new Error(marker);
  }

  return null;
}
