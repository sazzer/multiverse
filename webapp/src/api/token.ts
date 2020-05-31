/** The details of an access token */
interface Token {
  /** The actual token */
  token: string;
  /** The valid until date */
  validUntil: Date;
}

/** The actual token for this session */
let currentToken: Token | undefined = undefined;

/**
 * Store a new token for this session
 * @param token The token
 * @param validUntil The token valid until date
 */
export function storeToken(token: string, validUntil: Date) {
  currentToken = {
    token,
    validUntil,
  };
}

/**
 * Get the current token to use for API requests
 * @return the token
 */
export function getToken(): string | undefined {
  let result;

  if (currentToken !== undefined) {
    if (currentToken.validUntil > new Date()) {
      result = currentToken.token;
    }
  }

  return result;
}

/**
 * Clear the current token to use for API requests
 */
export function clearToken() {
  currentToken = undefined;
}
