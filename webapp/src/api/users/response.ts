/** The shape of a User on the API */
export interface UserResponse {
  /** The users unique username */
  username: string;
  /** The users display name */
  display_name: string;
  /** The users email address */
  email_address: string;
  /** The avatar for the user */
  avatar_url?: string;
}
