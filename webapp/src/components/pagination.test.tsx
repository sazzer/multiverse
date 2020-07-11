import { Pagination } from "./pagination";
import React from "react";
import { render } from "@testing-library/react";

describe("Rendering a paginator", () => {
  test("No pages", () => {
    const { container } = render(
      <Pagination current={0} total={0} onClick={() => {}} />
    );
    expect(container).toMatchSnapshot();
  });

  for (let total = 1; total < 10; ++total) {
    describe(`${total} pages`, () => {
      for (let current = 0; current < total; ++current) {
        test(`On page ${current + 1}`, () => {
          const { container } = render(
            <Pagination current={current} total={total} onClick={() => {}} />
          );
          expect(container).toMatchSnapshot();
        });
      }
    });
  }
});
