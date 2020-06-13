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
