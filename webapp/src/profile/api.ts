import { request } from "../api/http";

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
  avatarUrl?: string;
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
    avatarUrl: response.body?.avatar_url,
  };
}

/**
 * Update the details of a user
 * @param user The details of the user to update
 */
export async function updateUser(user: User) {
  const response = await request<UserResponse>("/users/{userId}", {
    method: "PATCH",
    urlParams: {
      userId: user.userId,
    },
    body: {
      email_address: user.emailAddress,
      display_name: user.displayName,
      avatar_url: user.avatarUrl,
    },
  });

  console.log(response);
}
