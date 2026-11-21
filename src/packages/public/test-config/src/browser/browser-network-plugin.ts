import { msw } from "msw/vite";

export const browserNetworkPlugin = msw({ mode: "worker-only" });
