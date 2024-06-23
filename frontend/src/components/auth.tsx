"use client";
/* eslint-disable react/no-unescaped-entities */
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import useUserContext from "@/components/user-context";
import { Alert, AlertTitle } from "@/components/ui/alert";
import Link from "next/link";
import { useFormState } from "react-dom";
import { loginAction, registerAction } from "@/lib/actions";
import { redirect } from "next/navigation";

export default function Auth({ registration }: { registration?: boolean }) {
  const { user, setUser } = useUserContext();

  // if (user) {
  //   redirect(`/@${user.username}`);
  // }

  async function action(_currentState: unknown, formData: FormData) {
    if (registration) {
      return await registerAction(_currentState, formData);
    } else {
      const me = await loginAction(_currentState, formData);
      console.log("aaaaaa", me);
      if (typeof me === "string") {
        return me;
      }
      setUser(me);
      return redirect(`/@${me?.username}`);
    }
  }

  const [errorMessage, dispatch] = useFormState(action, undefined);

  return (
    <form className="mx-auto max-w-md space-y-6" action={dispatch}>
      <div className="space-y-2 text-center">
        <h1 className="text-3xl font-bold">
          {registration ? "Sign Up" : "Login"}
        </h1>
        <p className="text-gray-500 dark:text-gray-400">
          {registration
            ? "Enter username and password to create new account"
            : "Enter your username and password to sign in to your account."}
        </p>
      </div>
      <div className="space-y-4">
        <div className="space-y-2">
          <Label htmlFor="username">username</Label>
          <Input
            id="username"
            placeholder="username"
            required
            type="username"
            name="username"
          />
        </div>
        <div className="space-y-2">
          <Label htmlFor="password">password</Label>
          <Input id="password" required type="password" name="password" />
        </div>
        {registration && (
          <div className="space-y-2">
            <Label htmlFor="confirmPassword">confirm password</Label>
            <Input
              id="confirmPassword"
              required
              type="password"
              name="confirmPassword"
            />
          </div>
        )}
        {errorMessage && (
          <Alert variant={"destructive"}>
            <div />
            <AlertTitle>{errorMessage}</AlertTitle>
          </Alert>
        )}
        <Button className="w-full" type="submit">
          {registration ? "Sign Up" : "Sign In"}
        </Button>
        <div className="text-center text-sm text-gray-500 dark:text-gray-400">
          {registration ? "Already" : "Don't"} have an account?{" "}
          <Link
            className="underline"
            href={registration ? "/login" : "/register"}
          >
            {registration ? "Login" : "Sign up"}
          </Link>
        </div>
      </div>
    </form>
  );
}
