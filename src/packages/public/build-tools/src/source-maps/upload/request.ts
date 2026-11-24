import { InterfereError, InterfereTimeoutError } from "@interfere/javascript";

export async function uploadSourceMapRequest<T>(
  request: () => Promise<T>,
  signal: AbortSignal,
) {
  signal.throwIfAborted();

  try {
    await request();
  } catch (error) {
    if (
      signal.aborted ||
      !(
        error instanceof InterfereTimeoutError ||
        (error instanceof InterfereError &&
          error.statusCode === undefined &&
          error.cause === "timeout")
      )
    ) {
      throw error;
    }

    signal.throwIfAborted();

    await request();
  }
}
