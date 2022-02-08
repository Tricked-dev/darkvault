<script context="module">
	import FileLayout from '$lib/components/FileLayout.svelte';
	/** @type {import('@sveltejs/kit').Load} */
	export async function load({ params, fetch, session, stuff }) {
		const response = await window.fetch(
			`${import.meta.env['VITE_SERVER_URL'] || ''}/api/list?path=${params.files}`
		);

		let data = response.ok && (await response.json());
		document.documentElement.setAttribute(
			'data-theme',
			response.headers.get('X-vault-theme') || 'light'
		);

		return {
			status: response.status,
			props: {
				name: response.headers.get('X-vault-name'),
				data
			}
		};
	}
</script>

<script>
	export let data;
	export let name;
</script>

<FileLayout {data} {name} />
