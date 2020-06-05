import React, { useContext, useEffect, useState } from "react";
import { User, loadUser } from "./profile/api";

import debug from "debug";

/** The logger to use */
const LOGGER = debug("multiverse:currentUser");

/** The key into Session Storage where the current user ID is stored */
const USER_ID_KEY = "multiverse_current_user";

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
  useEffect(() => {
    const storedUserId = sessionStorage.getItem(USER_ID_KEY);
    if (storedUserId) {
      LOGGER("Loading remembered user: %s", storedUserId);
      loadUser(storedUserId).then(setUser);
    }
  }, []);

  const contextValue = {
    user,
    setUserId: (userId: string) => {
      LOGGER("Loading user: %s", userId);
      loadUser(userId).then((user) => {
        setUser(user);
        sessionStorage.setItem(USER_ID_KEY, userId);
      });
    },
    clearUserId: () => {
      sessionStorage.removeItem(USER_ID_KEY);
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
    userId: context.user?.userId,
    hasUser: context.user !== undefined,
    setUserId: context.setUserId,
    clearUserId: context.clearUserId,
  };
}
