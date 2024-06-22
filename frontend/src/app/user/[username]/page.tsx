"use client";

import { getUser } from "@/lib/api";
import User from "@/lib/models/user";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";

export default function Profile({ params }: { params: { username: string } }) {
  const router = useRouter();

  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState(false);
  const [user, setUser] = useState({ id: "", username: "" } as User);

  useEffect(() => {
    async function getUserByUsername() {
      const response = await getUser(params.username);
      if (!response) {
        setError(true);
        return;
      }

      setUser(response);
      setIsLoading(false);
    }
    getUserByUsername();
  }, [params.username, setUser]);

  if (error) {
    router.push("/login");
  }

  if (isLoading) {
    return <h1>Loading</h1>;
  }

  return <h1>{JSON.stringify(user)}</h1>;
}
