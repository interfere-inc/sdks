import { captureRequestError, init } from "@sentry/nextjs";
import type { Instrumentation } from "next";
import {
  onRequestError as interfereOnRequestError,
  register as registerInterfere,
} from "@interfere/next/instrumentation";

export async function register() {
  init({
    dsn: process.env["SENTRY_DSN"],
    tracesSampleRate: 1,
    sendClientReports: false,
  });

  await registerInterfere({ propagateTo: [process.env["OUTBOUND_URL"] ?? ""] });
}

export const onRequestError: Instrumentation.onRequestError = async (
  error,
  request,
  context,
) => {
  captureRequestError(error, request, context);

  await interfereOnRequestError(error, request, context);
};
