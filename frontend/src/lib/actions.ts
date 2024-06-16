"use server";

import { redirect } from "next/navigation";
import { request } from "./api";
import { cookies } from "next/headers";

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

  return redirect("/");
}
