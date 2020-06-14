import { Problem, request } from "../http";

import { User } from "./model";
import { UserResponse } from "./response";

/**
 * Update the details of a user
 * @param user The details of the user to update
 */
export async function updateUser(user: User) {
  await request<UserResponse>("/users/{userId}", {
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

/**
 * Error from changing the users password to indicate that the provided old password was wrong
 */
export class InvalidOldPasswordError extends Error {}

/**
 * Change the password of a user
 * @param userId The ID of the user to update
 * @param oldPassword The users old password
 * @param password The users new password
 */
export async function changePassword(
  userId: string,
  oldPassword: string,
  password: string
) {
  try {
    await request<UserResponse>("/users/{userId}", {
      method: "PATCH",
      urlParams: {
        userId: userId,
      },
      body: {
        old_password: oldPassword,
        password: password,
      },
    });
  } catch (e) {
    if (
      e instanceof Problem &&
      e.type === "tag:multiverse,2020:users/problems/invalid_old_password"
    ) {
      throw new InvalidOldPasswordError();
    } else {
      throw e;
    }
  }
}
