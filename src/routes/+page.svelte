<script lang="ts">
    import {invoke} from "@tauri-apps/api/core";
    import Session from "./Session.svelte";

    let number: number = 42;
    let greetMsg = "";

    async function greet() {
        greetMsg = await invoke("answer", {number});
    }

    function closeApp() {
        invoke("close_app");
    }
</script>

<div class="flex items-center justify-center h-screen gradient-background">
    <div class="text-center">
        <h1 class="text-4xl">Welcome to Tauri!</h1>
        <Session/>

        <form class="row mt-4" on:submit|preventDefault={greet}>
            <input id="greet-input" type="number" placeholder="Enter a name..." bind:value={number} class="mr-2"/>
            <button type="submit" class="btn">Greet</button>
        </form>

        <button on:click={closeApp} class="btn mb-4">Close the app</button>

        <p>{greetMsg}</p>
    </div>
</div>