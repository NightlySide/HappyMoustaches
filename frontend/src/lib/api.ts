import { writable } from 'svelte/store';
import { setCookie } from './utils';

const API_HOST = "localhost:3000";
export const API_URL = `http://${API_HOST}`;

export interface LoggedUser {
    id: number;
    email: string;
};

export interface APIStoreData {
    user: LoggedUser | null,
    token: string | null,
    loading: boolean,
    error: string | null,
};

const initAPIStore = () => {
    const { subscribe, set, update } = writable({
        user: null,
        token: null,
        loading: false,
        error: null,
    } as APIStoreData);

    return {
        subscribe,
        login: async (email: string, password: string) => {
            update((state) => ({ ...state, loading: true }));

            // make the request
            try {
                const res = await fetch("/login", {
                    method: "POST",
                    headers: [["Content-Type", "application/json"], ["Accept", "application/json"]],
                    body: JSON.stringify({
                        email,
                        password,
                    }),
                    cache: "no-cache"
                });
                const { token } = await res.json();

                // check that the user is authorized
                if (res.status != 200) {
                    set({ user: null, token: null, error: "Unauthorized", loading: false });
                }

                // get the token from the cookie
                set({ user: null, token, error: "", loading: false });
                setCookie("id", (token as string).replace("id=", ""), 30, false);

                return res.status == 200;
            } catch (error: any) {
                console.error("Error while login:", error);
                set({ user: null, token: null, error: error, loading: false });
            }
        },

        logout: async (custom_fetch?: any) => {
            const fetchfunc = custom_fetch ? custom_fetch : fetch;
            update((state) => ({ ...state, loading: true }));

            try {
                const res = await fetchfunc("/logout", {
                    method: "GET",
                    cache: "no-cache"
                });

                return res.status == 200;
            } catch (error: any) {
                console.error("Error while logout:", error);
                set({ user: null, token: null, error: error, loading: false });
            }
        },

        get_user: async (token?: string, custom_fetch?: any) => {
            const fetchfunc = custom_fetch ? custom_fetch : fetch;
            update((state) => ({ ...state, loading: true }));

            // make the request
            try {
                const res = await fetchfunc("/api/user", {
                    method: "GET",
                    headers: [["Accept", "application/json"], ["cookie", token ? token : document.cookie]],
                });
                const user = await res.json();

                // check that the user is authorized
                if (res.status != 200) {
                    update((state) => ({ ...state, user: null, error: "Unauthorized", loading: false }));
                }

                // update the user
                update((state) => ({ ...state, user: user, error: "", loading: false }));

                return user;
            } catch (error: any) {
                console.error("Error while fetching user info:", error);
                update((state) => ({ ...state, user: null, error: error, loading: false }));
            }
        }
    }
}

export const api = initAPIStore();