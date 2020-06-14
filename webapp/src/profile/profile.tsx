import { Button, Input } from "../components/form";
import React, { useEffect, useReducer, useState } from "react";
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

interface ProfileState {
  state: "INITIAL" | "SAVING" | "SAVED" | "ERROR";
  error?: string;
}

interface SavingAction {
  action: "SAVING";
}

interface SavedAction {
  action: "SAVED";
}

interface ErrorAction {
  action: "ERROR";
  message: string;
}

function reducer(
  state: ProfileState,
  action: SavingAction | SavedAction | ErrorAction
): ProfileState {
  switch (action.action) {
    case "SAVING":
      return {
        state: "SAVING",
      };
    case "SAVED":
      return {
        state: "SAVED",
      };
    case "ERROR":
      return {
        state: "ERROR",
        error: action.message,
      };
    default:
      return state;
  }
}

const ProfileForm: React.FC<ProfileFormProps> = ({ user }) => {
  const { t } = useTranslation();

  const [state, dispatch] = useReducer(reducer, { state: "INITIAL" });
  const { setUserId } = useUser();

  const { register, handleSubmit, errors } = useForm<ProfileForm>({
    defaultValues: {
      username: user.username,
      email_address: user.emailAddress,
      display_name: user.displayName,
    },
  });

  const onSubmitHandler = (data: ProfileForm) => {
    dispatch({ action: "SAVING" });
    updateUser({
      userId: user.userId,
      username: data.username,
      emailAddress: data.email_address,
      displayName: data.display_name,
    })
      .then(() => {
        dispatch({ action: "SAVED" });
        setUserId(user.userId);
      })
      .catch(() => {
        dispatch({ action: "ERROR", message: t("page.errors.unexpected") });
      });
  };

  return (
    <form
      onSubmit={handleSubmit(onSubmitHandler)}
      aria-label={t("profile.profile.label")}
    >
      <fieldset disabled={state.state === "SAVING"}>
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
            loading={state.state === "SAVING"}
          />
        </div>
        {state.state === "ERROR" && (
          <div className="alert alert-danger" role="alert">
            {state.error}
          </div>
        )}
        {state.state === "SAVED" && (
          <div className="alert alert-success" role="alert">
            {t("profile.profile.success")}
          </div>
        )}
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
