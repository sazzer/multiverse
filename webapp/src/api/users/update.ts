import { Problem, request } from "../http";

import { UnknownUserError } from "./errors";
import { User } from "./model";
import { UserResponse } from "./response";

/**
 * Update the details of a user
 * @param user The details of the user to update
 */
export async function updateUser(user: User) {
  try {
    await request<UserResponse>(user.selfLink, {
      method: "PATCH",
      body: {
        email_address: user.emailAddress,
        display_name: user.displayName,
        avatar_url: user.avatarUrl,
      },
    });
  } catch (e) {
    if (e instanceof Problem) {
      switch (e.type) {
        case "tag:multiverse,2020:users/problems/invalid_old_password":
          throw new InvalidOldPasswordError();
        case "tag:multiverse,2020:users/problems/unknown_user_id":
          throw new UnknownUserError();
      }
    }
    throw e;
  }
}

/**
 * Error from changing the users password to indicate that the provided old password was wrong
 */
export class InvalidOldPasswordError extends Error {
  constructor() {
    super("Provided old password was invalid");
  }
}

/**
 * Change the password of a user
 * @param user The URL of the user to update
 * @param oldPassword The users old password
 * @param password The users new password
 */
export async function changePassword(
  user: string,
  oldPassword: string,
  password: string
) {
  try {
    await request<UserResponse>(user, {
      method: "PATCH",
      body: {
        old_password: oldPassword,
        password: password,
      },
    });
  } catch (e) {
    if (e instanceof Problem) {
      switch (e.type) {
        case "tag:multiverse,2020:users/problems/invalid_old_password":
          throw new InvalidOldPasswordError();
        case "tag:multiverse,2020:users/problems/unknown_user_id":
          throw new UnknownUserError();
      }
    }
    throw e;
  }
}
