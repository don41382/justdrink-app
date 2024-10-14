<script lang="ts">
    import {commands} from '../../bindings';
    import {enable} from "@tauri-apps/plugin-autostart";
    import {info, warn} from "@tauri-apps/plugin-log";
    import {onMount} from "svelte";
    import BackgroundVideo from "../BackgroundVideo.svelte";
    import * as tauri_path from "@tauri-apps/api/path";
    import {convertFileSrc} from "@tauri-apps/api/core";

    let initialDuration = 180

    let next_break_duration_minutes: number = initialDuration
    let enable_on_startup = true;


    let icon_path: string;

    onMount(async () => {
        await info("welcome video mounted")
        icon_path = convertFileSrc(`${await tauri_path.resourceDir()}/icons/128x128.png`);
    });

    async function startSession() {
        await info("start session")
        if (enable_on_startup) {
            await enable()
        }

        await commands.startFirstSession(
            next_break_duration_minutes,
            enable_on_startup
        );

    }
    
    function selectDuration(duration: number) {
        next_break_duration_minutes = duration;
    }

    // allows no context menu
    document.addEventListener('contextmenu', event => event.preventDefault());
</script>


<div class="bg-gray-100 h-screen flex flex-col items-center justify-center cursor-default rounded-2xl">
    <div class="px-16 z-10">
        <div class="mb-6 text-center" id="logo">
            <div class="flex items-center justify-center">
                <img alt="logo" class="w-32 h-32" src="{icon_path}">
            </div>
        </div>

        <h1 class="text-4xl text-mm-orange text-center mb-2">Motion Minute</h1>

        <p class="text-center text-neutral-600 mb-8">
            Stay active and energized throughout your day with gentle reminders to stretch and move.
        </p>

        <div class="mb-8" id="session-options">
            <h2 class="text-lg font-semibold mb-3">How often do you want to exercise?</h2>
            <div class="grid grid-cols-1 gap-3">
                    {#each [30, 60, initialDuration, 240] as duration}
                        <button 
                                on:click={() => selectDuration(duration)}
                                class="{next_break_duration_minutes === duration ? 'text-white bg-mm-green' : 'text-black bg-gray-200'} hover:bg-mm-green-300 py-2 rounded-md">
                            <span class="font-thin">every</span>
                            {duration >= 60 ?  `${duration / 60} hour` : `${duration} minutes`}
                        </button>
                    {/each}
                </div>
        </div>

        <div class="flex items-center mb-8" >
            <input bind:checked={enable_on_startup} class="mr-2 h-4 w-4 rounded border-neutral-300 text-neutral-600 focus:ring-neutral-500" type="checkbox">
            <label class="text-sm text-neutral-600" for="load-startup">Load on startup</label>
        </div>

        <button class="w-full bg-mm-orange hover:bg-mm-orange-200 text-white py-2 rounded-md" on:click={startSession}>
            Start your first session
        </button>
    </div>
</div>

<style>
    .video-background-ready {
        opacity: 1.0;
        transition: opacity 1s;
    }

    .video-not-ready {
        opacity: 0;
    }
</style>