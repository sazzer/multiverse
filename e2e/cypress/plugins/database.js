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
 * @return the result of the query
 */
async function query(sql) {
  console.log("Executing query", sql);
  return await pool.query(sql);
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

module.exports = {
  openPool,
  reset,
};
