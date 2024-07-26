export const clearCookies = () => {
    const cookies = document.cookie.split(";");

    // if empty do nothing
    if (cookies.length === 0) return;

    let new_cookies = "";
    for (let i = 0; i < cookies.length; i++) {
        const cookie = cookies[i];
        if (cookie.trim().length == 0) continue;

        const eqPos = cookie.indexOf("=");
        const name = eqPos > -1 ? cookie.substr(0, eqPos) : cookie;

        if (name.trim().length == 0) continue;

        new_cookies += name + "=;expires=Thu, 01 Jan 1970 00:00:00 UTC;path=/;";
    }

    if (new_cookies != "")
        document.cookie = new_cookies;
}

export function setCookie(name: string, value: string, exdays: number, secure: boolean) {
    const d = new Date();
    d.setTime(d.getTime() + (exdays * 24 * 60 * 60 * 1000));
    let expires = "expires=" + d.toUTCString();
    if (secure) {
        document.cookie = name + "=" + value + ";" + expires + ";path=/; secure";
    } else {
        document.cookie = name + "=" + value + ";" + expires + ";path=/";
    }
}

export function getCookie(name: string) {
    let cookieName = name + "=";
    let decodedCookie = decodeURIComponent(document.cookie);
    let ca = decodedCookie.split(';');
    for (let i = 0; i < ca.length; i++) {
        let c = ca[i];
        while (c.charAt(0) == ' ') {
            c = c.substring(1);
        }
        if (c.indexOf(cookieName) == 0) {
            return c.substring(cookieName.length, c.length);
        }
    }
    return "";
}
