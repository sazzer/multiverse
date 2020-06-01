import React from "react";
import { useUser } from "../currentUser";

/**
 * The user menu as displayed in the header bar
 */
export const UserMenu = () => {
  const { user } = useUser();

  if (user) {
    return (
      <li className="nav-item dropdown">
        <a
          className="nav-link dropdown-toggle"
          href="#"
          id="navbarDropdown"
          role="button"
          data-toggle="dropdown"
          aria-haspopup="true"
          aria-expanded="false"
        >
          {user.displayName}
        </a>
        <div
          className="dropdown-menu dropdown-menu-right"
          aria-labelledby="navbarDropdown"
        >
          <a className="dropdown-item" href="#">
            Log Out
          </a>
        </div>
      </li>
    );
  } else {
    return <></>;
  }
};
