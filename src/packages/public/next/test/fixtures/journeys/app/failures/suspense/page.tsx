/* oxlint-disable import/no-default-export, react-perf/jsx-no-jsx-as-prop */
import { Suspense } from "react";
import { setTimeout } from "node:timers/promises";

export const dynamic = "force-dynamic";

async function Late({ marker }: { marker: string }) {
  await setTimeout(100);

  if (marker.length > 0) {
    throw new Error(marker);
  }

  return null;
}

export default async function StreamedFailure({
  searchParams,
}: {
  searchParams: Promise<{ marker?: string }>;
}) {
  const { marker } = await searchParams;

  return (
    <main>
      <h1>Streamed shell</h1>
      <Suspense fallback={<p>Loading</p>}>
        <Late marker={marker ?? "journey-suspense-error"} />
      </Suspense>
    </main>
  );
}
