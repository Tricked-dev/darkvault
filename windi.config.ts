import typography from 'windicss/plugin/typography';
import { defineConfig } from 'vite-plugin-windicss';
import { transform } from 'windicss/helpers';
import daisyColors from 'daisyui/colors/windi';
export default defineConfig({
	theme: {
		extend: {
			colors: daisyColors
		}
	},
	extract: {
		include: ['./src/**/*.{html,js,svelte,ts}']
	},
	plugins: [typography, transform('daisyui')]
});
