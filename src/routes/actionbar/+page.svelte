<script lang="ts">

    import {onDestroy, onMount} from "svelte";
    import Icon from "@iconify/svelte";
    import {commands, events, type TimerStatus} from "../../bindings";
    import type {UnlistenFn} from "@tauri-apps/api/event";
    import {getCurrentWindow} from "@tauri-apps/api/window";
    import AutoSize from "../AutoSize.svelte";

    let countdownUnlistenFn: UnlistenFn;


    let {data} = $props();
    let iconPath = $state(data.iconPath);

    let ready = $state(false);

    let countdown = $state({
        time: '',
        pause: false
    });

    onMount(async () => {
        countdownUnlistenFn = await events.countdownEvent.listen(async (data) => {
            countdown.time = formatTime(getSeconds(data.payload.status));
            countdown.pause = isPause(data.payload.status);
            ready = true;
        });
    })


    onDestroy(async () => {
        countdownUnlistenFn();
    });

    async function updateTimer() {
        await commands.getCurrentTimerStatus().then(status => {
            countdown.time = formatTime(getSeconds(status));
            countdown.pause = isPause(status);
        });
    }

    function getSeconds(timeStatus: TimerStatus): number {
        switch (typeof timeStatus) {
            case "string":
                return timeStatus === "Finished" ? 0 : 0;

            case "object":
                if ("NotStarted" in timeStatus) {
                    return timeStatus.NotStarted;
                }
                if ("Active" in timeStatus) {
                    return timeStatus.Active;
                }
                if ("Paused" in timeStatus) {
                    return timeStatus.Paused[1];
                }
                break;

            default:
                return 0;
        }

        return 0;
    }

    function isPause(timeStatus: TimerStatus): boolean {
        switch (typeof timeStatus) {
            case "string":
                return false;

            case "object":
                if ("NotStarted" in timeStatus) {
                    return true;
                }
                if ("Active" in timeStatus) {
                    return false;
                }
                if ("Paused" in timeStatus) {
                    return true;
                }
                break;

            default:
                return false;
        }
        return false;
    }

    function formatTime(seconds: number): string {
        const hours = Math.floor(seconds / 3600);
        const minutes = Math.floor((seconds % 3600) / 60);
        const secs = seconds % 60;

        if (seconds < (60 * 60)) { // 90 minutes * 60 seconds
            // Format as MM:SS for times less than 90 minutes
            return `${String(minutes + hours * 60).padStart(2, '0')}:${String(secs).padStart(2, '0')}`;
        } else {
            // Format as H:MM:SS for times 90 minutes or more
            return `${hours}:${String(minutes).padStart(2, '0')}:${String(secs).padStart(2, '0')}`;
        }
    }

    async function close() {
        await getCurrentWindow().close();
    }

    async function openSettings() {
        await commands.openSettings();
        await getCurrentWindow().close();
    }

    async function startSession() {
        await commands.startSession();
    }

    async function toggleTimer() {
        await commands.toggleTimer();
        await updateTimer();
    }

</script>

<AutoSize class="flex flex-col items-center space-y-6 w-fit h-full max-w-md bg-white px-8 py-8 rounded-2xl shadow-lg"
          ready={ready}>
    <!-- Header with Icon and Title -->
    <div class="flex items-center space-x-3">
        <div class="flex items-center space-x-2 mr-16">
            <img alt="mm" class="w-8 h-8" src="{iconPath}">
            <p class="text-xl font-semibold text-left whitespace-nowrap">Motion Minute</p>
        </div>
        <div class="flex space-x-2 justify-end">
            <button class="flex flex-col items-center justify-center cursor-pointer rounded-full hover:bg-mm-green-100 hover:text-white p-1"
                    onclick={async () => { await openSettings()}}>
                <Icon class="size-6" icon="mdi-light:settings"/>
            </button>
            <button class="flex flex-col items-center justify-center cursor-pointer rounded-full hover:bg-mm-green-100 hover:text-white p-1"
                    onclick={async () => { await close() }}>
                <Icon class="size-6" icon="iconoir:xmark"/>
            </button>

        </div>
    </div>
    <!-- Timer Section -->
    <div class="flex flex-col w-full text-center bg-mm-blue-50/20 rounded-lg">
        <div class="p-6">
            <div class="text-2xl font-light">next motion in</div>
            <div class="text-6xl font-bold">{countdown.time}</div>
        </div>
        <div class="w-full border-b-2 border-white/70"></div>
        <div class="flex items-stretch w-full rounded-b-2xl">
            <button class="w-1/2 flex flex-col items-center justify-center cursor-pointer hover:bg-gray-600 hover:text-white p-6 rounded-bl-2xl"
                    onclick={async () => await toggleTimer()}>
                {#if countdown.pause}
                    <Icon class="w-8 h-8" icon="iconoir:play"/>
                    <span class="text-lg font-light tracking-wide">Run</span>
                {:else}
                    <Icon class="w-8 h-8" icon="iconoir:pause"/>
                    <span class="text-lg font-light tracking-wide">Pause</span>
                {/if}
            </button>
            <div class="border-l-2 border-white/70"></div>
            <button class="w-1/2 flex flex-col items-center justify-center cursor-pointer hover:bg-primary hover:text-white p-6 rounded-br-2xl"
                    onclick={async () => { await startSession() }}>
                <Icon class="w-8 h-8" icon="hugeicons:workout-warm-up"/>
                <span class="text-lg font-light tracking-wide">Start Now</span>
            </button>
        </div>
    </div>
</AutoSize>