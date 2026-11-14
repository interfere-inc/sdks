import { resetIngestBackoff } from "../src/internal/ingest-backoff/ingest-backoff.js";
import { setLogLevel } from "../src/util/log.js";
import { beforeEach } from "vitest";

beforeEach(() => {
  setLogLevel("none");

  resetIngestBackoff();
});
