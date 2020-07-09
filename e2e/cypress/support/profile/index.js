import { getChangePasswordForm } from "./changePasswordForm";
import { getNewWorldForm } from "./newWorldForm";
import { getProfileForm } from "./profileForm";
import { getProfileMenu } from "./menu";

Cypress.Commands.add("getProfilePage", (user, callback) => {
  cy.findByLabelText(`User Profile: ${user}`).within(() => {
    if (callback) {
      callback({
        getProfileMenu,
        getProfileForm,
        getChangePasswordForm,
        getNewWorldForm,
      });
    }
  });
});
