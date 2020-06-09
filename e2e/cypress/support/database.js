import { SeedData } from "../data/seed";

beforeEach(() => {
  cy.task("db:reset");
});

Cypress.Commands.add("seedData", (data) => {
  cy.task("db:seed", { sql: data.sql, binds: data.binds });
});
