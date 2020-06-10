import { User } from "../../data/user";

describe("Registering a new user", () => {
  beforeEach(() => {
    cy.seedData(new User().withUsername("username"));

    cy.getStartAuthenticationForm(({ username, submit }) => {
      username().type("username");
      submit();
    });
  });

  it("Without filling out the form", () => {
    cy.getLoginForm(({ submit }) => {
      submit();
    });

    cy.getLoginForm(({ username, password }) => {
      username()
        .should("be.visible")
        .should("have.value", "username")
        .should("not.have.error");
      password()
        .should("be.visible")
        .should("have.value", "")
        .should("not.have.error");
    });
  });

  it("With a whitespace password", () => {
    cy.getLoginForm(({ password, submit }) => {
      password().type("  ");
      submit();
    });

    cy.getLoginForm(({ username, password }) => {
      username()
        .should("be.visible")
        .should("have.value", "username")
        .should("not.have.error");
      password()
        .should("be.visible")
        .should("have.value", "  ")
        .should("have.errorMessage", "Please enter a password");
    });
  });
});
