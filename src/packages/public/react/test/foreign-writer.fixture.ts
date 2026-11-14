import { foreignWriter } from "./foreign-writer.js";

declare global {
  var foreignWriter: typeof import("./foreign-writer.js").foreignWriter;
}

globalThis.foreignWriter = foreignWriter;
