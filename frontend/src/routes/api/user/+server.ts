import { API_URL, type LoggedUser } from "$lib/api";

export const GET = async ({ request, fetch }) => {
    const res = await fetch(API_URL + "/user", {
        method: "GET",
        headers: [["Accept", "application/json"], ["cookie", request.headers.get("cookie")!]],
    });

    const user: LoggedUser = await res.json();

    return new Response(JSON.stringify(user), { status: res.status, headers: { ...res.headers, "Content-Type": "application/json" } });
};