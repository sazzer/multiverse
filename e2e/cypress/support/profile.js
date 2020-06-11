function getProfileForm(callback) {
  cy.findByLabelText("Profile", { selector: "form" }).within(() => {
    callback({
      username: () => cy.findByLabelText("Username"),
      emailAddress: () => cy.findByLabelText("Email Address"),
      displayName: () => cy.findByLabelText("Display Name"),
      submit: () =>
        cy.findByText("Save Changes", { selector: "button" }).click(),
    });
  });
}

Cypress.Commands.add("getProfilePage", (user, callback) => {
  cy.findByLabelText(`User Profile: ${user}`).within(() => {
    if (callback) {
      callback({
        getProfileForm,
      });
    }
  });
});
