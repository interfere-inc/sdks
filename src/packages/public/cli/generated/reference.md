# Interfere API CLI Reference

Full command reference for `interfere`.

## Commands

- [`interfere audit`](#interfere-audit)
- [`interfere billing`](#interfere-billing)
- [`interfere billing invoices`](#interfere-billing-invoices)
- [`interfere configuration`](#interfere-configuration)
- [`interfere consumers`](#interfere-consumers)
- [`interfere consumers devices`](#interfere-consumers-devices)
- [`interfere domains`](#interfere-domains)
- [`interfere drain`](#interfere-drain)
- [`interfere feedback`](#interfere-feedback)
- [`interfere findings`](#interfere-findings)
- [`interfere integrations`](#interfere-integrations)
- [`interfere keys private`](#interfere-keys-private)
- [`interfere keys public`](#interfere-keys-public)
- [`interfere network`](#interfere-network)
- [`interfere notifications`](#interfere-notifications)
- [`interfere notifications settings`](#interfere-notifications-settings)
- [`interfere problems`](#interfere-problems)
- [`interfere problems timeline`](#interfere-problems-timeline)
- [`interfere releases`](#interfere-releases)
- [`interfere replay`](#interfere-replay)
- [`interfere replay sessions`](#interfere-replay-sessions)
- [`interfere scim directories`](#interfere-scim-directories)
- [`interfere scim groups`](#interfere-scim-groups)
- [`interfere services`](#interfere-services)
- [`interfere sessions`](#interfere-sessions)
- [`interfere sso connections`](#interfere-sso-connections)
- [`interfere support messages`](#interfere-support-messages)
- [`interfere support threads`](#interfere-support-threads)
- [`interfere surface-keys`](#interfere-surface-keys)
- [`interfere surfaces`](#interfere-surfaces)
- [`interfere surfaces environments`](#interfere-surfaces-environments)
- [`interfere tenants`](#interfere-tenants)
- [`interfere tenants tiers`](#interfere-tenants-tiers)
- [`interfere tiers`](#interfere-tiers)
- [`interfere traces`](#interfere-traces)
- [`interfere users`](#interfere-users)
- [`interfere users invitations`](#interfere-users-invitations)
- [`interfere users requests`](#interfere-users-requests)
- [`interfere workspaces`](#interfere-workspaces)

---

### `interfere audit`

#### `interfere audit count`

Returns a capped count of matching audit events for workspace owners and administrators.

`GET /v3/workspaces/{workspaceSlug}/audit/count`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--search` | `string` | No |  |
| `--search-actions` | `string[]` | No |  |
| `--actions` | `string[]` | No |  |
| `--actors` | `string (uuid)[]` | No |  |
| `--since` | `number` | No |  |

#### `interfere audit get`

Read one workspace audit event by its event ID from audit.list. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/workspaceAuditEvents.byId`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere audit list`

List workspace audit events to investigate who changed a resource. Use the schema's filters and pagination to narrow the time range and target. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/workspaceAuditEvents.list`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `interfere billing`

#### `interfere billing create-billing-checkout`

Returns a checkout session for the workspace. An existing active subscription prevents checkout.

`POST /v3/workspaces/{workspaceSlug}/billing/checkout`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere billing create-billing-portal`

Returns a billing portal session for the workspace's billing customer.

`POST /v3/workspaces/{workspaceSlug}/billing/portal`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `interfere billing invoices`

#### `interfere billing invoices list`

List workspace invoices to inspect billing history and payment status. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/billingInvoices.forOrg`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

---

### `interfere configuration`

#### `interfere configuration get-sdk-config`

Plugin and sourcing configuration for the authenticated surface's browser SDK.

`GET /v3/config`

---

### `interfere consumers`

#### `interfere consumers consumer-activity`

Returns activity for users within the requested workspace and time range.

`POST /v3/workspaces/{workspaceSlug}/consumers/activity`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `interfere consumers consumer-countries`

Returns the countries represented by the workspace's users.

`GET /v3/workspaces/{workspaceSlug}/consumers/countries`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |

#### `interfere consumers consumer-details`

Returns details for an application user in the workspace.

`GET /v3/workspaces/{workspaceSlug}/consumers/{endUserId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--end-user-id` | `string (uuid)` | Yes |  |

#### `interfere consumers consumer-lookup`

Finds an application user by email address or identifier and returns their profile, active problems, most recent error and recent sessions.

`GET /v3/workspaces/{workspaceSlug}/consumers/lookup`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--email` | `string` | Yes |  |

#### `interfere consumers get`

Read a tracked application user by internal user ID. This is distinct from an Interfere member or external identity. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/consumers.getById`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere consumers list`

List tracked application users to resolve an internal user ID before fetching activity, devices or sessions. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/consumers.list`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `interfere consumers page`

Read a page of tracked application users using the documented pagination and filter arguments. Do not invent cursor fields. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/consumers.getPageQuery`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `interfere consumers devices`

#### `interfere consumers devices by-end-user`

List devices associated with an application user, with each device's app (surface) even when that app was deleted (check surface.deletedAt). Resolve the user ID with consumers.list first. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/consumerDevices.byEndUser`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `interfere domains`

#### `interfere domains by-id-include-deleted`

Read a tracking domain by ID, including its deletion marker. Requires workspace domain read permission; null can also indicate insufficient access. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Requires org:workspace_domains:read; insufficient permission returns an access-denied error. An authorized read with no matching resource returns WORKSPACE_RESOURCE_NOT_FOUND (404). Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/domains.byIdIncludeDeleted`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere domains delete-tracking-domain`

Removes the named tracking domain from the workspace.

`DELETE /v3/workspaces/{workspaceSlug}/domains/{domainName}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--domain-name` | `string` | Yes |  |

#### `interfere domains domain-connect`

Returns connection information for a workspace tracking domain.

`GET /v3/workspaces/{workspaceSlug}/domains/connect`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--name` | `string` | Yes |  |
| `--redirect-uri` | `string (uri)` | Yes |  |

#### `interfere domains get`

Read a workspace domain by its record ID from domains.list. A domain record ID is distinct from its hostname. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/domains.byId`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere domains inspect-domain-dns`

Checks DNS configuration for a workspace tracking domain.

`GET /v3/workspaces/{workspaceSlug}/domains/dns`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--name` | `string` | Yes |  |

#### `interfere domains list`

List domains associated with the workspace, including their verification and enrollment configuration. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/domains.forOrg`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `interfere domains proxy-in-use`

Check whether a proxy domain is already in use before proposing a domain configuration change. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/domains.proxyInUse`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

---

### `interfere drain`

#### `interfere drain otlp-export`

Accepts OTLP telemetry in protobuf or JSON. The logs signal also accepts Vercel NDJSON. HTTP 2xx means accepted for asynchronous processing, not that telemetry is already queryable in Basin. Delivery is at least once: retries can produce duplicates, and downstream consumers must deduplicate on packet_id. Clients should not retry a successful response or OTLP partial-success response; retry transport failures and retryable errors with backoff, honoring Retry-After. Preserve the original payload on retries. Acceptance is not a guarantee against telemetry loss if background processing fails. Authentication, payload validation and admission run before acknowledgment.

`POST /drain/v1/{signal}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--signal` | `traces | logs | metrics` | Yes | OTLP signal |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `interfere feedback`

#### `interfere feedback submit-problem-feedback`

Records feedback on a problem in the workspace.

`POST /v3/workspaces/{workspaceSlug}/problems/{problemId}/feedback`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--problem-id` | `string (uuid)` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere feedback submit-workspace-feedback`

Submits workspace feedback to the Interfere team: a feature request, a bug in Interfere, or a problem that Interfere failed to detect (kind `missed`). Reuse the original request identity when retrying the same submission.

`POST /v3/workspaces/{workspaceSlug}/feedback`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `interfere findings`

#### `interfere findings list-by-problem`

List findings grouped under a problem, newest last-seen first, including related apps (surfaces). Resolve problemId with problems.list or problems.get; use finding IDs for trace endpoints. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/findings.listByProblem`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `interfere integrations`

#### `interfere integrations cloudflare-installation`

Legacy alias for connections.cloudflareInstallation. Read the workspace’s Cloudflare installation. No installation ID is required. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/integrations.cloudflareInstallation`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `interfere integrations cloudflare-installations`

Legacy alias for connections.cloudflareInstallations. List Cloudflare installations available in this workspace. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/integrations.cloudflareInstallations`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `interfere integrations connected-installations`

Legacy alias for connections.connectedInstallations. List connected integration installations for the required providers array. Inspect these before proposing installation or configuration changes. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/integrations.connectedInstallations`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere integrations create-integration-install-url`

Returns an installation URL for the requested integration and workspace.

`GET /v3/workspaces/{workspaceSlug}/integrations/{connectionId}/install`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--connection-id` | `github | ghe_cloud | ghe_server | vercel | cloudflare | slack | linear` | Yes |  |
| `--return-path` | `string` | No |  |
| `--host` | `string` | No |  |
| `--account-id` | `string` | No |  |

#### `interfere integrations detect-integrations`

During setup, call once with no signals to get the signal catalogue (signal id and what to look for), scan the repository and environment variable names locally, then call again with the matching signal ids and where you found each. Unknown signal ids are rejected. Returns every supported integration with its connection status, whether the company likely uses it, the evidence, and the next step; for likely but unconnected integrations the next step is createIntegrationInstallUrl, which the user completes in their browser. Never send secret values as evidence. Catalogued tools Interfere does not integrate with are listed under unsupported.

`POST /v3/workspaces/{workspaceSlug}/integrations/detect`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `interfere integrations enable-cli-integration`

Enables the CLI integration for the workspace.

`POST /v3/workspaces/{workspaceSlug}/integrations/cli/enable`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |

#### `interfere integrations enable-otel-integration`

Enables the managed OpenTelemetry integration for a workspace.

`POST /v3/workspaces/{workspaceSlug}/integrations/otel/enable`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |

#### `interfere integrations get`

Legacy alias for connections.byId. Read an integration record by ID. Use integrations.installations to discover installed integrations first. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/integrations.byId`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere integrations ghe-cloud-installation`

Legacy alias for connections.gheCloudInstallation. Read the workspace’s GitHub Enterprise Cloud installation. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/integrations.gheCloudInstallation`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `interfere integrations ghe-server-installation`

Legacy alias for connections.gheServerInstallation. Read the workspace’s GitHub Enterprise Server installation. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/integrations.gheServerInstallation`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `interfere integrations github-installation`

Legacy alias for connections.githubInstallation. Read the workspace’s GitHub installation. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/integrations.githubInstallation`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `interfere integrations github-repositories`

Legacy alias for connections.githubRepositories. List GitHub repository records available through the workspace's installation. Repository IDs are distinct from installation IDs. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/integrations.githubRepositories`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `interfere integrations installation`

Legacy alias for connections.installation. Read one integration installation by its documented arguments. Discover candidates with integrations.installations. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/integrations.installation`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere integrations installations`

Legacy alias for connections.installations. List integration installations to discover provider, connection state and installation identifiers. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/integrations.installations`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere integrations link-cli-destination`

Links an app (surface) to the workspace's CLI integration.

`POST /v3/workspaces/{workspaceSlug}/integrations/cli/destination`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere integrations link-surface-to-destination`

Link an app (surface) to an integration destination. Inspect the existing installation and mapping first. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/integrations.linkSurfaceToDestination`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere integrations link-surface-to-repository`

Link an app (surface) to a repository. Resolve repository and app IDs from their list queries first. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/integrations.linkSurfaceToRepository`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere integrations list-slack-channels`

Lists channels accessible through the workspace Slack integration.

`GET /v3/workspaces/{workspaceSlug}/integrations/slack/channels`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |

#### `interfere integrations refresh-github-repositories`

Refreshes repositories available to the connected GitHub installation.

`POST /v3/workspaces/{workspaceSlug}/integrations/github/repositories/refresh`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere integrations repositories`

Legacy alias for connections.repositories. List repository mappings for integrations. Inspect mappings before linking an app to a repository. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/integrations.repositories`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere integrations scan-github-repository`

Queues a repository scan. A scanning response acknowledges the durable job, not completed results.

`POST /v3/workspaces/{workspaceSlug}/integrations/github/apps/scan`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere integrations send-slack-test-message`

Sends a test message through the workspace Slack integration.

`POST /v3/workspaces/{workspaceSlug}/integrations/slack/test`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere integrations set-surface-destination-mapping`

Set an app's integration destination mapping. Read the existing mapping before replacing it. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/integrations.setSurfaceDestinationMapping`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere integrations slack-installation`

Legacy alias for connections.slackInstallation. Read the Slack installation and configuration for the workspace. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/integrations.slackInstallation`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `interfere integrations uninstall-integration`

Disconnects the specified integration from the workspace.

`POST /v3/workspaces/{workspaceSlug}/integrations/{connectionId}/uninstall`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--connection-id` | `github | ghe_cloud | ghe_server | vercel | cloudflare | slack | linear | cli | mcp` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `interfere integrations unlink-surface-destination-mapping`

Remove an app's destination mapping. This changes integration routing. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/integrations.unlinkSurfaceDestinationMapping`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere integrations unlink-surface-mapping`

Remove an app's integration mapping. Inspect the selected mapping before unlinking it. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/integrations.unlinkSurfaceMapping`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere integrations update-slack-chat-routing`

Change Slack chat routing for an integration. Inspect the installation and channel selection first. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/integrations.updateSlackChatRouting`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere integrations update-surface-mapping`

Update an existing app integration mapping using its current identifiers. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/integrations.updateSurfaceMapping`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere integrations vercel-installation`

Legacy alias for connections.vercelInstallation. Read the workspace’s Vercel installation. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/integrations.vercelInstallation`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `interfere integrations vercel-installations`

Legacy alias for connections.vercelInstallations. List Vercel installations available in this workspace. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/integrations.vercelInstallations`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

---

### `interfere keys private`

#### `interfere keys private create`

Creates a private API key with explicit scopes and expiration. Supply surfaceSlug for an app key; omit it for a workspace key. Grants cannot exceed the caller's permissions. Reuse the same idempotency key and request after an uncertain response.

`POST /v3/workspaces/{workspaceSlug}/api-keys`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere keys private create-for-surface`

Creates a secret API key. Reuse the same idempotencyKey and intent after a transport failure to recover the same creation attempt.

`POST /v3/workspaces/{workspaceSlug}/surfaces/{surfaceSlug}/apiKey`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--surface-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere keys private delete`

Revokes a workspace or app API key so it can no longer authorize API requests. Requires workspace authentication write permission for workspace keys or app write permission for app keys.

`DELETE /v3/workspaces/{workspaceSlug}/api-keys/{apiKeyId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--api-key-id` | `string (uuid)` | Yes |  |

#### `interfere keys private get`

Reads a workspace or app key's grants and expiration without retrieving its secret. Requires workspace authentication read permission for workspace keys or app read permission for app keys.

`GET /v3/workspaces/{workspaceSlug}/api-keys/{apiKeyId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--api-key-id` | `string (uuid)` | Yes |  |

#### `interfere keys private list`

List private API-key metadata across the workspace. This does not recover secret key material; use keys.private.listForSurface to narrow the result to one app. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/apiKeys.forOrg`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere keys private list-for-surface`

List private API-key metadata for an app (surface). Discover the app slug with surfaces.list; secret key material is not returned. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/apiKeys.forSurface`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere keys private revoke`

Revoke a private API key identified from keys.private.listForSurface. Clients using it may lose access. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/apiKeys.revoke`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere keys private rotate`

Rotates a secret API key. Reuse the same idempotencyKey and intent when retrying the rotation.

`POST /v3/workspaces/{workspaceSlug}/surfaces/{surfaceSlug}/apiKey/{apiKeyId}/roll`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--surface-slug` | `string` | Yes |  |
| `--api-key-id` | `string (uuid)` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere keys private update-name`

Rename a private API key identified from keys.private.listForSurface. The secret and its access are unchanged. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/apiKeys.updateName`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `interfere keys public`

#### `interfere keys public create`

Creates a publishable key for the app (surface).

`POST /v3/workspaces/{workspaceSlug}/surfaces/{surfaceSlug}/publicKeys`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--surface-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere keys public delete`

Revokes a publishable key and all its rotation grace-period values. Requires app write permission. Repeating revocation of an existing key succeeds without changing its revocation time.

`DELETE /v3/workspaces/{workspaceSlug}/surfaces/{surfaceSlug}/publicKeys/{publicKeyId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--surface-slug` | `string` | Yes |  |
| `--public-key-id` | `string (uuid)` | Yes |  |

#### `interfere keys public for-workspace`

List workspace-scoped publishable key records (surfaceId null). Every app in the workspace can ship a workspace key; these are separate from app keys and from private build API keys. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/publicKeys.forWorkspace`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere keys public get`

Reads a publishable key's identity, current value and revocation status. Requires app read permission. Revoked keys and keys belonging to deleted apps report revoked: true.

`GET /v3/workspaces/{workspaceSlug}/surfaces/{surfaceSlug}/publicKeys/{publicKeyId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--surface-slug` | `string` | Yes |  |
| `--public-key-id` | `string (uuid)` | Yes |  |

#### `interfere keys public list-for-surface`

List publishable key records for an app (surface). These are separate from private build API keys. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/publicKeys.forSurface`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere keys public revoke`

Revoke a publishable app key. SDK clients using it may stop sending accepted telemetry. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/publicKeys.revoke`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere keys public revoke-workspace-key`

Revoke a workspace publishable key. Every app shipping it stops authenticating; confirm before revoking the last live key. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/publicKeys.revokeWorkspaceKey`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere keys public rotate`

Rotates a publishable key for the app (surface).

`POST /v3/workspaces/{workspaceSlug}/surfaces/{surfaceSlug}/publicKeys/{publicKeyId}/roll`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--surface-slug` | `string` | Yes |  |
| `--public-key-id` | `string (uuid)` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere keys public update-name`

Rename a publishable app key. The key content and accepted telemetry are unchanged. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/publicKeys.updateName`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `interfere network`

#### `interfere network asn`

Look up an ASN already recorded in the shared catalog. This does not start an external ASN lookup. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/networkAccess.asn`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere network rules`

List workspace IP, CIDR and ASN access rules. An empty rule list means there is no configured network restriction. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/networkAccess.rules`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

---

### `interfere notifications`

#### `interfere notifications knock-token`

Returns a token for the authenticated user's notification feed.

`GET /v3/knock/token`

---

### `interfere notifications settings`

#### `interfere notifications settings mine`

Read the authenticated user's notification preferences. Inspect existing values before changing a workflow channel. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/notificationPreferences.mine`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `interfere notifications settings set-workflow-channel`

Change the authenticated user's channel preference for a notification workflow. Read notifications.settings.mine first. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/notificationPreferences.setWorkflowChannel`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `interfere problems`

#### `interfere problems action-requests`

List outstanding problem action-request timeline records for inspection. Reading does not approve an action. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/problems.actionRequests`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `interfere problems affected-by-problem`

Returns users affected by a problem within the requested time range.

`GET /v3/workspaces/{workspaceSlug}/problems/{problemId}/affectedUsers`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--problem-id` | `string (uuid)` | Yes |  |

#### `interfere problems affected-by-problem-timeseries`

Returns affected-user counts grouped into time buckets for a problem.

`GET /v3/workspaces/{workspaceSlug}/problems/{problemId}/affectedUsers/timeseries`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--problem-id` | `string (uuid)` | Yes |  |

#### `interfere problems dismiss`

Dismiss a problem. Inspect its current evidence and state before applying the requested dismissal. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/problems.dismiss`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere problems editable-properties`

Get a problem whose category, priority, and assignee can be edited after investigation confirmation. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/problems.editableProperties`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere problems get`

Read a visible problem with its app (surface), assignee and timeline, including comments and references. Obtain problemId from problems.list before investigating details. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/problems.getById`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere problems kanban`

List visible problems for a board, optionally filtered by app (surface) and environment patterns. Obtain app IDs and environment identifiers from surfaces.list and surfaces.environments.listForSurface. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/problems.kanban`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `interfere problems list`

List visible, non-deleted problems ordered by lastSeenAt descending, including assignees. Use IDs from this result for problem details, findings and affected-user endpoints. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/problems.list`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `interfere problems navigable`

List visible, non-deleted problems with each problem's newest customer-visible timeline event (detection, status change, observation, regression, action request, comment, root cause or fix). Order by the later of createdAt and that event's createdAt to rank problems by recent activity. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/problems.navigable`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `interfere problems previous-accounts`

Read archived investigation accounts preceding a Problem, scoped to the current Workspace. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/problems.previousAccounts`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere problems problem-evidence`

Returns the findings grouped into a problem with their fingerprints, representative occurrences with their exception messages, and the investigator's Observations with the evidence each one cited, such as source files and lines, Facts, spans and logs.

`GET /v3/workspaces/{workspaceSlug}/problems/{problemId}/evidence`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--problem-id` | `string (uuid)` | Yes |  |

#### `interfere problems problem-impact`

Returns the problem's occurrences and recorded affected users over the last 7 days, with its first and last seen times.

`GET /v3/workspaces/{workspaceSlug}/problems/{problemId}/impact`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--problem-id` | `string (uuid)` | Yes |  |

#### `interfere problems problem-related`

Returns the problem this one was merged into, problems merged into it, server problems whose errors ran in the same failing requests, and earlier problems with a recorded root cause in the same code area.

`GET /v3/workspaces/{workspaceSlug}/problems/{problemId}/related`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--problem-id` | `string (uuid)` | Yes |  |

#### `interfere problems problem-sessions`

Returns sessions associated with a problem in the workspace.

`GET /v3/workspaces/{workspaceSlug}/problems/{problemId}/sessions`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--problem-id` | `string (uuid)` | Yes |  |

#### `interfere problems problem-source`

Returns the repository linked to the failing code, the commit of the newest captured occurrence, and the builds that served after it, so a fix can be checked against the build that first contains it.

`GET /v3/workspaces/{workspaceSlug}/problems/{problemId}/source`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--problem-id` | `string (uuid)` | Yes |  |

#### `interfere problems problem-timeseries`

Returns daily occurrences and recorded affected-user counts for the last 30 UTC days.

`GET /v3/workspaces/{workspaceSlug}/problems/{problemId}/timeseries`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--problem-id` | `string (uuid)` | Yes |  |

#### `interfere problems restore`

Restore a problem from its dismissed state as allowed by the lifecycle rules. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/problems.restore`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere problems update-assignee`

Change a confirmed problem's assignee. Resolve the problem and workspace member IDs before writing. Unconfirmed problems are left unchanged. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/problems.updateAssignee`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere problems update-category`

Change a confirmed problem's category using a value allowed by the input schema. Unconfirmed problems are left unchanged. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/problems.updateCategory`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere problems update-details`

Update a problem's editable details. Read problems.get first and preserve fields the user did not ask to change. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/problems.updateDetails`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere problems update-priority`

Change a confirmed problem's priority using a value allowed by the input schema. Unconfirmed problems are left unchanged. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/problems.updatePriority`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `interfere problems timeline`

#### `interfere problems timeline create-comment`

Post a comment to a problem timeline. Resolve problemId first; this publishes content to other workspace members. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/timelineEvents.createComment`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere problems timeline respond-to-action`

Respond to a problem action request. Inspect problems.actionRequests and the problem timeline before approving or rejecting the proposed action. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/timelineEvents.respondToAction`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere problems timeline soft-delete-comment`

Remove a timeline comment using its event ID from problems.get. Verify authorship and target before acting. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/timelineEvents.softDeleteComment`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere problems timeline toggle-reaction`

Toggle a reaction on a timeline event. Repeating the same request can reverse the intended reaction. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/timelineEvents.toggleReaction`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere problems timeline update-comment`

Edit an existing problem timeline comment. Read the current text and event ID from problems.get first. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/timelineEvents.updateComment`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `interfere releases`

#### `interfere releases create-release`

Registers a release for the surface the build-time key belongs to.

`POST /v3/releases`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere releases finalize-source-maps`

Durably accepts all signed files and their verified manifest. R2 publication retries in the background.

`POST /v3/releases/{releaseSlug}/source-maps/finalize`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--release-slug` | `string` | Yes |  |
| `--environment` | `string` | No | Deploy environment of the build. Each environment of a release has its own source maps; omit it to use the source maps of the release. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere releases get-releases-config`

Resolves the surface and organization a build-time key belongs to.

`GET /v3/releases/config`

#### `interfere releases get-source-map-publication-status`

Reads the exact environment upload session and verifies its published manifest without changing upload state.

`GET /v3/releases/{releaseSlug}/source-maps`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--release-slug` | `string` | Yes |  |
| `--environment` | `string` | No | Deploy environment of the build. Each environment of a release has its own source maps; omit it to use the source maps of the release. |

#### `interfere releases latest-by-surface`

Read the latest release for the selected app (surface) using the documented arguments. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/releases.latestBySurface`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere releases list-for-surface`

List releases for an app (surface) to correlate a problem with deployed versions. Obtain the app slug from surfaces.list. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/releases.forSurface`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere releases recent-by-surface`

Read recent releases for the selected app (surface) to compare deployment timing with a regression. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/releases.recentBySurface`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere releases report-telemetry`

Records source-map upload timings for a release.

`POST /v3/releases/{releaseSlug}/telemetry`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--release-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere releases sign-source-map-uploads`

Initializes a source-map upload session and returns its expiry and file inventory.

`POST /v3/releases/{releaseSlug}/source-maps/sign`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--release-slug` | `string` | Yes |  |
| `--environment` | `string` | No | Deploy environment of the build. Each environment of a release has its own source maps; omit it to use the source maps of the release. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere releases upload-source-map`

Uploads a source-map chunk declared to the sign call. Accepts Content-Encoding: gzip; encoded and decoded chunks must each fit within 8 MiB. Offsets refer to decoded bytes. Send the sign response's uploadGrant as the X-Interfere-Upload-Grant header in place of the API key.

`PUT /v3/releases/{releaseSlug}/source-maps`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--release-slug` | `string` | Yes |  |
| `--environment` | `string` | No | Deploy environment of the build. Each environment of a release has its own source maps; omit it to use the source maps of the release. |
| `--offset` | `integer,null` | No |  |
| `--path` | `string` | Yes | Build-relative path of the source map, as declared to the sign call |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere releases upload-source-map-batch`

Atomically uploads source-map chunks from the signed inventory. Accepts Content-Encoding: gzip. The encoded and decoded JSON bodies must each fit within 8 MiB. Send the sign response's uploadGrant as the X-Interfere-Upload-Grant header in place of the API key.

`PUT /v3/releases/{releaseSlug}/source-maps/batch`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--release-slug` | `string` | Yes |  |
| `--environment` | `string` | No | Deploy environment of the build. Each environment of a release has its own source maps; omit it to use the source maps of the release. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `interfere replay`

#### `interfere replay session-replay-index`

Returns a page of replay chunk metadata for a recording.

`GET /v3/workspaces/{workspaceSlug}/consumers/{endUserId}/sessions/{sessionRecordId}/replay/index`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--end-user-id` | `string (uuid)` | Yes |  |
| `--session-record-id` | `string` | Yes |  |
| `--date-from` | `string` | Yes |  |
| `--date-to` | `string` | Yes |  |
| `--after` | `integer` | No |  |

---

### `interfere replay sessions`

#### `interfere replay sessions delete`

Delete a replay session. This is destructive; verify the replay key and requested recording before acting. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/replaySessions.delete`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere replay sessions get-by-key`

Read replay-session metadata by replaySessionKey. This key is not necessarily the analytics session record ID. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/replaySessions.getByKey`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere replay sessions list-by-keys`

Read replay-session metadata for explicit replay keys in one request. Do not substitute user or trace IDs. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/replaySessions.listByKeys`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere replay sessions list-removed-by-keys`

Read tombstones for explicit replay keys whose replay was deleted or expired by retention, to explain why a replay cannot play. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/replaySessions.listRemovedByKeys`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `interfere scim directories`

#### `interfere scim directories list`

List workspace SCIM provisioning directories and their enabled state. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/scimDirectories.forOrg`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `interfere scim directories set-enabled`

Enable or disable a SCIM provisioning directory. Inspect scim.directories.list first. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/workspaces.setScimDirectoryEnabled`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `interfere scim groups`

#### `interfere scim groups list`

List provisioned SCIM groups and their workspace role mappings. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/scimGroups.forOrg`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

---

### `interfere services`

#### `interfere services service-activity`

Returns the latest span time for each OpenTelemetry service in the workspace over the last 7 days. Services with no spans in that window are omitted. Also returns hourly span, log and metric point counts per service over the last 24 hours, aligned to `buckets`; services with no signals in that window are omitted from `signals`. `sdks` lists the Interfere SDK name and version that sent each namespace-less service's spans over the same 24 hours, with span counts.

`GET /v3/workspaces/{workspaceSlug}/services/activity`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |

#### `interfere services set-service-reportable-environment`

Sets the environment whose exceptions create findings and problems for an OpenTelemetry service, compared with the deployment.environment.name resource attribute. Exceptions from other or missing environments are still stored as telemetry. Null, the default, reports every environment.

`PUT /v3/workspaces/{workspaceSlug}/services/reportable-environment`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere services set-service-source`

Links an OpenTelemetry service to the repository Datasource and optional working directory that hold its code, or removes the link when source is null. Investigations read the commit from each Fact's service.version; no app or release is created.

`PUT /v3/workspaces/{workspaceSlug}/services/source`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `interfere sessions`

#### `interfere sessions create-session`

Records a device heartbeat for a session.

`POST /v3/session`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere sessions identify-session`

Attaches an application user identity to a session.

`POST /v3/session/identify`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere sessions list`

Lists sessions in descending last-event order. For the next page, send both cursor fields from the final session: its lastEventAt formatted as a UTC date-time and its sessionId. Continue until a page contains fewer than the requested limit.

`GET /v3/workspaces/{workspaceSlug}/consumers/{endUserId}/sessions`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--end-user-id` | `string (uuid)` | Yes |  |
| `--cursor-last-event-at` | `string` | No |  |
| `--cursor-session-id` | `string` | No |  |
| `--cursor-surface-id` | `string (uuid)` | No |  |
| `--cursor-device-id` | `string` | No |  |
| `--date-from` | `string (date)` | Yes |  |
| `--date-to` | `string (date)` | Yes |  |
| `--limit` | `integer` | No |  |
| `--surface-id` | `string (uuid)` | No |  |
| `--url` | `string` | No |  |

#### `interfere sessions session-batch-events`

Returns events keyed by session recording identity. Supply at most 10 identities, either as repeated query parameters or comma-separated values.

`GET /v3/workspaces/{workspaceSlug}/consumers/{endUserId}/sessions/events`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--end-user-id` | `string (uuid)` | Yes |  |
| `--date-from` | `string` | Yes |  |
| `--date-to` | `string` | Yes |  |
| `--session-record-ids` | `string[]` | Yes | Repeat this parameter for multiple app (surface) UUID:session ID identities. CSV values are also accepted, with a total limit of 10 identities after splitting. |

#### `interfere sessions session-detail`

Returns details of a session owned by the specified user.

`GET /v3/workspaces/{workspaceSlug}/consumers/{endUserId}/sessions/{sessionRecordId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--end-user-id` | `string (uuid)` | Yes |  |
| `--session-record-id` | `string` | Yes |  |
| `--date-from` | `string` | Yes |  |
| `--date-to` | `string` | Yes |  |

---

### `interfere sso connections`

#### `interfere sso connections list`

List enterprise authentication connections and their configuration for this workspace. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/enterpriseConnections.forOrg`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

---

### `interfere support messages`

#### `interfere support messages create`

Adds a reply to a workspace support conversation. Reuse the request identity on retry; do not repeat uncertain delivery as a new request.

`POST /v3/workspaces/{workspaceSlug}/support/threads/{threadId}/replies`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--thread-id` | `string (uuid)` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere support messages for-thread`

Read messages in a workspace support thread. Obtain the thread ID from support.threads.list. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/supportMessages.forThread`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `interfere support threads`

#### `interfere support threads create`

Creates a support conversation. Reuse the request identity on retry. A delivery-pending error means delivery is uncertain; do not submit a fresh request.

`POST /v3/workspaces/{workspaceSlug}/support/threads`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere support threads get`

Read a workspace support thread by ID. Use support.messages.forThread to retrieve its conversation. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/supportThreads.byId`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere support threads list`

List workspace support conversations and their state. Reading does not contact support. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/supportThreads.forOrg`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

---

### `interfere surface-keys`

#### `interfere surface-keys validate-key`

Do not assume a found Interfere key works. Call this for every public or secret key found in environment variables, .env files, Worker secrets or CI secrets before reusing it. The key is checked live against the same authentication the SDKs and build plugins use and compared with this workspace; it is never stored, logged or returned. Act on outcome and nextStep: only `valid` means the key can be reused here.

`POST /v3/workspaces/{workspaceSlug}/keys/validate`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `interfere surfaces`

#### `interfere surfaces create-surface`

Creates an app (surface) in the workspace. Reuse the original request identity when retrying the same creation attempt.

`POST /v3/workspaces/{workspaceSlug}/surfaces`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere surfaces delete`

Delete an app (surface). This is destructive; resolve its exact ID and slug with surfaces.list and verify the requested target. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/surfaces.delete`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere surfaces get`

Read one active app (surface) by its human-facing slug. Use surfaces.list to discover slugs and IDs; they are not interchangeable. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/surfaces.getBySlug`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere surfaces get-by-slug-include-deleted`

Read an app (surface) by slug even if deleted. Use only when investigating historical references; prefer surfaces.get for active resources. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Requires org:surfaces:read; insufficient permission returns an access-denied error. An authorized read with no matching resource returns WORKSPACE_RESOURCE_NOT_FOUND (404). Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/surfaces.getBySlugIncludeDeleted`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere surfaces list`

List active apps (surfaces) in the workspace. Start here to resolve an app name to its ID and slug before querying environments, releases or keys. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/surfaces.list`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `interfere surfaces set-anonymous-replay-policy`

Choose when an app (surface) records session replay for visitors that are not identified: always, on_error (only once an error occurs) or never. Identified users always record. Inspect its current setting first. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/surfaces.setAnonymousReplayPolicy`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere surfaces set-anonymous-user-tracking`

Enable or disable anonymous-user tracking for an app (surface). Inspect its current setting first. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/surfaces.setAnonymousUserTracking`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere surfaces set-onboarded`

Mark an app's (surface's) onboarding state after verifying the requested setup status. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/surfaces.setOnboarded`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere surfaces set-sdk-plugin`

Turn one browser SDK feature (replay, logs, pageEvents or rageClick) on or off for an app (surface). Other SDK settings are kept. Next.js and Vite apps bake this at build time, so it takes effect on their next build and deploy. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/surfaces.setSdkPlugin`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere surfaces surface-activity`

Returns activity for apps (surfaces) in the workspace.

`GET /v3/workspaces/{workspaceSlug}/surfaces/activity`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |

#### `interfere surfaces surface-events-timeseries`

Returns event counts grouped into time buckets for the app (surface).

`GET /v3/workspaces/{workspaceSlug}/surfaces/{surfaceSlug}/events/timeseries`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--surface-slug` | `string` | Yes |  |
| `--environment` | `string` | No |  |
| `--window` | `4h | 8h | 12h | 24h` | No |  |

#### `interfere surfaces surface-sdk-status`

Returns apps (surfaces) with outdated SDKs in the workspace, and those whose SDK is too old for problem and finding generation.

`GET /v3/workspaces/{workspaceSlug}/surfaces/activity/sdk-status`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |

#### `interfere surfaces update-name`

Rename an app (surface). Resolve its ID with surfaces.list and preserve the rest of its settings. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/surfaces.updateName`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `interfere surfaces environments`

#### `interfere surfaces environments delete`

Delete an app (surface) environment identified from surfaces.environments.listForSurface. Verify the intended app and environment. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/surfaceEnvironments.delete`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere surfaces environments for-inbox`

List app (surface) environments used for inbox filtering. Use returned identifiers when selecting an environment. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/surfaceEnvironments.forInbox`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `interfere surfaces environments list-for-surface`

List an app's (surface's) environments to resolve environment identifiers before filtering problems or activity. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/surfaceEnvironments.forSurface`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere surfaces environments set-production`

Mark an app (surface) environment as production or not. Problems seen only in non-production environments skip workspace Slack, personal Slack and desktop push notifications. Identify the environment from surfaces.environments.listForSurface. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/surfaceEnvironments.setProduction`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere surfaces environments set-reportable`

Turn Problem reporting for an app (surface) environment on or off. Off stops new Problems from that environment; existing Problems stay. Identify the environment from surfaces.environments.listForSurface. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/surfaceEnvironments.setReportable`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `interfere tenants`

#### `interfere tenants get`

Read a company inferred from user email domains within the workspace. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/tenants.getById`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere tenants get-context`

Returns the company's workspace tier with its badge shade, or null when the company has no tier.

`GET /v3/workspaces/{workspaceSlug}/tenants/{endUserOrgId}/context`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--end-user-org-id` | `string (uuid)` | Yes |  |

#### `interfere tenants list`

List companies inferred from user email domains. Use their IDs to relate users to the businesses using the application. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/tenants.list`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `interfere tenants merge`

Moves every domain and user of the absorbed company onto the kept company, keeps the kept company's tier (filling a blank from the absorbed one), and removes the absorbed company from the list. Refuses when both companies carry different tiers. Safe to repeat: a pair that is already merged returns alreadyMerged. Requires workspace admin.

`POST /v3/workspaces/{workspaceSlug}/tenants/merge`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere tenants page`

List companies with optional name/domain search, last-seen and tier filters. Related users are capped at 100 per company; this is not an exact user count. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/tenants.getPageQuery`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere tenants seen-user`

Read the first- or last-seen live user for a company. Supply its endUserOrgId and choose the first or last edge. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/tenants.seenUser`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere tenants update-context`

Sets or clears the company's tier. null clears it. Tier names are trimmed, case-sensitive and created within the workspace (with the gray shade) when first assigned.

`PATCH /v3/workspaces/{workspaceSlug}/tenants/{endUserOrgId}/context`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--end-user-org-id` | `string (uuid)` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere tenants update-contexts`

Atomically sets or clears the tier of up to 100 distinct companies. Unknown, deleted, or foreign companies reject the entire batch. null clears a tier.

`PATCH /v3/workspaces/{workspaceSlug}/tenants/context`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `interfere tenants tiers`

#### `interfere tenants tiers list`

List the workspace-managed company tiers with their badge shade, ordered by value. Requires tiers read permission; a company references a tier by its value. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/tiers.list`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

---

### `interfere tiers`

#### `interfere tiers create`

Creates a workspace company tier. Names are trimmed, case-sensitive and unique within the workspace; the shade defaults to gray. Requires tier write permission.

`POST /v3/workspaces/{workspaceSlug}/tiers`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere tiers delete`

Deletes a company tier and clears it from every company that carried it, returning how many companies were cleared. Requires tier write permission.

`DELETE /v3/workspaces/{workspaceSlug}/tiers/{value}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--value` | `string` | Yes |  |

#### `interfere tiers list`

Lists the workspace's company tiers in name order with their badge shade. Companies reference a tier by its name.

`GET /v3/workspaces/{workspaceSlug}/tiers`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |

#### `interfere tiers update`

Renames a company tier or changes its shade. Renaming moves every company on the tier to the new name. Requires tier write permission.

`PATCH /v3/workspaces/{workspaceSlug}/tiers/{value}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--value` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `interfere traces`

#### `interfere traces finding-occurrences`

Lists the traces where a finding occurred, newest first.

`GET /v3/findings/{findingId}/traces`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--finding-id` | `string` | Yes | Finding whose traces are requested |
| `--cursor` | `string` | No |  |
| `--environment` | `string` | No |  |
| `--limit` | `integer` | No |  |
| `--release-slug` | `string` | No |  |

#### `interfere traces span-detail`

Reads one span exactly as its instrumentation reported it, including the raw OTel attribute blob the trace read omits.

`GET /v3/findings/{findingId}/traces/{traceId}/spans/{spanId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--finding-id` | `string` | Yes | Finding whose traces are requested |
| `--trace-id` | `string` | Yes | W3C trace id |
| `--span-id` | `string` | Yes | W3C span id |
| `--ended-at-unix-nano` | `string` | No | Optional endedAtUnixNano from GET /v3/findings/{findingId}/traces/{traceId}; narrows the read to the trace's extent within the window days |
| `--started-at-unix-nano` | `string` | No | Optional startedAtUnixNano from GET /v3/findings/{findingId}/traces/{traceId}; narrows the read to the trace's extent within the window days |
| `--window-end` | `string (date)` | Yes | windowEnd is required: pass windowEnd from GET /v3/findings/{findingId}/traces/{traceId} |
| `--window-start` | `string (date)` | Yes | windowStart is required: pass windowStart from GET /v3/findings/{findingId}/traces/{traceId} |

#### `interfere traces span-duration-history`

Reads the recent durations of one span identity, oldest first, with the median over the sampled window. Identity is the (serviceName, name) pair the span row already carries; kind narrows the read to one granule range.

`GET /v3/findings/{findingId}/span-duration-history`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--finding-id` | `string` | Yes | Finding whose traces are requested |
| `--kind` | `string` | Yes |  |
| `--name` | `string` | Yes |  |
| `--service-name` | `string` | Yes |  |

#### `interfere traces span-stack`

Reads exception frames for the selected span within its trace window.

`GET /v3/findings/{findingId}/traces/{traceId}/spans/{spanId}/stack`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--finding-id` | `string` | Yes | Finding whose traces are requested |
| `--trace-id` | `string` | Yes | W3C trace id |
| `--span-id` | `string` | Yes | W3C span id |
| `--ended-at-unix-nano` | `string` | No | Optional endedAtUnixNano from GET /v3/findings/{findingId}/traces/{traceId}; narrows the read to the trace's extent within the window days |
| `--started-at-unix-nano` | `string` | No | Optional startedAtUnixNano from GET /v3/findings/{findingId}/traces/{traceId}; narrows the read to the trace's extent within the window days |
| `--window-end` | `string (date)` | Yes | windowEnd is required: pass windowEnd from GET /v3/findings/{findingId}/traces/{traceId} |
| `--window-start` | `string (date)` | Yes | windowStart is required: pass windowStart from GET /v3/findings/{findingId}/traces/{traceId} |

#### `interfere traces trace-activity`

Reads nearby activity correlated to a finding occurrence by session and page view. These records can belong to separate execution traces.

`GET /v3/findings/{findingId}/traces/{traceId}/activity`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--finding-id` | `string` | Yes | Finding whose traces are requested |
| `--trace-id` | `string` | Yes | W3C trace id |
| `--limit` | `integer` | No |  |
| `--occurred-at` | `string` | Yes | occurredAt is required: pass the trace's firstAtUnixNano from GET /v3/findings/{findingId}/traces |

#### `interfere traces trace-detail`

Reads every retained span in one trace of a finding.

`GET /v3/findings/{findingId}/traces/{traceId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--finding-id` | `string` | Yes | Finding whose traces are requested |
| `--trace-id` | `string` | Yes | W3C trace id |
| `--limit` | `integer` | No |  |
| `--occurred-at` | `string` | Yes | occurredAt is required: pass the trace's firstAtUnixNano from GET /v3/findings/{findingId}/traces |

#### `interfere traces trace-logs`

Reads the log records emitted within one trace of a finding, oldest first. A generic exception message often hides the condition that a log line in the same trace names.

`GET /v3/findings/{findingId}/traces/{traceId}/logs`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--finding-id` | `string` | Yes | Finding whose traces are requested |
| `--trace-id` | `string` | Yes | W3C trace id |
| `--limit` | `integer` | No |  |
| `--occurred-at` | `string` | Yes | occurredAt is required: pass the trace's firstAtUnixNano from GET /v3/findings/{findingId}/traces |

---

### `interfere users`

#### `interfere users list`

List workspace members and roles. Use member IDs from this result when assigning problems or proposing membership changes. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/memberships.forOrg`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `interfere users me`

Read the authenticated Interfere user's profile. This is not a tracked user of a customer's application. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/users.me`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `interfere users membership`

Read the authenticated user's membership in this workspace, including its role. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/memberships.mineForOrg`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `interfere users memberships`

List memberships visible for the authenticated user. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/memberships.mine`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `interfere users remove`

Remove a member's workspace access. Resolve the member from users.list and verify the intended person. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/members.remove`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere users update-locale`

Change the authenticated user's locale preference using a supported value from the schema. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/users.updateLocale`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere users update-name`

Change the authenticated Interfere user's name. This does not change a tracked application user profile. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/users.updateName`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere users update-role`

Change a member's workspace role and permissions. Read users.list and verify the selected person and role first. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/members.updateRole`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere users update-timezone`

Change the authenticated user's timezone preference using the schema's accepted timezone format. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/users.updateTimezone`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `interfere users invitations`

#### `interfere users invitations create`

Invite a member to the workspace with the requested role. Check existing memberships and invitations first to avoid duplicate invitations. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/members.invite`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere users invitations list`

List workspace invitations and their current state. Reading invitations does not send or resend email. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/invitations.forOrg`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `interfere users invitations mine`

List invitations associated with the current user. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/invitations.forUser`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `interfere users invitations pending`

List pending invitations so existing invitations can be inspected before inviting a member again. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/invitations.pending`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `interfere users invitations resend`

Resend an existing invitation email. Inspect users.invitations.pending first; this contacts the invitee. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/members.resendInvitation`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere users invitations revoke`

Revoke a pending workspace invitation so it can no longer be accepted. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/members.revokeInvitation`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `interfere users requests`

#### `interfere users requests accept`

Approve a membership request and grant workspace membership. Resolve the request from users.requests.pending. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/members.acceptRequest`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere users requests list`

List requests to join the workspace. Reading does not approve or reject a request. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/membershipRequests.forOrg`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `interfere users requests mine`

List workspace membership requests associated with the current user. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/membershipRequests.forUser`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `interfere users requests pending`

List pending workspace membership requests that may need administrator review. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Permission-filtered queries may return an empty result; do not assume that proves the resource does not exist. Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/membershipRequests.pending`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `interfere users requests reject`

Reject a pending workspace membership request identified from users.requests.pending. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/members.rejectRequest`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `interfere workspaces`

#### `interfere workspaces add-proxy-domain`

Add a tracking proxy domain to the workspace. Check domains.list and domains.proxyInUse first. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/workspaces.addProxyDomain`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere workspaces complete-onboarding`

Mark workspace onboarding complete after the required setup has actually been performed. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/workspaces.completeOnboarding`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `interfere workspaces confirm-domain-ownership`

Record that a workspace domain's DNS ownership is verified, after the sign-in service has confirmed the TXT record. Fails while ownership is still pending. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/workspaces.confirmDomainOwnership`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere workspaces create-workspace-public-key`

Creates a publishable key for the whole workspace. Every app in the workspace can ship it; the gateway resolves the app from the telemetry. Requires workspace authentication write permission. Reuse the same idempotency key and request after an uncertain response.

`POST /v3/workspaces/{workspaceSlug}/publicKeys`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere workspaces current`

Read the authenticated workspace's current details and settings. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation is read-only despite using POST. Requires org:workspace_basics:read; insufficient permission returns an access-denied error. An authorized read with no matching resource returns WORKSPACE_RESOURCE_NOT_FOUND (404). Only use pagination arguments declared in the schema.

`POST /v3/workspaces/{workspaceSlug}/queries/workspaces.current`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `interfere workspaces delete-workspace-avatar`

Removes the avatar of a workspace the caller can manage.

`DELETE /v3/workspaces/{workspaceSlug}/avatar`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |

#### `interfere workspaces destroy`

Delete the workspace. This is a destructive administrative operation; verify the exact workspace and explicit deletion intent. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/workspaces.destroy`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere workspaces lookup-network-asn`

Ensures an ASN catalog record exists and schedules a registry lookup. Metadata is delivered through Zero.

`POST /v3/workspaces/{workspaceSlug}/network/asns`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere workspaces network-access`

Returns the trusted connection metadata after enforcing the dashboard network policy.

`GET /v3/workspaces/{workspaceSlug}/network/access`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |

#### `interfere workspaces read-network-rules`

Returns the networks allowed to reach the workspace dashboard and API. An empty list means no restriction. Requires workspace authentication read permission.

`GET /v3/workspaces/{workspaceSlug}/network/rules`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |

#### `interfere workspaces read-workspace-public-key`

Reads a workspace publishable key's identity, current value and revocation status. Requires workspace authentication read or app read permission.

`GET /v3/workspaces/{workspaceSlug}/publicKeys/{publicKeyId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--public-key-id` | `string (uuid)` | Yes |  |

#### `interfere workspaces remove-proxy-domain`

Remove a tracking proxy domain. This can affect telemetry sent through that hostname. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/workspaces.removeProxyDomain`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere workspaces reset-demo-workspace`

Resets a demo workspace. Reuse the same request identity when retrying an interrupted reset.

`POST /v3/workspaces/{workspaceSlug}/demo/reset`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere workspaces revoke-workspace-public-key`

Revokes a workspace publishable key and all its rotation grace-period values. Every app shipping this key stops authenticating, so confirm with the user before revoking the last live workspace key. Requires workspace authentication write permission. Repeating revocation of an existing key succeeds without changing its revocation time.

`DELETE /v3/workspaces/{workspaceSlug}/publicKeys/{publicKeyId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--public-key-id` | `string (uuid)` | Yes |  |

#### `interfere workspaces roll-workspace-public-key`

Rotates a workspace publishable key. The previous value keeps authenticating for retireAfterSeconds (seven days by default) so every app can ship the new value. Requires workspace authentication write permission.

`POST /v3/workspaces/{workspaceSlug}/publicKeys/{publicKeyId}/roll`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--public-key-id` | `string (uuid)` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere workspaces save-network-rules`

Replaces the networks allowed to reach the workspace dashboard and API, and records an audit event. An empty list removes all restrictions. The saving administrator's connection must remain allowed.

`PUT /v3/workspaces/{workspaceSlug}/network/rules`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere workspaces set-enterprise-connection-active`

Enable or disable an enterprise authentication connection. This changes how members can sign in. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/workspaces.setEnterpriseConnectionActive`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere workspaces set-workspace-avatar`

Uploads an image for the workspace. Send the file as multipart/form-data.

`PUT /v3/workspaces/{workspaceSlug}/avatar`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere workspaces update-auto-assign-problems`

Turn automatic assignment of confirmed problems on or off for the workspace. Manual assignments are never overwritten. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/workspaces.updateAutoAssignProblems`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere workspaces update-auto-create-apps`

Turn automatic app creation from workspace-key telemetry on or off. Unknown services then land in the default app. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/workspaces.updateAutoCreateApps`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere workspaces update-basics`

Update workspace name or other basic settings described by the input schema. Read workspaces.current first. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/workspaces.updateBasics`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere workspaces update-default-surface`

Set the app that receives workspace-key telemetry without a service name. Resolve the app ID from surfaces.list first. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/workspaces.updateDefaultSurface`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere workspaces update-enterprise-connection`

Update an enterprise authentication connection. Resolve its ID from sso.connections.list. Pass the workspace slug in the URL and operation arguments inside the JSON body as { args: ... }. Inspect the request schema for required fields and exact identifier formats. This operation writes data under the caller's existing permissions and records its audit event. Do not automatically retry after a timeout; read the affected resource to determine whether the write completed.

`POST /v3/workspaces/{workspaceSlug}/actions/workspaces.updateEnterpriseConnection`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--workspace-slug` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere workspaces workspace-create`

Creates a workspace for the authenticated user, subject to workspace creation permissions.

`POST /v3/workspaces`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `interfere workspaces workspace-gate`

Returns memberships, invitations, suggestions, and whether the user can create a workspace.

`GET /v3/workspaces/gate`

---

## Global flags

These flags are available on every command:

| Flag | Description |
|------|-------------|
| `--dry-run` | Print the HTTP request without sending it |
| `--json <JSON\|->` | Supply the request body as JSON (or `-` for stdin) |
| `--params <JSON>` | Merge extra parameters as JSON |
| `--format <json\|table\|yaml\|csv>` | Output format (default: `json`) |
| `--output <PATH>` | Write binary responses to a file |
| `--base-url <URL>` | Override the API base URL |
| `--no-extract` | Print the full response body instead of the `x-fern-sdk-return-value` extraction |
| `--no-retry` | Disable retries declared by `x-fern-retries`, including network errors |
| `-q, --quiet` | Suppress stdout on success |
| `-h, --help` | Print help |
| `-V, --version` | Print version |

Operations the spec describes how to page (via `x-fern-pagination` or a root `page_token` parameter) also accept:

| Flag | Description |
|------|-------------|
| `--page-all` | Auto-paginate and stream all results |
| `--page-limit <N>` | Max pages to fetch (default: `10`) |
| `--page-delay <MS>` | Delay between page fetches in milliseconds (default: `100`) |
| `--no-pager` | Disable the pager even on interactive terminals |

Operations the spec marks as streaming (via `x-fern-streaming`) also accept:

| Flag | Description |
|------|-------------|
| `--no-stream` | Buffer the streaming response and print it as a single value once complete |
