import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import wasm from 'vite-plugin-wasm';
import top_level_await from 'vite-plugin-top-level-await';

export default defineConfig({
	plugins: [tailwindcss(), sveltekit(), wasm(), top_level_await()],
	server: {
		fs: {
			allow: ['../pkg']
		}
	}
});
