import debug from "debug";

/** The logger to use */
const LOGGER = debug("multiverse:api:http:token");

/** The key into Session Storage where the token is stored */
const TOKEN_STORAGE_KEY = "multiverse_token";

/** The details of an access token */
interface Token {
  /** The actual token */
  token: string;
  /** The valid until date */
  validUntil: Date;
}

/** The details of an access token as stored in the Session Storage */
interface StoredToken {
  /** The actual token */
  token: string;
  /** The valid until date */
  validUntil: string;
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

  sessionStorage.setItem(
    TOKEN_STORAGE_KEY,
    JSON.stringify({
      token: token,
      validUntil: validUntil.toISOString(),
    })
  );
}

/**
 * Get the current token to use for API requests
 * @return the token
 */
export function getToken(): string | undefined {
  let result;

  if (currentToken === undefined) {
    const storedToken = sessionStorage.getItem(TOKEN_STORAGE_KEY);
    if (storedToken) {
      const parsedToken = JSON.parse(storedToken) as StoredToken;
      currentToken = {
        token: parsedToken.token,
        validUntil: new Date(parsedToken.validUntil),
      };
      LOGGER("Loaded access token from session storage: %o", currentToken);
    }
  }

  if (currentToken !== undefined) {
    if (currentToken.validUntil > new Date()) {
      result = currentToken.token;
    } else {
      clearToken();
    }
  }

  return result;
}

/**
 * Clear the current token to use for API requests
 */
export function clearToken() {
  currentToken = undefined;
  sessionStorage.removeItem(TOKEN_STORAGE_KEY);
}
