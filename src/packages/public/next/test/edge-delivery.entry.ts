import { onRequestError, register } from "../src/instrumentation.edge.js";

Object.assign(globalThis, {
  async qualify() {
    await register({ tracing: "application" });

    await onRequestError(
      new Error("isolated-edge-export"),
      { method: "GET", path: "/" },
      { routerKind: "App Router", routePath: "/", routeType: "render" },
    );
  },
});
