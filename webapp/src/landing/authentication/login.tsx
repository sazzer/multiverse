import { AuthenticationError, login } from "./api";
import React, { useState } from "react";

import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";

/**
 * Shape of the props needed to login
 */
export interface LoginProps {
  /** The username to log in as */
  username: string;
  /** Callback to cancel logging in */
  onCancel: () => void;
}

/**
 * Shape of the form data for logging in
 */
interface LoginForm {
  /** The username */
  username: string;
  /** The password  */
  password: string;
}

export const Login: React.FC<LoginProps> = ({ username, onCancel }) => {
  const { t } = useTranslation();
  const [error, setError] = useState<string>();
  const [loading, setLoading] = useState(false);

  const { register, handleSubmit } = useForm({
    defaultValues: {
      username,
      password: "",
    },
  });
  const onSubmitHandler = (form: LoginForm) => {
    setError(undefined);
    setLoading(true);

    login(form.username, form.password)
      .catch((e) => {
        if (e instanceof AuthenticationError) {
          setError(t("authentication.errors.invalidPassword"));
        } else {
          setError(e.toString());
        }
      })
      .then(() => {
        setLoading(false);
      });
  };

  return (
    <form onSubmit={handleSubmit(onSubmitHandler)}>
      <h2>{t("authentication.login.title")}</h2>

      <fieldset disabled={loading}>
        <div className="form-group">
          <label htmlFor="username">{t("authentication.username.label")}</label>
          <input
            type="text"
            className="form-control"
            id="username"
            name="username"
            ref={register({ required: true })}
            required
            readOnly
          />
        </div>

        <div className="form-group">
          <label htmlFor="password">{t("authentication.password.label")}</label>
          <input
            type="password"
            className="form-control"
            id="password"
            name="password"
            ref={register({ required: true })}
            required
            autoFocus
          />
        </div>

        <div className="btn-group form-group">
          <button type="submit" className="btn btn-primary">
            {loading && (
              <span
                className="spinner-border spinner-border-sm"
                role="status"
                aria-hidden="true"
              ></span>
            )}
            {t("authentication.login.submit")}
          </button>

          <button
            type="button"
            className="btn btn-secondary"
            onClick={onCancel}
          >
            {t("authentication.login.cancel")}
          </button>
        </div>
      </fieldset>

      {error && (
        <div className="alert alert-danger" role="alert">
          {error}
        </div>
      )}
    </form>
  );
};
