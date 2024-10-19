<script lang="ts">
    import Icon from "@iconify/svelte";
    import {commands} from "../../bindings";
    import {onMount} from "svelte";
    import {fitAndShowWindow} from "../../helper";

    const params = new URLSearchParams(window.location.search);

    let title = params.get("title") ?? "Unknown Error"
    let message = params.get("message") ?? "Sorry, something went wrong."

    let content: HTMLDivElement;

    onMount(async () => {
        await fitAndShowWindow(content);
    })

    function close() {
        commands.closeErrorWindow();
    }
</script>


<div bind:this={content} class="w-[500px] p-6 text-blue-800 border border-mm-blue rounded-lg bg-blue-50 dark:bg-mm-green-800 dark:text-mm-blue-400 dark:border-mm-blue-800 overflow-y-hidden flex flex-col"
     id="alert" role="alert">
    <div class="flex-none flex items-center">
        <Icon class="mr-2" height="24" icon="mdi-light:alert"/>
        <h3 class="text-lg font-medium">{title}</h3>
    </div>
    <div class="flex-grow mt-2 mb-8 text-sm">
        {message}
    </div>
    <div class="flex-none">
        <button aria-label="Close"
                class="text-blue-800 bg-transparent border border-blue-800 hover:bg-blue-900 hover:text-white focus:ring-4 focus:outline-none focus:ring-blue-200 font-medium rounded-lg text-xs px-3 py-1.5 text-center dark:hover:bg-blue-600 dark:border-blue-600 dark:text-blue-400 dark:hover:text-white dark:focus:ring-blue-800"
                data-dismiss-target="#alert-additional-content-1" on:click={close} type="button">
            Dismiss
        </button>
    </div>
</div>