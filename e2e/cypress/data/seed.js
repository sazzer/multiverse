/**
 * Base class that all Seed Data can inherit from
 */
export class SeedData {
  /**
   * Generate the SQL to insert the data into the database
   */
  async sql() {
    return "";
  }

  /**
   * Generate the binds for the SQL
   */
  async binds() {
    return [];
  }
}
