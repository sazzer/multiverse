import { SeedData } from "../data/seed";

beforeEach(() => {
  cy.task("db:reset");
});

Cypress.Commands.add("seedData", (data) => {
  cy.wrap(data.sql()).then((sql) => {
    cy.wrap(data.binds()).then((binds) => {
      cy.task("db:seed", { sql, binds });
    });
  });
});
