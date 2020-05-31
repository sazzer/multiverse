import { Problem, request } from "../../api";

import debug from "debug";

/** The logger to use */
const LOGGER = debug("multiverse:landing:authentication:api");

/** Error to indicate that the user failed to authenticate */
export class AuthenticationError extends Error {}

/**
 * Attempt to log in to the system and retrieve an access token
 * // TODO: Not specified behaviour on success yet
 * @param username The username to log in as
 * @param password The password to log in with
 */
export async function login(username: string, password: string) {
  try {
    const response = await request("/login", {
      method: "POST",
      body: {
        username,
        password,
      },
    });
    LOGGER("Authenticated successfully: %o", response);
  } catch (e) {
    if (
      e instanceof Problem &&
      e.type === "tag:multiverse,2020:users/problems/authentication_error"
    ) {
      LOGGER("Failed to authenticate: %o", e);
      throw new AuthenticationError();
    } else {
      LOGGER("Something went wrong: %o", e);
      throw e;
    }
  }
}
