import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import wasm from 'vite-plugin-wasm';
import top_level_await from 'vite-plugin-top-level-await';

export default defineConfig({
	plugins: [wasm(), top_level_await(), sveltekit()]
});
