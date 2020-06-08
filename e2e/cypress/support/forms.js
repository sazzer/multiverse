chai.Assertion.addMethod("error", function (expectedError) {
  const target = this._obj;
  new chai.Assertion(target).to.exist;
  new chai.Assertion(target).to.have.attr("aria-invalid", "true");

  const describedBy = target.attr("aria-describedby");
  const describedByElem = Cypress.$(`#${describedBy}[role="alert"]`);
  new chai.Assertion(describedByElem).to.exist;
  new chai.Assertion(describedByElem).to.be.visible;
  new chai.Assertion(describedByElem).to.have.text(expectedError);
});
