<script context="module" lang="ts">
  type Version = {
    id: string;
    url: string;
  };

  let versions: Version[] = [];
  export let selectedVersion = "";

  async function fetchVersions(): Promise<Version[]> {
    return fetch(
      "https://launchermeta.mojang.com/mc/game/version_manifest_v2.json"
    )
      .then((response) => response.json())
      .then((data) => {
        const versions = data.versions;
        const versionObjs: Version[] = [];

        for (let i = 0; i < versions.length; i++) {
          const version = versions[i];
          const versionObj: Version = { id: version.id, url: version.url };
          versionObjs.push(versionObj);
        }

        return versionObjs;
      })
      .catch((error) => {
        console.error(error);
        return Promise.reject(error);
      });
  }

  fetchVersions().then((data) => {
    versions = data;
    selectedVersion = versions[0].url;
  });
</script>

<select bind:value={selectedVersion}>
  {#each versions as version}
    <option value={version.url}>{version.id}</option>
  {/each}
</select>
