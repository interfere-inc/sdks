import { captureError, flush, register } from "@interfere/vite/server";

await register({ consoleBridge: false });

captureError(new Error("vite-server-error"));

await flush();
