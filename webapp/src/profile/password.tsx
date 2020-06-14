import { Button, Input } from "../components/form";
import { InvalidOldPasswordError, changePassword } from "../api/users";
import React, { useState } from "react";

import { Spinner } from "../components/spinner";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { useUser } from "../currentUser";

/**
 * The props needed for the Change Password form
 */
interface PasswordFormProps {
  userId: string;
}

interface PasswordForm {
  oldPassword: string;
  password: string;
  password2: string;
}

const PasswordForm: React.FC<PasswordFormProps> = ({ userId }) => {
  const { t } = useTranslation();
  const [saving, setSaving] = useState(false);
  const [globalError, setGlobalError] = useState<string | null>(null);

  const { register, handleSubmit, errors, setError } = useForm<PasswordForm>({
    defaultValues: {},
  });

  const onSubmitHandler = (data: PasswordForm) => {
    if (data.oldPassword === data.password) {
      setError("password", "unchanged");
      return;
    }
    if (data.password !== data.password2) {
      setError("password2", "noMatch");
      return;
    }
    setSaving(true);
    setGlobalError(null);

    changePassword(userId, data.oldPassword, data.password)
      .then(() => {
        setSaving(false);
      })
      .catch((e) => {
        if (e instanceof InvalidOldPasswordError) {
          setGlobalError(t("profile.password.errors.invalidPassword"));
        } else {
          setGlobalError(t("page.errors.unexpected"));
        }
        setSaving(false);
      });
  };

  return (
    <form
      onSubmit={handleSubmit(onSubmitHandler)}
      aria-label={t("profile.password.label")}
    >
      <fieldset disabled={saving}>
        <Input
          id="oldPassword"
          i18n="profile.password.oldPassword"
          type="password"
          error={errors.oldPassword}
          required
          autoFocus
          inputProps={{
            ref: register({ required: true, pattern: /[^\s]/ }),
          }}
        />
        <Input
          id="password"
          i18n="profile.password.password"
          type="password"
          error={errors.password}
          required
          inputProps={{
            ref: register({ required: true, pattern: /[^\s]/ }),
          }}
        />
        <Input
          id="password2"
          i18n="profile.password.password2"
          type="password"
          error={errors.password2}
          required
          inputProps={{
            ref: register({ required: true, pattern: /[^\s]/ }),
          }}
        />

        <div className="btn-group form-group">
          <Button
            label="profile.password.submit"
            type="submit"
            loading={saving}
          />
        </div>
        {globalError && (
          <div className="alert alert-danger" role="alert">
            {globalError}
          </div>
        )}
      </fieldset>
    </form>
  );
};

export const PasswordView: React.FC = () => {
  const { userId } = useUser();

  return userId == null ? <Spinner /> : <PasswordForm userId={userId} />;
};
