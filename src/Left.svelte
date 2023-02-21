<script>
    import Image from "./lib/image.svelte";
    import NewInstance from "./lib/NewInstance.svelte";
    import NewAccount from "./lib/addAccount.svelte";

    let show_instance = false;
    let show_accounts = false;
</script>

<NewInstance bind:show={show_instance} />
<NewAccount bind:show={show_accounts} />
<main>
    <button class="invincible-btn" on:click={() => {show_accounts = true;}}>
    <div class="user">
        <Image
            src="https://mc-heads.net/avatar/06a6aad7425e47ed9a0a7ae7606d74599/100"
        />
        <p>USERNAME</p>
    </div>
    </button>
    <button on:click={() => {show_instance = true;}}>New instance</button>
</main>

