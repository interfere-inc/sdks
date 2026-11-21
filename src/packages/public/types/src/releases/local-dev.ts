import type { CreateReleaseRequest } from "./definition.ts";
import { LOCAL_DEV_RELEASE_SLUG } from "./slug.ts";

/** The one release a service's local development servers share: no destination, no commit, so it is never deployed, never matched to a deployment and never supersedes another release. A later ready release may still cancel it; its tokens keep working, because ingest checks only revoked and deleted. */
export function localDevReleaseRequest(
  input: Pick<CreateReleaseRequest, "producerVersion" | "service">,
) {
  return {
    buildId: LOCAL_DEV_RELEASE_SLUG,
    destination: null,
    producerVersion: input.producerVersion,
    service: input.service,
    slug: LOCAL_DEV_RELEASE_SLUG,
    source: {
      branch: "local",
      commitMessage: "",
      commitSha: null,
      provider: "github",
    },
  } satisfies CreateReleaseRequest;
}
