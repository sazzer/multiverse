import { Problem, request } from "../http";

import { UnknownUserError } from "./errors";
import { User } from "./model";
import { UserResponse } from "./response";

/**
 * Load a user from the server
 *
 * @param user The URL of the user to load
 * @param ignoreCache Whether to ignore the cache or not
 */
export async function loadUser(
  user: string,
  ignoreCache?: boolean
): Promise<User> {
  try {
    const response = await request<UserResponse>(user, {
      ignoreCache,
    });

    return {
      selfLink: response.links.getLinkByRel("self")!!.target,
      username: response.body!!.username,
      displayName: response.body!!.display_name,
      emailAddress: response.body!!.email_address,
      avatarUrl: response.body?.avatar_url,
    };
  } catch (e) {
    if (
      e instanceof Problem &&
      e.type === "tag:multiverse,2020:users/problems/unknown_user_id"
    ) {
      throw new UnknownUserError();
    } else {
      throw e;
    }
  }
}
