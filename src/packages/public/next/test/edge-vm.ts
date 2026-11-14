import { readFileSync } from "node:fs";
import { EdgeVM } from "@edge-runtime/vm";

const vm = new EdgeVM();

const globals: unknown = vm.evaluate(
  "[typeof process, typeof Buffer, typeof require]",
);

vm.evaluate(readFileSync(0, "utf8"));

const recording: unknown = await vm.evaluate("qualify()");

process.stdout.write(JSON.stringify({ globals, recording }));
