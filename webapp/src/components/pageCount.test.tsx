import { PageCount } from "./pageCount";
import React from "react";
import { render } from "@testing-library/react";

describe("Rendering a page count", () => {
  test("No pages", () => {
    const { container } = render(
      <PageCount offset={0} total={0} thisPage={0} />
    );
    expect(container).toMatchInlineSnapshot(`<div />`);
  });

  test("Only page", () => {
    const { container } = render(
      <PageCount offset={0} total={10} thisPage={10} />
    );
    expect(container).toMatchInlineSnapshot(`
      <div>
        1 to 10 of 10
      </div>
    `);
  });

  test("Middle page", () => {
    const { container } = render(
      <PageCount offset={10} total={30} thisPage={10} />
    );
    expect(container).toMatchInlineSnapshot(`
      <div>
        11 to 20 of 30
      </div>
    `);
  });

  test("Last page", () => {
    const { container } = render(
      <PageCount offset={20} total={25} thisPage={5} />
    );
    expect(container).toMatchInlineSnapshot(`
      <div>
        21 to 25 of 25
      </div>
    `);
  });
});
