<script>
    import { fade } from "svelte/transition";
    import { invoke } from "@tauri-apps/api/tauri";
    import VersionList from "./dropdown.svelte";

    import { playNotificationSound } from "./NotifySound" // not work, tauri block sound

    function overlay_click(e) {
        if ("close" in e.target.dataset) show = false;
    }

    let name = "";

    export let show = false;

    import * as tauri from "@tauri-apps/api";
    import { toast } from '@zerodevx/svelte-toast'
    
    async function get_minecraft() {
        try {
            //await tauri.invoke("get_minecraft", {
            //    url: selectedVersion,
            //});
            playNotificationSound();
            toast.push('Minecraft downloaded successfully!', {
                theme: {
                  '--toastColor': 'mintcream',
                  '--toastBackground': 'rgba(72,187,120,0.9)',
                  '--toastBarBackground': '#2F855A'
                }
            })
        } catch (error) {
            console.error(error);
        }
    }
</script>

{#if show}
    <div>
        <!-- svelte-ignore a11y-click-events-have-key-events -->
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
            </div>
        </div>
    </div>
{/if}

<style>
    main {
        padding: 0.5rem;
    }
</style>
