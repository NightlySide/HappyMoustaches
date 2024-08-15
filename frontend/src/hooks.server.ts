import type { Handle } from '@sveltejs/kit';

export const handle: Handle = async ({ event, resolve }) => {
  // Check if the user is logged in
  const isLoggedIn = event.cookies.get("id") != undefined;

  // If the user is not logged in and the requested path is not the login page, redirect to the login page
  if (!isLoggedIn && !event.url.pathname.startsWith('/login') && event.url.pathname != "/") {
    return new Response(null, {
      status: 302,
      headers: {
        location: '/login',
      },
    });
  }

  if (isLoggedIn && event.url.pathname.startsWith("/login")) {
    return new Response(null, {
      status: 301,
      headers: {
        location: '/dashboard',
      },
    });
  }

  // If the user is logged in or the requested path is the login page, continue to the requested page
  return resolve(event);
};