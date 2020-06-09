const database = require("./database");

module.exports = (on, config) => {
  database.openPool(config.env.POSTGRES_URL);
  on("task", {
    "db:reset": database.reset,
    "db:seed": database.seed,
  });
};
