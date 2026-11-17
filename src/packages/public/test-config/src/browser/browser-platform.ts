export interface BrowserMetadataOptions {
  userAgent: string;
  language: string;
  screen: { width: number; height: number };
  clientHints?: {
    brands?: { brand: string; version: string }[];
    fullVersionList?: { brand: string; version: string }[];
    platform: string;
    platformVersion: string;
    architecture: string;
    model: string;
    mobile: boolean;
    bitness?: string;
  };
}

export { commands } from "vitest/browser";

declare module "vitest/browser" {
  interface BrowserCommands {
    setVirtualAuthenticator: (
      enabled: boolean,
      automaticPresenceSimulation?: boolean,
    ) => Promise<void>;
    closeBrowserPopups: () => Promise<void>;
    collectGarbage: () => Promise<void>;
    observePageEvents: (running: boolean) => Promise<void>;
    parkPointer: () => Promise<void>;
    setBrowserMetadata: (
      metadata: BrowserMetadataOptions | "reset",
    ) => Promise<void>;
    setClipboardAccess: (allowed: boolean) => Promise<void>;
    setColorScheme: (preference: "light" | "dark") => Promise<void>;
    setMediaType: (media: "print" | "screen") => Promise<void>;
    printDocument: () => Promise<void>;
    setReducedMotion: (preference: "reduce" | "no-preference") => Promise<void>;
  }
}
