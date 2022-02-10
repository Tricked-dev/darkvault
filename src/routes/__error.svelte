<script context="module">
	export function load({ error, status }) {
		return {
			props: {
				status: status,
				title: `${status}: ${error.message}`,
				stack: error.stack
			}
		};
	}
</script>

<script>
	export const prerender = true;
	export const hydrate = false;
	export const router = false;
	export let title;
	export let status;
	export let stack;
</script>

<div class="bg-accent w-full p-4">
	<a class="link-primary" href="/">Return</a>
</div>

<div class="prose text-base-content p-2 hover:prose-a:text-accent-focus mx-auto">
	{#if status === 404}
		<h1>404 not found</h1>
		<p>Tried finding the right url :D</p>
	{:else}
		<h1>{title}</h1>
		<p>
			If this is undefined behaviour please take your time to <a
				href="https://github.com/Tricked-dev/darkvault"
				rel="external"
				target="_blank"
				class="link">report your issue</a
			>
		</p>
		<details>
			<summary>Error stack</summary>
			<pre><code>{stack}</code></pre>
		</details>
	{/if}
</div>
