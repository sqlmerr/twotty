import type { Metadata } from "next";
import { Geologica } from "next/font/google";
import "./globals.css";
import { Header } from "@/components/header";
import { UserProvider } from "@/components/user-context";

const font = Geologica({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "Twotty",
  description: "Social network",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={font.className}>
        <UserProvider>
          <Header />
          {children}
        </UserProvider>
      </body>
    </html>
  );
}
