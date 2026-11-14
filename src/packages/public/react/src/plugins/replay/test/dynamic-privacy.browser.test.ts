import { EventType } from "@rrweb/types";
import { startReplayHarness } from "../../../../test/fixtures/replay.test-utils.js";
import { replayUploads } from "../../../../test/fixtures/replay-uploads.js";
import { describe, expect, onTestFinished, test, vi } from "vitest";

const { uploads } = replayUploads;

interface SnapshotNode {
  attributes?: {
    class?: string;
    id?: string;
    rr_height?: string;
    rr_width?: string;
    value?: string;
  };
  childNodes?: SnapshotNode[];
  textContent?: string;
  tagName?: string;
  type: number;
}

function mount(html: string) {
  const host = document.createElement("div");

  host.innerHTML = html;

  document.body.append(host);

  onTestFinished(() => {
    host.remove();
  });

  return host;
}

function query(host: HTMLElement, selector: string) {
  const element = host.querySelector<HTMLInputElement>(selector);

  if (!element) {
    throw new Error(`${selector} missing from the page`);
  }

  return element;
}

function walk(node: SnapshotNode, visit: (node: SnapshotNode) => void) {
  visit(node);

  for (const child of node.childNodes ?? []) {
    walk(child, visit);
  }
}

async function firstFullSnapshot() {
  await vi.advanceTimersByTimeAsync(2000);

  await replayUploads.uploadsReached(1);

  const event = uploads[0]?.events.find(
    (candidate): candidate is { data: { node: SnapshotNode } } =>
      (candidate as { type?: number }).type === EventType.FullSnapshot,
  );

  if (!event) {
    throw new Error("no full snapshot uploaded");
  }

  return event.data.node;
}

function findBy(root: SnapshotNode, key: "class" | "id", value: string) {
  let found: SnapshotNode | undefined;

  walk(root, (node) => {
    if (node.attributes?.[key] === value) {
      found = node;
    }
  });

  if (!found) {
    throw new Error(`${key}=${value} missing from the snapshot`);
  }

  return found;
}

function textUnder(node: SnapshotNode) {
  let text = "";

  walk(node, (child) => {
    text += child.textContent ?? "";
  });

  return text;
}

async function startWithMarkup(html: string) {
  mount(html);

  await startReplayHarness();

  return firstFullSnapshot();
}

describe("C-M1 password input values never leave the browser", () => {
  test("records a password value as asterisks", async () => {
    const root = await startWithMarkup(
      '<input id="pw" type="password" value="hunter2-secret">',
    );

    expect(findBy(root, "id", "pw").attributes?.value).toBe("**************");

    expect(uploads[0]?.text).not.toContain("hunter2-secret");
  });
});

describe("C-M2 email and tel inputs preserve current masking defaults", () => {
  test("records email and tel values without an explicit masking option", async () => {
    await startWithMarkup(
      '<input id="em" type="email" value="ada@example.com"><input id="tel" type="tel" value="+15551234567">',
    );

    expect(uploads[0]?.text).toContain("ada@example.com");

    expect(uploads[0]?.text).toContain("+15551234567");
  });
});

describe("C-M3 interfere-mask hides text but keeps the layout", () => {
  test("replaces masked text with asterisks of the same length", async () => {
    const root = await startWithMarkup(
      '<p id="acct" class="interfere-mask">IBAN GB33BUKB20201555555555</p><p id="pub">Public copy</p>',
    );

    expect(textUnder(findBy(root, "id", "acct"))).toMatch(/^[* ]+$/u);

    expect(textUnder(findBy(root, "id", "pub"))).toBe("Public copy");

    expect(uploads[0]?.text).not.toContain("GB33BUKB20201555555555");
  });
});

describe("C-M4 interfere-block subtrees are never serialised", () => {
  test("records a blocked element as an empty placeholder with its size", async () => {
    const root = await startWithMarkup(
      '<div id="card" class="interfere-block" style="width:120px;height:40px"><span>4242 4242 4242 4242</span><img src="/secret.png"></div>',
    );

    const card = findBy(root, "class", "interfere-block");

    expect(card.childNodes ?? []).toStrictEqual([]);

    expect(card.attributes).toMatchObject({
      rr_width: "120px",
      rr_height: "40px",
    });

    expect(uploads[0]?.text).not.toContain("4242 4242");

    expect(uploads[0]?.text).not.toContain("secret.png");
  });

  test("never records mutations inside a blocked subtree", async () => {
    const host = mount(
      '<div id="card2" class="interfere-block"><span id="inner">start</span></div><p id="pubm">before</p>',
    );

    await startReplayHarness();

    await vi.advanceTimersByTimeAsync(2000);

    await replayUploads.uploadsReached(1);

    query(host, "#inner").textContent = "late-secret-4111";

    query(host, "#pubm").textContent = "public-after";

    await vi.advanceTimersByTimeAsync(10_000);

    await replayUploads.uploadsReached(2);

    expect(uploads[1]?.text).toContain("public-after");

    expect(uploads.map(({ text }) => text).join("")).not.toContain(
      "late-secret-4111",
    );
  });
});

describe("C-M5 interfere-ignore inputs record no input events", () => {
  test("does not record typing into an ignored input", async () => {
    const host = mount(
      '<input id="otp" class="interfere-ignore" value=""><input id="plain" value="">',
    );

    await startReplayHarness();

    await vi.advanceTimersByTimeAsync(2000);

    await replayUploads.uploadsReached(1);

    const input = query(host, "#otp");

    input.value = "998877";

    input.dispatchEvent(new Event("input", { bubbles: true }));

    await vi.advanceTimersByTimeAsync(10_000);

    const plain = query(host, "#plain");

    plain.value = "visible-typed";

    plain.dispatchEvent(new Event("input", { bubbles: true }));

    await vi.advanceTimersByTimeAsync(10_000);

    await replayUploads.uploadsReached(2);

    expect(uploads.map(({ text }) => text).join("")).toContain("visible-typed");

    expect(uploads.map(({ text }) => text).join("")).not.toContain("998877");
  });
});
