import { Button, Input } from "../../components/form";
import React, { useReducer } from "react";

import { Spinner } from "../../components/spinner";
import slugify from "slugify";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";
import { useUser } from "../../currentUser";

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
 * Reducer to convert the current state into the new one
 * @param state The current state
 * @param action The action to process
 */
function reducer(state: NewWorldState, action: any): NewWorldState {
  switch (action.action) {
    default:
      return state;
  }
}

const NewWorldForm: React.FC = () => {
  const { t } = useTranslation();
  const [state, dispatch] = useReducer(reducer, { state: "INITIAL" });

  const { register, handleSubmit, errors, watch } = useForm<NewWorldForm>({
    defaultValues: {},
  });
  const watchName = watch("name") || "";
  const defaultSlug = slugify(watchName, { lower: true });
  const hasDefaultSlug = defaultSlug.trim().length > 0;

  const onSubmitHandler = (data: NewWorldForm) => {
    dispatch({ action: "SAVING" });
    console.log(data);
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
          <Input
            id="description"
            i18n="profile.worlds.new.description"
            type="text"
            error={errors.description}
            inputProps={{
              ref: register({ required: !hasDefaultSlug }),
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
              ref: register({}),
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
  const { userLink } = useUser();

  return userLink == null ? <Spinner /> : <NewWorldForm />;
};
