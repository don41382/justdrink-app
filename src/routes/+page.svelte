<script lang="ts">
    import {invoke} from "@tauri-apps/api/core";
    import {info} from '@tauri-apps/plugin-log';

    let number: number = 42;
    let greetMsg = "";

    async function greet() {
        await info('sending a greeting');
        greetMsg = await invoke("answer", {number});
    }

    function closeApp() {
        invoke("close_app");
    }


    // allows no context menu
    // document.addEventListener('contextmenu', event => event.preventDefault());
</script>

<div class="flex items-center justify-center h-screen gradient-background select-none cursor-default">
    <div class="text-center">
        <h1 class="text-4xl mb-8">Minute Motion</h1>

        <button on:click={closeApp}
                class="btn mb-4 bg-blue-500 bg-opacity-50 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-lg shadow-md border border-white">
            Finished
        </button>

        <p>{greetMsg}</p>
    </div>
</div>