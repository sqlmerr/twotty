import type { Metadata } from "next";
import { Montserrat } from "next/font/google";
import "./globals.css";
import { Header } from "@/components/header";
import { UserProvider } from "@/components/user-context";
import { ThemeProvider } from "@/components/theme-provider";
import { cn } from "@/lib/utils";

const font = Montserrat({ subsets: ["latin"] });

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
      <body className={cn(font.className, "dark:bg-zinc-950")}>
        <ThemeProvider
          attribute="class"
          defaultTheme="system"
          enableSystem
          disableTransitionOnChange
        >
          <UserProvider>
            <Header />
            {children}
          </UserProvider>
        </ThemeProvider>
      </body>
    </html>
  );
}
