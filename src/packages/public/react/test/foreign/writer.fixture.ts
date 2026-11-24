import { foreignWriter } from "./writer.js";

declare global {
  var foreignWriter: typeof import("./writer.js").foreignWriter;
}

globalThis.foreignWriter = foreignWriter;
