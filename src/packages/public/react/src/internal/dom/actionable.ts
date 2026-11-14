import { fieldLabel, privacyLevel, publicText } from "./privacy.js";
import type { PrivacyLevel, PrivacyPolicy } from "./privacy.js";

const ACTIONABLE_TAGS = new Set([
  "a",
  "button",
  "input",
  "select",
  "textarea",
  "summary",
  "label",
]);

const ACTIONABLE_ROLES = new Set([
  "button",
  "link",
  "menuitem",
  "tab",
  "checkbox",
  "radio",
  "switch",
]);

export function isActionable(el: Element) {
  if (ACTIONABLE_TAGS.has(el.tagName.toLowerCase())) {
    return true;
  }

  const role = el.getAttribute("role");

  if (role && ACTIONABLE_ROLES.has(role)) {
    return true;
  }

  return (
    (el instanceof HTMLElement || el instanceof SVGElement) &&
    Object.hasOwn(el.dataset, "trackClick")
  );
}

export function closestActionable(target: EventTarget | null) {
  if (!(target instanceof Element)) {
    return null;
  }

  let el: Element | null = target;

  while (el) {
    if (isActionable(el)) {
      return el;
    }

    el = el.parentElement;
  }

  return null;
}

export interface ActionableDescriptor {
  ariaLabel?: string | undefined;
  id?: string | undefined;
  label?: string | undefined;
  name?: string | undefined;
  privacy?: Exclude<PrivacyLevel, "public"> | undefined;
  role?: string | undefined;
  tag: string;
  text?: string | undefined;
}

export function describeActionable(el: Element, policy: PrivacyPolicy) {
  const desc: ActionableDescriptor = {
    tag: el.tagName.toLowerCase(),
  };

  const label = fieldLabel(el, policy);

  if (label) {
    desc.label = label;
  }

  const level = privacyLevel(el, policy);

  if (level === "blocked") {
    desc.privacy = level;

    return desc;
  }

  const role = el.getAttribute("role");

  if (role) {
    desc.role = role;
  }

  if (level === "masked") {
    desc.privacy = level;

    return desc;
  }

  if (el.id) {
    desc.id = el.id;
  }

  const name = el.getAttribute("name");

  if (name) {
    desc.name = name;
  }

  const ariaLabel = el.getAttribute("aria-label");

  if (ariaLabel) {
    desc.ariaLabel = ariaLabel;
  }

  const text = publicText(el, policy);

  if (text) {
    desc.text = text;
  }

  return desc;
}
