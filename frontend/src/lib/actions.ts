"use server";

import { redirect } from "next/navigation";
import { getMe, request } from "./api";
import { cookies } from "next/headers";
import User from "./models/user";

export async function loginAction(_currentState: unknown, formData: FormData) {
  const username = formData.get("username");
  const password = formData.get("password");

  let response = await request("/auth/login", {
    method: "POST",
    body: JSON.stringify({ username: username, password: password }),
    headers: {
      "Content-Type": "application/json",
    },
  });

  if (response.status == 401) {
    return "Invalid username or password";
  } else if (response.status != 200) {
    console.log(response.status);
    return "Server error";
  }

  let body = await response.json();
  console.log(body);

  cookies().set("access-token", body.access_token);
  const me = await getMe(body.access_token);

  return me as User;
}

export async function registerAction(
  _currentState: unknown,
  formData: FormData
) {
  const username = formData.get("username");
  const password = formData.get("password");
  const confirmPassword = formData.get("confirmPassword");
  if (password != confirmPassword) {
    return "Passwords do not match";
  }
  let response = await request("/auth/register", {
    method: "POST",
    body: JSON.stringify({ username: username, password: password, about: "" }),
    headers: {
      "Content-Type": "application/json",
    },
  });
  if (response.status == 403) {
    return "Username is already occupied";
  }
  if (response.status != 201) {
    console.log(response.status);
    return "Server error";
  }

  return redirect("/login");
}

export async function getUserStateAction() {
  const token = cookies().get("access-token")?.value;
  if (!token) {
    return;
  }
  const user = await getMe(token);
  if (!user) {
    return;
  }
  return user;
}

export async function deleteCookie(cookieName: string) {
  cookies().delete(cookieName);
}
