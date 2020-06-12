import * as api from "./register";

import nock from "nock";
import { storeToken } from "../../api/http/token";

jest.mock("../../api/http/token");

beforeEach(() => {
  (storeToken as jest.Mock).mockClear();
});

test("Successful registration", async () => {
  nock("https://multiverse-cd.herokuapp.com")
    .defaultReplyHeaders({ "access-control-allow-origin": "*" })
    .post("/register", {
      username: "username",
      password: "password",
      email_address: "test@example.com",
      display_name: "Test User",
    })
    .reply(200, {
      token: "authToken",
      valid_until: "2020-09-08T10:09:55.139275303Z",
      user_id: "a9846a08-d66c-4ec6-956d-be32e92a6fd6",
      display_name: "Test User",
    });

  const result = await api.registerUser(
    "username",
    "password",
    "test@example.com",
    "Test User"
  );

  expect(storeToken).toBeCalledTimes(1);
  expect(storeToken).toBeCalledWith(
    "authToken",
    new Date("2020-09-08T10:09:55.139275303Z")
  );

  expect(result).toEqual("a9846a08-d66c-4ec6-956d-be32e92a6fd6");
});

test("Duplicate username", async () => {
  nock("https://multiverse-cd.herokuapp.com")
    .defaultReplyHeaders({ "access-control-allow-origin": "*" })
    .post("/register", {
      username: "username",
      password: "password",
      email_address: "test@example.com",
      display_name: "Test User",
    })
    .reply(
      422,
      {
        type: "tag:multiverse,2020:users/problems/duplicate_username",
        title: "Duplicate Username",
        status: 422,
      },
      {
        "content-type": "application/problem+json",
      }
    );

  try {
    await api.registerUser(
      "username",
      "password",
      "test@example.com",
      "Test User"
    );
    fail();
  } catch (e) {
    expect(e).toBeInstanceOf(api.DuplicateUsernameError);
  }
  expect(storeToken).not.toBeCalled();
});
