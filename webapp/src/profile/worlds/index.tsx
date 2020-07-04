import { NavLink, Route, Switch } from "react-router-dom";

import { ListWorldsView } from "./list";
import { NewWorldView } from "./new";
import React from "react";

export const WorldsProfileSection: React.FC = () => {
  return (
    <div>
      <Switch>
        <Route path="/profile/worlds/new" component={NewWorldView} />
        <Route path="/profile/worlds" component={ListWorldsView} />
      </Switch>
    </div>
  );
};
