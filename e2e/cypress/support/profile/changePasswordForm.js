import { getProfileMenu } from "./menu";

export function getChangePasswordForm(callback) {
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
      successMessage: () =>
        cy.findByRole("status", { selector: ".alert-success" }),
    });
  });
}
