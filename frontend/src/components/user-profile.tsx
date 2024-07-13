"use client";

/**
 * This code was generated by v0 by Vercel.
 * @see https://v0.dev/t/cSK1wK8fG7M
 * Documentation: https://v0.dev/docs#integrating-generated-code-into-your-nextjs-app
 */

import { Avatar, AvatarImage, AvatarFallback } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import { Textarea } from "@/components/ui/textarea";
import Post from "@/lib/models/post";
import User from "@/lib/models/user";
import { Post as PostCard } from "./post";
import { useState } from "react";
import { createPost, follow, getUserPosts, unfollow } from "@/lib/api";
import useUserContext from "./user-context";
import { redirect } from "next/navigation";
import { Alert, AlertTitle } from "./ui/alert";

export function UserProfile({
  author,
  setAuthor,
  posts,
  setPosts,
  isFollowed,
  setIsFollowed,
}: {
  author: User;
  setAuthor: any;
  posts: [Post];
  setPosts: any;
  isFollowed: boolean;
  setIsFollowed: any;
}) {
  const { user, setUser } = useUserContext();
  const [message, setMessage] = useState("");
  const [errorMessage, setErrorMessage] = useState("");

  if (!user) {
    console.log(user);
    redirect("/login");
  }

  async function handleSubmit(e: any) {
    e.preventDefault();
    const response = await createPost(message);
    if (!response) {
      setErrorMessage("Text length must be less than 256 characters");
      return;
    }
    const userPosts = await getUserPosts(author.username);
    setPosts(userPosts);
    setMessage("");
    setErrorMessage("");
  }

  async function handleFollow(e: any) {
    e.preventDefault();
    if (typeof author.followers !== "number") {
      return;
    }

    if (isFollowed) {
      await unfollow(author.id);
      setIsFollowed(false);
      if (author.followers < 1) {
        author.followers = 0;
      } else {
        author.followers -= 1;
      }
      setAuthor(author);
    } else {
      await follow(author.id);
      setIsFollowed(true);

      author.followers += 1;
      setAuthor(author);
    }
  }

  return (
    <div className="grid md:grid-cols-[300px_1fr] gap-8 max-w-6xl mx-auto px-4 py-8">
      <div className="flex flex-col items-center gap-4">
        <Avatar className="w-24 h-24 dark:border-slate-50">
          <AvatarImage
            src={author.avatar ? author.avatar : "/placeholder-user.jpg"}
          />
          <AvatarFallback>JD</AvatarFallback>
        </Avatar>
        <div className="text-center">
          <div className="text-2xl font-bold">@{author.username}</div>
          {/* <div className="text-slate-500 dark:text-slate-400">@johndoe</div> */}
          <div className="text-muted-foreground mt-2">{author.about}</div>
          <div className="text-slate-500 dark:text-slate-400">
            followers: {author.followers ? author.followers : 0}
          </div>
          <div className="text-slate-500 dark:text-slate-400">
            followings: {author.followings ? author.followings : 0}
          </div>
          {user?.id != author.id && (
            <Button variant={"secondary"} size={"sm"} onClick={handleFollow}>
              {isFollowed ? "unfollow" : "follow"}
            </Button>
          )}
        </div>
      </div>
      <div className="grid gap-6">
        {errorMessage && (
          <Alert variant={"destructive"}>
            <AlertTitle>{errorMessage}</AlertTitle>
          </Alert>
        )}
        {user.id == author.id && (
          <div className="grid gap-4">
            <Textarea
              value={message}
              onChange={(e) => setMessage(e.target.value)}
              className="p-4 min-h-[100px] rounded-md border focus:border-primary focus:ring-primary"
              placeholder="Write a new post..."
            />
            <Button onClick={handleSubmit} className="ml-auto">
              Post
            </Button>
          </div>
        )}
        <div className="grid gap-4">
          {posts.map((post) => {
            return (
              <PostCard
                key={post.id}
                id={post.id}
                text={post.text}
                edited={post.edited}
                createdAt={post.createdAt}
                author={author}
                currentUser={user}
                setPosts={setPosts}
              />
            );
          })}
        </div>
      </div>
    </div>
  );
}
