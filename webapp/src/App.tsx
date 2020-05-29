import { Route, BrowserRouter as Router, Switch } from "react-router-dom";

import HeaderBar from "./header";
import LandingPage from "./landing";
import React from "react";

export default () => {
  return (
    <Router>
      <HeaderBar />

      <div className="container-fluid">
        <Switch>
          <Route path="/">
            <LandingPage />
          </Route>
        </Switch>
      </div>
    </Router>
  );
};
