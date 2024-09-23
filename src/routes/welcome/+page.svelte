<script lang="ts">
    import {commands} from '../../bindings';
    import {enable} from "@tauri-apps/plugin-autostart";
    import {info, warn} from "@tauri-apps/plugin-log";
    import {onMount} from "svelte";
    import BackgroundVideo from "./BackgroundVideo.svelte";

    let next_break_duration_minutes: string = "180"
    let enable_on_startup = true;

    let backgroundVideoLoaded: boolean = false;

    async function startSession() {
        await info("start session")
        if (enable_on_startup) {
            await enable()
        }

        await commands.startFirstSession(
            parseInt(next_break_duration_minutes),
            enable_on_startup
        );

    }

    // allows no context menu
    document.addEventListener('contextmenu', event => event.preventDefault());
</script>


<div class="{backgroundVideoLoaded ? 'video-background-ready' : 'video-not-ready'} h-screen flex flex-col items-center justify-center cursor-default rounded-2xl">
    <div class="relative z-10 flex flex-col items-center p-8">
        <h1 class="text-4xl mb-4 accent-mm-blue">Welcome to</h1>
        <h2 class="text-6xl text-mm-orange mb-8">Motion Minutes</h2>
        <p class="px-16 text-xl text-center italic mb-16 block">
            Stay active and energized throughout your day with gentle reminders to stretch and move.
        </p>
        <label class="flex items-center space-x-4 mb-4">
            <span class="text-gray-700 font-thin">Load on startup</span><input bind:checked={enable_on_startup}
                                                                               class="checked:bg-mm-blue-50 bg-mm-blue-50"
                                                                               type="checkbox">
        </label>
        <div class="flex items-center space-x-4 mb-6">
            <p class="text-xl">Start my session every</p>
            <select
                    bind:value={next_break_duration_minutes}
                    class="p-2 bg-transparent bg-mm-pink-100 rounded-l shadow-sm text-right text-black w-24">
                <option class="p-2" value=10>10 min</option>
                <option class="p-2" value=30>30 min</option>
                <option class="p-2" value=60>1 hour</option>
                <option class="p-2" value=120>2 hours</option>
                <option class="p-2" value=180>3 hours</option>
            </select>
        </div>
        <div>
            <button class="bg-mm-orange-100 hover:bg-mm-orange hover:text-white text-mm-blue-500 font-medium py-2 px-4 rounded-xl text-xl cursor-pointer"
                    on:click={startSession}>
                Start your first session
            </button>
        </div>
    </div>
    <BackgroundVideo bind:backgroundVideoLoaded={backgroundVideoLoaded}></BackgroundVideo>
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