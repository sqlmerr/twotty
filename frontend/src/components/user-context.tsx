"use client";

import { useState, useEffect, createContext, useContext } from "react";
import { redirect } from "next/navigation";
import { getMe } from "@/lib/api";
import { getUserStateAction } from "@/lib/actions";
import User from "@/lib/models/user";

interface UserState {
  user?: User;
  setUser: (user: User) => void;
}

export const UserContext = createContext({} as UserState);

export function UserProvider({ children }: { children: React.ReactNode }) {
  const [user, setUser] = useState({} as User);

  useEffect(() => {
    async function getUser() {
      const user = await getUserStateAction();
      if (!user) {
        // redirect("/login");
        return;
      }
      setUser(user);
    }
    getUser();
  }, []);

  return (
    <UserContext.Provider value={{ user, setUser } as UserState}>
      {children}
    </UserContext.Provider>
  );
}

export default function useUserContext() {
  return useContext(UserContext);
}
