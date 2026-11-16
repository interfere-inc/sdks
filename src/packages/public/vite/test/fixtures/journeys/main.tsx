/* oxlint-disable react-perf/jsx-no-new-function-as-prop, typescript/no-misused-promises */
import { StrictMode, useState } from "react";
import { createRoot } from "react-dom/client";
import {
  capture,
  close,
  consent,
  identity,
  init,
  span,
} from "@interfere/vite/init";
import { InterfereProvider, useInterfere } from "@interfere/vite/provider";

const query = new URL(location.href).searchParams;

if (!query.has("denied")) {
  consent.set(true);
}

const initializing = init(query.has("denied") ? { consent: "required" } : {});

if (query.has("early")) {
  await identity.set({
    identifier: "early-user",
    source: { type: "custom", name: "Journeys" },
  });
}

await initializing;

function Customer() {
  const sdk = useInterfere();
  const [message, setMessage] = useState("ready");
  const [marker, setMarker] = useState("journey-error");
  const [session, setSession] = useState(sdk.session.getId());

  return (
    <main>
      <h1>Customer journey</h1>
      <output aria-label="Result">{message}</output>
      <output aria-label="Session">{session}</output>
      <label>
        Marker
        <input
          value={marker}
          onChange={(event) => setMarker(event.target.value)}
        />
      </label>
      <label>
        Visible text
        <input defaultValue="visible-text-7351" />
      </label>
      <label>
        Email
        <input type="email" defaultValue="visible-email-2841@example.test" />
      </label>
      <label>
        Telephone
        <input type="tel" defaultValue="555-010-6789" />
      </label>
      <label>
        Password
        <input type="password" defaultValue="secret-password-9251" />
      </label>
      <p className="interfere-mask">secret-mask-8462</p>
      <p className="interfere-block">secret-block-3971</p>
      <button
        type="button"
        onClick={() => {
          capture(new Error(marker));

          setMessage(`captured:${marker}`);
        }}
      >
        Capture
      </button>
      <button
        type="button"
        onClick={async () => {
          await identity.set({
            identifier: "user-a",
            source: { type: "custom", name: "Journeys" },
          });

          setMessage("identified-a");
        }}
      >
        Identify A
      </button>
      <button
        type="button"
        onClick={async () => {
          await identity.set({
            identifier: "user-a",
            source: { type: "custom", name: "Other source" },
          });

          setMessage("identified-other");
        }}
      >
        Identify other source
      </button>
      <button
        type="button"
        onClick={() => {
          identity.clear();

          setSession(sdk.session.getId());

          setMessage("anonymous");
        }}
      >
        Logout
      </button>
      <button
        type="button"
        onClick={() => {
          consent.set(true);

          setMessage("granted");
        }}
      >
        Grant consent
      </button>
      <button
        type="button"
        onClick={() => {
          consent.set(false);

          setMessage("revoked");
        }}
      >
        Revoke consent
      </button>
      <button
        type="button"
        onClick={() => {
          history.pushState({}, "", "/?navigation=1");

          setMessage("navigated");
        }}
      >
        Navigate
      </button>
      <button
        type="button"
        onClick={async () => {
          await Promise.all([init(), init(), init()]);

          setSession(sdk.session.getId());

          setMessage("initialized");
        }}
      >
        Initialize concurrently
      </button>
      <button
        type="button"
        onClick={async () => {
          await close();

          setMessage("closed");
        }}
      >
        Close SDK
      </button>
      <button
        type="button"
        onClick={async () => {
          await init();

          setMessage("restarted");
        }}
      >
        Restart SDK
      </button>
      <button
        type="button"
        onClick={async () => {
          const result = await span("journey-parent", async () =>
            span("journey-child", async () => {
              await new Promise<void>((resolve) => {
                queueMicrotask(resolve);
              });

              return "child-result";
            }),
          );

          setMessage(result);
        }}
      >
        Nested spans
      </button>
    </main>
  );
}

function Shell() {
  const [generation, setGeneration] = useState(0);

  return (
    <>
      <button type="button" onClick={() => setGeneration(generation + 1)}>
        Remount provider
      </button>
      <InterfereProvider key={generation}>
        <Customer />
      </InterfereProvider>
    </>
  );
}

const root = document.querySelector("#root");

if (!root) {
  throw new Error("Missing application root");
}

createRoot(root).render(
  <StrictMode>
    <Shell />
  </StrictMode>,
);
