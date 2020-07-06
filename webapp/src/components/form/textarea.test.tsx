import React from "react";
import { Textarea } from "./textarea";
import { render } from "@testing-library/react";

describe("Rendering an textarea", () => {
  test("Simple text textarea", () => {
    const { container } = render(
      <Textarea i18n="authentication.username" id="username" />
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
        <textarea
          aria-describedby="username-error "
          aria-invalid="false"
          aria-required="false"
          class="form-control "
          id="username"
          name="username"
          rows="5"
        />
      </div>
    `);
  });

  test("Textarea with a different ID and Name", () => {
    const { container } = render(
      <Textarea i18n="authentication.username" id="name" name="username" />
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
        <textarea
          aria-describedby="name-error "
          aria-invalid="false"
          aria-required="false"
          class="form-control "
          id="name"
          name="username"
          rows="5"
        />
      </div>
    `);
  });

  test("Textarea specifying number of rows", () => {
    const { container } = render(
      <Textarea i18n="authentication.username" id="username" rows={10} />
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
        <textarea
          aria-describedby="username-error "
          aria-invalid="false"
          aria-required="false"
          class="form-control "
          id="username"
          name="username"
          rows="10"
        />
      </div>
    `);
  });

  test("Readonly Textarea", () => {
    const { container } = render(
      <Textarea i18n="authentication.username" id="username" readOnly />
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
        <textarea
          aria-describedby="username-error "
          aria-invalid="false"
          aria-required="false"
          class="form-control "
          id="username"
          name="username"
          readonly=""
          rows="5"
        />
      </div>
    `);
  });

  test("Autofocus Textarea", () => {
    const { container } = render(
      <Textarea i18n="authentication.username" id="username" autoFocus />
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
        <textarea
          aria-describedby="username-error "
          aria-invalid="false"
          aria-required="false"
          class="form-control "
          id="username"
          name="username"
          rows="5"
        />
      </div>
    `);
  });

  test("Required Textarea", () => {
    const { container } = render(
      <Textarea i18n="authentication.username" id="username" required />
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
        <textarea
          aria-describedby="username-error "
          aria-invalid="false"
          aria-required="true"
          class="form-control "
          id="username"
          name="username"
          required=""
          rows="5"
        />
      </div>
    `);
  });

  test("Erroring Textarea", () => {
    const { container } = render(
      <Textarea
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
        <textarea
          aria-describedby="username-error "
          aria-invalid="true"
          aria-required="false"
          class="form-control is-invalid"
          id="username"
          name="username"
          rows="5"
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
      <Textarea
        i18n="authentication.username"
        id="username"
        textareaProps={{ "data-answer": 42 }}
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
        <textarea
          aria-describedby="username-error "
          aria-invalid="false"
          aria-required="false"
          class="form-control "
          data-answer="42"
          id="username"
          name="username"
          rows="5"
        />
      </div>
    `);
  });
});
