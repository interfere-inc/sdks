const HYDRATION_CODE =
  /(?:Minified React error #|react\.dev\/errors\/)(?:418|423|425)/u;

const HYDRATION_TEXT =
  /hydration failed|error while hydrating|hydrated but|text content does not match|server[- ]rendered html|did(?:n't| not) match the client/iu;

// oxlint-disable-next-line anti-slop/no-unknown-parameters -- React error callbacks can receive non-Error throws; only actual Error messages participate in hydration classification.
export function isHydrationError(error: unknown) {
  if (!(error instanceof Error)) {
    return false;
  }

  const { message } = error;

  return HYDRATION_CODE.test(message) || HYDRATION_TEXT.test(message);
}
