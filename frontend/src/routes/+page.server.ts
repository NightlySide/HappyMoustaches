import { api } from '$lib/api.js';

export const load = async ({ fetch, cookies }) => {
    let auth_cookie = cookies.get("id");
    let user = await api.get_user(auth_cookie, fetch);
    return { user };
}