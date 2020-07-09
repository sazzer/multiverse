function getUserMenu(callback) {
  cy.findByTestId("userMenu").within(($menu) => {
    if (callback) {
      callback({
        getElement: () => $menu,
        getDropdownButton: () => cy.get('button[data-toggle="dropdown"]'),
        logout: () => {
          cy.get('button[data-toggle="dropdown"]').click();
          cy.findByText("Log Out", { role: "menuitem" }).click();
        },
        newWorld: () => {
          cy.get('button[data-toggle="dropdown"]').click();
          cy.findByText("New World", { role: "menuitem" }).click();
        },
      });
    }
  });
}

Cypress.Commands.add("getPageHeader", (callback) => {
  cy.findByRole("banner").within(() => {
    callback({
      getUserMenu,
    });
  });
});
