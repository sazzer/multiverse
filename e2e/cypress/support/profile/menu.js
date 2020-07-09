export function getProfileMenu(callback) {
  cy.findByRole("navigation").within(() => {
    callback({
      profile: () => cy.findByText("Profile"),
      changePassword: () => cy.findByText("Change Password"),
    });
  });
}
