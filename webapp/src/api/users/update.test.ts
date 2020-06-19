import * as api from "./update";

import { UnauthorizedError } from "../errors";
import { UnknownUserError } from "./errors";
import nock from "nock";

beforeEach(() => {
  nock("https://multiverse-cd.herokuapp.com")
    .defaultReplyHeaders({
      "access-control-allow-origin": "*",
      "Access-Control-Expose-Headers": "Link, Content-Type",
    })
    .options("/users/someUserId")
    .reply(200);
});

describe("Update User Profile", () => {
  test("Successfully", async () => {
    nock("https://multiverse-cd.herokuapp.com")
      .defaultReplyHeaders({
        "access-control-allow-origin": "*",
        "Access-Control-Expose-Headers": "Link, Content-Type",
      })
      .patch("/users/someUserId", {
        display_name: "New User",
        email_address: "newuser@example.com",
      })
      .reply(200, {
        username: "testuser",
        display_name: "New User",
        email_address: "newuser@example.com",
      });

    await api.updateUser({
      selfLink: "/users/someUserId",
      username: "testuser",
      displayName: "New User",
      emailAddress: "newuser@example.com",
    });
  });

  test("Unknown User", async () => {
    nock("https://multiverse-cd.herokuapp.com")
      .defaultReplyHeaders({
        "access-control-allow-origin": "*",
        "Access-Control-Expose-Headers": "Link, Content-Type",
      })
      .patch("/users/someUserId", {
        display_name: "New User",
        email_address: "newuser@example.com",
      })
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
      await api.updateUser({
        selfLink: "/users/someUserId",
        username: "testuser",
        displayName: "New User",
        emailAddress: "newuser@example.com",
      });
      fail("Expected UnknownUserError");
    } catch (e) {
      expect(e).toBeInstanceOf(UnknownUserError);
    }
  });

  test("When not allowed", async () => {
    nock("https://multiverse-cd.herokuapp.com")
      .defaultReplyHeaders({
        "access-control-allow-origin": "*",
        "Access-Control-Expose-Headers": "Link, Content-Type",
      })
      .patch("/users/someUserId", {
        display_name: "New User",
        email_address: "newuser@example.com",
      })
      .reply(
        403,
        {
          type: "tag:multiverse,2020:problems/unauthorized",
          title: "An invalid access token was provided",
          status: 403,
        },
        {
          "content-type": "application/problem+json",
        }
      );

    try {
      await api.updateUser({
        selfLink: "/users/someUserId",
        username: "testuser",
        displayName: "New User",
        emailAddress: "newuser@example.com",
      });
      fail("Expected UnknownUserError");
    } catch (e) {
      expect(e).toBeInstanceOf(UnauthorizedError);
    }
  });
});

describe("Change Password", () => {
  test("Successfully", async () => {
    nock("https://multiverse-cd.herokuapp.com")
      .defaultReplyHeaders({
        "access-control-allow-origin": "*",
        "Access-Control-Expose-Headers": "Link, Content-Type",
      })
      .patch("/users/someUserId", {
        old_password: "oldPassword",
        password: "newPassword",
      })
      .reply(200, {
        username: "testuser",
        display_name: "Test User",
        email_address: "testuser@example.com",
      });

    await api.changePassword("/users/someUserId", "oldPassword", "newPassword");
  });

  test("Invalid Old Password", async () => {
    nock("https://multiverse-cd.herokuapp.com")
      .defaultReplyHeaders({
        "access-control-allow-origin": "*",
        "Access-Control-Expose-Headers": "Link, Content-Type",
      })
      .patch("/users/someUserId", {
        old_password: "oldPassword",
        password: "newPassword",
      })
      .reply(
        422,
        {
          type: "tag:multiverse,2020:users/problems/invalid_old_password",
          title: "Old Password incorrect when changing password",
          status: 422,
        },
        {
          "content-type": "application/problem+json",
        }
      );

    try {
      await api.changePassword(
        "/users/someUserId",
        "oldPassword",
        "newPassword"
      );
      fail("Expected InvalidOldPasswordError");
    } catch (e) {
      expect(e).toBeInstanceOf(api.InvalidOldPasswordError);
    }
  });

  test("Unknown User", async () => {
    nock("https://multiverse-cd.herokuapp.com")
      .defaultReplyHeaders({
        "access-control-allow-origin": "*",
        "Access-Control-Expose-Headers": "Link, Content-Type",
      })
      .patch("/users/someUserId", {
        old_password: "oldPassword",
        password: "newPassword",
      })
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
      await api.changePassword(
        "/users/someUserId",
        "oldPassword",
        "newPassword"
      );
      fail("Expected UnknownUserError");
    } catch (e) {
      expect(e).toBeInstanceOf(UnknownUserError);
    }
  });

  test("When not allowed", async () => {
    nock("https://multiverse-cd.herokuapp.com")
      .defaultReplyHeaders({
        "access-control-allow-origin": "*",
        "Access-Control-Expose-Headers": "Link, Content-Type",
      })
      .patch("/users/someUserId", {
        old_password: "oldPassword",
        password: "newPassword",
      })
      .reply(
        403,
        {
          type: "tag:multiverse,2020:problems/unauthorized",
          title: "An invalid access token was provided",
          status: 403,
        },
        {
          "content-type": "application/problem+json",
        }
      );

    try {
      await api.changePassword(
        "/users/someUserId",
        "oldPassword",
        "newPassword"
      );
      fail("Expected InvalidOldPasswordError");
    } catch (e) {
      expect(e).toBeInstanceOf(UnauthorizedError);
    }
  });
});
