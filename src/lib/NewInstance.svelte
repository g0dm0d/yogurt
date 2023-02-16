<script>
    import { fade } from "svelte/transition";
    import { invoke } from "@tauri-apps/api/tauri";

    import VersionList from "./dropdown.svelte";

    import { selectedVersion } from "./dropdown.svelte";

    function overlay_click(e) {
        if ("close" in e.target.dataset) show = false;
    }

    let name = "";
    let test = "";

    export let show = false;

    import * as tauri from "@tauri-apps/api";

    async function get_minecraft() {
        try {
            const minecraft = await tauri.invoke("get_minecraft", {
                url: selectedVersion,
            });
            console.log(minecraft);
        } catch (error) {
            console.error(error);
        }
    }
</script>

{#if show}
    <div>
        <div
            class="modal-overlay"
            data-close
            on:click={overlay_click}
            transition:fade={{ duration: 150 }}
        >
            <div class="modal-container">
                <main>
                    <input
                        id="instance-name"
                        placeholder="Enter a name..."
                        bind:value={name}
                    />
                    <VersionList />
                </main>
                <div style="display: flex; justify-content: center; ">
                    <button on:click={get_minecraft}>Create</button>
                </div>
                <p>{test}</p>
            </div>
        </div>
    </div>
{/if}

<style>
    main {
        padding: 0.5rem;
    }
</style>
