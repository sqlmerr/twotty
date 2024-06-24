"use client";

import useUserContext from "@/components/user-context";
import { deleteCookie } from "@/lib/actions";
import { redirect } from "next/navigation";
import { useEffect } from "react";

export default function LogoutPage() {
  const { user, setUser } = useUserContext();

  useEffect(() => {
    async function action() {
      await deleteCookie("access-token");
      setUser(undefined);
    }

    action();
  });
  return <div></div>;
}
