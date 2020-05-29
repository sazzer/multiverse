import React from "react";
import { useForm } from "react-hook-form";
import { useTranslation } from "react-i18next";

export default () => {
  const { t } = useTranslation();
  const { register, handleSubmit, errors } = useForm();
  const onSubmit = (data: any) => {
    console.log(data);
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

      <button type="submit" className="btn btn-primary">
        {t("authentication.start.submit")}
      </button>
    </form>
  );
};
