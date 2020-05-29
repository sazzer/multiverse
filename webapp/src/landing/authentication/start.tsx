import { Problem, request } from "../../api";
import React, { useState } from "react";

import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";

export default () => {
  const { t } = useTranslation();
  const [error, setError] = useState();

  const { register, handleSubmit } = useForm({
    defaultValues: {
      username: "",
    },
  });
  const onSubmit = ({ username }: { username: string }) => {
    setError(null);
    request("/usernames/{username}", {
      urlParams: {
        username,
      },
    })
      .then((response) => {
        console.log("Username exists", response);
      })
      .catch((e) => {
        if (
          e instanceof Problem &&
          e.type === "tag:multiverse,2020:users/problems/unknown_username"
        ) {
          console.log("Username doesn't exist");
        } else {
          console.log("Something went wrong", e);
          setError(e.toString());
        }
      });
  };

  return (
    <form onSubmit={handleSubmit(onSubmit)}>
      <h2>{t("authentication.start.title")}</h2>

      <div className="form-group">
        <label htmlFor="username">{t("authentication.username.label")}</label>
        <input
          type="text"
          className="form-control"
          id="username"
          name="username"
          ref={register({ required: true })}
          required
          autoFocus
        />
      </div>

      <div className="form-group">
        <button type="submit" className="btn btn-primary">
          {t("authentication.start.submit")}
        </button>
      </div>

      {error && (
        <div className="alert alert-danger" role="alert">
          {error}
        </div>
      )}
    </form>
  );
};
