import type { IdentifyParams } from "@interfere/types/sdk/identify";
import type {
  ConsentDecision,
  ConsentState,
} from "@interfere/types/sdk/plugins/manifest";

import { isBotUserAgent } from "../util/bot.js";
import { createLogger } from "../util/log.js";
import { hasPublicKey, isForceEnableActive } from "./config/config.js";
import { registerKernel, unregisterKernel } from "./kernel-registry.js";
import { createKernel, isEnabledByEnvironment } from "./kernel.js";
import { pinConsolePatch } from "./console-patch/console-patch.js";
import { primeReactDomKeys } from "./react-dom-keys.js";
import type { Kernel, KernelInternalOptions, KernelOptions } from "./kernel.js";

const log = createLogger("init");

export interface WrapperSingleton {
  close: () => Promise<void>;
  consent: {
    get: () => ConsentState | null;
    set: (decision: ConsentDecision) => void;
  };
  getKernel: () => Kernel;
  getKernelOrNull: () => Kernel | null;
  identity: {
    clear: () => void;
    get: () => IdentifyParams | null;
    set: (params: IdentifyParams) => Promise<void>;
  };
  init: (opts?: KernelOptions) => Promise<Kernel | null>;
  subscribeToKernel: (listener: () => void) => () => void;
}

export interface WrapperSingletonInput {
  /**
   * Surfaced in the "kernel not initialized" error so the message points
   * customers at the right entry file (e.g. "instrumentation-client" /
   * "main.ts"). Single concrete fact — no other behavior change.
   */
  initEntryName: string;
  /**
   * Wrapper SDK identifier injected as the first entry in
   * `__INTERFERE_SDK_STACK__` (e.g. `@interfere/next@10.0.0`). Threaded
   * through to the kernel so the OTel resource carries the full stack.
   */
  producerVersion: string;
  /**
   * Installs the console wrappers when the wrapper module loads, before
   * framework code saves its own reference to `console.error`.
   */
  pinConsole?: boolean;
}

interface StartingKernel {
  status: "starting";
  kernel: Kernel | null;
  ready: Promise<Kernel | null>;
}

type Lifecycle =
  | { status: "idle" }
  | StartingKernel
  | { status: "ready"; kernel: Kernel }
  | { status: "closing"; closed: Promise<void> };

/** Owns one wrapper's kernel; initialization and shutdown finish before another kernel can start. */
export function createWrapperSingleton(input: WrapperSingletonInput) {
  let lifecycle: Lifecycle = { status: "idle" };
  let queuedConsent: { value: ConsentDecision } | null = null;
  let queuedIdentity: IdentifyParams | null = null;

  const listeners = new Set<() => void>();

  if (input.pinConsole && typeof window !== "undefined") {
    pinConsolePatch();
  }

  function currentKernel() {
    return lifecycle.status === "ready" ? lifecycle.kernel : null;
  }

  function emit() {
    for (const listener of listeners) {
      listener();
    }
  }

  async function initKernel(
    starting: StartingKernel,
    internalOpts: KernelInternalOptions,
  ) {
    try {
      const k = await createKernel({
        opts: internalOpts,
        onCreated: (created) => {
          starting.kernel = created;

          if (queuedConsent) {
            created.consent.set(queuedConsent.value);
          }
        },
      });

      if (lifecycle !== starting) {
        await k.dispose();

        return null;
      }

      lifecycle = { status: "ready", kernel: k };

      registerKernel(k);

      queuedConsent = null;

      if (queuedIdentity) {
        void k.identity.set(queuedIdentity);

        queuedIdentity = null;
      }

      emit();

      return k;
    } finally {
      if (lifecycle === starting) {
        lifecycle = { status: "idle" };
      }
    }
  }

  async function finishClose(
    previous: StartingKernel | { status: "ready"; kernel: Kernel },
  ) {
    try {
      if (previous.status === "starting") {
        await previous.ready;
      } else {
        await previous.kernel.dispose();
      }
    } finally {
      lifecycle = { status: "idle" };
    }
  }

  const returnValue: WrapperSingleton = {
    async close() {
      if (lifecycle.status === "closing") {
        return lifecycle.closed;
      }

      queuedConsent = null;

      queuedIdentity = null;

      if (lifecycle.status === "idle") {
        return;
      }

      const previous = lifecycle;
      const closed = finishClose(previous);

      lifecycle = { status: "closing", closed };

      if (previous.status === "ready") {
        unregisterKernel(previous.kernel);

        emit();
      }

      return closed;
    },

    consent: {
      get: () => currentKernel()?.consent.get() ?? null,
      set: (value) => {
        const kernel = currentKernel();

        if (kernel) {
          kernel.consent.set(value);
        } else {
          queuedConsent = { value };

          if (lifecycle.status === "starting") {
            lifecycle.kernel?.consent.set(value);
          }
        }
      },
    },

    getKernel() {
      const kernel = currentKernel();

      if (!kernel) {
        throw new Error(
          `Interfere SDK not initialized. Call init() from your ${input.initEntryName} entrypoint.`,
        );
      }

      return kernel;
    },

    getKernelOrNull: currentKernel,

    identity: {
      clear: () => {
        queuedIdentity = null;

        currentKernel()?.identity.clear();
      },
      get: () => currentKernel()?.identity.get() ?? queuedIdentity,
      set: async (params) => {
        const kernel = currentKernel();

        if (kernel) {
          return kernel.identity.set(params);
        }

        queuedIdentity = params;
      },
    },

    async init(opts: KernelOptions = {}) {
      if (lifecycle.status === "closing") {
        await lifecycle.closed;

        return returnValue.init(opts);
      }

      if (lifecycle.status === "ready") {
        return lifecycle.kernel;
      }

      if (lifecycle.status === "starting") {
        return lifecycle.ready;
      }

      if (!(opts.enabled ?? isEnabledByEnvironment())) {
        return null;
      }

      if (!isForceEnableActive() && isBotUserAgent()) {
        return null;
      }

      if (!hasPublicKey()) {
        log.warn(
          "No public key is configured, so the SDK is disabled. Set INTERFERE_PUBLIC_KEY (VITE_INTERFERE_PUBLIC_KEY for Vite) on builds that should send telemetry.",
        );

        return null;
      }

      const internalOpts: KernelInternalOptions = {
        ...opts,
        _wrapperVersions: [input.producerVersion],
      };

      primeReactDomKeys();

      let resolveReady!: (kernel: Kernel | null) => void;

      let rejectReady!: Parameters<
        ConstructorParameters<PromiseConstructor>[0]
      >[1];

      const ready = new Promise<Kernel | null>((resolve, reject) => {
        resolveReady = resolve;

        rejectReady = reject;
      });

      const starting: StartingKernel = {
        status: "starting",
        kernel: null,
        ready,
      };

      lifecycle = starting;

      void (async () => {
        try {
          resolveReady(await initKernel(starting, internalOpts));
        } catch (error) {
          rejectReady(error);
        }
      })();

      return ready;
    },

    subscribeToKernel(listener: () => void) {
      listeners.add(listener);

      return (() => {
        listeners.delete(listener);
      }) satisfies () => void;
    },
  };

  return returnValue;
}
