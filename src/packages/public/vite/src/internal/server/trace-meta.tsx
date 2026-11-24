import { traceMeta } from "./request/trace";

/**
 * Server Component that emits the W3C trace-context meta tags
 * (`traceparent`, plus `tracestate` and `baggage` when present) so the
 * client SDK can stitch every browser span onto the server-side trace.
 * The `tracestate` tag carries the freshness stamp that lets the browser
 * reject trace context served from cached HTML.
 *
 * Customer usage — drop into the root layout's `<head>` (TanStack
 * Start uses the `head()` route option, but the meta tags also work
 * if rendered into the React tree):
 *
 * ```tsx
 * import { TraceMeta } from "@interfere/vite/server";
 *
 * function RootComponent() {
 *   return (
 *     <html>
 *       <head>
 *         <TraceMeta />
 *       </head>
 *       <body>...</body>
 *     </html>
 *   );
 * }
 * ```
 *
 * Renders nothing when no OTel span is active (dev without OTel
 * registered). Unsampled spans render with their flags intact; the
 * browser keeps that context rather than starting a new sampled root.
 */
export function TraceMeta() {
  const meta = traceMeta();

  if (meta.length === 0) {
    return null;
  }

  return (
    <>
      {meta.map(({ name, content }) => (
        <meta content={content} key={name} name={name} />
      ))}
    </>
  );
}
