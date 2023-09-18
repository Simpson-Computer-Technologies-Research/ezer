<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import type { EzerJson } from "$lib/types";

  // The data that will be shown
  let currentTab: string = "Repositories";
  let data: any = {
    Repositories: [],
  };

  /**
   * Get the repos of a user
   * @param user The user to get the repos of
   */
  const getRepos = async (user: string): Promise<any> => {
    const res = await fetch(
      `https://api.github.com/users/${user}/repos?per_page=100`
    );
    return res.ok ? await res.json() : [];
  };

  /**
   * Append a basic repo in the data
   * @param user The user of the repo
   * @param repo The repo to append
   */
  const appendBasicRepo = (user: string, repo: any): void => {
    const cloneUrl: string = `git clone https://github.com/${user}/${repo.name}.git`;

    data.Repositories = [
      ...data.Repositories,
      {
        ...repo,
        install: {
          select_dir: false,
          mac: [cloneUrl],
          windows: [cloneUrl],
        },
      },
    ];
  };

  /**
   * Append an ezer repo in the data
   * @param repo The repo to append
   * @param ezerJson The ezer.json of the repo
   */
  const appendEzerRepo = (repo: any, ezerJson: EzerJson): void => {
    const tabName: string = ezerJson.tab;

    // Create the tab if it doesn't exist
    if (!data[tabName]) data[tabName] = [];

    // Append the repo to the tab
    data[tabName] = [
      ...data[tabName],
      {
        ...repo,
        install: ezerJson.install,
      },
    ];
  };

  // The users who's repos will be shown
  const users = ["realTristan", "Simpson-Computer-Technologies-Research"];

  onMount(async () => {
    for (const user of users) {
      const repos: any[] = await getRepos(user);

      for (const repo of repos) {
        const res: Response = await fetch(
          `https://raw.githubusercontent.com/${user}/${repo.name}/main/ezer.json`
        );

        if (!res.ok) {
          appendBasicRepo(user, repo);
          continue;
        }

        const json = await res.json();
        appendEzerRepo(repo, json);
      }
    }
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
          currentTab === tab ? "bg-indigo-500 text-white" : "bg-indigo-400"
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
