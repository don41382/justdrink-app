<script lang="ts">

    import {fade} from 'svelte/transition';
    import {events} from "../../bindings";
    import {onDestroy, onMount} from "svelte";
    import type {UnlistenFn} from "@tauri-apps/api/event";
    import * as tauri_path from "@tauri-apps/api/path";
    import {convertFileSrc} from "@tauri-apps/api/core";
    import {info} from "@tauri-apps/plugin-log";

    let eventTickerUnListen: UnlistenFn;
    let countdown = 0;

    let icon_path: string;

    onMount(async () => {
        let resource_dir = await tauri_path.resourceDir();
        icon_path = convertFileSrc(`${resource_dir}/icons/128x128.png`);
        eventTickerUnListen = await events.countdownEvent.listen(async ({payload}) => {
            if (payload.status.Active) {
                countdown = payload.status.Active;
            } else {
                countdown = 0;
            }
        })

    });

    onDestroy(async () => {
        eventTickerUnListen();
    })
</script>

{#if countdown > 0}
    <div class="bg-gray-300 w-fit h-fit rounded-br-xl rounded-t-xl border-gray-400 border-2 p-3 space-x-2 items-center flex"
         in:fade={{ duration: 500 }}>
        <img class="h-14 w-14" src="{icon_path}" alt="mm">
        <div>
            <p class="text-m font-normal">Session starts in <span class="font-medium">{countdown} seconds</span></p>
            <p class="font-thin">Shake your mouse, to skip.</p>
        </div>
    </div>
{/if}