import React, { useContext, useState } from "react";

import debug from "debug";
import { request } from "./api";

/** The logger to use */
const LOGGER = debug("multiverse:currentUser");

/**
 * The shape of the user returned by the API
 */
interface UserResponse {
  /** The users unique username */
  username: string;
  /** The users display name */
  display_name: string;
  /** The users email address */
  email_address: string;
  /** The avatar for the user */
  avatar_url?: string;
}

/**
 * The shape of the user as stored in the webapp
 */
export interface User {
  /** The ID of the user */
  userId: string;
  /** The users unique username */
  username: string;
  /** The users display name */
  displayName: string;
  /** The users email address */
  emailAddress: string;
  /** The avatar for the user */
  avatarUrl?: string;
}

/**
 * The shape of the actual context store
 */
export interface UserContext {
  /** The user details */
  user: User | null;
  /** Callback to store the ID of the User */
  setUserId: (userId: string) => void;
  /** Callback to clear the User */
  clearUserId: () => void;
}

/** The actual context type */
const userContext = React.createContext<UserContext>({
  user: null,
  setUserId: () => {},
  clearUserId: () => {},
});

export const UserProvider: React.FC = ({ children }) => {
  const [user, setUser] = useState<User | null>(null);

  const contextValue = {
    user,
    setUserId: (userId: string) => {
      LOGGER("Setting User ID: %s", userId);
      request<UserResponse>("/users/{userId}", {
        urlParams: {
          userId,
        },
      })
        .then((response) => response.body!!)
        .then((user) => {
          LOGGER("User details: %o", user);
          setUser({
            userId,
            username: user.username,
            displayName: user.display_name,
            emailAddress: user.email_address,
            avatarUrl: user.avatar_url,
          });
        });
    },
    clearUserId: () => {
      setUser(null);
    },
  };

  return (
    <userContext.Provider value={contextValue}>{children}</userContext.Provider>
  );
};

/**
 * Hook to access the user details
 */
export function useUser() {
  const context = useContext(userContext);

  return {
    user: context.user,
    hasUser: context.user !== undefined,
    setUserId: context.setUserId,
    clearUserId: context.clearUserId,
  };
}
