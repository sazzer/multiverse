Cypress.Commands.add("withinStartAuthentication", (callback) => {
  cy.get("form").within(() => {
    cy.findByRole("heading", { "aria-level": "2" })
      .should("be.visible")
      .should("have.text", "Login / Register");

    callback();
  });
});

Cypress.Commands.add("startAuthentication", (username) => {
  cy.withinStartAuthentication(() => {
    if (username !== "") {
      cy.findByLabelText("Username").type(username);
    }
    cy.findByText("Login / Register", { selector: "button" }).click();
  });
});

Cypress.Commands.add("getStartAuthenticationForm", (callback) => {
  cy.withinStartAuthentication(() => {
    const username = cy.findByLabelText("Username");

    callback({
      username,
    });
  });
});
