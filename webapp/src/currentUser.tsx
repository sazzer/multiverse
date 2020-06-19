import React, { useContext, useEffect, useState } from "react";
import { User, loadUser } from "./api/users";

import debug from "debug";

/** The logger to use */
const LOGGER = debug("multiverse:currentUser");

/** The key into Session Storage where the current user ID is stored */
const USER_KEY = "multiverse_current_user";

/**
 * The shape of the actual context store
 */
export interface UserContext {
  /** The user details */
  user: User | null;
  /** Callback to store the ID of the User */
  setUserLink: (userLink: string) => void;
  /** Callback to clear the User */
  clearUserId: () => void;
}

/** The actual context type */
const userContext = React.createContext<UserContext>({
  user: null,
  setUserLink: () => {},
  clearUserId: () => {},
});

export const UserProvider: React.FC = ({ children }) => {
  const [user, setUser] = useState<User | null>(null);
  useEffect(() => {
    const storedUserId = sessionStorage.getItem(USER_KEY);
    if (storedUserId) {
      LOGGER("Loading remembered user: %s", storedUserId);
      loadUser(storedUserId).then(setUser);
    }
  }, []);

  const contextValue = {
    user,
    setUserLink: (userLink: string) => {
      LOGGER("Loading user: %s", userLink);
      loadUser(userLink).then((user) => {
        setUser(user);
        sessionStorage.setItem(USER_KEY, userLink);
      });
    },
    clearUserId: () => {
      sessionStorage.removeItem(USER_KEY);
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
    userLink: context.user?.selfLink,
    hasUser: context.user !== undefined,
    setUserLink: context.setUserLink,
    reloadUser: () => {
      if (context.user) {
        context.setUserLink(context.user.selfLink);
      }
    },
    clearUserId: context.clearUserId,
  };
}
