import * as api from "./load";

import { UnknownUserError } from "./errors";
import nock from "nock";

describe("Successfully load user", () => {
  test("Without Avatar URL", async () => {
    nock("https://multiverse-cd.herokuapp.com")
      .defaultReplyHeaders({ "access-control-allow-origin": "*" })
      .get("/users/someUserId")
      .reply(200, {
        username: "testuser",
        display_name: "Test User",
        email_address: "testuser@example.com",
      });

    const result = await api.loadUser("someUserId");

    expect(result).toEqual({
      username: "testuser",
      displayName: "Test User",
      emailAddress: "testuser@example.com",
      userId: "someUserId",
    });
  });

  test("With Avatar URL", async () => {
    nock("https://multiverse-cd.herokuapp.com")
      .defaultReplyHeaders({ "access-control-allow-origin": "*" })
      .get("/users/someUserId")
      .reply(200, {
        username: "testuser",
        display_name: "Test User",
        email_address: "testuser@example.com",
        avatar_url: "http://example.com/avatar",
      });

    const result = await api.loadUser("someUserId");

    expect(result).toEqual({
      username: "testuser",
      displayName: "Test User",
      emailAddress: "testuser@example.com",
      userId: "someUserId",
      avatarUrl: "http://example.com/avatar",
    });
  });
});

test("Load unknown user", async () => {
  nock("https://multiverse-cd.herokuapp.com")
    .defaultReplyHeaders({ "access-control-allow-origin": "*" })
    .get("/users/someUserId")
    .reply(
      404,
      {
        type: "tag:multiverse,2020:users/problems/unknown_user_id",
        title: "The requested user ID was unknown",
        status: 404,
      },
      {
        "content-type": "application/problem+json",
      }
    );

  try {
    await api.loadUser("someUserId");
    fail("Expected an UnknownUserError");
  } catch (e) {
    expect(e).toEqual(new UnknownUserError());
  }
});
