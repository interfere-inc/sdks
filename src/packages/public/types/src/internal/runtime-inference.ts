interface RuntimeEvidence {
  hasWindow: boolean;
  hasDocument: boolean;
  hasNavigator: boolean;
  edgeRuntime?: unknown;
  nodeVersion?: unknown;
  nextRuntime?: unknown;
}

export function inferRuntimeFromEvidence(evidence: RuntimeEvidence) {
  if (evidence.hasWindow && evidence.hasDocument && evidence.hasNavigator) {
    return "browser";
  }

  // oxlint-disable-next-line anti-slop/no-runtime-typeof
  if (typeof evidence.edgeRuntime === "string") {
    return "edge";
  }

  if (evidence.nextRuntime === "edge") {
    return "edge";
  }

  // oxlint-disable-next-line anti-slop/no-runtime-typeof
  if (typeof evidence.nodeVersion === "string") {
    return "node";
  }

  return null;
}
