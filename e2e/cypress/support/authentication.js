Cypress.Commands.add("getStartAuthenticationForm", (callback) => {
  cy.get("form").within(() => {
    cy.findByRole("heading", { "aria-level": "2" })
      .should("be.visible")
      .should("have.text", "Login / Register");
    callback({
      username: () => cy.findByLabelText("Username"),
      submit: () =>
        cy.findByText("Login / Register", { selector: "button" }).click(),
    });
  });
});

Cypress.Commands.add("getRegisterForm", (callback) => {
  cy.get("form").within(() => {
    cy.findByRole("heading", { "aria-level": "2" })
      .should("be.visible")
      .should("have.text", "Register");
    callback({
      username: () => cy.findByLabelText("Username"),
      emailAddress: () => cy.findByLabelText("Email Address"),
      displayName: () => cy.findByLabelText("Display Name"),
      password: () => cy.findByLabelText("Password"),
      repeatPassword: () => cy.findByLabelText("Repeat Password"),
    });
  });
});

Cypress.Commands.add("getLoginForm", (callback) => {
  cy.get("form").within(() => {
    cy.findByRole("heading", { "aria-level": "2" })
      .should("be.visible")
      .should("have.text", "Login");
    callback({
      username: () => cy.findByLabelText("Username"),
      password: () => cy.findByLabelText("Password"),
    });
  });
});
