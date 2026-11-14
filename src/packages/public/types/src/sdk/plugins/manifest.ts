export const CONSENT_CATEGORIES = ["necessary", "analytics", "replay"] as const;

type AtLeastOne<T> = {
  [K in keyof T]-?: Required<Pick<T, K>> & Partial<T>;
}[keyof T];

export const PLUGIN_MANIFEST = [
  {
    consentCategory: "necessary",
    defaultEnabled: true,
    events: [{ enrich: true, name: "error" }],
    name: "errors",
  },
  {
    consentCategory: "analytics",
    defaultEnabled: true,
    events: [],
    name: "device",
  },
  {
    consentCategory: "analytics",
    defaultEnabled: true,
    events: [
      { enrich: true, name: "pageview" },
      { enrich: true, name: "pageleave" },
      { enrich: true, name: "ui_event" },
    ],
    name: "pageEvents",
  },
  {
    consentCategory: "analytics",
    defaultEnabled: true,
    events: [{ enrich: true, name: "rage_click" }],
    name: "rageClick",
  },
  {
    consentCategory: "replay",
    defaultEnabled: true,
    events: [{ enrich: false, name: "replay_chunk" }],
    name: "replay",
  },
  {
    consentCategory: "analytics",
    defaultEnabled: true,

    events: [],
    name: "logs",
  },
] as const;

export type PluginManifest = typeof PLUGIN_MANIFEST;

export type ConsentCategory = (typeof CONSENT_CATEGORIES)[number];

export type PluginKey = PluginManifest[number]["name"];

export type GateableCategory = Exclude<ConsentCategory, "necessary">;

export type ConsentState = AtLeastOne<Record<GateableCategory, boolean>>;

/** Current permission for analytics and replay, together or by category. */
export type ConsentInput = boolean | ConsentState;
