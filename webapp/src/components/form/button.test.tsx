import { fireEvent, render } from "@testing-library/react";

import { Button } from "./button";
import React from "react";

describe("Rendering a button", () => {
  test("Simple button", () => {
    const { container } = render(
      <Button label="authentication.login.submit" />
    );
    expect(container).toMatchInlineSnapshot(`
    <div>
      <button
        class="btn btn-primary"
        type="button"
      >
        Login
      </button>
    </div>
  `);
  });

  test("Secondary display format", () => {
    const { container } = render(
      <Button label="authentication.login.cancel" display="secondary" />
    );
    expect(container).toMatchInlineSnapshot(`
    <div>
      <button
        class="btn btn-secondary"
        type="button"
      >
        Cancel
      </button>
    </div>
  `);
  });

  test("Submit button", () => {
    const { container } = render(
      <Button label="authentication.login.submit" type="submit" />
    );
    expect(container).toMatchInlineSnapshot(`
    <div>
      <button
        class="btn btn-primary"
        type="submit"
      >
        Login
      </button>
    </div>
  `);
  });

  test("Loading button", () => {
    const { container } = render(
      <Button label="authentication.login.submit" loading />
    );
    expect(container).toMatchInlineSnapshot(`
    <div>
      <button
        class="btn btn-primary"
        type="button"
      >
        <span
          aria-hidden="true"
          class="spinner-border spinner-border-sm"
          role="status"
        />
         
        Login
      </button>
    </div>
  `);
  });
});

test("Clicking on a button triggers the callback", () => {
  const callback = jest.fn();
  const { container, getByText } = render(
    <Button label="authentication.login.submit" onClick={callback} />
  );

  expect(callback).not.toBeCalled();

  fireEvent.click(getByText("Login"));

  expect(callback).toBeCalledTimes(1);
});
