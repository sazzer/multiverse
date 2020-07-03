import { Link, useHistory } from "react-router-dom";

import React from "react";
import { clearToken } from "../api/http/token";
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
      <li className="nav-item dropdown" role="menuitem" data-testid="userMenu">
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
          role="menu"
        >
          <Link to="/profile" className="dropdown-item" role="menuitem">
            {t("header.userMenu.profile")}
          </Link>
          <div className="dropdown-divider" role="separator"></div>
          <Link
            to="/profile/worlds/new"
            className="dropdown-item"
            role="menuitem"
          >
            {t("header.userMenu.newWorld")}
          </Link>
          <div className="dropdown-divider" role="separator"></div>
          <button className="dropdown-item" onClick={onLogout} role="menuitem">
            {t("header.userMenu.logout")}
          </button>
        </div>
      </li>
    );
  } else {
    return <></>;
  }
};
