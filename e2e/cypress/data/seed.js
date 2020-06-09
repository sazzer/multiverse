/**
 * Base class that all Seed Data can inherit from
 */
export class SeedData {
  /**
   * Generate the SQL to insert the data into the database
   */
  get sql() {
    return "";
  }

  /**
   * Generate the binds for the SQL
   */
  get binds() {
    return [];
  }
}
