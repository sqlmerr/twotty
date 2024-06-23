"use server";

import { cookies } from "next/headers";
import User from "./models/user";
import Post from "./models/post";

export async function request(url: string | URL, init: RequestInit) {
  return await fetch(`http://localhost:8000${url}`, init);
}

export async function getMe(token: string) {
  const response = await request("/auth/me", {
    headers: {
      Authorization: `Bearer ${token}`,
    },
  });
  if (response.status != 200) {
    return;
  }

  const body = await response.json();
  console.log(body);
  return {
    id: body.id,
    username: body.username,
  } as User;
}

export async function getUser(username: string) {
  const token = cookies().get("access-token")?.value;
  if (!token) {
    return;
  }
  const response = await request(`/auth/@${username}`, {
    headers: {
      Authorization: `Bearer ${token}`,
    },
  });
  if (response.status != 200) {
    cookies().delete("access-token");
    return;
  }
  const body = await response.json();
  return {
    id: body.id,
    username: body.username,
    avatar: body.avatar,
    about: body.about,
  } as User;
}

export async function getUserPosts() {
  const token = cookies().get("access-token")?.value;
  if (!token) {
    return;
  }

  const response = await request(`/posts`, {
    headers: {
      Authorization: `Bearer ${token}`,
    },
  });
  if (response.status != 200) {
    cookies().delete("access-token");
    return;
  }
  const body = await response.json();
  // return {
  //   id: body.id,
  //   text: body.text,
  //   authorId: body.author_id,
  //   createdAt: body.created_at,
  //   edited: body.edited,
  // } as Post;
  console.log(body);

  return body.map((post: any) => {
    return {
      id: post.id,
      text: post.text,
      authorId: post.author_id,
      createdAt: post.created_at,
      edited: post.edited,
    } as Post;
  }) as [Post];
}
