import { Button, Input } from "../components/form";
import React, { useEffect, useReducer, useState } from "react";
import { User, loadUser, updateUser } from "../api/users";

import { Spinner } from "../components/spinner";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { useUser } from "../currentUser";

/**
 * The props needed for the profile form
 */
interface ProfileFormProps {
  user: User;
}

/**
 * The shape of the data for the profile form
 */
interface ProfileForm {
  /** The username */
  username: string;
  /** The email address */
  email_address: string;
  /** The display name */
  display_name: string;
}

/**
 * The state of the profile component
 */
interface ProfileState {
  state: "INITIAL" | "SAVING" | "SAVED" | "ERROR";
  error?: string;
}

/**
 * Action to indicate we are going to start saving the form
 */
interface SavingAction {
  /** The name of the action */
  action: "SAVING";
}

/**
 * Action to indicate that we successfully saved the form
 */
interface SavedAction {
  /** The name of the action */
  action: "SAVED";
}

/**
 * Action to indicate that we unsuccessfully saved the form
 */
interface ErrorAction {
  /** The name of the action */
  action: "ERROR";
  /** The error message */
  message: string;
}

/**
 * Reducer to convert the current state into the new one
 * @param state The current state
 * @param action The action to process
 */
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
  const { reloadUser } = useUser();

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
      selfLink: user.selfLink,
      username: data.username,
      emailAddress: data.email_address,
      displayName: data.display_name,
    })
      .then(() => {
        dispatch({ action: "SAVED" });
        reloadUser();
      })
      .catch(() => {
        dispatch({ action: "ERROR", message: t("page.errors.unexpected") });
      });
  };

  return (
    <form
      onSubmit={handleSubmit(onSubmitHandler)}
      aria-labelledby="profileFormLabel"
    >
      <fieldset disabled={state.state === "SAVING"}>
        <h3 id="profileFormLabel">{t("profile.profile.label")}</h3>
        <div className="form-group">
          <Input
            id="username"
            i18n="profile.profile.username"
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
            i18n="profile.profile.email_address"
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
            i18n="profile.profile.display_name"
            error={errors.display_name}
            required
            inputProps={{
              ref: register({ required: true, pattern: /[^\s]/ }),
            }}
          />
        </div>
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
          <div className="alert alert-success" role="status">
            {t("profile.profile.success")}
          </div>
        )}
      </fieldset>
    </form>
  );
};

export const ProfileView: React.FC = () => {
  const { userLink } = useUser();
  const [user, setUser] = useState<User | null>(null);
  useEffect(() => {
    if (userLink) {
      loadUser(userLink, true).then(setUser);
    }
  }, [userLink]);

  return user == null ? <Spinner /> : <ProfileForm user={user} />;
};
