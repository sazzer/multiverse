import { Route, BrowserRouter as Router, Switch } from "react-router-dom";

import HeaderBar from "./header";
import LandingPage from "./landing";
import { Profile } from "./profile";
import React from "react";
import { UserProvider } from "./currentUser";

export default () => {
  return (
    <UserProvider>
      <Router>
        <HeaderBar />

        <div className="container-fluid">
          <Switch>
            <Route path="/profile" component={Profile} />
            <Route path="/" component={LandingPage} />
          </Switch>
        </div>
      </Router>
    </UserProvider>
  );
};
