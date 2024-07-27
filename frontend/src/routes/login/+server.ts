import { API_URL } from "$lib/api";

export async function POST({ request, fetch }) {
    const { email, password } = await request.json();

    const res = await fetch(API_URL + "/login", {
        method: "POST",
        headers: [["Content-Type", "application/json"], ["Accept", "application/json"]],
        body: JSON.stringify({
            email,
            password,
        }),
        credentials: "include",
        cache: "no-cache"
    });

    const cookies = res.headers.getSetCookie();

    return new Response(JSON.stringify({ "token": cookies[0] }), { status: res.status, headers: { ...res.headers, "Content-Type": "application/json" } });
}