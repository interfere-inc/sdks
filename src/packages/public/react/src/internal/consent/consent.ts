import { PLUGIN_MANIFEST } from "@interfere/types/sdk/plugins/manifest";
import type {
  ConsentCategory,
  ConsentInput,
  ConsentState,
  GateableCategory,
  PluginKey,
} from "@interfere/types/sdk/plugins/manifest";

export const DEFAULT_CONSENT = {
  analytics: true,
  replay: true,
} as const satisfies ConsentState;

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
  consentState: ConsentState | null,
) {
  return (
    category === "necessary" ||
    consentState === null ||
    consentState[category] === true
  );
}

export function resolveGrantedConsent(consent?: ConsentInput) {
  if (consent === true || consent === false) {
    return { analytics: consent, replay: consent } satisfies ConsentState;
  }

  return { ...DEFAULT_CONSENT, ...consent } satisfies ConsentState;
}

export function hasConsentChanged(
  current: ConsentState | null,
  next: ConsentState | null,
) {
  for (const category of GATEABLE_CATEGORIES) {
    if (current?.[category] !== next?.[category]) {
      return true;
    }
  }

  return false;
}
