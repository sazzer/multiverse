import * as api from "./api";

import nock from "nock";

describe("lookupUsername", () => {
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
});

describe("login", () => {
  test("Successful login", async () => {
    nock("https://multiverse-cd.herokuapp.com")
      .defaultReplyHeaders({ "access-control-allow-origin": "*" })
      .post("/login", { username: "username", password: "password" })
      .reply(204);

    const result = await api.login("username", "password");
    expect(result).toBeUndefined(); // TODO: Not specified behaviour yet
  });

  test("Unsuccessful login", async () => {
    nock("https://multiverse-cd.herokuapp.com")
      .defaultReplyHeaders({ "access-control-allow-origin": "*" })
      .post("/login", { username: "username", password: "password" })
      .reply(
        401,
        {
          type: "tag:multiverse,2020:users/problems/authentication_error",
          title: "Invalid username or password",
          status: 401,
        },
        {
          "content-type": "application/problem+json",
        }
      );

    try {
      await api.login("username", "password");
      fail();
    } catch (e) {
      expect(e).toBeInstanceOf(api.AuthenticationError);
    }
  });
});
