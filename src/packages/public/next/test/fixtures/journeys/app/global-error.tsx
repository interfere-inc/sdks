/* oxlint-disable import/no-default-export */
"use client";

export default function GlobalError({
  error,
}: {
  error: Error & { digest?: string };
}) {
  return (
    <html lang="en">
      <body>
        <h1>Global failure</h1>
        <output aria-label="Global message">{error.message}</output>
      </body>
    </html>
  );
}
