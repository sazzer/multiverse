import { Link, Route, BrowserRouter as Router, Switch } from "react-router-dom";

import HeaderBar from "./header";
import React from "react";
import { useTranslation } from "react-i18next";

export default () => {
  const { t } = useTranslation();

  return (
    <Router>
      <HeaderBar />
      <div>{t("page.title")}</div>
      <Link to="/">Home</Link>
      <Link to="/about">About</Link>
      <Link to="/users">Users</Link>
      <Switch>
        <Route path="/about">
          <div>About</div>
        </Route>
        <Route path="/users">
          <div>Users</div>
        </Route>
        <Route path="/">
          <div>Home</div>
        </Route>
      </Switch>
    </Router>
  );
};
