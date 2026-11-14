import { EventType, IncrementalSource, NodeType } from "@rrweb/types";
import type {
  attributeMutation,
  attributes as elementAttributes,
  eventWithTime,
  serializedNodeWithId,
} from "@rrweb/types";

const URL_ATTRIBUTES = [
  "action",
  "cite",
  "data",
  "formaction",
  "href",
  "poster",
  "src",
  "xlink:href",
];

type RedactUrl = (url: string) => string;

function redactSrcset(srcset: string, redactUrl: RedactUrl) {
  return srcset
    .split(",")
    .map((candidate) => {
      const trimmed = candidate.trim();
      const space = trimmed.search(/\s/u);

      return space === -1
        ? redactUrl(trimmed)
        : `${redactUrl(trimmed.slice(0, space))}${trimmed.slice(space)}`;
    })
    .join(", ");
}

function redactAttributes(
  attributes: attributeMutation["attributes"] | elementAttributes,
  redactUrl: RedactUrl,
) {
  for (const name of URL_ATTRIBUTES) {
    const value = attributes[name];

    // oxlint-disable-next-line anti-slop/no-runtime-typeof -- rrweb attribute values may be strings, numbers, booleans, style objects or null; only strings hold URLs.
    if (typeof value === "string") {
      attributes[name] = redactUrl(value);
    }
  }

  const { srcset } = attributes;

  // oxlint-disable-next-line anti-slop/no-runtime-typeof -- rrweb attribute values may be strings, numbers, booleans, style objects or null; only strings hold URLs.
  if (typeof srcset === "string") {
    attributes["srcset"] = redactSrcset(srcset, redactUrl);
  }
}

function redactNode(node: serializedNodeWithId, redactUrl: RedactUrl) {
  if (node.type === NodeType.Element) {
    redactAttributes(node.attributes, redactUrl);
  }

  if (node.type === NodeType.Element || node.type === NodeType.Document) {
    for (const child of node.childNodes) {
      redactNode(child, redactUrl);
    }
  }
}

/** Rewrites URLs that rrweb records in page metadata and DOM attributes before the event leaves the browser. */
export function redactReplayEvent(event: eventWithTime, redactUrl: RedactUrl) {
  if (event.type === EventType.Meta) {
    event.data.href = redactUrl(event.data.href);

    return;
  }

  if (event.type === EventType.FullSnapshot) {
    redactNode(event.data.node, redactUrl);

    return;
  }

  if (
    event.type === EventType.IncrementalSnapshot &&
    event.data.source === IncrementalSource.Mutation
  ) {
    for (const added of event.data.adds) {
      redactNode(added.node, redactUrl);
    }

    for (const mutation of event.data.attributes) {
      redactAttributes(mutation.attributes, redactUrl);
    }
  }
}
