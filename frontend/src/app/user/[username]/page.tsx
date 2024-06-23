"use client";

import { Post as PostCard } from "@/components/post";
import { UserProfile } from "@/components/user-profile";
import { getUser, getUserPosts } from "@/lib/api";
import Post from "@/lib/models/post";
import User from "@/lib/models/user";
import { useRouter } from "next/navigation";
import { useEffect, useState } from "react";

export default function Profile({ params }: { params: { username: string } }) {
  const router = useRouter();

  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState(false);
  const [user, setUser] = useState({
    id: "",
    username: "",
    avatar: null,
  } as User);
  const [posts, setPosts] = useState([
    { id: "", text: "", authorId: "", edited: false, createdAt: new Date(1) },
  ] as [Post]);

  useEffect(() => {
    async function getUserByUsernameAndPosts() {
      const user = await getUser(params.username);
      if (!user) {
        setError(true);
        return;
      }

      const posts = await getUserPosts();
      if (!posts) {
        setError(true);
        console.log(posts);
        return;
      }

      setUser(user);
      setPosts(posts);
      setIsLoading(false);
    }
    getUserByUsernameAndPosts();
  }, [params.username, setUser]);

  if (error) {
    router.push("/login");
  }

  if (isLoading) {
    return <h1>Loading</h1>;
  }

  return <UserProfile user={user} posts={posts} />;
}
