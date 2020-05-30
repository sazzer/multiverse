import React, { useReducer } from "react";

import { Login } from "./login";
import { Register } from "./register";
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

/**
 * The shape of the action indicating authentication was cancelled
 */
interface CancelAuthAction {
  /** The action being performed */
  action: "CANCEL_AUTH";
}

/**
 * Reducer to manage this components state
 * @param state The previous state
 * @param action The action being performed
 */
function reducer(
  state: ReducerState,
  action: LookupUsernameAction | CancelAuthAction
): ReducerState {
  if (action.action === "LOOKUP_USERNAME") {
    return {
      username: action.payload.username,
      known: action.payload.known,
    };
  } else if (action.action === "CANCEL_AUTH") {
    return {};
  } else {
    return state;
  }
}

export default () => {
  const [state, dispatch] = useReducer(reducer, {});

  if (state.username) {
    if (state.known) {
      return (
        <Login
          username={state.username}
          onCancel={() => dispatch({ action: "CANCEL_AUTH" })}
        />
      );
    } else {
      return (
        <Register
          username={state.username}
          onCancel={() => dispatch({ action: "CANCEL_AUTH" })}
        />
      );
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
