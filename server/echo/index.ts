type BodyType = "text" | "base64";
type RequestConfig = {
  method: string;
  host: string;
  path: string;
  headers: Record<string, string>;
  queries: Record<string, string>;
  body: string;
  bodyType: BodyType;
};

const accessControlHeaders = {
  "access-control-allow-origin": "*",
  "access-control-allow-methods": "*",
  "access-control-allow-headers": "*",
  "access-control-expose-headers": "*",
  "access-control-max-age": "86400",
  "access-control-allow-credentials": "true",
};

export default {
  async fetch(request: Request): Promise<Response> {
    if (request.method === "OPTIONS") {
      return new Response(null, {
        headers: accessControlHeaders,
      });
    }

    const requestData = await getRequestBody(request);
    return new Response(JSON.stringify(requestData), {
      status: 200,
      headers: {
        "content-type": "application/json",
        ...accessControlHeaders,
      },
    });
  },
};

const getHeaders = (request: Request): Record<string, string> => {
  const headers: Record<string, string> = {};
  request.headers.forEach((value, key) => {
    headers[key] = value;
  });
  return headers;
};

const getQueries = (request: Request): Record<string, string> => {
  const queries: Record<string, string> = {};
  const url = new URL(request.url);
  url.searchParams.forEach((value, key) => {
    queries[key] = value;
  });
  return queries;
};

const getBody = async (request: Request): Promise<[string, BodyType]> => {
  const body = await request.arrayBuffer();
  const textDecoder = new TextDecoder("utf8", { fatal: true, ignoreBOM: true });
  try {
    const text = textDecoder.decode(body);
    return [text, "text"];
  } catch (e) {
    const base64 = btoa(String.fromCharCode(...new Uint8Array(body)));
    return [base64, "base64"];
  }
};

const getRequestBody = async (request: Request): Promise<RequestConfig> => {
  const url = new URL(request.url);

  const method = request.method;
  const host = url.host;
  const path = url.pathname;

  const headers = getHeaders(request);
  const queries = getQueries(request);
  const [body, bodyType] = await getBody(request);

  return {
    method,
    host,
    path,
    queries,
    body,
    bodyType,
    headers,
  };
};
