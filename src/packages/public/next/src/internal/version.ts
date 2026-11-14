import pkg from "../../package.json" with { type: "json" };

export const PRODUCER_VERSION = `${pkg.name}@${pkg.version}`;
