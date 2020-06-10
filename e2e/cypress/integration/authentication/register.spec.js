import { User } from "../../data/user";

describe("Registering a new user", () => {
  beforeEach(() => {
    cy.getStartAuthenticationForm(({ username, submit }) => {
      username().type("username");
      submit();
    });
  });

  it("Without filling out the form", () => {
    cy.getRegisterForm(({ submit }) => {
      submit();
    });

    cy.getRegisterForm(
      ({ username, emailAddress, displayName, password, repeatPassword }) => {
        username()
          .should("be.visible")
          .should("have.value", "username")
          .should("not.have.error");
        emailAddress()
          .should("be.visible")
          .should("have.value", "")
          .should("not.have.error");
        displayName()
          .should("be.visible")
          .should("have.value", "")
          .should("not.have.error");
        password()
          .should("be.visible")
          .should("have.value", "")
          .should("not.have.error");
        repeatPassword()
          .should("be.visible")
          .should("have.value", "")
          .should("not.have.error");
      }
    );
  });

  it("Whitespace only passwords", () => {
    cy.getRegisterForm(
      ({ emailAddress, displayName, password, repeatPassword, submit }) => {
        emailAddress().type("testuser@example.com");
        displayName().type("Test User");
        password().type("  ");
        repeatPassword().type("  ");
        submit();
      }
    );

    cy.getRegisterForm(
      ({ username, emailAddress, displayName, password, repeatPassword }) => {
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
        password()
          .should("be.visible")
          .should("have.value", "  ")
          .should("have.errorMessage", "Please enter a password");
        repeatPassword()
          .should("be.visible")
          .should("have.value", "  ")
          .should("not.have.error");
      }
    );
  });

  it("Mismatched passwords", () => {
    cy.getRegisterForm(
      ({ emailAddress, displayName, password, repeatPassword, submit }) => {
        emailAddress().type("testuser@example.com");
        displayName().type("Test User");
        password().type("Password");
        repeatPassword().type("pa55word");
        submit();
      }
    );

    cy.getRegisterForm(
      ({ username, emailAddress, displayName, password, repeatPassword }) => {
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
        password()
          .should("be.visible")
          .should("have.value", "Password")
          .should("not.have.error");
        repeatPassword()
          .should("be.visible")
          .should("have.value", "pa55word")
          .should("have.errorMessage", "Passwords do not match");
      }
    );
  });

  it("Duplicate username", () => {
    cy.seedData(new User().withUsername("username"));

    cy.getRegisterForm(
      ({ emailAddress, displayName, password, repeatPassword, submit }) => {
        emailAddress().type("testuser@example.com");
        displayName().type("Test User");
        password().type("Password");
        repeatPassword().type("Password");
        submit();
      }
    );

    cy.getRegisterForm(
      ({
        username,
        emailAddress,
        displayName,
        password,
        repeatPassword,
        errorMessage,
      }) => {
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
        password()
          .should("be.visible")
          .should("have.value", "Password")
          .should("not.have.error");
        repeatPassword()
          .should("be.visible")
          .should("have.value", "Password")
          .should("not.have.error");
        errorMessage()
          .should("be.visible")
          .should("have.text", "That username is already registered");
      }
    );
  });
});
