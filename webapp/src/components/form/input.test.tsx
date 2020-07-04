import { fireEvent, render } from "@testing-library/react";

import { Input } from "./input";
import React from "react";

describe("Rendering an input", () => {
  test("Simple text input", () => {
    const { container } = render(
      <Input i18n="authentication.username" id="username" />
    );
    expect(container).toMatchInlineSnapshot(`
      <div>
        <div
          class="form-group"
        >
          <label
            for="username"
          >
            Username
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
      </div>
    `);
  });

  test("Input with a different ID and Name", () => {
    const { container } = render(
      <Input i18n="authentication.username" id="name" name="username" />
    );
    expect(container).toMatchInlineSnapshot(`
      <div>
        <div
          class="form-group"
        >
          <label
            for="name"
          >
            Username
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
      </div>
    `);
  });

  test("Email Input", () => {
    const { container } = render(
      <Input i18n="authentication.username" id="username" type="email" />
    );
    expect(container).toMatchInlineSnapshot(`
      <div>
        <div
          class="form-group"
        >
          <label
            for="username"
          >
            Username
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
      </div>
    `);
  });

  test("Readonly Input", () => {
    const { container } = render(
      <Input i18n="authentication.username" id="username" readOnly />
    );
    expect(container).toMatchInlineSnapshot(`
      <div>
        <div
          class="form-group"
        >
          <label
            for="username"
          >
            Username
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
      </div>
    `);
  });

  test("Autofocus Input", () => {
    const { container } = render(
      <Input i18n="authentication.username" id="username" autoFocus />
    );
    expect(container).toMatchInlineSnapshot(`
      <div>
        <div
          class="form-group"
        >
          <label
            for="username"
          >
            Username
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
      </div>
    `);
  });

  test("Required Input", () => {
    const { container } = render(
      <Input i18n="authentication.username" id="username" required />
    );
    expect(container).toMatchInlineSnapshot(`
      <div>
        <div
          class="form-group"
        >
          <label
            for="username"
          >
            Username
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
        <div
          class="form-group"
        >
          <label
            for="username"
          >
            Username
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
        <div
          class="form-group"
        >
          <label
            for="username"
          >
            Username
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
      </div>
    `);
  });
});
