const accessControlHeaders = {
  "access-control-allow-origin": "*",
  "access-control-allow-methods": "*",
  "access-control-allow-headers": "*",
  "access-control-expose-headers": "*",
  "access-control-max-age": "86400",
  "access-control-allow-credentials": "true",
  vary: "Origin",
};

export default {
  async fetch(request: Request) {
    const url = /^http(s)?:\/\//i.test(request.url)
      ? new URL(request.url)
      : new URL(`https://${request.url}`);

    if (request.method === "OPTIONS") {
      return new Response(null, {
        headers: accessControlHeaders,
      });
    } else {
      const response = await fetch(url, request);

      return new Response(response.body, {
        ...response,
        headers: {
          ...response.headers,
          ...accessControlHeaders,
        },
      });
    }
  },
};
