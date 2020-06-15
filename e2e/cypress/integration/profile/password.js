import { User } from "../../data/user";

describe("Change Password", () => {
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

  it("Saving with an empty form", () => {
    cy.getProfilePage("Test User", ({ getChangePasswordForm }) => {
      getChangePasswordForm(({ submit }) => {
        submit();
      });
    });

    cy.logout();
    cy.login("username", "Pa55word");
  });

  it("Saving with an entirely whitespace form", () => {
    cy.getProfilePage("Test User", ({ getChangePasswordForm }) => {
      getChangePasswordForm(({ oldPassword, password, password2, submit }) => {
        oldPassword().type(" ");
        password().type(" ");
        password2().type(" ");
        submit();
      });
    });

    cy.getProfilePage("Test User", ({ getChangePasswordForm }) => {
      getChangePasswordForm(
        ({ oldPassword, password, password2, successMessage }) => {
          oldPassword()
            .should("have.value", " ")
            .should("have.errorMessage", "Please enter a password");
          password()
            .should("have.value", " ")
            .should("have.errorMessage", "Please enter a password");
          password2()
            .should("have.value", " ")
            .should("have.errorMessage", "Please enter a password");
          successMessage().should("not.be.visible");
        }
      );
    });

    cy.logout();
    cy.login("username", "Pa55word");
  });

  it("Saving with mismatched new passwords", () => {
    cy.getProfilePage("Test User", ({ getChangePasswordForm }) => {
      getChangePasswordForm(({ oldPassword, password, password2, submit }) => {
        oldPassword().type("Pa55word");
        password().type("newPassword");
        password2().type("NewPa55word");
        submit();
      });
    });

    cy.getProfilePage("Test User", ({ getChangePasswordForm }) => {
      getChangePasswordForm(
        ({ oldPassword, password, password2, successMessage }) => {
          oldPassword()
            .should("have.value", "Pa55word")
            .should("not.have.error");
          password()
            .should("have.value", "newPassword")
            .should("not.have.error");
          password2()
            .should("have.value", "NewPa55word")
            .should("have.errorMessage", "Passwords do not match");
          successMessage().should("not.be.visible");
        }
      );
    });

    cy.logout();
    cy.login("username", "Pa55word");
  });

  it("Saving with incorrect old password", () => {
    cy.getProfilePage("Test User", ({ getChangePasswordForm }) => {
      getChangePasswordForm(({ oldPassword, password, password2, submit }) => {
        oldPassword().type("Incorrect");
        password().type("newPassword");
        password2().type("newPassword");
        submit();
      });
    });

    cy.getProfilePage("Test User", ({ getChangePasswordForm }) => {
      getChangePasswordForm(
        ({
          oldPassword,
          password,
          password2,
          errorMessage,
          successMessage,
        }) => {
          oldPassword()
            .should("have.value", "Incorrect")
            .should("not.have.error");
          password()
            .should("have.value", "newPassword")
            .should("not.have.error");
          password2()
            .should("have.value", "newPassword")
            .should("not.have.error");
          errorMessage()
            .should("be.visible")
            .should("have.text", "The old password was incorrect");
          successMessage().should("not.be.visible");
        }
      );
    });

    cy.logout();
    cy.login("username", "Pa55word");
  });

  it("Saving successfully", () => {
    cy.getProfilePage("Test User", ({ getChangePasswordForm }) => {
      getChangePasswordForm(({ oldPassword, password, password2, submit }) => {
        oldPassword().type("Pa55word");
        password().type("newPassword");
        password2().type("newPassword");
        submit();
      });
    });

    cy.getProfilePage("Test User", ({ getChangePasswordForm }) => {
      getChangePasswordForm(
        ({
          oldPassword,
          password,
          password2,
          errorMessage,
          successMessage,
        }) => {
          oldPassword()
            .should("have.value", "Pa55word")
            .should("not.have.error");
          password()
            .should("have.value", "newPassword")
            .should("not.have.error");
          password2()
            .should("have.value", "newPassword")
            .should("not.have.error");
          errorMessage().should("not.be.visible");
          successMessage()
            .should("be.visible")
            .should("have.text", "Password changed successfully");
        }
      );
    });

    cy.logout();
    cy.login("username", "newPassword");
  });
});
