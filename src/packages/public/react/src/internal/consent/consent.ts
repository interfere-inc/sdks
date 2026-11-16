import { PLUGIN_MANIFEST } from "@interfere/types/sdk/plugins/manifest";
import type {
  ConsentCategory,
  ConsentDecision,
  ConsentState,
  GateableCategory,
  PluginKey,
} from "@interfere/types/sdk/plugins/manifest";

export const DEFAULT_CONSENT = Object.freeze({
  analytics: true,
  replay: true,
}) satisfies ConsentState;

export const PENDING_CONSENT = Object.freeze({
  analytics: false,
  replay: false,
}) satisfies ConsentState;

// oxlint-disable-next-line typescript/no-unsafe-type-assertion
const PLUGIN_CONSENT_BY_KEY = Object.fromEntries(
  PLUGIN_MANIFEST.map((plugin) => [plugin.name, plugin.consentCategory]),
) as Record<PluginKey, ConsentCategory>;

// oxlint-disable-next-line typescript/no-unsafe-type-assertion
const GATEABLE_CATEGORIES = Object.keys(DEFAULT_CONSENT) as GateableCategory[];

export function getPluginConsentCategory(key: PluginKey) {
  return PLUGIN_CONSENT_BY_KEY[key];
}

export function isConsentAllowed(
  category: ConsentCategory,
  consentState: ConsentState,
) {
  return category === "necessary" || consentState[category];
}

export function resolveConsentDecision(decision: ConsentDecision) {
  if (decision === true || decision === false) {
    return { analytics: decision, replay: decision } satisfies ConsentState;
  }

  return { analytics: true, replay: decision.replay } satisfies ConsentState;
}

export function hasConsentChanged(current: ConsentState, next: ConsentState) {
  return GATEABLE_CATEGORIES.some(
    (category) => current[category] !== next[category],
  );
}
