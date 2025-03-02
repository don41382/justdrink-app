<script lang="ts">

    import {onDestroy, onMount} from "svelte";
    import {commands, events, type TimerStatus} from "../../bindings";
    import type {UnlistenFn} from "@tauri-apps/api/event";
    import {getCurrentWindow} from "@tauri-apps/api/window";
    import AutoSize from "../AutoSize.svelte";
    import {info} from "@tauri-apps/plugin-log";
    import SettingsGear from "../../icons/SettingsGear.svelte";
    import Xmark from "../../icons/Xmark.svelte";
    import Play from "../../icons/Play.svelte";
    import Pause from "../../icons/Pause.svelte";
    import GlassEmpty from "../../icons/GlassEmpty.svelte";

    let countdownUnlistenFn: UnlistenFn;

    let {data} = $props();
    let iconPath = $state(data.iconPath);

    let ready = $state(false);

    let countdown: { time: string | undefined, pause: boolean } = $state({
        time: undefined,
        pause: false
    });

    onMount(async () => {
        await info("dashboard mounted")
        ready = true;
        countdown.time = formatTime(getSeconds(data.timerStatus))
        countdown.pause = isPause(data.timerStatus)
        countdownUnlistenFn = await events.countdownEvent.listen(async (response) => {
            countdown.time = formatTime(getSeconds(response.payload.status));
            countdown.pause = isPause(response.payload.status);
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
    }

    async function startSession() {
        await commands.startSession(null);
    }

    async function toggleTimer() {
        await commands.toggleTimer();
        await updateTimer();
    }

</script>

<AutoSize
        class="flex flex-col items-center space-y-6 w-fit h-full max-w-md bg-accent px-8 py-8 rounded-2xl shadow-lg cursor-grab active:cursor-grabbing"
        data-tauri-drag-region ready={ready}>
    <!-- Header with Icon and Title -->
    <div class="flex items-center space-x-3" data-tauri-drag-region>
        <div class="flex items-center space-x-2 mr-16 select-none" data-tauri-drag-region>
            <img alt="mm" class="w-8 h-8" data-tauri-drag-region src="{iconPath}">
            <p class="text-xl text-primary/ font-light text-left whitespace-nowrap text-primary" data-tauri-drag-region>Drink Now!</p>
        </div>
        <div class="flex space-x-2 justify-end">
            <button class="flex items-center cursor-pointer rounded-full hover:bg-gray-600 text-gray-400 hover:text-white p-1 size-8"
                    onclick={async () => { await openSettings()}}>
                <SettingsGear/>
            </button>
            <button class="flex items-center cursor-pointer rounded-full hover:bg-gray-600 text-gray-400 hover:text-white p-1 size-8"
                    onclick={async () => { await close() }}>
                <Xmark/>
            </button>

        </div>
    </div>
    <!-- Timer Section -->
    <div class="flex flex-col w-full text-center text-black bg-gray-200/80 rounded-2xl cursor-default">
        <div class="p-6">
            <div class="text-2xl font-light text-accent">next reminder in</div>
            <div class="text-6xl font-bold text-black">{countdown.time}</div>
        </div>
        <div class="w-full border-b-2 border-white/70"></div>
        <div class="flex items-stretc w-full rounded-b-2xl">
            <button class="w-1/2 flex flex-col items-center justify-center cursor-pointer hover:bg-primary hover:text-white p-6 rounded-bl-2xl"
                    onclick={async () => await toggleTimer()}>
                {#if countdown.pause}
                    <div class="size-8">
                        <Play/>
                    </div>
                    <span class="text-lg font-light mt-1 tracking-wide">Run</span>
                {:else}
                    <div class="size-8">
                        <Pause/>
                    </div>
                    <span class="text-lg font-light mt-1 tracking-wide">Pause</span>
                {/if}
            </button>
            <div class="border-l-2 border-white/70"></div>
            <button class="w-1/2 flex flex-col items-center justify-center cursor-pointer hover:bg-primary hover:text-white p-6 rounded-br-2xl"
                    onclick={async () => { await startSession() }}>
                <div class="size-8">
                    <GlassEmpty/>
                </div>
                <span class="text-lg font-light mt-1 tracking-wide whitespace-nowrap">Drink Now</span>
            </button>
        </div>
    </div>
</AutoSize>