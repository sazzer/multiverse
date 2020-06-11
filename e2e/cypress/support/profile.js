function getProfileForm(callback) {
  cy.findByLabelText("Profile", { selector: "form" }).within(() => {
    callback({
      username: () => cy.findByLabelText("Username"),
      emailAddress: () => cy.findByLabelText("Email Address"),
      displayName: () => cy.findByLabelText("Display Name"),
    });
  });
}

Cypress.Commands.add("getProfilePage", (user, callback) => {
  cy.findByLabelText(`User Profile: ${user}`).within(() => {
    callback({
      getProfileForm,
    });
  });
});
