import { Link } from "react-router-dom";
import React from "react";
import { useTranslation } from "react-i18next";

export default () => {
  const { t } = useTranslation();
  return (
    <nav className="navbar navbar-expand-lg navbar-dark bg-dark">
      <Link className="navbar-brand" to="/">
        {t("page.title")}
      </Link>
      <button
        className="navbar-toggler"
        type="button"
        data-toggle="collapse"
        data-target="#navbarSupportedContent"
        aria-controls="navbarSupportedContent"
        aria-expanded="false"
        aria-label={t("header.toggleNavigation")}
      >
        <span className="navbar-toggler-icon"></span>
      </button>

      <div className="collapse navbar-collapse" id="navbarSupportedContent">
        <ul className="navbar-nav ml-auto"></ul>
      </div>
    </nav>
  );
};
