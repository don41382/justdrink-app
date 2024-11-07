<script lang="ts">
    import {info, warn} from "@tauri-apps/plugin-log";
    import {commands} from "../../bindings";
    import {relaunch} from "@tauri-apps/plugin-process";
    import AutoSize from "../AutoSize.svelte";
    import type {StateType} from "./stateType";

    let {data} = $props();
    let error: string | null = $state(data.error);
    let currentState: StateType = $state(data.initialState);

    let downloaded = $state(0);
    let total = $state(0);

    let percentage = $derived(parseFloat((total > 0 ? (downloaded / total) * 100 : 0).toFixed(0)));

    async function closeWindow() {
        await commands.updaterClose();
    }

    async function installUpdate() {
        if (data.update) {
            await data.update.downloadAndInstall(async (event) => {
                switch (event.event) {
                    case 'Started':
                        currentState = "started"
                        downloaded = 0;
                        total = event.data.contentLength ?? 0;
                        break;
                    case 'Progress':
                        if (currentState != "progress") {
                            currentState = "progress"
                        }
                        downloaded += event.data.chunkLength;
                        break;
                    case 'Finished':
                        await info("finished download")
                        currentState = "finished"
                        downloaded = total
                        break;
                }
            }).catch(async (err) => {
                currentState = "error"
                error = err
            })

            if (currentState == "finished") {
                await relaunch().catch(async (err) => {
                    currentState = "error"
                    error = `Unable to relaunch: ${err}`
                })
            } else {
                await warn(`unknown update status: ${currentState}`)
            }
        }
    }

</script>

<AutoSize class="bg-white w-[500px] rounded-lg border-mm-blue-50 border-2 outline-mm-blue p-6">
    {#if data.update}
        <!-- Logo and Update Info -->
        <div class="flex items-start mb-4">
            <img src="{data.iconPath}" alt="App Logo" class="w-14 h-14 mr-4">
            <div>
                <h1 class="text-xl font-semibold text-gray-800">Update Available</h1>
                <p class="text-gray-500">New version: <b>{data.update.version}</b></p>
            </div>
        </div>

        <p class="text-gray-600 mb-6">A new version of Motion Minute is available. Would you like to update to the
            latest version?</p>

        <!-- Progress bar (visible when download starts) -->
        {#if currentState === "started" || currentState === "progress"}
            <div class="mb-4">
                <div class="w-full bg-gray-200 rounded-full h-2.5">
                    <div class="bg-blue-500 h-2.5 rounded-full" style="width: {percentage}%;"></div>
                </div>
                <p class="text-sm text-gray-500 mt-2">Updating... {percentage}%</p>
            </div>
        {/if}

        <div class="flex justify-end space-x-3">
            {#if currentState === "init"}
                <button class="bg-gray-300 text-gray-800 px-4 py-2 rounded-lg hover:bg-gray-400 focus:outline-none focus:ring-2 focus:ring-gray-300 cursor-pointer"
                        onclick={async () => closeWindow()}>
                    Later
                </button>
                <button class="bg-mm-green text-white px-4 py-2 rounded-lg hover:bg-mm-green-600 focus:outline-none focus:ring-2 focus:ring-blue-500 cursor-pointer"
                        onclick={async () => installUpdate()}>
                    Update Now
                </button>
            {:else if currentState === "finished"}
                <p>Please wait, restarting ....</p>
            {:else if currentState === "error"}
                <p>Error while updating: {error}</p>
                <button class="bg-gray-300 text-gray-800 px-4 py-2 rounded-lg hover:bg-gray-400 focus:outline-none focus:ring-2 focus:ring-gray-300 cursor-pointer"
                        onclick={async () => closeWindow()}>
                    Close
                </button>
            {/if}
        </div>
    {:else}
        {#if currentState === "newest" }
            <!-- Logo and Update Info -->
            <div class="flex items-start mb-4">
                <img src="{data.iconPath}" alt="App Logo" class="w-14 h-14 mr-4">
                <div>
                    <h1 class="text-xl font-semibold text-gray-800">Up to date.</h1>
                    <p class="text-gray-500">Their is no newer version.</p>
                </div>
            </div>

            <p class="text-gray-600 mb-6">You are currently on the newest version. Please check later.</p>

            <div class="flex justify-end space-x-3">
                <button class="bg-gray-300 text-gray-800 px-4 py-2 rounded-lg hover:bg-gray-400 focus:outline-none focus:ring-2 focus:ring-gray-300 cursor-pointer"
                        onclick={async () => closeWindow()}>
                    Close
                </button>
            </div>
        {:else if currentState === "error"}
            <h1 class="text-xl font-normal text-amber-950 mb-4">Error on Update.</h1>
            <p class="text-gray-600 mb-6">{error}</p>
            <div class="flex justify-end space-x-3">
                <button class="bg-gray-300 text-gray-800 px-4 py-2 rounded-lg hover:bg-gray-400 focus:outline-none focus:ring-2 focus:ring-gray-300 cursor-pointer"
                        onclick={async () => closeWindow()}>
                    Close
                </button>
            </div>
        {:else if currentState === "finished"}
            <h1 class="font-normal text-gray-800">Please wait, checking version ...</h1>
        {:else}
            <h1 class="font-normal text-gray-800">Please wait, checking update ...</h1>
        {/if}
    {/if}
</AutoSize>
