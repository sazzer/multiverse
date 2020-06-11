function getUserMenu(callback) {
  cy.findByTestId("userMenu").within(($menu) => {
    callback({
      getElement: () => $menu,
      getDropdownButton: () => cy.get('button[data-toggle="dropdown"]'),
      logout: () => {
        cy.get('button[data-toggle="dropdown"]').click();
        cy.findByText("Log Out", { role: "menuitem" }).click();
      },
    });
  });
}

Cypress.Commands.add("getPageHeader", (callback) => {
  cy.findByRole("banner").within(() => {
    callback({
      getUserMenu,
    });
  });
});
