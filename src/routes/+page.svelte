<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import type { EzerJson } from '$lib/types';

	// The users who's repos will be shown
	const users = ['realTristan', 'Simpson-Computer-Technologies-Research'];

	// The data that will be shown
	let currentTab: string = 'Repositories';
	let data: any = {
		Repositories: []
	};

	// Get the github repos of the users
	const getRepos = async (user: string) => {
		const res = await fetch(`https://api.github.com/users/${user}/repos?per_page=100`);
		return res.ok ? await res.json() : [];
	};

	// Get the ezer.json from the repo
	const getEzerJson = async (user: string, repo: string): Promise<EzerJson | null> => {
		const res = await fetch(`https://raw.githubusercontent.com/${user}/${repo}/main/ezer.json`);
		return res.ok ? await res.json() : null;
	};

	// On mount, for every user, get the repos, ezer.json and README.md
	onMount(async () => {
		const cachedData: string = await invoke('read_from_json', { file: 'data.json' });
		data = cachedData ? JSON.parse(cachedData) : data;

		for (const user of users) {
			const repos = await getRepos(user);

			for (const repo of repos) {
				const ezerJson = await getEzerJson(user, repo.name);

				if (!ezerJson) {
					const cloneUrl: string = `git clone https://github.com/${user}/${repo.name}.git`;
					const cloneData = {
						...repo,
						install: {
							mac: cloneUrl,
							windows: cloneUrl
						}
					};

					data.Repositories.push(cloneData);

					continue;
				}

				const tabName: string = ezerJson.tab;

				if (!data[tabName]) data[tabName] = [];

				data[tabName].push({
					...repo,
					install: ezerJson.install
				});
			}
		}

		// Save the data to a json file
		await invoke('write_to_json', { file: 'data.json', data: JSON.stringify(data) });
	});
</script>

<main class="p-10 flex flex-col">
	<h1 class="font-bold text-5xl">Installer</h1>
	<div class="flex flex-row gap-4">
		{#each Object.keys(data) as tab}
			<button
				on:click={() => {
					currentTab = tab;
				}}
				class={`px-10 py-3 ${
					currentTab === tab ? 'bg-indigo-500 text-white' : 'bg-indigo-400'
				} hover:bg-indigo-600 my-4 rounded-lg text-white font-semibold tracking-wider`}
				>{tab}</button
			>
		{/each}
	</div>

	<div class="flex flex-wrap gap-4">
		{#each data[currentTab] as repo}
			<a
				href={`/install?data=${btoa(JSON.stringify(repo))}`}
				class="bg-slate-900 hover:bg-slate-950 hover:scale-110 py-3 px-10 rounded-lg text-white font-semibold tracking-wider"
				>{repo.name}</a
			>
		{/each}
	</div>
</main>
