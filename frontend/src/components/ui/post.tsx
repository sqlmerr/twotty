import User from "@/lib/models/user";

interface Props {
  text: string;
  author: User;
  edited: boolean;
}

export function PostMini({ text, author, edited }: Props) {}

export function PostFull({ text, author, edited }: Props) {}
