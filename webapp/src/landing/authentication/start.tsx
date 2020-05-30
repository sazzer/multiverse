import { Button, Input } from "../../components/form";
import React, { useState } from "react";

import { lookupUsername } from "./api";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";

/**
 * Shape of the props needed to start authentication
 */
export interface StartAuthProps {
  /** Callback with the username that was entered */
  onSubmit: (username: string, known: boolean) => void;
}

/**
 * Shape of the form data for starting authentication
 */
interface StartAuthForm {
  /** The username */
  username: string;
}

export const StartAuthentication: React.FC<StartAuthProps> = ({ onSubmit }) => {
  const { t } = useTranslation();
  const [error, setError] = useState();
  const [loading, setLoading] = useState(false);

  const { register, handleSubmit, errors } = useForm({
    defaultValues: {
      username: "",
    },
  });

  const onSubmitHandler = (form: StartAuthForm) => {
    setLoading(true);
    setError(undefined);

    lookupUsername(form.username)
      .then((known: boolean) => {
        setLoading(false);
        onSubmit(form.username, known);
      })
      .catch((e) => {
        setError(e.toString());
        setLoading(false);
      });
  };

  return (
    <form onSubmit={handleSubmit(onSubmitHandler)}>
      <h2>{t("authentication.start.title")}</h2>

      <fieldset disabled={loading}>
        <Input
          id="username"
          i18n="authentication.username"
          error={errors.username}
          required
          autoFocus
          inputProps={{
            ref: register({ required: true }),
          }}
        />

        <div className="btn-group form-group">
          <Button
            label="authentication.start.submit"
            type="submit"
            loading={loading}
          />
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
