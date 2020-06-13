import { User } from "./model";
import { UserResponse } from "./response";
import { request } from "../http";

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
}
