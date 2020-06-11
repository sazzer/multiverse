import { User } from "../../data/user";

it("Log Out", () => {
  cy.seedData(new User().withUsername("username").withPassword("Pa55word"));

  cy.getStartAuthenticationForm(({ username, submit }) => {
    username().type("username");
    submit();
  });

  cy.getLoginForm(({ password, submit }) => {
    password().type("Pa55word");
    submit();
  });

  cy.getPageHeader(({ getUserMenu }) => {
    getUserMenu(({ logout }) => {
      logout();
    });
  });

  cy.getStartAuthenticationForm();
});
