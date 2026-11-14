import type { ReactNode } from "react";
import { expectTypeOf, test } from "vitest";

import type { ErrorBoundary, ErrorBoundaryProps } from "./error-boundary.js";
import type {
  ReactErrorHandlerCallbacks,
  reactErrorHandler,
} from "./react-error-handler.js";

test("boundary callbacks accept arbitrary thrown values", () => {
  type Fallback = Exclude<ErrorBoundaryProps["fallback"], ReactNode>;

  expectTypeOf<Fallback>().parameter(0).toBeUnknown();

  expectTypeOf<NonNullable<ErrorBoundaryProps["onError"]>>()
    .parameter(0)
    .toBeUnknown();

  expectTypeOf<typeof ErrorBoundary.getDerivedStateFromError>()
    .parameter(0)
    .toBeUnknown();

  expectTypeOf<ErrorBoundary["componentDidCatch"]>().parameter(0).toBeUnknown();
});

test("root callbacks accept arbitrary thrown values", () => {
  type Callback = NonNullable<
    ReactErrorHandlerCallbacks[keyof ReactErrorHandlerCallbacks]
  >;

  type Handler = ReturnType<
    typeof reactErrorHandler
  >[keyof ReactErrorHandlerCallbacks];

  expectTypeOf<Callback>().parameter(0).toBeUnknown();

  expectTypeOf<Handler>().parameter(0).toBeUnknown();
});
