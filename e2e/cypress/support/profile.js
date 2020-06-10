Cypress.Commands.add("getProfileForm", (callback) => {
  cy.findByLabelText("Profile", { selector: "form" }).within(() => {
    callback({});
  });
});
