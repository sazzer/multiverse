/**
 * Error to indicate that the URL Slug was a duplicate
 */
export class DuplicateUrlSlugError extends Error {
  constructor() {
    super("The provided URL Slug already exists for this user");
  }
}
