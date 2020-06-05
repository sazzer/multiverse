import { Button, Input } from "../components/form";
import React, { useEffect, useState } from "react";
import { User, loadUser } from "./api";

import { Spinner } from "../components/spinner";
import { useForm } from "react-hook-form";
import { useUser } from "../currentUser";

interface ProfileFormProps {
  user: User;
}

const ProfileForm: React.FC<ProfileFormProps> = ({ user }) => {
  const { register, handleSubmit, errors } = useForm({
    defaultValues: {
      username: user.username,
      email_address: user.emailAddress,
      display_name: user.displayName,
    },
  });

  const onSubmitHandler = console.log;

  return (
    <form onSubmit={handleSubmit(onSubmitHandler)}>
      <fieldset>
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
          <Button label="profile.profile.submit" type="submit" />
          <Button
            label="profile.profile.cancel"
            type="reset"
            display="secondary"
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
