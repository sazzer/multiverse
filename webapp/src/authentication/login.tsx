import { AuthenticationError, login } from "./api";
import { Button, Input } from "../components/form";
import React, { useState } from "react";

import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { useUser } from "../currentUser";

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
  const { setUserId } = useUser();

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
          setUserId(userId);
        }
      });
  };

  return (
    <form onSubmit={handleSubmit(onSubmitHandler)}>
      <h2>{t("authentication.login.title")}</h2>

      <fieldset disabled={loading}>
        <Input
          id="username"
          i18n="authentication.username"
          required
          readOnly
          inputProps={{
            ref: register({ required: true }),
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
            ref: register({ required: true }),
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
