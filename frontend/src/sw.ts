// service worker
/// <reference lib="WebWorker" />
/// <reference types="vite/client" />
/// <reference no-default-lib="true"/>
/// <reference lib="esnext" />
import { cleanupOutdatedCaches, precacheAndRoute } from 'workbox-precaching'

declare let self: ServiceWorkerGlobalScope

self.addEventListener('message', (event) => {
    if (event.data && event.data.type === 'SKIP_WAITING')
        self.skipWaiting()
})

// clean old assets
cleanupOutdatedCaches()

// self.__WB_MANIFEST is default injection point
precacheAndRoute(self.__WB_MANIFEST)

// let allowlist: undefined | RegExp[]
// if (import.meta.env.DEV)
//     allowlist = [/^\/$/]

// // to allow work offline
// registerRoute(new NavigationRoute(
//     createHandlerBoundToURL('/'),
//     { allowlist },
// ))