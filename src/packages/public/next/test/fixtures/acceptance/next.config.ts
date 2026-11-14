import { withInterfere } from "@interfere/next/config";
import { parse } from "node:path";

export default withInterfere({
  experimental: { cpus: 2 },
  turbopack: { root: parse(import.meta.dirname).root },
});
