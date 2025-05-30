import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import wasm from 'vite-plugin-wasm';

export default defineConfig({
	plugins: [tailwindcss(), sveltekit(), wasm()],
	build: {
		target: ['es2022', 'edge89', 'firefox89', 'chrome89', 'safari15']
	},
	server: {
		fs: {
			allow: ['../pkg']
		}
	}
});
