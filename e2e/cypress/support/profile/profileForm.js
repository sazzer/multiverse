import { getProfileMenu } from "./menu";

export function getProfileForm(callback) {
  getProfileMenu(({ profile }) => {
    profile().click();
  });

  cy.findByLabelText("Profile", { selector: "form" }).within(() => {
    callback({
      username: () => cy.findByLabelText("Username"),
      emailAddress: () => cy.findByLabelText("Email Address"),
      displayName: () => cy.findByLabelText("Display Name"),
      submit: () => {
        cy.findByText("Save Changes", { selector: "button" }).click();
        cy.get("fieldset").should("not.be.disabled");
      },
      errorMessage: () => cy.findByRole("alert", { selector: ".alert-error" }),
      successMessage: () =>
        cy.findByRole("status", { selector: ".alert-success" }),
    });
  });
}
