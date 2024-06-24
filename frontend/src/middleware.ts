import { type NextRequest, NextResponse } from "next/server";

export function middleware(request: NextRequest) {
  const accessToken = request.cookies.get("access-token")?.value;

  if (!accessToken && request.nextUrl.pathname != "/login") {
    return NextResponse.redirect(new URL("/login", request.url));
  } else {
  }
  //   if (
  //     !accessToken &&
  //     !request.nextUrl.pathname.startsWith("/register") &&
  //     !request.nextUrl.pathname.startsWith("/login")
  //   ) {
  //     return NextResponse.redirect(new URL("/register", request.url));
  //   }
  if (accessToken && request.nextUrl.pathname.startsWith("/login")) {
    return NextResponse.redirect(new URL("/", request.url));
  }
}

export const config = {
  matcher: ["/", "/login", "/user/:path", "/logout"],
};
