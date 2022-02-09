<script>
	import { metaStore } from '$lib/metastore';
	import { onMount } from 'svelte';
	import SvelteSeo from 'svelte-seo';

	import '../app.css';
	let title, description;
	async function getMeta() {
		let meta;
		if (localStorage.getItem('meta')) {
			meta = localStorage.getItem('meta');
		} else {
			meta = await fetch(`${import.meta.env['VITE_SERVER_URL'] || ''}/meta.json`).then((r) =>
				r.text()
			);
		}

		localStorage.setItem('meta', meta);
		let meth = JSON.parse(meta);
		document.documentElement.setAttribute(
			'data-theme',
			document.documentElement.getAttribute('data-theme') || meth.theme || 'light'
		);
		metaStore.set(meth);
		title = meth.name;
		description = meth.description;
		return meth;
	}
	onMount(async () => await getMeta());
</script>

<SvelteSeo title={title || 'Darkvault'} description={description || 'A simple file store'} />

<slot />
