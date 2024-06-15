"use client";
/* eslint-disable react/no-unescaped-entities */
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Alert, AlertTitle, AlertDescription } from "@/components/ui/alert";
import Link from "next/link";
import { useFormState, useFormStatus } from "react-dom";
import { authAction } from "@/lib/actions";

export default function Auth() {
  const [errorMessage, dispatch] = useFormState(authAction, undefined);

  return (
    <form className="mx-auto max-w-md space-y-6" action={dispatch}>
      <div className="space-y-2 text-center">
        <h1 className="text-3xl font-bold">Login</h1>
        <p className="text-gray-500 dark:text-gray-400">
          Enter your username and password to sign in to your account.
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
        {errorMessage && (
          <Alert variant={"destructive"}>
            <div />
            <AlertTitle>{errorMessage}</AlertTitle>
          </Alert>
        )}
        <Button className="w-full" type="submit">
          Sign In
        </Button>
        <div className="text-center text-sm text-gray-500 dark:text-gray-400">
          Don't have an account?
          <Link className="underline" href="#">
            Sign up
          </Link>
        </div>
      </div>
    </form>
  );
}
