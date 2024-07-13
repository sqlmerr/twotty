"use client";

import { Post as PostCard } from "@/components/post";
import useUserContext from "@/components/user-context";
import { UserProfile } from "@/components/user-profile";
import {
  getFollowers,
  getFollowings,
  getIsFollowed,
  getUser,
  getUserPosts,
} from "@/lib/api";
import Post from "@/lib/models/post";
import User from "@/lib/models/user";
import Loader from "@/components/ui/loader";
import { redirect, useRouter } from "next/navigation";
import { useEffect, useState } from "react";
import { Yesteryear } from "next/font/google";

export default function Profile({ params }: { params: { username: string } }) {
  const router = useRouter();

  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState(false);
  const [user, setUser] = useState({} as User);
  const [posts, setPosts] = useState([
    { id: "", text: "", authorId: "", edited: false, createdAt: new Date(1) },
  ] as [Post]);
  const [isFollowed, setIsFollowed] = useState(false);

  useEffect(() => {
    async function getData() {
      const user = await getUser(params.username);
      if (!user) {
        setError(true);
        console.log("user: ", user);
        return;
      }

      const followings = await getFollowings(user.id);
      const followers = await getFollowers(user.id);
      if (typeof followings !== "number" || typeof followers !== "number") {
        setError(true);
        console.log("followings + followers: ", followings, followers);
        return;
      }
      user.followers = followers;
      user.followings = followings;

      const posts = await getUserPosts(user.username);
      if (!posts) {
        setError(true);
        console.log("posts: ", posts);
        return;
      }

      const followed = await getIsFollowed(user.id);
      if (followed === undefined) {
        setError(true);
        return;
      }

      setUser(user);
      setPosts(posts);
      setIsFollowed(followed);
      setIsLoading(false);
    }
    getData();
  }, [params.username, setUser]);

  if (error) {
    console.log(error);
    router.push("/login");
  }

  if (isLoading) {
    return <Loader />;
  }

  return (
    <UserProfile
      author={user}
      setAuthor={setUser}
      posts={posts}
      setPosts={setPosts}
      isFollowed={isFollowed}
      setIsFollowed={setIsFollowed}
    />
  );
}
