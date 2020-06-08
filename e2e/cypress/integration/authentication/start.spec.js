describe("Starting Authentication", () => {
  it("Without entering a username", () => {
    cy.visit("/");
    cy.startAuthentication("");
  });

  it("Entering a whitespace username", () => {
    cy.visit("/");
    cy.startAuthentication(" ");

    cy.withinStartAuthentication(() => {
      cy.findByLabelText("Username")
        .should("be.visible")
        .should("have.value", " ")
        .should("have.error", "Please enter a username");
    });
  });
});
