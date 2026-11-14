/* oxlint-disable import/no-default-export */
import { omitUndefined } from "@interfere/helpers/omit-undefined";

import { describeActionable } from "../internal/dom/actionable.js";
import type { PrivacyPolicy } from "../internal/dom/privacy.js";
import type { Plugin } from "./lib/types.js";

const CLICK_THRESHOLD = 3;

const TIME_WINDOW_MS = 800;

const PROXIMITY_PX = 30;

const TRACER_NAME = "@interfere/react/rage-clicks";

interface Click {
  target: Element | null;
  ts: number;
  x: number;
  y: number;
}

function distance(a: Click, b: Click) {
  return Math.hypot(a.x - b.x, a.y - b.y);
}

function describeTarget(el: Element | null, policy: PrivacyPolicy) {
  if (!el) {
    return { selector: "unknown", text: "" };
  }

  const { id, label, privacy, tag, text = "" } = describeActionable(el, policy);
  const classes = privacy ? [] : [...el.classList].slice(0, 3);

  return {
    label,
    privacy,
    selector: id ? `#${id}` : [tag, ...classes].join("."),
    text,
  };
}

const rageClicksPlugin: Plugin = {
  name: "rage-clicks",

  setup(ctx) {
    const clicks: Click[] = [];

    const onClick = (event: MouseEvent) => {
      const now = Date.now();

      clicks.push({
        target: event.target instanceof Element ? event.target : null,
        ts: now,
        x: event.clientX,
        y: event.clientY,
      });

      while (clicks.length > 0 && now - (clicks[0]?.ts ?? 0) > TIME_WINDOW_MS) {
        clicks.shift();
      }

      if (clicks.length < CLICK_THRESHOLD) {
        return;
      }

      const [anchor] = clicks;

      if (!anchor) {
        return;
      }

      const clustered = clicks.filter(
        (c) => distance(anchor, c) <= PROXIMITY_PX,
      );

      if (clustered.length < CLICK_THRESHOLD) {
        return;
      }

      const last = clustered.at(-1);

      if (!last) {
        return;
      }

      const target = describeTarget(last.target, ctx.privacyPolicy);

      const span = ctx.getTracer(TRACER_NAME).startSpan("rage_click", {
        attributes: omitUndefined({
          "ui.event_type": "rage_click",
          "ui.rage_click.count": clustered.length,
          "ui.rage_click.selector": target.selector,
          "ui.rage_click.text": target.text,
          "ui.rage_click.time_window_ms": last.ts - anchor.ts,
          "ui.rage_click.x": last.x,
          "ui.rage_click.y": last.y,
          "ui.target.label": target.label,
          "ui.target.privacy": target.privacy,
        }),
      });

      span.end();

      clicks.length = 0;
    };

    document.addEventListener("click", onClick, { capture: true });

    return () => {
      document.removeEventListener("click", onClick, { capture: true });
    };
  },
};

export default rageClicksPlugin;
