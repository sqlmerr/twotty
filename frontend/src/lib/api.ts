"use server";

import { cookies } from "next/headers";
import User from "./models/user";
import Post from "./models/post";
import { redirect } from "next/navigation";

export async function request(url: string | URL, init: RequestInit) {
  const backend_url = process.env.BACKEND_URL
  return await fetch(`${backend_url}${url}`, init);
}

export async function getMe(token?: string) {
  if (!token) {
    const token = cookies().get("access-token")?.value;
    const response = await request("/auth/me", {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    });
    if (response.status != 200) {
      return;
    }

    const body = await response.json();
    return body as User;
  }
  const response = await request("/auth/me", {
    headers: {
      Authorization: `Bearer ${token}`,
    },
  });
  if (response.status != 200) {
    return;
  }

  const body = await response.json();
  return body as User;
}

export async function getUser(username: string) {
  const token = cookies().get("access-token")?.value;
  if (!token) {
    return;
  }
  const response = await request(`/users/@${username}`, {
    headers: {
      Authorization: `Bearer ${token}`,
    },
  });
  if (response.status != 200) {
    console.log(response.status);
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

export async function updateUser(data: object) {
  const token = cookies().get("access-token")?.value;
  if (!token) {
    return false;
  }
  const response = await request(`/auth`, {
    method: "PATCH",
    headers: {
      Authorization: `Bearer ${token}`,
      "Content-Type": "application/json",
    },
    body: JSON.stringify(data),
  });
  const body = await response.json();
  console.log(response, body);
  if (response.status != 200) {
    if (response.status == 403) {
      return body.message as string;
    }
    if (response.status == 400) {
      return redirect("/login");
    }
    console.log(response, body);
    return false;
  }
  return true;
}

export async function getUserPosts(username: string) {
  const token = cookies().get("access-token")?.value;
  if (!token) {
    return;
  }

  const response = await request(`/posts/@${username}`, {
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

export async function createPost(text: string) {
  const token = cookies().get("access-token")?.value;
  if (!token) {
    return;
  }
  const response = await request(`/posts`, {
    method: "POST",
    headers: {
      Authorization: `Bearer ${token}`,
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ text: text }),
  });
  if (response.status != 201) {
    return;
  }
  const body = await response.json();
  return body as Post;
}

export async function deletePost(id: string) {
  const token = cookies().get("access-token")?.value;
  if (!token) {
    return;
  }
  const response = await request(`/posts/${id}`, {
    method: "DELETE",
    headers: {
      Authorization: `Bearer ${token}`,
      "Content-Type": "application/json",
    },
  });
  if (response.status != 200) {
    cookies().delete("access-token");
    return;
  }
}

export async function editPost(id: string, text: string) {
  const token = cookies().get("access-token")?.value;
  if (!token) {
    return false;
  }
  const response = await request(`/posts/${id}`, {
    method: "PATCH",
    headers: {
      Authorization: `Bearer ${token}`,
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ text: text }),
  });
  if (response.status != 200) {
    return false;
  }
  const body = await response.json();
  return body.ok;
}

export async function getFollowings(id: string) {
  const token = cookies().get("access-token")?.value;
  const response = await request(`/users/${id}/followings`, {
    method: "GET",
    headers: {
      Authorization: `Bearer ${token}`,
      "Content-Type": "application/json",
    },
  });
  if (response.status != 200) {
    return false;
  }
  const body = await response.json();
  return body.count;
}

export async function getFollowers(id: string) {
  const token = cookies().get("access-token")?.value;
  const response = await request(`/users/${id}/followers`, {
    method: "GET",
    headers: {
      Authorization: `Bearer ${token}`,
      "Content-Type": "application/json",
    },
  });
  if (response.status != 200) {
    console.log(await response.json());
    cookies().delete("access-token");
    return redirect("/login");
  }
  const body = await response.json();
  return body.count;
}

export async function getIsFollowed(id: string) {
  const token = cookies().get("access-token")?.value;
  const response = await request(`/users/${id}/followed`, {
    method: "GET",
    headers: {
      Authorization: `Bearer ${token}`,
      "Content-Type": "application/json",
    },
  });

  if (response.status !== 200) {
    return;
  }

  const body = await response.json();
  return body.isFollowed;
}

export async function follow(id: string) {
  const token = cookies().get("access-token")?.value;
  const response = await request(`/users/${id}/follow`, {
    method: "POST",
    headers: {
      Authorization: `Bearer ${token}`,
      "Content-Type": "application/json",
    },
  });

  if (response.status !== 200 && response.status !== 201) {
    return;
  }
  const body = await response.json();
  return body as { ok: boolean };
}

export async function unfollow(id: string) {
  const token = cookies().get("access-token")?.value;
  const response = await request(`/users/${id}/unfollow`, {
    method: "POST",
    headers: {
      Authorization: `Bearer ${token}`,
      "Content-Type": "application/json",
    },
  });

  if (response.status !== 200 && response.status !== 201) {
    return;
  }
  const body = await response.json();
  return body as { ok: boolean };
}
