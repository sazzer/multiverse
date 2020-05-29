import { Problem, request } from "../../api";
import React, { useState } from "react";

import debug from "debug";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";

/** The logger to use */
const LOGGER = debug("multiverse:landing:authentication:start");

/**
 * Shape of the props needed to start authentication
 */
export interface StartAuthProps {
  /** Callback with the username that was entered */
  onSubmit: (username: string, known: boolean) => void;
}

export const StartAuthentication: React.FC<StartAuthProps> = ({ onSubmit }) => {
  const { t } = useTranslation();
  const [error, setError] = useState();
  const [loading, setLoading] = useState(false);

  const { register, handleSubmit } = useForm({
    defaultValues: {
      username: "",
    },
  });
  const onSubmitHandler = ({ username }: { username: string }) => {
    setError(undefined);
    setLoading(true);
    request("/usernames/{username}", {
      urlParams: {
        username,
      },
    })
      .then((response) => {
        LOGGER("Username exists: %o", response);
        onSubmit(username, true);
      })
      .catch((e) => {
        if (
          e instanceof Problem &&
          e.type === "tag:multiverse,2020:users/problems/unknown_username"
        ) {
          LOGGER("Username doesn't exist");
          onSubmit(username, false);
        } else {
          LOGGER("Something went wrong: %o", e);
          setError(e.toString());
        }
      })
      .then(() => setLoading(false));
  };

  return (
    <form onSubmit={handleSubmit(onSubmitHandler)}>
      <h2>{t("authentication.start.title")}</h2>

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
            autoFocus
          />
        </div>

        <div className="form-group">
          <button type="submit" className="btn btn-primary">
            {loading && (
              <span
                className="spinner-border spinner-border-sm"
                role="status"
                aria-hidden="true"
              ></span>
            )}
            {t("authentication.start.submit")}
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
