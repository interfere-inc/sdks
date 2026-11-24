function explode(request: Request) {
  throw new Error(`worker failure at ${new URL(request.url).pathname}`);
}

const handler = {
  fetch(request: Request) {
    try {
      explode(request);
    } catch (error) {
      return new Response(error instanceof Error ? error.stack : "", {
        status: 500,
      });
    }

    return new Response(null, { status: 204 });
  },
};

// oxlint-disable-next-line import/no-default-export -- Native Worker entrypoints require a default export.
export default handler;
