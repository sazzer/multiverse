import * as api from "./login";

import nock from "nock";
import { storeToken } from "../../api/token";

jest.mock("../../api/token");

beforeEach(() => {
  (storeToken as jest.Mock).mockClear();
});

test("Successful login", async () => {
  nock("https://multiverse-cd.herokuapp.com")
    .defaultReplyHeaders({ "access-control-allow-origin": "*" })
    .post("/login", { username: "username", password: "password" })
    .reply(200, {
      token: "authToken",
      valid_until: "2020-09-08T10:09:55.139275303Z",
      user_id: "a9846a08-d66c-4ec6-956d-be32e92a6fd6",
      display_name: "Test User",
    });

  const result = await api.login("username", "password");

  expect(storeToken).toBeCalledTimes(1);
  expect(storeToken).toBeCalledWith(
    "authToken",
    new Date("2020-09-08T10:09:55.139275303Z")
  );

  expect(result).toEqual("a9846a08-d66c-4ec6-956d-be32e92a6fd6");
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
  expect(storeToken).not.toBeCalled();
});
