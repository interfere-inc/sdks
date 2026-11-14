import pkg from "../../package.json" with { type: "json" };

export const SDK_NAME = pkg.name;

export const SDK_VERSION = pkg.version;

export const PRODUCER_VERSION = `${SDK_NAME}@${SDK_VERSION}`;
