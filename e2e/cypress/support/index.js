import "@testing-library/cypress/add-commands";
import "cypress-commands";
import "./forms";
import "./authentication";
import "./database";

beforeEach(() => {
  cy.visit("/");
});
