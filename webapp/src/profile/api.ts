import { request } from "../api";

/** The shape of a User on the API */
interface UserResponse {
  /** The users unique username */
  username: string;
  /** The users display name */
  display_name: string;
  /** The users email address */
  email_address: string;
  /** The avatar for the user */
  avatar_url?: string;
}

/** The shape of a User in the application */
export interface User {
  /** The ID of the user */
  userId: string;
  /** The users unique username */
  username: string;
  /** The users display name */
  displayName: string;
  /** The users email address */
  emailAddress: string;
  /** The avatar for the user */
  avatar_Url?: string;
}

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
    avatar_Url: response.body?.avatar_url,
  };
}
