<script lang="ts">

    import {onDestroy, onMount} from "svelte";
    import {fitAndShowWindow} from "../../helper";
    import Icon from "@iconify/svelte";
    import * as tauri_path from "@tauri-apps/api/path";
    import {convertFileSrc} from "@tauri-apps/api/core";
    import {commands, events, type TimerStatus} from "../../bindings";
    import type {UnlistenFn} from "@tauri-apps/api/event";
    import {getCurrentWindow} from "@tauri-apps/api/window";

    let contentDiv: HTMLDivElement
    let countdownUnlistenFn: UnlistenFn;

    let icon_path = $state("");
    let countdown = $state({
        time: '',
        pause: false
    });

    $effect.pre(() => {
        tauri_path.resourceDir().then(async resource_dir => {
            icon_path = convertFileSrc(`${resource_dir}/icons/128x128.png`);
        });
        updateTimer();
    });


    onMount(async () => {
        countdownUnlistenFn = await events.countdownEvent.listen(async (data) => {
            countdown.time = formatTime(getSeconds(data.payload.status));
            countdown.pause = isPause(data.payload.status);
        });

        await fitAndShowWindow(contentDiv);
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
    }

    async function startSession() {
        await commands.startSession();
    }

    async function toggleTimer() {
        await commands.toggleTimer();
        await updateTimer();
    }

    async function addTime() {
        await commands.timerChange({
            Add: 5
        })
        await updateTimer();
    }

    async function removeTime() {
        await commands.timerChange({
            Remove: 5
        })
        await updateTimer();
    }

</script>

<div bind:this={contentDiv}
     class="flex flex-col items-center space-y-6 w-full h-full max-w-md bg-white px-8 py-8 rounded-2xl shadow-lg">
    <!-- Header with Icon and Title -->
    <div class="flex items-center space-x-3">
        <img alt="mm" class="w-8 h-8" src="{icon_path}">
        <p class="text-xl font-semibold text-left">Motion Minute</p>
    </div>

    <!-- Timer Section -->
    <div class="flex flex-col items-center w-full text-center space-y-4 p-6 bg-mm-blue-50/20 rounded-lg">
        <div class="text-2xl font-light">Next Motion</div>
        <div class="text-6xl font-bold">{countdown.time}</div>
        <div class="flex justify-center items-center space-x-4">
            {#if !countdown.pause}
                <button onclick={async () => await removeTime()}>
                    <Icon class="w-6 h-6 text-gray-700 hover:bg-mm-green hover:text-white rounded-2xl size-20"
                          icon="mdi-light:minus"/>
                </button>
            {/if}
            <button onclick={async () => await toggleTimer()}>
                {#if countdown.pause}
                    <Icon class="w-8 h-8 text-gray-700" icon="iconoir:play"/>
                {:else}
                    <Icon class="w-8 h-8 text-gray-700" icon="iconoir:pause"/>
                {/if}
            </button>
            {#if !countdown.pause}
                <button onclick={async () => await addTime()}>
                    <Icon class="w-6 h-6 text-gray-700 hover:bg-mm-green hover:text-white rounded-2xl size-20"
                          icon="mdi-light:plus"/>
                </button>
            {/if}
        </div>
    </div>

    <!-- Action Buttons -->
    <div class="flex space-x-4 w-full justify-center">
        <button class="flex flex-col items-center justify-center cursor-pointer hover:bg-mm-green hover:text-white size-20 rounded-2xl p-2 transition-all duration-200"
                onclick={async () => { await close() }}>
            <Icon class="w-10 h-10" icon="iconoir:xmark"/>
            <span class="mt-1 text-sm font-light">Hide</span>
        </button>
        <button class="flex flex-col items-center justify-center cursor-pointer hover:bg-mm-green hover:text-white size-20 rounded-2xl p-2 transition-all duration-200"
                onclick={async () => { await openSettings()}}>
            <Icon class="w-10 h-10" icon="mdi-light:settings"/>
            <span class="mt-1 text-sm font-light">Settings</span>
        </button>
        <button class="flex flex-col items-center justify-center cursor-pointer hover:bg-mm-green hover:text-white size-20 rounded-2xl p-2 transition-all duration-200"
                onclick={async () => { await startSession() }}>
            <Icon class="w-10 h-10" icon="iconoir:play-solid"/>
            <span class="mt-1 text-sm font-light">Start</span>
        </button>
    </div>
</div>