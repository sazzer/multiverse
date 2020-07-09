import { User } from "../../data/user";
import { World } from "../../data/world";

describe("New World", () => {
  beforeEach(() => {
    const user = new User()
      .withUsername("username")
      .withPassword("Pa55word")
      .withDisplayName("Test User")
      .withEmailAddress("testuser@example.com");
    cy.seedData(user);
    cy.seedData(new World().withUrlSlug("existing").withOwnerId(user._userId));

    cy.login("username", "Pa55word");
  });

  it("Creating with a blank form", () => {
    cy.getPageHeader(({ getUserMenu }) => {
      getUserMenu(({ newWorld }) => {
        newWorld();
      });
    });

    cy.getProfilePage("Test User", ({ getNewWorldForm }) => {
      getNewWorldForm(({ name, description, urlSlug, submit }) => {
        submit();
        name()
          .should("be.visible")
          .should("have.value", "")
          .should("have.focus")
          .should("not.have.error");
        description()
          .should("be.visible")
          .should("have.value", "")
          .should("not.have.error");
        urlSlug()
          .should("be.visible")
          .should("have.value", "")
          .should("not.have.error");
      });
    });
  });

  it("Creating with a name that doesn't generate a slug", () => {
    cy.getPageHeader(({ getUserMenu }) => {
      getUserMenu(({ newWorld }) => {
        newWorld();
      });
    });

    cy.getProfilePage("Test User", ({ getNewWorldForm }) => {
      getNewWorldForm(({ name, description, urlSlug, submit }) => {
        name().type("#^");

        submit();

        name()
          .should("be.visible")
          .should("have.value", "#^")
          .should("not.have.error");
        description()
          .should("be.visible")
          .should("have.value", "")
          .should("not.have.error");
        urlSlug()
          .should("be.visible")
          .should("have.value", "")
          .should("have.focus")
          .should("not.have.error");
      });
    });
  });

  it("Creating with a duplicate slug", () => {
    cy.getPageHeader(({ getUserMenu }) => {
      getUserMenu(({ newWorld }) => {
        newWorld();
      });
    });

    cy.getProfilePage("Test User", ({ getNewWorldForm }) => {
      getNewWorldForm(({ name, description, urlSlug, submit }) => {
        name().type("Existing");

        submit();

        name()
          .should("be.visible")
          .should("have.value", "Existing")
          .should("not.have.error");
        description()
          .should("be.visible")
          .should("have.value", "")
          .should("not.have.error");
        urlSlug()
          .should("be.visible")
          .should("have.value", "")
          .should("have.error")
          .should(
            "have.errorMessage",
            "You already have a world with this URL Slug"
          );
      });
    });
  });
});
