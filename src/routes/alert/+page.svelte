<script lang="ts">
    import Icon from "@iconify/svelte";
    import {type AlertEvent, commands, events} from "../../bindings";
    import {onMount} from "svelte";
    import {getCurrentWindow} from "@tauri-apps/api/window";
    import {error, info} from "@tauri-apps/plugin-log";

    let alert: AlertEvent = {
        title: "Unknown error",
        message: "An unknown error has occurred.",
    };

    async function init() {
        await info("alert window is mounted");
        await events.alertEvent.listen(async ({payload}) => {
            alert = payload;
            await getCurrentWindow().show();
            await getCurrentWindow().setFocus();
        })
    }

    onMount(async () => {
        await init().catch((e) => {
            error(`error during init: ${e}`)
        })
    });

    function closeAndSend() {
        commands.closeErrorAndSend(alert.title + "," + alert.message)
    }

    function close() {
        commands.closeErrorWindow();
    }
</script>


<div class="h-screen p-4 text-blue-800 border border-mm-blue rounded-lg bg-blue-50 dark:bg-mm-green-800 dark:text-mm-blue-400 dark:border-mm-blue-800 overflow-y-hidden flex flex-col"
     id="alert" role="alert">
    <div class="flex-none flex items-center">
        <Icon class="mr-2" height="24" icon="mdi-light:alert"/>
        <h3 class="text-lg font-medium">{alert.title}</h3>
    </div>
    <div class="flex-grow mt-2 mb-4 text-sm">
        {alert.message}
    </div>
    <div class="flex-none">
        <button class="text-white bg-blue-800 hover:bg-blue-900 focus:ring-4 focus:outline-none focus:ring-blue-200 font-medium rounded-lg text-xs px-3 py-1.5 me-2 text-center inline-flex items-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                on:click={closeAndSend} type="button">
            <Icon height="24" icon="mdi-light:send-rounded"/>
            Send Error
        </button>
        <button aria-label="Close"
                class="text-blue-800 bg-transparent border border-blue-800 hover:bg-blue-900 hover:text-white focus:ring-4 focus:outline-none focus:ring-blue-200 font-medium rounded-lg text-xs px-3 py-1.5 text-center dark:hover:bg-blue-600 dark:border-blue-600 dark:text-blue-400 dark:hover:text-white dark:focus:ring-blue-800"
                data-dismiss-target="#alert-additional-content-1" on:click={close} type="button">
            Dismiss
        </button>
    </div>
</div>