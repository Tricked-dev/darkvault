import { mdsvex } from 'mdsvex';
import mdsvexConfig from './mdsvex.config.js';
import adapter from '@sveltejs/adapter-static';
import preprocess from 'svelte-preprocess';
import WindiCSS from 'vite-plugin-windicss';
/** @type {import('@sveltejs/kit').Config} */
const config = {
	extensions: ['.svelte', ...mdsvexConfig.extensions],

	// Consult https://github.com/sveltejs/svelte-preprocess
	// for more information about preprocessors
	preprocess: [preprocess(), mdsvex(mdsvexConfig)],

	kit: {
		adapter: adapter({
			fallback: '200.html'
		}),
		amp: false,
		trailingSlash: 'never',
		prerender: {
			enabled: false
		},
		vite: {
			server: {
				fs: {
					// Allow serving files from one level up to the project root
					allow: ['..']
				}
			},
			plugins: [WindiCSS()]
		}
	}
};

export default config;
