import React, { useContext, useState } from "react";

import debug from "debug";

/** The logger to use */
const LOGGER = debug("multiverse:currentUser");

export interface UserContext {
  user: string | null;
  hasUser: boolean;
  setUserId: (userId: string) => void;
}

/** The actual context type */
const userContext = React.createContext<UserContext>({
  user: null,
  hasUser: false,
  setUserId: () => {},
});

export const UserProvider: React.FC = ({ children }) => {
  const [user, setUser] = useState<string | null>(null);

  const contextValue = {
    user,
    hasUser: user !== null,
    setUserId: (userId: string) => {
      LOGGER("Setting User ID: %s", userId);
      setUser(userId);
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
  return useContext(userContext);
}
