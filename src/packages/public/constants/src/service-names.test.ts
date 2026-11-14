import { SDK_DEFAULT_SERVICE_NAMES } from "./service-names.js";
import { expect, test } from "vitest";

test("keeps the service names the SDKs have always reported by default", () => {
  expect(SDK_DEFAULT_SERVICE_NAMES).toStrictEqual({
    elysia: "elysia-app",
    next: "interfere-sdk-next-server",
    nextEdge: "interfere-sdk-next-edge",
    node: "node-app",
    react: "interfere-sdk",
    vite: "interfere-sdk-vite-server",
  });
});
