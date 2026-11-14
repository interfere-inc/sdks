/* oxlint-disable import/no-default-export */

// oxlint-disable-next-line anti-slop/no-unsafe-dictionary-type -- Loader globals preserve arbitrary customer JSON values under dynamically supplied global names.
type BakedGlobals = Record<string, unknown>;

export interface ValueInjectionLoaderOptions {
  readonly serializedValues?: string;
}

interface LoaderThis<T> {
  getOptions: () => T;
}

export default function valueInjectionLoader(
  this: LoaderThis<ValueInjectionLoaderOptions>,
  userCode: string,
) {
  // oxlint-disable-next-line oxc/no-this-in-exported-function
  return render(resolveValues(this.getOptions()), userCode);
}

function render(values: BakedGlobals, userCode: string) {
  const lines = Object.entries(values).map(
    ([key, value]) =>
      `globalThis[${JSON.stringify(key)}] = ${JSON.stringify(value)};`,
  );

  return lines.length === 0 ? userCode : `${lines.join("\n")}\n${userCode}`;
}

function parseJson(text: string | undefined) {
  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- Webpack invokes loaders from JavaScript configuration, where serialized options may be omitted or non-string.
  if (typeof text !== "string") {
    return;
  }

  try {
    const decodedValue: unknown = JSON.parse(text);

    return decodedValue;
  } catch {
    return;
  }
}

function resolveValues(options: ValueInjectionLoaderOptions) {
  const parsed = parseJson(options.serializedValues);

  return (isRecord(parsed) ? parsed : {}) satisfies BakedGlobals;
}

// oxlint-disable-next-line anti-slop/no-unsafe-dictionary-type -- This check admits arbitrary JSON object values for global injection without imposing an application payload schema.
function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}
