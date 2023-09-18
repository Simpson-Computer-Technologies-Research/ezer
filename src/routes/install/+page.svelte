<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	// Get the repo data from the data url params
	const encodedRepoData: string | null = new URLSearchParams(window.location.search).get('data');

	// Base64 decode the repo data
	const decodedRepoData: string = atob(encodedRepoData || '');

	// Convert the decoded repo data to a map from the json string
	const repoData: any = JSON.parse(decodedRepoData);

	let os: string = 'mac';
	let cmds: any = repoData.install[os];
	let response: string = '';
</script>

<main class="p-10 flex flex-col">
	<a
		href="/"
		class="bg-indigo-500 flex justify-center hover:bg-indigo-600 items-center rounded-lg w-12 h-12"
	>
		<svg
			version="1.1"
			id="Capa_1"
			xmlns="http://www.w3.org/2000/svg"
			viewBox="0 0 495.398 495.398"
			class="h-8 w-8 fill-white"
		>
			<g>
				<g>
					<g>
						<path
							d="M487.083,225.514l-75.08-75.08V63.704c0-15.682-12.708-28.391-28.413-28.391c-15.669,0-28.377,12.709-28.377,28.391
				v29.941L299.31,37.74c-27.639-27.624-75.694-27.575-103.27,0.05L8.312,225.514c-11.082,11.104-11.082,29.071,0,40.158
				c11.087,11.101,29.089,11.101,40.172,0l187.71-187.729c6.115-6.083,16.893-6.083,22.976-0.018l187.742,187.747
				c5.567,5.551,12.825,8.312,20.081,8.312c7.271,0,14.541-2.764,20.091-8.312C498.17,254.586,498.17,236.619,487.083,225.514z"
						/>
						<path
							d="M257.561,131.836c-5.454-5.451-14.285-5.451-19.723,0L72.712,296.913c-2.607,2.606-4.085,6.164-4.085,9.877v120.401
				c0,28.253,22.908,51.16,51.16,51.16h81.754v-126.61h92.299v126.61h81.755c28.251,0,51.159-22.907,51.159-51.159V306.79
				c0-3.713-1.465-7.271-4.085-9.877L257.561,131.836z"
						/>
					</g>
				</g>
			</g>
		</svg></a
	>
	<h1 class="font-bold text-5xl mt-6">{repoData.name}</h1>
	<p class="mt-2">{repoData.description}</p>
	<div class="flex flex-row gap-4 w-full">
		<button
			on:click={() => {
				os = 'mac';
				cmds = repoData.install[os];
			}}
			class={`mt-3 px-8 py-2 w-full rounded-lg text-white font-medium text-sm tracking-wide ${
				os == 'mac' ? 'bg-indigo-700 hover:bg-indigo-600' : 'bg-indigo-300 hover:bg-indigo-300'
			}`}>MacOS</button
		>
		<button
			on:click={() => {
				os = 'windows';
				cmds = repoData.install[os];
			}}
			class={`mt-3 px-8 py-2 w-full rounded-lg text-white font-medium text-sm tracking-wide ${
				os == 'windows' ? 'bg-indigo-700 hover:bg-indigo-600' : 'bg-indigo-200 hover:bg-indigo-300'
			}`}>Windows</button
		>
	</div>

	<button
		on:click={async () => (response = await invoke('install', { os, cmds }))}
		class="mt-3 px-10 py-3 bg-indigo-500 rounded-lg text-white font-medium tracking-wide hover:bg-indigo-600"
		>Install</button
	>
	<p class="m-2">{response}</p>
</main>
