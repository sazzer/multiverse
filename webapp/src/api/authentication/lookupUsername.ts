import { Problem, request } from "../http";

import debug from "debug";

/** The logger to use */
const LOGGER = debug("multiverse:landing:authentication:api");

/**
 * Actually look up the username to see if it exists
 * @param username The username to look up
 */
export async function lookupUsername(username: string): Promise<boolean> {
  try {
    const response = await request("/usernames/{username}", {
      urlParams: {
        username,
      },
    });
    LOGGER("Username exists: %o", response);
    return true;
  } catch (e) {
    if (
      e instanceof Problem &&
      e.type === "tag:multiverse,2020:users/problems/unknown_username"
    ) {
      LOGGER("Username doesn't exist");
      return false;
    } else {
      LOGGER("Something went wrong: %o", e);
      throw e;
    }
  }
}
