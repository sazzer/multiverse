import "@testing-library/cypress/add-commands";
import "cypress-commands";
import "./forms";
import "./authentication";
import "./database";
import "./profile";

beforeEach(() => {
  cy.visit("/");
});
