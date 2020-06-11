import { User } from "../../data/user";

describe("Registering a new user", () => {
  beforeEach(() => {
    cy.seedData(
      new User()
        .withUsername("username")
        .withPassword("Pa55word")
        .withDisplayName("Test User")
        .withEmailAddress("testuser@example.com")
    );

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

  it("With an incorrect password", () => {
    cy.getLoginForm(({ password, submit }) => {
      password().type("incorrect");
      submit();
    });

    cy.getLoginForm(({ username, password, errorMessage }) => {
      username()
        .should("be.visible")
        .should("have.value", "username")
        .should("not.have.error");
      password()
        .should("be.visible")
        .should("have.value", "incorrect")
        .should("not.have.error");
      errorMessage()
        .should("be.visible")
        .should("have.text", "Invalid username or password");
    });
  });

  it("With the correct password", () => {
    cy.getLoginForm(({ password, submit }) => {
      password().type("Pa55word");
      submit();
    });

    cy.getProfileForm(({ username, emailAddress, displayName }) => {
      username()
        .should("be.visible")
        .should("have.value", "username")
        .should("not.have.error");
      emailAddress()
        .should("be.visible")
        .should("have.value", "testuser@example.com")
        .should("not.have.error");
      displayName()
        .should("be.visible")
        .should("have.value", "Test User")
        .should("not.have.error");
    });
  });
});
