import React from "react";
import { useTranslation } from "react-i18next";

export const Spinner = () => {
  const { t } = useTranslation();

  return (
    <>
      <div className="d-flex justify-content-center">
        <div
          className="spinner-border text-info"
          role="status"
          style={{ width: "3rem", height: "3rem" }}
        ></div>
      </div>
      <div className="d-flex justify-content-center">
        <span>{t("page.spinner.label")}</span>
      </div>
    </>
  );
};
