import { Link, useHistory } from "react-router-dom";

import React from "react";
import { clearToken } from "../api/token";
import { useTranslation } from "react-i18next";
import { useUser } from "../currentUser";

/**
 * The user menu as displayed in the header bar
 */
export const UserMenu = () => {
  const { t } = useTranslation();
  const { user, clearUserId } = useUser();
  const history = useHistory();

  const onLogout = () => {
    clearUserId();
    clearToken();
    history.push("/");
  };

  if (user) {
    return (
      <li className="nav-item dropdown">
        <button
          className="nav-link dropdown-toggle btn btn-link"
          id="userMenuDropdown"
          data-toggle="dropdown"
          aria-haspopup="true"
          aria-expanded="false"
        >
          {user.displayName}
        </button>
        <div
          className="dropdown-menu dropdown-menu-right"
          aria-labelledby="userMenuDropdown"
        >
          <Link to="/profile" className="dropdown-item">
            {t("header.userMenu.profile")}
          </Link>
          <div className="dropdown-divider"></div>
          <button className="dropdown-item" onClick={onLogout}>
            {t("header.userMenu.logout")}
          </button>
        </div>
      </li>
    );
  } else {
    return <></>;
  }
};
