import type { BrowserMetadataOptions } from "./browser-platform.ts";
import type { BrowserCommandContext } from "vitest/node";
import { defineBrowserCommand } from "@vitest/browser-playwright";

export const setClipboardAccess = defineBrowserCommand(
  async ({ context, page }, allowed: boolean) => {
    if (allowed) {
      await context.clearPermissions();

      await context.grantPermissions(["clipboard-read", "clipboard-write"]);

      return;
    }

    const session = await context.newCDPSession(page);

    try {
      const { targetInfo } = await session.send("Target.getTargetInfo");

      if (!targetInfo.browserContextId) {
        throw new Error("Missing browser context");
      }

      await session.send("Browser.setPermission", {
        permission: { name: "clipboard-write", allowWithoutSanitization: true },
        setting: "denied",
        origin: new URL(page.url()).origin,
        browserContextId: targetInfo.browserContextId,
      });
    } finally {
      await session.detach();
    }
  },
);

type MetadataSession = Awaited<
  ReturnType<BrowserCommandContext["context"]["newCDPSession"]>
>;

interface UserAgentOptions {
  userAgent: string;
  acceptLanguage: string;
  userAgentMetadata?: NonNullable<BrowserMetadataOptions["clientHints"]>;
}

const metadataSessions = new WeakMap<object, MetadataSession>();

export const setBrowserMetadata = defineBrowserCommand(
  async ({ context, page }, metadata: BrowserMetadataOptions | "reset") => {
    const existing = metadataSessions.get(page);

    if (metadata === "reset") {
      if (existing) {
        metadataSessions.delete(page);

        await existing.detach();
      }

      return;
    }

    const session = existing ?? (await context.newCDPSession(page));

    metadataSessions.set(page, session);

    const userAgentOptions: UserAgentOptions = {
      userAgent: metadata.userAgent,
      acceptLanguage: metadata.language,
    };

    if (metadata.clientHints) {
      userAgentOptions.userAgentMetadata = metadata.clientHints;
    }

    await session.send("Emulation.setUserAgentOverride", userAgentOptions);

    await session.send("Emulation.setDeviceMetricsOverride", {
      width: metadata.screen.width,
      height: metadata.screen.height,
      screenWidth: metadata.screen.width,
      screenHeight: metadata.screen.height,
      mobile: false,
      deviceScaleFactor: 1,
      screenOrientation: { type: "landscapePrimary", angle: 90 },
    });
  },
);

export const closeBrowserPopups = defineBrowserCommand(
  async ({ context, page }) => {
    await Promise.all(
      context
        .pages()
        .filter((candidate) => candidate !== page)
        .map(async (popup) => popup.close()),
    );
  },
);

const authenticatorSessions = new WeakMap<object, MetadataSession>();

export const setVirtualAuthenticator = defineBrowserCommand(
  async (
    { context, page },
    enabled: boolean,
    automaticPresenceSimulation?: boolean,
  ) => {
    const existing = authenticatorSessions.get(page);

    if (existing) {
      await existing.send("WebAuthn.disable");

      await existing.detach();

      authenticatorSessions.delete(page);
    }

    if (!enabled) {
      return;
    }

    const session = await context.newCDPSession(page);

    await session.send("WebAuthn.enable", { enableUI: false });

    await session.send("WebAuthn.addVirtualAuthenticator", {
      options: {
        protocol: "ctap2",
        transport: "internal",
        hasResidentKey: true,
        hasUserVerification: true,
        isUserVerified: true,
        automaticPresenceSimulation: automaticPresenceSimulation ?? true,
      },
    });

    authenticatorSessions.set(page, session);
  },
);
