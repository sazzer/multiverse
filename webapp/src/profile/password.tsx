import { Button, Input } from "../components/form";
import { InvalidOldPasswordError, changePassword } from "../api/users";
import React, { useReducer } from "react";

import { Spinner } from "../components/spinner";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { useUser } from "../currentUser";

/**
 * The props needed for the Change Password form
 */
interface PasswordFormProps {
  userLink: string;
}

/**
 * The shape of the data for the Change Password form
 */
interface PasswordForm {
  oldPassword: string;
  password: string;
  password2: string;
}

/**
 * The state of the Change Password component
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

/**
 * Action to indicate that we unsuccessfully saved the form
 */
interface ErrorAction {
  /** The name of the action */
  action: "ERROR";
  /** The error message */
  message: string;
}

const PasswordForm: React.FC<PasswordFormProps> = ({ userLink }) => {
  const { t } = useTranslation();
  const [state, dispatch] = useReducer(reducer, { state: "INITIAL" });

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
    dispatch({ action: "SAVING" });

    changePassword(userLink, data.oldPassword, data.password)
      .then(() => {
        dispatch({ action: "SAVED" });
      })
      .catch((e) => {
        if (e instanceof InvalidOldPasswordError) {
          dispatch({
            action: "ERROR",
            message: t("profile.password.errors.invalidPassword"),
          });
        } else {
          dispatch({
            action: "ERROR",
            message: t("page.errors.unexpected"),
          });
        }
      });
  };

  return (
    <form
      onSubmit={handleSubmit(onSubmitHandler)}
      aria-labelledby="passwordFormLabel"
    >
      <fieldset disabled={state.state === "SAVING"}>
        <h3 id="passwordFormLabel">{t("profile.password.label")}</h3>
        <div className="form-group">
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
        </div>
        <div className="form-group">
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
        </div>
        <div className="form-group">
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
        </div>

        <div className="btn-group form-group">
          <Button
            label="profile.password.submit"
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
            {t("profile.password.success")}
          </div>
        )}
      </fieldset>
    </form>
  );
};

export const PasswordView: React.FC = () => {
  const { userLink } = useUser();

  return userLink == null ? <Spinner /> : <PasswordForm userLink={userLink} />;
};
