import { NavLink, Route, Switch } from "react-router-dom";

import { PasswordView } from "./password";
import { ProfileView } from "./profile";
import React from "react";
import { WorldsProfileSection } from "./worlds";
import { useTranslation } from "react-i18next";
import { useUser } from "../currentUser";

export const Profile: React.FC = () => {
  const { t } = useTranslation();
  const { user } = useUser();

  return (
    <div className="container">
      <h2 id="profilePageLabel">
        {t("profile.title", { name: user?.displayName })}
      </h2>
      <div className="row" aria-labelledby="profilePageLabel">
        <div className="col-12 col-md-9 order-sm-3">
          <Switch>
            <Route path="/profile/password" component={PasswordView} />
            <Route path="/profile/worlds" component={WorldsProfileSection} />
            <Route path="/profile" component={ProfileView} />
          </Switch>
        </div>
        <div className="col-12 col-md-3" role="navigation">
          <ul className="nav nav-pills flex-column">
            <li className="nav-item">
              <NavLink to="/profile" exact className="nav-link">
                {t("profile.profile.label")}
              </NavLink>
            </li>
            <li className="nav-item">
              <NavLink to="/profile/password" className="nav-link">
                {t("profile.password.label")}
              </NavLink>
            </li>
            <li className="nav-item">
              <NavLink to="/profile/worlds" className="nav-link">
                {t("profile.worlds.label")}
              </NavLink>
            </li>
          </ul>
        </div>
      </div>
    </div>
  );
};
