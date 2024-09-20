<script lang="ts">

    import {fade} from 'svelte/transition';
    import {events} from "../../bindings";
    import {onDestroy, onMount} from "svelte";
    import type {UnlistenFn} from "@tauri-apps/api/event";

    let eventTickerUnListen: UnlistenFn;
    let countdown = 0;

    onMount(async () => {
        eventTickerUnListen = await events.countdownEvent.listen(async ({payload}) => {
            switch (payload.status) {
                case "Start":
                    countdown = 0;
                    break;
                case "Finished":
                    countdown = 0;
                    break;
                default:
                    countdown = payload.status.RunningSeconds.countdown_seconds;
                    break;
            }
        })

    });

    onDestroy(async () => {
        eventTickerUnListen();
    })
</script>

{#if countdown > 0}
    <div class="bg-gray-300 w-[250px] h-fit rounded-br-xl rounded-t-xl border-gray-400 border-2 p-2 px-4 flex flex-col"
         in:fade={{ duration: 500 }}>
        <p class="text-m font-normal">Session starts in <span class="font-medium">{countdown} seconds</span></p>
        <p class="font-thin">Shake your mouse, to skip.</p>
    </div>
{/if}