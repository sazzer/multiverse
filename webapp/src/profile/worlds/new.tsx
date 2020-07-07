import { Button, Input, Textarea } from "../../components/form";
import { DuplicateUrlSlugError, createWorld } from "../../api/worlds";
import React, { useReducer } from "react";

import { Spinner } from "../../components/spinner";
import UrlTemplate from "url-template";
import slugify from "slugify";
import { useForm } from "react-hook-form";
import { useHistory } from "react-router-dom";
import { useTranslation } from "react-i18next";
import { useUser } from "../../currentUser";

/** The URL Template to use to redirect the user after creating a world */
const REDIRECT_TEMPLATE = UrlTemplate.parse("/u/{username}/w/{slug}");

/**
 * The shape of the data for the New World form
 */
interface NewWorldForm {
  name: string;
  description: string;
  slug?: string;
}

/**
 * The state of the New World component
 */
interface NewWorldState {
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
  state: NewWorldState,
  action: SavingAction | SavedAction | ErrorAction
): NewWorldState {
  switch (action.action) {
    case "SAVING":
      return {
        state: "SAVING",
        error: undefined,
      };
    case "SAVED":
      return {
        state: "SAVED",
        error: undefined,
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

interface NewWorldFormProps {
  username: string;
}

const NewWorldForm: React.FC<NewWorldFormProps> = ({ username }) => {
  const { t } = useTranslation();
  const [state, dispatch] = useReducer(reducer, { state: "INITIAL" });
  const history = useHistory();

  const { register, handleSubmit, errors, watch, setError } = useForm<
    NewWorldForm
  >({
    defaultValues: {},
  });
  const watchName = watch("name") || "";
  const defaultSlug = slugify(watchName, { lower: true });
  const hasDefaultSlug = defaultSlug.trim().length > 0;

  const onSubmitHandler = (data: NewWorldForm) => {
    dispatch({ action: "SAVING" });
    const slug = data.slug || defaultSlug;
    createWorld({
      name: data.name,
      description: data.description,
      slug,
    })
      .then(() => {
        dispatch({ action: "SAVED" });
        history.push(
          REDIRECT_TEMPLATE.expand({
            username,
            slug,
          })
        );
      })
      .catch((e) => {
        if (e instanceof DuplicateUrlSlugError) {
          dispatch({
            action: "ERROR",
            message: t("profile.worlds.new.errors.failed"),
          });
          setError("slug", "duplicateUrlSlug");
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
      aria-labelledby="newWorldFormLabel"
    >
      <fieldset disabled={state.state === "SAVING"}>
        <h3 id="newWorldFormLabel">{t("profile.worlds.new.label")}</h3>
        <div className="form-group">
          <Input
            id="name"
            i18n="profile.worlds.new.name"
            type="text"
            error={errors.name}
            required
            autoFocus
            inputProps={{
              ref: register({ required: true, pattern: /[^\s]/ }),
            }}
          />
        </div>

        <div className="form-group">
          <Textarea
            id="description"
            i18n="profile.worlds.new.description"
            error={errors.description}
            textareaProps={{
              ref: register({}),
            }}
          />
        </div>

        <div className="form-group">
          <Input
            id="slug"
            i18n="profile.worlds.new.slug"
            type="text"
            error={errors.slug}
            required={!hasDefaultSlug}
            inputProps={{
              ref: register({ required: !hasDefaultSlug }),
            }}
            describedBy="slugDefault"
          />
          <small id="slugDefault" className="form-text text-muted">
            {hasDefaultSlug &&
              t("profile.worlds.new.slug.description", {
                slug: defaultSlug,
              })}
          </small>
        </div>

        <div className="btn-group form-group">
          <Button
            label="profile.worlds.new.submit"
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
            {t("profile.worlds.new.success")}
          </div>
        )}
      </fieldset>
    </form>
  );
};

export const NewWorldView: React.FC = () => {
  const { user } = useUser();

  return user == null ? <Spinner /> : <NewWorldForm username={user.username} />;
};
