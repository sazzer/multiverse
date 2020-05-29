import React, { useReducer } from "react";

import { Login } from "./login";
import { StartAuthentication } from "./start";

/**
 * The shape of the state in the reducer
 */
interface ReducerState {
  /** The username we are authentiacting as */
  username?: string;
  /** If we have a username, whether it's a known one or not */
  known?: boolean;
}

/**
 * The shape of the action indicating a username has been looked up
 */
interface LookupUsernameAction {
  /** The action being performed */
  action: "LOOKUP_USERNAME";
  /** The action payload */
  payload: {
    /** The username that was looked up */
    username: string;
    /** Whether the username is known or not */
    known: boolean;
  };
}

function reducer(
  state: ReducerState,
  action: LookupUsernameAction
): ReducerState {
  return {
    username: action.payload.username,
    known: action.payload.known,
  } as ReducerState;
}

export default () => {
  const [state, dispatch] = useReducer(reducer, {});

  if (state.username) {
    if (state.known) {
      return <Login username={state.username} />;
    } else {
      return <div>Register as user: {state.username}</div>;
    }
  } else {
    return (
      <StartAuthentication
        onSubmit={(username, known) =>
          dispatch({ action: "LOOKUP_USERNAME", payload: { username, known } })
        }
      />
    );
  }
};
