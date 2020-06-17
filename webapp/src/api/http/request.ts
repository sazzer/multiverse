import { Links, parseLinks } from "./links";

import { Problem } from "./problem";
import { UnauthorizedError } from "../errors";
import UrlTemplate from "url-template";
import debug from "debug";
import env from "@beam-australia/react-env";
import { getToken } from "./token";

/** The logger to use */
const LOGGER = debug("multiverse:api:http:request");

/**
 * The details needed in order to make an HTTP Request
 */
export interface Request {
  /** The HTTP Method to use. Defaults to GET if not provided */
  method?: "GET" | "POST" | "PUT" | "DELETE" | "PATCH" | "HEAD";
  /** Any parameters to use for URL expansion */
  urlParams?: { [key: string]: any };
  /** Any body to submit */
  body?: any;
  /** Whether to ignore the cache when making the request */
  ignoreCache?: boolean;
}

/**
 * The details of an HTTP Response
 */
export interface Response<B> {
  /** The response status code */
  status: number;
  /** The response headers */
  headers: Headers;
  /** The parsed links from the response */
  links: Links;
  /** The parsed body of the response */
  body?: B;
}

/**
 * Actually make an HTTP request and get the response
 * @param url The URL to call. This is a URL-Template as defined in RFC-6570.
 * @param request Any additional parameters for the request
 */
export async function request<B>(
  url: string,
  request: Request = {}
): Promise<Response<B>> {
  const template = UrlTemplate.parse(url);
  const finalUrl = template.expand(request.urlParams);
  LOGGER("Making request to %s: %o", finalUrl, request);

  const token = getToken();
  const headers = new Headers();
  if (token !== undefined) {
    headers.set("authorization", `Bearer ${token}`);
  }
  if (request.ignoreCache) {
    headers.set("cache-control", "no-cache");
  }

  try {
    const response = await fetch(env("URL_BASE") + finalUrl, {
      method: request.method || "GET",
      body: JSON.stringify(request.body),
      headers,
    });
    LOGGER("Received response from %s: %o", finalUrl, response);

    const contentType = response.headers.get("content-type");
    const linksHeader = response.headers.get("link");
    const links = parseLinks(linksHeader || "");

    if (contentType) {
      const body = await response.json();
      LOGGER("Response had payload: %o", body);

      if (contentType === "application/problem+json") {
        LOGGER("Response was a Problem");
        const type = body.type as string;
        if (type === "tag:multiverse,2020:problems/unauthorized") {
          throw new UnauthorizedError();
        }
        throw new Problem(type, body.title as string, response.status, body);
      } else {
        LOGGER("Response was not a Problem");
        return {
          status: response.status,
          headers: response.headers,
          links,
          body,
        };
      }
    } else {
      LOGGER("Response had no payload");
      return { status: response.status, headers: response.headers, links };
    }
  } catch (e) {
    LOGGER("Unexpected error making HTTP request: %o", e);
    throw e;
  }
}
