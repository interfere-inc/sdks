// oxlint-disable-next-line anti-slop/no-unknown-parameters
export function parseRoutePrefixInput(value: unknown) {
  // oxlint-disable-next-line anti-slop/no-runtime-typeof
  return typeof value === "string" && value.length > 0 ? value : undefined;
}
