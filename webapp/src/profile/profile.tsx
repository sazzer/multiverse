import { Button, Input } from "../components/form";
import React, { useEffect, useState } from "react";

import { Spinner } from "../components/spinner";
import { request } from "../api";
import { useForm } from "react-hook-form";
import { useUser } from "../currentUser";

/**
 * The shape of the user returned by the API
 */
interface UserResponse {
  /** The users unique username */
  username: string;
  /** The users display name */
  display_name: string;
  /** The users email address */
  email_address: string;
  /** The avatar for the user */
  avatar_url?: string;
}

interface ProfileFormProps {
  user: UserResponse;
}

const ProfileForm: React.FC<ProfileFormProps> = ({ user }) => {
  const { register, handleSubmit, errors } = useForm({
    defaultValues: {
      username: user.username,
      email_address: user.email_address,
      display_name: user.display_name,
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
  const [user, setUser] = useState<UserResponse | null>(null);
  useEffect(() => {
    if (userId) {
      request<UserResponse>("/users/{userId}", {
        urlParams: {
          userId,
        },
        authenticated: true,
        ignoreCache: true,
      })
        .then((response) => response.body!!)
        .then((user) => {
          setUser(user);
        });
    }
  }, [userId]);

  return user == null ? <Spinner /> : <ProfileForm user={user} />;
};
