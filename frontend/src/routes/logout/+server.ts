import { API_URL } from "$lib/api";

export const GET = async () => {
    const res = await fetch(API_URL + "/logout", {
        method: "GET",
        headers: [["Accept", "application/json"]],
    });

    return new Response(null, { status: res.status });
};