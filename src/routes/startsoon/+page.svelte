<script lang="ts">

    import {commands, events} from "../../bindings";
    import {onDestroy, onMount} from "svelte";
    import {info} from "@tauri-apps/plugin-log";
    import type {UnlistenFn} from "@tauri-apps/api/event";
    import {getCurrentWindow, LogicalSize, PhysicalSize} from "@tauri-apps/api/window";

    let bodyElement: HTMLBodyElement;
    let eventTickerUnListen: UnlistenFn;
    let countdown = 0;

    info("init start soon window");

    onMount(async () => {
        await info("set size")

        eventTickerUnListen = await events.eventTicker.listen(async ({payload}) => {
            countdown = payload.countdown
        })
    });

    onDestroy(async () => {
        eventTickerUnListen();
    })
</script>

{#if countdown > 0}
    <div class="bg-gray-300 w-[250px] h-fit rounded-br-xl rounded-t-xl border-gray-400 border-2 p-2 px-4 flex flex-col" bind:this={bodyElement}>
        <p class="text-m font-normal">Session starts in <span class="font-medium">{countdown} seconds</span></p>
        <p class="font-thin">Shake your mouse, to skip.</p>
    </div>
{/if}