<script lang="ts">

    import {onDestroy, onMount} from "svelte";
    import {fitAndShowWindow} from "../../helper";
    import Icon from "@iconify/svelte";
    import * as tauri_path from "@tauri-apps/api/path";
    import {convertFileSrc} from "@tauri-apps/api/core";
    import {commands, events, type TimerStatus} from "../../bindings";
    import {info} from "@tauri-apps/plugin-log";
    import type {UnlistenFn} from "@tauri-apps/api/event";
    import {getCurrentWindow} from "@tauri-apps/api/window";

    let contentDiv: HTMLDivElement
    let icon_path: string;

    let countdownUnlistenFn: UnlistenFn;

    let time: string;
    let pause: boolean = true;

    onMount(async () => {
        let resource_dir = await tauri_path.resourceDir();
        icon_path = convertFileSrc(`${resource_dir}/icons/128x128.png`);

        let status = await commands.getCurrentTimerStatus();
        time = formatTime(getSeconds(status));
        pause = isPause(status);

        countdownUnlistenFn = await events.countdownEvent.listen(async (data) => {
            time = formatTime(getSeconds(data.payload.status));
            pause = isPause(data.payload.status);
        });

        await fitAndShowWindow(contentDiv)
    })

    onDestroy(async () => {
        countdownUnlistenFn();
    });

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
        <div class="text-6xl font-bold">{time}</div>
        <div class="flex justify-center items-center space-x-4">
            <Icon class="w-6 h-6 text-gray-700 hover:bg-mm-green hover:text-white rounded-2xl size-20"
                  icon="mdi-light:minus"/>
            {#if pause}
                <Icon class="w-8 h-8 text-gray-700" icon="iconoir:play"/>
            {:else}
                <Icon class="w-8 h-8 text-gray-700" icon="iconoir:pause"/>
            {/if}
            <Icon class="w-6 h-6 text-gray-700 hover:bg-mm-green hover:text-white rounded-2xl size-20"
                  icon="mdi-light:plus"/>
        </div>
    </div>

    <!-- Action Buttons -->
    <div class="flex space-x-4 w-full justify-center">
        <button class="flex flex-col items-center justify-center cursor-pointer hover:bg-mm-green hover:text-white size-20 rounded-2xl p-2 transition-all duration-200"
                on:click={async () => { await close() }}>
            <Icon class="w-10 h-10 text-gray-700 hover:text-white" icon="iconoir:xmark"/>
            <span class="mt-1 text-sm font-light text-gray-700  hover:text-white">Hide</span>
        </button>
        <button class="flex flex-col items-center justify-center cursor-pointer hover:bg-mm-green hover:text-white size-20 rounded-2xl p-2 transition-all duration-200"
                on:click={async () => { await openSettings()}}>
            <Icon class="w-10 h-10" icon="mdi-light:settings"/>
            <span class="mt-1 text-sm font-light">Settings</span>
        </button>
        <button class="flex flex-col items-center justify-center cursor-pointer hover:bg-mm-green hover:text-white size-20 rounded-2xl p-2 transition-all duration-200"
                on:click={async () => { await startSession() }}>
            <Icon class="w-10 h-10" icon="iconoir:play-solid"/>
            <span class="mt-1 text-sm font-light">Start</span>
        </button>
    </div>
</div>