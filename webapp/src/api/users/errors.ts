/**
 * Error to indicate that the user was unknown
 */
export class UnknownUserError extends Error {
  constructor() {
    super("The requested user was unknown");
  }
}
