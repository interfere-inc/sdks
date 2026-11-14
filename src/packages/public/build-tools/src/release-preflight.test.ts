import { once } from "node:events";
import { createServer } from "node:http";
import { InterfereClient } from "@interfere/javascript";
import { afterAll, beforeAll, expect, test } from "vitest";
import {
  missingReleaseBinding,
  preflightReleaseBinding,
} from "./release-request";

let body: string;
let client: InterfereClient;

let server: ReturnType<typeof createServer>;

const configured = {
  org: { id: "org", slug: "customer" },
  surface: {
    id: "surface",
    slug: "web",
    sourceProvider: "github",
    destinationProvider: "cli",
    environmentFiltering: true,
    sdkConfig: null,
  },
};

beforeAll(async () => {
  body = "{}";

  server = createServer((_request, response) => {
    response.writeHead(200, { "content-type": "application/json" }).end(body);
  });

  server.listen(0, "127.0.0.1");

  await once(server, "listening");

  const address = server.address();

  if (!(address instanceof Object)) {
    throw new TypeError("Preflight receiver did not bind");
  }

  client = new InterfereClient({
    baseUrl: `http://127.0.0.1:${address.port}`,
    apiKey: { token: "interfere_secret_us_preflight" },
    maxRetries: 0,
  });
});

afterAll(async () => {
  server.closeAllConnections();

  await new Promise<void>((resolve, reject) => {
    server.close((error) => (error ? reject(error) : resolve()));
  });
});

test.each([
  null,
  [],
  {},
  { org: configured.org, surface: null },
  { org: configured.org, surface: {} },
  { org: null, surface: configured.surface },
  { ...configured, surface: { ...configured.surface, sourceProvider: [] } },
  {
    ...configured,
    surface: { ...configured.surface, destinationProvider: 123 },
  },
])("treats malformed HTTP200 config %j as unavailable", async (invalid) => {
  body = JSON.stringify(invalid);

  await expect(preflightReleaseBinding(client)).resolves.toBeNull();
});

test("preserves configured release metadata", async () => {
  body = JSON.stringify(configured);

  const result = await preflightReleaseBinding(client);

  expect(result).toStrictEqual(configured);
  expect(result && missingReleaseBinding(result)).toBeNull();
});

test.each([
  { field: "sourceProvider", value: null, missing: "version control provider" },
  { field: "destinationProvider", value: null, missing: "deployment target" },
  {
    field: "sourceProvider",
    value: "future-source",
    missing: "version control provider",
  },
  {
    field: "destinationProvider",
    value: "future-target",
    missing: "deployment target",
  },
])(
  "retains a confirmed unsupported $field ($value)",
  async ({ field, value, missing }) => {
    const config = {
      ...configured,
      surface: { ...configured.surface, [field]: value },
    };

    body = JSON.stringify(config);

    const result = await preflightReleaseBinding(client);

    expect(result).toStrictEqual(config);
    expect(result && missingReleaseBinding(result)).toContain(missing);

    expect(result && missingReleaseBinding(result)).toContain(
      "https://interfere.com/~/customer/apps/web",
    );
  },
);
