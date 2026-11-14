/* oxlint-disable import/no-unassigned-import */
import "../../src/instrumentation-client.js";
import { BUILD_MARKER_GLOBAL } from "@interfere/types/sdk/injected-globals";

Object.assign(globalThis, { [BUILD_MARKER_GLOBAL]: true });
