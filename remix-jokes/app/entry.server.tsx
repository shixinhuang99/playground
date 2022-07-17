import { type HandleDocumentRequestFunction } from "@remix-run/node";
import { RemixServer } from "@remix-run/react";
import { renderToString } from "react-dom/server";

const handleRequest: HandleDocumentRequestFunction = (
  request,
  status,
  headers,
  context
) => {
  const markup = renderToString(
    <RemixServer context={context} url={request.url} />
  );

  headers.set("content-type", "text/html");

  return new Response(`<!DOCTYPE html>\n${markup}`, {
    status,
    headers,
  });
};

export default handleRequest;
