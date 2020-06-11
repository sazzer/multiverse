import { User } from "../../data/user";

describe("User Profile", () => {
  beforeEach(() => {
    cy.seedData(
      new User()
        .withUsername("username")
        .withPassword("Pa55word")
        .withDisplayName("Test User")
        .withEmailAddress("testuser@example.com")
    );

    cy.login("username", "Pa55word");
  });

  it("Saving with a blank email", () => {
    cy.getProfilePage("Test User", ({ getProfileForm }) => {
      getProfileForm(({ emailAddress, displayName, submit }) => {
        emailAddress().clear();
        displayName().clear().type("New User");
        submit();
      });
    });

    cy.getPageHeader(({ getUserMenu }) => {
      getUserMenu(({ getDropdownButton }) => {
        getDropdownButton()
          .should("be.visible")
          .should("have.text", "Test User");
      });
    });

    cy.getProfilePage("Test User");
  });

  it("Saving with a blank display name", () => {
    cy.getProfilePage("Test User", ({ getProfileForm }) => {
      getProfileForm(({ emailAddress, displayName, submit }) => {
        emailAddress().clear().type("newuser@example.com");
        displayName().clear();
        submit();
      });
    });

    cy.getPageHeader(({ getUserMenu }) => {
      getUserMenu(({ getDropdownButton }) => {
        getDropdownButton()
          .should("be.visible")
          .should("have.text", "Test User");
      });
    });

    cy.getProfilePage("Test User");
  });

  it("Saving with a whitespace display name", () => {
    cy.getProfilePage("Test User", ({ getProfileForm }) => {
      getProfileForm(({ emailAddress, displayName, submit }) => {
        emailAddress().clear().type("newuser@example.com");
        displayName().clear().type("  ");
        submit();
      });
    });

    cy.getPageHeader(({ getUserMenu }) => {
      getUserMenu(({ getDropdownButton }) => {
        getDropdownButton()
          .should("be.visible")
          .should("have.text", "Test User");
      });
    });

    cy.getProfilePage("Test User", ({ getProfileForm }) => {
      getProfileForm(({ displayName }) => {
        displayName().should(
          "have.errorMessage",
          "Please enter a display name"
        );
      });
    });
  });

  it("Saving changes", () => {
    cy.getProfilePage("Test User", ({ getProfileForm }) => {
      getProfileForm(({ emailAddress, displayName, submit }) => {
        emailAddress().clear().type("newuser@example.com");
        displayName().clear().type("New User");
        submit();
      });
    });

    cy.getPageHeader(({ getUserMenu }) => {
      getUserMenu(({ getDropdownButton }) => {
        getDropdownButton()
          .should("be.visible")
          .should("have.text", "New User");
      });
    });

    cy.getProfilePage("New User");
  });

  it("Saving changes and logging in again", () => {
    cy.getProfilePage("Test User", ({ getProfileForm }) => {
      getProfileForm(({ emailAddress, displayName, submit }) => {
        emailAddress().clear().type("newuser@example.com");
        displayName().clear().type("New User");
        submit();
      });
    });

    cy.getPageHeader(({ getUserMenu }) => {
      getUserMenu(({ logout }) => {
        logout();
      });
    });

    cy.login("username", "Pa55word");

    cy.getPageHeader(({ getUserMenu }) => {
      getUserMenu(({ getDropdownButton }) => {
        getDropdownButton()
          .should("be.visible")
          .should("have.text", "New User");
      });
    });

    cy.getProfilePage("New User", ({ getProfileForm }) => {
      getProfileForm(({ username, emailAddress, displayName }) => {
        username()
          .should("be.visible")
          .should("have.value", "username")
          .should("not.have.error");
        emailAddress()
          .should("be.visible")
          .should("have.value", "newuser@example.com")
          .should("not.have.error");
        displayName()
          .should("be.visible")
          .should("have.value", "New User")
          .should("not.have.error");
      });
    });
  });
});
