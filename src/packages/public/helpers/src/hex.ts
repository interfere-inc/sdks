const HEX_BYTES = Array.from(
  { length: 256 },
  (_, byte) => `${byte < 16 ? "0" : ""}${byte.toString(16)}`,
);

/** Encodes bytes as lowercase hex, two characters per byte. */
export function bytesToHex(bytes: ArrayBuffer | Uint8Array) {
  let hex = "";

  for (const byte of new Uint8Array(bytes)) {
    hex += HEX_BYTES[byte];
  }

  return hex;
}

/** SHA-256 of a string (UTF-8) or bytes as lowercase hex, via Web Crypto. */
export async function sha256Hex(input: ArrayBuffer | string | Uint8Array) {
  const data =
    input instanceof ArrayBuffer || input instanceof Uint8Array
      ? new Uint8Array(input)
      : new TextEncoder().encode(input);

  return bytesToHex(await crypto.subtle.digest("SHA-256", data));
}
