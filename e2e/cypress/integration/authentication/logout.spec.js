import { User } from "../../data/user";

it("Log Out", () => {
  cy.seedData(new User().withUsername("username").withPassword("Pa55word"));
  cy.login("username", "Pa55word");

  cy.getPageHeader(({ getUserMenu }) => {
    getUserMenu(({ logout }) => {
      logout();
    });
  });

  cy.getStartAuthenticationForm();
});
