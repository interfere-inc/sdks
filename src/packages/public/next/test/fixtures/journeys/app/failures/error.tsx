/* oxlint-disable import/no-default-export */
"use client";

export default function FailureBoundary({
  error,
}: {
  error: Error & { digest?: string };
}) {
  return (
    <main>
      <h1>Failure boundary</h1>
      <output aria-label="Digest">{error.digest}</output>
    </main>
  );
}
