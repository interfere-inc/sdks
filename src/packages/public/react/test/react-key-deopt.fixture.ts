import { createElement, useEffect, useState } from "react";
import { createRoot } from "react-dom/client";

declare const FIXTURE_MODE: "allocate" | "sdk";

const WORDS =
  "streaming tokens arrive at thirty hertz while the page keeps rendering".split(
    " ",
  );

function Stream() {
  const [words, setWords] = useState<string[]>([]);

  useEffect(() => {
    const id = setInterval(() => {
      setWords((previous) => [
        ...previous.slice(-200),
        WORDS[previous.length % WORDS.length] ?? "",
      ]);
    }, 33);

    return () => {
      clearInterval(id);
    };
  }, []);

  return createElement(
    "p",
    null,
    words.map((word, index) =>
      createElement("span", { key: index }, word, " "),
    ),
  );
}

function allocate() {
  const retained: unknown[] = [];

  for (let index = 0; index < 400_000; index += 1) {
    retained.push({ index, label: `x${index}`, list: [index] });
  }

  return retained.length;
}

const container = document.querySelector("#root");

if (!container) {
  throw new Error("missing #root");
}

const root = createRoot(container);

if (FIXTURE_MODE === "allocate") {
  document.body.dataset["allocated"] = String(allocate());
} else {
  const { createWrapperSingleton } =
    await import("../dist/internal/wrapper-singleton.mjs");

  const wrapper = createWrapperSingleton({
    initEntryName: "fixture",
    producerVersion: "@interfere/fixture@0.0.0",
  });

  const kernel = await wrapper.init({ enabled: true, serviceName: "fixture" });

  document.body.dataset["kernel"] = String(kernel !== null);
}

root.render(createElement(Stream));

document.body.dataset["ready"] = "true";
