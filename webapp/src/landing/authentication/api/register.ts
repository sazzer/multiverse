import { Problem, request } from "../../../api";

import debug from "debug";

/** The logger to use */
const LOGGER = debug("multiverse:landing:authentication:api");

/** Error to indicate that the username was already registered */
export class DuplicateUsernameError extends Error {}

/**
 * Attempt to register a new user
 * // TODO: Not specified behaviour on success yet
 * @param username The username to register with
 * @param password The password to register with
 * @param email The email address to register with
 * @param displayName The display name to register with
 */
export async function registerUser(
  username: string,
  password: string,
  email: string,
  displayName?: string
) {
  try {
    const response = await request("/register", {
      method: "POST",
      body: {
        username,
        email_address: email,
        display_name: displayName,
        password,
      },
    });
    LOGGER("Registered successfully: %o", response);
  } catch (e) {
    if (
      e instanceof Problem &&
      e.type === "tag:multiverse,2020:users/problems/duplicate_username"
    ) {
      LOGGER("Failed to authenticate: %o", e);
      throw new DuplicateUsernameError();
    } else {
      LOGGER("Something went wrong: %o", e);
      throw e;
    }
  }
}
