export function getNewWorldForm(callback) {
  cy.findByLabelText("New World", { selector: "form" }).within(() => {
    callback({
      name: () => cy.findByLabelText("Name"),
      description: () => cy.findByLabelText("Description"),
      urlSlug: () => cy.findByLabelText("URL Slug"),
      submit: () => {
        cy.findByText("Create World", { selector: "button" }).click();
        cy.get("fieldset").should("not.be.disabled");
      },
      errorMessage: () => cy.findByRole("alert"),
      successMessage: () =>
        cy.findByRole("status", { selector: ".alert-success" }),
    });
  });
}
