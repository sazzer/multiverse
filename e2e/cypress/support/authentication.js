Cypress.Commands.add("getStartAuthenticationForm", (callback) => {
  cy.findByLabelText("Login / Register", { selector: "form" }).within(() => {
    callback({
      username: () => cy.findByLabelText("Username"),
      submit: () =>
        cy.findByText("Login / Register", { selector: "button" }).click(),
    });
  });
});

Cypress.Commands.add("getRegisterForm", (callback) => {
  cy.findByLabelText("Register", { selector: "form" }).within(() => {
    callback({
      username: () => cy.findByLabelText("Username"),
      emailAddress: () => cy.findByLabelText("Email Address"),
      displayName: () => cy.findByLabelText("Display Name"),
      password: () => cy.findByLabelText("Password"),
      repeatPassword: () => cy.findByLabelText("Repeat Password"),
      submit: () => cy.findByText("Register", { selector: "button" }).click(),
      cancel: () => cy.findByText("Cancel", { selector: "button" }).click(),
      errorMessage: () => cy.findByRole("alert"),
    });
  });
});

Cypress.Commands.add("getLoginForm", (callback) => {
  cy.findByLabelText("Login", { selector: "form" }).within(() => {
    callback({
      username: () => cy.findByLabelText("Username"),
      password: () => cy.findByLabelText("Password"),
      submit: () => cy.findByText("Login", { selector: "button" }).click(),
      cancel: () => cy.findByText("Cancel", { selector: "button" }).click(),
      errorMessage: () => cy.findByRole("alert"),
    });
  });
});
