/* oxlint-disable vitest/prefer-importing-vitest-globals */
import { expect, test } from "@playwright/test";

test("@desktop native .mjs SDK config builds and serves a hydrated production app", async ({
  page,
}) => {
  const errors: Error[] = [];

  page.on("pageerror", (error) => errors.push(error));

  const response = await page.goto("/");

  expect(response?.status()).toBe(200);

  await page.getByRole("button", { name: "Native config: 0" }).click();

  await expect(
    page.getByRole("button", { name: "Native config: 1" }),
  ).toBeVisible();

  expect(errors).toStrictEqual([]);
});
