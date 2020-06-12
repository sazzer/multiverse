import { Problem, request } from "../http";
import { TokenResponse, recordToken } from "./token";

import debug from "debug";

/** The logger to use */
const LOGGER = debug("multiverse:landing:authentication:api");

/** Error to indicate that the username was already registered */
export class DuplicateUsernameError extends Error {}

/**
 * Attempt to register a new user
 * @param username The username to register with
 * @param password The password to register with
 * @param email The email address to register with
 * @param displayName The display name to register with
 * @return The ID of the user that registered
 */
export async function registerUser(
  username: string,
  password: string,
  email: string,
  displayName?: string
) {
  try {
    const response = await request<TokenResponse>("/register", {
      method: "POST",
      body: {
        username,
        email_address: email,
        display_name: displayName,
        password,
      },
    });
    LOGGER("Registered successfully: %o", response);
    if (response.body) {
      recordToken(response.body);
      return response.body.user_id;
    }
    return null;
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
