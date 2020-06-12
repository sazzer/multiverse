import { storeToken } from "../../api/http/token";

/**
 * Shape of the response from authenticating a user
 */
export interface TokenResponse {
  /** The access token */
  token: string;
  /** The time the token is valid until */
  valid_until: string;
  /** The ID of the user that authenticated */
  user_id: string;
  /** The display name of the user */
  display_name: string;
}

/**
 * Record a token with the system to use for future API calls
 * @param token The token to record
 */
export function recordToken(token: TokenResponse) {
  storeToken(token.token, new Date(token.valid_until));
}
