describe("Starting Authentication", () => {
  it("Without entering a username", () => {
    cy.visit("/");
    cy.getStartAuthenticationForm(({ submit }) => {
      submit();
    });
  });

  it("Entering a whitespace username", () => {
    cy.visit("/");
    cy.getStartAuthenticationForm(({ username, submit }) => {
      username().type(" ");
      submit();

      username()
        .should("be.visible")
        .should("have.value", " ")
        .should("have.errorMessage", "Please enter a username");
    });
  });

  ["username", "!@#$%^&*()", "<>?,./:\"|;'\\{}[]", "用户名"].forEach(
    (usernameInput) => {
      it(`Entering a valid, unknown username: ${usernameInput}`, () => {
        cy.visit("/");
        cy.getStartAuthenticationForm(({ username, submit }) => {
          username().type(usernameInput);
          submit();
        });

        cy.getRegisterForm(
          ({
            username,
            emailAddress,
            displayName,
            password,
            repeatPassword,
          }) => {
            username()
              .should("be.visible")
              .should("have.value", usernameInput)
              .should("not.have.error");
            emailAddress()
              .should("be.visible")
              .should("have.value", "")
              .should("not.have.error");
            displayName()
              .should("be.visible")
              .should("have.value", "")
              .should("not.have.error");
            password()
              .should("be.visible")
              .should("have.value", "")
              .should("not.have.error");
            repeatPassword()
              .should("be.visible")
              .should("have.value", "")
              .should("not.have.error");
          }
        );
      });
    }
  );
});
