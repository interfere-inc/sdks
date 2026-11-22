import { HttpNetworkFrame, WebSocketNetworkFrame } from "msw/experimental";

export function checkUnhandledBrowserFrame(
  frame: { protocol: string },
  page: Pick<Location, "host" | "origin">,
  unexpectedRequests: string[],
) {
  if (frame instanceof WebSocketNetworkFrame) {
    const { url } = frame.data.connection.client;

    if (url.host !== page.host) {
      unexpectedRequests.push(`WS ${url.href}`);

      throw new Error(`Unexpected WebSocket: ${url.href}`);
    }

    return;
  }

  if (!(frame instanceof HttpNetworkFrame)) {
    throw new TypeError(`Unexpected network protocol: ${frame.protocol}`);
  }

  const { request } = frame.data;
  const url = new URL(request.url);

  if (
    (url.origin === page.origin &&
      (/^(?:\/@(?:fs|id|vite)\/|\/__vitest|\/node_modules\/|\/src\/|\/test\/|\/mockServiceWorker\.js$|\/favicon\.ico$)/u.test(
        url.pathname,
      ) ||
        (url.searchParams.has("browserv") &&
          /\.browser\.test\.[cm]?[jt]sx?$/u.test(url.pathname)))) ||
    /^https:\/\/clerk-[\da-f-]+\.clerk\.accounts\.dev\//u.test(request.url)
  ) {
    return;
  }

  unexpectedRequests.push(`${request.method} ${request.url}`);

  throw new Error(`Unexpected request: ${request.method} ${request.url}`);
}
