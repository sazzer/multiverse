import * as api from "./create";

import { DuplicateUrlSlugError } from "./errors";
import { UnauthorizedError } from "../errors";
import nock from "nock";

beforeEach(() => {
  nock("https://multiverse-cd.herokuapp.com")
    .defaultReplyHeaders({
      "access-control-allow-origin": "*",
      "Access-Control-Expose-Headers": "Link, Content-Type",
    })
    .options("/worlds")
    .reply(200);
});

describe("Create World", () => {
  test("Successfully", async () => {
    nock("https://multiverse-cd.herokuapp.com")
      .defaultReplyHeaders({
        "access-control-allow-origin": "*",
        "Access-Control-Expose-Headers": "Link, Content-Type",
      })
      .post("/worlds", {
        name: "Test World",
        description: "The test world",
        url_slug: "test-world",
      })
      .reply(200, {
        name: "Test World",
        description: "The test world",
        url_slug: "test-world",
      });

    await api.createWorld({
      name: "Test World",
      description: "The test world",
      slug: "test-world",
    });
  });

  test("Successfully", async () => {
    nock("https://multiverse-cd.herokuapp.com")
      .defaultReplyHeaders({
        "access-control-allow-origin": "*",
        "Access-Control-Expose-Headers": "Link, Content-Type",
      })
      .post("/worlds", {
        name: "Test World",
        description: "The test world",
        url_slug: "test-world",
      })
      .reply(
        422,
        {
          type: "tag:multiverse,2020:worlds/problems/duplicate_url_slug",
          title: "The URL Slug was already present for this user",
          status: 422,
        },
        {
          "content-type": "application/problem+json",
        }
      );

    try {
      await api.createWorld({
        name: "Test World",
        description: "The test world",
        slug: "test-world",
      });
    } catch (e) {
      expect(e).toBeInstanceOf(DuplicateUrlSlugError);
    }
  });
});
