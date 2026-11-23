import { CompositePropagator } from "@opentelemetry/core";

export const requestPropagator = new CompositePropagator({ propagators: [] });

export function composeRequestMethod<
  Host extends object,
  Key extends keyof Host,
>(
  host: Host,
  key: Key,
  original: Host[Key],
  compose: (patched: Host[Key]) => Host[Key],
) {
  const patched = host[key];

  if (patched === original) {
    return () => undefined;
  }

  const ours = compose(patched);

  host[key] = ours;

  return () => {
    if (host[key] === ours) {
      host[key] = original;
    }
  };
}
