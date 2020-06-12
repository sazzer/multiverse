import * as token from "./token";

test("Getting no token", () => {
  expect(token.getToken()).toBeUndefined();
});

test("Getting an expired token", () => {
  const now = new Date();
  const expiry = new Date(now.getTime() - 1000);

  token.storeToken("myToken", expiry);
  expect(token.getToken()).toBeUndefined();
});

test("Getting an unexpired token", () => {
  const now = new Date();
  const expiry = new Date(now.getTime() + 1000);

  token.storeToken("myToken", expiry);
  expect(token.getToken()).toEqual("myToken");
});
