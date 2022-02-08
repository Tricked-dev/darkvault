<script lang="ts">
	export let data: any[];
	export let name: string | undefined;
	let found = data;
	import {
		FolderIcon,
		FileIcon,
		CornerDownLeftIcon,
		BoxIcon,
		SearchIcon
	} from 'svelte-feather-icons';
	import SvelteMarkdown from 'svelte-markdown';
	// @ts-ignore
	import { formatByteSize } from '@theredhead/formatbytesize';
	import { onMount } from 'svelte';

	const here = location.href.split('/').slice(3);
	const parts = [{ text: name || 'Index', link: '/' }];

	for (let i = 0; i < here.length; i++) {
		const part = here[i];
		const text = decodeURIComponent(part).split('.')[0];
		const link = '/' + here.slice(0, i + 1).join('/');
		parts.push({ text: text, link: link });
	}
	let readme;
	let readmeFile;
	onMount(async () => {
		readmeFile = data?.find(
			(x) => x.path.split('/').at(-1).toLowerCase().split('.')[0] == 'readme'
		);
		let file = await fetch(
			`${import.meta.env['VITE_SERVER_URL'] || ''}/api/download/${readmeFile.path}`
		).then((r) => r.text());
		readme = file;
	});
</script>

<div class="bg-accent w-full p-4">
	<div class="flex gap-2 md:flex-row flex-col">
		<div class="flex md:justify-center">
			<BoxIcon size="28" />
			<p class="text-xl text-primary align-middle">
				{name || 'File repostitory'}
			</p>
		</div>
		<div
			class="ml-auto p-2 text-primary align-middle text-lg rounded-xl flex bg-primary-content hover:outline-base-100 outline-1 gap-2 focus:outline"
		>
			<SearchIcon size="26" />
			<input
				class="outline-none bg-inherit text-inherit"
				placeholder="search"
				on:input={(e) => {
					// @ts-ignore
					if (e.target.value == '') found = data;
					// @ts-ignore
					found = [...data].filter((x) => x.path.split('/').at(-1).includes(e.target.value));
				}}
			/>
		</div>
	</div>
	<div class="flex font-mono text-neutral-content text-lg">
		<div class="breadcrumbs">
			<ul>
				{#each parts as part}
					<li>
						<a rel="external" href={part.link}>{part.text}</a>
					</li>
				{/each}
			</ul>
		</div>
	</div>
</div>

<div class="mx-auto md:w-[80rem] p-4">
	{#if readme}
		<details class="bg-base-300">
			<summary class="w-full p-2 bg-base-200 block hover:cursor-pointer">
				<h1>{readmeFile.path.split('/').at(-1)}</h1>
			</summary>
			<hr />
			<div class="prose text-base-content p-2">
				<SvelteMarkdown source={readme} />
			</div>
		</details>
	{/if}
	{#if data}
		<table class="w-full">
			<tr class="w-full text-left">
				<th>File Name</th>
				<th>Size</th>
				<th>Modified</th>
			</tr>

			{#each found as path}
				<tr class="w-full">
					<td class="flex gap-2 pr-2">
						{#if path.directory}
							<FolderIcon size="24" class="h-auto w-auto" />
						{:else}
							<FileIcon size="24" class="h-auto w-auto" />
						{/if}<a
							href={path.directory
								? `/${path.path}`
								: `${import.meta.env['VITE_SERVER_URL'] || ''}/api/download/${path.path}`}
							download={path.directory ? undefined : path.path.split('/').at(-1)}
							class="text-lg truncate text-info hover:text-primary-focus"
							rel="external">{path.path.split('/').at(-1)}</a
						></td
					>
					<td
						><p class="textarea-secondary">
							{#if !path.directory}
								{formatByteSize(path.size)}
							{:else}
								-
							{/if}
						</p>
					</td>
					<td><pre class="text-accent">{path.modified}</pre></td>
				</tr>
			{/each}
		</table>
		{#if window.location.pathname !== '/'}
			<a
				href={parts.at(-2).link}
				rel="external"
				class="flex gap-2 py-2 hover:bg-accent-focus rounded-md duration-75"
			>
				<CornerDownLeftIcon size="24" />
				<p>...</p>
			</a>
		{/if}
	{/if}
</div>
