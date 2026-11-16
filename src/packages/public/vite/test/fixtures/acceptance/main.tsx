/* oxlint-disable react-perf/jsx-no-new-function-as-prop, react-perf/jsx-no-jsx-as-prop, typescript/no-misused-promises, typescript/require-await */
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
import { ErrorBoundary } from "@interfere/react/error-boundary";

const denied = new URL(location.href).searchParams.has("denied");

const bulk = new URL(location.href).searchParams.has("bulk");

const BULK_ROWS = 4000;

function Bulk() {
  return (
    <ul aria-label="Bulk">
      {Array.from({ length: BULK_ROWS }, (_, index) => (
        <li key={index} data-row={index}>
          bulk row {index}: the quick brown fox jumps over the lazy dog
        </li>
      ))}
    </ul>
  );
}

if (new URL(location.href).searchParams.has("queue")) {
  await navigator.serviceWorker.register("/sw.js", { type: "module" });

  await navigator.serviceWorker.ready;

  if (!navigator.serviceWorker.controller) {
    await new Promise<void>((resolve) => {
      navigator.serviceWorker.addEventListener(
        "controllerchange",
        () => resolve(),
        { once: true },
      );
    });
  }
}

if (!denied) {
  consent.set(true);
}

await init(denied ? { consent: "required" } : {});

function Crash(): never {
  throw new Error("acceptance-render");
}

function Customer() {
  const sdk = useInterfere();
  const [message, setMessage] = useState("ready");
  const [crashed, setCrashed] = useState(false);
  const [count, setCount] = useState(0);
  const [session, setSession] = useState(sdk.session.getId());

  return (
    <main>
      <h1>Customer application</h1>
      <output aria-label="Result">{message}</output>
      <output aria-label="Session">{session}</output>
      <p className="interfere-mask">masked-secret-5329</p>
      <div className="interfere-block">blocked-secret-7451</div>
      <label>
        Password
        <input type="password" defaultValue="password-secret-9371" />
      </label>
      <label>
        Email
        <input type="email" defaultValue="email-secret-6821" />
      </label>
      <button type="button" onClick={() => setCount(count + 1)}>
        Count {count}
      </button>
      <button
        type="button"
        onClick={() => {
          capture(new Error("acceptance-manual"));

          setMessage("captured");
        }}
      >
        Capture
      </button>
      <button
        type="button"
        onClick={() => {
          setTimeout(() => {
            throw new Error("acceptance-unhandled");
          }, 0);
        }}
      >
        Unhandled error
      </button>
      <button
        type="button"
        onClick={() => {
          void Promise.reject(new Error("acceptance-rejection"));
        }}
      >
        Reject
      </button>
      <button type="button" onClick={() => setCrashed(true)}>
        Render error
      </button>
      <button
        type="button"
        onClick={async () => {
          const value = await span(
            "acceptance-result",
            async () => "original-result",
          );

          setMessage(value);
        }}
      >
        Span result
      </button>
      <button
        type="button"
        onClick={async () => {
          const capturedError = new Error("acceptance-span-error");

          try {
            await span("acceptance-throw", async () => {
              throw capturedError;
            });
          } catch (error) {
            setMessage(
              error === capturedError ? "same-error" : "changed-error",
            );
          }
        }}
      >
        Span failure
      </button>
      <button
        type="button"
        onClick={async () => {
          await identity.set({
            identifier: "consumer-a",
            source: { type: "custom", name: "Acceptance" },
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
            identifier: "consumer-b",
            source: { type: "custom", name: "Acceptance" },
          });

          setMessage("identified-b");
        }}
      >
        Identify B
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
          await init();

          await init();

          setMessage("initialized");
        }}
      >
        Initialize again
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
      <ErrorBoundary fallback={<p>Customer fallback</p>}>
        {crashed ? <Crash /> : <p>Healthy</p>}
      </ErrorBoundary>
      {bulk ? <Bulk /> : null}
    </main>
  );
}

const root = document.querySelector("#root");

if (!root) {
  throw new Error("Missing application root");
}

createRoot(root).render(
  <StrictMode>
    <InterfereProvider plugins={{ replay: { maskAllInputs: true } }}>
      <Customer />
    </InterfereProvider>
  </StrictMode>,
);
