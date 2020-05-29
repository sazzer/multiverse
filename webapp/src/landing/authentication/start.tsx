import React from "react";
import { useTranslation } from "react-i18next";

export default () => {
  const { t } = useTranslation();

  return (
    <form>
      <h2>{t("authentication.start.title")}</h2>

      <div className="form-group">
        <label htmlFor="username">{t("authentication.username.label")}</label>
        <input type="text" className="form-control" id="username" autoFocus />
      </div>

      <button type="submit" className="btn btn-primary">
        {t("authentication.start.submit")}
      </button>
    </form>
  );
};
