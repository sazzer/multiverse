Cypress.Commands.add("getProfileForm", (callback) => {
  cy.findByLabelText("Profile", { selector: "form" }).within(() => {
    callback({
      username: () => cy.findByLabelText("Username"),
      emailAddress: () => cy.findByLabelText("Email Address"),
      displayName: () => cy.findByLabelText("Display Name"),
    });
  });
});
