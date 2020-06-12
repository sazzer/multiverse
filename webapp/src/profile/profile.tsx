import { Button, Input } from "../components/form";
import React, { useEffect, useState } from "react";
import { User, loadUser, updateUser } from "../api/users";

import { Spinner } from "../components/spinner";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { useUser } from "../currentUser";

/**
 * The props needed forthe profile form
 */
interface ProfileFormProps {
  user: User;
}

interface ProfileForm {
  /** The username */
  username: string;
  /** The email address */
  email_address: string;
  /** The display name */
  display_name: string;
}

const ProfileForm: React.FC<ProfileFormProps> = ({ user }) => {
  const { t } = useTranslation();
  const [saving, setSaving] = useState(false);
  const { setUserId } = useUser();

  const { register, handleSubmit, errors } = useForm<ProfileForm>({
    defaultValues: {
      username: user.username,
      email_address: user.emailAddress,
      display_name: user.displayName,
    },
  });

  const onSubmitHandler = (data: ProfileForm) => {
    setSaving(true);
    updateUser({
      userId: user.userId,
      username: data.username,
      emailAddress: data.email_address,
      displayName: data.display_name,
    }).then(() => {
      setSaving(false);
      setUserId(user.userId);
    });
  };

  return (
    <form
      onSubmit={handleSubmit(onSubmitHandler)}
      aria-label={t("profile.profile.label")}
    >
      <fieldset disabled={saving}>
        <Input
          id="username"
          i18n="profile.profile.username"
          required
          readOnly
          inputProps={{
            ref: register({ required: true, pattern: /[^\s]/ }),
          }}
        />
        <Input
          id="email_address"
          i18n="profile.profile.email_address"
          error={errors.email_address}
          type="email"
          required
          autoFocus
          inputProps={{
            ref: register({ required: true, pattern: /[^\s]/ }),
          }}
        />
        <Input
          id="display_name"
          i18n="profile.profile.display_name"
          error={errors.display_name}
          required
          inputProps={{
            ref: register({ required: true, pattern: /[^\s]/ }),
          }}
        />

        <div className="btn-group form-group">
          <Button
            label="profile.profile.submit"
            type="submit"
            loading={saving}
          />
        </div>
      </fieldset>
    </form>
  );
};

export const ProfileView: React.FC = () => {
  const { userId } = useUser();
  const [user, setUser] = useState<User | null>(null);
  useEffect(() => {
    if (userId) {
      loadUser(userId, true).then(setUser);
    }
  }, [userId]);

  return user == null ? <Spinner /> : <ProfileForm user={user} />;
};
