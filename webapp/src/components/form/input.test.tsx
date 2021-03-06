import { Input } from "./input";
import React from "react";
import { render } from "@testing-library/react";

describe("Rendering an input", () => {
  test("Simple text input", () => {
    const { container } = render(
      <Input i18n="authentication.username" id="username" />
    );
    expect(container).toMatchInlineSnapshot(`
      <div>
        <label
          for="username"
        >
          Username
          <span
            aria-hidden="true"
            class=""
          />
        </label>
        <input
          aria-describedby="username-error "
          aria-invalid="false"
          aria-required="false"
          class="form-control "
          id="username"
          name="username"
          type="text"
        />
      </div>
    `);
  });

  test("Input with a different ID and Name", () => {
    const { container } = render(
      <Input i18n="authentication.username" id="name" name="username" />
    );
    expect(container).toMatchInlineSnapshot(`
      <div>
        <label
          for="name"
        >
          Username
          <span
            aria-hidden="true"
            class=""
          />
        </label>
        <input
          aria-describedby="name-error "
          aria-invalid="false"
          aria-required="false"
          class="form-control "
          id="name"
          name="username"
          type="text"
        />
      </div>
    `);
  });

  test("Email Input", () => {
    const { container } = render(
      <Input i18n="authentication.username" id="username" type="email" />
    );
    expect(container).toMatchInlineSnapshot(`
      <div>
        <label
          for="username"
        >
          Username
          <span
            aria-hidden="true"
            class=""
          />
        </label>
        <input
          aria-describedby="username-error "
          aria-invalid="false"
          aria-required="false"
          class="form-control "
          id="username"
          name="username"
          type="email"
        />
      </div>
    `);
  });

  test("Readonly Input", () => {
    const { container } = render(
      <Input i18n="authentication.username" id="username" readOnly />
    );
    expect(container).toMatchInlineSnapshot(`
      <div>
        <label
          for="username"
        >
          Username
          <span
            aria-hidden="true"
            class=""
          />
        </label>
        <input
          aria-describedby="username-error "
          aria-invalid="false"
          aria-required="false"
          class="form-control "
          id="username"
          name="username"
          readonly=""
          type="text"
        />
      </div>
    `);
  });

  test("Autofocus Input", () => {
    const { container } = render(
      <Input i18n="authentication.username" id="username" autoFocus />
    );
    expect(container).toMatchInlineSnapshot(`
      <div>
        <label
          for="username"
        >
          Username
          <span
            aria-hidden="true"
            class=""
          />
        </label>
        <input
          aria-describedby="username-error "
          aria-invalid="false"
          aria-required="false"
          class="form-control "
          id="username"
          name="username"
          type="text"
        />
      </div>
    `);
  });

  test("Required Input", () => {
    const { container } = render(
      <Input i18n="authentication.username" id="username" required />
    );
    expect(container).toMatchInlineSnapshot(`
      <div>
        <label
          for="username"
        >
          Username
          <span
            aria-hidden="true"
            class="multiverse-required"
          />
        </label>
        <input
          aria-describedby="username-error "
          aria-invalid="false"
          aria-required="true"
          class="form-control "
          id="username"
          name="username"
          required=""
          type="text"
        />
      </div>
    `);
  });

  test("Erroring Input", () => {
    const { container } = render(
      <Input
        i18n="authentication.username"
        id="username"
        error={{ type: "required" }}
      />
    );
    expect(container).toMatchInlineSnapshot(`
      <div>
        <label
          for="username"
        >
          Username
          <span
            aria-hidden="true"
            class=""
          />
        </label>
        <input
          aria-describedby="username-error "
          aria-invalid="true"
          aria-required="false"
          class="form-control is-invalid"
          id="username"
          name="username"
          type="text"
        />
        <div
          class="invalid-feedback"
          id="username-error"
          role="alert"
        >
          Please enter a username
        </div>
      </div>
    `);
  });

  test("Additional Props", () => {
    const { container } = render(
      <Input
        i18n="authentication.username"
        id="username"
        inputProps={{ "data-answer": 42 }}
      />
    );
    expect(container).toMatchInlineSnapshot(`
      <div>
        <label
          for="username"
        >
          Username
          <span
            aria-hidden="true"
            class=""
          />
        </label>
        <input
          aria-describedby="username-error "
          aria-invalid="false"
          aria-required="false"
          class="form-control "
          data-answer="42"
          id="username"
          name="username"
          type="text"
        />
      </div>
    `);
  });
});
