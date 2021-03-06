import { Button, Input } from "../components/form";
import { DuplicateUsernameError, registerUser } from "../api/authentication";
import React, { useState } from "react";

import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";

/**
 * Shape of the props needed to register
 */
export interface RegisterProps {
  /** The username to register as */
  username: string;
  /** Callback to cancel registering */
  onCancel: () => void;
  /** Callback for when we successfully authenticated */
  onAuthenticated: (userLink: string) => void;
}

/**
 * Shape of the form data for registering
 */
interface RegisterForm {
  /** The username */
  username: string;
  /** The display name */
  display_name?: string;
  /** The email address */
  email_address: string;
  /** The password  */
  password: string;
  /** The repeated password */
  password2: string;
}

export const Register: React.FC<RegisterProps> = ({
  username,
  onCancel,
  onAuthenticated,
}) => {
  const { t } = useTranslation();
  const [error, setGlobalError] = useState<string>();
  const [loading, setLoading] = useState(false);

  const { register, handleSubmit, setError, errors } = useForm({
    defaultValues: {
      username,
      email_address: "",
      display_name: "",
      password: "",
      password2: "",
    },
  });
  const onSubmitHandler = (form: RegisterForm) => {
    setGlobalError(undefined);
    if (form.password !== form.password2) {
      setError("password2", "noMatch");
      return;
    }
    setLoading(true);

    registerUser(
      form.username,
      form.password,
      form.email_address,
      form.display_name
    )
      .catch((e) => {
        if (e instanceof DuplicateUsernameError) {
          setGlobalError(t("authentication.errors.duplicateUsername"));
        } else {
          setGlobalError(t("page.errors.unexpected"));
        }
      })
      .then((userLink) => {
        setLoading(false);
        if (userLink) {
          onAuthenticated(userLink);
        }
      });
  };

  return (
    <form
      onSubmit={handleSubmit(onSubmitHandler)}
      aria-labelledby="authenticationHeader"
    >
      <h2 id="authenticationHeader">{t("authentication.register.title")}</h2>

      <fieldset disabled={loading}>
        <div className="form-group">
          <Input
            id="username"
            i18n="authentication.username"
            required
            readOnly
            inputProps={{
              ref: register({ required: true, pattern: /[^\s]/ }),
            }}
          />
        </div>
        <div className="form-group">
          <Input
            id="email_address"
            i18n="authentication.email_address"
            error={errors.email_address}
            type="email"
            required
            autoFocus
            inputProps={{
              ref: register({ required: true, pattern: /[^\s]/ }),
            }}
          />
        </div>
        <div className="form-group">
          <Input
            id="display_name"
            i18n="authentication.display_name"
            error={errors.display_name}
            inputProps={{
              ref: register(),
            }}
          />
        </div>
        <div className="form-group">
          <Input
            id="password"
            i18n="authentication.password"
            type="password"
            error={errors.password}
            required
            inputProps={{
              ref: register({ required: true, pattern: /[^\s]/ }),
            }}
          />
        </div>
        <div className="form-group">
          <Input
            id="password2"
            i18n="authentication.password2"
            type="password"
            error={errors.password2}
            required
            inputProps={{
              ref: register({ required: true }),
            }}
          />
        </div>
        <div className="btn-group form-group">
          <Button
            label="authentication.register.submit"
            type="submit"
            loading={loading}
          />
          <Button
            label="authentication.register.cancel"
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
