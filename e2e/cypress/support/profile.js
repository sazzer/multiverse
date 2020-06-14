function getProfileMenu(callback) {
  cy.findByRole("navigation").within(() => {
    callback({
      profile: () => cy.findByText("Profile"),
      changePassword: () => cy.findByText("Change Password"),
    });
  });
}

function getProfileForm(callback) {
  getProfileMenu(({ profile }) => {
    profile().click();
  });

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

function getChangePasswordForm(callback) {
  getProfileMenu(({ changePassword }) => {
    changePassword().click();
  });

  cy.findByLabelText("Change Password", { selector: "form" }).within(() => {
    callback({
      oldPassword: () => cy.findByLabelText("Current Password"),
      password: () => cy.findByLabelText("New Password"),
      password2: () => cy.findByLabelText("Repeat New Password"),
      submit: () => {
        cy.findByText("Change Password", { selector: "button" }).click();
        cy.get("fieldset").should("not.be.disabled");
      },
      errorMessage: () => cy.findByRole("alert"),
    });
  });
}

Cypress.Commands.add("getProfilePage", (user, callback) => {
  cy.findByLabelText(`User Profile: ${user}`).within(() => {
    if (callback) {
      callback({
        getProfileMenu,
        getProfileForm,
        getChangePasswordForm,
      });
    }
  });
});
