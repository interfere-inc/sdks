import { createContextKey } from "@opentelemetry/api";
import type { Context } from "@opentelemetry/api";
import { identityAttributes } from "../../../tracking/view.js";
import type { IdentityIds } from "../../../tracking/view.js";

const KEY = createContextKey("interfere.request.identity");

class RequestIdentity {
  readonly attributes: Record<string, string>;
  readonly isCurrent: () => boolean;
  constructor(attributes: Record<string, string>, isCurrent: () => boolean) {
    this.isCurrent = isCurrent;
    this.attributes = attributes;
  }
}

export function snapshotRequestIdentity(
  ctx: Context,
  ids: IdentityIds,
  isCurrent: () => boolean = () => true,
) {
  return ctx.setValue(
    KEY,
    new RequestIdentity(identityAttributes(ids), isCurrent),
  );
}

export function requestIdentity(ctx: Context | undefined) {
  const identity = ctx?.getValue(KEY);

  if (!(identity instanceof RequestIdentity)) {
    return;
  }

  return identity.isCurrent() ? identity.attributes : {};
}

export function requestIdentityIsCurrent(ctx: Context) {
  const identity = ctx.getValue(KEY);

  return !(identity instanceof RequestIdentity) || identity.isCurrent();
}
