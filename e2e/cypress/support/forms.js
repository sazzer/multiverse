chai.Assertion.addMethod("error", function () {
  const target = this._obj;
  const ariaInvalid = target.attr("aria-invalid");
  this.assert(
    ariaInvalid === "true",
    "expected #{this} to have aria-invalid set to true",
    "expected #{this} not to have aria-invalid set to true",
    target
  );
});

chai.Assertion.addMethod("errorMessage", function (expectedError) {
  const target = this._obj;
  new chai.Assertion(target).to.have.error();

  const describedBy = target.attr("aria-describedby");
  if (describedBy) {
    const selector = describedBy
      .split(" ")
      .map((value) => value.trim())
      .filter((value) => value.length > 0)
      .map((value) => `#${value}[role="alert"]`)
      .join(",");
    const describedByElem = Cypress.$(selector);
    new chai.Assertion(describedByElem).to.exist;
    new chai.Assertion(describedByElem).to.be.visible;
    new chai.Assertion(describedByElem).to.have.text(expectedError);
  }
});
