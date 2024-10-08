import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import { SvelteKitPWA } from '@vite-pwa/sveltekit';

export default defineConfig({
	plugins: [sveltekit(), SvelteKitPWA({
		strategies: 'injectManifest',
		registerType: "prompt",
		srcDir: './src',
		mode: "development",
		filename: 'sw.ts',
		manifest: {
			short_name: 'SvelteKit PWA',
			name: 'SvelteKit PWA',
			description: "Application de suivi des moustachus",
			start_url: '/login',
			scope: '/',
			display: 'standalone',
			theme_color: "#ffffff",
			background_color: "#ffffff",
			orientation: "portrait",
			dir: "ltr",
			id: "moustache-and-co",
			icons: [
				{
					src: '/pwa-192x192.png',
					sizes: '192x192',
					type: 'image/png',
				},
				{
					src: '/pwa-512x512.png',
					sizes: '512x512',
					type: 'image/png',
				},
				{
					src: '/pwa-512x512.png',
					sizes: '512x512',
					type: 'image/png',
					purpose: 'any maskable',
				},
			],
		},
		workbox: {
			globPatterns: ['client/**/*.{js,css,ico,png,svg,webp,woff,woff2}']

		},
		devOptions: {
			enabled: true,
			suppressWarnings: process.env.SUPPRESS_WARNING === 'true',
			type: 'module',
			navigateFallback: '/',
		}
	})],
	define: {
		'process.env.NODE_ENV': process.env.NODE_ENV === 'production'
			? '"production"'
			: '"development"'
	}
});
