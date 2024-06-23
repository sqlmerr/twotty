"use client";

import { useState, createContext, useContext } from "react";
import User from "@/lib/models/user";

interface UserState {
  user?: User;
  setUser: (user: User) => void;
}

export const UserContext = createContext({} as UserState);

export function UserProvider({ children }: { children: React.ReactNode }) {
  const [user, setUser] = useState({} as User);
  return (
    <UserContext.Provider value={{ user, setUser } as UserState}>
      {children}
    </UserContext.Provider>
  );
}

export default function useUserContext() {
  return useContext(UserContext);
}
