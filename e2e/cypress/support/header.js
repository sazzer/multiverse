function getUserMenu(callback) {
  cy.findByTestId("userMenu").within(($menu) => {
    callback({
      getElement: () => $menu,
      getDropdownButton: () => cy.get('button[data-toggle="dropdown"]'),
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
