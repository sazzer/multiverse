import React, { useState } from "react";

import debug from "debug";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";

/** The logger to use */
const LOGGER = debug("multiverse:landing:authentication:login");

/**
 * Shape of the props needed to login
 */
export interface LoginProps {
  /** The username to log in as */
  username: string;
  /** Callback to cancel logging in */
  onCancel: () => void;
}

export const Login: React.FC<LoginProps> = ({ username, onCancel }) => {
  const { t } = useTranslation();
  const [error, setError] = useState();
  const [loading, setLoading] = useState(false);

  const { register, handleSubmit } = useForm({
    defaultValues: {
      username,
    },
  });
  const onSubmitHandler = (data: any) => {
    setError(undefined);
    setLoading(true);
    LOGGER("Submitting form: %o", data);
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
