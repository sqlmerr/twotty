"use server";

import { Dispatch, SetStateAction } from "react";

export async function authAction(_currentState: unknown, formData: FormData) {
  const username = formData.get("username");
  const password = formData.get("password");

  if (username !== "sqlmerr" || password !== "password") {
    return "Invalid username or password";
  }

  console.log(formData);
}
