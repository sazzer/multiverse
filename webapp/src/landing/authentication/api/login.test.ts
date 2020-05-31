import * as api from "./login";

import nock from "nock";

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
