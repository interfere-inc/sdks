# Interfere CLI

Command-line access to the Interfere API for developers and coding agents.

```sh
npm install -g @interfere/cli
interfere auth login --scheme oauth-production
export INTERFERE_WORKSPACE=your-workspace
interfere problems list
```

The CLI opens your browser for sign-in and organization selection. Authentication uses PKCE and stores credentials in your operating system's credential store. Linux static builds use a file restricted to your user. Expired access tokens refresh automatically.

## Sign in without a browser

```sh
interfere login --device
```

Device sign-in prints a one-time code and a URL. Open the URL from any browser, enter the code, and the CLI finishes signing in on its own. `interfere login` and `interfere auth login` choose device sign-in automatically over SSH, in CI, or on Linux without a display; pass `--no-browser` to print the URL instead of opening it. Device sign-in stores the same credential as browser sign-in, so every later command works the same way. Device sign-in must be enabled on the Interfere OAuth application; until it is, the CLI says so and points back to browser sign-in or `INTERFERE_ACCESS_TOKEN`.

## Discover commands

```sh
interfere --help
interfere problems --help
interfere problems get --schema
interfere problems get --args.problem-id PROBLEM_ID
interfere audit list
interfere keys private --help
interfere billing invoices list
interfere users invitations list
interfere notifications settings --help
interfere scim --help
interfere support --help
```

API commands come from the published API contract and support `--help`, `--schema`, and `--dry-run`. Use `--schema` to discover arguments before constructing a request. Dry runs show the request without sending it; the API still performs final validation when you execute it.

The command tree follows product workflows. Internal entitlements, subscriptions, presence, fingerprint rollups, and replay chunks are excluded from public discovery.

## Inspect a problem

```sh
interfere problems inspect PROBLEM_ID --workspace your-workspace
interfere problems inspect PROBLEM_ID --query '{problem: problem, findings: findings}' --format json
interfere problems inspect PROBLEM_ID --dry-run
```

`problems inspect` combines problem details, its timeline, and related findings. It exits with an error if either request fails. Its dry run lists the planned requests without contacting the API. This composite command supports `--help`; use `problems get --schema` and `findings list-by-problem --schema` for the underlying API schemas.

## Output for scripts and agents

Piped output defaults to JSON. Use `--format json` to request JSON explicitly, or `--query` to select fields with JMESPath. Errors in machine output modes are JSON on stdout with a nonzero exit status. Check the exit status before using the output as a successful response.

```sh
interfere problems list --format json
interfere problems get --args.problem-id PROBLEM_ID --dry-run
```

## Update company context

```sh
interfere companies update-context --end-user-org-id COMPANY_ID \
  --json '{"tier":"Enterprise"}'
interfere companies get-context --end-user-org-id COMPANY_ID
interfere companies update-contexts \
  --json '{"companies":[{"endUserOrgId":"COMPANY_ID","tier":"Enterprise"}]}'
```

Use `--json` to set the tier; `"tier":null` clears it. Tier names are trimmed, case-sensitive and unique within the workspace; assigning a new name creates that tier with the gray shade. Manage tier names and shades with the tiers endpoints (`GET|POST /v3/workspaces/{slug}/tiers`, `PATCH|DELETE /v3/workspaces/{slug}/tiers/{value}`). Bulk updates accept up to 100 distinct existing companies and either all succeed or all fail.

Company IDs come from `interfere companies list`. Reading and updating company context require the corresponding workspace permissions.

## Credentials and workspaces

`INTERFERE_ACCESS_TOKEN` supplies a user access token for noninteractive runs. Build API operations use a workspace secret. The CLI currently reads it from `INTERFERE_SECRET_KEY`; the SDK build plugins read the same kind of credential from `INTERFERE_API_KEY`. Each credential applies only to API operations that accept it. Never put workspace secrets in browser code.

Set `INTERFERE_WORKSPACE` or pass `--workspace` for workspace operations. Named profiles can store workspace defaults and credentials. Use `interfere profiles --help` for profile commands. Explicit flags override profile values, and profile values override environment variables.

## Services and workspace keys

Interfere organizes telemetry by service. `serviceName`, with an optional `serviceNamespace`, names the service, and every request carries it: SDK requests send `x-interfere-service` and `x-interfere-service-namespace`, and OTLP requests use the `service.name` and `service.namespace` resource attributes. Interfere records a service the first time it sees one, within the workspace's service limit.

Build plugins use a workspace secret with `release:write` and a service name to register releases and bake release tokens into browser bundles. Servers and collectors use a workspace secret with `org:telemetry:write`. Source-map publication is automatic in the framework plugins.

Keep the service name aligned between the build plugin, browser initialization and server registration. This is telemetry identity, separate from the CLI's `--workspace` selection.

### Legacy credentials

`--public-key` and `INTERFERE_PUBLIC_KEY` are deprecated legacy inputs for older API operations. They remain documented for compatibility, not for new SDK setup. Per-App keys are deprecated; migrate builds to workspace secrets and release tokens.

Use `interfere keys public --help` to inspect the legacy key-management commands.

### Environment variables

| Variable                 | Description                                                                             |
| ------------------------ | --------------------------------------------------------------------------------------- |
| `INTERFERE_PUBLIC_KEY`   | Deprecated legacy public key for older API operations; not used in new SDK setup.       |
| `INTERFERE_SECRET_KEY`   | CLI credential source for a workspace build secret; SDKs use INTERFERE_API_KEY instead. |
| `INTERFERE_ACCESS_TOKEN` | User access token for noninteractive API commands.                                      |
| `INTERFERE_WORKSPACE`    | Default workspace for workspace operations.                                             |

## Preview environment

```sh
export INTERFERE_ENVIRONMENT=preview
interfere auth login --scheme oauth-preview
interfere problems list --workspace your-preview-workspace
```

The default environment is `production`. Preview uses a separate OAuth client, API endpoint, and stored credential. Use separate profiles for workspaces in different environments.

## Supported systems

Native binaries are available for macOS on Apple Silicon and Intel, Linux on ARM64 and x64, and Windows on x64. npm selects the platform package through optional dependencies, so keep optional dependencies enabled. Installation does not run a download script.
