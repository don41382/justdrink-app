<script lang="ts">
    import {commands} from "../../bindings";
    import {onMount} from "svelte";
    import {fitAndShowWindow} from "../../helper";
    import Alert from "../../icons/Alert.svelte";

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


<div bind:this={content}
     class="w-[500px] p-6 bg-accent border border-mm-blue rounded-lg overflow-y-hidden flex flex-col"
     id="alert" role="alert">
    <div class="flex-none flex items-center text-primary">
        <div class="size-8 mr-2">
            <Alert/>
        </div>
        <h3 class="text-lg font-medium">{title}</h3>
    </div>
    <div class="flex-grow text-gray-400 mt-2 mb-6 text-sm">
        {message}
    </div>
    <div class="flex-none">
        <button aria-label="Close"
                class="bg-primary hover:bg-primary/50 text-black py-2 rounded-md px-4"
                data-dismiss-target="#alert-additional-content-1" on:click={close} type="button">
            Dismiss
        </button>
    </div>
</div>