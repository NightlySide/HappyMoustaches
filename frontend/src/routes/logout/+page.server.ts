import { api } from '$lib/api.js';
import { redirect } from '@sveltejs/kit';

export const load = async ({ fetch, cookies }) => {
    // delete old cookie
    cookies.delete("id", { path: "/" });

    await api.logout(fetch);
    return redirect(301, "/login");
}