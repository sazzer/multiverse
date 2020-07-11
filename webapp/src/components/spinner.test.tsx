import React from "react";
import { Spinner } from "./spinner";
import { render } from "@testing-library/react";

test("Rendering a spinner", () => {
  const { container } = render(<Spinner />);
  expect(container).toMatchInlineSnapshot(`
    <div>
      <div
        class="d-flex justify-content-center"
      >
        <div
          class="spinner-border text-info"
          role="status"
          style="width: 3rem; height: 3rem;"
        />
      </div>
      <div
        class="d-flex justify-content-center"
      >
        <span>
          Loading...
        </span>
      </div>
    </div>
  `);
});
