import type { Metadata } from "next";
import { Geologica } from "next/font/google";
import "./globals.css";
import { Header } from "@/components/header";
import { UserProvider } from "@/components/user-context";
import { ThemeProvider } from "@/components/theme-provider";

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
        <ThemeProvider
          attribute="class"
          defaultTheme="light"
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
