import React, { useReducer } from "react";

import { StartAuthentication } from "./start";

interface ReducerState {
  username?: string;
  known?: boolean;
}

interface LookupUsernameAction {
  action: "LOOKUP_USERNAME";
  username: string;
  known: boolean;
}

export default () => {
  const initialState: ReducerState = {};

  const reducer = (state: ReducerState, action: LookupUsernameAction) => {
    return {
      username: action.username,
      known: action.known,
    } as ReducerState;
  };

  const [state, dispatch] = useReducer(reducer, initialState);

  if (state.known === true) {
    return <div>Log in as user: {state.username}</div>;
  } else if (state.known === false) {
    return <div>Register as user: {state.username}</div>;
  } else {
    return (
      <StartAuthentication
        onSubmit={(username, known) =>
          dispatch({ action: "LOOKUP_USERNAME", username, known })
        }
      />
    );
  }
};
