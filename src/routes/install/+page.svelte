<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import HomeIcon from "../../components/HomeIcon.svelte";

  // Get the repo data from the data url params
  const encodedRepoData: string | null = new URLSearchParams(
    window.location.search
  ).get("data");

  // Base64 decode the repo data
  const decodedRepoData: string = atob(encodedRepoData || "");

  // Convert the decoded repo data to a map from the json string
  const repoData: any = JSON.parse(decodedRepoData);

  let os: string = "mac";
  let cmds: any = repoData.install[os];
  let password: string = "";
  let response: string = "";
</script>

<main class="p-10 flex flex-col">
  <a
    href="/"
    class="bg-indigo-500 flex justify-center hover:bg-indigo-600 items-center rounded-lg w-12 h-12"
  >
    <HomeIcon /></a
  >

  <h1 class="font-bold text-5xl mt-6">{repoData.name}</h1>
  <p class="mt-2">{repoData.description}</p>
  <div class="flex flex-row gap-4 w-full">
    <button
      on:click={() => {
        os = "mac";
        cmds = repoData.install[os];
      }}
      class={`mt-3 px-8 py-2 w-full rounded-lg text-white font-medium text-sm tracking-wide ${
        os == "mac"
          ? "bg-indigo-700 hover:bg-indigo-600"
          : "bg-indigo-300 hover:bg-indigo-300"
      }`}>MacOS</button
    >

    <button
      on:click={() => {
        os = "windows";
        cmds = repoData.install[os];
      }}
      class={`mt-3 px-8 py-2 w-full rounded-lg text-white font-medium text-sm tracking-wide ${
        os == "windows"
          ? "bg-indigo-700 hover:bg-indigo-600"
          : "bg-indigo-200 hover:bg-indigo-300"
      }`}>Windows</button
    >
  </div>

  <!-- Input the password if os is mac -->
  {#if os == "mac"}
    <input
      type="password"
      class="mt-3 px-2 py-2 w-full rounded-lg text-slate-900 border-2 border-indigo-500 tracking-wide bg-white"
      placeholder="Enter sudo password"
      bind:value={password}
    />
  {/if}

  <button
    on:click={async () => {
      response = await invoke("install", { os, cmds, password });
    }}
    class="mt-3 px-10 py-3 bg-indigo-500 rounded-lg text-white font-medium tracking-wide hover:bg-indigo-600"
    >Install</button
  >

  <p class="m-2">{response}</p>
</main>
