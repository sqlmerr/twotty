/**
 * This code was generated by v0 by Vercel.
 * @see https://v0.dev/t/hMVCxaoL2Hy
 * Documentation: https://v0.dev/docs#integrating-generated-code-into-your-nextjs-app
 */

import { Card, CardContent } from "@/components/ui/card";
import Link from "next/link";
import { Avatar, AvatarImage, AvatarFallback } from "@/components/ui/avatar";
import User from "@/lib/models/user";
import { format } from "date-fns";

interface PostProps {
  text: string;
  edited: boolean;
  createdAt: Date;
  user: User;
}

export function Post({ text, edited, createdAt, user }: PostProps) {
  const time = format(createdAt, "yyyy-MM-dd HH:mm:ss");
  return (
    <Card className="w-full max-w-2xl">
      <CardContent className="p-4 md:p-6">
        <div className="flex items-start gap-4">
          <Link href="#" className="flex-shrink-0" prefetch={false}>
            <Avatar className="w-10 h-10 border">
              <AvatarImage
                src={user.avatar ? user.avatar : "/placeholder-user.jpg"}
              />
              <AvatarFallback>{user.username}</AvatarFallback>
            </Avatar>
          </Link>
          <div className="flex-1">
            <div className="flex items-center gap-2">
              <Link
                href={`/@${user.username}`}
                className="font-medium"
                prefetch={false}
              >
                @{user.username}
              </Link>
              <div className="text-xs text-slate-500 dark:text-slate-400">
                <time dateTime={time}>{time}</time>
              </div>
            </div>
            <div className="prose prose-sm text-slate-950 dark:text-slate-50">
              <p>{text}</p>
            </div>
            {edited && (
              <div className="mt-2 text-xs text-slate-500 dark:text-slate-400">
                Edited
              </div>
            )}
          </div>
        </div>
      </CardContent>
    </Card>
  );
}
