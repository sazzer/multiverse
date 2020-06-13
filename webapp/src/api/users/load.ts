import { User } from "./model";
import { UserResponse } from "./response";
import { request } from "../http";

/**
 * Load a user from the server
 *
 * @param userId The ID of the user to load
 * @param ignoreCache Whether to ignore the cache or not
 */
export async function loadUser(
  userId: string,
  ignoreCache?: boolean
): Promise<User> {
  const response = await request<UserResponse>("/users/{userId}", {
    urlParams: {
      userId,
    },
    ignoreCache,
  });

  return {
    userId,
    username: response.body!!.username,
    displayName: response.body!!.display_name,
    emailAddress: response.body!!.email_address,
    avatarUrl: response.body?.avatar_url,
  };
}
