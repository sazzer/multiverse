import * as api from "./lookupUsername";

import nock from "nock";

test("Known Username", async () => {
  nock("https://multiverse-cd.herokuapp.com")
    .defaultReplyHeaders({ "access-control-allow-origin": "*" })
    .get("/usernames/known")
    .reply(204);

  const result = await api.lookupUsername("known");
  expect(result).toBe(true);
});

test("Unknown Username", async () => {
  nock("https://multiverse-cd.herokuapp.com")
    .defaultReplyHeaders({ "access-control-allow-origin": "*" })
    .get("/usernames/unknown")
    .reply(
      404,
      {
        type: "tag:multiverse,2020:users/problems/unknown_username",
        title: "The username was unknown",
        status: 404,
      },
      {
        "content-type": "application/problem+json",
      }
    );

  const result = await api.lookupUsername("unknown");
  expect(result).toBe(false);
});

test("Unexpected Error", async () => {
  nock("https://multiverse-cd.herokuapp.com")
    .defaultReplyHeaders({ "access-control-allow-origin": "*" })
    .get("/usernames/unknown")
    .replyWithError({ code: "ETIMEDOUT" });

  try {
    await api.lookupUsername("unknown");
    fail();
  } catch (e) {
    expect(e.toString()).toBe("TypeError: Network request failed");
  }
});
