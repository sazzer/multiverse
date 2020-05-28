import { Link, Route, BrowserRouter as Router, Switch } from "react-router-dom";

import React from "react";

export default () => {
  return (
    <Router>
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
