/* oxlint-disable unicorn/prefer-top-level-await */
import { init } from "./instrument-client.js";

// oxlint-disable-next-line promise/prefer-await-to-then, anti-slop/no-unknown-parameters -- Side-effect startup must not await initialization or normalize arbitrary rejection values before development logging.
init().catch((error: unknown) => {
  if (process.env.NODE_ENV !== "production") {
    console.warn("[interfere] init() threw during boot", error);
  }
});
