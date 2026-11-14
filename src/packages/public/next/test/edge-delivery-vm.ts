import { readFileSync } from "node:fs";
import { EdgeVM } from "@edge-runtime/vm";

const vm = new EdgeVM();

const capabilities: unknown = vm.evaluate(
  "[typeof process, typeof Buffer, typeof require, typeof AbortSignal.any]",
);

vm.evaluate(readFileSync(0, "utf8"));

await vm.evaluate("qualify()");

process.stdout.write(JSON.stringify(capabilities));
