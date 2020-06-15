export class UnauthorizedError extends Error {
  constructor() {
    super("API call was unauthorized");
  }
}
