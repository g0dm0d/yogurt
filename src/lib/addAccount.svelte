<script>
    import { fade } from "svelte/transition";
    import { invoke } from "@tauri-apps/api/tauri";

    import { playNotificationSound } from "./NotifySound" // not work, tauri block sound

    function overlay_click(e) {
        if ("close" in e.target.dataset) show = false;
    }

    let name = "";

    export let show = false;

    import * as tauri from "@tauri-apps/api";
    import { toast } from '@zerodevx/svelte-toast'

    import { open } from '@tauri-apps/api/shell';
    const signin_url = "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize?client_id=d8e1d9bf-287f-4773-a176-e012722257f4&response_type=code&redirect_uri=http://localhost:9397&scope=XboxLive.signin%20offline_access&state=NOT_NEEDED"
    const openLogin = () => {
        open(signin_url);
    }

    async function add_account() {
        try {
            openLogin();
            await tauri.invoke("add_account");
            playNotificationSound();
        } catch (error) {
            console.error(error);
        }
    }

    async function get_accounts() {
        try {
            const users = await invoke("get_all_users", {});
            return users
        } catch (error) {
            console.error(error);
        }
    }
    console.log(get_accounts());
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
                <div>
                    
                </div>
                <div style="display: flex; justify-content: center; ">
                    <button on:click={add_account}>Add account</button>
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
