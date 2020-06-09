const { Pool } = require("pg");

/** The connection pool */
let pool;

/**
 * Open the connection pool
 * @param url The URL to connect to
 */
function openPool(url) {
  if (pool === undefined) {
    console.log("Connecting to database", url);
    pool = new Pool({ connectionString: url });
  }
  return pool;
}

/**
 * Execute a query against the database
 * @param sql The SQL to execute
 * @param binds The binds to the query
 * @return the result of the query
 */
async function query(sql, binds) {
  console.log("Executing query", sql, binds);
  try {
    return await pool.query(sql, binds);
  } catch (e) {
    console.log("Error executing query", e);
    throw e;
  }
}

/**
 * Reset the database
 */
async function reset() {
  console.log("Resetting database");
  const tables = await query(
    "SELECT table_name FROM information_schema.tables WHERE table_schema IN ('public')"
  );

  const toTruncate = tables.rows
    .map((row) => row.table_name)
    .filter((table) => table !== "__migrations");
  await query("TRUNCATE " + toTruncate.join(","));
  return null;
}

/**
 * Seed the database with some data provided.
 * The provided data needs to have a property "sql" with the SQL to execute, and optionally a property "binds"
 * with any SQL binds for this SQL to execute.
 *
 * @param {object} data The data to seed the database with
 */
async function seed(data) {
  console.log("Seeding database");
  if (data.sql) {
    await query(data.sql, data.binds);
  }
  return null;
}

module.exports = {
  openPool,
  reset,
  seed,
};
