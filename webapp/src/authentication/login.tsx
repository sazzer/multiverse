import { AuthenticationError, login } from "../api/authentication";
import { Button, Input } from "../components/form";
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
  /** Callback for when we successfully authenticated */
  onAuthenticated: (userId: string) => void;
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

export const Login: React.FC<LoginProps> = ({
  username,
  onCancel,
  onAuthenticated,
}) => {
  const { t } = useTranslation();
  const [error, setError] = useState<string>();
  const [loading, setLoading] = useState(false);

  const { register, handleSubmit, errors } = useForm({
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
      .then((userId) => {
        setLoading(false);
        if (userId) {
          onAuthenticated(userId);
        }
      });
  };

  return (
    <form
      onSubmit={handleSubmit(onSubmitHandler)}
      aria-labelledby="authenticationHeader"
    >
      <h2 id="authenticationHeader">{t("authentication.login.title")}</h2>

      <fieldset disabled={loading}>
        <Input
          id="username"
          i18n="authentication.username"
          required
          readOnly
          inputProps={{
            ref: register({ required: true, pattern: /[^\s]/ }),
          }}
        />
        <Input
          id="password"
          i18n="authentication.password"
          type="password"
          error={errors.password}
          required
          autoFocus
          inputProps={{
            ref: register({ required: true, pattern: /[^\s]/ }),
          }}
        />

        <div className="btn-group form-group">
          <Button
            label="authentication.login.submit"
            type="submit"
            loading={loading}
          />
          <Button
            label="authentication.login.cancel"
            display="secondary"
            onClick={onCancel}
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
